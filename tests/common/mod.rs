#![allow(dead_code)]

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use axum::Router;
use serde_json::Value;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use tower::ServiceExt;

use biosouk::config::Config;
use biosouk::create_app;

static MIGRATIONS: OnceCell<()> = OnceCell::const_new();

pub const VALID_PASSWORD: &str = "S3cure-passphrase!";
pub const TEST_JWT_SECRET: &str = "test-secret-test-secret-test-secret-123456";

pub fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/biosouk".to_string())
}

/// Return a connection pool, applying migrations exactly once per test binary.
pub async fn pool() -> PgPool {
    let url = database_url();
    MIGRATIONS
        .get_or_init(|| async {
            let pool = PgPool::connect(&url).await.expect("connect database");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("apply migrations");
            pool.close().await;
        })
        .await;

    PgPool::connect(&url).await.expect("connect database")
}

pub struct TestApp {
    pub router: Router,
    pub config: Config,
}

pub async fn setup() -> TestApp {
    let pool = pool().await;
    let config = Config::for_testing(database_url());
    let router = create_app(pool, config.clone());
    TestApp { router, config }
}

pub fn unique_email(prefix: &str) -> String {
    format!("{prefix}+{}@example.com", uuid::Uuid::new_v4())
}

pub async fn call(
    app: &TestApp,
    method: Method,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);

    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }

    let request = match body {
        Some(body) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .expect("build request"),
        None => builder.body(Body::empty()).expect("build request"),
    };

    let response = app
        .router
        .clone()
        .oneshot(request)
        .await
        .expect("router response");

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("response body");

    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };

    (status, value)
}

pub async fn register(app: &TestApp, email: &str) -> Value {
    let (status, body) = call(
        app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(serde_json::json!({ "email": email, "password": VALID_PASSWORD })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "registration failed: {body}");
    body
}

pub async fn login(app: &TestApp, email: &str, password: &str) -> (StatusCode, Value) {
    call(
        app,
        Method::POST,
        "/api/v1/auth/login",
        None,
        Some(serde_json::json!({ "email": email, "password": password })),
    )
    .await
}

pub async fn assign_role(email: &str, role: &str) {
    let pool = pool().await;
    sqlx::query(
        "INSERT INTO platform.user_roles (user_id, role_id) \
         SELECT u.id, r.id FROM platform.users u, platform.roles r \
         WHERE u.email = $1 AND r.code = $2 \
         ON CONFLICT (user_id, role_id) DO NOTHING",
    )
    .bind(email)
    .bind(role)
    .execute(&pool)
    .await
    .expect("assign role");
}

pub async fn set_status(email: &str, status: &str) {
    let pool = pool().await;
    sqlx::query("UPDATE platform.users SET status = $2, updated_at = NOW() WHERE email = $1")
        .bind(email)
        .bind(status)
        .execute(&pool)
        .await
        .expect("update status");
}
