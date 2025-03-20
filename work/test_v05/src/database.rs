use crate::errors::AppError;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls, Row};

pub struct BatchRepository {
    client: Client,
}

impl BatchRepository {
    pub async fn new() -> Result<Self, AppError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("Database connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn insert_batch(&self, batch_name: &str) -> Result<i32, AppError> {
        let row: Row = self.client
            .query_one(
                "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), 'Started') RETURNING id",
                &[&batch_name],
            )
            .await?;

        Ok(row.get("id"))
    }

    pub async fn update_batch(&self, batch_id: i32, status: &str) -> Result<u64, AppError> {
        let rows_affected = self.client
            .execute(
                "UPDATE batch_execs SET batch_status = $1 WHERE id = $2",
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
