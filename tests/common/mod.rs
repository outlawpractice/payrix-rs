//! Shared test infrastructure for integration tests.
//!
//! This module provides common utilities, constants, and helper functions
//! used across all integration test files.

use payrix::{Customer, Environment, EntityType, Merchant, PayrixClient, Token};
use serde_json::json;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

// =============================================================================
// Test Merchant Constants
// =============================================================================
// These IDs are from a merchant created via test_merchant_onboarding_with_trust_account
// Use these for tests that create customers, tokens, transactions, etc.

/// Test entity ID (the business entity)
#[allow(dead_code)]
pub const TEST_ENTITY_ID: &str = "t1_ent_6941bf37e9b488e9ff0392a";

/// Test merchant ID (for creating customers, tokens)
/// NOTE: This merchant currently has status=NotReady. Once it is approved through
/// Payrix's underwriting process, it will have status=Boarded and can process transactions.
pub const TEST_MERCHANT_ID: &str = "t1_mer_6941bf385591f9e279b1937";

/// Test operating account ID (type=All, primary=true)
#[allow(dead_code)]
pub const TEST_OPERATING_ACCOUNT_ID: &str = "t1_act_6941bf3803046cefe558296";

/// Test trust account ID (type=Credit, primary=false)
#[allow(dead_code)]
pub const TEST_TRUST_ACCOUNT_ID: &str = "t1_act_6941bf38481536ee5afca28";

// =============================================================================
// Chargeback Test Constants
// =============================================================================

/// Test constant: An open chargeback for read-only testing.
/// Update this ID if the chargeback status changes or a new open chargeback is available.
pub const TEST_OPEN_CHARGEBACK_ID: &str = "t1_chb_6616a9de06fd751e5ae91e5";

/// Test constant: A closed chargeback for read-only testing.
pub const TEST_CLOSED_CHARGEBACK_ID: &str = "t1_chb_6616a9f7c19a47bea938957";

/// Test constant: A won chargeback for read-only testing.
#[allow(dead_code)]
pub const TEST_WON_CHARGEBACK_ID: &str = "t1_chb_6615a4fbc5e0e79dac81419";

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
        let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
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
            env::set_var("RUST_LOG", "payrix=debug");
        }
    });
}

/// Create a PayrixClient from environment variable
pub fn create_client() -> PayrixClient {
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    PayrixClient::new(&api_key, Environment::Test).unwrap()
}
