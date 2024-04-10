use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use is_url::is_url;
use penumbra_proto::penumbra::core::asset::v1 as pb;
use svg_hush::{data_url_filter, Filter};

use crate::error::AppResult;
use crate::querier::request_img;

pub async fn sanitize_and_inline_metadata(
    all_metadata: Vec<pb::Metadata>,
) -> AppResult<Vec<pb::Metadata>> {
    let mut sanitized = vec![];
    for metadata in all_metadata {
        let mut cloned = metadata.clone();

        for img in &mut cloned.images {
            if is_url(&img.svg) {
                let bytes = request_img(&img.svg).await?;
                let sanitized = sanitize_svg(&bytes)?;
                if sanitized.is_empty() {
                    img.svg = "".to_string();
                }

                let base_64 = BASE64_STANDARD.encode(sanitized);
                let data_uri = format!("data:image/svg+xml;base64,{}", base_64);
                img.svg = data_uri;
            }
            if is_url(&img.png) {
                let bytes = request_img(&img.png).await?;
                let base_64 = BASE64_STANDARD.encode(bytes);
                let data_uri = format!("data:image/png;base64,{}", base_64);
                img.png = data_uri;
            }
        }
        sanitized.push(cloned)
    }
    Ok(sanitized)
}

/// SVGs can contain harmful scripts. This strips those out.
pub fn sanitize_svg(svg: &[u8]) -> AppResult<Vec<u8>> {
    let mut filter = Filter::new();
    filter.set_data_url_filter(data_url_filter::allow_standard_images);

    let mut out = Vec::new();
    filter.filter(svg, &mut out)?;
    Ok(out)
}
