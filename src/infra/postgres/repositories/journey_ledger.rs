use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

use crate::{
    domain::repositories::journey_ledger::JourneyLedgerRepository,
    infra::postgres::postgres_connection::PgPoolSquad,
};

pub struct JourneyLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JourneyLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepository for JourneyLedgerPostgres {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!();
    }

    async fn to_complete(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!();
    }

    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!();
    }
}
