mod event_generator;
mod event_processor;
use event_generator::EventGenerator;
use event_processor::EventProcessor;
use tokio::sync::mpsc;
use tokio::time::Instant;
use std::collections::VecDeque;

const MAC_NUMBER: usize = 10000;
const CHANNEL_SIZE: usize = 100;
const BATCH_SIZE: usize = 5000;

#[tokio::main]
async fn main() {
    let start = Instant::now();
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

    // Start generating events in an async task
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(_) = tx.send(event).await {
                println!("Channel closed, stopping...");
                break;
            }
        }
    });

    // Process events in a separate task
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

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}