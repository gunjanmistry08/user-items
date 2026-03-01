use crate::types::{Timestamp, UserId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct UserCreatedEvent {
    pub user_id: UserId,
    pub email: String,
    pub created_at: Timestamp,
}

impl UserCreatedEvent {
    pub const EVENT_TYPE: &'static str = "user.created";
}
