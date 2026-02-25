use common::envelope::EventEnvelope;
use common::events::UserCreatedEvent;

use crate::redis::client::RedisClient;

pub async fn process_message(
    redis: &RedisClient,
    payload: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let envelope: EventEnvelope<serde_json::Value> = serde_json::from_str(payload)?;

    match envelope.event_type.as_str() {
        UserCreatedEvent::EVENT_TYPE => {
            let event: UserCreatedEvent = serde_json::from_value(envelope.payload)?;

            let serialized = serde_json::to_string(&event)?;
            redis
                .set_user(&event.user_id.to_string(), &serialized)
                .await?;
        }

        _ => {}
    }

    Ok(())
}
