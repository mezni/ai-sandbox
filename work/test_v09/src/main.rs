use std::sync::Arc;
use datafusion::arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};

pub struct EventProcessor {
    fields: Vec<String>,
    schema: Arc<Schema>,
}

impl EventProcessor {
    fn new(fields: Vec<String>) -> Self {
        let schema = Arc::new(Schema::new(
            fields.iter().map(|field| Field::new(field.as_str(), DataType::Utf8, false)).collect::<Vec<_>>(),
        ));
        EventProcessor { fields, schema }
    }
    
    fn process(&self) {
        println!("Processing fields:");
        for field in &self.fields {
            println!("{}", field);
        }
    }
    
    fn schema(&self) -> Arc<Schema> {
        self.schema.clone()
    }
}

fn main() {
    let fields = vec![
        "mac_address".to_string(), 
        "event_time".to_string(), 
        "ip_address_src".to_string(), 
        "port_src".to_string(),
        "ip_address_dst".to_string(), 
        "port_dst".to_string(), 
        "event_type".to_string(),
    ];
    let processor = EventProcessor::new(fields);
    println!("Start");
    processor.process();
    let schema = processor.schema();
    println!("Schema: {:?}", schema);
}