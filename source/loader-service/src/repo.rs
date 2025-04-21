use core::errors::AppError;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Debug, Serialize)]
pub struct RoamInDataDBRecord {
    pub batch_id: i32,
    pub batch_date: String,
    pub hlraddr: String,
    pub nsub: i32,
    pub nsuba: i32,
}

pub async fn insert_batch_exec(
    db_client: &Client,
    batch_name: &str,
    source_type: &str,
    source_name: &str,
) -> Result<i32, AppError> {
    let batch_status = "Started";

    let row = db_client
        .query_one(
            "INSERT INTO batch_execs (batch_name, source_type, source_name, start_time, batch_status)
             VALUES ($1, $2, $3, NOW(), $4)
             RETURNING id",
            &[&batch_name, &source_type, &source_name, &batch_status],
        )
        .await?;

    let id: i32 = row.get("id");
    Ok(id)
}

pub async fn insert_roam_in_stg_records(
    db_client: &Client,
    records: Vec<RoamInDataDBRecord>,
) -> Result<(), AppError> {
    let query = "
        INSERT INTO stg_roam_in (batch_id, batch_date, hlraddr, nsub, nsuba)
        VALUES ($1, $2, $3, $4, $5)
    ";

    for record in records {
        db_client
            .execute(
                query,
                &[
                    &(record.batch_id as i32),
                    &record.batch_date,
                    &record.hlraddr,
                    &(record.nsub as i32),
                    &(record.nsuba as i32),
                ],
            )
            .await?;
    }

    Ok(())
}
