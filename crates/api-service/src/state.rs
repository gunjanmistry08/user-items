use crate::producer::kafka_producer::KafkaProducer;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub producer: Arc<KafkaProducer>,
    pub kafka_topic: String,
}
