use std::default::Default;
use anyhow::Result;
use dotenvy::dotenv;
use super::{config_models::{Database, DotEnvConfig, Server, AdventureSecret}, stage::Stage};

pub fn load() -> Result<DotEnvConfig> {
    dotenv().ok();
    let server: Server = Server {
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is not set").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is not set").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is not set").parse()?,
    };
    let database: Database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set").parse()?,
    };

    Ok(DotEnvConfig {
        server,
        database,
    })
}

pub fn get_stage() -> Stage {
    dotenv().ok();
    let stage_str = std::env::var("STAGE").unwrap_or("local".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventure_secret_env() -> Result<AdventureSecret> {
    dotenv().ok();
    Ok(AdventureSecret {
        secret: std::env::var("JWT_ADVENTURE_SECRET").expect("JWT_ADVENTURE_SECRET is not set").parse()?,
        refresh_secret: std::env::var("JWT_ADVENTURE_REFRESH_SECRET").expect("JWT_ADVENTURE_REFRESH_SECRET is not set").parse()?,
    })
}
pub fn get_guild_commander_secret_env() -> Result<AdventureSecret> {
    dotenv().ok();
    Ok(AdventureSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET").expect("JWT_GUILD_COMMANDER_SECRET is not set").parse()?,
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET").expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is not set").parse()?,
    })
}