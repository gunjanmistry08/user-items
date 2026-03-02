use common::envelope::EventEnvelope;
use common::events::UserCreatedEvent;

use crate::cassandra::repository::CassandraRepository;

pub async fn process_message(
    repo: &CassandraRepository,
    payload: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let envelope: EventEnvelope<serde_json::Value> = serde_json::from_str(payload)?;
    dbg!(&envelope);

    match envelope.event_type.as_str() {
        UserCreatedEvent::EVENT_TYPE => {
            let event: UserCreatedEvent = serde_json::from_value(envelope.payload)?;
            dbg!(&event);
            repo.insert_user(
                &event.user_id.to_string(),
                &event.email.to_string(),
                &event.name.to_string(),
            )
            .await?;
        }

        _ => {}
    }

    Ok(())
}
