use crate::errors::AppError;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls};

#[derive(Debug)]
pub struct RoamOutDBRecord {
    pub batch_id: i32,
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
}

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub async fn new() -> Result<Self, AppError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("Database connection error: {}", e);
            }
        });

        Ok(PostgresClient { client })
    }

    pub async fn insert_stg_roam_out(&self, records: Vec<RoamOutDBRecord>) -> Result<(), AppError> {
        for record in &records {
            self.client
                .execute(
                    "INSERT INTO stg_roam_out (batch_id, imsi, msisdn, vlr_number) VALUES ($1, $2, $3, $4)",
                    &[&record.batch_id, &record.imsi, &record.msisdn, &record.vlr_number],
                )
                .await
                .map_err(AppError::from)?;
        }

        log::info!("Inserted {} records into the stg_roam_out table.", records.len());
        Ok(())
    }
}
