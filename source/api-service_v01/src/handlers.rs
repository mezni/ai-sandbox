use crate::service::{self, ErrorResponse};
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpResponse, get, web};
use core::db::DBManager;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct MetricsQuery {
    pub direction: String,
    pub dimensions: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<String>,
}

#[get("/api/v1/health")]
async fn health_check() -> HttpResponse {
    let resp = service::health_service().await;
    HttpResponse::Ok().json(resp)
}

#[get("/api/v1/metrics")]
async fn get_metrics(
    db: web::Data<Arc<DBManager>>,
    params: web::Query<MetricsQuery>,
) -> Result<HttpResponse, Error> {
    let dir = params.direction.to_ascii_uppercase();
    let dim = params.dimensions.to_ascii_uppercase();

    // Validate direction
    if !["TOTIN", "ACTIN", "OUT"].contains(&dir.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid direction. Must be TOTIN, ACTIN or OUT.".into(),
        }));
    }

    // Validate dimensions
    if !["GLOBAL", "COUNTRY", "OPERATOR"].contains(&dim.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid dimension. Must be GLOBAL, COUNTRY, or OPERATOR.".into(),
        }));
    }

    // Parse or default limit
    let limit = if dim == "GLOBAL" {
        params
            .limit
            .as_deref()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
            .to_string()
    } else {
        params.limit.clone().unwrap_or_else(|| "5".to_string())
    };

    // Call service
    let data = service::get_metrics(
        db.as_ref(),
        &dir,
        &dim,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
        Some(&limit),
    )
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
    cfg.service(get_metrics);
}
