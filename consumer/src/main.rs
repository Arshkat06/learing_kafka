use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::message::Message;

async fn run_consumer(group_id: &str, name: &str,topic_name:&str) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic_name]).unwrap();

    println!("{} listening...", name);

    loop {
        match consumer.recv().await {
            Err(e) => println!("{} error: {}", name, e),
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    println!(
                        "{} received: {}",
                        name,
                        String::from_utf8_lossy(payload)
                    );
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let c1 = tokio::spawn(run_consumer("group-1", "Consumer-1","test-topic"));
    let c2 = tokio::spawn(run_consumer("group-2", "Consumer-2","test-topic2"));

    let _ = tokio::join!(c1, c2);
}