use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Extension, Json, Router,
};
use std::sync::Arc;

use crate::{
    application::usecases::journey_ledger::JourneyLedgerUseCases,
    domain::repositories::{
        journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
    },
    infra::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_usecase = JourneyLedgerUseCases::new(
        Arc::new(journey_ledger_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/in_the_journey", post(in_the_journey))
        .route("/to_complete", post(to_complete))
        .route("/to_failed", post(to_failed))
        .with_state(Arc::new(journey_ledger_usecase))
}

pub async fn in_the_journey<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgerUseCases<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn to_failed<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgerUseCases<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
pub async fn to_complete<T1, T2>(
    State(journey_ledger_usecase): State<Arc<JourneyLedgerUseCases<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
