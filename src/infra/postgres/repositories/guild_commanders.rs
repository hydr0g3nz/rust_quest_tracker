use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repositories::guild_commanders::GuildCommanderRepository,
    },
    infra::postgres::{postgres_connection::PgPoolSquad, schemas::guild_commanders},
};
use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;
use diesel::{dsl::insert_into, prelude::*};
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
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(guild_commanders::table)
            .values(&body)
            .returning(guild_commanders::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }
    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = guild_commanders::table
            .filter(guild_commanders::username.eq(username))
            .select(GuildCommanderEntity::as_select())
            .get_result(&mut conn)?;
        Ok(result)
    }
}
