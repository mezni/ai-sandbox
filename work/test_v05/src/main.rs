mod generator;
mod queue;

use generator::EventGenerator;

use queue::Queue;

use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json, Value};
use sled::Db;
use std::str;
#[derive(Serialize, Deserialize, Debug)]
struct Event {
    id: u64,
    message: String,
}

#[tokio::main]
async fn main() -> sled::Result<()> {
    println!("Start");
    let mut queue = Queue::new("queue_db")?;
    let mut event_generator = EventGenerator::new(100, 3).await;
    for event in event_generator {
        queue.push(&event)?;
        if let Some(popped_event) = queue.pop()? {
            if let Some(event_time) = popped_event.get("event_time") {
                println!("Event Time: {}", event_time);
            } else {
                println!("No event_time found in the popped event.");
            }
        }
    }

    Ok(())
}
