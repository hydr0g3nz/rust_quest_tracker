use std::sync::Arc;

use crate::{
    domain::{entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity}, repositories::adventurers::AdventurerRepository, value_objects::adventutrt_models::RegisterAdverturerModel},
    infra::postgres::{postgres_connection::PgPoolSquad, schemas::adventurers::{self, table}},
};
use anyhow::Result;
use axum::async_trait;
use diesel::{dsl::insert_into, prelude::*};

pub struct AdventurerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}
#[async_trait]
impl AdventurerRepository for AdventurerPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(adventurers::table)
            .values(&register_adventurer_entity)
            .returning(adventurers::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .get_result(&mut conn)?;
        Ok(result)
    }
}
