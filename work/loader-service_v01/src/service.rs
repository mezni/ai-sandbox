use crate::db::connection;
use crate::errors::AppError;
use crate::repositories::BatchRepository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub struct LoadService {
    client: Arc<Mutex<Client>>,
    batch_repo: BatchRepository,
    carriers_map: HashMap<String, String>,
}

impl LoadService {
    pub async fn new() -> Result<Self, AppError> {
        let client = connection().await?;
        let client_arc = Arc::new(Mutex::new(client));

        let batch_repo = BatchRepository {
            client: Arc::clone(&client_arc),
        };

        // Initialize an empty carriers_map
        let carriers_map = HashMap::new();

        Ok(LoadService { client: client_arc, batch_repo, carriers_map })
    }

    pub async fn load(&self, file_path: &str) -> Result<(), AppError> {
        let batch_id = self.batch_repo.insert_batch(file_path).await?;
        println!("Inserted batch with ID: {}", batch_id);

        self.batch_repo.update_batch(batch_id, "Completed").await?;
        println!("Batch {} marked as Completed.", batch_id);        
        Ok(())
    }

    pub fn add_carrier(&mut self, code: String, name: String) {
        self.carriers_map.insert(code, name);
    }

    pub fn get_carrier_name(&self, code: &str) -> Option<&String> {
        self.carriers_map.get(code)
    }
}
