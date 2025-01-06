use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::authentications,
    domain::repositories::{
        adventurers::AdventurerRepository, guild_commanders::GuildCommanderRepository,
    },
    infra::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{adventurers::AdventurerPostgres, guild_commanders::GuildCommanderPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(Arc::clone(&db_pool));
    let guild_commander_repository = GuildCommanderPostgres::new(Arc::clone(&db_pool));
    let authentications_usecase =
        crate::application::usecases::authentications::AuthenticationsUseCases::new(
            Arc::new(adventurers_repository),
            Arc::new(guild_commander_repository),
        );

    Router::new()
        .route("/adventurers/login", post(adventurers_login))
        .route(
            "/adventurers/refresh_token",
            post(adventurers_refresh_token),
        )
        .route("/guild_commanders/login", post(guild_commanders_login))
        .route(
            "/guild_commanders/refresh_token",
            post(guild_commanders_refresh_token),
        )
        .with_state(Arc::new(authentications_usecase))
}
pub async fn adventurers_login<T1, T2>(
    State(authentications_usecase): State<Arc<authentications::AuthenticationsUseCases<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn guild_commanders_login<T1, T2>(
    State(authentications_usecase): State<Arc<authentications::AuthenticationsUseCases<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn adventurers_refresh_token<T1, T2>(
    State(authentications_usecase): State<Arc<authentications::AuthenticationsUseCases<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn guild_commanders_refresh_token<T1, T2>(
    State(authentications_usecase): State<Arc<authentications::AuthenticationsUseCases<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}
