use serde_json::Value;
pub struct EventProcessor {}

impl EventProcessor {
    pub fn new() -> Self {
        EventProcessor {}
    }
    pub async fn process(&mut self, events: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Processing {} events", events.len());
        Ok(())
    }
}
