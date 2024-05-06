use std::fs;
use std::path::Path;

use penumbra_asset::asset::Metadata;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    pub chain_id: String,
    pub rpcs: Vec<Rpc>,
    pub ibc_connections: Vec<IbcInput>,
    pub native_assets: Vec<Metadata>,
    pub canonical_numeraires: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rpc {
    pub name: String,
    pub url: String,
    pub images: Vec<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcInput {
    pub chain_id: String,
    pub channel_id: String,
    pub counterparty_channel_id: String,
    pub address_prefix: String,
    pub cosmos_registry_dir: String,
    pub display_name: String,
    pub images: Vec<Image>,
}

pub const LOCAL_REGISTRY_DIR: &str = "../../registry";
pub const LOCAL_INPUT_DIR: &str = "../../input";

/// Retrieves a list of `ChainConfig` objects representing the configuration for various chains.
/// This function assumes a specific directory structure and configs inside:
///
/// input/
/// ├── penumbra-testnet-deimos-6.json
/// └── mars-1.json
pub fn get_chain_configs(registry_dir: &str, input_dir: &str) -> AppResult<Vec<ChainConfig>> {
    // Clear registry output dir
    let registry_dir = Path::new(registry_dir);
    if registry_dir.exists() {
        fs::remove_dir_all(registry_dir)?;
    }
    fs::create_dir_all(registry_dir)?;

    let chain_configs = fs::read_dir(input_dir)?;
    Ok(chain_configs
        .into_iter()
        .map(|input| -> AppResult<ChainConfig> {
            let input_path = input?.path();
            let input_contents = fs::read_to_string(input_path)?;
            let chain_config = serde_json::from_str(&input_contents)?;
            Ok(chain_config)
        })
        .filter_map(|result| match result {
            Ok(config) => Some(config),
            Err(e) => {
                tracing::info!("{}", e.to_string());
                None
            }
        })
        .collect())
}
