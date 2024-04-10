use penumbra_registry::error::AppResult;
use penumbra_registry::processor::generate_registry;

#[tokio::main]
async fn main() -> AppResult<()> {
    generate_registry().await
}
