mod generator;
mod queue;

use generator::EventGenerator;

use queue::Queue;

use serde::{Deserialize, Serialize};
use serde_json;
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
        queue.push_json(&event)?;
        println!("{}", event);
    }

    //    queue.push_json(&event1)?;

    //    println!("{:?}", queue.pop_json::<Event>()?);

    Ok(())
}
