//! Server error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Server result type
pub type ServerResult<T> = Result<T, ServerError>;

/// Server error types
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Error response for JSON API
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,

    /// HTTP status code
    pub status: u16,

    /// Request ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::Internal(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ServerError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ServerError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ServerError::Config(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ServerError::Io(ref err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            status: status.as_u16(),
            request_id: None,
        });

        (status, body).into_response()
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ServerError::NotFound("Resource not found".to_string());
        assert_eq!(error.to_string(), "Not found: Resource not found");
    }

    #[test]
    fn test_error_response_json() {
        let response = ErrorResponse {
            error: "Test error".to_string(),
            status: 400,
            request_id: Some("123".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test error"));
        assert!(json.contains("400"));
    }
}
