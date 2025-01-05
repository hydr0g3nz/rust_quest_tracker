use std::sync::Arc;

use crate::domain::{
    repositories::guild_commanders::GuildCommanderRepository,
    value_objects::guild_commander_models::{self, RegisterGuildCommanderModel},
};
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
        register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        unimplemented!()
    }
}
