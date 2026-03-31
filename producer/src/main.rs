use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");
    let delivery_status = producer.send(FutureRecord::to("test-topic")
                                            .payload("Hello Kafka from Rust!")
                                            .key("key1"), Duration::from_secs(0),).await;

    println!("Delivery status: {:?}", delivery_status);

    let delivery_status2 = producer.send(FutureRecord::to("test-topic2")
                                            .payload("Hello Kafka from Rust2!")
                                            .key("key2"), Duration::from_secs(0),).await;
    println!("Delivery status2: {:?}", delivery_status2);
}

