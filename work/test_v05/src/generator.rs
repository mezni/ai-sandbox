use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use rand::prelude::IndexedRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::Ipv4Addr;

const MIN_PORT: u16 = 1024;
const MAX_PORT: u16 = 32000;
const START_TIME_INTERVAL_MINUTES: u8 = 2;
const BUFFER_SIZE: usize = 10000;

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

pub struct EventGenerator {
    mac_addresses: Vec<String>,
    buffer: Vec<(DateTime<Utc>, Event)>,
}

fn generate_random_mac(rng: &mut impl Rng) -> String {
    let mac: [u8; 6] = rng.random();
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

fn generate_random_ipv4(rng: &mut impl Rng) -> Ipv4Addr {
    Ipv4Addr::new(rng.random(), rng.random(), rng.random(), rng.random())
}

fn generate_event(rng: &mut impl Rng, mac_addresses: &[String]) -> (Event, Event) {
    let now = Utc::now();
    let start_interval = now - Duration::minutes(START_TIME_INTERVAL_MINUTES as i64);
    let random_seconds = rng.random_range(0..60);
    let start_ts = start_interval + Duration::seconds(random_seconds as i64);
    let duration = rng.random_range(0..60);
    let end_ts = start_ts + Duration::seconds(duration as i64);

    let mac_address = mac_addresses[rng.random_range(0..mac_addresses.len())].clone();
    let ip_address_src = generate_random_ipv4(rng).to_string();
    let port_src = rng.random_range(MIN_PORT..=MAX_PORT).to_string();
    let ip_address_dst = generate_random_ipv4(rng).to_string();
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

impl EventGenerator {
    pub async fn new(mac_size: usize, mac_invalid_size: usize) -> Self {
        let mut rng = rand::rng();

        let mac_addresses_invalid: Vec<String> = (0..mac_invalid_size)
            .map(|_| {
                let mac = generate_random_mac(&mut rng);
                mac.get(1..)
                    .map(|rest| format!("Z{}", rest))
                    .unwrap_or_else(|| "Z".to_string())
            })
            .collect();

        let mut mac_addresses: Vec<String> = (0..mac_size - mac_invalid_size)
            .map(|_| generate_random_mac(&mut rng))
            .collect();

        mac_addresses.extend(mac_addresses_invalid);

        let mut buffer = Vec::with_capacity(BUFFER_SIZE);

        while buffer.len() < BUFFER_SIZE {
            let random_selection_number = mac_size / 4;
            let random_macs: Vec<String> = mac_addresses
                .choose_multiple(&mut rng, random_selection_number)
                .cloned()
                .collect();

            let (event_open, event_close) = generate_event(&mut rng, &random_macs);
            buffer.push((event_open.event_time, event_open));
            buffer.push((event_close.event_time, event_close));
        }

        EventGenerator {
            mac_addresses,
            buffer,
        }
    }

    fn fill_buffer(&mut self) {
        let mut rng = rand::rng();
        while self.buffer.len() < BUFFER_SIZE {
            let random_selection_number = self.mac_addresses.len() / 4;
            let random_macs: Vec<String> = self
                .mac_addresses
                .choose_multiple(&mut rng, random_selection_number)
                .cloned()
                .collect();

            let (event_open, event_close) = generate_event(&mut rng, &random_macs);
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
        let (_, event) = self.buffer.remove(0);
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
