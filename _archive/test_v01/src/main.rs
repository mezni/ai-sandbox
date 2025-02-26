mod event_generator;
mod event_processor;

use event_generator::EventGenerator;
use event_processor::EventProcessor;
use rusqlite::Connection;
use std::result::Result;
use tokio::sync::mpsc;

const MAC_NUMBER: usize = 10000;
const CHANNEL_SIZE: usize = 100;
const BATCH_SIZE: usize = 5000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("macs.db")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS macs (
            mac_address_id INTEGER PRIMARY KEY,
            mac_address TEXT NOT NULL,
            first_seen TEXT,
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
            if tx.send(event).await.is_err() {
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
                match rx.recv().await {
                    Some(event) => events.push(event),
                    None => {
                        println!("Channel closed, stopping...");
                        return;
                    }
                }
            }
            if !events.is_empty() {
                if let Err(e) = event_processor.process(events).await {
                    eprintln!("Error processing events: {}", e);
                }
            }
        }
    });

    // Monitor the database
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let count = get_macs_count(&conn)?;
        println!("Macs count: {}", count);
    }
}

fn get_macs_count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM macs")?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let count: i64 = row.get(0)?;
        Ok(count)
    } else {
        Ok(0)
    }
}