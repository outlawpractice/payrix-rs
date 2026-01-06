//! Shared test infrastructure for integration tests.
//!
//! This module provides common utilities, constants, and helper functions
//! used across all integration test files.
//!
//! ## Environment Variables
//!
//! Test data can be configured via environment variables. If not set, defaults are used.
//!
//! | Variable | Description | Default |
//! |----------|-------------|---------|
//! | `TEST_PAYRIX_API_KEY` | API key (required) | None |
//! | `TEST_MERCHANT_ID` | Merchant for creating resources | `t1_mer_6941bf385591f9e279b1937` |
//! | `TEST_ENTITY_ID` | Business entity ID | `t1_ent_6941bf37e9b488e9ff0392a` |
//! | `TEST_OPEN_CHARGEBACK_ID` | Open chargeback for testing | `t1_chb_6616a9de06fd751e5ae91e5` |
//! | `TEST_CLOSED_CHARGEBACK_ID` | Closed chargeback for testing | `t1_chb_6616a9f7c19a47bea938957` |
//! | `TEST_WON_CHARGEBACK_ID` | Won chargeback for testing | `t1_chb_6615a4fbc5e0e79dac81419` |
//!
//! ## Fixture Loading
//!
//! For offline testing, use the `fixtures` module to load mock data:
//!
//! ```ignore
//! use common::fixtures::{load_fixture, load_single_fixture};
//!
//! let chargebacks: Vec<Chargeback> = load_fixture("chargebacks");
//! let chargeback: Chargeback = load_single_fixture("chargebacks");
//! ```

pub mod fixtures;

use payrix::{Customer, Environment, EntityType, Merchant, PayrixClient, Token};
use serde_json::json;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

// =============================================================================
// Default Test IDs (fallbacks when env vars not set)
// =============================================================================

const DEFAULT_ENTITY_ID: &str = "t1_ent_6941bf37e9b488e9ff0392a";
const DEFAULT_MERCHANT_ID: &str = "t1_mer_6941bf385591f9e279b1937";
const DEFAULT_OPERATING_ACCOUNT_ID: &str = "t1_act_6941bf3803046cefe558296";
const DEFAULT_TRUST_ACCOUNT_ID: &str = "t1_act_6941bf38481536ee5afca28";
const DEFAULT_OPEN_CHARGEBACK_ID: &str = "t1_chb_6616a9de06fd751e5ae91e5";
const DEFAULT_CLOSED_CHARGEBACK_ID: &str = "t1_chb_6616a9f7c19a47bea938957";
const DEFAULT_WON_CHARGEBACK_ID: &str = "t1_chb_6615a4fbc5e0e79dac81419";

// =============================================================================
// Test Data Accessors (environment variable with fallback)
// =============================================================================

/// Get the test entity ID. Checks `TEST_ENTITY_ID` env var first.
#[allow(dead_code)]
pub fn test_entity_id() -> String {
    env::var("TEST_ENTITY_ID").unwrap_or_else(|_| DEFAULT_ENTITY_ID.to_string())
}

/// Get the test merchant ID. Checks `TEST_MERCHANT_ID` env var first.
///
/// NOTE: This merchant may have status=NotReady. Once approved through
/// Payrix's underwriting process, it will have status=Boarded and can process transactions.
pub fn test_merchant_id() -> String {
    env::var("TEST_MERCHANT_ID").unwrap_or_else(|_| DEFAULT_MERCHANT_ID.to_string())
}

/// Get the test operating account ID. Checks `TEST_OPERATING_ACCOUNT_ID` env var first.
#[allow(dead_code)]
pub fn test_operating_account_id() -> String {
    env::var("TEST_OPERATING_ACCOUNT_ID").unwrap_or_else(|_| DEFAULT_OPERATING_ACCOUNT_ID.to_string())
}

/// Get the test trust account ID. Checks `TEST_TRUST_ACCOUNT_ID` env var first.
#[allow(dead_code)]
pub fn test_trust_account_id() -> String {
    env::var("TEST_TRUST_ACCOUNT_ID").unwrap_or_else(|_| DEFAULT_TRUST_ACCOUNT_ID.to_string())
}

/// Get an open chargeback ID for testing. Checks `TEST_OPEN_CHARGEBACK_ID` env var first.
///
/// Returns `Some(id)` if available (from env or default), `None` if explicitly disabled.
pub fn test_open_chargeback_id() -> Option<String> {
    match env::var("TEST_OPEN_CHARGEBACK_ID") {
        Ok(val) if val.is_empty() || val == "none" => None,
        Ok(val) => Some(val),
        Err(_) => Some(DEFAULT_OPEN_CHARGEBACK_ID.to_string()),
    }
}

/// Get a closed chargeback ID for testing. Checks `TEST_CLOSED_CHARGEBACK_ID` env var first.
pub fn test_closed_chargeback_id() -> Option<String> {
    match env::var("TEST_CLOSED_CHARGEBACK_ID") {
        Ok(val) if val.is_empty() || val == "none" => None,
        Ok(val) => Some(val),
        Err(_) => Some(DEFAULT_CLOSED_CHARGEBACK_ID.to_string()),
    }
}

