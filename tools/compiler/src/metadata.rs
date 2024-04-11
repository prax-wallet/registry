use std::default::Default;

use penumbra_asset::asset::Id;
use penumbra_proto::core::asset::v1::asset_image::Theme;
use penumbra_proto::core::asset::v1::{AssetImage, DenomUnit};
use penumbra_proto::penumbra::core::asset::v1 as pb;

use crate::error::AppResult;
use crate::github::assetlist_schema::Asset;
use crate::parser::IbcConfig;

pub fn convert_to_proto_metadata(ibc_asset: &IbcConfig, asset: Asset) -> AppResult<pb::Metadata> {
    // This conversion dance populates the penumbra_asset_id field
    // with the proper blake hash of the denom
    let asset_id = pb::AssetId {
        alt_base_denom: asset.base.clone(),
        ..Default::default()
    };
    let id = Id::try_from(asset_id)?;
    let asset_id = pb::AssetId::from(id);

    let denom_units = asset
        .denom_units
        .iter()
        .map(|du| DenomUnit {
            denom: format!("transfer/{}/{}", ibc_asset.ibc_channel, du.denom.clone()),
            exponent: du.exponent,
            aliases: du.aliases.clone(),
        })
        .collect();

    let images = asset
        .images
        .iter()
        .map(|ai| AssetImage {
            png: ai.clone().png.unwrap_or_default(),
            svg: ai.clone().svg.unwrap_or_default(),
            theme: ai.clone().theme.map(|t| Theme {
                primary_color_hex: t
                    .primary_color_hex
                    .map(|h| h.to_string())
                    .unwrap_or("".to_string()),
                circle: t.circle.unwrap_or_default(),
                dark_mode: t.dark_mode.unwrap_or_default(),
            }),
        })
        .collect();

    Ok(pb::Metadata {
        description: asset.description.unwrap_or_default(),
        base: asset.base,
        display: asset.display,
        name: asset.name.into(),
        symbol: asset.symbol,
        penumbra_asset_id: Some(asset_id),
        denom_units,
        images,
    })
}
