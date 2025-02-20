use serde_json::Value;
use std::collections::VecDeque;

const BATCH_SIZE: usize = 5000;

pub struct EventProcessor {
    batch: VecDeque<Value>,
}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {
            batch: VecDeque::new(),
        }
    }

    pub async fn process(&mut self, event: Value) -> Result<(), Box<dyn std::error::Error>> {
        // Add the event to the batch
        self.batch.push_back(event.clone());

        // Process batch when it reaches BATCH_SIZE
        if self.batch.len() >= BATCH_SIZE {
            println!("Processing batch of size: {}", self.batch.len());

            // Here, you can send to a database, API, or process them further
            self.batch.clear(); // Clear the batch after processing
        }

        Ok(())
    }
}
