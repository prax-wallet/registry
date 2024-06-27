use std::fs;
use std::path::Path;

use penumbra_asset::asset::Metadata;
use penumbra_proto::core::asset::v1::AssetImage;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;
use crate::processor::Globals;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsInput {
    pub rpcs: Vec<Rpc>,
    pub frontends: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    pub chain_id: String,
    pub validators: Vec<ValidatorInput>,
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

impl From<Image> for AssetImage {
    fn from(image: Image) -> Self {
        AssetImage {
            png: image.png.unwrap_or_default(),
            svg: image.svg.unwrap_or_default(),
            theme: None,
        }
    }
}

pub trait IntoPbImages {
    fn into_pb_images(self) -> Vec<AssetImage>;
}

impl IntoPbImages for Vec<Image> {
    fn into_pb_images(self) -> Vec<AssetImage> {
        self.into_iter().map(AssetImage::from).collect()
    }
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorInput {
    pub name: String,
    pub base: String,
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
pub fn get_chain_configs(input_dir: &str) -> AppResult<Vec<ChainConfig>> {
    let input_path = Path::new(input_dir).join("chains");
    let chain_configs = fs::read_dir(input_path)?;
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

// Validates globals and copies over to registry without change
pub fn copy_globals(input_dir: &str, registry_dir: &str) -> AppResult<()> {
    let input_path = Path::new(input_dir).join("globals.json");
    let json_data = fs::read_to_string(input_path)?;
    let globals_input: GlobalsInput = serde_json::from_str(&json_data)?;
    let globals: Globals = globals_input.try_into()?;

    // Write the validated JSON data to the output file
    let output_path = Path::new(registry_dir).join("globals.json");
    let output_json = serde_json::to_string_pretty::<Globals>(&globals)?;
    fs::write(output_path, output_json)?;

    Ok(())
}

// Deletes and re-creates registry dir
pub fn reset_registry_dir(path: &str) -> AppResult<()> {
    let dir_path = Path::new(path);

    // Create the directory if it doesn't exist
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    // Remove all the contents of the directory
    for entry in fs::read_dir(dir_path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path)?;
        } else {
            fs::remove_file(entry_path)?;
        }
    }

    // Create the "chains" directory inside
    let chains_dir = dir_path.join("chains");
    fs::create_dir(chains_dir)?;

    Ok(())
}
