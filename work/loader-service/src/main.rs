mod db;
mod errors;
mod repositories;
mod service;

use crate::errors::AppError;
use crate::service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let service = LoadService::new().await?;
    service.load("TEST1").await?;
    Ok(())
}
