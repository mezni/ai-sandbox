use chrono::{DateTime, Utc};
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
use std::process;
use std::sync::Arc;
use tokio::sync::mpsc;
const MAX_EVENTS: usize = 200000;

pub struct Receiver {
    rx: mpsc::Receiver<Value>,
}

impl Receiver {
    pub fn new(rx: mpsc::Receiver<Value>) -> Self {
        Receiver { rx }
    }

    pub async fn run(&mut self) {
        let mut events = Vec::new();
        let schema = Arc::new(Schema::new(vec![
            Field::new("mac_address", DataType::Utf8, false),
            Field::new("event_time", DataType::Utf8, false),
        ]));

        loop {
            match self.rx.recv().await {
                Some(event) => {
                    //                    println!("Received event: {:?}", event);
                    events.push(event);
                    if events.len() >= MAX_EVENTS {
                        println!("Received {} events. Processing...", MAX_EVENTS);
                        self.process_events(events.clone(), schema.clone()).await;
                        events.clear();
                        process::exit(0);
                    }
                }
                None => {
                    println!("Channel closed.");
                    if !events.is_empty() {
                        self.process_events(events, schema.clone()).await;
                    }
                    break;
                }
            }
        }
    }

    async fn process_events(&self, events: Vec<Value>, schema: Arc<Schema>) -> Result<()> {
        // Process the events here
        println!("Processing {} events...", events.len());

        let mut mac_addresses = Vec::new();
        let mut event_times = Vec::new();

        for event in events {
            if let Some(mac_address) = event["mac_address"].as_str() {
                mac_addresses.push(mac_address.to_string());
            } else {
                println!("Error: mac_address field not found in event");
            }

            if let Some(event_time) = event["event_time"].as_str() {
                event_times.push(event_time.to_string());
            } else {
                println!("Error: event_time field not found in event");
            }
        }

        let mac_address_array =
            Arc::new(StringArray::from(mac_addresses)) as Arc<dyn datafusion::arrow::array::Array>;
        let event_time_array =
            Arc::new(StringArray::from(event_times)) as Arc<dyn datafusion::arrow::array::Array>;

        match RecordBatch::try_new(schema.clone(), vec![mac_address_array, event_time_array]) {
            Ok(batch) => {
                //                println!("{:?}", batch);
                let table = MemTable::try_new(schema, vec![vec![batch]])?;
                let ctx = SessionContext::new();
                ctx.register_table("my_table", Arc::new(table))?;

                let df = ctx.sql("SELECT * FROM my_table").await?;
                df.clone().show().await?;

                let now: DateTime<Utc> = Utc::now();
                let output_file_name = now.format("%Y%m%d_%H%M%S_%6f.parquet").to_string();
                let target_path = "OUTPUT/".to_owned() + &output_file_name;

                df.write_parquet(
                    &target_path,
                    DataFrameWriteOptions::new(),
                    None, // writer_options
                )
                .await;

                let df = ctx
                    .sql("SELECT mac_address,max(event_time) FROM my_table GROUP BY mac_address")
                    .await?;
                df.clone().show().await?;

                Ok(())
            }
            Err(e) => {
                println!("Error creating RecordBatch: {}", e);
                Err(e.into())
            }
        }
    }
}
