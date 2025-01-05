use crate::{
    domain::{
        entities::quests::{AddQuestEntity, EditQuestEntity},
        repositories::quest_ops::QuestOpsRepository,
    },
    infra::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

pub struct QuestOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
    async fn add(&self, addd_quest_entity: AddQuestEntity) -> Result<i32> {
        unimplemented!();
    }

    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32> {
        unimplemented!();
    }

    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        unimplemented!();
    }
}
