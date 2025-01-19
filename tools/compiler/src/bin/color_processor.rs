use anyhow::Result;
use color_thief::{Color, ColorFormat};
use resvg::{render, tiny_skia::Pixmap, usvg::*};
use serde_json::value::RawValue;
use serde_json::{json, Map, Value};
use std::{fs, path::Path};

fn main() -> Result<()> {
    let registry_dir = Path::new("../../registry/chains");
    let entries = fs::read_dir(registry_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            println!("Processing file: {}", path.display());
            if let Err(e) = process_registry_file(&path) {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}

fn process_registry_file(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let raw = RawValue::from_string(content.clone())?;
    let mut chain: Value = serde_json::from_str(raw.get())?;
    let mut modified = false;

    // Process each asset
    if let Some(assets) = chain.get_mut("assetById").and_then(|a| a.as_object_mut()) {
        for (id, asset) in assets {
            let asset_name = asset
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(id)
                .to_string();

            if let Some(images) = asset.get_mut("images").and_then(|i| i.as_array_mut()) {
                for image in images {
                    // Try PNG first, then SVG
                    let (image_url, is_svg) =
                        if let Some(png) = image.get("png").and_then(|p| p.as_str()) {
                            (png.to_string(), false)
                        } else if let Some(svg) = image.get("svg").and_then(|p| p.as_str()) {
                            (svg.to_string(), true)
                        } else {
                            continue;
                        };

                    // Only process if we don't already have a primaryColorHex
                    let has_color = image
                        .get("theme")
                        .and_then(|t| t.as_object())
                        .and_then(|t| t.get("primaryColorHex"))
                        .is_some();

                    if !has_color {
                        if is_svg {
                            println!("Processing SVG: {} for asset {}", image_url, asset_name);
                        }
                        let color = if is_svg {
                            get_dominant_color_from_svg(&image_url)?
                        } else {
                            get_dominant_color_from_png(&image_url)?
                        };

                        let hex = format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b);
                        if is_svg {
                            println!("Found color {} for SVG {}", hex, asset_name);
                        }

                        // Get or create theme object while preserving order
                        let theme = image.get_mut("theme").and_then(|t| t.as_object_mut());
                        if let Some(theme) = theme {
                            theme.insert("primaryColorHex".to_string(), json!(hex));
                        } else {
                            // Create new theme object with primaryColorHex
                            let mut map = Map::new();
                            map.insert("primaryColorHex".to_string(), json!(hex));
                            image["theme"] = Value::Object(map);
                        }
                        modified = true;
                    }
                }
            }
        }
    }

    // Only write if modified
    if modified {
        let file = fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &chain)?;
        println!("Updated {}", path.display());
    }

    Ok(())
}

fn get_dominant_color_from_svg(url: &str) -> Result<Color> {
    // Download the SVG content
    let response = reqwest::blocking::get(url)?;
    let svg_content = response.text()?;

    // Parse SVG into a usvg tree
    let options = Options::default();
    let rtree = Tree::from_data(svg_content.as_bytes(), &options)?;

    // Create a pixel buffer with dimensions from the SVG
    let view_box = rtree.size();
    let mut pixmap = Pixmap::new(view_box.width() as u32, view_box.height() as u32)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

    // Render the SVG to the pixel buffer
    render(&rtree, Transform::default(), &mut pixmap.as_mut());

    // Convert pixmap to RGB format for color-thief
    let raw_pixels = pixmap.data();

    // Get dominant color using color-thief
    let dominant = color_thief::get_palette(raw_pixels, ColorFormat::Rgba, 1, 10)
        .map_err(|_| anyhow::anyhow!("Failed to get dominant color"))?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No dominant color found"))?;

    Ok(dominant)
}

fn get_dominant_color_from_png(url: &str) -> Result<Color> {
    // Download the image
    let response = reqwest::blocking::get(url)?;
    let img_bytes = response.bytes()?;
    let img = image::load_from_memory(&img_bytes)?;

    // Convert image to RGB pixels
    let pixels: Vec<u8> = img.to_rgb8().into_raw();

    // Get dominant color using color-thief
    let dominant = color_thief::get_palette(&pixels, ColorFormat::Rgb, 1, 10)
        .map_err(|_| anyhow::anyhow!("Failed to get dominant color"))?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No dominant color found"))?;

    Ok(dominant)
}
