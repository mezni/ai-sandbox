use crate::models::MacVendor;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error as RusqliteError;
use std::io;

pub fn get_mac_vendors(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> std::result::Result<Vec<MacVendor>, io::Error> {
    let mut stmt = conn
        .prepare("SELECT id, name FROM mac_vendors")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let vendors = stmt
        .query_map([], |row| {
            Ok(MacVendor {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .collect::<Result<Vec<_>, RusqliteError>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(vendors)
}
