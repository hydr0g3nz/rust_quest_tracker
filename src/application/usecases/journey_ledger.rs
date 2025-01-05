use std::sync::Arc;

use crate::domain::repositories::{
    journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
};
use anyhow::Result;

pub struct JourneyLedger<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> JourneyLedger<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            journey_ledger_repository,
            quest_viewing_repository,
        }
    }
    pub async fn in_the_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<bool> {
        unimplemented!()
    }
    pub async fn to_complete(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
    pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
}
