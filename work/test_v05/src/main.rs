//mod generator;

//use generator::EventGenerator;

mod queue;
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
    //    let mut event_generator = EventGenerator::new(100, 3).await;
    //    for event in event_generator {
    //        println!("{}", event);
    //    }

    let mut queue = Queue::new("queue_db")?;

    let event1 = Event {
        id: 1,
        message: "Hello, world!".to_string(),
    };
    let event2 = Event {
        id: 2,
        message: "Rust is awesome!".to_string(),
    };

    queue.push_json(&event1)?;
    queue.push_json(&event2)?;

    println!("{:?}", queue.pop_json::<Event>()?);
    println!("{:?}", queue.pop_json::<Event>()?);
    println!("{:?}", queue.pop_json::<Event>()?);

    Ok(())
}
