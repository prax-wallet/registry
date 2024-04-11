// Generated from cosmos assetlist schema: https://github.com/cosmos/chain-registry/blob/master/assetlist.schema.json
// Via Typify: https://github.com/oxidecomputer/typify
#![allow(clippy::all)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Asset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base\","]
#[doc = "    \"denom_units\","]
#[doc = "    \"display\","]
#[doc = "    \"name\","]
#[doc = "    \"symbol\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"address\": {"]
#[doc = "      \"description\": \"[OPTIONAL] The address of the asset. Only required for type_asset : cw20, snip20\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"base\": {"]
#[doc = "      \"description\": \"The base unit of the asset. Must be in denom_units.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"coingecko_id\": {"]
#[doc = "      \"description\": \"[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"denom_units\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/denom_unit\""]
#[doc = "      },"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"deprecated\": {"]
#[doc = "      \"description\": \"[OPTIONAL] Whether the asset has been deprecated for use. For readability, it is best to omit this property unless TRUE.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"[OPTIONAL] A short description of the asset\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"display\": {"]
#[doc = "      \"description\": \"The human friendly unit of the asset. Must be in denom_units.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extended_description\": {"]
#[doc = "      \"description\": \"[OPTIONAL] A long description of the asset\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ibc\": {"]
#[doc = "      \"description\": \"[OPTIONAL] IBC Channel between src and dst between chain\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"dst_channel\","]
#[doc = "        \"source_channel\","]
#[doc = "        \"source_denom\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"dst_channel\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source_channel\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source_denom\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"image_sync\": {"]
#[doc = "            \"$ref\": \"#/$defs/pointer\""]
#[doc = "          },"]
#[doc = "          \"png\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri-reference\","]
#[doc = "            \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "          },"]
#[doc = "          \"svg\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri-reference\","]
#[doc = "            \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "          },"]
#[doc = "          \"theme\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"minProperties\": 1,"]
#[doc = "            \"properties\": {"]
#[doc = "              \"circle\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"dark_mode\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"primary_color_hex\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"pattern\": \"^#[0-9a-fA-F]{6}$\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"keywords\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"maxContains\": 20,"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"logo_URIs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"png\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri-reference\","]
#[doc = "          \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "        },"]
#[doc = "        \"svg\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri-reference\","]
#[doc = "          \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The project name of the asset. For example Bitcoin.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 42"]
#[doc = "    },"]
#[doc = "    \"socials\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"minProperties\": 1,"]
#[doc = "      \"properties\": {"]
#[doc = "        \"twitter\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        },"]
#[doc = "        \"website\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"description\": \"The symbol of an asset. For example BTC.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"traces\": {"]
#[doc = "      \"description\": \"The origin of the asset, starting with the index, and capturing all transitions in form and location.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ibc_transition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/ibc_cw20_transition\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/non_ibc_transition\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"type_asset\": {"]
#[doc = "      \"description\": \"[OPTIONAL] The potential options for type of asset. By default, assumes sdk.coin\","]
#[doc = "      \"default\": \"sdk.coin\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"sdk.coin\","]
#[doc = "        \"cw20\","]
#[doc = "        \"erc20\","]
#[doc = "        \"ics20\","]
#[doc = "        \"snip20\","]
#[doc = "        \"snip25\","]
#[doc = "        \"bitcoin-like\","]
#[doc = "        \"evm-base\","]
#[doc = "        \"svm-base\","]
#[doc = "        \"substrate\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    #[doc = "[OPTIONAL] The address of the asset. Only required for type_asset : cw20, snip20"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The base unit of the asset. Must be in denom_units."]
    pub base: String,
    #[doc = "[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coingecko_id: Option<String>,
    pub denom_units: Vec<DenomUnit>,
    #[doc = "[OPTIONAL] Whether the asset has been deprecated for use. For readability, it is best to omit this property unless TRUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = "[OPTIONAL] A short description of the asset"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The human friendly unit of the asset. Must be in denom_units."]
    pub display: String,
    #[doc = "[OPTIONAL] A long description of the asset"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibc: Option<AssetIbc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<AssetImagesItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(rename = "logo_URIs", default, skip_serializing_if = "Option::is_none")]
    pub logo_ur_is: Option<AssetLogoUrIs>,
    #[doc = "The project name of the asset. For example Bitcoin."]
    pub name: AssetName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socials: Option<AssetSocials>,
    #[doc = "The symbol of an asset. For example BTC."]
    pub symbol: String,
    #[doc = "The origin of the asset, starting with the index, and capturing all transitions in form and location."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traces: Vec<AssetTracesItem>,
    #[doc = "[OPTIONAL] The potential options for type of asset. By default, assumes sdk.coin"]
    #[serde(default = "defaults::asset_type_asset")]
    pub type_asset: AssetTypeAsset,
}
impl From<&Asset> for Asset {
    fn from(value: &Asset) -> Self {
        value.clone()
    }
}
impl Asset {
    pub fn builder() -> builder::Asset {
        Default::default()
    }
}
#[doc = "[OPTIONAL] IBC Channel between src and dst between chain"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] IBC Channel between src and dst between chain\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dst_channel\","]
#[doc = "    \"source_channel\","]
#[doc = "    \"source_denom\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dst_channel\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_channel\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_denom\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIbc {
    pub dst_channel: String,
    pub source_channel: String,
    pub source_denom: String,
}
impl From<&AssetIbc> for AssetIbc {
    fn from(value: &AssetIbc) -> Self {
        value.clone()
    }
}
impl AssetIbc {
    pub fn builder() -> builder::AssetIbc {
        Default::default()
    }
}
#[doc = "AssetImagesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"image_sync\": {"]
#[doc = "      \"$ref\": \"#/$defs/pointer\""]
#[doc = "    },"]
#[doc = "    \"png\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "    },"]
#[doc = "    \"theme\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"minProperties\": 1,"]
#[doc = "      \"properties\": {"]
#[doc = "        \"circle\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"dark_mode\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"primary_color_hex\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^#[0-9a-fA-F]{6}$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetImagesItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_sync: Option<Pointer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<AssetImagesItemTheme>,
}
impl From<&AssetImagesItem> for AssetImagesItem {
    fn from(value: &AssetImagesItem) -> Self {
        value.clone()
    }
}
impl AssetImagesItem {
    pub fn builder() -> builder::AssetImagesItem {
        Default::default()
    }
}
#[doc = "AssetImagesItemTheme"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"minProperties\": 1,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"circle\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"dark_mode\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"primary_color_hex\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^#[0-9a-fA-F]{6}$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetImagesItemTheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circle: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_color_hex: Option<AssetImagesItemThemePrimaryColorHex>,
}
impl From<&AssetImagesItemTheme> for AssetImagesItemTheme {
    fn from(value: &AssetImagesItemTheme) -> Self {
        value.clone()
    }
}
impl AssetImagesItemTheme {
    pub fn builder() -> builder::AssetImagesItemTheme {
        Default::default()
    }
}
#[doc = "AssetImagesItemThemePrimaryColorHex"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^#[0-9a-fA-F]{6}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetImagesItemThemePrimaryColorHex(String);
impl std::ops::Deref for AssetImagesItemThemePrimaryColorHex {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetImagesItemThemePrimaryColorHex> for String {
    fn from(value: AssetImagesItemThemePrimaryColorHex) -> Self {
        value.0
    }
}
impl From<&AssetImagesItemThemePrimaryColorHex> for AssetImagesItemThemePrimaryColorHex {
    fn from(value: &AssetImagesItemThemePrimaryColorHex) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetImagesItemThemePrimaryColorHex {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^#[0-9a-fA-F]{6}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^#[0-9a-fA-F]{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetImagesItemThemePrimaryColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetImagesItemThemePrimaryColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetImagesItemThemePrimaryColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetImagesItemThemePrimaryColorHex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Asset lists are a similar mechanism to allow frontends and other UIs to fetch metadata associated with Cosmos SDK denoms, especially for assets sent over IBC."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://osmosis.zone/assetlists.schema.json\","]
#[doc = "  \"title\": \"Asset Lists\","]
#[doc = "  \"description\": \"Asset lists are a similar mechanism to allow frontends and other UIs to fetch metadata associated with Cosmos SDK denoms, especially for assets sent over IBC.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assets\","]
#[doc = "    \"chain_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^(\\\\.\\\\./)+assetlist\\\\.schema\\\\.json$\""]
#[doc = "    },"]
#[doc = "    \"assets\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/asset\""]
#[doc = "      },"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetLists {
    pub assets: Vec<Asset>,
    pub chain_name: String,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<AssetListsSchema>,
}
impl From<&AssetLists> for AssetLists {
    fn from(value: &AssetLists) -> Self {
        value.clone()
    }
}
impl AssetLists {
    pub fn builder() -> builder::AssetLists {
        Default::default()
    }
}
#[doc = "AssetListsSchema"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(\\\\.\\\\./)+assetlist\\\\.schema\\\\.json$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetListsSchema(String);
impl std::ops::Deref for AssetListsSchema {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetListsSchema> for String {
    fn from(value: AssetListsSchema) -> Self {
        value.0
    }
}
impl From<&AssetListsSchema> for AssetListsSchema {
    fn from(value: &AssetListsSchema) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetListsSchema {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(\\.\\./)+assetlist\\.schema\\.json$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(\\.\\./)+assetlist\\.schema\\.json$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetListsSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetListsSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetListsSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetListsSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "AssetLogoUrIs"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"png\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetLogoUrIs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
}
impl From<&AssetLogoUrIs> for AssetLogoUrIs {
    fn from(value: &AssetLogoUrIs) -> Self {
        value.clone()
    }
}
impl AssetLogoUrIs {
    pub fn builder() -> builder::AssetLogoUrIs {
        Default::default()
    }
}
#[doc = "The project name of the asset. For example Bitcoin."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The project name of the asset. For example Bitcoin.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 42"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetName(String);
impl std::ops::Deref for AssetName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetName> for String {
    fn from(value: AssetName) -> Self {
        value.0
    }
}
impl From<&AssetName> for AssetName {
    fn from(value: &AssetName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 42usize {
            return Err("longer than 42 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "AssetSocials"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"minProperties\": 1,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"twitter\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"website\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetSocials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl From<&AssetSocials> for AssetSocials {
    fn from(value: &AssetSocials) -> Self {
        value.clone()
    }
}
impl AssetSocials {
    pub fn builder() -> builder::AssetSocials {
        Default::default()
    }
}
#[doc = "AssetTracesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ibc_transition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ibc_cw20_transition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/non_ibc_transition\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AssetTracesItem {
    IbcTransition(IbcTransition),
    IbcCw20Transition(IbcCw20Transition),
    NonIbcTransition(NonIbcTransition),
}
impl From<&AssetTracesItem> for AssetTracesItem {
    fn from(value: &AssetTracesItem) -> Self {
        value.clone()
    }
}
impl From<IbcTransition> for AssetTracesItem {
    fn from(value: IbcTransition) -> Self {
        Self::IbcTransition(value)
    }
}
impl From<IbcCw20Transition> for AssetTracesItem {
    fn from(value: IbcCw20Transition) -> Self {
        Self::IbcCw20Transition(value)
    }
}
impl From<NonIbcTransition> for AssetTracesItem {
    fn from(value: NonIbcTransition) -> Self {
        Self::NonIbcTransition(value)
    }
}
#[doc = "[OPTIONAL] The potential options for type of asset. By default, assumes sdk.coin"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] The potential options for type of asset. By default, assumes sdk.coin\","]
#[doc = "  \"default\": \"sdk.coin\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"sdk.coin\","]
#[doc = "    \"cw20\","]
#[doc = "    \"erc20\","]
#[doc = "    \"ics20\","]
#[doc = "    \"snip20\","]
#[doc = "    \"snip25\","]
#[doc = "    \"bitcoin-like\","]
#[doc = "    \"evm-base\","]
#[doc = "    \"svm-base\","]
#[doc = "    \"substrate\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AssetTypeAsset {
    #[serde(rename = "sdk.coin")]
    SdkCoin,
    #[serde(rename = "cw20")]
    Cw20,
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "ics20")]
    Ics20,
    #[serde(rename = "snip20")]
    Snip20,
    #[serde(rename = "snip25")]
    Snip25,
    #[serde(rename = "bitcoin-like")]
    BitcoinLike,
    #[serde(rename = "evm-base")]
    EvmBase,
    #[serde(rename = "svm-base")]
    SvmBase,
    #[serde(rename = "substrate")]
    Substrate,
}
impl From<&AssetTypeAsset> for AssetTypeAsset {
    fn from(value: &AssetTypeAsset) -> Self {
        value.clone()
    }
}
impl ToString for AssetTypeAsset {
    fn to_string(&self) -> String {
        match *self {
            Self::SdkCoin => "sdk.coin".to_string(),
            Self::Cw20 => "cw20".to_string(),
            Self::Erc20 => "erc20".to_string(),
            Self::Ics20 => "ics20".to_string(),
            Self::Snip20 => "snip20".to_string(),
            Self::Snip25 => "snip25".to_string(),
            Self::BitcoinLike => "bitcoin-like".to_string(),
            Self::EvmBase => "evm-base".to_string(),
            Self::SvmBase => "svm-base".to_string(),
            Self::Substrate => "substrate".to_string(),
        }
    }
}
impl std::str::FromStr for AssetTypeAsset {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "sdk.coin" => Ok(Self::SdkCoin),
            "cw20" => Ok(Self::Cw20),
            "erc20" => Ok(Self::Erc20),
            "ics20" => Ok(Self::Ics20),
            "snip20" => Ok(Self::Snip20),
            "snip25" => Ok(Self::Snip25),
            "bitcoin-like" => Ok(Self::BitcoinLike),
            "evm-base" => Ok(Self::EvmBase),
            "svm-base" => Ok(Self::SvmBase),
            "substrate" => Ok(Self::Substrate),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for AssetTypeAsset {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetTypeAsset {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetTypeAsset {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for AssetTypeAsset {
    fn default() -> Self {
        AssetTypeAsset::SdkCoin
    }
}
#[doc = "DenomUnit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"denom\","]
#[doc = "    \"exponent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"denom\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"exponent\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DenomUnit {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    pub denom: String,
    pub exponent: u32,
}
impl From<&DenomUnit> for DenomUnit {
    fn from(value: &DenomUnit) -> Self {
        value.clone()
    }
}
impl DenomUnit {
    pub fn builder() -> builder::DenomUnit {
        Default::default()
    }
}
#[doc = "IbcCw20Transition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"chain\","]
#[doc = "    \"counterparty\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"chain\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"channel_id\","]
#[doc = "        \"path\","]
#[doc = "        \"port\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"counterparty\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"base_denom\","]
#[doc = "        \"chain_name\","]
#[doc = "        \"channel_id\","]
#[doc = "        \"port\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base_denom\": {"]
#[doc = "          \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ibc-cw20\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcCw20Transition {
    pub chain: IbcCw20TransitionChain,
    pub counterparty: IbcCw20TransitionCounterparty,
    #[serde(rename = "type")]
    pub type_: IbcCw20TransitionType,
}
impl From<&IbcCw20Transition> for IbcCw20Transition {
    fn from(value: &IbcCw20Transition) -> Self {
        value.clone()
    }
}
impl IbcCw20Transition {
    pub fn builder() -> builder::IbcCw20Transition {
        Default::default()
    }
}
#[doc = "IbcCw20TransitionChain"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel_id\","]
#[doc = "    \"path\","]
#[doc = "    \"port\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcCw20TransitionChain {
    #[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcCw20TransitionChainChannelId,
    #[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
    pub path: String,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    pub port: String,
}
impl From<&IbcCw20TransitionChain> for IbcCw20TransitionChain {
    fn from(value: &IbcCw20TransitionChain) -> Self {
        value.clone()
    }
}
impl IbcCw20TransitionChain {
    pub fn builder() -> builder::IbcCw20TransitionChain {
        Default::default()
    }
}
#[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionChainChannelId(String);
impl std::ops::Deref for IbcCw20TransitionChainChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionChainChannelId> for String {
    fn from(value: IbcCw20TransitionChainChannelId) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionChainChannelId> for IbcCw20TransitionChainChannelId {
    fn from(value: &IbcCw20TransitionChainChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionChainChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^channel-\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^channel-\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionChainChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "IbcCw20TransitionCounterparty"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base_denom\","]
#[doc = "    \"chain_name\","]
#[doc = "    \"channel_id\","]
#[doc = "    \"port\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_denom\": {"]
#[doc = "      \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcCw20TransitionCounterparty {
    #[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    pub base_denom: String,
    #[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
    pub chain_name: String,
    #[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcCw20TransitionCounterpartyChannelId,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    pub port: String,
}
impl From<&IbcCw20TransitionCounterparty> for IbcCw20TransitionCounterparty {
    fn from(value: &IbcCw20TransitionCounterparty) -> Self {
        value.clone()
    }
}
impl IbcCw20TransitionCounterparty {
    pub fn builder() -> builder::IbcCw20TransitionCounterparty {
        Default::default()
    }
}
#[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionCounterpartyChannelId(String);
impl std::ops::Deref for IbcCw20TransitionCounterpartyChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionCounterpartyChannelId> for String {
    fn from(value: IbcCw20TransitionCounterpartyChannelId) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionCounterpartyChannelId> for IbcCw20TransitionCounterpartyChannelId {
    fn from(value: &IbcCw20TransitionCounterpartyChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionCounterpartyChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^channel-\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^channel-\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionCounterpartyChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "IbcCw20TransitionType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ibc-cw20\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IbcCw20TransitionType {
    #[serde(rename = "ibc-cw20")]
    IbcCw20,
}
impl From<&IbcCw20TransitionType> for IbcCw20TransitionType {
    fn from(value: &IbcCw20TransitionType) -> Self {
        value.clone()
    }
}
impl ToString for IbcCw20TransitionType {
    fn to_string(&self) -> String {
        match *self {
            Self::IbcCw20 => "ibc-cw20".to_string(),
        }
    }
}
impl std::str::FromStr for IbcCw20TransitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ibc-cw20" => Ok(Self::IbcCw20),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IbcTransition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"chain\","]
#[doc = "    \"counterparty\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"chain\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"channel_id\","]
#[doc = "        \"path\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"counterparty\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"base_denom\","]
#[doc = "        \"chain_name\","]
#[doc = "        \"channel_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base_denom\": {"]
#[doc = "          \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^channel-(JEnb|\\\\d+)$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ibc\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcTransition {
    pub chain: IbcTransitionChain,
    pub counterparty: IbcTransitionCounterparty,
    #[serde(rename = "type")]
    pub type_: IbcTransitionType,
}
impl From<&IbcTransition> for IbcTransition {
    fn from(value: &IbcTransition) -> Self {
        value.clone()
    }
}
impl IbcTransition {
    pub fn builder() -> builder::IbcTransition {
        Default::default()
    }
}
#[doc = "IbcTransitionChain"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel_id\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcTransitionChain {
    #[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcTransitionChainChannelId,
    #[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
    pub path: String,
}
impl From<&IbcTransitionChain> for IbcTransitionChain {
    fn from(value: &IbcTransitionChain) -> Self {
        value.clone()
    }
}
impl IbcTransitionChain {
    pub fn builder() -> builder::IbcTransitionChain {
        Default::default()
    }
}
#[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The chain's IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcTransitionChainChannelId(String);
impl std::ops::Deref for IbcTransitionChainChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcTransitionChainChannelId> for String {
    fn from(value: IbcTransitionChainChannelId) -> Self {
        value.0
    }
}
impl From<&IbcTransitionChainChannelId> for IbcTransitionChainChannelId {
    fn from(value: &IbcTransitionChainChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcTransitionChainChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^channel-\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^channel-\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcTransitionChainChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "IbcTransitionCounterparty"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base_denom\","]
#[doc = "    \"chain_name\","]
#[doc = "    \"channel_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_denom\": {"]
#[doc = "      \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^channel-(JEnb|\\\\d+)$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IbcTransitionCounterparty {
    #[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    pub base_denom: String,
    #[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
    pub chain_name: String,
    #[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcTransitionCounterpartyChannelId,
}
impl From<&IbcTransitionCounterparty> for IbcTransitionCounterparty {
    fn from(value: &IbcTransitionCounterparty) -> Self {
        value.clone()
    }
}
impl IbcTransitionCounterparty {
    pub fn builder() -> builder::IbcTransitionCounterparty {
        Default::default()
    }
}
#[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^channel-(JEnb|\\\\d+)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcTransitionCounterpartyChannelId(String);
impl std::ops::Deref for IbcTransitionCounterpartyChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcTransitionCounterpartyChannelId> for String {
    fn from(value: IbcTransitionCounterpartyChannelId) -> Self {
        value.0
    }
}
impl From<&IbcTransitionCounterpartyChannelId> for IbcTransitionCounterpartyChannelId {
    fn from(value: &IbcTransitionCounterpartyChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcTransitionCounterpartyChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^channel-(JEnb|\\d+)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^channel-(JEnb|\\d+)$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcTransitionCounterpartyChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "IbcTransitionType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ibc\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IbcTransitionType {
    #[serde(rename = "ibc")]
    Ibc,
}
impl From<&IbcTransitionType> for IbcTransitionType {
    fn from(value: &IbcTransitionType) -> Self {
        value.clone()
    }
}
impl ToString for IbcTransitionType {
    fn to_string(&self) -> String {
        match *self {
            Self::Ibc => "ibc".to_string(),
        }
    }
}
impl std::str::FromStr for IbcTransitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ibc" => Ok(Self::Ibc),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "NonIbcTransition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"counterparty\","]
#[doc = "    \"provider\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"chain\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"contract\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"contract\": {"]
#[doc = "          \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"counterparty\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"base_denom\","]
#[doc = "        \"chain_name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"base_denom\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"contract\": {"]
#[doc = "          \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"description\": \"The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company].\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"bridge\","]
#[doc = "        \"liquid-stake\","]
#[doc = "        \"synthetic\","]
#[doc = "        \"wrapped\","]
#[doc = "        \"additional-mintage\","]
#[doc = "        \"test-mintage\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonIbcTransition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain: Option<NonIbcTransitionChain>,
    pub counterparty: NonIbcTransitionCounterparty,
    #[doc = "The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company]."]
    pub provider: String,
    #[serde(rename = "type")]
    pub type_: NonIbcTransitionType,
}
impl From<&NonIbcTransition> for NonIbcTransition {
    fn from(value: &NonIbcTransition) -> Self {
        value.clone()
    }
}
impl NonIbcTransition {
    pub fn builder() -> builder::NonIbcTransition {
        Default::default()
    }
}
#[doc = "NonIbcTransitionChain"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contract\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contract\": {"]
#[doc = "      \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonIbcTransitionChain {
    #[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
    pub contract: String,
}
impl From<&NonIbcTransitionChain> for NonIbcTransitionChain {
    fn from(value: &NonIbcTransitionChain) -> Self {
        value.clone()
    }
}
impl NonIbcTransitionChain {
    pub fn builder() -> builder::NonIbcTransitionChain {
        Default::default()
    }
}
#[doc = "NonIbcTransitionCounterparty"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"base_denom\","]
#[doc = "    \"chain_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_denom\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"contract\": {"]
#[doc = "      \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonIbcTransitionCounterparty {
    pub base_denom: String,
    #[doc = "The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
    pub chain_name: String,
    #[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}
impl From<&NonIbcTransitionCounterparty> for NonIbcTransitionCounterparty {
    fn from(value: &NonIbcTransitionCounterparty) -> Self {
        value.clone()
    }
}
impl NonIbcTransitionCounterparty {
    pub fn builder() -> builder::NonIbcTransitionCounterparty {
        Default::default()
    }
}
#[doc = "NonIbcTransitionType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bridge\","]
#[doc = "    \"liquid-stake\","]
#[doc = "    \"synthetic\","]
#[doc = "    \"wrapped\","]
#[doc = "    \"additional-mintage\","]
#[doc = "    \"test-mintage\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NonIbcTransitionType {
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "liquid-stake")]
    LiquidStake,
    #[serde(rename = "synthetic")]
    Synthetic,
    #[serde(rename = "wrapped")]
    Wrapped,
    #[serde(rename = "additional-mintage")]
    AdditionalMintage,
    #[serde(rename = "test-mintage")]
    TestMintage,
}
impl From<&NonIbcTransitionType> for NonIbcTransitionType {
    fn from(value: &NonIbcTransitionType) -> Self {
        value.clone()
    }
}
impl ToString for NonIbcTransitionType {
    fn to_string(&self) -> String {
        match *self {
            Self::Bridge => "bridge".to_string(),
            Self::LiquidStake => "liquid-stake".to_string(),
            Self::Synthetic => "synthetic".to_string(),
            Self::Wrapped => "wrapped".to_string(),
            Self::AdditionalMintage => "additional-mintage".to_string(),
            Self::TestMintage => "test-mintage".to_string(),
        }
    }
}
impl std::str::FromStr for NonIbcTransitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bridge" => Ok(Self::Bridge),
            "liquid-stake" => Ok(Self::LiquidStake),
            "synthetic" => Ok(Self::Synthetic),
            "wrapped" => Ok(Self::Wrapped),
            "additional-mintage" => Ok(Self::AdditionalMintage),
            "test-mintage" => Ok(Self::TestMintage),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The (primary) key used to identify an object within the Chain Registry."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The (primary) key used to identify an object within the Chain Registry.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"chain_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base_denom\": {"]
