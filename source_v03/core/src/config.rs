use crate::errors::AppError;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub dbname: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            dbname: None,
            user: None,
            password: None,
            host: None,
        }
    }
}

pub fn read_srv_config() -> Result<ServerConfig, AppError> {
    // Load environment variables from the .env file.
    if let Err(err) = dotenv() {
        return Err(AppError::FileNotFound(".env".to_string()));
    }

    let mut cfg = ServerConfig::new();

    cfg.dbname = Some(env::var("DB_NAME").map_err(|_| AppError::MissingEnvVar("DB_NAME".into()))?);
    cfg.user = Some(env::var("DB_USER").map_err(|_| AppError::MissingEnvVar("DB_USER".into()))?);
    cfg.password =
        Some(env::var("DB_PASSWORD").map_err(|_| AppError::MissingEnvVar("DB_PASSWORD".into()))?);
    cfg.host = Some(env::var("DB_HOST").map_err(|_| AppError::MissingEnvVar("DB_HOST".into()))?);

    Ok(cfg)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub sources: Vec<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub source_type: String,
    pub source_directory: String,
    pub file_pattern: Option<String>,
    pub post_action: Option<String>,
    pub archive_directory: Option<String>,
}

pub fn read_app_config(file_path: &str) -> Result<AppConfig, AppError> {
    let file = File::open(Path::new(file_path))?;
    let config = serde_yaml::from_reader(file).map_err(AppError::YamlError)?;
    Ok(config)
}
