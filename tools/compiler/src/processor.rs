use anyhow::anyhow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::instrument;

use crate::assetlist_schema::{AssetList, AssetTypeAsset};
use crate::error::{AppError, AppResult};
use crate::parser::{
    copy_globals, get_chain_configs, reset_registry_dir, ChainConfig, EntityMetadata, GlobalsInput,
    IbcInput, Image, LOCAL_INPUT_DIR, LOCAL_REGISTRY_DIR,
};
use crate::validator::generate_metadata_from_validators;
use penumbra_asset::asset::{Id, Metadata};
use penumbra_asset::STAKING_TOKEN_ASSET_ID;
use penumbra_proto::penumbra::core::asset::v1 as pb;
use serde::{Deserialize, Serialize};

// Location of the `cosmos/chain-registry` submodule directory
const LOCAL_COSMOS_REGISTRY_DIR: &str = "./files/chain-registry/";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub address_prefix: String,
    pub chain_id: String,
    pub channel_id: String,
    pub counterparty_channel_id: String,
    pub display_name: String,
    pub images: Vec<Image>,
}

impl From<IbcInput> for Chain {
    fn from(config: IbcInput) -> Self {
        Chain {
            address_prefix: config.address_prefix,
            chain_id: config.chain_id,
            channel_id: config.channel_id,
            counterparty_channel_id: config.counterparty_channel_id,
            display_name: config.display_name,
            images: config.images,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Globals {
    pub rpcs: Vec<EntityMetadata>,
    #[deprecated]
    pub frontends: Vec<String>,
    pub frontends_v2: Vec<EntityMetadata>,
    pub staking_asset_id: Id,
}

impl TryFrom<GlobalsInput> for Globals {
    type Error = AppError;

    fn try_from(g: GlobalsInput) -> AppResult<Self> {
        #![allow(deprecated)]
        Ok(Globals {
            rpcs: g.rpcs,
            frontends: g.frontends,
            frontends_v2: g.frontends_v2,
            staking_asset_id: *STAKING_TOKEN_ASSET_ID,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub chain_id: String,
    pub ibc_connections: Vec<Chain>,
    pub asset_by_id: BTreeMap<String, Metadata>, // Using a BTreeMap to have sorted (deterministic) output
    pub numeraires: Vec<String>,
}

#[instrument]
pub fn generate_registry() -> AppResult<()> {
    reset_registry_dir(LOCAL_REGISTRY_DIR)?;
    copy_globals(LOCAL_INPUT_DIR, LOCAL_REGISTRY_DIR)?;

    // Get local configs from /input directory
    let chain_configs = get_chain_configs(LOCAL_INPUT_DIR)?;

    // Take resulting registries and save to /registry
    for c in chain_configs {
        let registry = process_chain_config(c)?;
        let file_name = format!("{}.json", registry.chain_id);
        let output_path = Path::new(LOCAL_REGISTRY_DIR).join("chains").join(file_name);
        let output_json = serde_json::to_string_pretty(&registry)?;
        fs::write(output_path, output_json)?;
    }

    Ok(())
}

/// Given `ibc_data` describing a channel and `source_asset` on the source chain,
/// compute the metadata for the asset when it is transported along the channel onto a Penumbra chain.
#[instrument(skip_all)]
pub fn transport_metadata_along_channel(
    ibc_data: &IbcInput,
    source_asset: Metadata,
    priority_scores_by_base: &HashMap<String, u64>,
) -> AppResult<Metadata> {
    // The `Metadata` structure doesn't allow modifying the internals, so drop to raw proto data
    let mut pb_metadata: pb::Metadata = source_asset.into();
    tracing::trace!(?pb_metadata, "original");

    let prefix_channel = |x: &mut String| {
        *x = format!("transfer/{}/{}", ibc_data.channel_id, x);
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

    // Add priority score if available
    if let Some(score) = priority_scores_by_base.get(&pb_metadata.base) {
        pb_metadata.priority_score = *score;
    }

    tracing::trace!(?pb_metadata, "new");
    Ok(Metadata::try_from(pb_metadata)?)
}

pub fn base64_id(id: &Id) -> AppResult<String> {
    let id_json = serde_json::to_value(id)?;
    let base64_str = id_json
        .get("inner")
        .and_then(|s| s.as_str()) // This extracts the string without the double quotes
        .map(|s| s.to_owned())
        .ok_or_else(|| anyhow!("Unexpected id json structure"))?;
    Ok(base64_str)
}

#[tracing::instrument(skip_all)]
fn process_chain_config(chain_config: ChainConfig) -> AppResult<Registry> {
    let mut all_metadata = Vec::new();

    all_metadata.extend(generate_metadata_from_validators(&chain_config.validators)?);
    all_metadata.extend(chain_config.native_assets.clone());

    // For each ibc connection, grab all metadata of native assets from the cosmos registry
    for ibc_input in &chain_config.ibc_connections {
        let assetlist_path = Path::new(LOCAL_COSMOS_REGISTRY_DIR)
            .join(&ibc_input.cosmos_registry_dir)
            .join("assetlist.json");

        // Parse the local JSON into the AssetList struct
        let data = fs::read_to_string(assetlist_path)?;
        let asset_list: AssetList = serde_json::from_str(&data)?;

        for source_asset in asset_list.assets {
            // ICS20 assets should be unwound through their native chains, we can skip
            if source_asset.type_asset == AssetTypeAsset::Ics20 {
                continue;
            }
            // Turn the asset back into JSON so we can deserialize it as a penumbra Metadata
            let asset_json = serde_json::to_string(&source_asset)?;
            let source_asset_metadata = serde_json::from_str(&asset_json)?;

            let transferred_asset = transport_metadata_along_channel(
                ibc_input,
                source_asset_metadata,
                &chain_config.priority_scores_by_base,
            )?;
            all_metadata.push(transferred_asset);
        }
    }

    Ok(Registry {
        chain_id: chain_config.chain_id,
        ibc_connections: chain_config
            .ibc_connections
            .into_iter()
            .map(Into::into)
            .collect(),
        asset_by_id: all_metadata
            .clone()
            .into_iter()
            .map(|m| {
                let id = base64_id(&m.id())?;
                Ok((id, m))
            })
            .collect::<AppResult<_>>()?,
        numeraires: all_metadata
            .into_iter()
            .filter(|metadata| {
                chain_config
                    .canonical_numeraires
                    .contains(&metadata.base_denom().denom)
            })
            .filter_map(|m| base64_id(&m.id()).ok())
            .collect(),
    })
}
