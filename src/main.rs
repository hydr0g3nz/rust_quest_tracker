use std::sync::Arc;

use learn_project::config::config_loader;
use learn_project::infra::axum_http::http_serve::start;
use learn_project::infra::postgres::postgres_connection;
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
    let postgres_poo = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    info!("database has been connected: {:#?}", postgres_poo);
    start(Arc::new(dotenvy_env), Arc::new(postgres_poo))
    .await
    .expect("Failed to start server");
}
