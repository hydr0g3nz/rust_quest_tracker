use crate::{
    domain::{
        entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infra::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}
impl QuestViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        unimplemented!();
    }
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        unimplemented!();
    }
    async fn adventurers_countig_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        unimplemented!();
    }
}
