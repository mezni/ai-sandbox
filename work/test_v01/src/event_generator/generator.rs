use rand::Rng;
use chrono::{DateTime, Utc};

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
    pub fn random_mac(&self) -> String {
        let mut rng = rand::rng();
        self.mac_addresses[rng.random_range(0..self.mac_addresses.len())].clone()
    }
}

impl Iterator for EventGenerator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::rng();

        // Randomly select a MAC address from the pre-generated list
        let mac_address = self.mac_addresses[rng.random_range(0..self.mac_addresses.len())].clone();

        // Generate the current timestamp
        let event_time = Utc::now();

        // Return an Event
        Some(Event {
            mac_address,
            event_time,
        })
    }
}