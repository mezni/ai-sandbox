mod errors;
mod service;

use crate::errors::AppError;
use crate::service::LoadService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");
    let service = LoadService::new().await?;
    Ok(())
}
