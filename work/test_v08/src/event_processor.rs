use serde_json::Value;

pub struct EventProcessor;

use datafusion::arrow::{
    array::{Float64Array, StringArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use std::sync::Arc;

impl EventProcessor {
    pub fn process(events: Vec<Value>) {
        let mut mac_addresses = Vec::new();
        let mut event_times = Vec::new();
        let mut ip_address_srcs = Vec::new();
        let mut port_srcs = Vec::new();
        let mut ip_address_dsts = Vec::new();
        let mut port_dsts = Vec::new();
        let mut event_types = Vec::new();
        let mut discarded_events = Vec::new();

        for event in events {
            let (ok_mac, mac_address) = Self::extract_field(&event, "mac_address");
            let (ok_time, event_time) = Self::extract_field(&event, "event_time");
            let (ok_src_ip, ip_address_src) = Self::extract_field(&event, "ip_address_src");
            let (ok_src_port, port_src) = Self::extract_field(&event, "port_src");
            let (ok_dst_ip, ip_address_dst) = Self::extract_field(&event, "ip_address_dst");
            let (ok_dst_port, port_dst) = Self::extract_field(&event, "port_dst");
            let (ok_type, event_type) = Self::extract_field(&event, "event_type");

            if ok_mac && ok_time && ok_src_ip && ok_src_port && ok_dst_ip && ok_dst_port && ok_type
            {
                mac_addresses.push(mac_address);
                event_times.push(event_time);
                ip_address_srcs.push(ip_address_src);
                port_srcs.push(port_src);
                ip_address_dsts.push(ip_address_dst);
                port_dsts.push(port_dst);
                event_types.push(event_type);
            } else {
                discarded_events.push(event);
            }
        }

        // Print valid events count
        println!("Processed {} valid events", mac_addresses.len());
        println!("Discarded {} invalid events", discarded_events.len());

        let schema = Self::schema();

        let mac_address_array = Arc::new(StringArray::from(mac_addresses));
        let event_time_array = Arc::new(StringArray::from(event_times));
        let ip_address_src_array = Arc::new(StringArray::from(ip_address_srcs));
        let port_src_array = Arc::new(StringArray::from(port_srcs));
        let ip_address_dst_array = Arc::new(StringArray::from(ip_address_dsts));
        let port_dst_array = Arc::new(StringArray::from(port_dsts));
        let event_type_array = Arc::new(StringArray::from(event_types));

        match RecordBatch::try_new(
            schema,
            vec![
                mac_address_array,
                event_time_array,
                ip_address_src_array,
                port_src_array,
                ip_address_dst_array,
                port_dst_array,
                event_type_array,
            ],
        ) {
            Ok(batch) => println!("Created RecordBatch with {} rows", batch.num_rows()),
            Err(e) => eprintln!("Failed to create RecordBatch: {}", e),
        }
    }

    fn extract_field(event: &Value, field: &str) -> (bool, String) {
        if let Some(value) = event[field].as_str() {
            (true, value.to_string())
        } else {
            (false, String::new())
        }
    }

    fn schema() -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("mac_address", DataType::Utf8, false),
            Field::new("event_time", DataType::Utf8, false),
            Field::new("ip_address_src", DataType::Utf8, false),
            Field::new("port_src", DataType::Utf8, false),
            Field::new("ip_address_dst", DataType::Utf8, false),
            Field::new("port_dst", DataType::Utf8, false),
            Field::new("event_type", DataType::Utf8, false),
        ]))
    }
}

