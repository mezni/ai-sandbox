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
    start_date: &str,
    end_date: &str,
    limit: &str,
) -> Result<Vec<(String, Option<String>, Option<String>, i32)>, AppError> {
    let metric_name = match resolve_metric_name(dimensions, direction) {
        Some(name) => name,
        None => return Err(AppError::BadRequest("Invalid metric or dimension".to_string())),
    };

    let limit: i64 = match limit.parse() {
        Ok(val) => val,
        Err(_) if dimensions.eq_ignore_ascii_case("GLOBAL") => 1,
        Err(_) if dimensions.eq_ignore_ascii_case("COUNTRY") => 5,
        Err(_) if dimensions.eq_ignore_ascii_case("OPERATOR") => 5,
        Err(_) => return Err(AppError::BadRequest("Limit must be a valid integer".to_string())),
    };

    let query = "
        SELECT date_str, country, operator, value
        FROM v_metrics
        WHERE metric_name = $1
          AND date_str >= $2 
          AND date_str < $3
        ORDER BY date_str DESC
        LIMIT $4
    ";

    let rows = client
        .query(query, &[&metric_name, &start_date, &end_date, &limit])
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
