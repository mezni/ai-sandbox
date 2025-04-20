
use core::logger::Logger;
use core::errors::AppError;
use file::file::FileManager;
use std::process;

const SERVICE_NAME: &str = "Loader";

pub struct LoaderService;

impl LoaderService {
    pub fn run(&self) {
        Logger::info(&format!("{} - start service", SERVICE_NAME));
    }
}
#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Start process");

    let file_manager = FileManager::new();
    let (env_exists, config_exists) = file_manager.check_config_files();

    if !env_exists {
        Logger::error("Config file does not exists: .env ");
        Logger::info("Exit process");
        process::exit(1);
    }

    if !config_exists {
        Logger::error("Config file does not exists: config.yaml ");
        Logger::info("Exit process");
        process::exit(1);
    }

    let service = LoaderService;
    service.run();

    Logger::info("Stop process");
    Ok(())
}
