//! Route composition for the `/api/v1` surface.

use axum::routing::{get, post};
use axum::Router;

use crate::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .route("/auth/refresh", post(handlers::refresh))
        .route("/auth/logout", post(handlers::logout))
        .route("/auth/me", get(handlers::me))
        .route(
            "/users/:user_id",
            get(handlers::get_user).patch(handlers::update_user),
        )
}
