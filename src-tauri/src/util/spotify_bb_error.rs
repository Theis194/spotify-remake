use serde::Serialize;
use thiserror::Error;

// Tauri does not like returnng erros that are not serializable
#[derive(Error, Debug, Serialize)]
pub enum BbError {
    #[error("Environment variable {0} not found")]
    EnvVarNotFound(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("No user profile found")]
    NoUserProfileError,

    #[error("Failed to make request")]
    RequestError,

    #[error("Unknown error")]
    Unknown,
}