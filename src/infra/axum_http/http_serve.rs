use super::default_routers;
use crate::config::config_models::DotEnvConfig;
use crate::infra::postgres::postgres_connection::PgPoolSquad;
use anyhow::{Ok, Result};
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
pub async fn start(config: Arc<DotEnvConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health_check", get(default_routers::health_check))
        .layer(
            TimeoutLayer::new(Duration::from_secs(config.server.timeout))
        )
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?
        ))
        .layer(CorsLayer::new().allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH
            ]).allow_origin(Any))
            .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>info!("received Ctrl+C, shutting down"),
        _ = terminate => info!("received terminate signal, shutting down"),
    }
}