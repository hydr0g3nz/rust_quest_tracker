use serde::{Serialize, Deserialize};
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BoardCheckingFilter {
    pub name : Option<String>,
    pub status : Option<String>,
}
