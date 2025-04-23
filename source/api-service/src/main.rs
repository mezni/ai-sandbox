mod handlers;
mod service;

use core::errors::AppError;
use core::logger::Logger;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

// Constants for configuration
const FRONTEND_URL_LOCAL: &str = "http://localhost:8080";
const FRONTEND_URL_DOCKER: &str = "http://frontend:8080";
const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    Logger::info("Starting process");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(FRONTEND_URL_LOCAL)
                    .allowed_origin(FRONTEND_URL_DOCKER)
                    .allow_any_method()
                    .allow_any_header(),
            )
            //    .app_data(db_data.clone())
            .configure(handlers::config)
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await?;

    Logger::info("Stopping process");

    Ok(())
}
