use std::sync::Arc;

use crate::domain::{
    entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
    value_objects::board_checking_filter::BoardCheckingFilter,
};
use anyhow::Result;
pub struct QuestViewingUseCases<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCases<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        unimplemented!()
    }
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        unimplemented!()
    }
    async fn adventurers_countig_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        unimplemented!()
    }
}
