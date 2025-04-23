use core::errors::AppError;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

pub async fn get_next_batch_id(db_client: &Client) -> Result<Option<(i32, String)>, AppError> {
    let query = "
        SELECT batch_id, source_type
        FROM batch_execs
        WHERE batch_id = (
            SELECT MIN(batch_id)
            FROM (
                SELECT batch_id
                FROM batch_execs 
                WHERE batch_name = 'loader-srv'
                AND batch_status = 'Success'
                EXCEPT
                SELECT corr_id AS batch_id
                FROM batch_execs 
                WHERE batch_name = 'analytics-srv'
                AND batch_status = 'Success'
            ) AS unmatched_ids
        )
    ";

    let row = db_client
        .query_opt(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = row.map(|r| {
        let id: i32 = r.get(0);
        let source_type: String = r.get(1);
        (id, source_type)
    });

    Ok(result)
}

pub async fn insert_batch_exec(
    db_client: &Client,
    batch_name: &str,
    source_type: &str,
    corr_id: i32,
) -> Result<i32, AppError> {
    let batch_status = "Started";

    let row = db_client
        .query_one(
            "INSERT INTO batch_execs (batch_name, source_type, corr_id, start_time, batch_status)
             VALUES ($1, $2, $3, NOW(), $4)
             RETURNING batch_id",
            &[&batch_name, &source_type, &corr_id, &batch_status],
        )
        .await?;

    let batch_id: i32 = row.get("batch_id");
    Ok(batch_id)
}


pub async fn update_batch_status(
    db_client: &Client,
    batch_id: i32,
    batch_status: &str,
) -> Result<(), AppError> {
    db_client
        .execute(
            "UPDATE batch_execs
             SET batch_status = $1, end_time = NOW()
             WHERE batch_id = $2",
            &[&batch_status, &batch_id],
        )
        .await?;

    Ok(())
}

pub async fn insert_roam_in_metrics(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    Ok(())
}

pub async fn insert_roam_out_metrics(db_client: &Client, corr_id: i32) -> Result<(), AppError> {
    Ok(())
}
