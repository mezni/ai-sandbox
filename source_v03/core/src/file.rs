// core/src/file/mod.rs
use crate::errors::AppError;
use std::path::Path;

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        FileManager
    }

    pub fn check_env_file(&self) -> Result<(), AppError> {
        let env_path = Path::new(".env");

        if !env_path.exists() {
            return Err(AppError::FileNotFound(".env".to_string()));
        }

        Ok(())
    }

    pub fn check_config_file(&self) -> Result<(), AppError> {
        let env_path = Path::new("config.yaml");

        if !env_path.exists() {
            return Err(AppError::FileNotFound("config.yaml".to_string()));
        }

        Ok(())
    }
}
