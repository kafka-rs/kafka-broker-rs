//! # Kafka Broker (Rust Implementation)
//!
//! This file represents the starting point of a simple Kafka broker in Rust.
//! It initializes logging, loads configuration, starts a TCP listener to accept incoming connections,
//! and supports graceful shutdown via Ctrl+C (SIGINT) with a draining phase for active connections.

use tokio::net::TcpListener;
use tokio::{select, signal, task::JoinSet, time};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn, Instrument};
use tracing_subscriber;

mod broker_state;
mod client_handler;
mod config;
mod kafka_protocol;

use crate::broker_state::{BrokerState, SharedBrokerState};
use crate::config::Config;

/// Sets up tracing/logging by reading the `RUST_LOG` environment variable or using
/// default levels if `RUST_LOG` isn't set.
fn setup() -> anyhow::Result<()> {
    // Initialize logging with optional filtering via RUST_LOG.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Kafka broker...");
    Ok(())
}

/// Accepts incoming TCP connections in a loop, spawning a new `handle_client`
/// task for each connection. This function returns when the `shutdown_token` is triggered.
///
/// # Parameters
///
/// - `config`: The broker configuration (host, port, drain time).
/// - `broker_state`: Shared state (e.g., topics, offsets).
/// - `shutdown_token`: A cancellation token for graceful shutdown.
/// - `join_set`: A `JoinSet` that tracks spawned client tasks so we can wait on them later.
///
/// # Errors
///
/// Returns an `anyhow::Error` if binding the TCP listener fails (e.g., port already in use).
async fn accept_loop(
    config: &Config,
    broker_state: SharedBrokerState,
    shutdown_token: CancellationToken,
    join_set: &mut JoinSet<anyhow::Result<()>>,
) -> anyhow::Result<()> {
    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).await?;
    info!("Broker listening on {address}");

    loop {
        select! {
            result = listener.accept() => {
                match result {
                    Ok((socket, addr)) => {
                        info!("Accepted new connection from {}", addr);
                        let span = tracing::info_span!("client_session", client_addr = %addr);
                        let state_clone = broker_state.clone();

                        // Spawn the new function from client_handler
                        join_set.spawn(
                            client_handler::handle_client(socket, state_clone).instrument(span)
                        );
                    },
                    Err(e) => {
                        error!("Failed to accept new client connection: {}", e);
                        continue;
                    }
                }
            },
            _ = shutdown_token.cancelled() => {
                warn!("Graceful shutdown requested; stopping accept loop.");
                break;
            }
        }
    }

    Ok(())
}

/// Drains any remaining client tasks by awaiting them with a timeout.
///
/// If the tasks finish before the timeout, we log success. Otherwise,
/// we log a warning indicating that we timed out.
///
/// # Parameters
///
/// - `join_set`: The set of all spawned client tasks.
/// - `timeout_secs`: The maximum time (in seconds) to wait for tasks to finish.
async fn drain_tasks(join_set: &mut JoinSet<anyhow::Result<()>>, timeout_secs: u64) {
    info!(
        "Draining client tasks with a {} second timeout...",
        timeout_secs
    );
    let timeout_duration = time::Duration::from_secs(timeout_secs);

    // We'll attempt to join all tasks within the timeout.
    let drain_result = time::timeout(timeout_duration, async {
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(Ok(())) => {
                    debug!("A client task exited cleanly.");
                }
                Ok(Err(e)) => {
                    error!("A client task returned an error: {:?}", e);
                }
                Err(join_err) => {
                    // This means the task itself panicked or was cancelled.
                    error!("A client task panicked or was cancelled: {:?}", join_err);
                }
            }
        }
    })
    .await;

    match drain_result {
        Ok(_) => {
            info!("All client tasks have exited gracefully.");
        }
        Err(_) => {
            warn!(
                "Timed out while waiting for client tasks to finish ({}s). Shutting down now.",
                timeout_secs
            );
        }
    }
}

/// Runs the main server loop in parallel with a shutdown listener (Ctrl+C).
///
/// When Ctrl+C is pressed, we'll trigger the `CancellationToken` which in turn
/// stops `accept_loop` gracefully. Then we attempt to drain existing tasks for
/// up to `client_drain_timeout_secs`.
async fn run_server(config: Config) -> anyhow::Result<()> {
    // Create the shared broker state (topics, metadata, etc.).
    let broker_state = BrokerState::new();
    let broker_state_arc = SharedBrokerState::from(broker_state);

    // Create a cancellation token for graceful shutdown.
    let shutdown_token = CancellationToken::new();

    // We'll spawn a task that listens for Ctrl+C signals to trigger this token.
    let shutdown_token_clone = shutdown_token.clone();
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            error!("Failed to listen for Ctrl+C: {}", e);
        }
        info!("SIGINT (Ctrl+C) received, triggering shutdown...");
        shutdown_token_clone.cancel();
    });

    // This JoinSet will track all spawned client tasks.
    let mut join_set = JoinSet::new();

    // Accept connections until the cancellation token fires.
    accept_loop(&config, broker_state_arc, shutdown_token, &mut join_set).await?;

    // After the accept loop ends, give client tasks a chance to finish.
    drain_tasks(&mut join_set, config.client_drain_timeout_secs).await;

    info!("Server has shut down gracefully.");
    Ok(())
}

/// The main entry point for the Kafka broker.
///
/// 1) Sets up tracing/logging.
/// 2) Loads configuration from environment.
/// 3) Runs the server loop with graceful shutdown, including client-task draining.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup()?;

    let config = Config::from_env()?;

    run_server(config).await
}
