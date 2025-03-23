mod config;
mod errors;
mod service;
mod store;

use crate::config::read_config;
use crate::errors::AppError;
use crate::service::LoadService;

use env_logger;
use log::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();
    info!("Application started");    
    println!("Hello, world!");
    let config_file = "config.yaml";
    let config = read_config(config_file).unwrap();
    let service = LoadService::new(config).await?;
    service.execute().await?;
    Ok(())
}
