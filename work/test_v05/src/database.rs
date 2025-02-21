// src/database.rs
use crate::error::AppError;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Error as RusqliteError};
use std::fs;

type DbPool = Pool<SqliteConnectionManager>;

pub fn get_pool(db_name: &str) -> Result<DbPool, AppError> {
    let manager = SqliteConnectionManager::file(db_name);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .map_err(|e| AppError::R2D2Error(e))
}

pub fn initialize_database(pool: &DbPool) -> Result<(), AppError> {
    let conn: PooledConnection<SqliteConnectionManager> =
        pool.get().map_err(|e| AppError::R2D2Error(e))?;

    let sql_script = fs::read_to_string("database.sql").map_err(|e| AppError::IoError(e))?;

    conn.execute_batch(&sql_script)
        .map_err(|e| AppError::RusqliteError(e))?;

    println!("Database initialized successfully.");
    Ok(())
}
