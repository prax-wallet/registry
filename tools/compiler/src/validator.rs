use crate::error::AppResult;
use crate::parser::IntoPbImages;

use crate::parser::ValidatorInput;
use penumbra_asset::asset::Metadata;
use penumbra_proto::core::asset::v1::DenomUnit;
use penumbra_proto::penumbra::core::asset::v1 as pb;

pub fn generate_metadata_from_validators(
    validators: &[ValidatorInput],
) -> AppResult<Vec<Metadata>> {
    validators
        .iter()
        .map(|v| {
            // udelegation_penumbravalid1... -> delegation_penumbravalid1...
            let display_denom = v.base.chars().skip(1).collect::<String>();

            let pb_metadata = pb::Metadata {
                symbol: format!("delUM({})", v.name),
                base: v.base.clone(),
                images: v.images.clone().into_pb_images(),
                display: display_denom.clone(),
                denom_units: vec![
                    DenomUnit {
                        denom: v.base.clone(),
                        exponent: 0,
                        aliases: vec![],
                    },
                    DenomUnit {
                        denom: format!("m{}", display_denom.clone()),
                        exponent: 3,
                        aliases: vec![],
                    },
                    DenomUnit {
                        denom: display_denom,
                        exponent: 6,
                        aliases: vec![],
                    },
                ],
                ..Default::default()
            };

            Ok(Metadata::try_from(pb_metadata)?)
        })
        .collect()
}
