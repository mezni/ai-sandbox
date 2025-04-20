mod service;

use service::LoadService;

use core::config;
use core::errors::AppError;
use core::file::FileManager;
use core::logger::Logger;

use std::process;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Starting process");

    let srv_config = match config::read_srv_config() {
        Ok(cfg) => {
            Logger::info(&format!("Server Config loaded"));
            cfg
        }
        Err(e) => {
            Logger::error(&format!("{}", e));
            Logger::info("Stop.");
            process::exit(1);
        }
    };

    let config_file = "config.yaml";
    let app_config = match config::read_app_config(config_file) {
        Ok(cfg) => {
            Logger::info(&format!("App Config loaded"));
            cfg
        }
        Err(e) => {
            Logger::error(&format!("{}", e));
            Logger::info("Stop.");
            process::exit(1);
        }
    };

    let service = LoadService::new();

    Logger::info("Stopping process");

    Ok(())
}
