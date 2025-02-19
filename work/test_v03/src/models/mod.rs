use rusqlite::Result;
use rusqlite::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacVendor {
    pub id: i32,
    pub designation: String,
    pub org_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacAddress {
    pub id: i32,
    pub mac_address: String,
    pub mac_vendor_id: i32,
}

pub trait FromRow {
    fn from_row(row: &Row) -> Result<Self>
    where
        Self: Sized;
}

// Implement FromRow for MacVendor
impl FromRow for MacVendor {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(MacVendor {
            id: row.get("id")?,
            designation: row.get("designation")?,
            org_name: row.get("org_name")?,
        })
    }
}

// Implement FromRow for MacAddress
impl FromRow for MacAddress {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(MacAddress {
            id: row.get("id")?,
            mac_address: row.get("mac_address")?,
            mac_vendor_id: row.get("mac_vendor_id")?,
        })
    }
}
