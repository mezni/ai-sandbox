use core::errors::AppError;
use tokio_postgres::Client;

pub async fn get_number_subscribers_in(client: &Client) -> Result<Vec<(String)>, AppError> {
    let query = "select date_str, value from v_metrics where metric_name = 'number_subscribers_in'";
    let rows = client
        .query(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let date: String = row.get("date");
            let count: i64 = row.get("count");
            (date, count)
        })
        .collect();

    Ok(results)
}