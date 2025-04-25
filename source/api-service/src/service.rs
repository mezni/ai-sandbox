use crate::repo;
use chrono::{Duration, NaiveDate, Utc};
use core::db::DBManager;
use core::errors::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    pub count: i32,
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Healthy".to_string(),
    }
}

fn resolve_date_range(
    start_date: Option<&str>,
    end_date: Option<&str>,
    max_date: NaiveDate,
) -> Result<(NaiveDate, NaiveDate), AppError> {
    match (start_date, end_date) {
        (Some(s), Some(e)) => Ok((
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid start date: {}", e)))?,
            NaiveDate::parse_from_str(e, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid end date: {}", e)))?,
        )),
        (Some(s), None) => {
            let sd = NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid start date: {}", e)))?;
            Ok((sd, max_date))
        }
        (None, Some(e)) => {
            let ed = NaiveDate::parse_from_str(e, "%Y-%m-%d")
                .map_err(|e| AppError::Unexpected(format!("Invalid end date: {}", e)))?;
            Ok((ed - Duration::days(10), ed))
        }
        (None, None) => Ok((max_date - Duration::days(10), max_date)),
    }
}

pub async fn get_metrics(
    db: &DBManager,
    direction: &str,
    dimensions: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<&str>,
) -> Result<Vec<MetricsResponse>, AppError> {
    let db_client = db.get_client().await?;

    let max_date_str = repo::get_max_date(&db_client, direction).await?;
    let max_date = NaiveDate::parse_from_str(&max_date_str, "%Y-%m-%d")
        .map_err(|e| AppError::Unexpected(format!("Invalid max date: {}", e)))?;

    let (sd, ed) = resolve_date_range(start_date, end_date, max_date)?;
    let sd_str = sd.format("%Y-%m-%d").to_string();
    let ed_str = ed.format("%Y-%m-%d").to_string();

    let limit_str = limit.unwrap_or("10");

    let rows = repo::get_metrics(
        &db_client, direction, dimensions, &sd_str, &ed_str, limit_str,
    )
    .await?;

    let metrics = rows
        .into_iter()
        .map(|(date, country, operator, count)| MetricsResponse {
            date,
            country,
            operator,
            count,
        })
        .collect();

    Ok(metrics)
}
