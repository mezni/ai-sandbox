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
    dimension: &str,
    kind: &str,
    limit: i64,
) -> Result<Vec<(String, Option<String>, Option<String>, i32)>, AppError> {
    let metric_name = resolve_metric_name(dimension, direction)
        .ok_or_else(|| AppError::BadRequest("Invalid metric or dimension".to_string()))?;

    let (filter_clause, order_clause, query_params): (
        &str,
        String,
        Vec<&(dyn tokio_postgres::types::ToSql + Sync)>,
    ) = if kind == "LATEST" {
        let filter = "AND date_str = (SELECT MAX(date_str) FROM v_metrics WHERE metric_name = $1)";
        let limit_clause = if limit > 0 { " LIMIT $2" } else { "" };

        match dimension.to_ascii_uppercase().as_str() {
            "GLOBAL" => (
                filter,
                format!("{}", limit_clause),
                if limit > 0 {
                    vec![&metric_name, &limit]
                } else {
                    vec![&metric_name]
                },
            ),
            "COUNTRY" | "OPERATOR" => (
                filter,
                format!("ORDER BY value DESC{}", limit_clause),
                if limit > 0 {
                    vec![&metric_name, &limit]
                } else {
                    vec![&metric_name]
                },
            ),
            _ => return Err(AppError::BadRequest("Invalid dimension".into())),
        }
    } else {
        let limit_clause = if limit > 0 { " LIMIT $2" } else { "" };
        (
            "",
            format!("ORDER BY date_str DESC{}", limit_clause),
            if limit > 0 {
                vec![&metric_name, &limit]
            } else {
                vec![&metric_name]
            },
        )
    };

    let query = format!(
        "SELECT date_str, country, operator, value
         FROM v_metrics
         WHERE metric_name = $1
         {}
         {}",
        filter_clause, order_clause
    );

    let rows = client
        .query(&query, &query_params)
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
