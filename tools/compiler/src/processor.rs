use anyhow::anyhow;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tracing::instrument;

use crate::assetlist_schema::{AssetList, AssetTypeAsset};
use crate::error::{AppError, AppResult};
use crate::parser::{
    copy_globals, get_chain_configs, reset_registry_dir, ChainConfig, EntityMetadata, GlobalsInput,
    IbcInput, LOCAL_INPUT_DIR, LOCAL_REGISTRY_DIR,
};
use crate::validator::generate_metadata_from_validators;
use color_thief::{Color, ColorFormat};
use image;
use penumbra_asset::asset::{Id, Metadata};
use penumbra_asset::STAKING_TOKEN_ASSET_ID;
use penumbra_proto::core::asset::v1::{asset_image::Theme, AssetImage};
use penumbra_proto::penumbra::core::asset::v1 as pb;
use reqwest;
use resvg::{
    render,
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
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
    pub images: Vec<AssetImage>,
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

/// Process images in the registry to extract and add dominant colors
pub fn process_registry_images(registry: &mut Registry) -> AppResult<()> {
    for metadata in registry.asset_by_id.values_mut() {
        let mut pb_metadata: pb::Metadata = metadata.clone().into();

        for image in pb_metadata.images.iter_mut() {
            // Skip if we already have a color
            if image
                .theme
                .as_ref()
                .map(|t| !t.primary_color_hex.is_empty())
                .unwrap_or(false)
            {
                continue;
            }

            let color = if !image.svg.is_empty() {
                get_dominant_color_from_svg(&image.svg)
                    .map_err(|e| anyhow!("Failed to process SVG: {}", e))?
            } else if !image.png.is_empty() {
                get_dominant_color_from_png(&image.png)
                    .map_err(|e| anyhow!("Failed to process PNG: {}", e))?
            } else {
                continue;
            };

            let hex = format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b);

            // Create or update theme
            image.theme = Some(Theme {
                primary_color_hex: hex,
                ..Default::default()
            });
        }

        // Update the metadata with the new image data
        *metadata = Metadata::try_from(pb_metadata)?;
    }
    Ok(())
}

fn get_dominant_color_from_svg(url: &str) -> Result<Color, anyhow::Error> {
    // Download the SVG content
    let response =
        reqwest::blocking::get(url).map_err(|e| anyhow!("Failed to download SVG: {}", e))?;
    let svg_content = response
        .text()
        .map_err(|e| anyhow!("Failed to read SVG content: {}", e))?;

    // Parse SVG into a usvg tree
    let options = Options::default();
    let rtree = Tree::from_data(svg_content.as_bytes(), &options)
        .map_err(|e| anyhow!("Failed to parse SVG: {}", e))?;

    // Create a pixel buffer with dimensions from the SVG
    let view_box = rtree.size();
    let mut pixmap = Pixmap::new(view_box.width() as u32, view_box.height() as u32)
        .ok_or_else(|| anyhow!("Failed to create pixmap"))?;

    // Render the SVG to the pixel buffer
    render(&rtree, Transform::default(), &mut pixmap.as_mut());

    // Convert pixmap to RGB format for color-thief
    let raw_pixels = pixmap.data();

    // Get dominant color using color-thief
    let dominant = color_thief::get_palette(raw_pixels, ColorFormat::Rgba, 1, 10)
        .map_err(|_| anyhow!("Failed to get dominant color"))?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No dominant color found"))?;

    Ok(dominant)
}

fn get_dominant_color_from_png(url: &str) -> Result<Color, anyhow::Error> {
    // Download the image
    let response =
        reqwest::blocking::get(url).map_err(|e| anyhow!("Failed to download PNG: {}", e))?;
    let img_bytes = response
        .bytes()
        .map_err(|e| anyhow!("Failed to read PNG bytes: {}", e))?;
    let img =
        image::load_from_memory(&img_bytes).map_err(|e| anyhow!("Failed to load PNG: {}", e))?;

    // Convert image to RGB pixels
    let pixels: Vec<u8> = img.to_rgb8().into_raw();

    // Get dominant color using color-thief
    let dominant = color_thief::get_palette(&pixels, ColorFormat::Rgb, 1, 10)
        .map_err(|_| anyhow!("Failed to get dominant color"))?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No dominant color found"))?;

    Ok(dominant)
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

            let transferred_asset =
                transport_metadata_along_channel(ibc_input, source_asset_metadata)?;
            all_metadata.push(transferred_asset);
        }
    }

    // add priority score if available
    for metadata in &mut all_metadata {
        if let Some(score) = chain_config
            .priority_scores_by_base
            .get(&metadata.base_denom().denom)
        {
            let mut pb_metadata: pb::Metadata = metadata.clone().into();
            pb_metadata.priority_score = *score;
            *metadata = Metadata::try_from(pb_metadata)?;
        }
    }

    // add coingecko_id if available
    for metadata in &mut all_metadata {
        if let Some(coingecko_id) = chain_config
            .ibc_assets
            .iter()
            .find(|asset| asset.base.eq(&metadata.base_denom().denom))
            .and_then(|asset| asset.coingecko_id.clone())
        {
            let mut pb_metadata: pb::Metadata = metadata.clone().into();
            pb_metadata.coingecko_id = coingecko_id;
            *metadata = Metadata::try_from(pb_metadata)?;
        }
    }

    // add badges if available
    for metadata in &mut all_metadata {
        if let Some(badges) = chain_config
            .badges_by_base
            .get(&metadata.base_denom().denom)
        {
            let mut pb_metadata: pb::Metadata = metadata.clone().into();
            pb_metadata.badges = badges
                .iter()
                .map(|b| {
                    chain_config
                        .badges
                        .get(b)
                        .ok_or_else(|| anyhow::anyhow!("Badge not found: {}", b))
                        .cloned()
                })
                .collect::<Result<Vec<_>, _>>()?;
            *metadata = Metadata::try_from(pb_metadata)?;
        }
    }

    let mut registry = Registry {
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
    };

    // Process images to add dominant colors
    process_registry_images(&mut registry)?;

    Ok(registry)
}
