use crate::AppError;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub struct BatchRepository {
    pub client: Arc<Mutex<Client>>,
}

impl BatchRepository {
    pub async fn insert_batch(&self, batch_name: &str) -> Result<i32, AppError> {
        let client = self.client.lock().await;
        let row = client
            .query_one(
                "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), 'Started') RETURNING id",
                &[&batch_name],
            )
            .await?;

        Ok(row.get("id"))
    }

    pub async fn update_batch(&self, batch_id: i32, status: &str) -> Result<u64, AppError> {
        let mut client = self.client.lock().await;
        let rows_affected = client
            .execute(
                "UPDATE batch_execs SET batch_status = $1, end_time=NOW() WHERE id = $2",
                &[&status, &batch_id],
            )
            .await?;

        if rows_affected == 0 {
            return Err(AppError::Unexpected(format!(
                "No batch found with ID: {}",
                batch_id
            )));
        }

        Ok(rows_affected)
    }
}
