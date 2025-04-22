use core::config;
use core::db;
use core::errors::AppError;
use core::logger::Logger;

pub struct AnalyticsService {
    srv_config: config::ServerConfig,
    db_manager: db::DBManager,
}

impl AnalyticsService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        srv_config.validate()?;

        let db_manager = db::DBManager::new(srv_config.clone())?;

        Ok(AnalyticsService {
            srv_config,
            db_manager,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        Ok(())
    }
}
