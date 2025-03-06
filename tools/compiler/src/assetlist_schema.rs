// Generated from cosmos assetlist schema: https://github.com/cosmos/chain-registry/blob/master/assetlist.schema.json
// Via Typify: https://github.com/oxidecomputer/typify
// === NOTE ===
// Three manual edits have been made:
// - This documentation at the top
// - Removal of the if/then clause within the schema. Typify does not currently support it.
// - Removing the few clippy allowances previously for #![allow(clippy::all)] as there are
//   a number of things that get triggered.

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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"base\": {"]
#[doc = "      \"description\": \"The base unit of the asset. Must be in denom_units.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"coingecko_id\": {"]
#[doc = "      \"description\": \"[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"display\": {"]
#[doc = "      \"description\": \"The human friendly unit of the asset. Must be in denom_units.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"extended_description\": {"]
#[doc = "      \"description\": \"[OPTIONAL] A long description of the asset\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
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
#[doc = "            \"minLength\": 1,"]
#[doc = "            \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "          },"]
#[doc = "          \"svg\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri-reference\","]
#[doc = "            \"minLength\": 1,"]
#[doc = "            \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "          },"]
#[doc = "          \"theme\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"minProperties\": 1,"]
#[doc = "            \"properties\": {"]
#[doc = "              \"background_color_hex\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"minLength\": 1,"]
#[doc = "                \"pattern\": \"^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$\""]
#[doc = "              },"]
#[doc = "              \"circle\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"dark_mode\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"monochrome\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"primary_color_hex\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"minLength\": 1,"]
#[doc = "                \"pattern\": \"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$\""]
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
#[doc = "        \"type\": \"string\","]
#[doc = "        \"minLength\": 1"]
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
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "        },"]
#[doc = "        \"svg\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri-reference\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The project name of the asset. For example Bitcoin.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 42,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"socials\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"minProperties\": 1,"]
#[doc = "      \"properties\": {"]
#[doc = "        \"twitter\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"website\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"description\": \"The symbol of an asset. For example BTC.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
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
#[doc = "            \"$ref\": \"#/$defs/ibc_bridge_transition\""]
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
#[doc = "        \"substrate\","]
#[doc = "        \"unknown\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    #[doc = "[OPTIONAL] The address of the asset. Only required for type_asset : cw20, snip20"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AssetAddress>,
    #[doc = "The base unit of the asset. Must be in denom_units."]
    pub base: AssetBase,
    #[doc = "[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coingecko_id: Option<AssetCoingeckoId>,
    pub denom_units: Vec<DenomUnit>,
    #[doc = "[OPTIONAL] Whether the asset has been deprecated for use. For readability, it is best to omit this property unless TRUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc = "[OPTIONAL] A short description of the asset"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<AssetDescription>,
    #[doc = "The human friendly unit of the asset. Must be in denom_units."]
    pub display: AssetDisplay,
    #[doc = "[OPTIONAL] A long description of the asset"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_description: Option<AssetExtendedDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibc: Option<AssetIbc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<AssetImagesItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<AssetKeywordsItem>,
    #[serde(rename = "logo_URIs", default, skip_serializing_if = "Option::is_none")]
    pub logo_ur_is: Option<AssetLogoUrIs>,
    #[doc = "The project name of the asset. For example Bitcoin."]
    pub name: AssetName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socials: Option<AssetSocials>,
    #[doc = "The symbol of an asset. For example BTC."]
    pub symbol: AssetSymbol,
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
#[doc = "[OPTIONAL] The address of the asset. Only required for type_asset : cw20, snip20"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] The address of the asset. Only required for type_asset : cw20, snip20\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetAddress(String);
impl std::ops::Deref for AssetAddress {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetAddress> for String {
    fn from(value: AssetAddress) -> Self {
        value.0
    }
}
impl From<&AssetAddress> for AssetAddress {
    fn from(value: &AssetAddress) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetAddress {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetAddress {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetAddress {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetAddress {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetAddress {
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
#[doc = "The base unit of the asset. Must be in denom_units."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base unit of the asset. Must be in denom_units.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetBase(String);
impl std::ops::Deref for AssetBase {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetBase> for String {
    fn from(value: AssetBase) -> Self {
        value.0
    }
}
impl From<&AssetBase> for AssetBase {
    fn from(value: &AssetBase) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetBase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetBase {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetBase {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetBase {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetBase {
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
#[doc = "[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] The coingecko id to fetch asset data from coingecko v3 api. See https://api.coingecko.com/api/v3/coins/list\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetCoingeckoId(String);
impl std::ops::Deref for AssetCoingeckoId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetCoingeckoId> for String {
    fn from(value: AssetCoingeckoId) -> Self {
        value.0
    }
}
impl From<&AssetCoingeckoId> for AssetCoingeckoId {
    fn from(value: &AssetCoingeckoId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetCoingeckoId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetCoingeckoId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetCoingeckoId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetCoingeckoId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetCoingeckoId {
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
#[doc = "[OPTIONAL] A short description of the asset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] A short description of the asset\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetDescription(String);
impl std::ops::Deref for AssetDescription {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetDescription> for String {
    fn from(value: AssetDescription) -> Self {
        value.0
    }
}
impl From<&AssetDescription> for AssetDescription {
    fn from(value: &AssetDescription) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetDescription {
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
#[doc = "The human friendly unit of the asset. Must be in denom_units."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The human friendly unit of the asset. Must be in denom_units.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetDisplay(String);
impl std::ops::Deref for AssetDisplay {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetDisplay> for String {
    fn from(value: AssetDisplay) -> Self {
        value.0
    }
}
impl From<&AssetDisplay> for AssetDisplay {
    fn from(value: &AssetDisplay) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetDisplay {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetDisplay {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetDisplay {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetDisplay {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetDisplay {
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
#[doc = "[OPTIONAL] A long description of the asset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"[OPTIONAL] A long description of the asset\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetExtendedDescription(String);
impl std::ops::Deref for AssetExtendedDescription {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetExtendedDescription> for String {
    fn from(value: AssetExtendedDescription) -> Self {
        value.0
    }
}
impl From<&AssetExtendedDescription> for AssetExtendedDescription {
    fn from(value: &AssetExtendedDescription) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetExtendedDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetExtendedDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetExtendedDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetExtendedDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetExtendedDescription {
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
#[serde(deny_unknown_fields)]
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
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "    },"]
#[doc = "    \"theme\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"minProperties\": 1,"]
#[doc = "      \"properties\": {"]
#[doc = "        \"background_color_hex\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$\""]
#[doc = "        },"]
#[doc = "        \"circle\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"dark_mode\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"monochrome\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"primary_color_hex\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$\""]
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
#[serde(deny_unknown_fields)]
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
#[doc = "    \"background_color_hex\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$\""]
#[doc = "    },"]
#[doc = "    \"circle\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"dark_mode\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"monochrome\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"primary_color_hex\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetImagesItemTheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color_hex: Option<AssetImagesItemThemeBackgroundColorHex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circle: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monochrome: Option<bool>,
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
#[doc = "AssetImagesItemThemeBackgroundColorHex"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetImagesItemThemeBackgroundColorHex(String);
impl std::ops::Deref for AssetImagesItemThemeBackgroundColorHex {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetImagesItemThemeBackgroundColorHex> for String {
    fn from(value: AssetImagesItemThemeBackgroundColorHex) -> Self {
        value.0
    }
}
impl From<&AssetImagesItemThemeBackgroundColorHex> for AssetImagesItemThemeBackgroundColorHex {
    fn from(value: &AssetImagesItemThemeBackgroundColorHex) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetImagesItemThemeBackgroundColorHex {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        if regress::Regex::new("^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})|none)$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetImagesItemThemeBackgroundColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetImagesItemThemeBackgroundColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetImagesItemThemeBackgroundColorHex {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetImagesItemThemeBackgroundColorHex {
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
#[doc = "AssetImagesItemThemePrimaryColorHex"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$\""]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        if regress::Regex::new("^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$\"".into());
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
#[doc = "AssetKeywordsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetKeywordsItem(String);
impl std::ops::Deref for AssetKeywordsItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetKeywordsItem> for String {
    fn from(value: AssetKeywordsItem) -> Self {
        value.0
    }
}
impl From<&AssetKeywordsItem> for AssetKeywordsItem {
    fn from(value: &AssetKeywordsItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetKeywordsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetKeywordsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetKeywordsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetKeywordsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetKeywordsItem {
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
#[doc = "  \"title\": \"AssetList\","]
#[doc = "  \"description\": \"Asset lists are a similar mechanism to allow frontends and other UIs to fetch metadata associated with Cosmos SDK denoms, especially for assets sent over IBC.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"assets\","]
#[doc = "    \"chain_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetList {
    pub assets: Vec<Asset>,
    pub chain_name: AssetListChainName,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<AssetListSchema>,
}
impl From<&AssetList> for AssetList {
    fn from(value: &AssetList) -> Self {
        value.clone()
    }
}
impl AssetList {
    pub fn builder() -> builder::AssetList {
        Default::default()
    }
}
#[doc = "AssetListChainName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetListChainName(String);
impl std::ops::Deref for AssetListChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetListChainName> for String {
    fn from(value: AssetListChainName) -> Self {
        value.0
    }
}
impl From<&AssetListChainName> for AssetListChainName {
    fn from(value: &AssetListChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetListChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetListChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetListChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetListChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetListChainName {
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
#[doc = "AssetListSchema"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^(\\\\.\\\\./)+assetlist\\\\.schema\\\\.json$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetListSchema(String);
impl std::ops::Deref for AssetListSchema {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetListSchema> for String {
    fn from(value: AssetListSchema) -> Self {
        value.0
    }
}
impl From<&AssetListSchema> for AssetListSchema {
    fn from(value: &AssetListSchema) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetListSchema {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
impl std::convert::TryFrom<&str> for AssetListSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetListSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetListSchema {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetListSchema {
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
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.png$\""]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^https://raw\\\\.githubusercontent\\\\.com/cosmos/chain-registry/master/(|testnets/|_non-cosmos/)[a-z0-9]+/images/.+\\\\.svg$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[doc = "  \"maxLength\": 42,"]
#[doc = "  \"minLength\": 1"]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
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
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"website\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
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
#[doc = "The symbol of an asset. For example BTC."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The symbol of an asset. For example BTC.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AssetSymbol(String);
impl std::ops::Deref for AssetSymbol {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AssetSymbol> for String {
    fn from(value: AssetSymbol) -> Self {
        value.0
    }
}
impl From<&AssetSymbol> for AssetSymbol {
    fn from(value: &AssetSymbol) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AssetSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AssetSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AssetSymbol {
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
#[doc = "      \"$ref\": \"#/$defs/ibc_bridge_transition\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/non_ibc_transition\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetTracesItem {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<IbcTransition>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<IbcCw20Transition>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<IbcBridgeTransition>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<NonIbcTransition>,
}
impl From<&AssetTracesItem> for AssetTracesItem {
    fn from(value: &AssetTracesItem) -> Self {
        value.clone()
    }
}
impl AssetTracesItem {
    pub fn builder() -> builder::AssetTracesItem {
        Default::default()
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
#[doc = "    \"substrate\","]
#[doc = "    \"unknown\""]
#[doc = "  ],"]
#[doc = "  \"minLength\": 1"]
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
    #[serde(rename = "unknown")]
    Unknown,
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
            Self::Unknown => "unknown".to_string(),
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
            "unknown" => Ok(Self::Unknown),
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
#[doc = "        \"type\": \"string\","]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"minContains\": 1"]
#[doc = "    },"]
#[doc = "    \"denom\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
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
#[serde(deny_unknown_fields)]
pub struct DenomUnit {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<DenomUnitAliasesItem>,
    pub denom: DenomUnitDenom,
    pub exponent: i64,
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
#[doc = "DenomUnitAliasesItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DenomUnitAliasesItem(String);
impl std::ops::Deref for DenomUnitAliasesItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<DenomUnitAliasesItem> for String {
    fn from(value: DenomUnitAliasesItem) -> Self {
        value.0
    }
}
impl From<&DenomUnitAliasesItem> for DenomUnitAliasesItem {
    fn from(value: &DenomUnitAliasesItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for DenomUnitAliasesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for DenomUnitAliasesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DenomUnitAliasesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DenomUnitAliasesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for DenomUnitAliasesItem {
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
#[doc = "DenomUnitDenom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DenomUnitDenom(String);
impl std::ops::Deref for DenomUnitDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<DenomUnitDenom> for String {
    fn from(value: DenomUnitDenom) -> Self {
        value.0
    }
}
impl From<&DenomUnitDenom> for DenomUnitDenom {
    fn from(value: &DenomUnitDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for DenomUnitDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for DenomUnitDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DenomUnitDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DenomUnitDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for DenomUnitDenom {
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
#[doc = "IbcBridgeTransition"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"chain\","]
#[doc = "    \"counterparty\","]
#[doc = "    \"provider\","]
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
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
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
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"description\": \"The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company].\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ibc-bridge\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcBridgeTransition {
    pub chain: IbcBridgeTransitionChain,
    pub counterparty: IbcBridgeTransitionCounterparty,
    #[doc = "The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company]."]
    pub provider: IbcBridgeTransitionProvider,
    #[serde(rename = "type")]
    pub type_: IbcBridgeTransitionType,
}
impl From<&IbcBridgeTransition> for IbcBridgeTransition {
    fn from(value: &IbcBridgeTransition) -> Self {
        value.clone()
    }
}
impl IbcBridgeTransition {
    pub fn builder() -> builder::IbcBridgeTransition {
        Default::default()
    }
}
#[doc = "IbcBridgeTransitionChain"]
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
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcBridgeTransitionChain {
    #[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcBridgeTransitionChainChannelId,
    #[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
    pub path: IbcBridgeTransitionChainPath,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IbcBridgeTransitionChainPort>,
}
impl From<&IbcBridgeTransitionChain> for IbcBridgeTransitionChain {
    fn from(value: &IbcBridgeTransitionChain) -> Self {
        value.clone()
    }
}
impl IbcBridgeTransitionChain {
    pub fn builder() -> builder::IbcBridgeTransitionChain {
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
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionChainChannelId(String);
impl std::ops::Deref for IbcBridgeTransitionChainChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionChainChannelId> for String {
    fn from(value: IbcBridgeTransitionChainChannelId) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionChainChannelId> for IbcBridgeTransitionChainChannelId {
    fn from(value: &IbcBridgeTransitionChainChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionChainChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
impl std::convert::TryFrom<&str> for IbcBridgeTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionChainChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionChainChannelId {
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
#[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionChainPath(String);
impl std::ops::Deref for IbcBridgeTransitionChainPath {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionChainPath> for String {
    fn from(value: IbcBridgeTransitionChainPath) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionChainPath> for IbcBridgeTransitionChainPath {
    fn from(value: &IbcBridgeTransitionChainPath) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionChainPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionChainPath {
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
#[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionChainPort(String);
impl std::ops::Deref for IbcBridgeTransitionChainPort {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionChainPort> for String {
    fn from(value: IbcBridgeTransitionChainPort) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionChainPort> for IbcBridgeTransitionChainPort {
    fn from(value: &IbcBridgeTransitionChainPort) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionChainPort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionChainPort {
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
#[doc = "IbcBridgeTransitionCounterparty"]
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcBridgeTransitionCounterparty {
    #[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    pub base_denom: IbcBridgeTransitionCounterpartyBaseDenom,
    #[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
    pub chain_name: IbcBridgeTransitionCounterpartyChainName,
    #[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcBridgeTransitionCounterpartyChannelId,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IbcBridgeTransitionCounterpartyPort>,
}
impl From<&IbcBridgeTransitionCounterparty> for IbcBridgeTransitionCounterparty {
    fn from(value: &IbcBridgeTransitionCounterparty) -> Self {
        value.clone()
    }
}
impl IbcBridgeTransitionCounterparty {
    pub fn builder() -> builder::IbcBridgeTransitionCounterparty {
        Default::default()
    }
}
#[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionCounterpartyBaseDenom(String);
impl std::ops::Deref for IbcBridgeTransitionCounterpartyBaseDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionCounterpartyBaseDenom> for String {
    fn from(value: IbcBridgeTransitionCounterpartyBaseDenom) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionCounterpartyBaseDenom> for IbcBridgeTransitionCounterpartyBaseDenom {
    fn from(value: &IbcBridgeTransitionCounterpartyBaseDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionCounterpartyBaseDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionCounterpartyBaseDenom {
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
#[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionCounterpartyChainName(String);
impl std::ops::Deref for IbcBridgeTransitionCounterpartyChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionCounterpartyChainName> for String {
    fn from(value: IbcBridgeTransitionCounterpartyChainName) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionCounterpartyChainName> for IbcBridgeTransitionCounterpartyChainName {
    fn from(value: &IbcBridgeTransitionCounterpartyChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionCounterpartyChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionCounterpartyChainName {
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
#[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionCounterpartyChannelId(String);
impl std::ops::Deref for IbcBridgeTransitionCounterpartyChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionCounterpartyChannelId> for String {
    fn from(value: IbcBridgeTransitionCounterpartyChannelId) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionCounterpartyChannelId> for IbcBridgeTransitionCounterpartyChannelId {
    fn from(value: &IbcBridgeTransitionCounterpartyChannelId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionCounterpartyChannelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
impl std::convert::TryFrom<&str> for IbcBridgeTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionCounterpartyChannelId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionCounterpartyChannelId {
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
#[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionCounterpartyPort(String);
impl std::ops::Deref for IbcBridgeTransitionCounterpartyPort {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionCounterpartyPort> for String {
    fn from(value: IbcBridgeTransitionCounterpartyPort) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionCounterpartyPort> for IbcBridgeTransitionCounterpartyPort {
    fn from(value: &IbcBridgeTransitionCounterpartyPort) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionCounterpartyPort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionCounterpartyPort {
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
#[doc = "The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company]."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company].\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcBridgeTransitionProvider(String);
impl std::ops::Deref for IbcBridgeTransitionProvider {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcBridgeTransitionProvider> for String {
    fn from(value: IbcBridgeTransitionProvider) -> Self {
        value.0
    }
}
impl From<&IbcBridgeTransitionProvider> for IbcBridgeTransitionProvider {
    fn from(value: &IbcBridgeTransitionProvider) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcBridgeTransitionProvider {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcBridgeTransitionProvider {
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
#[doc = "IbcBridgeTransitionType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ibc-bridge\""]
#[doc = "  ],"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IbcBridgeTransitionType {
    #[serde(rename = "ibc-bridge")]
    IbcBridge,
}
impl From<&IbcBridgeTransitionType> for IbcBridgeTransitionType {
    fn from(value: &IbcBridgeTransitionType) -> Self {
        value.clone()
    }
}
impl ToString for IbcBridgeTransitionType {
    fn to_string(&self) -> String {
        match *self {
            Self::IbcBridge => "ibc-bridge".to_string(),
        }
    }
}
impl std::str::FromStr for IbcBridgeTransitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ibc-bridge" => Ok(Self::IbcBridge),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IbcBridgeTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcBridgeTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcBridgeTransitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
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
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"port\": {"]
#[doc = "          \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ibc-cw20\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcCw20TransitionChain {
    #[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcCw20TransitionChainChannelId,
    #[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
    pub path: IbcCw20TransitionChainPath,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    pub port: IbcCw20TransitionChainPort,
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
#[doc = "  \"minLength\": 1,"]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
#[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionChainPath(String);
impl std::ops::Deref for IbcCw20TransitionChainPath {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionChainPath> for String {
    fn from(value: IbcCw20TransitionChainPath) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionChainPath> for IbcCw20TransitionChainPath {
    fn from(value: &IbcCw20TransitionChainPath) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionChainPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionChainPath {
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
#[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionChainPort(String);
impl std::ops::Deref for IbcCw20TransitionChainPort {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionChainPort> for String {
    fn from(value: IbcCw20TransitionChainPort) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionChainPort> for IbcCw20TransitionChainPort {
    fn from(value: &IbcCw20TransitionChainPort) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionChainPort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionChainPort {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionChainPort {
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"port\": {"]
#[doc = "      \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcCw20TransitionCounterparty {
    #[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    pub base_denom: IbcCw20TransitionCounterpartyBaseDenom,
    #[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
    pub chain_name: IbcCw20TransitionCounterpartyChainName,
    #[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcCw20TransitionCounterpartyChannelId,
    #[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
    pub port: IbcCw20TransitionCounterpartyPort,
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
#[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionCounterpartyBaseDenom(String);
impl std::ops::Deref for IbcCw20TransitionCounterpartyBaseDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionCounterpartyBaseDenom> for String {
    fn from(value: IbcCw20TransitionCounterpartyBaseDenom) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionCounterpartyBaseDenom> for IbcCw20TransitionCounterpartyBaseDenom {
    fn from(value: &IbcCw20TransitionCounterpartyBaseDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionCounterpartyBaseDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionCounterpartyBaseDenom {
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
#[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionCounterpartyChainName(String);
impl std::ops::Deref for IbcCw20TransitionCounterpartyChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionCounterpartyChainName> for String {
    fn from(value: IbcCw20TransitionCounterpartyChainName) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionCounterpartyChainName> for IbcCw20TransitionCounterpartyChainName {
    fn from(value: &IbcCw20TransitionCounterpartyChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionCounterpartyChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionCounterpartyChainName {
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
#[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
#[doc = "The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port used to transfer IBC assets; often 'transfer', but sometimes varies, e.g., for outgoing cw20 transfers.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcCw20TransitionCounterpartyPort(String);
impl std::ops::Deref for IbcCw20TransitionCounterpartyPort {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcCw20TransitionCounterpartyPort> for String {
    fn from(value: IbcCw20TransitionCounterpartyPort) -> Self {
        value.0
    }
}
impl From<&IbcCw20TransitionCounterpartyPort> for IbcCw20TransitionCounterpartyPort {
    fn from(value: &IbcCw20TransitionCounterpartyPort) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcCw20TransitionCounterpartyPort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcCw20TransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcCw20TransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcCw20TransitionCounterpartyPort {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcCw20TransitionCounterpartyPort {
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
#[doc = "  ],"]
#[doc = "  \"minLength\": 1"]
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
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
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
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"channel_id\": {"]
#[doc = "          \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^channel-(JEnb|\\\\d+)$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ibc\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-\\\\d+$\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcTransitionChain {
    #[doc = "The chain's IBC transfer channel(, e.g., 'channel-1')."]
    pub channel_id: IbcTransitionChainChannelId,
    #[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
    pub path: IbcTransitionChainPath,
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
#[doc = "  \"minLength\": 1,"]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
#[doc = "The port/channel/denom input string that generates the 'ibc/...' denom."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The port/channel/denom input string that generates the 'ibc/...' denom.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcTransitionChainPath(String);
impl std::ops::Deref for IbcTransitionChainPath {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcTransitionChainPath> for String {
    fn from(value: IbcTransitionChainPath) -> Self {
        value.0
    }
}
impl From<&IbcTransitionChainPath> for IbcTransitionChainPath {
    fn from(value: &IbcTransitionChainPath) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcTransitionChainPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionChainPath {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcTransitionChainPath {
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"channel_id\": {"]
#[doc = "      \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^channel-(JEnb|\\\\d+)$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IbcTransitionCounterparty {
    #[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    pub base_denom: IbcTransitionCounterpartyBaseDenom,
    #[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
    pub chain_name: IbcTransitionCounterpartyChainName,
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
#[doc = "The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base unit of the asset on its source platform. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcTransitionCounterpartyBaseDenom(String);
impl std::ops::Deref for IbcTransitionCounterpartyBaseDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcTransitionCounterpartyBaseDenom> for String {
    fn from(value: IbcTransitionCounterpartyBaseDenom) -> Self {
        value.0
    }
}
impl From<&IbcTransitionCounterpartyBaseDenom> for IbcTransitionCounterpartyBaseDenom {
    fn from(value: &IbcTransitionCounterpartyBaseDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcTransitionCounterpartyBaseDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcTransitionCounterpartyBaseDenom {
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
#[doc = "The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The name of the counterparty chain. (must match exactly the chain name used in the Chain Registry)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IbcTransitionCounterpartyChainName(String);
impl std::ops::Deref for IbcTransitionCounterpartyChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IbcTransitionCounterpartyChainName> for String {
    fn from(value: IbcTransitionCounterpartyChainName) -> Self {
        value.0
    }
}
impl From<&IbcTransitionCounterpartyChainName> for IbcTransitionCounterpartyChainName {
    fn from(value: &IbcTransitionCounterpartyChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IbcTransitionCounterpartyChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IbcTransitionCounterpartyChainName {
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
#[doc = "The counterparty IBC transfer channel(, e.g., 'channel-1')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The counterparty IBC transfer channel(, e.g., 'channel-1').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1,"]
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
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
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
#[doc = "  ],"]
#[doc = "  \"minLength\": 1"]
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
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
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
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"chain_name\": {"]
#[doc = "          \"description\": \"The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"contract\": {"]
#[doc = "          \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"description\": \"The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company].\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"bridge\","]
#[doc = "        \"liquid-stake\","]
#[doc = "        \"synthetic\","]
#[doc = "        \"wrapped\","]
#[doc = "        \"additional-mintage\","]
#[doc = "        \"test-mintage\","]
#[doc = "        \"legacy-mintage\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonIbcTransition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain: Option<NonIbcTransitionChain>,
    pub counterparty: NonIbcTransitionCounterparty,
    #[doc = "The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company]."]
    pub provider: NonIbcTransitionProvider,
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonIbcTransitionChain {
    #[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
    pub contract: NonIbcTransitionChainContract,
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
#[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NonIbcTransitionChainContract(String);
impl std::ops::Deref for NonIbcTransitionChainContract {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NonIbcTransitionChainContract> for String {
    fn from(value: NonIbcTransitionChainContract) -> Self {
        value.0
    }
}
impl From<&NonIbcTransitionChainContract> for NonIbcTransitionChainContract {
    fn from(value: &NonIbcTransitionChainContract) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NonIbcTransitionChainContract {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionChainContract {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionChainContract {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionChainContract {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NonIbcTransitionChainContract {
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"contract\": {"]
#[doc = "      \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonIbcTransitionCounterparty {
    pub base_denom: NonIbcTransitionCounterpartyBaseDenom,
    #[doc = "The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
    pub chain_name: NonIbcTransitionCounterpartyChainName,
    #[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<NonIbcTransitionCounterpartyContract>,
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
#[doc = "NonIbcTransitionCounterpartyBaseDenom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NonIbcTransitionCounterpartyBaseDenom(String);
impl std::ops::Deref for NonIbcTransitionCounterpartyBaseDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NonIbcTransitionCounterpartyBaseDenom> for String {
    fn from(value: NonIbcTransitionCounterpartyBaseDenom) -> Self {
        value.0
    }
}
impl From<&NonIbcTransitionCounterpartyBaseDenom> for NonIbcTransitionCounterpartyBaseDenom {
    fn from(value: &NonIbcTransitionCounterpartyBaseDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NonIbcTransitionCounterpartyBaseDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionCounterpartyBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NonIbcTransitionCounterpartyBaseDenom {
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
#[doc = "The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The chain or platform from which the asset originates. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NonIbcTransitionCounterpartyChainName(String);
impl std::ops::Deref for NonIbcTransitionCounterpartyChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NonIbcTransitionCounterpartyChainName> for String {
    fn from(value: NonIbcTransitionCounterpartyChainName) -> Self {
        value.0
    }
}
impl From<&NonIbcTransitionCounterpartyChainName> for NonIbcTransitionCounterpartyChainName {
    fn from(value: &NonIbcTransitionCounterpartyChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NonIbcTransitionCounterpartyChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionCounterpartyChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NonIbcTransitionCounterpartyChainName {
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
#[doc = "The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contract address where the transition takes place, where applicable. E.g., The Ethereum contract that locks up the asset while it's minted on another chain.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NonIbcTransitionCounterpartyContract(String);
impl std::ops::Deref for NonIbcTransitionCounterpartyContract {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NonIbcTransitionCounterpartyContract> for String {
    fn from(value: NonIbcTransitionCounterpartyContract) -> Self {
        value.0
    }
}
impl From<&NonIbcTransitionCounterpartyContract> for NonIbcTransitionCounterpartyContract {
    fn from(value: &NonIbcTransitionCounterpartyContract) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NonIbcTransitionCounterpartyContract {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionCounterpartyContract {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionCounterpartyContract {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionCounterpartyContract {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NonIbcTransitionCounterpartyContract {
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
#[doc = "The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company]."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The entity offering the service. E.g., 'Gravity Bridge' [Network] or 'Tether' [Company].\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NonIbcTransitionProvider(String);
impl std::ops::Deref for NonIbcTransitionProvider {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NonIbcTransitionProvider> for String {
    fn from(value: NonIbcTransitionProvider) -> Self {
        value.0
    }
}
impl From<&NonIbcTransitionProvider> for NonIbcTransitionProvider {
    fn from(value: &NonIbcTransitionProvider) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NonIbcTransitionProvider {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NonIbcTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonIbcTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonIbcTransitionProvider {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NonIbcTransitionProvider {
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
#[doc = "    \"test-mintage\","]
#[doc = "    \"legacy-mintage\""]
#[doc = "  ],"]
#[doc = "  \"minLength\": 1"]
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
    #[serde(rename = "legacy-mintage")]
    LegacyMintage,
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
            Self::LegacyMintage => "legacy-mintage".to_string(),
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
            "legacy-mintage" => Ok(Self::LegacyMintage),
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
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"chain_name\": {"]
#[doc = "      \"description\": \"The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pointer {
    #[doc = "The base denom of the asset from which the object originates. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_denom: Option<PointerBaseDenom>,
    #[doc = "The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
    pub chain_name: PointerChainName,
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
#[doc = "The base denom of the asset from which the object originates. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base denom of the asset from which the object originates. E.g., when describing ATOM from Cosmos Hub, specify 'uatom', NOT 'atom' nor 'ATOM'; base units are unique per platform.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PointerBaseDenom(String);
impl std::ops::Deref for PointerBaseDenom {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<PointerBaseDenom> for String {
    fn from(value: PointerBaseDenom) -> Self {
        value.0
    }
}
impl From<&PointerBaseDenom> for PointerBaseDenom {
    fn from(value: &PointerBaseDenom) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PointerBaseDenom {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PointerBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PointerBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PointerBaseDenom {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PointerBaseDenom {
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
#[doc = "The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The chain name or platform from which the object resides. E.g., 'cosmoshub', 'ethereum', 'forex', or 'nasdaq'\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PointerChainName(String);
impl std::ops::Deref for PointerChainName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<PointerChainName> for String {
    fn from(value: PointerChainName) -> Self {
        value.0
    }
}
impl From<&PointerChainName> for PointerChainName {
    fn from(value: &PointerChainName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PointerChainName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PointerChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PointerChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PointerChainName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PointerChainName {
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Asset {
        address: Result<Option<super::AssetAddress>, String>,
        base: Result<super::AssetBase, String>,
        coingecko_id: Result<Option<super::AssetCoingeckoId>, String>,
        denom_units: Result<Vec<super::DenomUnit>, String>,
        deprecated: Result<Option<bool>, String>,
        description: Result<Option<super::AssetDescription>, String>,
        display: Result<super::AssetDisplay, String>,
        extended_description: Result<Option<super::AssetExtendedDescription>, String>,
        ibc: Result<Option<super::AssetIbc>, String>,
        images: Result<Vec<super::AssetImagesItem>, String>,
        keywords: Result<Vec<super::AssetKeywordsItem>, String>,
        logo_ur_is: Result<Option<super::AssetLogoUrIs>, String>,
        name: Result<super::AssetName, String>,
        socials: Result<Option<super::AssetSocials>, String>,
        symbol: Result<super::AssetSymbol, String>,
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
            T: std::convert::TryInto<Option<super::AssetAddress>>,
            T::Error: std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for address: {}", e));
            self
        }
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AssetBase>,
            T::Error: std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn coingecko_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetCoingeckoId>>,
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
            T: std::convert::TryInto<Option<super::AssetDescription>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn display<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AssetDisplay>,
            T::Error: std::fmt::Display,
        {
            self.display = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display: {}", e));
            self
        }
        pub fn extended_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetExtendedDescription>>,
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
            T: std::convert::TryInto<Vec<super::AssetKeywordsItem>>,
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
            T: std::convert::TryInto<super::AssetSymbol>,
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
        background_color_hex: Result<Option<super::AssetImagesItemThemeBackgroundColorHex>, String>,
        circle: Result<Option<bool>, String>,
        dark_mode: Result<Option<bool>, String>,
        monochrome: Result<Option<bool>, String>,
        primary_color_hex: Result<Option<super::AssetImagesItemThemePrimaryColorHex>, String>,
    }
    impl Default for AssetImagesItemTheme {
        fn default() -> Self {
            Self {
                background_color_hex: Ok(Default::default()),
                circle: Ok(Default::default()),
                dark_mode: Ok(Default::default()),
                monochrome: Ok(Default::default()),
                primary_color_hex: Ok(Default::default()),
            }
        }
    }
    impl AssetImagesItemTheme {
        pub fn background_color_hex<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetImagesItemThemeBackgroundColorHex>>,
            T::Error: std::fmt::Display,
        {
            self.background_color_hex = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_color_hex: {}",
                    e
                )
            });
            self
        }
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
        pub fn monochrome<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.monochrome = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for monochrome: {}", e));
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
                background_color_hex: value.background_color_hex?,
                circle: value.circle?,
                dark_mode: value.dark_mode?,
                monochrome: value.monochrome?,
                primary_color_hex: value.primary_color_hex?,
            })
        }
    }
    impl From<super::AssetImagesItemTheme> for AssetImagesItemTheme {
        fn from(value: super::AssetImagesItemTheme) -> Self {
            Self {
                background_color_hex: Ok(value.background_color_hex),
                circle: Ok(value.circle),
                dark_mode: Ok(value.dark_mode),
                monochrome: Ok(value.monochrome),
                primary_color_hex: Ok(value.primary_color_hex),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssetList {
        assets: Result<Vec<super::Asset>, String>,
        chain_name: Result<super::AssetListChainName, String>,
        schema: Result<Option<super::AssetListSchema>, String>,
    }
    impl Default for AssetList {
        fn default() -> Self {
            Self {
                assets: Err("no value supplied for assets".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                schema: Ok(Default::default()),
            }
        }
    }
    impl AssetList {
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
            T: std::convert::TryInto<super::AssetListChainName>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AssetListSchema>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetList> for super::AssetList {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetList) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                assets: value.assets?,
                chain_name: value.chain_name?,
                schema: value.schema?,
            })
        }
    }
    impl From<super::AssetList> for AssetList {
        fn from(value: super::AssetList) -> Self {
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
    pub struct AssetTracesItem {
        subtype_0: Result<Option<super::IbcTransition>, String>,
        subtype_1: Result<Option<super::IbcCw20Transition>, String>,
        subtype_2: Result<Option<super::IbcBridgeTransition>, String>,
        subtype_3: Result<Option<super::NonIbcTransition>, String>,
    }
    impl Default for AssetTracesItem {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
            }
        }
    }
    impl AssetTracesItem {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IbcTransition>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IbcCw20Transition>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IbcBridgeTransition>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonIbcTransition>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AssetTracesItem> for super::AssetTracesItem {
        type Error = super::error::ConversionError;
        fn try_from(value: AssetTracesItem) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
            })
        }
    }
    impl From<super::AssetTracesItem> for AssetTracesItem {
        fn from(value: super::AssetTracesItem) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DenomUnit {
        aliases: Result<Vec<super::DenomUnitAliasesItem>, String>,
        denom: Result<super::DenomUnitDenom, String>,
        exponent: Result<i64, String>,
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
            T: std::convert::TryInto<Vec<super::DenomUnitAliasesItem>>,
            T::Error: std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DenomUnitDenom>,
            T::Error: std::fmt::Display,
        {
            self.denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for denom: {}", e));
            self
        }
        pub fn exponent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
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
    pub struct IbcBridgeTransition {
        chain: Result<super::IbcBridgeTransitionChain, String>,
        counterparty: Result<super::IbcBridgeTransitionCounterparty, String>,
        provider: Result<super::IbcBridgeTransitionProvider, String>,
        type_: Result<super::IbcBridgeTransitionType, String>,
    }
    impl Default for IbcBridgeTransition {
        fn default() -> Self {
            Self {
                chain: Err("no value supplied for chain".to_string()),
                counterparty: Err("no value supplied for counterparty".to_string()),
                provider: Err("no value supplied for provider".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IbcBridgeTransition {
        pub fn chain<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionChain>,
            T::Error: std::fmt::Display,
        {
            self.chain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain: {}", e));
            self
        }
        pub fn counterparty<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionCounterparty>,
            T::Error: std::fmt::Display,
        {
            self.counterparty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for counterparty: {}", e));
            self
        }
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionProvider>,
            T::Error: std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcBridgeTransition> for super::IbcBridgeTransition {
        type Error = super::error::ConversionError;
        fn try_from(value: IbcBridgeTransition) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                chain: value.chain?,
                counterparty: value.counterparty?,
                provider: value.provider?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IbcBridgeTransition> for IbcBridgeTransition {
        fn from(value: super::IbcBridgeTransition) -> Self {
            Self {
                chain: Ok(value.chain),
                counterparty: Ok(value.counterparty),
                provider: Ok(value.provider),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcBridgeTransitionChain {
        channel_id: Result<super::IbcBridgeTransitionChainChannelId, String>,
        path: Result<super::IbcBridgeTransitionChainPath, String>,
        port: Result<Option<super::IbcBridgeTransitionChainPort>, String>,
    }
    impl Default for IbcBridgeTransitionChain {
        fn default() -> Self {
            Self {
                channel_id: Err("no value supplied for channel_id".to_string()),
                path: Err("no value supplied for path".to_string()),
                port: Ok(Default::default()),
            }
        }
    }
    impl IbcBridgeTransitionChain {
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionChainChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionChainPath>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IbcBridgeTransitionChainPort>>,
            T::Error: std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcBridgeTransitionChain> for super::IbcBridgeTransitionChain {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IbcBridgeTransitionChain,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel_id: value.channel_id?,
                path: value.path?,
                port: value.port?,
            })
        }
    }
    impl From<super::IbcBridgeTransitionChain> for IbcBridgeTransitionChain {
        fn from(value: super::IbcBridgeTransitionChain) -> Self {
            Self {
                channel_id: Ok(value.channel_id),
                path: Ok(value.path),
                port: Ok(value.port),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IbcBridgeTransitionCounterparty {
        base_denom: Result<super::IbcBridgeTransitionCounterpartyBaseDenom, String>,
        chain_name: Result<super::IbcBridgeTransitionCounterpartyChainName, String>,
        channel_id: Result<super::IbcBridgeTransitionCounterpartyChannelId, String>,
        port: Result<Option<super::IbcBridgeTransitionCounterpartyPort>, String>,
    }
    impl Default for IbcBridgeTransitionCounterparty {
        fn default() -> Self {
            Self {
                base_denom: Err("no value supplied for base_denom".to_string()),
                chain_name: Err("no value supplied for chain_name".to_string()),
                channel_id: Err("no value supplied for channel_id".to_string()),
                port: Ok(Default::default()),
            }
        }
    }
    impl IbcBridgeTransitionCounterparty {
        pub fn base_denom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionCounterpartyBaseDenom>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionCounterpartyChainName>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn channel_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcBridgeTransitionCounterpartyChannelId>,
            T::Error: std::fmt::Display,
        {
            self.channel_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel_id: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IbcBridgeTransitionCounterpartyPort>>,
            T::Error: std::fmt::Display,
        {
            self.port = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for port: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IbcBridgeTransitionCounterparty>
        for super::IbcBridgeTransitionCounterparty
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IbcBridgeTransitionCounterparty,
        ) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                base_denom: value.base_denom?,
                chain_name: value.chain_name?,
                channel_id: value.channel_id?,
                port: value.port?,
            })
        }
    }
    impl From<super::IbcBridgeTransitionCounterparty> for IbcBridgeTransitionCounterparty {
        fn from(value: super::IbcBridgeTransitionCounterparty) -> Self {
            Self {
                base_denom: Ok(value.base_denom),
                chain_name: Ok(value.chain_name),
                channel_id: Ok(value.channel_id),
                port: Ok(value.port),
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
        path: Result<super::IbcCw20TransitionChainPath, String>,
        port: Result<super::IbcCw20TransitionChainPort, String>,
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
            T: std::convert::TryInto<super::IbcCw20TransitionChainPath>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn port<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionChainPort>,
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
        base_denom: Result<super::IbcCw20TransitionCounterpartyBaseDenom, String>,
        chain_name: Result<super::IbcCw20TransitionCounterpartyChainName, String>,
        channel_id: Result<super::IbcCw20TransitionCounterpartyChannelId, String>,
        port: Result<super::IbcCw20TransitionCounterpartyPort, String>,
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
            T: std::convert::TryInto<super::IbcCw20TransitionCounterpartyBaseDenom>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcCw20TransitionCounterpartyChainName>,
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
            T: std::convert::TryInto<super::IbcCw20TransitionCounterpartyPort>,
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
        path: Result<super::IbcTransitionChainPath, String>,
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
            T: std::convert::TryInto<super::IbcTransitionChainPath>,
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
        base_denom: Result<super::IbcTransitionCounterpartyBaseDenom, String>,
        chain_name: Result<super::IbcTransitionCounterpartyChainName, String>,
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
            T: std::convert::TryInto<super::IbcTransitionCounterpartyBaseDenom>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IbcTransitionCounterpartyChainName>,
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
        provider: Result<super::NonIbcTransitionProvider, String>,
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
            T: std::convert::TryInto<super::NonIbcTransitionProvider>,
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
        contract: Result<super::NonIbcTransitionChainContract, String>,
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
            T: std::convert::TryInto<super::NonIbcTransitionChainContract>,
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
        base_denom: Result<super::NonIbcTransitionCounterpartyBaseDenom, String>,
        chain_name: Result<super::NonIbcTransitionCounterpartyChainName, String>,
        contract: Result<Option<super::NonIbcTransitionCounterpartyContract>, String>,
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
            T: std::convert::TryInto<super::NonIbcTransitionCounterpartyBaseDenom>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NonIbcTransitionCounterpartyChainName>,
            T::Error: std::fmt::Display,
        {
            self.chain_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chain_name: {}", e));
            self
        }
        pub fn contract<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NonIbcTransitionCounterpartyContract>>,
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
        base_denom: Result<Option<super::PointerBaseDenom>, String>,
        chain_name: Result<super::PointerChainName, String>,
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
            T: std::convert::TryInto<Option<super::PointerBaseDenom>>,
            T::Error: std::fmt::Display,
        {
            self.base_denom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_denom: {}", e));
            self
        }
        pub fn chain_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PointerChainName>,
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
