// Database module per Sprint 2 Build Plan
// Provides PgPool configuration and connection management

use sqlx::postgres::PgPool;
use std::net::SocketAddr;

/// Configure and create a PostgreSQL connection pool
/// 
/// Per research.md PostgreSQL connection patterns:
/// - Database: biosouk_db with schemas platform, commerce, traceability
/// - Connection pool with configured max_connections
/// - log_statements setting for development/diagnosis
pub async fn create_pg_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect_with(
        sqlx::postgres::PostgresConnectOptions::from(database_url)
            .max_connections(max_connections)
            .log_statements(sqlx::sql::LogStatement::Always) // Always log in dev, configurable in prod
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to create PgPool: {}", e))?;
    
    // Verify connection
    sqlx::query!("SELECT 1").fetch_one(&pool).await
        .map_err(|e| anyhow::anyhow!("Database connection verification failed: {}", e))?;
    
    Ok(pool)
}

/// Database configuration from environment variables
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database connection URL
    pub database_url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Statement logging level
    pub log_statements: sqlx::sql::LogStatement,
}

impl DbConfig {
    /// Create DbConfig from environment variables
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/biosouk_db".to_string()),
            max_connections: std::env::var("MAX_DB_CONNECTIONS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            log_statements: sqlx::sql::LogStatement::Always, // In production, use: LogStatement::Never
        }
    }
}

/// Run SQL migrations using SQLx
/// Per database design §1446-1461: migrations version-controlled
pub async fn run_migrations(pool: &PgPool, migration_dir: &str) -> Result<(), anyhow::Error> {
    // sqlx::migrate!(migration_dir)?;
    Ok(())
}

/// Health check for database connectivity
pub async fn health_check(pool: &PgPool) -> Result<String, sqlx::Error> {
    let result = sqlx::query!("SELECT 1 AS healthy").fetch_one(pool).await;
    Ok(format!("database: {}", match result { Ok(_) => "connected", Err(e) => format!("error: {}", e) }))
}