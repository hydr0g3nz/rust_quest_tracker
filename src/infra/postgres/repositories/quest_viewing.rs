use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
        value_objects::{adventutrt_models::AdventurerViewModel, board_checking_filter::BoardCheckingFilter},
    }, infra::postgres::{postgres_connection::PgPoolSquad, schemas::{adventurers, quest_adventurer_junction, quests}},

};

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quests::table
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .select(QuestEntity::as_select())
            .first::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = quests::table
            .filter(quests::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(quests::status.eq(status.to_string()));
        };

        if let Some(name) = &filter.name {
            query = query.filter(quests::name.ilike(format!("%{}%", name)));
        }

        let results = query
            .select(QuestEntity::as_select())
            .order_by(quests::created_at.desc())
            .load::<QuestEntity>(&mut conn)?;

        Ok(results)
    }

    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = quest_adventurer_junction::table
            .filter(quest_adventurer_junction::quest_id.eq(quest_id))
            .count()
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
    async fn adventurers_by_quest_id(&self, quest_id: i32) -> Result<Vec<AdventurerViewModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
    
        let results = adventurers::table
            .inner_join(quest_adventurer_junction::table
                .on(adventurers::id.eq(quest_adventurer_junction::adventurer_id)))
            .filter(quest_adventurer_junction::quest_id.eq(quest_id))
            .select((adventurers::id, adventurers::username))
            .load::<(i32, String)>(&mut conn)?
            .into_iter()
            .map(|(id, username)| AdventurerViewModel { id, username })
            .collect();
    
        Ok(results)
    }
}