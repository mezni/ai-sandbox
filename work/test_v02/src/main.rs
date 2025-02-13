mod generator;
use tokio;

#[tokio::main]
async fn main() {
    let mut generator = generator::EventGenerator::new(10);
    for _ in 0..10 {
        println!("{:?}", generator.next());
    }
}
