use core::config;
use core::db;
use core::errors::AppError;
use core::file;

const CONFIG_FILE: &str = "config.yaml";

pub struct LoadService {
    srv_config: config::ServerConfig,
    app_config: config::AppConfig,
    db_manager: db::DBManager,
    file_manager: file::FileManager,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let srv_config = config::read_srv_config()?;
        srv_config.validate()?;

        let app_config = config::read_app_config(CONFIG_FILE)?;
        app_config.validate()?;

        let db_manager = db::DBManager::new(srv_config.clone())?;
        let file_manager = file::FileManager::new();

        Ok(LoadService {
            srv_config,
            app_config,
            db_manager,
            file_manager,
        })
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let db_client = self.db_manager.get_client().await?;
        let result = self.file_manager.execute(self.app_config.clone()).await?;

        if let Some(roam_in_data) = result {
            // You can use roam_in_data.metadata and roam_in_data.records here
            println!("Parsed RoamInData: {:?}", roam_in_data);
            // e.g., insert into DB using db_client
        }

        Ok(())
    }
}
