//! Database pool creation and migration runner.

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Create a PostgreSQL connection pool and verify connectivity.
pub async fn create_pg_pool(database_url: &str, max_connections: u32) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections.max(1))
        .connect(database_url)
        .await?;

    sqlx::query("SELECT 1").execute(&pool).await?;
    Ok(pool)
}

/// Apply all versioned migrations from `migrations/`.
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Lightweight connectivity probe used by health handlers.
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
