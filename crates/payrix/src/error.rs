//! Error types for the Payrix client.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A Payrix API error returned in the response body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayrixApiError {
    /// Error message
    pub msg: String,
    /// Field that caused the error (if applicable)
    #[serde(default)]
    pub field: Option<String>,
    /// Numeric error code
    #[serde(default)]
    pub code: Option<i32>,
    /// String error code
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error severity
    #[serde(default)]
    pub severity: Option<i32>,
}

impl fmt::Display for PayrixApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        if let Some(field) = &self.field {
            write!(f, " (field: {})", field)?;
        }
        if let Some(code) = &self.error_code {
            write!(f, " [{}]", code)?;
        }
        Ok(())
    }
}

/// All possible errors from the Payrix client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Rate limited by the API - too many requests
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// One or more errors returned by the Payrix API
    #[error("Payrix API error: {}", format_api_errors(.0))]
    Api(Vec<PayrixApiError>),

    /// HTTP transport error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Bad request - invalid parameters
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unauthorized - invalid API key
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Service unavailable - Payrix is down
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Unprocessable entity - parameters are correct but request can't be processed
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    Internal(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation error - invalid input parameters
    #[error("Validation error: {0}")]
    Validation(String),

    /// IO error - file operations failed
    #[error("IO error: {0}")]
    Io(String),

    /// Database error (when cache feature is enabled)
    #[cfg(feature = "cache")]
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

fn format_api_errors(errors: &[PayrixApiError]) -> String {
    errors
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}

impl Error {
    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::RateLimited(_) | Error::ServiceUnavailable(_) | Error::Http(_)
        )
    }

    /// Create an API error from a list of Payrix errors.
    pub fn from_api_errors(errors: Vec<PayrixApiError>) -> Self {
        Error::Api(errors)
    }
}

/// Result type alias for Payrix operations.
pub type Result<T> = std::result::Result<T, Error>;
