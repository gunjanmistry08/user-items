use chrono::Utc;
use common::envelope::EventEnvelope;
use common::events::UserCreatedEvent;
use uuid::Uuid;

use crate::producer::kafka_producer::KafkaProducer;

pub struct UserService {
    producer: KafkaProducer,
}

impl UserService {
    pub fn new(producer: KafkaProducer) -> Self {
        Self { producer }
    }

    pub async fn create_user(&self, topic: &str, email: String) -> Result<(), String> {
        let user_id = Uuid::new_v4();

        let event = UserCreatedEvent {
            user_id,
            email,
            created_at: Utc::now(),
        };

        dbg!(&event);

        let envelope = EventEnvelope::new(UserCreatedEvent::EVENT_TYPE, event);

        dbg!(&envelope);

        let json = serde_json::to_string(&envelope).map_err(|e| e.to_string())?;

        dbg!(&json);
        
        self.producer
            .send(topic, &user_id.to_string(), &json)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
