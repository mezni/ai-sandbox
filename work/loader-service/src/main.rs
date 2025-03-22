mod db;
mod errors;
mod repositories;

use crate::db::connection;
use crate::errors::AppError;
use crate::repositories::BatchRepository;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");

    let mut client = connection().await?;
    let mut repo = BatchRepository {
        client: &mut client,
    };

    let batch_id = repo.insert_batch("Test Batch").await?;
    println!("Inserted batch with ID: {}", batch_id);

    repo.update_batch(batch_id, "Completed").await?;
    println!("Batch {} marked as Completed.", batch_id);

    Ok(())
}
