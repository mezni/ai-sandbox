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

#[get("/api/v1/stats/roamout_operators")]
async fn count_roam_out_operators_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::count_roam_out_operators_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

#[get("/api/v1/stats/roamout_countries")]
async fn count_roam_out_countries_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::count_roam_out_countries_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

#[get("/api/v1/stats/roamout_dates")]
async fn count_roam_out_dates_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::count_roam_out_dates_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

#[get("/api/v1/stats/notifications")]
async fn get_notifications_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::get_notifications_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

#[get("/api/v1/stats/anomalies")]
async fn get_anomalie_sor_endpoint(db: web::Data<Arc<DBManager>>) -> impl Responder {
    match service::get_anomalie_sor_service(db.as_ref()).await {
        Ok(data) => HttpResponse::Ok().json(json!({ "data": data })),
        Err(e) => HttpResponse::InternalServerError().json(service::ErrorResponse {
            error: "Failed to fetch roamout by country".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(overview_endpoint)
        .service(count_roam_out_operators_endpoint)
        .service(count_roam_out_countries_endpoint)
        .service(count_roam_out_dates_endpoint)
        .service(get_notifications_endpoint)
        .service(get_anomalie_sor_endpoint);
}
