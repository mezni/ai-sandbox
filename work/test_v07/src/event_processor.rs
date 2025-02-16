use serde_json::json;
use serde_json::Value;

pub struct EventProcessor;

impl EventProcessor {
    pub fn process(events: Vec<Value>) {
        for event in events {
            println!("{}", json!(event));
        }
    }
}
