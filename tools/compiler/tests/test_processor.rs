use penumbra_asset::asset::Metadata;
use penumbra_registry::parser::IbcInput;
use penumbra_registry::processor::{base64_id, transport_metadata_along_channel};

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
