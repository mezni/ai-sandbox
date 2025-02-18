use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod models;
mod config;

use crate::models::Status;
use dotenv::dotenv;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let config=crate::config::Config::from_env().unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}