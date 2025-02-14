use tokio::sync::mpsc;
use serde_json::Value;

pub struct Receiver {
    rx: mpsc::Receiver<Value>,
}

impl Receiver {
    pub fn new(rx: mpsc::Receiver<Value>) -> Self {
        Receiver { rx }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.rx.recv().await {
            println!("Received event: {:?}", event);
        }
    }
}