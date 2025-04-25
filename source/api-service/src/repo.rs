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

fn resolve_metric_name(dimension: &str, direction: &str) -> Option<&'static str> {
    match (
        dimension.to_ascii_uppercase().as_str(),
        direction.to_ascii_uppercase().as_str(),
    ) {
        ("GLOBAL", "TOTIN") => Some("number_subscribers_in"),
        ("GLOBAL", "OUT") => Some("number_subscribers_out"),

        ("COUNTRY", "TOTIN") => Some("number_subscribers_in_by_country"),
        ("COUNTRY", "OUT") => Some("number_subscribers_out_by_country"),

        ("OPERATOR", "TOTIN") => Some("number_subscribers_in_by_operator"),
        ("OPERATOR", "OUT") => Some("number_subscribers_out_by_operator"),

        ("GLOBAL", "ACTIN") => Some("number_active_subscribers_in"),
        ("COUNTRY", "ACTIN") => Some("number_active_subscribers_in_by_country"),
        ("OPERATOR", "ACTIN") => Some("number_active_subscribers_in_by_operator"),

        _ => None,
    }
}

pub async fn get_metrics(
    client: &Client,
    direction: &str,
    dimensions: &str,
    kind: &str,
    limit: i32,
) -> Result<Vec<(String, Option<String>, Option<String>, i32)>, AppError> {
    let metric_name = match resolve_metric_name(dimensions, direction) {
        Some(name) => name,
        None => {
            return Err(AppError::BadRequest(
                "Invalid metric or dimension".to_string(),
            ));
        }
    };



    let query = "
        SELECT date_str, country, operator, value
        FROM v_metrics
        WHERE metric_name = $1
        ORDER BY date_str DESC
        LIMIT $2
    ";

    let rows = client
        .query(query, &[&metric_name, &limit])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| {
            (
                row.get::<_, String>(0),
                row.get::<_, Option<String>>(1),
                row.get::<_, Option<String>>(2),
                row.get::<_, i32>(3),
            )
        })
        .collect();

    Ok(result)
}
