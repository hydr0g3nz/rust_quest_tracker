use crate::domain::{entities::adventurers::AdventurerEntity, value_objects::adventutrt_models::RegisterAdverturerModel};

use anyhow::Result;
use mockall::automock;
#[async_trait::async_trait]
#[automock]
pub trait AdventurerRepository {
    async fn register(&self, register_adventurer_model:RegisterAdverturerModel)->Result<i32>;
    async fn find_by_username(&self, username: String)->Result<AdventurerEntity>;
}