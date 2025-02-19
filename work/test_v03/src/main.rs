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

use crate::handlers::fetch_mac_vendors;
use crate::handlers::*;

type DbPool = Pool<SqliteConnectionManager>;

fn initialize_database(pool: &DbPool) -> Result<()> {
    let conn = pool.get().expect("Failed to get DB connection");

    let sql_script = fs::read_to_string("database.sql").expect("Failed to read database.sql");

    conn.execute_batch(&sql_script)?;

    println!("Database initialized successfully.");
    Ok(())
}

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
