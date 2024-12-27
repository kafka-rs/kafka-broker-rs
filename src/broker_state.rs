//! broker_state.rs
//!
//! Defines the shared state for our Kafka broker, including topic metadata,
//! consumer groups, offsets, or any other data we need to share between client
//! handlers.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A shared state structure that holds Kafka-related broker data.
///
/// For instance, it can hold topic metadata, consumer groups, offsets, etc.
/// The `RwLock` allows concurrent reads but exclusive writes.
/// Use `Arc<BrokerState>` when sharing across tasks.
pub struct BrokerState {
    /// A placeholder map. Replace `String` with more complex structs if needed.
    pub _topics: RwLock<HashMap<String, String>>,
}

impl BrokerState {
    /// Constructs a new, empty `BrokerState`.
    pub fn new() -> Self {
        Self {
            _topics: RwLock::new(HashMap::new()),
        }
    }
}

/// A helper type alias if you like to reference `Arc<BrokerState>` often.
pub type SharedBrokerState = Arc<BrokerState>;
