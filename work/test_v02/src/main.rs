use chrono::{DateTime, Utc};
use rand::Rng;
use serde_json::json;

#[derive(Debug)]
pub struct Event {
    pub mac_address: String,
    pub event_time: DateTime<Utc>,
}

// EventGenerator struct
pub struct EventGenerator {
    pub mac_addresses: Vec<String>, // Store the 10 MAC addresses
}

impl EventGenerator {
    // Initialize the EventGenerator with 10 random MAC addresses
    pub fn new(size: u64) -> Self {
        let mut rng = rand::rng();
        let mac_addresses: Vec<String> = (0..size)
            .map(|_| {
                let mac: [u8; 6] = rng.random();
                format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
                )
            })
            .collect();

        EventGenerator { mac_addresses }
    }

    // Return a random MAC address
    pub async fn random_mac(&self) -> String {
        let mut rng = rand::rng();
        self.mac_addresses[rng.random_range(0..self.mac_addresses.len())].clone()
    }
}

impl Iterator for EventGenerator {
    type Item = serde_json::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::rng();

        // Randomly select a MAC address from the pre-generated list
        let mac_address = self.mac_addresses[rng.random_range(0..self.mac_addresses.len())].clone();

        // Generate the current timestamp
        let event_time = Utc::now();

        // Return JSON
        Some(json!({
            "mac_address": mac_address,
            "event_time": event_time.to_rfc3339(),
        }))
    }
}

#[tokio::main]
async fn main() {
    let mut generator = EventGenerator::new(10);
    for _ in 0..10 {
        println!("{:?}", generator.next());
    }
}
