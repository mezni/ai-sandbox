use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use datafusion::arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};

pub struct EventProcessor;

impl EventProcessor {
    pub fn process(events: Vec<Value>) {
        let fields = vec![
            "mac_address", "event_time", "ip_address_src", "port_src",
            "ip_address_dst", "port_dst", "event_type",
        ];

        let mut extracted_data: HashMap<&str, Vec<String>> = fields
            .iter()
            .map(|&field| (field, Vec::new()))
            .collect();

        let mut discarded_events = Vec::new();

        for event in events {
            let mut valid = true;

            for &field in &fields {
                if let Some(value) = event[field].as_str() {
                    extracted_data.get_mut(field).unwrap().push(value.to_string());
                } else {
                    valid = false;
                    break;
                }
            }

            if !valid {
                discarded_events.push(event);
            }
        }

        println!("Processed {} valid events", extracted_data["mac_address"].len());
        println!("Discarded {} invalid events", discarded_events.len());

        let schema = Self::schema(&fields);
        let record_batch = Self::create_record_batch(schema, &fields, &extracted_data);

        match record_batch {
            Ok(batch) => println!("Created RecordBatch with {} rows", batch.num_rows()),
            Err(e) => eprintln!("Failed to create RecordBatch: {}", e),
        }
    }

    fn create_record_batch(
        schema: Arc<Schema>,
        fields: &[&str],
        extracted_data: &HashMap<&str, Vec<String>>,
    ) -> Result<RecordBatch, datafusion::arrow::error::ArrowError> {
        let arrays: Vec<_> = fields
            .iter()
            .map(|&field| Arc::new(StringArray::from(extracted_data[field].clone())) as _)
            .collect();

        RecordBatch::try_new(schema, arrays)
    }

    fn schema(fields: &[&str]) -> Arc<Schema> {
        Arc::new(Schema::new(
            fields.iter().map(|&field| Field::new(field, DataType::Utf8, false)).collect(),
        ))
    }
}
