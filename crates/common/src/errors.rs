use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Invalid event data")]
    InvalidEvent,

    #[error("Serialization error")]
    SerializationError,
}
