use axum::{extract::{Path, State}, middleware, response::IntoResponse, routing::{delete, patch, post}, Extension, Json, Router};

use crate::{
    application::usecases::quest_ops::{self, QuestOpsUseCase},
    domain::{repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository}, value_objects::quest_model::{AddQuestModel, EditQuestModel}},
    infra::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres},
    },
};
use std::sync::Arc;

use super::middleware::guild_commanders_authorization;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let quest_ops_usecase = QuestOpsUseCase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/add", post(add))
        .route("/edit/:quest_id", patch(edit))
        .route("/delete/:quest_id", delete(remove))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(quest_ops_usecase))
}

pub async fn add<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(addd_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn edit<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(addd_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn remove<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}