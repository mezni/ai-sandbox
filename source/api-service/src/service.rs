use core::db;
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

    pub count: i64,
}

pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Healthy".to_string(),
    }
}

pub async fn get_metrics(
    direction: &str,
    dimension: &str,
    aggregation: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> MetricsResponse {
    MetricsResponse {
        date: "2025-04-23".to_string(),
        country: if dimension == "COUNTRY" {
            Some("Tunisia".to_string())
        } else {
            None
        },
        operator: if dimension == "OPERATOR" {
            Some("Orange".to_string())
        } else {
            None
        },
        count: 12,
    }
}
