use crate::service;
use actix_web::{Error, HttpResponse, Responder, get, web};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct MetricsParams {
    direction: String,
    dimension: String,
    aggregation: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[get("/api/v1/health")]
async fn health_check() -> impl Responder {
    let resp = service::health_service().await;
    let body = service::HealthResponse {
        status: resp.status.to_string(),
    };
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/metrics")]
async fn get_metrics(params: web::Query<MetricsParams>) -> Result<HttpResponse, Error> {
    // Validate required parameters
    if !["IN", "OUT"].contains(&params.direction.as_str()) {
        return Ok(HttpResponse::BadRequest().json(service::ErrorResponse {
            error: "Invalid direction. Must be IN or OUT.".into(),
        }));
    }

    if !["GLOBAL", "COUNTRY", "OPERATOR"].contains(&params.dimension.as_str()) {
        return Ok(HttpResponse::BadRequest().json(service::ErrorResponse {
            error: "Invalid dimension. Must be GLOBAL, COUNTRY, or OPERATOR.".into(),
        }));
    }

    let result = service::get_metrics(
        &params.direction,
        &params.dimension,
        params.aggregation.as_deref(),
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    )
    .await;

    let body = json!({ "data": result });

    Ok(HttpResponse::Ok().json(body))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
    cfg.service(get_metrics);
}
