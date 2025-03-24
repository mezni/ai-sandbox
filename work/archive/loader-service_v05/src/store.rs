use crate::errors::AppError;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use tokio_postgres::{Client, NoTls, Row};

#[derive(Debug)]
pub struct RoamOutDBRecord {
    pub batch_id: i32,
    pub batch_date: String,
    pub imsi: String,
    pub msisdn: String,
    pub vlr_number: String,
    pub carrier_name: String,
    pub country_name: String,
}

pub async fn connection() -> Result<Client, AppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| AppError::Unexpected("DATABASE_URL not set".to_string()))?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|err| {
            error!("Failed to connect to database: {}", err);
            AppError::Unexpected(format!("Database connection failed: {}", err))
        })?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection error: {}", e);
        }
    });

    info!("Database connected successfully!");
    Ok(client)
}

pub async fn insert_batch_execs(client: &Client, path_name: String) -> Result<i32, AppError> {
    let status = "Started";
    match client
        .query_one(
            "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), $2) RETURNING id",
            &[&path_name, &status],
        )
        .await
    {
        Ok(row) => {
            let batch_id: i32 = row.get("id");
            info!("Batch '{}' started with ID: {}", path_name, batch_id);
            Ok(batch_id)
        }
        Err(err) => {
            error!("Failed to insert batch '{}': {}", path_name, err);
            Err(AppError::Unexpected(format!("Failed to insert batch '{}': {}", path_name, err)))
        }
    }
}

pub async fn update_batch_execs(
    client: &Client,
    batch_id: i32,
    status: &str,
) -> Result<u64, AppError> {
    match client
        .execute(
            "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2",
            &[&status, &batch_id],
        )
        .await
    {
        Ok(rows_affected) if rows_affected > 0 => {
            info!("Batch with ID {} updated to status: {}", batch_id, status);
            Ok(rows_affected)
        }
        Ok(_) => {
            error!("No batch found with ID: {}", batch_id);
            Err(AppError::NotFound(format!(
                "No batch found with ID: {}",
                batch_id
            )))
        }
        Err(err) => {
            error!(
                "Failed to update batch with ID {} to status '{}': {}",
                batch_id, status, err
            );
            Err(AppError::Unexpected(format!(
                "Failed to update batch with ID {}: {}",
                batch_id, err
            )))
        }
    }
}

pub async fn select_all_carriers(client: &Client) -> Result<Vec<Row>, AppError> {
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

    let rows = client.query(query, &[]).await?;
    Ok(rows)
}

pub async fn insert_roam_out_stg(
    client: &Client,
    db_records: Vec<RoamOutDBRecord>,
) -> Result<(), AppError> {
    let query = "
        INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number, carrier_name, country_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ";

    for record in db_records {
        client
            .execute(
                query,
                &[
                    &record.batch_id,
                    &record.batch_date,
                    &record.imsi,
                    &record.msisdn,
                    &record.vlr_number,
                    &record.carrier_name,
                    &record.country_name,
                ],
            )
            .await
            .map_err(AppError::DatabaseError)?;
    }

    Ok(())
}
