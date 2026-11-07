use std::net::SocketAddr;

use anyhow::Context;
use biosouk::config::Config;
use biosouk::{create_app, db};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env().context("invalid configuration")?;

    let pool = db::create_pg_pool(&config.database_url, config.max_db_connections)
        .await
        .context("failed to connect to PostgreSQL")?;

    db::run_migrations(&pool)
        .await
        .context("failed to apply migrations")?;

    let app = create_app(pool, config.clone());

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .with_context(|| format!("invalid bind address {}:{}", config.host, config.port))?;

    tracing::info!(%addr, environment = %config.environment, "starting BioSouk API");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
