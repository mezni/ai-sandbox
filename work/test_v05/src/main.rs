// src/main.rs

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Error as RusqliteError};

mod database;
mod error;
mod generator;
mod processor;
mod queue;

use database::{get_pool, initialize_database};
use error::AppError;
use generator::EventGenerator;
use processor::EventProcessor;
use queue::Queue;

use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const MAC_COUNT: usize = 10_000;
const MAC_INV_COUNT: usize = 5;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Start");

    let conn = get_pool("macs.db")?;

    initialize_database(&conn)?;

    let fields = vec![
        "mac_address".to_string(),
        "event_time".to_string(),
        "ip_address_src".to_string(),
        "port_src".to_string(),
        "ip_address_dst".to_string(),
        "port_dst".to_string(),
        "event_type".to_string(),
    ];
    let event_generator = EventGenerator::new(MAC_COUNT, MAC_INV_COUNT).await;
    let event_processor = Arc::new(Mutex::new(EventProcessor::new(fields)));
    let queue: Arc<Queue> = Arc::new(Queue::new("queue_db")?);

    // Producer Task
    let producer_queue: Arc<Queue> = Arc::clone(&queue);
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(e) = producer_queue.push(&event).await {
                eprintln!("Error pushing event: {}", e);
            }
        }
    });

    // Consumer Task
    let consumer_queue: Arc<Queue> = Arc::clone(&queue);
    let processor: Arc<Mutex<EventProcessor>> = Arc::clone(&event_processor);
    tokio::spawn(async move {
        loop {
            match consumer_queue.pop().await {
                Ok(Some(popped_event)) => {
                    let mut processor = processor.lock().await;
                    if let Err(e) = processor.process(popped_event).await {
                        eprintln!("Error processing event: {}", e);
                    }
                }
                Ok(None) => {
                    // No event available, avoid busy-waiting
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    eprintln!("Error popping event: {}", e);
                }
            }
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let count = get_macs_count(&conn)?;
        println!("Macs count: {}", count);
    }
}

fn get_macs_count(pool: &Pool<SqliteConnectionManager>) -> Result<i64, rusqlite::Error> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM mac_addresses")?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let count: i64 = row.get(0)?;
        Ok(count)
    } else {
        Ok(0)
    }
}