/// Get a won chargeback ID for testing. Checks `TEST_WON_CHARGEBACK_ID` env var first.
#[allow(dead_code)]
pub fn test_won_chargeback_id() -> Option<String> {
    match env::var("TEST_WON_CHARGEBACK_ID") {
        Ok(val) if val.is_empty() || val == "none" => None,
        Ok(val) => Some(val),
        Err(_) => Some(DEFAULT_WON_CHARGEBACK_ID.to_string()),
    }
}

/// Require an open chargeback ID. Panics if not available.
///
/// Use this in tests that absolutely require a chargeback ID.
#[allow(dead_code)]
pub fn require_open_chargeback_id() -> String {
    test_open_chargeback_id()
        .expect("TEST_OPEN_CHARGEBACK_ID must be set (or use default) for this test")
}

/// Require a closed chargeback ID. Panics if not available.
#[allow(dead_code)]
pub fn require_closed_chargeback_id() -> String {
    test_closed_chargeback_id()
        .expect("TEST_CLOSED_CHARGEBACK_ID must be set (or use default) for this test")
}

// =============================================================================
// Legacy Constants (deprecated, use functions above)
// =============================================================================

/// DEPRECATED: Use `test_merchant_id()` instead.
#[deprecated(since = "0.2.0", note = "Use test_merchant_id() function instead")]
#[allow(dead_code)]
pub const TEST_MERCHANT_ID: &str = DEFAULT_MERCHANT_ID;

/// DEPRECATED: Use `test_open_chargeback_id()` instead.
#[deprecated(since = "0.2.0", note = "Use test_open_chargeback_id() function instead")]
#[allow(dead_code)]
pub const TEST_OPEN_CHARGEBACK_ID: &str = DEFAULT_OPEN_CHARGEBACK_ID;

/// DEPRECATED: Use `test_closed_chargeback_id()` instead.
#[deprecated(since = "0.2.0", note = "Use test_closed_chargeback_id() function instead")]
#[allow(dead_code)]
pub const TEST_CLOSED_CHARGEBACK_ID: &str = DEFAULT_CLOSED_CHARGEBACK_ID;

// =============================================================================
// Test Context
// =============================================================================

/// Test context that holds created resources for cleanup.
pub struct TestContext {
    pub client: PayrixClient,
    pub created_customers: Vec<String>,
    pub created_tokens: Vec<String>,
    pub created_transactions: Vec<String>,
    pub merchant_id: String,
}

impl TestContext {
    /// Create a new test context using an existing merchant.
    ///
    /// Note: Creating new merchants requires special permissions in Payrix.
    /// For testing, we use an existing merchant from the test account.
    pub async fn new() -> Result<Self, payrix::Error> {
        let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
        let client = PayrixClient::new(&api_key, Environment::Test)?;

        // Find an existing merchant to use for testing
        let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await?;
        let merchant = merchants
            .first()
            .expect("At least one merchant must exist in the test account");

        println!(
            "Using existing merchant: {} ({:?})",
            merchant.id.as_str(),
            merchant.dba
        );

        Ok(Self {
            client,
            created_customers: Vec::new(),
            created_tokens: Vec::new(),
            created_transactions: Vec::new(),
            merchant_id: merchant.id.as_str().to_string(),
        })
    }

    /// Track a created customer for cleanup.
    pub fn track_customer(&mut self, id: &str) {
        self.created_customers.push(id.to_string());
    }

    /// Track a created token for cleanup.
    pub fn track_token(&mut self, id: &str) {
        self.created_tokens.push(id.to_string());
    }

    /// Track a created transaction for cleanup.
    pub fn track_transaction(&mut self, id: &str) {
        self.created_transactions.push(id.to_string());
    }

    /// Clean up all created resources by setting them to inactive.
    pub async fn cleanup(&self) {
        println!("Cleaning up test resources...");

        // Deactivate transactions (can't actually deactivate, but we track them)
        for id in &self.created_transactions {
            println!("  Transaction: {} (tracked)", id);
        }

        // Deactivate tokens
        for id in &self.created_tokens {
            match self
                .client
                .update::<_, Token>(EntityType::Tokens, id, &json!({"inactive": 1}))
                .await
            {
                Ok(_) => println!("  Deactivated token: {}", id),
                Err(e) => println!("  Failed to deactivate token {}: {}", id, e),
            }
        }

        // Deactivate customers
        for id in &self.created_customers {
            match self
                .client
                .update::<_, Customer>(EntityType::Customers, id, &json!({"inactive": 1}))
                .await
            {
                Ok(_) => println!("  Deactivated customer: {}", id),
                Err(e) => println!("  Failed to deactivate customer {}: {}", id, e),
            }
        }

        // Note: We don't deactivate the merchant since we're using an existing one
        println!("Cleanup complete.");
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

pub fn init_logging() {
    INIT.call_once(|| {
        // Set up basic logging if needed
        if env::var("RUST_LOG").is_err() {
            // SAFETY: This runs exactly once during test initialization via call_once,
            // ensuring single-threaded access to the environment.
            unsafe {
                env::set_var("RUST_LOG", "payrix=debug");
            }
        }
    });
}

/// Create a PayrixClient from environment variable
pub fn create_client() -> PayrixClient {
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    PayrixClient::new(&api_key, Environment::Test).unwrap()
}
