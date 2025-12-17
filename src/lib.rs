//! # payrix
//!
//! A Rust client library for the [Payrix](https://www.payrix.com/) payment processing API.
//!
//! ## Features
//!
//! - **Full async/await support** with Tokio
//! - **Built-in rate limiting** to avoid API throttling
//! - **Automatic retry** with exponential backoff for transient failures
//! - **Strongly typed** API responses with 68 enums and 26 resource types
//! - **Comprehensive error handling** with domain-specific error types
//! - **Optional SQLx support** for database integration
//!
//! ## Quick Start
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment, EntityType, Customer, NewCustomer};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), payrix::Error> {
//!     // Create a client
//!     let client = PayrixClient::new("your-api-key", Environment::Test)?;
//!
//!     // Get a customer by ID
//!     let customer: Option<Customer> = client.get_one(
//!         EntityType::Customers,
//!         "t1_cus_12345678901234567890123"
//!     ).await?;
//!
//!     // Create a new customer
//!     let new_customer: Customer = client.create(
//!         EntityType::Customers,
//!         &NewCustomer {
//!             merchant: "t1_mer_12345678901234567890123".to_string(),
//!             first: Some("John".to_string()),
//!             last: Some("Doe".to_string()),
//!             ..Default::default()
//!         }
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Searching
//!
//! Use [`SearchBuilder`] for complex queries:
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment, EntityType, Token, SearchBuilder};
//!
//! # async fn example() -> Result<(), payrix::Error> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! let search = SearchBuilder::new()
//!     .field("customer", "t1_cus_12345678901234567890123")
//!     .build();
//!
//! let tokens: Vec<Token> = client.search(EntityType::Tokens, &search).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Environment Configuration
//!
//! The client supports test and production environments:
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//!
//! // Test environment (sandbox)
//! let client = PayrixClient::new("api-key", Environment::Test).unwrap();
//!
//! // Production environment
//! let client = PayrixClient::new("api-key", Environment::Production).unwrap();
//! ```
//!
//! | Environment | Base URL |
//! |-------------|----------|
//! | Test | `https://test-api.payrix.com/` |
//! | Production | `https://api.payrix.com/` |
//!
//! ## Error Handling
//!
//! The library provides domain-specific error types:
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment, EntityType, Customer, Error};
//!
//! # async fn example() -> Result<(), Error> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! match client.get_one::<Customer>(EntityType::Customers, "id").await {
//!     Ok(Some(customer)) => println!("Found: {:?}", customer),
//!     Ok(None) => println!("Not found"),
//!     Err(Error::Unauthorized(_)) => println!("Invalid API key"),
//!     Err(Error::RateLimited(_)) => println!("Rate limited"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! - `sqlx` - Enable `sqlx::FromRow` derives for database storage

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod client;
pub mod entity;
mod error;
mod rate_limiter;
pub mod search;
pub mod types;
pub mod workflows;

pub use client::{Config, Environment, PayrixClient};
pub use entity::EntityType;
pub use error::{Error, PayrixApiError, Result};
pub use search::{make_payrix_date, make_search_field, parse_payrix_date, SearchBuilder, SearchOperator};
pub use types::*;

// Re-export for convenience
pub use types::{
    Customer, Entity, Merchant, NewCustomer, NewToken, NewTransaction, Token, Transaction,
};

// Re-export workflow types for convenience
pub use workflows::merchant_onboarding::{
    check_boarding_status, onboard_merchant, Address, BankAccountInfo, BankAccountMethod,
    BoardingStatus, BoardingStatusResult, BusinessInfo, MemberInfo, MerchantConfig,
    OnboardMerchantRequest, OnboardMerchantResult, TermsAcceptance,
};

// Re-export dispute handling types for convenience
pub use workflows::dispute_handling::{
    ActiveDispute, ChargebackDispute, ChargebackState, Evidence, EvidenceDocument,
    TypedChargeback, get_actionable_disputes,
};
