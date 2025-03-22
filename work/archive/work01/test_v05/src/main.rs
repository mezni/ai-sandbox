use env_logger;
use log::{error, info};
mod database;
mod errors;
use errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env_logger::init();
    info!("Starting microservice...");

    // Establish connection using BatchRepository
    let repo = database::BatchRepository::new().await?;

    // Insert a batch
    let batch_id = repo.insert_batch("Sample1").await?;
    info!("Batch inserted with ID: {}", batch_id);

    // Update batch status to Success
    match repo.update_batch(batch_id, "Success").await {
        Ok(_) => info!("Batch status updated to Success."),
        Err(e) => error!("Failed to update batch: {}", e),
    }

    Ok(())
}
