use super::default_routers;
use crate::infra::postgres::postgres_connection::PgPoolSquad;
use crate::{config::config_models::DotEnvConfig, infra::axum_http::routers};
use anyhow::{Ok, Result};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderValue;
use axum::{http::Method, routing::get, Router};
use axum_extra::headers::ContentType;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
pub async fn start(config: Arc<DotEnvConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest(
            "/authentication",
            routers::authentications::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/adventurers",
            routers::adventurers::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/guild-commanders",
            routers::guild_commanders::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/crew-switchboard",
            routers::crew_switchboard::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/journey-ledger",
            routers::journey_ledger::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/quest-ops",
            routers::quest_ops::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/quest-viewing",
            routers::quest_viewing::routes(Arc::clone(&db_pool)),
        )

        .route("/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                    Method::OPTIONS,
                ])
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT,CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
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
