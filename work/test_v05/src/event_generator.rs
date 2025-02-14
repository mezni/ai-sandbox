use chrono::{DateTime, Utc};
use rand::Rng;
use serde_json::json;
use std::net::Ipv4Addr;

pub struct EventGenerator {
    pub mac_addresses: Vec<String>,
}

impl EventGenerator {
    // Initialize the EventGenerator with a given number of random MAC addresses
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
}

// Generate a random IPv4 address
fn generate_random_ipv4() -> Ipv4Addr {
    let mut rng = rand::rng();
    Ipv4Addr::new(rng.random(), rng.random(), rng.random(), rng.random())
}

impl Iterator for EventGenerator {
    type Item = serde_json::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::rng();

        // Randomly select a MAC address from the pre-generated list
        let mac_address = self.mac_addresses[rng.random_range(0..self.mac_addresses.len())].clone();

        // Generate the current timestamp
        let event_time: DateTime<Utc> = Utc::now();

        // Return JSON event
        Some(json!({
            "mac_address": mac_address,
            "event_time": event_time.to_rfc3339(),
            "ip_address_src": generate_random_ipv4().to_string(),
            "port_src": rng.random_range(1024..=64000),
            "ip_address_dst": generate_random_ipv4().to_string(),
            "port_dst": rng.random_range(1024..=64000),
        }))
    }
}
