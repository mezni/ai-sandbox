use axum::http::StatusCode;
use axum::extract::State;
use axum::response::{IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    status: String,
    database: String,
}

pub async fn health(State(state): State<crate::AppState>) -> impl IntoResponse {
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await;

    let database = match db_ok {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    let _body = Json(HealthCheck {
        status: "healthy".to_string(),
        database: database.to_string(),
    });

    StatusCode::OK.into_response()
}

pub async fn status(State(_data): State<crate::AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "database": "configured"
    }))
}