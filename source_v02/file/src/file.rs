use corelib::errors::AppError;
use std::path::Path;

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        FileManager
    }

    pub fn check_config_files(&self) -> (bool, bool) {
        let env_exists = Path::new(".env").exists();
        let config_exists = Path::new("config.yaml").exists();
        (env_exists, config_exists)
    }
}
