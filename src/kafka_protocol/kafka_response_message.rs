//! Defines the KafkaResponseMessage struct, which represents a response
//! to send back to the client. Currently unused, since we're not creating
//! or sending responses yet.

#[derive(Debug)]
pub struct KafkaResponseMessage {
    pub message_size: i32,
    pub header: KafkaResponseHeader,
    pub payload: KafkaResponse,
}

/// Placeholder for the Kafka header (same as in `kafka_request_message.rs`).
#[derive(Debug)]
pub struct KafkaResponseHeader;

/// Placeholder for the Kafka response payload struct.
#[derive(Debug)]
pub struct KafkaResponse;
