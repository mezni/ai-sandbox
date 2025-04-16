use crate::service;
use actix_web::{HttpResponse, Responder, get, web};
use core::db::DBManager;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StatsParams {
    fact: String,
    dimensions: Option<String>,
}

#[get("/api/v1/health")]
async fn health_check() -> impl Responder {
    let resp = service::health_service().await;
    HttpResponse::Ok().json(resp)
}

#[get("/api/v1/stats/overview")]
async fn overview_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::overview_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(_) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch overview".to_string(),
        }),
    }
}

#[get("/api/v1/stats")]
async fn stats_endpoint(
    db: web::Data<Arc<DBManager>>,
    query: web::Query<StatsParams>,
) -> impl Responder {
    let fact_table = query.fact.clone();
    let dimensions: Vec<String> = query
        .dimensions
        .as_ref()
        .map(|d| {
            d.split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    match service::stats_service(db.as_ref(), &fact_table, &dimensions).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(_) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch stats".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(overview_endpoint)
        .service(stats_endpoint);
}
