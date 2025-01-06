use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::{
        repositories::adventurers::AdventurerRepository,
        value_objects::adventutrt_models::RegisterAdverturerModel,
    },
    infra::postgres::{
        postgres_connection::PgPoolSquad, repositories::adventurers::AdventurerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(db_pool);
    let adventurers_usecase = AdventurersUseCase::new(Arc::new(adventurers_repository));
    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_usecase))
}
pub async fn register<T>(
    State(adventurers_usecase): State<Arc<AdventurersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdverturerModel>,
) -> impl IntoResponse
where
    T: AdventurerRepository + Send + Sync,
{
    match adventurers_usecase
        .register(register_adventurer_model)
        .await
    {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Created adventurer id: {}", adventurer_id),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
