use core::config;

use core::errors::AppError;

pub struct LoadService {
    srv_config: config::ServerConfig,
    app_config: config::AppConfig,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {

        let srv_config = config::read_srv_config()
            .map_err(|e| {
                println!("Application error: {}", e);
                e
            })?;

        let config_file = "config.yaml";
        let app_config =  config::read_app_config(config_file) 
            .map_err(|e| {
                e
            })?;       

        Ok(LoadService {
            srv_config: srv_config,
            app_config: app_config,
        })
    }
}
