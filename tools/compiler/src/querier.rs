use anyhow::Context;
use reqwest::Client;

use crate::error::AppResult;
use crate::github::types::{AssetList, GitHubContent};
use crate::parser::{ChainConfig, IbcInput};
use tracing::instrument;

// URL for building API calls to Github, pointing to the Cosmos Chain Registry repo.
const GITHUB_API_BASE_URL: &str = "https://api.github.com/repos/cosmos/chain-registry/contents";

// Optional commit hash for looking up a specific version of the chain registry.
// If empty, the lookup defaults to the most recent commit on the default branch.
// If set, must be formatted as a URL param, e.g.
// const GITHUB_API_GIT_REF: &str = "?ref=b2862cf7b8aea5634cdbe8a13e80db499409429f";
const GITHUB_API_GIT_REF: &str = "";

/// Queries asset metadata from the cosmos asset registry
#[instrument(skip_all)]
pub async fn query_github_assets(
    chain_config: &ChainConfig,
) -> AppResult<Vec<(IbcInput, AssetList)>> {
    let client = Client::new();
    let mut futures = Vec::new();

    for ibc_asset in &chain_config.ibc_connections {
        let url = format!(
            "{}/{}/assetlist.json{}",
            GITHUB_API_BASE_URL, ibc_asset.cosmos_registry_dir, GITHUB_API_GIT_REF,
        );
        let future = fetch_asset_list(&client, url, ibc_asset);
        futures.push(future);
    }

    let results = futures::future::try_join_all(futures).await?;
    tracing::debug!("all assets fetched successfully");
    Ok(results)
}

#[instrument(skip_all)]
async fn fetch_asset_list(
    client: &Client,
    url: String,
    ibc_asset: &IbcInput,
) -> AppResult<(IbcInput, AssetList)> {
    tracing::debug!(ibc_asset=?ibc_asset.display_name, url, "fetching asset info");
    // First obtain GithubContent, from which we'll extract a URL to download the asset list as JSON.
    let res = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "request")
        .send()
        .await?
        .error_for_status()
        .map_err(|e| {
            tracing::error!("failed to get asset list for '{}'", ibc_asset.display_name);
            e
        })?;
    let github_content = res.json::<GitHubContent>().await.map_err(|e| {
        tracing::error!(
            "failed to parse asset info for '{}' as GitHubContent",
            ibc_asset.display_name
        );
        e
    })?;

    tracing::debug!(ibc_asset=?ibc_asset.display_name, url, "fetching asset list");
    let res = client
        .get(&github_content.download_url)
        .header(reqwest::header::USER_AGENT, "request")
        .send()
        .await
        .context("failed to get the jawn")?;

    let asset_list = res
        .json::<AssetList>()
        .await
        // TODO: unpack this list so we can fail on specific asset
        .context("failed to convert the json")?;
    Ok((ibc_asset.clone(), asset_list))
}
