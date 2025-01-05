use crate::{
    domain::{
        entities::adventurers::AdventurerEntity,
        repositories::{
            adventurers::AdventurerRepository, crew_switchboard::CrewSwitchboardRepository,
        },
        value_objects::{
            adventutrt_models::RegisterAdverturerModel,
            quest_adventurer_junction::QuestAdventurerJunction,
        },
    },
    infra::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchboardPostgres {
    async fn join(&self, juntion_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!();
    }

    async fn leave(&self, juntion_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!();
    }
}
