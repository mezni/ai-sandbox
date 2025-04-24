use chrono::Utc;
use core::errors::AppError;
use tokio_postgres::Client;

pub async fn get_max_date(client: &Client, direction: &str) -> Result<String, AppError> {
    let query = "
        SELECT MAX(date_str)
        FROM v_metrics
        WHERE metric = $1
    ";

    let row = match client.query_one(query, &[&direction]).await {
        Ok(r) => r,
        Err(_) => {
            let today = Utc::now().date_naive().to_string();
            return Ok(today);
        }
    };

    let max_date: Option<String> = row.get(0);
    Ok(max_date.unwrap_or_else(|| Utc::now().date_naive().to_string()))
}
