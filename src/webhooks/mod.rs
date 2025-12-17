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
//!                           │ (typestate)│              │            │    │           │
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
//!
//! # Integration with Dispute Handling
//!
//! The webhook server integrates seamlessly with the
//! [`dispute_handling`](crate::workflows::dispute_handling) workflow to provide
//! automated chargeback response handling:
//!
//! ```no_run
//! use payrix::webhooks::{WebhookServer, WebhookServerConfig, ChargebackEvent};
//! use payrix::workflows::dispute_handling::{ChargebackDispute, ActiveDispute};
//! use payrix::{PayrixClient, Environment};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Start webhook server
//! let (server, mut events) = WebhookServer::new();
//! tokio::spawn(server.run("0.0.0.0:13847".parse()?));
//!
//! // Handle chargeback events
//! while let Some(event) = events.recv().await {
//!     if let Some(cb_event) = event.as_chargeback_event() {
//!         match cb_event {
//!             ChargebackEvent::Created { data, .. } => {
//!                 // Convert to typed dispute for compile-time state checking
//!                 let dispute = ChargebackDispute::from_chargeback(data);
//!
//!                 match dispute {
//!                     ChargebackDispute::Active(ActiveDispute::First(first)) => {
//!                         // Can only call represent() or accept_liability() here
//!                         println!("New chargeback: {}", first.id());
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!             ChargebackEvent::Won { chargeback_id, .. } => {
//!                 println!("Won dispute: {}", chargeback_id);
//!             }
//!             ChargebackEvent::Lost { chargeback_id, .. } => {
//!                 println!("Lost dispute: {}", chargeback_id);
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! See the `examples/webhook_dispute_handler.rs` example for a complete
//! implementation with decision logic.

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
