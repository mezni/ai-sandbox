mod config;
mod handlers;
use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use color_eyre::Result;
use tracing::{info, instrument};

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");
    info!("Starting server at http://{}:{}/", config.host, config.port);
    HttpServer::new(move || App::new().wrap(Logger::default()))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok(())
}
