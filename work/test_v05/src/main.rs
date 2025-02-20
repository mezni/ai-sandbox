mod generator;
mod processor;
mod queue;

use generator::EventGenerator;
use processor::EventProcessor;
use queue::Queue;

use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const BATCH_SIZE: usize = 5000;
const MAC_COUNT: usize = 1000;
const MAC_INV_COUNT: usize = 5;

#[tokio::main]
async fn main() -> sled::Result<()> {
    println!("Start");

    let queue = Arc::new(Mutex::new(Queue::new("queue_db")?));
    let event_generator = EventGenerator::new(MAC_COUNT, MAC_INV_COUNT).await;
    let mut event_processor = EventProcessor::new();

    // Producer Task
    let producer_queue = Arc::clone(&queue);
    let consumer_queue = Arc::clone(&queue);

    tokio::spawn(async move {
        for event in event_generator {
            let mut queue = producer_queue.lock().await;
            if let Err(e) = queue.push(&event) {
                eprintln!("Error pushing event: {}", e);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let mut batch = Vec::new();

            while batch.len() < BATCH_SIZE {
                let queue = consumer_queue.lock().await;

                if let Ok(Some(popped_event)) = queue.pop() {
                    batch.push(popped_event);
                } else {
                    // Queue is empty, wait a bit before retrying to avoid busy-waiting
                    drop(queue); // Explicitly drop Mutex lock before sleeping
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }

            // Process the batch after collecting BATCH_SIZE events
            if let Err(e) = event_processor.process(batch).await {
                eprintln!("Error processing events: {}", e);
            }
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
