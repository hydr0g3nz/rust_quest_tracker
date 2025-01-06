use std::sync::Arc;

use crate::{
    domain::{
        repositories::adventurers::AdventurerRepository,
        value_objects::adventutrt_models::RegisterAdverturerModel,
    },
    infra::argon2_hashing,
};
use anyhow::{Ok, Result};
pub struct AdventurersUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    pub adventurers_repository: Arc<T>,
}
impl<T> AdventurersUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }
    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdverturerModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_adventurer_model.password.clone())?;
        register_adventurer_model.password = hashed_password;
        let adventurer_id = self
            .adventurers_repository
            .register(register_adventurer_model.to_entity())
            .await?;
        Ok(adventurer_id)
    }
}
