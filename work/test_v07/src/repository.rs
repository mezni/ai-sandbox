use crate::errors::AppError;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls, Row};

pub struct Repository {
    client: Client,
}

impl Repository {
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

    pub async fn select_carriers(&self) -> Result<Vec<Row>, AppError> {
        let query = "
        WITH CTE AS (
            SELECT id, carrier_id, carrier_name, country_name, country_code, national_destination_code, 
                   country_code || national_destination_code AS code,
                   ROW_NUMBER() OVER (PARTITION BY country_code || national_destination_code ORDER BY id DESC) AS rn
            FROM dim_carriers
            WHERE national_destination_code IS NOT NULL
        )
        SELECT country_name, carrier_id, carrier_name, country_code, national_destination_code, code
        FROM CTE 
        WHERE rn = 1
        ORDER BY country_name, carrier_id;
        ";

        let statement = self.client.prepare(query).await?;
        let rows = self.client.query(&statement, &[]).await?;
        Ok(rows)
    }
}
