use std::sync::Arc;

use axum::{
    extract::{Path, State}, middleware, response::IntoResponse, routing::{delete, post}, Extension, Router
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCases,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infra::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            crew_switchboard::CrewSwitchboardPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};

use super::middleware::adventurers_authorization;

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_usecase = CrewSwitchboardUseCases::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );
    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .route_layer(middleware::from_fn(adventurers_authorization))
        .with_state(Arc::new(crew_switchboard_usecase))
}

pub async fn join<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCases<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn leave<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCases<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
