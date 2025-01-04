use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::domain::entities::adventurers::AdventurerEntity;
use crate::domain::entities::quests::QuestEntity;
use crate::infra::postgres::schemas::quest_adventurer_junction;
#[derive(Debug, Clone,Serialize,Deserialize,Insertable,Queryable,Associations)]
#[diesel(table_name =quest_adventurer_junction)]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
pub struct QuestAdventurerJunction {   
    pub quest_id: i32,
    pub adventurer_id: i32
}
