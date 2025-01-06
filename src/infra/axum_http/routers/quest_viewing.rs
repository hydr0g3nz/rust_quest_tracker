use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

use std::sync::Arc;

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCases,
    domain::{repositories::quest_viewing::QuestViewingRepository, value_objects::board_checking_filter::BoardCheckingFilter},
    infra::postgres::{
        postgres_connection::PgPoolSquad, repositories::quest_viewing::QuestViewingPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_use_cases = QuestViewingUseCases::new(Arc::new(quest_viewing_repository));
    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board_checking", get(board_checking))
        .with_state(Arc::new(quest_viewing_use_cases))
}
pub async fn view_details<T>(
    State(quest_viewing_usecase): State<Arc<QuestViewingUseCases<T>>>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn board_checking<T>(
    State(quest_viewing_usecase): State<Arc<QuestViewingUseCases<T>>>,
    filter: Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
