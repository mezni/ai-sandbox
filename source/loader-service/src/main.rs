mod service;

use core::errors::AppError;
use core::logger::Logger;
use service::LoadService;

use std::process;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Starting process");

    let service = match LoadService::new().await {
        Ok(srv) => {
            Logger::info("Config loaded");
            srv
        }
        Err(e) => {
            Logger::error(&format!("{}", e));
            Logger::info("Stopping process");
            process::exit(1);
        }
    };

    if let Err(e) = service.execute().await {
        Logger::error(&format!("Execution failed: {}", e));
    }

    Logger::info("Stopping process");

    Ok(())
}
