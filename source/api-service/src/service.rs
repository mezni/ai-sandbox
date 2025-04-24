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


pub async fn get_metrics(
    db: &DBManager,
    direction: &str,
    dimension: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<&str>,    
) -> Result<Vec<MetricsResponse>, AppError> {
    let db_client = db.get_client().await?;

    let max_date_str = repo::get_max_date(&db_client, &dir).await?;
    let max_date = NaiveDate::parse_from_str(&max_date_str, "%Y-%m-%d")
        .map_err(|e| AppError::Unexpected(format!("Invalid max date: {}", e)))?;
    
    Ok(metrics)    
}