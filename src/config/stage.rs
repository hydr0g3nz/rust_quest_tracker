use core::fmt;

use anyhow::Result;

#[derive(Debug, Clone,Default,PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Dev,
    Production,
} 
impl  fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Local => write!(f, "local"),
            Stage::Dev => write!(f, "dev"),
            Stage::Production => write!(f, "production"),
        }
    }
    
}
impl  Stage {
    pub fn  try_from(stage: &str) -> Result<Self> {
        match stage {
            "local" => Ok(Stage::Local),
            "dev" => Ok(Stage::Dev),
            "production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("invalid stage"))
            
        }
    }
}