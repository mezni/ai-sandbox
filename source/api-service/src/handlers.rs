use crate::service::{self, ErrorResponse};
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, HttpResponse, get, web};
use core::db::DBManager;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct MetricsParams {
    direction: String,
    dimension: String,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<String>,
}

#[get("/api/v1/health")]
async fn health_check() -> HttpResponse {
    let resp = service::health_service().await;
    HttpResponse::Ok().json(resp)
}

#[get("/api/v1/metrics")]
async fn get_metrics(
    db: web::Data<Arc<DBManager>>,
    params: web::Query<MetricsParams>,
) -> Result<HttpResponse, Error> {
    // Normalize
    let dir = params.direction.to_ascii_uppercase();
    let dim = params.dimension.to_ascii_uppercase();

    // Validate
    if !["TOTIN", "ACTIN", "OUT"].contains(&dir.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid direction. Must be TOTIN, ACTIN or OUT.".into(),
        }));
    }
    if !["GLOBAL", "COUNTRY", "OPERATOR"].contains(&dim.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid dimension. Must be GLOBAL, COUNTRY, or OPERATOR.".into(),
        }));
    }

    let data = service::get_metrics(
        db.as_ref(),
        &dir,
        &dim,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
        params.limit.as_deref(),
    )
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
    //    cfg.service(get_metrics);
}
