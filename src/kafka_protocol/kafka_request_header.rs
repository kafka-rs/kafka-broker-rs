//! # KafkaRequestHeader Module
//!
//! This file defines an enum [`KafkaRequestHeader`] that can represent one of
//! three known versions (v0, v1, or v2) of a Kafka request header. Each version
//! includes at least `api_key`, `api_version`, and `correlation_id`. Versions 1 and 2
//! add a `client_id`; version 2 adds an additional `tag_buffer` field as a raw byte buffer.
//!
//! We parse these fields from raw bytes in [`KafkaRequestHeader::from_bytes`],
//! which identifies the `api_version` and constructs the appropriate variant.

use crate::kafka_protocol::kafka_error::{KafkaBrokerError, KafkaResult};
use crate::kafka_protocol::kafka_error_codes::{INVALID_REQUEST, UNSUPPORTED_VERSION};
use std::convert::TryInto;
use tracing::{debug, trace, warn};

/// A versioned Kafka request header. Each variant corresponds to a known header
/// version.
#[derive(Debug)]
pub enum KafkaRequestHeader {
    /// Kafka request header version 0 (no client_id).
    V0(KafkaRequestHeaderV0),
    /// Kafka request header version 1 (adds a `client_id`).
    V1(KafkaRequestHeaderV1),
    /// Kafka request header version 2 (adds a `client_id` and a `tag_buffer`).
    V2(KafkaRequestHeaderV2),
}

/// Basic fields common to all Kafka request header versions.
#[derive(Debug)]
pub struct CommonHeaderFields {
    /// Identifies which Kafka API this request is using (e.g. Produce=0, Fetch=1, etc.).
    pub api_key: i16,
    /// Version of the Kafka API for this request.
    pub api_version: i16,
    /// A broker-agnostic identifier for matching requests to responses.
    pub correlation_id: i32,
}

/// Kafka request header version 0. Contains only the common fields (no `client_id`).
#[derive(Debug)]
pub struct KafkaRequestHeaderV0 {
    pub common: CommonHeaderFields,
}

/// Kafka request header version 1. Includes an optional `client_id`.
#[derive(Debug)]
pub struct KafkaRequestHeaderV1 {
    pub common: CommonHeaderFields,
    /// The client’s name or ID, if provided. Some Kafka protocol versions allow
    /// a “null” or negative length to represent no client_id.
    pub client_id: Option<String>,
}

/// Kafka request header version 2. Includes a `client_id` plus a `tag_buffer`.
#[derive(Debug)]
pub struct KafkaRequestHeaderV2 {
    pub common: CommonHeaderFields,
    pub client_id: Option<String>,
    /// Raw bytes for any “tagged fields” or other optional protocol data.
    pub tag_buffer: Vec<u8>,
}

impl KafkaRequestHeader {
    /// Constructs a [`KafkaRequestHeader`] from the given `raw_data`.
    ///
    /// **Workflow**:
    /// 1. Reads the **common** fields (api_key, api_version, correlation_id).
    /// 2. Inspects `api_version` to decide which header variant to create (v0, v1, or v2).
    /// 3. Parses additional fields (`client_id`, `tag_buffer`) if the version requires them.
    ///
    /// # Fields Layout (Big-Endian)
    ///
    /// - `[0..2]` => `api_key` (i16)
    /// - `[2..4]` => `api_version` (i16)
    /// - `[4..8]` => `correlation_id` (i32)
    /// - `[8..]`  => possibly `client_id` or other fields (depending on version)
    ///
    /// # Errors
    ///
    /// Returns [`KafkaBrokerError::MalformedRequest`] if `raw_data` is too short to read
    /// the common fields, or if further parsing fails. Returns [`KafkaBrokerError::MalformedRequest`]
    /// with code [`UNSUPPORTED_VERSION`] if `api_version` is not 0, 1, or 2 in this example.
    pub fn from_bytes(raw_data: &[u8]) -> KafkaResult<Self> {
        debug!(
            "Parsing KafkaRequestHeader from {} bytes of raw data",
            raw_data.len()
        );

        // We need at least 8 bytes for the common fields.
        if raw_data.len() < 8 {
            warn!(
                "Not enough bytes ({}) to parse the common header fields.",
                raw_data.len()
            );
            return Err(KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: format!("Header requires at least 8 bytes; found {}", raw_data.len()),
            });
        }

