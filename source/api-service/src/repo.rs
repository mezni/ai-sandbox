use core::errors::AppError;
use serde_json::Value;
use tokio_postgres::{Client, Row};

const SELECT_LAST_DATE_QUERY: &str = "SELECT date_text
FROM dim_time
WHERE id = (
  SELECT MAX(date_id)
  FROM fct_roam_out
)";

const SELECT_LAST_ROAM_OUT_QUERY: &str = "
WITH latest AS (
  SELECT date_id, batch_id
  FROM fct_roam_out
  ORDER BY date_id DESC, batch_id DESC
  LIMIT 1
)
SELECT COUNT(*) AS cnt
FROM fct_roam_out t
JOIN latest l USING (date_id, batch_id)
";

const SELECT_ROAM_OUT_COUNTS_QUERY: &str = "
SELECT
  dt.date_text   AS date,
  COUNT(*)       AS count
FROM fct_roam_out fo
JOIN dim_time dt ON fo.date_id = dt.id
GROUP BY dt.date_text
ORDER BY dt.date_text;
";

const SELECT_COUNT_ROAM_IN_QUERY: &str = "SELECT COUNT(*) FROM fct_roam_in";

const SELECT_LAST_ANOMALIES_QUERY: &str = "WITH latest AS (
  SELECT date_id, batch_id
  FROM fct_roam_out
  ORDER BY date_id DESC, batch_id DESC
  LIMIT 1
)
SELECT COUNT(*) AS cnt
FROM notifications t
JOIN latest l USING (date_id, batch_id)";

const SELECT_LAST_NOTIFICATIONS_QUERY: &str = "WITH latest AS (
  SELECT date_id, batch_id
  FROM fct_roam_out
  ORDER BY date_id DESC, batch_id DESC
  LIMIT 1
)
SELECT COUNT(DISTINCT rule_id) AS cnt
FROM notifications t
JOIN latest l USING (date_id, batch_id)";

pub async fn last_date(client: &Client) -> Result<String, AppError> {
    let row = client
        .query_one(SELECT_LAST_DATE_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: String = row.get(0);
    Ok(result)
}

pub async fn count_last_roam_out(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_LAST_ROAM_OUT_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: i64 = row.get(0);
    Ok(result)
}

pub async fn count_last_roam_in(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_COUNT_ROAM_IN_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: i64 = row.get(0);
    Ok(result)
}

pub async fn count_anomalies(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_LAST_ANOMALIES_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: i64 = row.get(0);
    Ok(result)
}

pub async fn count_notifications(client: &Client) -> Result<i64, AppError> {
    let row = client
        .query_one(SELECT_LAST_NOTIFICATIONS_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result: i64 = row.get(0);
    Ok(result)
}

pub async fn fetch_stats(
    client: &Client,
    query: String,
) -> Result<Vec<(Option<String>, Option<String>, Option<String>, i64)>, AppError> {
    println!("Generated stats query: {}", query);

    let rows = client
        .query(query.as_str(), &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let date = row.try_get("date").ok();
            let country = row.try_get("country").ok();
            let operator = row.try_get("operator").ok();
            let count: i64 = row.get("count");

            (date, country, operator, count)
        })
        .collect();

    Ok(results)
}
