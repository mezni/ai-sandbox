use serde_json::Value;
use sled::Db;
use std::{convert::TryInto, sync::Arc};
use tokio::sync::Mutex;

pub struct Queue {
    db: Arc<Db>,
    next_key: Mutex<u64>,
}

impl Queue {
    pub fn new(path: &str) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let next_key = db
            .last()
            .transpose()
            .map(|res| res.map(|(k, _)| u64::from_be_bytes(k.as_ref().try_into().unwrap()) + 1))
            .unwrap_or(Ok(0))?;

        Ok(Self {
            db: Arc::new(db),
            next_key: Mutex::new(next_key),
        })
    }

    pub async fn push(&self, json_value: &Value) -> sled::Result<()> {
        let json_data = serde_json::to_vec(json_value)
            .map_err(|e| sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let mut next_key = self.next_key.lock().await;
        let key = next_key.to_be_bytes();

        self.db.insert(key, json_data)?;
        *next_key += 1;

        Ok(())
    }

    pub async fn pop(&self) -> sled::Result<Option<Value>> {
        if let Some((key, value)) = self.db.first()? {
            self.db.remove(&key)?;

            let json_value: Value = serde_json::from_slice(&value)
                .map_err(|e| sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            Ok(Some(json_value))
        } else {
            Ok(None)
        }
    }
}
