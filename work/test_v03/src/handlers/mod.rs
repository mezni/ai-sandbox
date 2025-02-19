use crate::db::get_mac_vendors;
use crate::models::Status;
use actix_web::{web, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

// Create a database connection pool
pub fn create_pool() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("my_database.db");
    Pool::new(manager).expect("Failed to create database pool")
}

/// Health check endpoint.
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

/// Fetch MAC vendors from the database.
pub async fn fetch_mac_vendors(pool: web::Data<Pool<SqliteConnectionManager>>) -> impl Responder {
    match pool.get() {
        Ok(conn) => match get_mac_vendors(&conn) {
            Ok(vendors) => {
                println!("Fetched {} vendors", vendors.len());
                for vendor in &vendors {
                    println!("{:?}", vendor);
                }
                HttpResponse::Ok().json(vendors)
            }
            Err(e) => {
                eprintln!("Failed to fetch MAC vendors: {}", e);
                HttpResponse::InternalServerError().body("Failed to fetch MAC vendors")
            }
        },
        Err(e) => {
            eprintln!("Failed to get a database connection: {}", e);
            HttpResponse::InternalServerError().body("Failed to get a database connection")
        }
    }
}
