use crate::repo;
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


pub async fn health_service() -> HealthResponse {
    HealthResponse {
        status: "Health check passed".to_string(),
    }
}

