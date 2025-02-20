mod generator;
mod processor;
mod queue;

use generator::EventGenerator;
use processor::EventProcessor;
use queue::Queue;

use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

const MAC_COUNT: usize = 10000;
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
            let queue = consumer_queue.lock().await;

            if let Ok(Some(popped_event)) = queue.pop() {
                if let Err(e) = event_processor.process(popped_event).await {
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
