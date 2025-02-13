use tokio::sync::mpsc;
use event_generator::Event;

pub struct Receiver {
    rx: mpsc::Receiver<Event>,
}

impl Receiver {
    pub fn new(rx: mpsc::Receiver<Event>) -> Self {
        Receiver { rx }
    }

    pub async fn start(&mut self) {
        while let Some(event) = self.rx.recv().await {
            println!("Received: {:#?}", event);
        }
    }
}