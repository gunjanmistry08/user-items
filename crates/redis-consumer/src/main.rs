use rdkafka::consumer::Consumer;
use rdkafka::message::Message;
use tokio_stream::StreamExt;

mod config;
mod consumer;
mod processor;
mod redis;

use config::Config;
use consumer::kafka_consumer::create_consumer;
use redis::client::RedisClient;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let consumer = create_consumer(&config.kafka_brokers, &config.kafka_group_id);

    consumer
        .subscribe(&[&config.kafka_topic])
        .expect("Subscription failed");

    let redis = RedisClient::new(&config.redis_url);

    let mut stream = consumer.stream();

    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    let payload_str = std::str::from_utf8(payload).unwrap();

                    if processor::user_processor::process_message(&redis, payload_str)
                        .await
                        .is_ok()
                    {
                        consumer
                            .commit_message(&msg, rdkafka::consumer::CommitMode::Async)
                            .unwrap();
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
