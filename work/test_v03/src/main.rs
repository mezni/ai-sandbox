use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod models;

use crate::models::Status;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
