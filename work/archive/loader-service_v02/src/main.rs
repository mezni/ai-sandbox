mod errors;
mod config;
mod service;

use crate::errors::AppError;
use crate::config::read_config;
use crate::service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");

    let config_file = "config.yaml";
    let config = read_config(config_file).unwrap();
    let service = LoadService::new(&config).await?;
    Ok(())
}