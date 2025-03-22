use crate::errors::AppError;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use tokio_postgres::{Client, NoTls};

pub async fn connection() -> Result<Client, AppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn the connection in a background task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Database connection error: {}", e);
        }
    });

    Ok(client)
}