        // 1) Parse `api_key` (i16).
        let api_key = i16::from_be_bytes(raw_data[0..2].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read api_key (2 bytes)".to_string(),
            }
        })?);

        // 2) Parse `api_version` (i16).
        let api_version = i16::from_be_bytes(raw_data[2..4].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read api_version (2 bytes)".to_string(),
            }
        })?);

        // 3) Parse `correlation_id` (i32).
        let correlation_id = i32::from_be_bytes(raw_data[4..8].try_into().map_err(|_| {
            KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read correlation_id (4 bytes)".to_string(),
            }
        })?);

        debug!(
            "Parsed common fields -> api_key={}, api_version={}, correlation_id={}",
            api_key, api_version, correlation_id
        );

        // Create the shared fields
        let common_fields = CommonHeaderFields {
            api_key,
            api_version,
            correlation_id,
        };

        // The remainder after the first 8 bytes might contain client_id, tag buffer, etc.
        let mut cursor = &raw_data[8..];
        trace!("Remaining bytes after common fields: {}", cursor.len());

        // 4) Decide the variant based on `api_version`.
        match api_version {
            0 => {
                debug!("KafkaRequestHeader: constructing V0 header (no extra fields).");
                Ok(KafkaRequestHeader::V0(KafkaRequestHeaderV0 {
                    common: common_fields,
                }))
            }
            1 => {
                debug!("KafkaRequestHeader: constructing V1 header (client_id).");
                let client_id = parse_client_id(&mut cursor)?;
                debug!("Parsed client_id = {:?}", client_id);

                Ok(KafkaRequestHeader::V1(KafkaRequestHeaderV1 {
                    common: common_fields,
                    client_id,
                }))
            }
            2 => {
                debug!("KafkaRequestHeader: constructing V2 header (client_id, tag_buffer).");
                let client_id = parse_client_id(&mut cursor)?;
                debug!("Parsed client_id = {:?}", client_id);

                let tag_buffer = parse_tag_buffer(&mut cursor)?;
                debug!("Parsed tag_buffer length = {}", tag_buffer.len());

                Ok(KafkaRequestHeader::V2(KafkaRequestHeaderV2 {
                    common: common_fields,
                    client_id,
                    tag_buffer,
                }))
            }
            _ => {
                warn!(
                    "Unsupported api_version {} for KafkaRequestHeader",
                    api_version
                );
                Err(KafkaBrokerError::MalformedRequest {
                    code: UNSUPPORTED_VERSION,
                    reason: format!("Unsupported header version: {}", api_version),
                })
            }
        }
    }
}

/// Parses a length-delimited `client_id` from the `cursor`.
///
/// We assume the next 2 bytes are `client_id_length` (i16). If `-1`, we interpret it as `None`.
/// If it's >= 0, we read that many bytes as UTF-8.
fn parse_client_id(cursor: &mut &[u8]) -> KafkaResult<Option<String>> {
    debug!(
        "Attempting to parse `client_id` from {} remaining bytes",
        cursor.len()
    );

    // We need at least 2 bytes for the client_id length.
    if cursor.len() < 2 {
        warn!(
            "Not enough bytes ({}) to read client_id length.",
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: "Not enough bytes to read client_id length".to_string(),
        });
    }
    let length_bytes: [u8; 2] =
        cursor[..2]
            .try_into()
            .map_err(|_| KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read client_id length (2 bytes)".to_string(),
            })?;
    let client_id_len = i16::from_be_bytes(length_bytes);

    // Advance the cursor past these 2 bytes.
    *cursor = &cursor[2..];

    if client_id_len < 0 {
        debug!("client_id_len is negative => interpret as None (no client_id).");
        return Ok(None);
    }

    let client_id_len = client_id_len as usize;
    if cursor.len() < client_id_len {
        warn!(
            "client_id says length={}, but only {} bytes remain",
            client_id_len,
            cursor.len()
        );
        return Err(KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: format!(
                "client_id says length={}, but only {} bytes remain",
                client_id_len,
                cursor.len()
            ),
        });
    }

    // Extract the string bytes.
    let string_bytes = &cursor[..client_id_len];
    *cursor = &cursor[client_id_len..];

    debug!("client_id raw bytes length = {}", string_bytes.len());

    // Attempt to parse as UTF-8. If it fails, treat it as malformed.
    let client_id_str = std::str::from_utf8(string_bytes).map_err(|_| {
        warn!("client_id bytes are not valid UTF-8");
        KafkaBrokerError::MalformedRequest {
            code: INVALID_REQUEST,
            reason: "client_id is not valid UTF-8".to_string(),
        }
    })?;

    debug!("Successfully parsed client_id = {:?}", client_id_str);
    Ok(Some(client_id_str.to_string()))
}

/// Parses the `tag_buffer` for v2.
///
/// For demonstration, assume the next 2 bytes represent the `tag_buffer_length` (u16),
/// and if it's non-zero, we read that many bytes.
fn parse_tag_buffer(cursor: &mut &[u8]) -> KafkaResult<Vec<u8>> {
    debug!(
        "Attempting to parse `tag_buffer` from {} remaining bytes",
        cursor.len()
    );

    // We need 2 bytes for the tag_buffer length (u16).
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
    let length_bytes: [u8; 2] =
        cursor[..2]
            .try_into()
            .map_err(|_| KafkaBrokerError::MalformedRequest {
                code: INVALID_REQUEST,
                reason: "Failed to read tag_buffer length (2 bytes)".to_string(),
            })?;
    let tag_buffer_len = u16::from_be_bytes(length_bytes);

    debug!("tag_buffer length = {}", tag_buffer_len);

    // Advance past these 2 bytes.
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

    // Extract the raw bytes for the tag buffer.
    let buffer = cursor[..tag_buffer_len as usize].to_vec();
    *cursor = &cursor[tag_buffer_len as usize..];

    trace!("Parsed {} bytes for tag_buffer.", buffer.len());
    Ok(buffer)
}
