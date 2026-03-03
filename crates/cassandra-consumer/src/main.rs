use rdkafka::consumer::Consumer;
use rdkafka::message::Message;
use tokio_stream::StreamExt;

mod cassandra;
mod config;
mod consumer;
mod processor;

use cassandra::repository::CassandraRepository;
use config::Config;
use consumer::kafka_consumer::create_consumer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let consumer = create_consumer(&config.kafka_brokers, &config.kafka_group_id);

    consumer
        .subscribe(&[&config.kafka_topic])
        .expect("Subscription failed");

    let repo = CassandraRepository::new(&config.cassandra_uri).await;

    let mut stream = consumer.stream();

    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    let payload_str = std::str::from_utf8(payload).unwrap();

                    match processor::user_processor::process_message(&repo, payload_str).await {
                        Ok(_) => {
                            consumer
                                .commit_message(&msg, rdkafka::consumer::CommitMode::Async)
                                .unwrap();
                        },
                        Err(err) => {
                            eprintln!("Kafka error: {:?}", err);
                        }
                    }
                    
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
