use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    mac_address: String,
    #[serde(with = "ts_seconds")]
    event_time: DateTime<Utc>,
    ip_address_src: String,
    port_src: String,
    ip_address_dst: String,
    port_dst: String,
}
