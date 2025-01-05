use std::sync::Arc;
use anyhow::Result;
use crate::domain::{
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{AddQuestModel, EditQuestModel},
};

pub struct QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    quest_ops_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository,
        }
    }
    pub async fn add(
        &self,
        guid_commander_id: i32,
        addd_quest_model: AddQuestModel,
    ) -> Result<i32> {
        unimplemented!()
    }
    pub async fn edit(
        &self,
        quest_id: i32,
        guild_commander_id: i32,
        edit_quest_model: EditQuestModel,
    ) -> Result<i32> {
        unimplemented!()
    }
    pub async fn delete(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        unimplemented!()
    }
}
