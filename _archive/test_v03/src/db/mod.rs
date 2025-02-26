use crate::models::MacVendor;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error as RusqliteError;
use rusqlite::Result;
use std::fs;
use std::io;
type DbPool = Pool<SqliteConnectionManager>;

pub fn initialize_database(pool: &DbPool) -> Result<()> {
    let conn = pool.get().expect("Failed to get DB connection");

    let sql_script = fs::read_to_string("database.sql").expect("Failed to read database.sql");

    conn.execute_batch(&sql_script)?;

    println!("Database initialized successfully.");
    Ok(())
}

pub fn get_pool() -> Pool<SqliteConnectionManager> {
    // Create a connection pool
    let manager = SqliteConnectionManager::file("my_database.db");
    Pool::new(manager).expect("Failed to create database pool")
}

pub fn get_mac_vendors(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> std::result::Result<Vec<MacVendor>, io::Error> {
    let mut stmt = conn
        .prepare("SELECT id, designation, org_name FROM mac_vendors")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let vendors = stmt
        .query_map([], |row| {
            Ok(MacVendor {
                id: row.get(0)?,
                designation: row.get(1)?,
                org_name: row.get(2)?,
            })
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(vendors)
}

pub fn get_mac_vendor(
    conn: &PooledConnection<SqliteConnectionManager>,
    id: i32,
) -> std::result::Result<Vec<MacVendor>, io::Error> {
    let mut stmt = conn
        .prepare("SELECT id, designation, org_name FROM mac_vendors WHERE id = ?1")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let vendors = stmt
        .query_map([&id], |row| {
            Ok(MacVendor {
                id: row.get(0)?,
                designation: row.get(1)?,
                org_name: row.get(2)?,
            })
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(vendors)
}
