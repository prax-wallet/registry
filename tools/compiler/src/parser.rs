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
    pub ibc_connections: Vec<IbcConnection>,
    pub native_assets: Vec<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rpc {
    pub name: String,
    pub url: String,
    pub images: Vec<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub png: Option<String>,
    pub svg: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcConnection {
    pub chain_id: String,
    pub ibc_channel: String,
    pub address_prefix: String,
    pub cosmos_registry_dir: String,
}

pub const LOCAL_REGISTRY_DIR: &str = "../../registry";
const LOCAL_INPUT_DIR: &str = "../../input";

/// Retrieves a list of `ChainConfig` objects representing the configuration for various chains.
/// This function assumes a specific directory structure and configs inside:
///
/// input/
/// ├── penumbra-testnet-deimos-6.json
/// └── mars-1.json
pub fn get_chain_configs() -> AppResult<Vec<ChainConfig>> {
    // Clear registry output dir
    let registry_dir = Path::new(LOCAL_REGISTRY_DIR);
    if registry_dir.exists() {
        fs::remove_dir_all(registry_dir)?;
    }
    fs::create_dir_all(registry_dir)?;

    let chain_configs = fs::read_dir(LOCAL_INPUT_DIR)?;
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
