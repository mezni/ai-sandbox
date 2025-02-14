use chrono::{DateTime, Utc};
use datafusion::arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::datasource::memory::MemTable;
use datafusion::error::Result as DataFusionResult;
use datafusion::execution::context::SessionContext;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::error::Error;
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
                    events.push(event);
                    if events.len() >= MAX_EVENTS {
                        println!("Received {} events. Processing...", MAX_EVENTS);
                        if let Err(e) = self.process_events(events.clone(), schema.clone()).await {
                            eprintln!("Error processing events: {}", e);
                        }
                        events.clear();
                        process::exit(0);
                    }
                }
                None => {
                    println!("Channel closed.");
                    if !events.is_empty() {
                        if let Err(e) = self.process_events(events, schema.clone()).await {
                            eprintln!("Error processing events: {}", e);
                        }
                    }
                    break;
                }
            }
        }
    }

    async fn process_events(&self, events: Vec<Value>, schema: Arc<Schema>) -> Result<(), Box<dyn Error>> {
        println!("Processing {} events...", events.len());

        let mut mac_addresses = Vec::new();
        let mut event_times = Vec::new();

        for event in events {
            if let Some(mac_address) = event["mac_address"].as_str() {
                mac_addresses.push(mac_address.to_string());
            } else {
                eprintln!("Error: mac_address field not found in event");
            }

            if let Some(event_time) = event["event_time"].as_str() {
                event_times.push(event_time.to_string());
            } else {
                eprintln!("Error: event_time field not found in event");
            }
        }

        let mac_address_array = Arc::new(StringArray::from(mac_addresses));
        let event_time_array = Arc::new(StringArray::from(event_times));

        let batch = RecordBatch::try_new(schema.clone(), vec![mac_address_array, event_time_array])?;
        let table = MemTable::try_new(schema, vec![vec![batch]])?;

        let ctx = SessionContext::new();
        ctx.register_table("my_table", Arc::new(table))?;

        // Query the DataFrame
        let df = ctx
            .sql("SELECT mac_address, MAX(event_time) as latest_event_time FROM my_table GROUP BY mac_address")
            .await?;
        df.clone().show().await?;

        // Write the DataFrame to a Parquet file
        let now: DateTime<Utc> = Utc::now();
        let output_file_name = now.format("%Y%m%d_%H%M%S_%6f.parquet").to_string();
        let target_path = "OUTPUT/".to_owned() + &output_file_name;

        df.clone().write_parquet(&target_path, DataFrameWriteOptions::new(), None)
            .await?;

        // Convert DataFrame to a Vec of tuples for SQLite upsert
        let batches = df.collect().await?;
        let mut data = Vec::new();
        for batch in batches {
            let mac_address = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
            let event_time = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
            for i in 0..batch.num_rows() {
                data.push((mac_address.value(i).to_string(), event_time.value(i).to_string()));
            }
        }

        // Upsert data into SQLite
        self.upsert_to_sqlite(data).await?;

        Ok(())
    }

    async fn upsert_to_sqlite(&self, data: Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open("events.db")?;

        // Create the table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                mac_address TEXT PRIMARY KEY,
                latest_event_time TEXT NOT NULL
            )",
            [],
        )?;

        // Upsert data
        for (mac_address, latest_event_time) in data {
            conn.execute(
                "INSERT INTO events (mac_address, latest_event_time) VALUES (?1, ?2)
                 ON CONFLICT(mac_address) DO UPDATE SET latest_event_time = ?2",
                params![mac_address, latest_event_time],
            )?;
        }

        println!("Data upserted into SQLite successfully.");
        Ok(())
    }
}