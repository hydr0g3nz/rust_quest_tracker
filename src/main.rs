use learn_project::config::config_loader;
use tracing::info;
use tracing::error;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load environment variables: {}", e);
            std::process::exit(1);
        }
    };
    info!("env has been loaded: {:#?}", dotenvy_env);
}
