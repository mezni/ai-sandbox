use core::config;
use core::errors::AppError;

const CONFIG_FILE: &str = "config.yaml";

pub struct LoadService {
    srv_config: config::ServerConfig,
    app_config: config::AppConfig,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        let app_config = config::read_app_config(CONFIG_FILE)?;

        Ok(LoadService {
            srv_config,
            app_config,
        })
    }
}