#[doc = "      \"description\": \"The base denom of the asset from which the object originates. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pointer {
    #[doc = "The base denom of the asset from which the object originates. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_denom: Option<String>,
    #[doc = "The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
    pub chain_name: String,
}
impl From<&Pointer> for Pointer {
    fn from(value: &Pointer) -> Self {
        value.clone()
    }
}
impl Pointer {
    pub fn builder() -> builder::Pointer {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Asset {
        address: Result<Option<String>, String>,
        base: Result<String, String>,
        coingecko_id: Result<Option<String>, String>,
        denom_units: Result<Vec<super::DenomUnit>, String>,
        deprecated: Result<Option<bool>, String>,
        description: Result<Option<String>, String>,
        display: Result<String, String>,
        extended_description: Result<Option<String>, String>,
        ibc: Result<Option<super::AssetIbc>, String>,
        images: Result<Vec<super::AssetImagesItem>, String>,
        keywords: Result<Vec<String>, String>,
        logo_ur_is: Result<Option<super::AssetLogoUrIs>, String>,
        name: Result<super::AssetName, String>,
        socials: Result<Option<super::AssetSocials>, String>,
        symbol: Result<String, String>,
        traces: Result<Vec<super::AssetTracesItem>, String>,
        type_asset: Result<super::AssetTypeAsset, String>,
    }
    impl Default for Asset {
        fn default() -> Self {
            Self {
                address: Ok(Default::default()),
                base: Err("no value supplied for base".to_string()),
                coingecko_id: Ok(Default::default()),
                denom_units: Err("no value supplied for denom_units".to_string()),
                deprecated: Ok(Default::default()),
                description: Ok(Default::default()),
                display: Err("no value supplied for display".to_string()),
                extended_description: Ok(Default::default()),
                ibc: Ok(Default::default()),
                images: Ok(Default::default()),
                keywords: Ok(Default::default()),
                logo_ur_is: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                socials: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
                traces: Ok(Default::default()),
                type_asset: Ok(super::defaults::asset_type_asset()),
            }
        }
    }
    impl Asset {
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address: {}", e));
            self
        }
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn coingecko_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.coingecko_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coingecko_id: {}", e));
            self
        }
        pub fn denom_units<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::DenomUnit>>,
            T::Error: std::fmt::Display,
        {
            self.denom_units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for denom_units: {}", e));
            self
        }
        pub fn deprecated<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.deprecated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deprecated: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn display<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.display = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display: {}", e));
            self
        }
        pub fn extended_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.extended_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for extended_description: {}",
                    e
                )
            });
            self
        }
        pub fn ibc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetIbc>>,
            T::Error: std::fmt::Display,
        {
            self.ibc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ibc: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AssetImagesItem>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {}", e));
            self
        }
        pub fn logo_ur_is<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetLogoUrIs>>,
            T::Error: std::fmt::Display,
        {
            self.logo_ur_is = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo_ur_is: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AssetName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn socials<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetSocials>>,
            T::Error: std::fmt::Display,
        {
            self.socials = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for socials: {}", e));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {}", e));
            self
        }
        pub fn traces<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AssetTracesItem>>,
            T::Error: std::fmt::Display,
        {
            self.traces = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traces: {}", e));
            self
        }
        pub fn type_asset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AssetTypeAsset>,
            T::Error: std::fmt::Display,
        {
            self.type_asset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_asset: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Asset> for super::Asset {
        type Error = super::error::ConversionError;
        fn try_from(value: Asset) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                address: value.address?,
                base: value.base?,
                coingecko_id: value.coingecko_id?,
                denom_units: value.denom_units?,
                deprecated: value.deprecated?,
                description: value.description?,
                display: value.display?,
                extended_description: value.extended_description?,
                ibc: value.ibc?,
                images: value.images?,
                keywords: value.keywords?,
                logo_ur_is: value.logo_ur_is?,
                name: value.name?,
                socials: value.socials?,
                symbol: value.symbol?,
                traces: value.traces?,
                type_asset: value.type_asset?,
            })
        }
    }
    impl From<super::Asset> for Asset {
        fn from(value: super::Asset) -> Self {
            Self {
                address: Ok(value.address),
                base: Ok(value.base),
                coingecko_id: Ok(value.coingecko_id),
                denom_units: Ok(value.denom_units),
                deprecated: Ok(value.deprecated),
                description: Ok(value.description),
                display: Ok(value.display),
                extended_description: Ok(value.extended_description),
                ibc: Ok(value.ibc),
                images: Ok(value.images),
                keywords: Ok(value.keywords),
                logo_ur_is: Ok(value.logo_ur_is),
                name: Ok(value.name),
                socials: Ok(value.socials),
                symbol: Ok(value.symbol),
                traces: Ok(value.traces),
                type_asset: Ok(value.type_asset),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetIbc {
        dst_channel: Result<String, String>,
        source_channel: Result<String, String>,
        source_denom: Result<String, String>,
    }
    impl Default for AssetIbc {
        fn default() -> Self {
            Self {
                dst_channel: Err("no value supplied for dst_channel".to_string()),
                source_channel: Err("no value supplied for source_channel".to_string()),
                source_denom: Err("no value supplied for source_denom".to_string()),
            }
        }
    }
    impl AssetIbc {
        pub fn dst_channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.dst_channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dst_channel: {}", e));
            self
        }
        pub fn source_channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.source_channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_channel: {}", e));
            self
        }
        pub fn source_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.source_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_denom: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetIbc> for super::AssetIbc {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetIbc) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                dst_channel: value.dst_channel?,
                source_channel: value.source_channel?,
                source_denom: value.source_denom?,
            })
        }
    }
    impl From<super::AssetIbc> for AssetIbc {
        fn from(value: super::AssetIbc) -> Self {
            Self {
                dst_channel: Ok(value.dst_channel),
                source_channel: Ok(value.source_channel),
                source_denom: Ok(value.source_denom),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetImagesItem {
        image_sync: Result<Option<super::Pointer>, String>,
        png: Result<Option<String>, String>,
        svg: Result<Option<String>, String>,
        theme: Result<Option<super::AssetImagesItemTheme>, String>,
    }
    impl Default for AssetImagesItem {
        fn default() -> Self {
            Self {
                image_sync: Ok(Default::default()),
                png: Ok(Default::default()),
                svg: Ok(Default::default()),
                theme: Ok(Default::default()),
            }
        }
    }
    impl AssetImagesItem {
        pub fn image_sync<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Pointer>>,
            T::Error: std::fmt::Display,
        {
            self.image_sync = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_sync: {}", e));
            self
        }
        pub fn png<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.png = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for png: {}", e));
            self
        }
        pub fn svg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.svg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for svg: {}", e));
            self
        }
        pub fn theme<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetImagesItemTheme>>,
            T::Error: std::fmt::Display,
        {
            self.theme = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for theme: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetImagesItem> for super::AssetImagesItem {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetImagesItem) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_sync: value.image_sync?,
                png: value.png?,
                svg: value.svg?,
                theme: value.theme?,
            })
        }
    }
    impl From<super::AssetImagesItem> for AssetImagesItem {
        fn from(value: super::AssetImagesItem) -> Self {
            Self {
                image_sync: Ok(value.image_sync),
                png: Ok(value.png),
                svg: Ok(value.svg),
                theme: Ok(value.theme),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetImagesItemTheme {
        circle: Result<Option<bool>, String>,
        dark_mode: Result<Option<bool>, String>,
        primary_color_hex: Result<Option<super::AssetImagesItemThemePrimaryColorHex>, String>,
    }
    impl Default for AssetImagesItemTheme {
        fn default() -> Self {
            Self {
                circle: Ok(Default::default()),
                dark_mode: Ok(Default::default()),
                primary_color_hex: Ok(Default::default()),
            }
        }
    }
    impl AssetImagesItemTheme {
        pub fn circle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.circle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for circle: {}", e));
            self
        }
        pub fn dark_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.dark_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dark_mode: {}", e));
            self
        }
        pub fn primary_color_hex<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetImagesItemThemePrimaryColorHex>>,
            T::Error: std::fmt::Display,
        {
            self.primary_color_hex = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for primary_color_hex: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<AssetImagesItemTheme> for super::AssetImagesItemTheme {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetImagesItemTheme) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                circle: value.circle?,
                dark_mode: value.dark_mode?,
                primary_color_hex: value.primary_color_hex?,
            })
        }
    }
    impl From<super::AssetImagesItemTheme> for AssetImagesItemTheme {
        fn from(value: super::AssetImagesItemTheme) -> Self {
            Self {
                circle: Ok(value.circle),
                dark_mode: Ok(value.dark_mode),
                primary_color_hex: Ok(value.primary_color_hex),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetLists {
        assets: Result<Vec<super::Asset>, String>,
        chain_name: Result<String, String>,
        schema: Result<Option<super::AssetListsSchema>, String>,
    }
    impl Default for AssetLists {
        fn default() -> Self {
            Self {
                assets: Err("no value supplied for assets".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                schema: Ok(Default::default()),
            }
        }
    }
    impl AssetLists {
        pub fn assets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Asset>>,
            T::Error: std::fmt::Display,
        {
            self.assets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assets: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetListsSchema>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetLists> for super::AssetLists {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetLists) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                assets: value.assets?,
                chain_name: value.chain_name?,
                schema: value.schema?,
            })
        }
    }
    impl From<super::AssetLists> for AssetLists {
        fn from(value: super::AssetLists) -> Self {
            Self {
                assets: Ok(value.assets),
                chain_name: Ok(value.chain_name),
                schema: Ok(value.schema),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetLogoUrIs {
        png: Result<Option<String>, String>,
        svg: Result<Option<String>, String>,
    }
    impl Default for AssetLogoUrIs {
        fn default() -> Self {
            Self {
                png: Ok(Default::default()),
                svg: Ok(Default::default()),
            }
        }
    }
    impl AssetLogoUrIs {
        pub fn png<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.png = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for png: {}", e));
            self
        }
        pub fn svg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.svg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for svg: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetLogoUrIs> for super::AssetLogoUrIs {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetLogoUrIs) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                png: value.png?,
                svg: value.svg?,
            })
        }
    }
    impl From<super::AssetLogoUrIs> for AssetLogoUrIs {
        fn from(value: super::AssetLogoUrIs) -> Self {
            Self {
                png: Ok(value.png),
                svg: Ok(value.svg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetSocials {
        twitter: Result<Option<String>, String>,
        website: Result<Option<String>, String>,
    }
    impl Default for AssetSocials {
        fn default() -> Self {
            Self {
                twitter: Ok(Default::default()),
                website: Ok(Default::default()),
            }
        }
    }
    impl AssetSocials {
        pub fn twitter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.twitter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for twitter: {}", e));
            self
        }
        pub fn website<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.website = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for website: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetSocials> for super::AssetSocials {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetSocials) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                twitter: value.twitter?,
                website: value.website?,
            })
        }
    }
    impl From<super::AssetSocials> for AssetSocials {
        fn from(value: super::AssetSocials) -> Self {
            Self {
                twitter: Ok(value.twitter),
                website: Ok(value.website),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DenomUnit {
        aliases: Result<Vec<String>, String>,
        denom: Result<String, String>,
        exponent: Result<u32, String>,
    }
    impl Default for DenomUnit {
        fn default() -> Self {
            Self {
                aliases: Ok(Default::default()),
                denom: Err("no value supplied for denom".to_string()),
                exponent: Err("no value supplied for exponent".to_string()),
            }
        }
    }
    impl DenomUnit {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for denom: {}", e));
            self
        }
        pub fn exponent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<u32>,
            T::Error: std::fmt::Display,
        {
            self.exponent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exponent: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DenomUnit> for super::DenomUnit {
        type Error = super::error::ConversionError;
        fn try_from(value: DenomUnit) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                denom: value.denom?,
                exponent: value.exponent?,
            })
        }
    }
    impl From<super::DenomUnit> for DenomUnit {
        fn from(value: super::DenomUnit) -> Self {
            Self {
                aliases: Ok(value.aliases),
                denom: Ok(value.denom),
                exponent: Ok(value.exponent),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcCw20Transition {
        chain: Result<super::IbcCw20TransitionChain, String>,
        counterparty: Result<super::IbcCw20TransitionCounterparty, String>,
        type_: Result<super::IbcCw20TransitionType, String>,
    }
    impl Default for IbcCw20Transition {
        fn default() -> Self {
            Self {
                chain: Err("no value supplied for chain".to_string()),
                counterparty: Err("no value supplied for counterparty".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IbcCw20Transition {
        pub fn chain<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionChain>,
            T::Error: std::fmt::Display,
        {
            self.chain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain: {}", e));
            self
        }
        pub fn counterparty<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionCounterparty>,
            T::Error: std::fmt::Display,
        {
            self.counterparty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for counterparty: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcCw20Transition> for super::IbcCw20Transition {
        type Error = super::error::ConversionError;
        fn try_from(value: IbcCw20Transition) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                chain: value.chain?,
                counterparty: value.counterparty?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IbcCw20Transition> for IbcCw20Transition {
        fn from(value: super::IbcCw20Transition) -> Self {
            Self {
                chain: Ok(value.chain),
                counterparty: Ok(value.counterparty),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcCw20TransitionChain {
        channel_id: Result<super::IbcCw20TransitionChainChannelId, String>,
        path: Result<String, String>,
        port: Result<String, String>,
    }
    impl Default for IbcCw20TransitionChain {
        fn default() -> Self {
            Self {
                channel_id: Err("no value supplied for channel_id".to_string()),
                path: Err("no value supplied for path".to_string()),
                port: Err("no value supplied for port".to_string()),
            }
        }
    }
    impl IbcCw20TransitionChain {
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionChainChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcCw20TransitionChain> for super::IbcCw20TransitionChain {
        type Error = super::error::ConversionError;
        fn try_from(value: IbcCw20TransitionChain) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel_id: value.channel_id?,
                path: value.path?,
                port: value.port?,
            })
        }
    }
    impl From<super::IbcCw20TransitionChain> for IbcCw20TransitionChain {
        fn from(value: super::IbcCw20TransitionChain) -> Self {
            Self {
                channel_id: Ok(value.channel_id),
                path: Ok(value.path),
                port: Ok(value.port),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcCw20TransitionCounterparty {
        base_denom: Result<String, String>,
        chain_name: Result<String, String>,
        channel_id: Result<super::IbcCw20TransitionCounterpartyChannelId, String>,
        port: Result<String, String>,
    }
    impl Default for IbcCw20TransitionCounterparty {
        fn default() -> Self {
            Self {
                base_denom: Err("no value supplied for base_denom".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                channel_id: Err("no value supplied for channel_id".to_string()),
                port: Err("no value supplied for port".to_string()),
            }
        }
    }
    impl IbcCw20TransitionCounterparty {
        pub fn base_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionCounterpartyChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcCw20TransitionCounterparty> for super::IbcCw20TransitionCounterparty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IbcCw20TransitionCounterparty,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_denom: value.base_denom?,
                chain_name: value.chain_name?,
                channel_id: value.channel_id?,
                port: value.port?,
            })
        }
    }
    impl From<super::IbcCw20TransitionCounterparty> for IbcCw20TransitionCounterparty {
        fn from(value: super::IbcCw20TransitionCounterparty) -> Self {
            Self {
                base_denom: Ok(value.base_denom),
                chain_name: Ok(value.chain_name),
                channel_id: Ok(value.channel_id),
                port: Ok(value.port),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcTransition {
        chain: Result<super::IbcTransitionChain, String>,
        counterparty: Result<super::IbcTransitionCounterparty, String>,
        type_: Result<super::IbcTransitionType, String>,
    }
    impl Default for IbcTransition {
        fn default() -> Self {
            Self {
                chain: Err("no value supplied for chain".to_string()),
                counterparty: Err("no value supplied for counterparty".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IbcTransition {
        pub fn chain<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionChain>,
            T::Error: std::fmt::Display,
        {
            self.chain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain: {}", e));
            self
        }
        pub fn counterparty<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionCounterparty>,
            T::Error: std::fmt::Display,
        {
            self.counterparty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for counterparty: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcTransition> for super::IbcTransition {
        type Error = super::error::ConversionError;
        fn try_from(value: IbcTransition) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                chain: value.chain?,
                counterparty: value.counterparty?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IbcTransition> for IbcTransition {
        fn from(value: super::IbcTransition) -> Self {
            Self {
                chain: Ok(value.chain),
                counterparty: Ok(value.counterparty),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcTransitionChain {
        channel_id: Result<super::IbcTransitionChainChannelId, String>,
        path: Result<String, String>,
    }
    impl Default for IbcTransitionChain {
        fn default() -> Self {
            Self {
                channel_id: Err("no value supplied for channel_id".to_string()),
                path: Err("no value supplied for path".to_string()),
            }
        }
    }
    impl IbcTransitionChain {
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionChainChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcTransitionChain> for super::IbcTransitionChain {
        type Error = super::error::ConversionError;
        fn try_from(value: IbcTransitionChain) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel_id: value.channel_id?,
                path: value.path?,
            })
        }
    }
    impl From<super::IbcTransitionChain> for IbcTransitionChain {
        fn from(value: super::IbcTransitionChain) -> Self {
            Self {
                channel_id: Ok(value.channel_id),
                path: Ok(value.path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcTransitionCounterparty {
        base_denom: Result<String, String>,
        chain_name: Result<String, String>,
        channel_id: Result<super::IbcTransitionCounterpartyChannelId, String>,
    }
    impl Default for IbcTransitionCounterparty {
        fn default() -> Self {
            Self {
                base_denom: Err("no value supplied for base_denom".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                channel_id: Err("no value supplied for channel_id".to_string()),
            }
        }
    }
    impl IbcTransitionCounterparty {
        pub fn base_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionCounterpartyChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcTransitionCounterparty> for super::IbcTransitionCounterparty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IbcTransitionCounterparty,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_denom: value.base_denom?,
                chain_name: value.chain_name?,
                channel_id: value.channel_id?,
            })
        }
    }
    impl From<super::IbcTransitionCounterparty> for IbcTransitionCounterparty {
        fn from(value: super::IbcTransitionCounterparty) -> Self {
            Self {
                base_denom: Ok(value.base_denom),
                chain_name: Ok(value.chain_name),
                channel_id: Ok(value.channel_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NonIbcTransition {
        chain: Result<Option<super::NonIbcTransitionChain>, String>,
        counterparty: Result<super::NonIbcTransitionCounterparty, String>,
        provider: Result<String, String>,
        type_: Result<super::NonIbcTransitionType, String>,
    }
    impl Default for NonIbcTransition {
        fn default() -> Self {
            Self {
                chain: Ok(Default::default()),
                counterparty: Err("no value supplied for counterparty".to_string()),
                provider: Err("no value supplied for provider".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl NonIbcTransition {
        pub fn chain<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonIbcTransitionChain>>,
            T::Error: std::fmt::Display,
        {
            self.chain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain: {}", e));
            self
        }
        pub fn counterparty<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NonIbcTransitionCounterparty>,
            T::Error: std::fmt::Display,
        {
            self.counterparty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for counterparty: {}", e));
            self
        }
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NonIbcTransitionType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NonIbcTransition> for super::NonIbcTransition {
        type Error = super::error::ConversionError;
        fn try_from(value: NonIbcTransition) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                chain: value.chain?,
                counterparty: value.counterparty?,
                provider: value.provider?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::NonIbcTransition> for NonIbcTransition {
        fn from(value: super::NonIbcTransition) -> Self {
            Self {
                chain: Ok(value.chain),
                counterparty: Ok(value.counterparty),
                provider: Ok(value.provider),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NonIbcTransitionChain {
        contract: Result<String, String>,
    }
    impl Default for NonIbcTransitionChain {
        fn default() -> Self {
            Self {
                contract: Err("no value supplied for contract".to_string()),
            }
        }
    }
    impl NonIbcTransitionChain {
        pub fn contract<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.contract = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contract: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NonIbcTransitionChain> for super::NonIbcTransitionChain {
        type Error = super::error::ConversionError;
        fn try_from(value: NonIbcTransitionChain) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                contract: value.contract?,
            })
        }
    }
    impl From<super::NonIbcTransitionChain> for NonIbcTransitionChain {
        fn from(value: super::NonIbcTransitionChain) -> Self {
            Self {
                contract: Ok(value.contract),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NonIbcTransitionCounterparty {
        base_denom: Result<String, String>,
        chain_name: Result<String, String>,
        contract: Result<Option<String>, String>,
    }
    impl Default for NonIbcTransitionCounterparty {
        fn default() -> Self {
            Self {
                base_denom: Err("no value supplied for base_denom".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                contract: Ok(Default::default()),
            }
        }
    }
    impl NonIbcTransitionCounterparty {
        pub fn base_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn contract<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.contract = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contract: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NonIbcTransitionCounterparty> for super::NonIbcTransitionCounterparty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NonIbcTransitionCounterparty,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_denom: value.base_denom?,
                chain_name: value.chain_name?,
                contract: value.contract?,
            })
        }
    }
    impl From<super::NonIbcTransitionCounterparty> for NonIbcTransitionCounterparty {
        fn from(value: super::NonIbcTransitionCounterparty) -> Self {
            Self {
                base_denom: Ok(value.base_denom),
                chain_name: Ok(value.chain_name),
                contract: Ok(value.contract),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pointer {
        base_denom: Result<Option<String>, String>,
        chain_name: Result<String, String>,
    }
    impl Default for Pointer {
        fn default() -> Self {
            Self {
                base_denom: Ok(Default::default()),
                chain_name: Err("no value supplied for chain_name".to_string()),
            }
        }
    }
    impl Pointer {
        pub fn base_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Pointer> for super::Pointer {
        type Error = super::error::ConversionError;
        fn try_from(value: Pointer) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_denom: value.base_denom?,
                chain_name: value.chain_name?,
            })
        }
    }
    impl From<super::Pointer> for Pointer {
        fn from(value: super::Pointer) -> Self {
            Self {
                base_denom: Ok(value.base_denom),
                chain_name: Ok(value.chain_name),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn asset_type_asset() -> super::AssetTypeAsset {
        super::AssetTypeAsset::SdkCoin
    }
}
