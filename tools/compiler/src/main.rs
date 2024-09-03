use penumbra_registry::error::AppResult;
use penumbra_registry::processor::generate_registry;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();

    if std::env::var("GITHUB_TOKEN").is_err() {
        tracing::warn!("GITHUB_TOKEN env var not set, requests may fail due to API ratelimiting");
    }
    generate_registry().await
}
