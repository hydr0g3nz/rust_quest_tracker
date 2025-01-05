use std::sync::Arc;
use anyhow::Result;
use crate::domain::repositories::{
    adventurers::AdventurerRepository, guild_commanders::GuildCommanderRepository,
};

pub struct AuthenticationsUseCases<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    adventurer_repository: Arc<T1>,
    guild_commander_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationsUseCases<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    pub fn new(adventurer_repository: Arc<T1>, guild_commander_repository: Arc<T2>) -> Self {
        Self {
            adventurer_repository,
            guild_commander_repository,
        }
    }
    pub async fn adventurer_login(&self, username: String, password: String) -> Result<i32> {
        unimplemented!()
    }
    pub async fn guild_commander_login(&self, username: String, password: String) -> Result<i32> {
        unimplemented!()
    }
    pub async fn adventurer_refresh_token(&self, token: String) -> Result<i32> {
        unimplemented!()
    }
    pub async fn guild_commander_refresh_token(&self, token: String) -> Result<i32> {
        unimplemented!()
    }
}
