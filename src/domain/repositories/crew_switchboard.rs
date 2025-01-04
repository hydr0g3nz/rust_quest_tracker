use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::value_objects::quest_adventurer_junction::QuestAdventurerJunction;
#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository{
async  fn join(&self,juntion_body:QuestAdventurerJunction) -> Result<()>;
async  fn leave(&self,juntion_body:QuestAdventurerJunction) -> Result<()>;
}