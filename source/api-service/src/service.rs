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

    pub count: i64,
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
    aggregation: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<MetricsResponse>, AppError> {
    let dir = direction.to_ascii_uppercase();
    let dim = dimension.to_ascii_uppercase();

    if aggregation.is_none() {
        let agg = "SUM";
    }

    if dir == "IN" && dim == "GLOBAL" {
        println!("IN GLOBAL");
    } else if dir == "IN" && dim == "COUNTRY" {
        println!("IN COUNTRY");
    }

    let entry = MetricsResponse {
        date: "2025-04-23".to_string(),
        country: if dim == "COUNTRY" {
            Some("Tunisia".to_string())
        } else {
            None
        },
        operator: if dim == "OPERATOR" {
            Some("Orange".to_string())
        } else {
            None
        },
        count: 12,
    };

    Ok(vec![entry])
}
