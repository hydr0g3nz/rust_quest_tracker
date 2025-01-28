use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::domain::entities::quests::{AddQuestEntity, EditQuestEntity};

use super::quest_statuses::QuestStatuses;
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct  QuestModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub adventurers_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct  AddQuestModel {
    pub name: String,
    pub description: Option<String>,
    pub guild_commander_id: i32,
}

impl AddQuestModel {
    pub fn to_entity(&self, guild_commander_id: i32) -> AddQuestEntity {
        AddQuestEntity{
            name: self.name.clone(),
            description: self.description.clone(),
            guild_commander_id: self.guild_commander_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            status: QuestStatuses::Open.to_string(),
            deleted_at: None
        }
    }
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct  EditQuestModel {
    pub name: String,
    pub description: Option<String>,
}

impl EditQuestModel {
    pub fn to_entity(&self, guild_commander_id: i32) -> EditQuestEntity {
        EditQuestEntity{
            name: self.name.clone(),
            description: self.description.clone(),
            guild_commander_id,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
