//! Defines configuration for our Kafka broker, including reading
//! from environment variables or an optional `.env` file.

use std::env;
use tracing::{debug, info, warn};

/// Represents the runtime configuration for the Kafka broker.
///
/// Constructed by reading environment variables (optionally from a `.env` file)
/// and falling back to sensible defaults if missing.
#[derive(Debug)]
pub struct Config {
    /// The host/IP address to bind the broker to.
    pub host: String,
    /// The port to bind the broker to.
    pub port: u16,
    /// Timeout in seconds for draining client tasks during shutdown.
    pub client_drain_timeout_secs: u64,
}

impl Config {
    /// Loads configuration by attempting to read environment variables from a
    /// `.env` file (if present). If `.env` is missing, a warning is logged and
    /// defaults are used. If `.env` is found but cannot be parsed, an error is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if a `.env` file is found but cannot be parsed. If the
    /// file is merely missing, a warning is logged instead of returning an error.
    pub fn from_env() -> anyhow::Result<Self> {
        // Attempt to load environment variables from `.env`.
        match dotenvy::dotenv() {
            Ok(path) => {
                info!("Loaded environment variables from {:?}", path);
            }
            Err(e) if e.not_found() => {
                // If it's just that `.env` is missing, log and keep going.
                warn!("No .env file found; relying on environment variables or defaults.");
            }
            Err(e) => {
                // If it's another error (e.g., permission denied or parse error),
                // return an error so we don't silently ignore it.
                return Err(e.into());
            }
        }

        // For debug purposes, log all environment variables.
        debug!("Environment variables: {:#?}", env::vars());

        // Read the host/port from the environment, with defaults if missing.
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9092);

        // Read the drain timeout (in seconds) from environment, default to 5 if not set.
        let client_drain_timeout_secs: u64 = env::var("CLIENT_DRAIN_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        Ok(Self {
            host,
            port,
            client_drain_timeout_secs,
        })
    }
}
