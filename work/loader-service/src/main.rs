mod config;
mod errors;
mod service;
mod store;

use crate::config::read_config;
use crate::errors::AppError;
use crate::service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Hello, world!");
    let config_file = "config.yaml";
    let config = read_config(config_file).unwrap();
    let service = LoadService::new(config).await?;
    service.execute().await?;
    Ok(())
}
