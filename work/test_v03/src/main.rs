use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::fs;

mod config;
mod db;
mod handlers;
mod models;

use crate::db::initialize_database;
use crate::handlers::{fetch_mac_vendors, status};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Create a connection pool
    let manager = SqliteConnectionManager::file("my_database.db");
    let pool = Pool::new(manager).expect("Failed to create database pool");

    if let Err(e) = initialize_database(&pool) {
        eprintln!("Database initialization failed: {}", e);
        std::process::exit(1);
    }

    let config = crate::config::Config::from_env().expect("Failed to load config from environment");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/vendors", web::get().to(fetch_mac_vendors))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
