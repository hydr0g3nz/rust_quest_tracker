use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCases, domain::{
        repositories::guild_commanders::GuildCommanderRepository,
        value_objects::guild_commander_models::RegisterGuildCommanderModel,
    }, infra::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    }
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commander_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commanders_usecase =
        GuildCommandersUseCases::new(Arc::new(guild_commander_repository));
    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_usecase))
}
pub async fn register<T>(
    State(guild_commanders_usecase): State<Arc<GuildCommandersUseCases<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommanderRepository + Send + Sync,
{
    println!("model {:?}", register_guild_commander_model);
    match guild_commanders_usecase
    .register(register_guild_commander_model)
    .await
{
    Ok(guid_commabder_id) => (
        StatusCode::CREATED,
        format!("Created guild commander id: {}", guid_commabder_id),
    )
        .into_response(),
    Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
}
}
