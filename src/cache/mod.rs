//! Local entity cache for Payrix data.
//!
//! This module provides a local database cache that mirrors Payrix entities,
//! reducing API calls and enabling faster queries.
//!
//! # Feature Gate
//!
//! This module requires the `cache` feature to be enabled:
//!
//! ```toml
//! [dependencies]
//! payrix = { version = "0.1", features = ["cache"] }
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐     webhook      ┌──────────────────┐
//! │   Payrix API    │ ───────────────> │  WebhookServer   │
//! └────────┬────────┘                  └────────┬─────────┘
//!          │                                    │
//!          │                                    ▼
//!          │                           ┌────────────────────┐
//!          │                           │   EntityCache      │
//!          │                           │   (PostgreSQL)     │
//!          │                           └────────┬───────────┘
//!          │                                    │
//!          │                                    ▼
//!          │                           ┌────────────────────┐
//!          │   initial sync            │   Your App         │
//!          └──────────────────────────>│   (queries cache)  │
//!                                      └────────────────────┘
//! ```
//!
//! # Benefits
//!
//! - **Reduce API calls** - Query local database instead of Payrix API
//! - **Faster queries** - Local database is much faster than API calls
//! - **Complex queries** - SQL JOINs, aggregations not possible via API
//! - **Offline resilience** - App continues working if Payrix is temporarily unavailable
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::cache::{EntityCache, CacheConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Connect to cache database
//! let cache = EntityCache::new(
//!     "postgres://user:pass@localhost/payrix_cache",
//!     client,
//! ).await?;
//!
//! // Initial sync from Payrix API
//! let stats = cache.initial_sync().await?;
//! println!("Synced {} chargebacks", stats.chargebacks);
//!
//! // Query from local cache (fast!)
//! let chargeback = cache.get_chargeback("t1_chb_123").await?;
//!
//! // Or fall back to API if not in cache
//! let chargeback = cache.get_or_fetch_chargeback("t1_chb_456").await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Webhook Integration
//!
//! The cache can be kept in sync via webhooks:
//!
//! ```no_run
//! use payrix::cache::EntityCache;
//! use payrix::webhooks::{WebhookServer, WebhookEvent};
//!
//! # async fn example(cache: EntityCache, mut events: tokio::sync::mpsc::Receiver<WebhookEvent>) {
//! while let Some(event) = events.recv().await {
//!     // Update cache from webhook event
//!     if let Err(e) = cache.process_webhook(&event).await {
//!         tracing::error!("Failed to update cache: {}", e);
//!     }
//! }
//! # }
//! ```
//!
//! # PCI DSS Considerations
//!
//! The cache stores tokenized data only - no raw card numbers:
//!
//! - **Safe to cache**: Transactions, Chargebacks, Merchants, Customers, Tokens
//! - **Payrix handles PCI**: Card numbers are tokenized before storage
//! - **No raw PAN data**: Webhooks contain the same tokenized data

mod entity_cache;
mod schema;
mod sync;

pub use entity_cache::{CacheConfig, EntityCache, SyncStats};
pub use schema::ensure_schema;
