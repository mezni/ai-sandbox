mod event_generator;
mod event_processor;
use event_generator::EventGenerator;
use event_processor::EventProcessor;
use rusqlite::{params, Connection, Result};
use std::collections::VecDeque;
use tokio::sync::mpsc;
use tokio::time::Instant;

const MAC_NUMBER: usize = 10000;
const CHANNEL_SIZE: usize = 100;
const BATCH_SIZE: usize = 5000;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let conn = Connection::open("macs.db")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS macs (
                mac_address_id INTEGER PRIMARY KEY,
                mac_address TEXT NOT NULL,
                first_seen TEXT ,
                last_seen TEXT,
                mac_vendor_id INTEGER
            )",
        [],
    )?;

    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE);

    let fields = vec![
        "mac_address".to_string(),
        "event_time".to_string(),
        "ip_address_src".to_string(),
        "port_src".to_string(),
        "ip_address_dst".to_string(),
        "port_dst".to_string(),
        "event_type".to_string(),
    ];
    let mut event_processor = EventProcessor::new(fields);

    let event_generator = EventGenerator::new(MAC_NUMBER).await;

    // Start generating events
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(_) = tx.send(event).await {
                println!("Channel closed, stopping...");
                break;
            }
        }
    });

    // Process events
    tokio::spawn(async move {
        loop {
            let mut events = Vec::new();
            for _ in 0..BATCH_SIZE {
                if let Some(event) = rx.recv().await {
                    events.push(event);
                } else {
                    println!("No event");
                    break;
                }
            }
            if !events.is_empty() {
                let _ = event_processor.process(events).await;
            }
        }
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
