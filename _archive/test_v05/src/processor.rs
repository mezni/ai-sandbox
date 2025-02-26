use crate::database::*;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};

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

use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::sync::Arc;
const BATCH_SIZE: usize = 100;

pub struct EventProcessor {
    fields: Vec<String>,
    schema: Arc<Schema>,
    batch: VecDeque<Value>,
}

impl EventProcessor {
    pub fn new(fields: Vec<String>) -> Self {
        let schema = Arc::new(Schema::new(
            fields
                .iter()
                .map(|field| Field::new(field.as_str(), DataType::Utf8, false))
                .collect::<Vec<_>>(),
        ));
        EventProcessor {
            fields: fields,
            schema: schema,
            batch: VecDeque::new(),
        }
    }

    pub async fn process(&mut self, event: Value) -> Result<(), Box<dyn Error>> {
        // Add the event to the batch
        self.batch.push_back(event.clone());

        // Process batch when it reaches BATCH_SIZE
        if self.batch.len() >= BATCH_SIZE {
            let batch_to_process: Vec<Value> = self.batch.drain(..).collect(); // Drain the batch into a Vec
            self.process_batch(batch_to_process).await?; // Process the drained batch
        }

        Ok(())
    }

    pub async fn process_batch(&self, events: Vec<Value>) -> Result<(), Box<dyn Error>> {
        println!("Processing batch of size: {}", events.len());

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

        self.execute_query(ctx).await?;

        Ok(())
    }

    async fn execute_query(&self,  ctx: SessionContext) -> Result<(), Box<dyn Error>> {
        let df = ctx.sql("SELECT * FROM mac_table").await?;

        let df = ctx
            .sql("SELECT mac_address,max(event_time) as event_time FROM mac_table GROUP BY mac_address")
            .await?;
        // df.clone().show().await?;
        let batches = df.collect().await?;
        for batch in batches {
            let mac_address_col = batch
                .column(0)
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| {
                    DataFusionError::Internal("Failed to cast mac_address column".to_string())
                })?;
            let event_time_col = batch
                .column(1)
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| {
                    DataFusionError::Internal("Failed to cast event_time column".to_string())
                })?;

            let mut conn = get_pool("macs.db")?.get()?;
            let tx = conn.transaction()?;
            let vendor_stmt_text =
                "INSERT OR REPLACE INTO mac_vendors (id, designation) VALUES (?, ?)";
            let mut vendor_stmt = tx.prepare(vendor_stmt_text)?;

            let mac_stmt_text =
                "INSERT OR REPLACE INTO mac_addresses  (id, mac_address , mac_vendor_id ) VALUES (?, ?, ?)";
            let mut mac_stmt = tx.prepare(mac_stmt_text)?;

            for i in 0..batch.num_rows() {
                let mac_address = mac_address_col.value(i);
                let event_time = event_time_col.value(i);
                let mac_address_id = get_mac_id(mac_address);
                if let Some((vendor_id, vendor_design)) = get_vendor_info(mac_address) {
                    vendor_stmt.execute(params![vendor_id, vendor_design])?;
                    mac_stmt.execute(params![mac_address_id, mac_address, vendor_id])?;
                    // println!("MAC {} Time {}", mac_address, event_time);
                } else {
                    println!("No vendor info found for MAC {}", mac_address);
                }
            }
            drop(vendor_stmt);
            drop(mac_stmt);
            tx.commit()?;
        }
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
    let mac_clean = mac.replace(":", "");
    u64::from_str_radix(&mac_clean, 16).ok()
}

fn get_vendor_info(mac: &str) -> Option<(u64, String)> {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() >= 3 {
        let oui = format!("{}{}{}", parts[0], parts[1], parts[2]);
        u64::from_str_radix(&oui, 16)
            .ok()
            .map(|vendor_id| (vendor_id, oui))
    } else {
        None
    }
}
