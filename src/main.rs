use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{StreamConsumer, CommitMode};
use rdkafka::consumer::Consumer;
use tokio::stream::StreamExt;
use std::str;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rust-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["rusty-msgs"])
        .expect("Error consuming messages");

    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => println!("Error {}", e),
            Ok(message) => {
                match message.payload() {
                    Some(payload) => {
                        println!("{}", str::from_utf8(&payload).unwrap());
                        let result = consumer.commit_message(&message, CommitMode::Async);
                        match result {
                            Ok(_result) => {
                                println!("Committed to the queue!");
                            },
                            Err(_e) => {
                                println!("No such commitment");
                            }
                        }
                    },
                    None => {
                        println!("Empty message!");
                    }
                }
            }
        };
    }
}