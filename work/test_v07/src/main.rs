use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::Ipv4Addr;

use tokio::sync::mpsc;
use tokio::time::Instant;

const BUFFER_SIZE: usize = 5000;
const MIN_PORT: u16 = 1024;
const MAX_PORT: u16 = 32000;
const START_TIME_INTERVAL_MINUTES: u8 = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    mac_address: String,
    #[serde(with = "ts_seconds")]
    event_time: DateTime<Utc>,
    ip_address_src: String,
    port_src: String,
    ip_address_dst: String,
    port_dst: String,
    event_type: String,
}

/// Generates a random IPv4 address.
async fn generate_random_ipv4(rng: &mut impl Rng) -> Ipv4Addr {
    Ipv4Addr::new(rng.random(), rng.random(), rng.random(), rng.random())
}

fn generate_random_mac(rng: &mut impl Rng) -> String {
    let mac: [u8; 6] = rng.random();
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

/// Generates a pair of events (open and close) with random data.
async fn generate_event(rng: &mut impl Rng, mac_addresses: &[String]) -> (Event, Event) {
    let now = Utc::now();
    let start_interval = now - Duration::minutes(START_TIME_INTERVAL_MINUTES as i64);
    let random_seconds = rng.random_range(0..60);
    let start_ts = start_interval + Duration::seconds(random_seconds as i64);
    let duration = rng.random_range(0..60);
    let end_ts = start_ts + Duration::seconds(duration as i64);

    let mac_address = mac_addresses[rng.random_range(0..mac_addresses.len())].clone();
    let ip_address_src = generate_random_ipv4(rng).await.to_string();
    let port_src = rng.random_range(MIN_PORT..=MAX_PORT).to_string();
    let ip_address_dst = generate_random_ipv4(rng).await.to_string();
    let port_dst = rng.random_range(MIN_PORT..=MAX_PORT).to_string();

    let event_open = Event {
        mac_address: mac_address.clone(),
        event_time: start_ts,
        ip_address_src: ip_address_src.clone(),
        port_src: port_src.clone(),
        ip_address_dst: ip_address_dst.clone(),
        port_dst: port_dst.clone(),
        event_type: "open".to_string(),
    };

    let event_close = Event {
        mac_address,
        event_time: end_ts,
        ip_address_src,
        port_src,
        ip_address_dst,
        port_dst,
        event_type: "close".to_string(),
    };

    (event_open, event_close)
}

pub struct EventGenerator {
    mac_addresses: Vec<String>,
    buffer: Vec<(DateTime<Utc>, Event)>,
}

impl EventGenerator {
    pub async fn new(size: usize) -> Self {
        let mut rng = rand::rng();

        // Generate random MAC addresses
        let mac_addresses: Vec<String> = (0..size).map(|_| generate_random_mac(&mut rng)).collect();

        // Generate events and fill the buffer
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);
        while buffer.len() < BUFFER_SIZE {
            let (event_open, event_close) = generate_event(&mut rng, &mac_addresses).await;
            buffer.push((event_open.event_time, event_open));
            buffer.push((event_close.event_time, event_close));
        }

        EventGenerator {
            mac_addresses,
            buffer,
        }
    }
    async fn fill_buffer(&mut self) {
        let mut rng = rand::rng();
        while self.buffer.len() < BUFFER_SIZE {
            let (event_open, event_close) = generate_event(&mut rng, &self.mac_addresses).await;
            self.buffer.push((event_open.event_time, event_open));
            self.buffer.push((event_close.event_time, event_close));
        }
    }
}

impl Iterator for EventGenerator {
    type Item = serde_json::Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill_buffer();

        self.buffer.sort_by_key(|(event_time, _)| *event_time);

        let (event_time, event) = self.buffer.remove(0);

        // Return the event as a JSON value
        Some(json!({
            "mac_address": event.mac_address,
            "event_time": event.event_time.to_rfc3339(),
            "ip_address_src": event.ip_address_src,
            "port_src": event.port_src,
            "ip_address_dst": event.ip_address_dst,
            "port_dst": event.port_dst,
            "event_type": event.event_type,
        }))
    }
}

#[tokio::main]
async fn main() {
    /*
        while let Some(event_json) = event_generator.next() {
            println!("Event JSON: {}", event_json);
        }
    */
    let start = Instant::now();
    let (tx, mut rx) = mpsc::channel(100);

    let event_generator = EventGenerator::new(1000).await;

    // Start generating events in an async task
    tokio::spawn(async move {
        for event in event_generator {
            if let Err(_) = tx.send(event).await {
                println!("Channel closed, stopping...");
                break;
            }
        }
    });

    let mut i = 0;
    while let Some(event) = rx.recv().await {
        i += 1;
        println!("{} {}", i, event);
        /*
                if let Some(obj) = event.as_object() {
                    //    sleep(std::time::Duration::from_millis(10)).await;
                    println!("{} {}", i, event);
                }
        */
        if i >= 1000 {
            // Stop after receiving 4000 events
            break;
        }
    }
    /*
    loop {
        match rx.try_recv() {
            Ok(event) => {
                // Channel is not empty, process the event
                println!("OK");
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Channel is empty
                println!("empty");
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // Channel is disconnected
                println!("disconnected");
            }
        }
    }
    */
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
