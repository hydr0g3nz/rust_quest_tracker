use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{domain::value_objects::quest_model::QuestModel, infra::postgres::schemas::{quest_adventurer_junction::adventurer_id, quests}};

#[derive(Debug, Clone,Identifiable,Selectable,Queryable)]
#[diesel(table_name =quests)]
pub struct QuestEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}
impl QuestEntity {
    pub fn to_model(&self,adventurers_count: i64) -> QuestModel {
        QuestModel{
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            guild_commander_id: self.guild_commander_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            adventurers_count
    }
}}
#[derive(Debug, Clone,Insertable,Queryable)]
#[diesel(table_name =quests)]
pub struct AddQuestEntity {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}
#[derive(Debug, Clone,AsChangeset,Queryable)]
#[diesel(table_name =quests)]
pub struct EditQuestEntity {
    pub name: String,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime,
}
