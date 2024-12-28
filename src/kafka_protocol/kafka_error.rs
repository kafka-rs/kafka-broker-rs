use std::io;
use thiserror::Error;

use super::kafka_error_codes::{
    INVALID_REQUEST, UNKNOWN_SERVER_ERROR, /* plus any other codes you need */
};

/// A specialized `Result` type for Kafka broker operations.
pub type KafkaResult<T> = std::result::Result<T, KafkaBrokerError>;

/// Represents the kinds of errors that can occur in your Kafka broker.
/// Each variant can (optionally) carry the Kafka error code to be returned
/// in a future response message.
#[derive(Error, Debug)]
pub enum KafkaBrokerError {
    /// The request is malformed or otherwise invalid.
    /// Carries a Kafka error code (e.g., `INVALID_REQUEST`).
    #[error("Invalid request: {reason}")]
    MalformedRequest {
        /// The Kafka error code for an invalid request (often `INVALID_REQUEST`).
        code: i16,
        /// A human-readable description of what went wrong.
        reason: String,
    },

    /// A generic internal server error, or an unexpected situation.
    /// By default, we might map this to `UNKNOWN_SERVER_ERROR`.
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    /// Wrapping an I/O error (such as from the socket),
    /// so we can unify `io::Error` under this custom type.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// If you ever want to wrap `anyhow::Error` explicitly, you can do so.
    /// But often you'll convert directly into more specific errors instead.
    #[error("Unhandled error: {0}")]
    Other(#[from] anyhow::Error),
}

impl KafkaBrokerError {
    /// Returns the best matching Kafka error code for each variant.
    /// This helps you embed the correct code in a future response.
    pub fn error_code(&self) -> i16 {
        match self {
            KafkaBrokerError::MalformedRequest { code, .. } => *code,
            KafkaBrokerError::InternalServerError(_) => UNKNOWN_SERVER_ERROR,
            KafkaBrokerError::Io(_) => UNKNOWN_SERVER_ERROR,
            KafkaBrokerError::Other(_) => UNKNOWN_SERVER_ERROR,
        }
    }
}
