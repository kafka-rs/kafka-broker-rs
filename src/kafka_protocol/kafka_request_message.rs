//! # KafkaRequestMessage Module
//!
//! This module defines the [`KafkaRequestMessage`] struct, which represents an incoming request
//! from a Kafka client. The Kafka protocol typically encodes a 4-byte message size, followed by
//! a header and a payload, all in a binary format. While the current implementation does not parse
//! the header or payload in detail, it provides the foundation for future expansions.
//!
//! This code uses the [`KafkaBrokerError`] type to indicate malformed requests, invalid sizes, or
//! other issues encountered during deserialization. Each of these errors can be mapped to a
//! standard Kafka error code (see `kafka_error_codes.rs`) so the broker can return appropriate
//! responses.

use crate::kafka_protocol::kafka_error::{KafkaBrokerError, KafkaResult};
use crate::kafka_protocol::kafka_error_codes::INVALID_REQUEST;
use crate::kafka_protocol::kafka_request_header::KafkaRequestHeader;
use std::convert::TryInto;
use tracing::warn;

/// A structured representation of a Kafka request message.
///
/// Currently, this struct only stores:
/// - An integer `message_size` (parsed from the first 4 bytes),
/// - A placeholder `KafkaRequestHeader`,
/// - A placeholder `KafkaRequest` payload.
///
/// # Future Enhancements
///
/// In a more complete implementation, you'd parse additional header fields
/// (e.g., API key, API version, correlation ID, client ID length, etc.) and the
/// request payload into strongly typed structures.
#[derive(Debug)]
pub struct KafkaRequestMessage {
    /// The overall size of the message in bytes, as read from the first 4 bytes of the buffer.
    pub message_size: i32,

    /// Placeholder for the future `KafkaRequestHeader` fields.
    /// In real Kafka requests, this contains metadata such as API key, version, etc.
    pub header: KafkaRequestHeader,

    /// Placeholder for the future `KafkaRequest` fields.
    /// This is where the actual request data (e.g., topic name, partition info) will be stored.
    pub payload: KafkaRequest,
}

/// Represents the payload portion of a Kafka request.
/// In the real Kafka protocol, this section may include details such as
/// topic-partition data, message sets, and other request-specific fields.
#[derive(Debug)]
pub struct KafkaRequest;

impl KafkaRequestMessage {
    /// Constructs a [`KafkaRequestMessage`] from the given raw bytes.
    ///
    /// # Errors
    ///
    /// Returns a [`KafkaBrokerError::MalformedRequest`] if:
    /// - There are fewer than 4 bytes to read the `message_size`.
    /// - The total length of `raw_data` is not equal to `message_size + 4`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::kafka_protocol::kafka_error::KafkaResult;
    /// use crate::kafka_protocol::kafka_request_message::KafkaRequestMessage;
    ///
    /// fn handle_bytes(raw_data: &[u8]) -> KafkaResult<()> {
    ///     let req_msg = KafkaRequestMessage::from_bytes(raw_data)?;
    ///     // ...
    ///     Ok(())
    /// }
    /// ```
    pub fn from_bytes(raw_data: &[u8]) -> KafkaResult<Self> {
        // We need at least 4 bytes to read the message_size field.
        if raw_data.len() < 4 {
            return Err(KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: format!(
                    "Not enough bytes for message_size; got {} bytes.",
                    raw_data.len()
                ),
            });
        }

        // Parse the first 4 bytes as a big-endian i32, which indicates total message size.
        // We know that raw_data.len() >= 4 due to the earlier length check.
        // Therefore, this call should never fail. If it does, we consider it a logic error.
        let size_bytes = match raw_data[0..4].try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                // In theory, this branch should never be reached because we checked raw_data.len() >= 4.
                // If we do end up here, it indicates a logical/data inconsistency we didn't account for.

                // If this is truly an unrecoverable or logic error, consider error! instead.
                warn!("Unexpected inability to convert first 4 bytes to [u8; 4]. Possible data corruption?");

                return Err(KafkaBrokerError::MalformedRequest {
                    code: INVALID_REQUEST,
                    reason: "Could not convert first 4 bytes to [u8; 4].".to_string(),
                });
            }
        };
        let message_size = i32::from_be_bytes(size_bytes);

        // Check if raw_data length matches message_size + 4 (the size field + the body).
        let expected_len = message_size as usize + 4;
        if raw_data.len() != expected_len {
            return Err(KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: format!(
                    "Data length ({}) does not match the indicated message_size ({}). Expected {} bytes total.",
                    raw_data.len(),
                    message_size,
                    expected_len
                ),
            });
        }

        // For now, we use placeholder structs for the header and payload.
        let header = KafkaRequestHeader::from_bytes(&raw_data[4..])?;

        let payload = KafkaRequest;

        // Construct and return the `KafkaRequestMessage`.
        Ok(Self {
            message_size,
            header,
            payload,
        })
    }
}
