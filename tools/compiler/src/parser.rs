use std::fs;
use std::path::Path;

use anyhow::anyhow;
use penumbra_proto::penumbra::core::asset::v1 as pb;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    pub chain_id: String,
    pub ibc_assets: Vec<IbcConfig>,
    pub native_assets: Vec<pb::Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcConfig {
    pub chain_id: String,
    pub ibc_channel: String,
    pub address_prefix: String,
    pub cosmos_registry_dir: String,
}

pub const LOCAL_REGISTRY_DIR: &str = "registry";
const LOCAL_INPUT_DIR: &str = "input";
const IBC_ASSETS_FILE: &str = "ibc-assets.json";
const NATIVE_ASSETS_FILE: &str = "native-assets.json";

/// Retrieves a list of `ChainConfig` objects representing the configuration for various chains.
/// This function assumes a specific directory structure and set of files:
///
/// input/
/// ├── osmosis-test-5/
/// │   ├── ibc-assets.json
/// │   └── native-assets.json
/// └── mars-1/
///     ├── ibc-assets.json
///     └── native-assets.json
///
pub fn get_chain_configs() -> AppResult<Vec<ChainConfig>> {
    let registry_dir = Path::new(LOCAL_REGISTRY_DIR);
    if registry_dir.exists() {
        fs::remove_dir_all(registry_dir)?;
    }
    fs::create_dir_all(registry_dir)?;
    let chain_dirs = fs::read_dir(LOCAL_INPUT_DIR)?;

    chain_dirs
        .into_iter()
        .map(|dir_entry| -> AppResult<ChainConfig> {
            let chain_path = dir_entry?.path();
            let chain_id = chain_path
                .file_name()
                .ok_or_else(|| anyhow!("Failed to get the file name from the path"))?
                .to_str()
                .ok_or_else(|| anyhow!("Failed to convert OsStr to str"))?
                .to_owned();

            // Read the IBC assets and native assets from their respective JSON files
            let ibc_assets_path = chain_path.join(IBC_ASSETS_FILE);
            let native_assets_path = chain_path.join(NATIVE_ASSETS_FILE);

            // You might need to adjust the logic here if the files can be optional
            let ibc_assets_contents = fs::read_to_string(ibc_assets_path)?;
            let native_assets_contents = fs::read_to_string(native_assets_path)?;

            let chain_config = ChainConfig {
                chain_id,
                ibc_assets: serde_json::from_str(&ibc_assets_contents)?,
                native_assets: serde_json::from_str(&native_assets_contents)?,
            };
            Ok(chain_config)
        })
        .collect()
}
