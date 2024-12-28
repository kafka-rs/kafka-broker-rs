//! This module provides the `handle_client` function, responsible for parsing incoming client
//! requests, managing message flow, and handling acknowledgments within a Kafka-like broker. It
//! leverages the shared broker state (topics, offsets, etc.) to coordinate concurrency and data
//! consistency.
//!
//! # Network
//!
//! Kafka uses a binary protocol over TCP. The protocol defines all APIs as request response message pairs. All messages are size delimited and are made up of the following primitive types.
//!
//! The client initiates a socket connection and then writes a sequence of request messages and reads back the corresponding response message. No handshake is required on connection or disconnection. TCP is happier if you maintain persistent connections used for many requests to amortize the cost of the TCP handshake, but beyond this penalty connecting is pretty cheap.
//!
//! The client will likely need to maintain a connection to multiple brokers, as data is partitioned and the clients will need to talk to the server that has their data. However it should not generally be necessary to maintain multiple connections to a single broker from a single client instance (i.e. connection pooling).
//!
//! The server guarantees that on a single TCP connection, requests will be processed in the order they are sent and responses will return in that order as well. The broker's request processing allows only a single in-flight request per connection in order to guarantee this ordering. Note that clients can (and ideally should) use non-blocking IO to implement request pipelining and achieve higher throughput. i.e., clients can send requests even while awaiting responses for preceding requests since the outstanding requests will be buffered in the underlying OS socket buffer. All requests are initiated by the client, and result in a corresponding response message from the server except where noted.
//!
//! The server has a configurable maximum limit on request size and any request that exceeds this limit will result in the socket being disconnected.
use crate::broker_state::SharedBrokerState;
use crate::kafka_protocol::kafka_request_message::KafkaRequestMessage;
use anyhow::{Context, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, info, instrument, trace};

/// Handles a single client connection, continuously reading requests and sending responses
/// until the client disconnects or an unrecoverable error occurs.
///
/// ## Workflow
///
/// 1. **Read** the raw bytes from the socket via [`read_request`].
/// 2. **Parse** the bytes into a request object via [`parse_request`].
/// 3. **Construct** an appropriate response using [`create_response`].
/// 4. **Send** the response back to the client via [`send_response`].
///
/// This loop continues until the socket returns 0 bytes (indicating the client closed the connection)
/// or an unrecoverable error is encountered. Errors bubble up as [`anyhow::Error`] and are handled
/// by the caller.
///
/// # Parameters
///
/// * `socket` - The client connection stream over which requests and responses flow.
/// * `state` - An [`SharedBrokerState`] that contains shared broker data (topic metadata, offsets, etc.).
#[instrument(skip(socket, state))]
pub async fn handle_client(mut socket: TcpStream, state: SharedBrokerState) -> Result<()> {
    info!("Starting client handler loop for a new connection.");

    loop {
        // 1) Read the request data
        let raw_data = read_request(&mut socket).await?;

        // If no data is returned, it typically means the client closed the connection.
        if raw_data.is_empty() {
            info!("Client appears to have disconnected; ending client handler loop.");
            break;
        }

        // 2) Parse the request
        debug!(
            "Attempting to parse request ({} bytes) from client.",
            raw_data.len()
        );
        let request_message = KafkaRequestMessage::from_bytes(&raw_data)?;

        // Log the parsed request object at debug level:
        debug!("Parsed KafkaRequestMessage: {:#?}", request_message);

        // 3) Create the response
        debug!("Generating response based on the parsed request.");
        let response = create_response(request_message, &state)?;

        // 4) Send back the response
        debug!("Sending response ({} bytes) to client.", response.len());
        send_response(&mut socket, &response).await?;
    }

    info!("Client handler loop has finished normally.");
    Ok(())
}

/// Reads a chunk of bytes from the socket into memory (up to 1024 bytes).
///
/// In a real Kafka broker, you'd likely use a protocol-aware reader to handle
/// variable-length messages or correlation IDs. For now, we do a simple, single read call.
///
/// # Errors
///
/// Returns a [`std::io::Error`] (wrapped in [`anyhow::Error`]) if the read fails.
/// This could be due to network issues, client disconnection, or other I/O errors.
async fn read_request(socket: &mut TcpStream) -> Result<Vec<u8>> {
    let mut buf = [0u8; 1024];
    let bytes_read = socket
        .read(&mut buf)
        .await
        .context("Failed to read from socket")?;

    debug!("read_request: read {} bytes from the socket.", bytes_read);

    // If `bytes_read` is 0, the client is likely done (EOF).
    Ok(buf[..bytes_read].to_vec())
}

/// Constructs a response based on the parsed request and the shared broker state.
///
/// For instance, if the request is a "fetch" request, you might look up the topic data
/// in [`BrokerState`], assemble the latest messages, and encode them into a response frame.
/// In a real implementation, you’d also handle errors (e.g., unknown topic) gracefully.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if any error occurs while building the response (for example,
/// if data required from [`BrokerState`] is unavailable or the serialization fails).
fn create_response(
    _request_message: KafkaRequestMessage,
    _state: &SharedBrokerState,
) -> Result<Vec<u8>> {
    // Eventually, this will produce a meaningful byte buffer representing the response.
    todo!("Implement response creation logic (e.g., produce/fetch responses)");
}

/// Sends the response bytes back to the client by writing them to the TCP socket.
///
/// This function handles writing the entire buffer, retrying as needed until all bytes
/// are sent or an error occurs. In a production scenario, you might split large responses
/// into multiple chunks or handle backpressure scenarios differently.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if the write operation fails (e.g., client disconnects mid-write).
async fn send_response(socket: &mut TcpStream, response: &[u8]) -> Result<()> {
    socket
        .write_all(response)
        .await
        .context("Failed to write response to socket")?;

    trace!(
        "send_response: successfully wrote {} bytes to the client.",
        response.len()
    );
    Ok(())
}
