mod event_generator;

fn main() {
    let event_generator = event_generator::EventGenerator::new(10);

    for (i, event) in event_generator.take(10).enumerate() {
        println!(
            "Event {}: MAC Address = {}, Event Time = {}",
            i + 1, event.mac_address, event.event_time
        );
    }
}