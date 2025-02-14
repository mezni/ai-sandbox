mod event;
mod event_generator;
mod event_processor;

//use tokio::sync::mpsc;
//use event_generator::event_generator;
//use event_processor::event_processor;

#[tokio::main]
async fn main() {
    let mut event_generator = event_generator::EventGenerator::new(100);
    for _ in 0..5 {
        if let Some(event) = event_generator.next() {
            println!("{}", event);
        }
    }
}
