mod generator;
mod receiver;
use tokio::sync::mpsc;

const MAX_MAC_ADDRS: u64 = 100000;
const CHANNEL_SIZE: usize = 1000;

#[tokio::main]
async fn main() {
    // Create a channel for event communication
    let event_generator = generator::EventGenerator::new(MAX_MAC_ADDRS);
    let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
    let mut receiver = receiver::Receiver::new(rx);

    // Send all events
    tokio::spawn(async move {
        for event in event_generator {
            tx.send(event).await.unwrap();
        }
    });

    receiver.run().await;
}
