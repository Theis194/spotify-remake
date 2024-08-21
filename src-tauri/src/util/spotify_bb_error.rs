use serde::Serialize;
use thiserror::Error;

// Tauri does not like returnng erros that are not serializable
#[derive(Error, Debug, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bb_error() {
        let error = BbError::EnvVarNotFound("test".to_string());
        assert_eq!(error.to_string(), "Environment variable test not found");
    }

    #[test]
    fn test_bb_error_deserialization() {
        let error = BbError::DeserializationError("test".to_string());
        assert_eq!(error.to_string(), "Deserialization error: test");
    }

    #[test]
    fn test_bb_error_no_user_profile() {
        let error = BbError::NoUserProfileError;
        assert_eq!(error.to_string(), "No user profile found");
    }

    #[test]
    fn test_bb_error_request() {
        let error = BbError::RequestError;
        assert_eq!(error.to_string(), "Failed to make request");
    }

    #[test]
    fn test_bb_error_unknown() {
        let error = BbError::Unknown;
        assert_eq!(error.to_string(), "Unknown error");
    }

    #[test]
    fn test_bb_error_authorization() {
        let error = BbError::AuthorizationError("test".to_string());
        assert_eq!(error.to_string(), "Authorization error: test");
    }
}