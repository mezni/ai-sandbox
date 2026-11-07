use std::sync::Arc;

use axum::middleware;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;

pub mod config;
pub mod db;
mod hello;
pub mod platform;

use config::Config;
use platform::application::policy::AuthPolicy;
use platform::application::AuthService;
use platform::infrastructure::argon2_hasher::Argon2idHasher;
use platform::infrastructure::jwt_issuer::JwtIssuer;
use platform::infrastructure::persistence::audit_repository::SqlxAuditRepository;
use platform::infrastructure::persistence::refresh_token_repository::SqlxRefreshTokenRepository;
use platform::infrastructure::persistence::role_repository::SqlxRoleRepository;
use platform::infrastructure::persistence::user_repository::SqlxUserRepository;
use platform::presentation::middleware::request_id::request_id_middleware;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth: Arc<AuthService>,
}

/// Compose the application router from a database pool and configuration.
pub fn create_app(db_pool: PgPool, config: Config) -> Router {
    let auth = Arc::new(build_auth_service(db_pool.clone(), &config));
    let state = AppState { db_pool, auth };

    let api = platform::presentation::routes::router();

    Router::new()
        .route("/health", get(hello::health))
        .route("/status", get(hello::status))
        .nest("/api/v1", api)
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(state)
}

fn build_auth_service(pool: PgPool, config: &Config) -> AuthService {
    let policy = AuthPolicy {
        access_ttl: config.jwt_access_ttl,
        refresh_ttl: config.jwt_refresh_ttl,
        remember_me_ttl: config.jwt_refresh_ttl * 2,
        password_min_length: config.password_min_length,
    };

    AuthService::new(
        Arc::new(SqlxUserRepository::new(pool.clone())),
        Arc::new(SqlxRoleRepository::new(pool.clone())),
        Arc::new(SqlxRefreshTokenRepository::new(pool.clone())),
        Arc::new(SqlxAuditRepository::new(pool)),
        Arc::new(Argon2idHasher::new(
            config.argon2_memory,
            config.argon2_time,
            config.argon2_threads,
        )),
        Arc::new(JwtIssuer::new(
            &config.jwt_secret,
            config.jwt_access_ttl,
            "biosouk",
            "biosouk-users",
        )),
        policy,
    )
}
