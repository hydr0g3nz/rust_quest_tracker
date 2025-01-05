use crate::domain::entities::guild_commanders::{
    GuildCommanderEntity, RegisterGuildCommanderEntity,
};

use anyhow::Result;
use axum::async_trait;
use mockall::automock;
#[async_trait]
#[automock]
pub trait GuildCommanderRepository {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity,
    ) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity>;
}
