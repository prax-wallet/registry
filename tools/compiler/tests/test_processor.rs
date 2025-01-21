use penumbra_asset::asset::Metadata;
use penumbra_registry::parser::IbcInput;
use penumbra_registry::processor::{base64_id, transport_metadata_along_channel, Registry};

#[test]
fn base64_id_extracts_correctly() {
    let asset_json = r#"
        {
            "base": "upenumbra",
            "penumbraAssetId": {
                "inner": "KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA="
            }
        }"#;
    let metadata: Metadata = serde_json::from_str(asset_json).unwrap();

    assert_eq!(
        base64_id(&metadata.id()).unwrap(),
        "KeqcLzNx9qSH5+lcJHBB9KNW+YPrBk5dKzvPMiypahA="
    );
}

#[test]
fn test_transport_metadata_along_channel() {
    let ibc_data = IbcInput {
        channel_id: "channel-123".to_string(),
        counterparty_channel_id: "channel-456".to_string(),
        chain_id: "love-999".to_string(),
        address_prefix: "love".to_string(),
        cosmos_registry_dir: "love-124".to_string(),
        display_name: "Strangelove".to_string(),
        images: vec![],
    };

    let input_json = r#"
        {
          "denomUnits": [
            {
              "denom": "gm",
              "exponent": 6
            },
            {
              "denom": "ugm"
            }
          ],
          "base": "ugm",
          "display": "gm",
          "symbol": "GM",
          "penumbraAssetId": {
            "inner": "HW2Eq3UZVSBttoUwUi/MUtE7rr2UU7/UH500byp7OAc="
          }
        }
    "#;
    let input_metadata = serde_json::from_str(input_json).unwrap();

    let output_json = r#"
        {
          "denomUnits": [
            {
              "denom": "transfer/channel-123/gm",
              "exponent": 6
            },
            {
              "denom": "transfer/channel-123/ugm"
            }
          ],
          "base": "transfer/channel-123/ugm",
          "display": "transfer/channel-123/gm",
          "symbol": "GM",
          "penumbraAssetId": {
            "inner": "YGObCaxvA7dR5tg6FeoNDxIGbwl9HK5eYr7jFho/GwQ="
          }
        }
    "#;
    let output_metadata = serde_json::from_str(output_json).unwrap();

    let result = transport_metadata_along_channel(&ibc_data, input_metadata).unwrap();
    assert_eq!(result, output_metadata);
}

#[test]
fn test_process_registry_images() {
    let mut registry = Registry {
        chain_id: "test-1".to_string(),
        ibc_connections: vec![],
        asset_by_id: Default::default(),
        numeraires: vec![],
    };

    // Create a test metadata with both SVG and PNG images
    let metadata_json = r##"{
        "base": "test",
        "display": "test",
        "name": "Test Token",
        "symbol": "TEST",
        "denomUnits": [
            {
                "denom": "test",
                "exponent": 6
            }
        ],
        "images": [
            {
                "png": "https://raw.githubusercontent.com/cosmos/chain-registry/master/cosmoshub/images/atom.png",
                "svg": ""
            },
            {
                "png": "",
                "svg": "https://raw.githubusercontent.com/cosmos/chain-registry/master/cosmoshub/images/atom.svg"
            },
            {
                "png": "",
                "svg": "",
                "theme": {
                    "primaryColorHex": "#123456"
                }
            }
        ]
    }"##;

    let metadata: Metadata = serde_json::from_str(metadata_json).unwrap();
    let id = base64_id(&metadata.id()).unwrap();
    registry.asset_by_id.insert(id, metadata);

    // Process the images
    penumbra_registry::processor::process_registry_images(&mut registry).unwrap();

    // Get the processed metadata
    let processed_metadata = registry.asset_by_id.values().next().unwrap();
    let pb_metadata: penumbra_proto::penumbra::core::asset::v1::Metadata =
        processed_metadata.clone().into();

    // Check that colors were added to images without themes
    assert!(pb_metadata.images[0].theme.is_some());
    assert!(pb_metadata.images[1].theme.is_some());

    // Check that existing theme was preserved
    assert_eq!(
        pb_metadata.images[2]
            .theme
            .as_ref()
            .unwrap()
            .primary_color_hex,
        "#123456"
    );

    // Verify the color format
    let color_hex = &pb_metadata.images[0]
        .theme
        .as_ref()
        .unwrap()
        .primary_color_hex;
    assert!(color_hex.starts_with('#'));
    assert_eq!(color_hex.len(), 7); // #RRGGBB format
    assert!(color_hex.chars().skip(1).all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_process_registry_images_with_invalid_urls() {
    let mut registry = Registry {
        chain_id: "test-1".to_string(),
        ibc_connections: vec![],
        asset_by_id: Default::default(),
        numeraires: vec![],
    };

    // Create a test metadata with invalid image URLs
    let metadata_json = r#"{
        "base": "test",
        "display": "test",
        "name": "Test Token",
        "symbol": "TEST",
        "denomUnits": [
            {
                "denom": "test",
                "exponent": 6
            }
        ],
        "images": [
            {
                "png": "https://invalid-url/image.png",
                "svg": ""
            },
            {
                "png": "",
                "svg": "https://invalid-url/image.svg"
            }
        ]
    }"#;

    let metadata: Metadata = serde_json::from_str(metadata_json).unwrap();
    let id = base64_id(&metadata.id()).unwrap();
    registry.asset_by_id.insert(id, metadata);

    // Process should fail gracefully
    let result = penumbra_registry::processor::process_registry_images(&mut registry);
    assert!(result.is_err());
}

#[test]
fn test_process_registry_images_empty_urls() {
    let mut registry = Registry {
        chain_id: "test-1".to_string(),
        ibc_connections: vec![],
        asset_by_id: Default::default(),
        numeraires: vec![],
    };

    // Create a test metadata with empty image URLs
    let metadata_json = r#"{
        "base": "test",
        "display": "test",
        "name": "Test Token",
        "symbol": "TEST",
        "denomUnits": [
            {
                "denom": "test",
                "exponent": 6
            }
        ],
        "images": [
            {
                "png": "",
                "svg": ""
            }
        ]
    }"#;

    let metadata: Metadata = serde_json::from_str(metadata_json).unwrap();
    let id = base64_id(&metadata.id()).unwrap();
    let id_clone = id.clone();
    registry.asset_by_id.insert(id, metadata.clone());

    // Process should succeed but not modify the metadata
    penumbra_registry::processor::process_registry_images(&mut registry).unwrap();
    assert_eq!(registry.asset_by_id[&id_clone], metadata);
}
