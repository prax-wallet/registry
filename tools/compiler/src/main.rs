use penumbra_registry::error::AppResult;
use penumbra_registry::processor::generate_registry;

fn main() -> AppResult<()> {
    tracing_subscriber::fmt::init();
    generate_registry()
}
