use core::errors::AppError;
use tokio_postgres::Client;


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
