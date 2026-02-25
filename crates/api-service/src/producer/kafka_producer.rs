use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Self { producer }
    }

    pub async fn send(
        &self,
        topic: &str,
        key: &str,
        payload: &str,
    ) -> Result<(), rdkafka::error::KafkaError> {
        self.producer
            .send(
                FutureRecord::to(topic).payload(payload).key(key),
                Duration::from_secs(0),
            )
            .await
            .map(|_| ())
            .map_err(|(e, _)| e)
    }
}
