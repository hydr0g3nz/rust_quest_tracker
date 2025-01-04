use serde::{Serialize, Deserialize};
use crate::domain::entities::adventurers::RegisterAdventurerEntity;
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RegisterAdverturerModel {
    pub username : String,
    pub password :String,
}
impl RegisterAdverturerModel {
    pub fn to_entity(&self) -> RegisterAdventurerEntity {
        RegisterAdventurerEntity {
            username : self.username.clone(),
            password : self.password.clone(),
            created_at : chrono::Utc::now().naive_utc(),
            updated_at : chrono::Utc::now().naive_utc(),
            
        }
    }
}