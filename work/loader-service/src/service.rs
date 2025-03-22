use crate::db::connection;
use crate::errors::AppError;
use crate::repositories::BatchRepository;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub struct LoadService {
    client: Arc<Mutex<Client>>,
    batch_repo: BatchRepository,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let client = connection().await?;
        let client_arc = Arc::new(Mutex::new(client));

        let batch_repo = BatchRepository {
            client: Arc::clone(&client_arc),
        };

        Ok(LoadService {
            client: client_arc,
            batch_repo,
        })
    }

    pub async fn load(&self, file_path: &str) -> Result<(), AppError> {
        println!("Starting batch for file: {}", file_path);

        // Insert batch and retrieve the batch ID
        let batch_id = match self.batch_repo.insert_batch(file_path).await {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to insert batch: {}", e);
                return Err(e);
            }
        };

        println!("Inserted batch with ID: {}", batch_id);

        // Update the batch to Completed
        if let Err(e) = self.batch_repo.update_batch(batch_id, "Completed").await {
            eprintln!("Failed to update batch {}: {}", batch_id, e);
            return Err(e);
        }

        println!("Batch {} marked as Completed.", batch_id);
        Ok(())
    }
}
