use std::sync::Arc;

use crate::{domain::{
    repositories::guild_commanders::GuildCommanderRepository,
    value_objects::guild_commander_models::{self, RegisterGuildCommanderModel},
}, infra::argon2_hashing};
use anyhow::Result;
pub struct GuildCommandersUseCases<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    guild_commander_repository: Arc<T>,
}

impl<T> GuildCommandersUseCases<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    pub fn new(guild_commander_repository: Arc<T>) -> Self {
        Self {
            guild_commander_repository,
        }
    }
    pub async fn register(
        &self,
        mut register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_guild_commander_model.password.clone())?;
        register_guild_commander_model.password = hashed_password;
        let guild_commander_id = self
            .guild_commander_repository
            .register(register_guild_commander_model.to_entity())
            .await?;
        Ok(guild_commander_id)
    }
}
