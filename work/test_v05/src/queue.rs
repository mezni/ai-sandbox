use serde::{Deserialize, Serialize};
use serde_json;
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

    pub fn push_json<T: Serialize>(&mut self, value: &T) -> sled::Result<()> {
        let json_data = serde_json::to_vec(value).unwrap();
        let key = self.next_key.to_be_bytes();
        self.db.insert(key, json_data)?;
        self.next_key += 1;
        Ok(())
    }

    pub fn pop_json<T: for<'de> Deserialize<'de>>(&self) -> sled::Result<Option<T>> {
        if let Some((key, value)) = self.db.first()? {
            self.db.remove(&key)?;
            let json_str = str::from_utf8(&value).unwrap();
            let obj: T = serde_json::from_str(json_str).unwrap();
            Ok(Some(obj))
        } else {
            Ok(None)
        }
    }
}
