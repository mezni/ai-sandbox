use datafusion::arrow::{
    array::{Float64Array, StringArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::datasource::memory::MemTable;
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::context::SessionContext;
use datafusion::prelude::*;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

// TO DO : add contexte to event processor
pub struct EventProcessor {
    fields: Vec<String>,
    schema: Arc<Schema>,
}

impl EventProcessor {
    pub fn new(fields: Vec<String>) -> Self {
        let schema = Arc::new(Schema::new(
            fields
                .iter()
                .map(|field| Field::new(field.as_str(), DataType::Utf8, false))
                .collect::<Vec<_>>(),
        ));
        EventProcessor { fields, schema }
    }

    pub async fn process(&mut self, events: Vec<Value>) -> Result<()> {
        let (extracted_data, discarded_events) = self.validate_events(events);

        println!(
            "Processed {} Discarded {}",
            extracted_data["mac_address"].len(),
            discarded_events.len()
        );

        let record_batch = self.create_record_batch(&extracted_data)?;

        let table = MemTable::try_new(self.schema.clone(), vec![vec![record_batch]])?;
        let ctx = SessionContext::new();
        ctx.register_table("mac_table", Arc::new(table))?;

        let df = ctx.sql("SELECT * FROM mac_table").await?;
        //df.clone().show().await?;

        let df = ctx
                    .sql("SELECT mac_address,max(event_time) as event_time FROM mac_table GROUP BY mac_address")
                    .await?;
        df.clone().show().await?;

    let batches = df.collect().await?;

    let mut conn = Connection::open("macs.db").map_err(|e| {
        DataFusionError::Internal(format!("Error opening database connection: {}", e))
    })?;
    let tx = conn.transaction().map_err(|e| {
        DataFusionError::Internal(format!("Error starting transaction: {}", e))
    })?;


    for batch in batches {
        let mac_address_col = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| DataFusionError::Internal("Failed to cast mac_address column".to_string()))?;
        let event_time_col = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| DataFusionError::Internal("Failed to cast event_time column".to_string()))?;

        for i in 0..batch.num_rows() {
            let mac_address = mac_address_col.value(i);
            let event_time = event_time_col.value(i);

            let mac_address_id = get_mac_id(mac_address);
            let vendor_id = get_vendor_id(mac_address);

            match (mac_address_id, vendor_id) {
                (Some(mac_id), Some(vendor)) => {
                    tx.execute(
                        "INSERT OR REPLACE INTO macs (mac_address_id, mac_address, last_seen, mac_vendor_id) VALUES (?1, ?2, ?3, ?4)",
                        params![mac_id, mac_address, event_time, vendor],
                    )
                    .map_err(|e| {
                        DataFusionError::Internal(format!("Error executing SQL query: {}", e))
                    })?;
                }
                (Some(mac_id), None) => {
                    println!(
                        "Row {}: mac_address={}, event_time={}, mac_address_id={}, Vendor extraction failed",
                        i, mac_address, event_time, mac_id
                    );
                }
                (None, _) => {
                    println!("Row {}: Invalid MAC address: {}", i, mac_address);
                }
            }
        }
    }

    tx.commit().map_err(|e| {
        DataFusionError::Internal(format!("Error committing transaction: {}", e))
    })?;
        Ok(())
    }

    fn validate_events(&self, events: Vec<Value>) -> (HashMap<&str, Vec<String>>, Vec<Value>) {
        let mut extracted_data: HashMap<&str, Vec<String>> = self
            .fields
            .iter()
            .map(|field| (field.as_str(), Vec::new()))
            .collect();

        let mut discarded_events: Vec<Value> = Vec::new();

        for event in events {
            let mut temp_values: HashMap<&str, String> = HashMap::new();
            let mut valid = true;

            for field in &self.fields {
                if let Some(value) = event.get(field).and_then(|v| v.as_str()) {
                    temp_values.insert(field.as_str(), value.to_string());
                } else {
                    valid = false;
                    break; // If any field is missing, discard the event
                }
            }

            if valid {
                // Push all values to extracted_data, ensuring we only push complete events
                for field in &self.fields {
                    extracted_data
                        .get_mut(field.as_str())
                        .unwrap()
                        .push(temp_values[field.as_str()].clone());
                }
            } else {
                discarded_events.push(event);
            }
        }

        (extracted_data, discarded_events)
    }

    fn create_record_batch(
        &self,
        extracted_data: &HashMap<&str, Vec<String>>,
    ) -> Result<RecordBatch, datafusion::arrow::error::ArrowError> {
        let arrays: Vec<_> = self
            .fields
            .iter()
            .map(|field| Arc::new(StringArray::from(extracted_data[field.as_str()].clone())) as _)
            .collect();

        RecordBatch::try_new(self.schema.clone(), arrays)
    }
}

fn get_mac_id(mac: &str) -> Option<u64> {
    let mac_clean = mac.replace(":", ""); // Remove colons
    u64::from_str_radix(&mac_clean, 16).ok() // Convert hex string to integer
}

fn get_vendor_id(mac: &str) -> Option<u64> {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() >= 3 {
        let oui = format!("{}{}{}", parts[0], parts[1], parts[2]); // First 3 octets
        u64::from_str_radix(&oui, 16).ok() // Convert to integer
    } else {
        None
    }
}
