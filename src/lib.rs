use axum::{routing::get, Router};
use sqlx::PgPool;

mod hello;

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

pub async fn create_app(db_pool: PgPool) -> Router {
    let state = AppState { db_pool };
    Router::new()
        .route("/health", get(hello::health))
        .route("/status", get(hello::status))
        .with_state(state)
}
