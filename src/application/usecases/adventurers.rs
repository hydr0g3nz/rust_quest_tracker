use std::sync::Arc;

use crate::domain::{repositories::adventurers::AdventurerRepository, value_objects::adventutrt_models::RegisterAdverturerModel};
use anyhow::Result;
pub struct AdventurersUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    pub adventurers_repository: Arc<T>,
}
impl <T>AdventurersUseCase<T> where T: AdventurerRepository + Send + Sync {
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }
    pub async fn register(&self, mut register_adventurer_model:RegisterAdverturerModel) -> Result<i32> {
        unimplemented!()
    }
}