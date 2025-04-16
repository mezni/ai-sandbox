use crate::service;
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use serde_json::json;
use std::sync::Arc;

#[get("/api/v1/health")]
async fn health_check() -> impl Responder {
    let resp = service::health_service().await;
    let body = service::HealthResponse {
        status: resp.status.to_string(),
    };
    HttpResponse::Ok().json(body)
}

#[get("/api/v1/stats/overview")]
async fn overview_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::overview_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to fetch overview".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
