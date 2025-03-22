use crate::AppError;
use tokio_postgres::{Client, Row};

pub struct BatchRepository<'a> {
    pub client: &'a mut Client,
}

impl<'a> BatchRepository<'a> {
    // Insert a new batch and return the batch ID
    pub async fn insert_batch(&mut self, batch_name: &str) -> Result<i32, AppError> {
        let row: Row = self.client
            .query_one(
                "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), 'Started') RETURNING id",
                &[&batch_name],
            )
            .await?;

        Ok(row.get("id"))
    }

    // Update batch status and set end_time using the batch ID
    pub async fn update_batch(&mut self, batch_id: i32, status: &str) -> Result<u64, AppError> {
        let rows_affected = self
            .client
            .execute(
                "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2",
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
