use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json, Value};
use sled::Db;
use std::str;

pub struct Queue {
    pub db: Db,
    next_key: u64,
}

impl Queue {
    pub fn new(path: &str) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let next_key = db
            .last()
            .transpose()
            .map(|res| res.map(|(k, _)| u64::from_be_bytes(k.as_ref().try_into().unwrap()) + 1))
            .unwrap_or(Ok(0))?;

        Ok(Self { db, next_key })
    }

    pub fn push(&mut self, json_value: &Value) -> sled::Result<()> {
        // Handle potential serialization error explicitly
        let json_data = serde_json::to_vec(json_value)
            .map_err(|e| sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let key = self.next_key.to_be_bytes();
        self.db.insert(key, json_data)?; // Insert the serialized JSON data
        self.next_key += 1;
        Ok(())
    }

    pub fn pop(&self) -> sled::Result<Option<Value>> {
        if let Some((key, value)) = self.db.first()? {
            self.db.remove(&key)?; // Remove the key-value pair from the database

            // Deserialize the value into a serde_json::Value
            let json_value: Value = serde_json::from_slice(&value)
                .map_err(|e| sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            Ok(Some(json_value)) // Return the JSON Value wrapped in Some
        } else {
            Ok(None) // Return None if the queue is empty
        }
    }
}
