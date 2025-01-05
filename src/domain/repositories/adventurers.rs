use crate::domain::{entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity}, value_objects::adventutrt_models::RegisterAdverturerModel};

use anyhow::Result;
use mockall::automock;
#[axum::async_trait]
#[automock]
pub trait AdventurerRepository {
    async fn register(&self, register_adventurer_entity:RegisterAdventurerEntity)->Result<i32>;
    async fn find_by_username(&self, username: String)->Result<AdventurerEntity>;
}