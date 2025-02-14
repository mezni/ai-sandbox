use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: u32,
    message: String,
    timestamp: u64,
}

// Event Generator
async fn event_generator(mut sender: mpsc::Sender<Event>) {
    let mut id = 0;
    loop {
        id += 1;
        let event = Event {
            id,
            message: format!("Event {}", id),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        // Send the event to the receiver
        if let Err(e) = sender.send(event).await {
            eprintln!("Failed to send event: {}", e);
            break;
        }

        println!("Generated event: {}", id);

        // Wait before generating the next event
        sleep(Duration::from_secs(1)).await;
    }
}

// Event Processor
async fn event_processor(mut receiver: mpsc::Receiver<Event>) {
    while let Some(event) = receiver.recv().await {
        println!("Processing event: {:?}", event);

        // Simulate processing by sleeping for a short time
        sleep(Duration::from_millis(500)).await;

        println!("Finished processing event: {}", event.id);
    }
}

#[tokio::main]
async fn main() {
    // Create a channel for communication between generator and processor
    let (sender, receiver) = mpsc::channel(32);

    // Spawn the event generator and processor tasks
    let generator_handle = tokio::spawn(event_generator(sender));
    let processor_handle = tokio::spawn(event_processor(receiver));

    // Wait for both tasks to complete (they won't in this example)
    let _ = tokio::join!(generator_handle, processor_handle);
}