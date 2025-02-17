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

    pub fn process(&mut self, events: Vec<Value>) {
        let (mut extracted_data, mut discarded_events) = self.validate_events(events);

        println!(
            "Processed {} Discarded {}",
            extracted_data["mac_address"].len(),
            discarded_events.len()
        );

        let record_batch = self.create_record_batch(&extracted_data);

        match record_batch {
            Ok(batch) => {
                println!("RecordBatch with {} rows", batch.num_rows());
                let table = MemTable::try_new(self.schema.clone(), vec![vec![batch]]);
                let ctx = SessionContext::new();
                //              ctx.register_table("my_table", Arc::new(table));
                //               let df = ctx.sql("SELECT * FROM my_table");
                //               df.clone().show();
            }
            Err(e) => eprintln!("Failed to create RecordBatch: {}", e),
        }
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
