mod event_generator;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create a channel for event communication
    let event_generator = event_generator::EventGenerator::new(1000);
    let (tx, mut rx) = mpsc::channel(10);

    // Send all events
    tokio::spawn(async move {
        for event in event_generator {
            tx.send(event).await.unwrap();
        }
    });

    // Receive all events
    while let Some(event) = rx.recv().await {
        println!("Received: {:#?}", event);
    }
}
