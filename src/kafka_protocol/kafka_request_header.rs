//! # KafkaRequestHeader Module
//!
//! This file defines an enum [`KafkaRequestHeader`] with three variants (V0, V1, V2).
//! Each version includes more fields than the last:
//! - V0: `request_api_key`, `request_api_version`, `correlation_id`
//! - V1: Same as V0 plus `client_id`
//! - V2: Same as V1 plus a `tag_buffer`
//!
//! In a real Kafka broker, the logic to pick V0 vs. V1 vs. V2 typically depends on
//! `(api_key, api_version)` and whether that combination is in the “flexible versions” list.

use crate::kafka_protocol::kafka_error::{KafkaBrokerError, KafkaResult};
use crate::kafka_protocol::kafka_error_codes::{INVALID_REQUEST, UNSUPPORTED_VERSION};
use std::convert::TryInto;
use tracing::{debug, trace, warn};

/// A versioned Kafka request header. Each variant corresponds to a known header format.
#[derive(Debug)]
pub enum KafkaRequestHeader {
    /// Version 0: Minimal fields.
    V0(KafkaRequestHeaderV0),

    /// Version 1: Adds a `client_id`.
    V1(KafkaRequestHeaderV1),

    /// Version 2: Adds a `tag_buffer`.
    V2(KafkaRequestHeaderV2),
}

/// V0 has only the three fundamental fields.
#[derive(Debug)]
pub struct KafkaRequestHeaderV0 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
}

/// V1 includes the same three fields plus a `client_id`.
#[derive(Debug)]
pub struct KafkaRequestHeaderV1 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<String>,
}

/// V2 includes everything in V1 plus a `tag_buffer` for additional fields.
#[derive(Debug)]
pub struct KafkaRequestHeaderV2 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<String>,
    pub tag_buffer: Vec<u8>,
}

impl KafkaRequestHeader {
    /// Constructs a [`KafkaRequestHeader`] by reading from `raw_data`.
    ///
    /// # Layout
    /// - `[0..2]`: `request_api_key` (i16)
    /// - `[2..4]`: `request_api_version` (i16)
    /// - `[4..8]`: `correlation_id` (i32)
    /// - `[8..]`: additional data depending on the version
    ///
    /// We do a simple match:
    /// - if `request_api_version == 0` => V0
    /// - if `request_api_version == 1` => V1
    /// - if `request_api_version >= 2` => V2
    ///
    /// In real Kafka, you might base this on whether `(api_key, api_version)` is known to
    /// be “flexible,” etc.
    pub fn from_bytes(raw_data: &[u8]) -> KafkaResult<Self> {
        debug!("Parsing KafkaRequestHeader from {} bytes", raw_data.len());

        // Need at least 8 bytes for the fundamental fields.
        if raw_data.len() < 8 {
            warn!(
                "Not enough bytes ({}) to parse the fundamental header fields.",
                raw_data.len()
            );
            return Err(KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: format!(
                    "Header requires at least 8 bytes; only found {}",
                    raw_data.len()
                ),
            });
        }

