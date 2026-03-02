use crate::types::{ UserId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: UserId,
    pub new_name: String,
    pub new_email: String,
}

impl UserUpdatedEvent {
    pub const EVENT_TYPE: &'static str = "user.updated";
}
