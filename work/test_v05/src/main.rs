mod generator;
mod processor;
mod queue;

use generator::EventGenerator;
use processor::EventProcessor;
use queue::Queue;

use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const MAC_COUNT: usize = 10_000;
const MAC_INV_COUNT: usize = 5;

#[tokio::main]
async fn main() -> sled::Result<()> {
    println!("Start");

    let queue = Arc::new(Queue::new("queue_db")?);
    let event_generator = EventGenerator::new(MAC_COUNT, MAC_INV_COUNT).await;
    let event_processor = Arc::new(Mutex::new(EventProcessor::new()));

    // Producer Task
    let producer_queue = Arc::clone(&queue);
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(e) = producer_queue.push(&event).await {
                eprintln!("Error pushing event: {}", e);
            }
        }
    });

    // Consumer Task
    let consumer_queue = Arc::clone(&queue);
    let processor = Arc::clone(&event_processor);
    tokio::spawn(async move {
        loop {
            if let Ok(Some(popped_event)) = consumer_queue.pop().await {
                let mut processor = processor.lock().await;
                if let Err(e) = processor.process(popped_event).await {
                    eprintln!("Error processing event: {}", e);
                }
            } else {
                // No event available, avoid busy-waiting
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