        // Parse the three shared fields
        let request_api_key = i16::from_be_bytes(raw_data[0..2].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read request_api_key (2 bytes)".to_string(),
            }
        })?);

        let request_api_version = i16::from_be_bytes(raw_data[2..4].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read request_api_version (2 bytes)".to_string(),
            }
        })?);

        let correlation_id = i32::from_be_bytes(raw_data[4..8].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read correlation_id (4 bytes)".to_string(),
            }
        })?);

        debug!(
            "Parsed: api_key={}, api_version={}, correlation_id={}",
            request_api_key, request_api_version, correlation_id
        );

        // Start parsing version-specific data from raw_data[8..]
        let mut cursor = &raw_data[8..];
        trace!("Bytes remaining after fundamental fields: {}", cursor.len());

        // We'll do a naive match on request_api_version:
        match request_api_version {
            0 => {
                debug!("Constructing V0 header (no extra fields).");
                Ok(KafkaRequestHeader::V0(KafkaRequestHeaderV0 {
                    request_api_key,
                    request_api_version,
                    correlation_id,
                }))
            }
            1 => {
                debug!("Constructing V1 header (client_id).");
                let client_id = parse_legacy_string(&mut cursor)?;
                debug!("Parsed client_id={:?}", client_id);

                Ok(KafkaRequestHeader::V1(KafkaRequestHeaderV1 {
                    request_api_key,
                    request_api_version,
                    correlation_id,
                    client_id,
                }))
            }
            2..=i16::MAX => {
                debug!("Constructing V2 header (client_id + tag_buffer).");
                let client_id = parse_legacy_string(&mut cursor)?;
                debug!("Parsed client_id={:?}", client_id);

                let tag_buffer = parse_tag_buffer(&mut cursor)?;
                debug!("Parsed tag_buffer length={}", tag_buffer.len());

                Ok(KafkaRequestHeader::V2(KafkaRequestHeaderV2 {
                    request_api_key,
                    request_api_version,
                    correlation_id,
                    client_id,
                    tag_buffer,
                }))
            }
            _ => {
                // If somehow request_api_version is negative or we just don't support it
                warn!(
                    "Unsupported request_api_version={} for KafkaRequestHeader",
                    request_api_version
                );
                Err(KafkaBrokerError::MalformedRequest {
                    code: UNSUPPORTED_VERSION,
                    reason: format!("Unsupported header version: {}", request_api_version),
                })
            }
        }
    }
}

/// Parse a normal 2-byte length “legacy” string (like older Kafka versions).
/// If the length is -1, we interpret that as `None`.
fn parse_legacy_string(cursor: &mut &[u8]) -> KafkaResult<Option<String>> {
    debug!(
        "Parsing legacy string from {} remaining bytes...",
        cursor.len()
    );
    if cursor.len() < 2 {
        warn!(
            "Not enough bytes ({}) to read legacy string length.",
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: "Not enough bytes to read string length".to_string(),
        });
    }
    let length_bytes: [u8; 2] =
        cursor[..2]
            .try_into()
            .map_err(|_| KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read string length (2 bytes)".to_string(),
            })?;
    let length = i16::from_be_bytes(length_bytes);
    *cursor = &cursor[2..];

    if length < 0 {
        // A negative length means “null” string in old Kafka protocols.
        debug!("Parsed negative string length => treating as None.");
        return Ok(None);
    }

    let length = length as usize;
    if cursor.len() < length {
        warn!(
            "Declared string length={}, but only {} bytes remain",
            length,
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: format!(
                "String says length={}, but only {} bytes remain",
                length,
                cursor.len()
            ),
        });
    }

    let bytes = &cursor[..length];
    *cursor = &cursor[length..];

    let s = std::str::from_utf8(bytes).map_err(|_| {
        warn!("String bytes are not valid UTF-8.");
        KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: "String is not valid UTF-8".to_string(),
        }
    })?;

    debug!("Successfully parsed string {:?}", s);
    Ok(Some(s.to_string()))
}

/// For V2, parse a “tag_buffer”.
/// Let’s assume we read 2 bytes => `tag_buffer_len`, then read that many bytes.
fn parse_tag_buffer(cursor: &mut &[u8]) -> KafkaResult<Vec<u8>> {
    debug!(
        "Parsing tag_buffer from {} remaining bytes...",
        cursor.len()
    );
    if cursor.len() < 2 {
        warn!(
            "Not enough bytes ({}) to read tag_buffer length.",
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: "Not enough bytes to read tag_buffer length".to_string(),
        });
    }
    let len_bytes: [u8; 2] =
        cursor[..2]
            .try_into()
            .map_err(|_| KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read tag_buffer length (2 bytes)".to_string(),
            })?;
    let tag_buffer_len = u16::from_be_bytes(len_bytes);
    *cursor = &cursor[2..];

    if cursor.len() < tag_buffer_len as usize {
        warn!(
            "tag_buffer says length={}, but only {} bytes remain",
            tag_buffer_len,
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: format!(
                "tag_buffer says length={}, but only {} bytes remain",
                tag_buffer_len,
                cursor.len()
            ),
        });
    }

    let buffer = cursor[..tag_buffer_len as usize].to_vec();
    *cursor = &cursor[tag_buffer_len as usize..];

    trace!("Parsed tag_buffer of length={}", buffer.len());
    Ok(buffer)
}
