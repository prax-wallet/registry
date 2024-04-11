use anyhow::anyhow;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use penumbra_asset::asset::Metadata;
use penumbra_proto::penumbra::core::asset::v1 as pb;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::error::AppResult;
use crate::github::assetlist_schema::AssetTypeAsset;
use crate::parser::{get_chain_configs, ChainConfig, IbcConfig, LOCAL_REGISTRY_DIR};
use crate::querier::query_github_assets;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcChains {
    pub address_prefix: String,
    pub chain_id: String,
    pub ibc_channel: String,
}

impl From<IbcConfig> for IbcChains {
    fn from(config: IbcConfig) -> Self {
        IbcChains {
            address_prefix: config.address_prefix,
            chain_id: config.chain_id,
            ibc_channel: config.ibc_channel,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub chain_id: String,
    pub ibc: Vec<IbcChains>,
    // Use a BTreeMap to have sorted (deterministic) output
    pub asset_by_id: BTreeMap<String, Metadata>,
}

pub async fn generate_registry() -> AppResult<()> {
    let mut tasks = Vec::new();

    // Get local configs from /input directory
    let chain_configs = get_chain_configs()?;
    chain_configs.into_iter().for_each(|c| {
        // Async fetch metadata for ibc assets from cosmos registry
        let task = task::spawn(async move { process_chain_config(c).await });
        tasks.push(task);
    });

    // Take resulting registries and save to /registry
    for task in tasks {
        let registry = task.await??;
        let file_name = format!("{}.json", registry.chain_id);
        let output_path = Path::new(LOCAL_REGISTRY_DIR).join(file_name);
        let output_json = serde_json::to_string_pretty(&registry)?;
        fs::write(output_path, output_json)?;
    }

    Ok(())
}

/// Given `ibc_data` describing a channel and `source_asset` on the source chain,
/// compute the metadata for the asset when it is transported along the channel onto a Penumbra chain.
fn transport_metadata_along_channel(
    ibc_data: &IbcConfig,
    source_asset: Metadata,
) -> AppResult<Metadata> {
    // The `Metadata` structure doesn't allow modifying the internals, so drop to raw proto data
    let mut pb_metadata: pb::Metadata = source_asset.into();
    tracing::debug!(?pb_metadata, "original");

    let prefix_channel = |x: &mut String| {
        *x = format!("transfer/{}/{}", ibc_data.ibc_channel, x);
    };

    // Prefix the channel to the base denom.
    prefix_channel(&mut pb_metadata.base);
    // Prefix the channel to the display denom.
    prefix_channel(&mut pb_metadata.display);
    // Prefix the channel to all denom units.
    for denom_unit in pb_metadata.denom_units.iter_mut() {
        prefix_channel(&mut denom_unit.denom);
    }

    // Delete the asset ID, so that it will be recomputed with the adjusted base denom.
    // Without this, decoding will fail because the asset ID won't match.
    pb_metadata.penumbra_asset_id = None;

    tracing::debug!(?pb_metadata, "new");
    Ok(Metadata::try_from(pb_metadata)?)
}

fn base64_id(m: &Metadata) -> AppResult<String> {
    let id_json = serde_json::to_value(m.id())?;
    let base64_str = id_json
        .get("inner")
        .and_then(|s| s.as_str()) // This extracts the string without the double quotes
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow!("Unexpected id json structure"))?;
    Ok(base64_str)
}

async fn process_chain_config(chain_config: ChainConfig) -> AppResult<Registry> {
    let mut all_metadata = Vec::new();
    all_metadata.extend(chain_config.native_assets.clone());

    // For each ibc connection, fetch all metadata of native assets from the cosmos registry
    let responses = query_github_assets(&chain_config).await?;

    for (ibc_data, asset_list) in responses {
        for source_asset in asset_list.assets {
            // ICS20 assets should be unwound through their native chains, we can skip
            if source_asset.type_asset == AssetTypeAsset::Ics20 {
                continue;
            }
            // Turn the asset back into JSON so we can deserialize it as a penumbra Metadata
            let asset_json = serde_json::to_string(&source_asset)?;
            let source_asset_metadata = serde_json::from_str(&asset_json)?;
            let transferred_asset =
                transport_metadata_along_channel(&ibc_data, source_asset_metadata)?;
            tracing::info!(?asset_json, transferred_asset_json = ?serde_json::to_string(&transferred_asset));
            all_metadata.push(transferred_asset);
        }
    }

    Ok(Registry {
        chain_id: chain_config.chain_id,
        ibc: chain_config
            .ibc_assets
            .into_iter()
            .map(Into::into)
            .collect(),
        asset_by_id: all_metadata
            .into_iter()
            .map(|m| {
                let id = base64_id(&m)?;
                Ok((id, m))
            })
            .collect::<AppResult<_>>()?,
    })
}
