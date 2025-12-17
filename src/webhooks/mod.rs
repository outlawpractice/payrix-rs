//! Webhook server and event handling for Payrix.
//!
//! This module provides an HTTP server for receiving Payrix webhook callbacks,
//! along with event types, logging, and channel-based event distribution.
//!
//! # Feature Gate
//!
//! This module requires the `webhooks` feature to be enabled:
//!
//! ```toml
//! [dependencies]
//! payrix = { version = "0.1", features = ["webhooks"] }
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐        ┌─────────────────┐        ┌──────────────────┐
//! │  Payrix API     │───────>│  WebhookServer  │───────>│  Event Channels  │
//! │  (sends POST)   │        │  (axum)         │        │  (tokio mpsc)    │
//! └─────────────────┘        └─────────────────┘        └────────┬─────────┘
//!                                                                │
//!                                 ┌──────────────────────────────┴─────────────┐
//!                                 │                              │             │
//!                           ┌─────▼─────┐              ┌─────────▼──┐    ┌─────▼─────┐
//!                           │ Chargeback │              │Transaction │    │  Other    │
//!                           │ Handler    │              │ Handler    │    │ Handlers  │
//!                           └───────────┘              └────────────┘    └───────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use payrix::webhooks::{WebhookServer, WebhookServerConfig};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure the webhook server
//! let config = WebhookServerConfig::new()
//!     .with_auth_header("X-Webhook-Secret", "my-secret-value")
//!     .with_stdout_logging(true);
//!
//! // Create the server and get the event receiver
//! let (server, mut events) = WebhookServer::with_config(config);
//!
//! // Spawn a task to handle incoming events
//! tokio::spawn(async move {
//!     while let Some(event) = events.recv().await {
//!         println!("Received: {} {}", event.event_type, event.resource_id);
//!
//!         // Handle chargeback events
//!         if let Some(chargeback_event) = event.as_chargeback_event() {
//!             println!("Chargeback: {}", chargeback_event.chargeback_id());
//!         }
//!     }
//! });
//!
//! // Run the server
//! let addr: SocketAddr = "0.0.0.0:13847".parse()?;
//! server.run(addr).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Security
//!
//! The webhook server supports two security mechanisms:
//!
//! 1. **IP Allowlist** - Only accept requests from specified IP ranges
//! 2. **Header Authentication** - Require a specific header with a secret value
//!
//! ```no_run
//! use payrix::webhooks::WebhookServerConfig;
//!
//! let config = WebhookServerConfig::new()
//!     // Only accept from specific IP ranges
//!     .with_allowed_ips(vec!["10.0.0.0/8".parse().unwrap()])
//!     // Require a secret header
//!     .with_auth_header("X-Webhook-Secret", "my-secret");
//! ```
//!
//! # Logging
//!
//! The server can log webhook events to various backends:
//!
//! - [`InMemoryWebhookLogger`] - For testing
//! - [`StdoutWebhookLogger`] - For development/debugging
//! - Custom implementations via [`WebhookLogger`] trait
//!
//! ```no_run
//! use payrix::webhooks::{WebhookServerConfig, InMemoryWebhookLogger};
//! use std::sync::Arc;
//!
//! let logger = Arc::new(InMemoryWebhookLogger::new());
//! let config = WebhookServerConfig::new()
//!     .with_logger(logger);
//! ```

pub mod events;
pub mod logging;
pub mod server;

// Re-export main types
pub use events::{ChargebackEvent, WebhookEvent, WebhookEventType};
pub use logging::{
    InMemoryWebhookLogger, ProcessingStatus, StdoutWebhookLogger, WebhookLogEntry, WebhookLogFilter,
    WebhookLogger,
};
pub use server::{WebhookServer, WebhookServerConfig};
