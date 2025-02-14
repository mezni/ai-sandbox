mod generator;
mod receiver;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create a channel for event communication
    let event_generator = generator::EventGenerator::new(10);
    let (tx, mut rx) = mpsc::channel(10);
    let mut receiver = receiver::Receiver::new(rx);

    // Send all events
    tokio::spawn(async move {
        for event in event_generator {
            tx.send(event).await.unwrap();
        }
    });

    receiver.run().await;
}
