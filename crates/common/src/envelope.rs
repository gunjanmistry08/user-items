use crate::types::Timestamp;
use crate::version::EVENT_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: String,
    pub event_type: String,
    pub schema_version: String,
    pub occurred_at: Timestamp,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(event_type: &str, payload: T) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            schema_version: EVENT_SCHEMA_VERSION.to_string(),
            occurred_at: chrono::Utc::now(),
            payload,
        }
    }
}
