mod common;

use axum::http::{Method, StatusCode};

#[tokio::test]
async fn migrations_create_auth_tables_and_seed_roles() {
    let app = common::setup().await;
    let pool = common::pool().await;

    let refresh_table: (bool,) =
        sqlx::query_as("SELECT to_regclass('platform.refresh_tokens') IS NOT NULL")
            .fetch_one(&pool)
            .await
            .expect("refresh_tokens table exists");
    assert!(refresh_table.0);

    let audit_table: (bool,) =
        sqlx::query_as("SELECT to_regclass('platform.audit_logs') IS NOT NULL")
            .fetch_one(&pool)
            .await
            .expect("audit_logs table exists");
    assert!(audit_table.0);

    let roles: (i64,) = sqlx::query_as(
        "SELECT count(*) FROM platform.roles \
         WHERE code IN ('admin', 'producer', 'customer', 'support')",
    )
    .fetch_one(&pool)
    .await
    .expect("roles query");
    assert_eq!(roles.0, 4);

    let permissions: (i64,) = sqlx::query_as(
        "SELECT count(*) FROM platform.permissions \
         WHERE code IN ('users.read', 'users.update', 'roles.assign', 'audit.read')",
    )
    .fetch_one(&pool)
    .await
    .expect("permissions query");
    assert_eq!(permissions.0, 4);

    let (status, body) = common::call(&app, Method::GET, "/health", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["database"], "connected");
}
