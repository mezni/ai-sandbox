use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    status: String,
    database: String,
}

pub async fn health(State(state): State<crate::AppState>) -> impl IntoResponse {
    let (code, database) = match crate::db::health_check(&state.db_pool).await {
        Ok(()) => (StatusCode::OK, "connected"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "disconnected"),
    };

    let body = Json(HealthCheck {
        status: "healthy".to_string(),
        database: database.to_string(),
    });

    (code, body).into_response()
}

pub async fn status(State(_state): State<crate::AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "biosouk"
    }))
}
