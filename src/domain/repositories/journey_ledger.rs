use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::{
    domain::value_objects::quest_adventurer_junction::QuestAdventurerJunction,
};
#[async_trait]
#[automock]
pub trait JourneyLedgerRepository {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
    async fn to_complete(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
}
