use std::fs;
use std::path::Path;

use penumbra_proto::penumbra::core::asset::v1 as pb;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::error::AppResult;
use crate::github::assetlist_schema::AssetTypeAsset;
use crate::metadata::convert_to_proto_metadata;
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
    pub assets: Vec<pb::Metadata>,
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

async fn process_chain_config(chain_config: ChainConfig) -> AppResult<Registry> {
    let mut all_metadata = Vec::new();
    all_metadata.extend(chain_config.native_assets.clone());

    // For each ibc connection, fetch all metadata of native assets from the cosmos registry
    let responses = query_github_assets(&chain_config).await?;

    for (ibc_asset, asset_list) in responses {
        for asset in asset_list.assets {
            // ICS20 assets should be unwound through their native chains, we can skip
            if asset.type_asset == AssetTypeAsset::Ics20 {
                continue;
            }
            let metadata = convert_to_proto_metadata(&ibc_asset, asset)?;
            all_metadata.push(metadata);
        }
    }

    Ok(Registry {
        chain_id: chain_config.chain_id,
        ibc: chain_config
            .ibc_assets
            .into_iter()
            .map(Into::into)
            .collect(),
        assets: all_metadata,
    })
}
