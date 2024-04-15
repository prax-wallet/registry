use reqwest::Client;

use crate::error::AppResult;
use crate::github::types::{AssetList, GitHubContent};
use crate::parser::{ChainConfig, IbcConnection};

const GITHUB_API_BASE_URL: &str = "https://api.github.com/repos/cosmos/chain-registry/contents";

/// Queries asset metadata from the cosmos asset registry
pub async fn query_github_assets(
    chain_config: &ChainConfig,
) -> AppResult<Vec<(IbcConnection, AssetList)>> {
    let client = Client::new();
    let mut futures = Vec::new();

    for ibc_asset in &chain_config.ibc_connections {
        let url = format!(
            "{}/{}/assetlist.json",
            GITHUB_API_BASE_URL, ibc_asset.cosmos_registry_dir
        );
        let future = fetch_asset_list(&client, url, ibc_asset);
        futures.push(future);
    }

    let results = futures::future::try_join_all(futures).await?;
    Ok(results)
}

async fn fetch_asset_list(
    client: &Client,
    url: String,
    ibc_asset: &IbcConnection,
) -> AppResult<(IbcConnection, AssetList)> {
    let res = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "request")
        .send()
        .await?;
    let github_content = res.json::<GitHubContent>().await?;

    let res = client
        .get(&github_content.download_url)
        .header(reqwest::header::USER_AGENT, "request")
        .send()
        .await?;
    let asset_list = res.json::<AssetList>().await?;
    Ok((ibc_asset.clone(), asset_list))
}
