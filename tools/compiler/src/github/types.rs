use std::default::Default;

use serde::{Deserialize, Serialize};

use crate::github::assetlist_schema::Asset;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AssetList {
    pub chain_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub git: String,
    pub html: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub encoding: String,
    #[serde(rename = "_links")]
    pub links: Links,
}
