use serde_json::Value;
use tokio::sync::mpsc;
use std::process;

const MAX_EVENTS: u32 = 10000;

pub struct Receiver {
    rx: mpsc::Receiver<Value>,
}

impl Receiver {
    pub fn new(rx: mpsc::Receiver<Value>) -> Self {
        Receiver { rx }
    }

    pub async fn run(&mut self) {
        let mut events = Vec::new();
        loop {
            match self.rx.recv().await {
                Some(event) => {
                    println!("Received event: {:?}", event);
                    events.push(event);
                    if events.len() >= MAX_EVENTS as usize {
                        println!("Received {} events. Processing...", MAX_EVENTS);
                        self.process_events(events.clone()).await;
                        events.clear();
                        process::exit(0);
                    }
                }
                None => {
                    println!("Channel closed.");
                    if !events.is_empty() {
                        self.process_events(events).await;
                    }
                    break;
                }
            }
        }
    }

    async fn process_events(&self, events: Vec<Value>) {
        // Process the events here
        println!("Processing {} events...", events.len());
        // Add your processing logic here
    }
}