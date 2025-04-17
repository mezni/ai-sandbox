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

const SELECT_STATS_ROAM_OUT_OPERATORS_QUERY: &str = "SELECT 
  t.date_text AS date,
  c.name_en AS country,
  o.operator AS operator,
  COUNT(*) AS count
FROM fct_roam_out fct
JOIN dim_countries c ON fct.country_id = c.id
JOIN dim_time t ON fct.date_id = t.id
JOIN dim_operators o ON fct.country_id = o.country_id 
                    AND fct.operator_id = o.id
WHERE fct.batch_id = (SELECT max(batch_id) FROM fct_roam_out)
GROUP BY t.date_text, c.name_en, o.operator
ORDER BY t.date_text, c.name_en, o.operator";

const SELECT_STATS_ROAM_OUT_COUNTRIES_QUERY: &str = "SELECT 
  t.date_text AS date,
  c.name_en AS country,
  COUNT(*) AS count
FROM fct_roam_out fct
JOIN dim_countries c ON fct.country_id = c.id
JOIN dim_time t ON fct.date_id = t.id
WHERE fct.batch_id = (SELECT max(batch_id) FROM fct_roam_out)
GROUP BY t.date_text, c.name_en
ORDER BY count desc";

const SELECT_STATS_ROAM_OUT_DATES_QUERY: &str = "SELECT 
  t.date_text AS date,
  COUNT(*) AS count
FROM fct_roam_out fct
JOIN dim_time t ON fct.date_id = t.id
GROUP BY t.date_text
ORDER BY t.date_text";

const SELECT_NOTIFICATIONS_QUERY: &str = "select count || ' ' || description AS notification
FROM (
select r.description , count(*) as count 
from notifications n 
join rules r on r.id = n.rule_id
and batch_id = (select max(batch_id) from notifications) 
group by r.description
)";

const GET_ANOMALIE_SOR_DEVIATION_QUERY: &str = "
select c.name_en, o.operator,  f.country_count , f.operator_count , p.rate as configure, percent as reel, p.routage  
from fct_sor_out f 
join sor_plan p on f.country_id = p.country_id and f.operator_id = p.operator_id
join dim_operators o on f.country_id = o.country_id and f.operator_id = o.id
join dim_countries c on f.country_id = c.id 
where f.batch_id = (SELECT max(batch_id) FROM fct_sor_out)";

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

pub async fn count_roam_out_operators(
    client: &Client,
) -> Result<Vec<(String, String, String, i64)>, AppError> {
    let rows = client
        .query(SELECT_STATS_ROAM_OUT_OPERATORS_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let date: String = row.get("date");
            let country: String = row.get("country");
            let operator: String = row.get("operator");
            let count: i64 = row.get("count");
            (date, country, operator, count)
        })
        .collect();

    Ok(results)
}

pub async fn count_roam_out_countries(
    client: &Client,
) -> Result<Vec<(String, String, i64)>, AppError> {
    let rows = client
        .query(SELECT_STATS_ROAM_OUT_COUNTRIES_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let date: String = row.get("date");
            let country: String = row.get("country");
            let count: i64 = row.get("count");
            (date, country, count)
        })
        .collect();

    Ok(results)
}

pub async fn count_roam_out_dates(client: &Client) -> Result<Vec<(String, i64)>, AppError> {
    let rows = client
        .query(SELECT_STATS_ROAM_OUT_DATES_QUERY, &[])
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

pub async fn get_notifications(client: &Client) -> Result<Vec<(String)>, AppError> {
    let rows = client
        .query(SELECT_NOTIFICATIONS_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let notification: String = row.get("notification");
            (notification)
        })
        .collect();

    Ok(results)
}


pub async fn get_anomalie_sor(client: &Client) -> Result<Vec<(String,String,String,String,String,String,String,)>, AppError> {
    let rows = client
        .query(GET_ANOMALIE_SOR_DEVIATION_QUERY, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let results = rows
        .into_iter()
        .map(|row| {
            let name_en: String = row.get("name_en");
            let operator: String = row.get("operator");
            let country_count: String = row.get("country_count");
            let operator_count: String = row.get("operator_count");
            let configure: String = row.get("configure");
            let reel: String = row.get("reel");
            let routage: String = row.get("routage");
            (name_en , operator , country_count , operator_count , configure , reel  , routage)
        })
        .collect();

    Ok(results)
}


