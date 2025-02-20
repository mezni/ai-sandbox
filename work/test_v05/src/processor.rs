use serde_json::Value;
use std::collections::VecDeque;
use std::error::Error;

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

    pub async fn process(&mut self, event: Value) -> Result<(), Box<dyn Error>> {
        // Add the event to the batch
        self.batch.push_back(event.clone());

        // Process batch when it reaches BATCH_SIZE
        if self.batch.len() >= BATCH_SIZE {
            let batch_to_process: Vec<Value> = self.batch.drain(..).collect(); // Drain the batch into a Vec
            self.process_batch(batch_to_process).await?; // Process the drained batch
        }

        Ok(())
    }

    pub async fn process_batch(&self, batch: Vec<Value>) -> Result<(), Box<dyn Error>> {
        println!("Processing batch of size: {}", batch.len());
        // Here, you can send to a database, API, or process them further

        // Example processing logic
        for event in batch {
            // Simulate processing each event
            println!("Processed event: {:?}", event);
        }

        Ok(())
    }
}
