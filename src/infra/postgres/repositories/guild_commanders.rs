use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repositories::guild_commanders::GuildCommanderRepository,
    },
    infra::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

pub struct GuildCommanderPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommanderPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}
#[async_trait]
impl GuildCommanderRepository for GuildCommanderPostgres {
    async fn register(&self, body: RegisterGuildCommanderEntity) -> Result<i32> {
        unimplemented!();
    }
    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        unimplemented!();
    }
}
