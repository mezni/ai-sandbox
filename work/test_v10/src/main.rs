mod event_generator;
mod event_processor;
use event_generator::EventGenerator;
use event_processor::EventProcessor;
use tokio::sync::mpsc;
use tokio::time::Instant;
//use serde_json::json;

const MAC_NUMBER: usize = 10000;
const CHANNEL_SIZE: usize = 100;
const BATCH_SIZE: usize = 5000;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE);

    let event_generator = EventGenerator::new(MAC_NUMBER).await;

    // Start generating events in an async task
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(_) = tx.send(event).await {
                println!("Channel closed, stopping...");
                break;
            }
        }
    });

    let mut i = 0;
    let mut events = Vec::new();
    while let Some(event) = rx.recv().await {
        i += 1;
        events.push(event);
        if i >= BATCH_SIZE {
            let event_processor = EventProcessor::process(events.clone());
            let now = chrono::Utc::now();
            println!("{} {}", now.to_rfc3339(), events.len());
            events = Vec::new();
            i = 0;
        }
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
