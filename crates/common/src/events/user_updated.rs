use crate::types::{Timestamp, UserId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: UserId,
    pub new_email: String,
    pub updated_at: Timestamp,
}

impl UserUpdatedEvent {
    pub const EVENT_TYPE: &'static str = "user.updated";
}
