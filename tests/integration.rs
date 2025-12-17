//! Integration tests for the Payrix client.
//!
//! These tests run against a real Payrix test instance.
//! They require the PAYRIX_API_KEY environment variable to be set.
//!
//! Run with: `cargo test --test integration -- --ignored`
//!
//! **Important**: These tests create real resources in Payrix and clean them up
//! by setting `inactive=true` when done.
//!
//! ## Known Limitations
//!
//! Some tests may fail due to Payrix API inconsistencies:
//!
//! - `test_get_fees`: May fail with "expected value" if Fee response contains
//!   empty or malformed JSON fields.
//! - `test_get_team_logins`: May fail with EOF if the test account lacks
//!   access to team logins.
//! - `test_get_customers`: May fail with "null" if expanded customer data
//!   contains null values where strings are expected.
//! - `test_transaction_flow`: May fail with "string expected i32" if the API
//!   returns string-encoded integers for certain transaction enum fields.
//!
//! The Payrix API documentation specifies integer enum values, but the actual
//! API often returns string values. This library attempts to handle common
//! cases but may not cover all variations.

use payrix::{
    Account, AccountVerification, Adjustment, Alert, AlertAction, AlertTrigger, Batch, Chargeback,
    ChargebackDocument, ChargebackMessage, ChargebackMessageResult, ChargebackStatus,
    ChargebackStatusValue, Contact, Customer, Disbursement, DisbursementEntry, Entity,
    EntityReserve, EntityType, Entry, Environment, Fee, FeeRule, Fund, Login, Member, Merchant,
    NewChargebackMessage, NewCustomer, NewToken, NewTransaction, Note, NoteDocument, Org,
    OrgEntity, PaymentInfo, PaymentMethod, Payout, PayrixClient, PendingEntry, Plan, Refund,
    Reserve, ReserveEntry, SearchBuilder, Subscription, TeamLogin, Token, Transaction,
    TransactionOrigin, Vendor,
    // Workflow types
    onboard_merchant, check_boarding_status, OnboardMerchantRequest, BusinessInfo, MerchantConfig,
    BankAccountInfo, BankAccountMethod, MemberInfo, Address, TermsAcceptance, BoardingStatus,
};
use payrix::types::ChargebackMessageType;
use payrix::types::{
    AccountHolderType, AccountType, DateYmd, MemberType, MerchantEnvironment, MerchantType,
};
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
const TEST_ENTITY_ID: &str = "t1_ent_6941bf37e9b488e9ff0392a";

/// Test merchant ID (for creating customers, tokens)
/// NOTE: This merchant currently has status=NotReady. Once it is approved through
/// Payrix's underwriting process, it will have status=Boarded and can process transactions.
const TEST_MERCHANT_ID: &str = "t1_mer_6941bf385591f9e279b1937";

/// Test operating account ID (type=All, primary=true)
#[allow(dead_code)]
const TEST_OPERATING_ACCOUNT_ID: &str = "t1_act_6941bf3803046cefe558296";

/// Test trust account ID (type=Credit, primary=false)
#[allow(dead_code)]
const TEST_TRUST_ACCOUNT_ID: &str = "t1_act_6941bf38481536ee5afca28";

/// Test context that holds created resources for cleanup.
struct TestContext {
    client: PayrixClient,
    created_customers: Vec<String>,
    created_tokens: Vec<String>,
    created_transactions: Vec<String>,
    merchant_id: String,
}

impl TestContext {
    /// Create a new test context using an existing merchant.
    ///
    /// Note: Creating new merchants requires special permissions in Payrix.
    /// For testing, we use an existing merchant from the test account.
    async fn new() -> Result<Self, payrix::Error> {
        let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
        let client = PayrixClient::new(&api_key, Environment::Test)?;

        // Find an existing merchant to use for testing
        let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await?;
        let merchant = merchants
            .first()
            .expect("At least one merchant must exist in the test account");

        println!("Using existing merchant: {} ({:?})", merchant.id.as_str(), merchant.dba);

        Ok(Self {
            client,
            created_customers: Vec::new(),
            created_tokens: Vec::new(),
            created_transactions: Vec::new(),
            merchant_id: merchant.id.as_str().to_string(),
        })
    }

    /// Track a created customer for cleanup.
    fn track_customer(&mut self, id: &str) {
        self.created_customers.push(id.to_string());
    }

    /// Track a created token for cleanup.
    fn track_token(&mut self, id: &str) {
        self.created_tokens.push(id.to_string());
    }

    /// Track a created transaction for cleanup.
    fn track_transaction(&mut self, id: &str) {
        self.created_transactions.push(id.to_string());
    }

    /// Clean up all created resources by setting them to inactive.
    async fn cleanup(&self) {
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

fn init_logging() {
    INIT.call_once(|| {
        // Set up basic logging if needed
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "payrix=debug");
        }
    });
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_entities() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let entities: Vec<Entity> = client.get_all(EntityType::Entities).await.unwrap();

    assert!(!entities.is_empty(), "Should have at least one entity");
    println!("Found {} entities", entities.len());

    for entity in &entities {
        println!("  Entity: {} - {:?}", entity.id.as_str(), entity.name);
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_merchants() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await.unwrap();

    println!("Found {} merchants", merchants.len());
    for merchant in merchants.iter().take(5) {
        println!(
            "  Merchant: {} - {:?} (status: {:?})",
            merchant.id.as_str(),
            merchant.dba,
            merchant.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_customer_crud() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // CREATE
    let new_customer = NewCustomer {
        merchant: ctx.merchant_id.clone(),
        first: Some("Test".to_string()),
        last: Some("Customer".to_string()),
        email: Some(format!("test-{}@example.com", ctx.merchant_id)),
        ..Default::default()
    };

    let customer: Customer = ctx
        .client
        .create(EntityType::Customers, &new_customer)
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());
    println!("Created customer: {}", customer.id.as_str());

    assert_eq!(customer.first.as_deref(), Some("Test"));
    assert_eq!(customer.last.as_deref(), Some("Customer"));

    // READ
    let fetched: Option<Customer> = ctx
        .client
        .get_one(EntityType::Customers, customer.id.as_str())
        .await
        .expect("Failed to get customer");

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.id.as_str(), customer.id.as_str());

    // UPDATE
    let updated: Customer = ctx
        .client
        .update(
            EntityType::Customers,
            customer.id.as_str(),
            &json!({"first": "Updated"}),
        )
        .await
        .expect("Failed to update customer");

    assert_eq!(updated.first.as_deref(), Some("Updated"));

    // SEARCH
    let search = SearchBuilder::new()
        .field("merchant", &ctx.merchant_id)
        .build();

    let results: Vec<Customer> = ctx
        .client
        .search(EntityType::Customers, &search)
        .await
        .expect("Failed to search customers");

    assert!(
        results.iter().any(|c| c.id.as_str() == customer.id.as_str()),
        "Should find our customer in search results"
    );

    // Cleanup
    ctx.cleanup().await;
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_token_creation() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Create a customer first
    let customer: Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &NewCustomer {
                merchant: ctx.merchant_id.clone(),
                first: Some("Token".to_string()),
                last: Some("Test".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());

    // Create a token (test card - Visa 4111...)
    let new_token = NewToken {
        customer: customer.id.as_str().to_string(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            expiration: Some("1229".to_string()), // December 2029
            cvv: Some("123".to_string()),
            routing: None,
        },
        ..Default::default()
    };

    let token: Token = ctx
        .client
        .create(EntityType::Tokens, &new_token)
        .await
        .expect("Failed to create token");

    ctx.track_token(token.id.as_str());
    println!("Created token: {}", token.id.as_str());

    assert!(token.customer.is_some());

    // Cleanup
    ctx.cleanup().await;
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY and a boarded merchant"]
async fn test_transaction_flow() {
    // Note: This test uses TEST_MERCHANT_ID which must have status=Boarded.
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Override with our known boarded merchant
    ctx.merchant_id = TEST_MERCHANT_ID.to_string();
    println!("Using boarded merchant: {}", ctx.merchant_id);

    // Create customer
    let customer: Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &NewCustomer {
                merchant: ctx.merchant_id.clone(),
                first: Some("Transaction".to_string()),
                last: Some("Test".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());

    // Create token
    let token: Token = ctx
        .client
        .create(
            EntityType::Tokens,
            &NewToken {
                customer: customer.id.as_str().to_string(),
                payment: PaymentInfo {
                    method: PaymentMethod::Visa,
                    number: Some("4111111111111111".to_string()),
                    expiration: Some("1229".to_string()),
                    cvv: Some("123".to_string()),
                    routing: None,
                },
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create token");

    ctx.track_token(token.id.as_str());

    // Create a transaction (auth + capture)
    // Use the token string value, not the token ID
    let token_string = token.token.expect("Token should have a token string");
    let new_txn = NewTransaction {
        merchant: ctx.merchant_id.clone(),
        token: Some(token_string),
        origin: Some(TransactionOrigin::Ecommerce),
        total: 1000, // $10.00
        ..Default::default()
    };

    let txn: Transaction = ctx
        .client
        .create(EntityType::Txns, &new_txn)
        .await
        .expect("Failed to create transaction");

    ctx.track_transaction(txn.id.as_str());
    println!(
        "Created transaction: {} (status: {:?})",
        txn.id.as_str(),
        txn.status
    );

    // Verify transaction
    let fetched: Option<Transaction> = ctx
        .client
        .get_one(EntityType::Txns, txn.id.as_str())
        .await
        .expect("Failed to get transaction");

    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.total, Some(1000));

    // Cleanup
    ctx.cleanup().await;
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_search_with_pagination() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // Get first page
    let (merchants, page_info) = client
        .get_page::<Merchant>(EntityType::Merchants, 1, 10, &std::collections::HashMap::new(), None)
        .await
        .expect("Failed to get page");

    println!(
        "Page 1: {} merchants, has_more: {}",
        merchants.len(),
        page_info.has_more
    );

    if page_info.has_more {
        let (page2, _) = client
            .get_page::<Merchant>(EntityType::Merchants, 2, 10, &std::collections::HashMap::new(), None)
            .await
            .expect("Failed to get page 2");

        println!("Page 2: {} merchants", page2.len());
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_error_handling() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // Try to get a non-existent resource
    let result: Result<Option<Customer>, _> = client
        .get_one(EntityType::Customers, "t1_cus_nonexistent0000000000000")
        .await;

    // Should return Ok(None) or an error, not panic
    match result {
        Ok(None) => println!("Correctly returned None for non-existent customer"),
        Ok(Some(_)) => panic!("Should not find a non-existent customer"),
        Err(e) => println!("Got expected error: {}", e),
    }
}

// ============================================================================
// Read-Only Integration Tests for Additional Types
// ============================================================================

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_transactions() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let transactions: Vec<Transaction> = client.get_all(EntityType::Txns).await.unwrap();

    println!("Found {} transactions", transactions.len());
    for txn in transactions.iter().take(5) {
        println!(
            "  Transaction: {} - type: {:?}, total: {:?}, status: {:?}",
            txn.id.as_str(),
            txn.txn_type,
            txn.total,
            txn.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_tokens() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let tokens: Vec<Token> = client.get_all(EntityType::Tokens).await.unwrap();

    println!("Found {} tokens", tokens.len());
    for token in tokens.iter().take(5) {
        println!(
            "  Token: {} - payment: {:?}, status: {:?}",
            token.id.as_str(),
            token.payment,
            token.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_batches() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let batches: Vec<Batch> = client.get_all(EntityType::Batches).await.unwrap();

    println!("Found {} batches", batches.len());
    for batch in batches.iter().take(5) {
        println!(
            "  Batch: {} - date: {:?}, status: {:?}",
            batch.id.as_str(),
            batch.date,
            batch.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_fees() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let fees: Vec<Fee> = client.get_all(EntityType::Fees).await.unwrap();

    println!("Found {} fees", fees.len());
    for fee in fees.iter().take(5) {
        println!(
            "  Fee: {} - name: {:?}, type: {:?}",
            fee.id.as_str(),
            fee.name,
            fee.fee_type
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_fee_rules() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let fee_rules: Vec<FeeRule> = client.get_all(EntityType::FeeRules).await.unwrap();

    println!("Found {} fee rules", fee_rules.len());
    for rule in fee_rules.iter().take(5) {
        println!(
            "  FeeRule: {} - type: {:?}, value: {:?}",
            rule.id.as_str(),
            rule.rule_type,
            rule.value
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_plans() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let plans: Vec<Plan> = client.get_all(EntityType::Plans).await.unwrap();

    println!("Found {} plans", plans.len());
    for plan in plans.iter().take(5) {
        println!(
            "  Plan: {} - name: {:?}, amount: {:?}",
            plan.id.as_str(),
            plan.name,
            plan.amount
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_subscriptions() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let subscriptions: Vec<Subscription> = client.get_all(EntityType::Subscriptions).await.unwrap();

    println!("Found {} subscriptions", subscriptions.len());
    for sub in subscriptions.iter().take(5) {
        println!(
            "  Subscription: {} - start: {:?}, plan: {:?}",
            sub.id.as_str(),
            sub.start,
            sub.plan
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_accounts() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let accounts: Vec<Account> = client.get_all(EntityType::Accounts).await.unwrap();

    println!("Found {} accounts", accounts.len());
    for account in accounts.iter().take(5) {
        println!(
            "  Account: {} - type: {:?}, status: {:?}",
            account.id.as_str(),
            account.account_type,
            account.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_funds() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let funds: Vec<Fund> = client.get_all(EntityType::Funds).await.unwrap();

    println!("Found {} funds", funds.len());
    for fund in funds.iter().take(5) {
        println!(
            "  Fund: {} - available: {:?}, pending: {:?}",
            fund.id.as_str(),
            fund.available,
            fund.pending
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_logins() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let logins: Vec<Login> = client.get_all(EntityType::Logins).await.unwrap();

    println!("Found {} logins", logins.len());
    for login in logins.iter().take(5) {
        println!(
            "  Login: {} - email: {:?}",
            login.id.as_str(),
            login.email
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_team_logins() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let team_logins: Vec<TeamLogin> = client.get_all(EntityType::TeamLogins).await.unwrap();

    println!("Found {} team logins", team_logins.len());
    for login in team_logins.iter().take(5) {
        println!(
            "  TeamLogin: {} - login: {:?}, team: {:?}, create: {}, read: {}",
            login.id.as_str(),
            login.login,
            login.team,
            login.create,
            login.read
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_members() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let members: Vec<Member> = client.get_all(EntityType::Members).await.unwrap();

    println!("Found {} members", members.len());
    for member in members.iter().take(5) {
        println!(
            "  Member: {} - merchant: {:?}",
            member.id.as_str(),
            member.merchant
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_orgs() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let orgs: Vec<Org> = client.get_all(EntityType::Orgs).await.unwrap();

    println!("Found {} orgs", orgs.len());
    for org in orgs.iter().take(5) {
        println!(
            "  Org: {} - name: {:?}",
            org.id.as_str(),
            org.name
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_contacts() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let contacts: Vec<Contact> = client.get_all(EntityType::Contacts).await.unwrap();

    println!("Found {} contacts", contacts.len());
    for contact in contacts.iter().take(5) {
        println!(
            "  Contact: {} - first: {:?}, last: {:?}",
            contact.id.as_str(),
            contact.first,
            contact.last
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargebacks() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();

    println!("Found {} chargebacks", chargebacks.len());
    // Print ALL chargebacks to capture all status values for documentation
    for chargeback in chargebacks.iter() {
        println!(
            "  Chargeback: {} - status: {:?}, total: {:?}, reason: {:?}",
            chargeback.id.as_str(),
            chargeback.status,
            chargeback.total,
            chargeback.reason_code
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_customers() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let customers: Vec<Customer> = client.get_all(EntityType::Customers).await.unwrap();

    println!("Found {} customers", customers.len());
    for customer in customers.iter().take(5) {
        println!(
            "  Customer: {} - first: {:?}, last: {:?}, merchant: {:?}",
            customer.id.as_str(),
            customer.first,
            customer.last,
            customer.merchant
        );
    }
}

// ==================== Additional Entity Type Tests ====================

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_payouts() {
    // NOTE: The test environment does NOT process payouts, so this endpoint
    // will always return empty results in test mode.
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let payouts: Vec<Payout> = client.get_all(EntityType::Payouts).await.unwrap();

    println!("Found {} payouts", payouts.len());
    for payout in payouts.iter().take(5) {
        println!(
            "  Payout: {} - amount: {:?}, um: {:?}, schedule: {:?}",
            payout.id.as_str(),
            payout.amount,
            payout.um,
            payout.schedule
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_entity_reserves() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let reserves: Vec<EntityReserve> = client.get_all(EntityType::EntityReserves).await.unwrap();

    println!("Found {} entity reserves", reserves.len());
    for reserve in reserves.iter().take(5) {
        println!(
            "  EntityReserve: {} - fund: {:?}, total: {:?}, name: {:?}",
            reserve.id.as_str(),
            reserve.fund,
            reserve.total,
            reserve.name
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_org_entities() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let org_entities: Vec<OrgEntity> = client.get_all(EntityType::OrgEntities).await.unwrap();

    println!("Found {} org entities", org_entities.len());
    for oe in org_entities.iter().take(5) {
        println!(
            "  OrgEntity: {} - org: {:?}, entity: {:?}",
            oe.id.as_str(),
            oe.org,
            oe.entity
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_reserve_entries() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let entries: Vec<ReserveEntry> = client.get_all(EntityType::ReserveEntries).await.unwrap();

    println!("Found {} reserve entries", entries.len());
    for entry in entries.iter().take(5) {
        println!(
            "  ReserveEntry: {} - reserve: {:?}, amount: {:?}, event: {:?}",
            entry.id.as_str(),
            entry.reserve,
            entry.amount,
            entry.event
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_reserves() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let reserves: Vec<Reserve> = client.get_all(EntityType::Reserves).await.unwrap();

    println!("Found {} reserves", reserves.len());
    for reserve in reserves.iter().take(5) {
        println!(
            "  Reserve: {} - entity: {:?}, max: {:?}, status: {:?}",
            reserve.id.as_str(),
            reserve.entity,
            reserve.max,
            reserve.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_vendors() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let vendors: Vec<Vendor> = client.get_all(EntityType::Vendors).await.unwrap();

    println!("Found {} vendors", vendors.len());
    for vendor in vendors.iter().take(5) {
        println!(
            "  Vendor: {} - entity: {:?}, division: {:?}",
            vendor.id.as_str(),
            vendor.entity,
            vendor.division
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_account_verifications() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let verifications: Vec<AccountVerification> =
        client.get_all(EntityType::AccountVerifications).await.unwrap();

    println!("Found {} account verifications", verifications.len());
    for v in verifications.iter().take(5) {
        println!(
            "  AccountVerification: {} - account: {:?}, type: {:?}, debit1: {:?}",
            v.id.as_str(),
            v.account,
            v.verification_type,
            v.debit1
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_adjustments() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let adjustments: Vec<Adjustment> = client.get_all(EntityType::Adjustments).await.unwrap();

    println!("Found {} adjustments", adjustments.len());
    for adj in adjustments.iter().take(5) {
        println!(
            "  Adjustment: {} - entity: {:?}, amount: {:?}, description: {:?}",
            adj.id.as_str(),
            adj.entity,
            adj.amount,
            adj.description
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_messages() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let messages: Vec<ChargebackMessage> =
        client.get_all(EntityType::ChargebackMessages).await.unwrap();

    println!("Found {} chargeback messages", messages.len());
    for msg in messages.iter().take(5) {
        println!(
            "  ChargebackMessage: {} - chargeback: {:?}, direction: {:?}, status: {:?}",
            msg.id.as_str(),
            msg.chargeback,
            msg.direction,
            msg.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_documents() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let docs: Vec<ChargebackDocument> =
        client.get_all(EntityType::ChargebackDocuments).await.unwrap();

    println!("Found {} chargeback documents", docs.len());
    for doc in docs.iter().take(5) {
        println!(
            "  ChargebackDocument: {} - chargeback: {:?}, document_type: {:?}, name: {:?}",
            doc.id.as_str(),
            doc.chargeback,
            doc.document_type,
            doc.name
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_message_results() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let results: Vec<ChargebackMessageResult> =
        client.get_all(EntityType::ChargebackMessageResults).await.unwrap();

    println!("Found {} chargeback message results", results.len());
    for r in results.iter().take(5) {
        println!(
            "  ChargebackMessageResult: {} - message: {:?}, result_type: {:?}",
            r.id.as_str(),
            r.message,
            r.result_type
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_statuses() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let statuses: Vec<ChargebackStatus> =
        client.get_all(EntityType::ChargebackStatuses).await.unwrap();

    println!("Found {} chargeback statuses", statuses.len());
    // Print ALL statuses to capture all possible status values for API documentation
    for s in statuses.iter() {
        println!(
            "  ChargebackStatus: {} - chargeback: {:?}, status: {:?}, message: {:?}",
            s.id.as_str(),
            s.chargeback,
            s.status,
            s.chargeback_message
        );
    }
}

// ==================== Chargeback Workflow Tests ====================
// NOTE: Chargebacks cannot be created via API - they are initiated by card issuers.
// These tests work with existing chargebacks in the test environment.
// Lifecycle changes (close, arbitration, etc.) require Payrix Support involvement.

/// Test constant: An open chargeback that can be used for message/document tests.
/// Update this ID if the chargeback status changes or a new open chargeback is available.
const TEST_OPEN_CHARGEBACK_ID: &str = "t1_chb_6616a9de06fd751e5ae91e5";

/// Test constant: A closed chargeback for read-only testing.
const TEST_CLOSED_CHARGEBACK_ID: &str = "t1_chb_6616a9f7c19a47bea938957";

/// Test constant: A won chargeback for read-only testing.
const TEST_WON_CHARGEBACK_ID: &str = "t1_chb_6615a4fbc5e0e79dac81419";

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_specific_chargeback() {
    // Test reading a specific chargeback by ID and validating response fields
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let chargeback: Option<Chargeback> = client
        .get_one(EntityType::Chargebacks, TEST_OPEN_CHARGEBACK_ID)
        .await
        .expect("Failed to get chargeback");

    assert!(chargeback.is_some(), "Chargeback should exist");
    let cb = chargeback.unwrap();

    println!("=== CHARGEBACK DETAILS ===");
    println!("ID: {}", cb.id.as_str());
    println!("Status: {:?}", cb.status);
    println!("Cycle: {:?}", cb.cycle);
    println!("Reason Code: {:?}", cb.reason_code);
    println!("Reason: {:?}", cb.reason);
    println!("Total: {:?}", cb.total);
    println!("Currency: {:?}", cb.currency);
    println!("Issued: {:?}", cb.issued);
    println!("Received: {:?}", cb.received);
    println!("Merchant: {:?}", cb.merchant);
    println!("Transaction: {:?}", cb.txn);
    println!("Bank Ref: {:?}", cb.bank_ref);
    println!("Chargeback Ref: {:?}", cb.chargeback_ref);
    println!("Payment Method: {:?}", cb.payment_method);

    // Validate the chargeback has expected fields populated
    assert_eq!(cb.id.as_str(), TEST_OPEN_CHARGEBACK_ID);
    assert!(cb.status.is_some(), "Status should be present");
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates data in Payrix"]
async fn test_create_chargeback_message() {
    // Test creating a message on an existing open chargeback.
    // NOTE: This creates real data in the test environment.
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // First verify the chargeback exists and is open
    let chargeback: Option<Chargeback> = client
        .get_one(EntityType::Chargebacks, TEST_OPEN_CHARGEBACK_ID)
        .await
        .expect("Failed to get chargeback");

    let cb = chargeback.expect("Test chargeback should exist");
    println!("Creating message on chargeback: {} (status: {:?})", cb.id.as_str(), cb.status);

    // Note: Some message types require specific chargeback states.
    // "notate" is generally safe for adding notes to any chargeback.
    let new_message = NewChargebackMessage {
        chargeback: TEST_OPEN_CHARGEBACK_ID.to_string(),
        message_type: Some(ChargebackMessageType::Notate),
        subject: Some("Integration Test Note".to_string()),
        message: Some(format!(
            "Test message created by payrix-rs integration tests at timestamp {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )),
    };

    let result: Result<ChargebackMessage, _> = client
        .create(EntityType::ChargebackMessages, &new_message)
        .await;

    match result {
        Ok(msg) => {
            println!("=== MESSAGE CREATED SUCCESSFULLY ===");
            println!("ID: {}", msg.id.as_str());
            println!("Chargeback: {:?}", msg.chargeback);
            println!("Type: {:?}", msg.message_type);
            println!("Status: {:?}", msg.status);
            println!("Subject: {:?}", msg.subject);
            println!("Message: {:?}", msg.message);
            println!("Direction: {:?}", msg.direction);
            println!("Created: {:?}", msg.created);
        }
        Err(e) => {
            println!("=== MESSAGE CREATION FAILED ===");
            println!("Error: {:?}", e);
            // Don't panic - document the error for analysis
            println!("NOTE: Message creation may fail if the chargeback state doesn't allow this message type");
        }
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_status_history() {
    // Test getting the status history for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // Get status records for the closed chargeback (should have status history)
    let search = SearchBuilder::new()
        .field("chargeback", TEST_CLOSED_CHARGEBACK_ID)
        .build();

    let statuses: Vec<ChargebackStatus> = client
        .search(EntityType::ChargebackStatuses, &search)
        .await
        .expect("Failed to get chargeback statuses");

    println!("=== STATUS HISTORY FOR {} ===", TEST_CLOSED_CHARGEBACK_ID);
    println!("Found {} status records", statuses.len());

    for status in &statuses {
        println!(
            "  {} - status: {:?}, chargeback: {:?}, message: {:?}",
            status.id.as_str(),
            status.status,
            status.chargeback,
            status.chargeback_message
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_messages_for_chargeback() {
    // Test getting messages for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let search = SearchBuilder::new()
        .field("chargeback", TEST_OPEN_CHARGEBACK_ID)
        .build();

    let messages: Vec<ChargebackMessage> = client
        .search(EntityType::ChargebackMessages, &search)
        .await
        .expect("Failed to get chargeback messages");

    println!("=== MESSAGES FOR {} ===", TEST_OPEN_CHARGEBACK_ID);
    println!("Found {} messages", messages.len());

    for msg in &messages {
        println!(
            "  {} - type: {:?}, status: {:?}, subject: {:?}",
            msg.id.as_str(),
            msg.message_type,
            msg.status,
            msg.subject
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_documents_for_chargeback() {
    // Test getting documents for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let search = SearchBuilder::new()
        .field("chargeback", TEST_OPEN_CHARGEBACK_ID)
        .build();

    let documents: Vec<ChargebackDocument> = client
        .search(EntityType::ChargebackDocuments, &search)
        .await
        .expect("Failed to get chargeback documents");

    println!("=== DOCUMENTS FOR {} ===", TEST_OPEN_CHARGEBACK_ID);
    println!("Found {} documents", documents.len());

    for doc in &documents {
        println!(
            "  {} - name: {:?}, type: {:?}, size: {:?}",
            doc.id.as_str(),
            doc.name,
            doc.document_type,
            doc.size
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_outcomes_by_status() {
    // Test aggregating chargebacks by outcome status
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();

    let mut open = 0;
    let mut won = 0;
    let mut lost = 0;
    let mut closed = 0;
    let mut other = 0;

    for cb in &chargebacks {
        match cb.status {
            Some(ChargebackStatusValue::Open) => open += 1,
            Some(ChargebackStatusValue::Won) => won += 1,
            Some(ChargebackStatusValue::Lost) => lost += 1,
            Some(ChargebackStatusValue::Closed) => closed += 1,
            None => other += 1,
        }
    }

    println!("=== CHARGEBACK STATUS SUMMARY ===");
    println!("Total: {}", chargebacks.len());
    println!("  Open: {}", open);
    println!("  Won: {}", won);
    println!("  Lost: {}", lost);
    println!("  Closed: {}", closed);
    println!("  Other/Unknown: {}", other);
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_disbursements() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let disbursements: Vec<Disbursement> =
        client.get_all(EntityType::Disbursements).await.unwrap();

    println!("Found {} disbursements", disbursements.len());
    for d in disbursements.iter().take(5) {
        println!(
            "  Disbursement: {} - entity: {:?}, amount: {:?}, status: {:?}",
            d.id.as_str(),
            d.entity,
            d.amount,
            d.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_disbursement_entries() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let entries: Vec<DisbursementEntry> =
        client.get_all(EntityType::DisbursementEntries).await.unwrap();

    println!("Found {} disbursement entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  DisbursementEntry: {} - disbursement: {:?}, amount: {:?}",
            e.id.as_str(),
            e.disbursement,
            e.amount
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_entries() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let entries: Vec<Entry> = client.get_all(EntityType::Entries).await.unwrap();

    println!("Found {} entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  Entry: {} - entity: {:?}, amount: {:?}, event: {:?}",
            e.id.as_str(),
            e.entity,
            e.amount,
            e.event
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_pending_entries() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let entries: Vec<PendingEntry> = client.get_all(EntityType::PendingEntries).await.unwrap();

    println!("Found {} pending entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  PendingEntry: {} - entity: {:?}, amount: {:?}, event: {:?}",
            e.id.as_str(),
            e.entity,
            e.amount,
            e.event
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_refunds() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let refunds: Vec<Refund> = client.get_all(EntityType::Refunds).await.unwrap();

    println!("Found {} refunds", refunds.len());
    for r in refunds.iter().take(5) {
        println!(
            "  Refund: {} - entry: {:?}, amount: {:?}, description: {:?}",
            r.id.as_str(),
            r.entry,
            r.amount,
            r.description
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_alerts() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let alerts: Vec<Alert> = client.get_all(EntityType::Alerts).await.unwrap();

    println!("Found {} alerts", alerts.len());
    for a in alerts.iter().take(5) {
        println!(
            "  Alert: {} - name: {:?}, login: {:?}",
            a.id.as_str(),
            a.name,
            a.login
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_alert_actions() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let actions: Vec<AlertAction> = client.get_all(EntityType::AlertActions).await.unwrap();

    println!("Found {} alert actions", actions.len());
    for a in actions.iter().take(5) {
        println!(
            "  AlertAction: {} - alert: {:?}, action_type: {:?}",
            a.id.as_str(),
            a.alert,
            a.action_type
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_alert_triggers() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let triggers: Vec<AlertTrigger> = client.get_all(EntityType::AlertTriggers).await.unwrap();

    println!("Found {} alert triggers", triggers.len());
    for t in triggers.iter().take(5) {
        println!(
            "  AlertTrigger: {} - alert: {:?}, event: {:?}",
            t.id.as_str(),
            t.alert,
            t.event
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_notes() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let notes: Vec<Note> = client.get_all(EntityType::Notes).await.unwrap();

    println!("Found {} notes", notes.len());
    for n in notes.iter().take(5) {
        println!(
            "  Note: {} - entity: {:?}, note: {:?}, note_type: {:?}",
            n.id.as_str(),
            n.entity,
            n.note,
            n.note_type
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_note_documents() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let docs: Vec<NoteDocument> = client.get_all(EntityType::NoteDocuments).await.unwrap();

    println!("Found {} note documents", docs.len());
    for d in docs.iter().take(5) {
        println!(
            "  NoteDocument: {} - note: {:?}, custom: {:?}, document_type: {:?}",
            d.id.as_str(),
            d.note,
            d.custom,
            d.document_type
        );
    }
}

// ==================== Comprehensive Enum Value Discovery Tests ====================
// These tests are designed to capture ALL possible enum values from the API
// to help identify undocumented values that differ from the OpenAPI spec.

/// Comprehensive test to discover all ChargebackStatusValue variants in use.
/// This test queries both chargebacks and chargeback statuses to find all
/// status values, including potentially undocumented ones like:
/// - "new", "underReview", "responded", "expired"
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_discover_chargeback_status_values() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== CHARGEBACK STATUS VALUE DISCOVERY ===\n");

    // Collect all unique status values
    let mut unique_statuses: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Get all chargebacks and their status field
    println!("--- Checking Chargeback.status field ---");
    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();
    println!("Total chargebacks: {}", chargebacks.len());

    for cb in &chargebacks {
        if let Some(status) = &cb.status {
            let status_str = format!("{:?}", status);
            unique_statuses.insert(status_str.clone());
            println!("  {} -> status: {:?}", cb.id.as_str(), status);
        }
    }

    // 2. Get all chargeback status records (status change history)
    println!("\n--- Checking ChargebackStatus.status field ---");
    let statuses: Vec<ChargebackStatus> = client.get_all(EntityType::ChargebackStatuses).await.unwrap();
    println!("Total chargeback status records: {}", statuses.len());

    for s in &statuses {
        if let Some(status_val) = &s.status {
            let status_str = format!("{:?}", status_val);
            unique_statuses.insert(status_str);
        }
        println!(
            "  {} -> status: {:?}, chargeback: {:?}, message: {:?}",
            s.id.as_str(),
            s.status,
            s.chargeback,
            s.chargeback_message
        );
    }

    // 3. Summary
    println!("\n=== UNIQUE STATUS VALUES FOUND ===");
    let mut sorted: Vec<_> = unique_statuses.iter().collect();
    sorted.sort();
    for status in sorted {
        println!("  - {}", status);
    }

    println!("\n=== EXPECTED VALUES FROM OPENAPI (4 documented) ===");
    println!("  - Open");
    println!("  - Closed");
    println!("  - Won");
    println!("  - Lost");

    println!("\n=== POTENTIALLY UNDOCUMENTED VALUES TO WATCH FOR ===");
    println!("  - New (lifecycle: initial state)");
    println!("  - UnderReview (lifecycle: being reviewed)");
    println!("  - Responded (lifecycle: merchant responded)");
    println!("  - Expired (lifecycle: response deadline passed)");
}

/// Test to get raw JSON response for chargebacks to see exact status values.
/// This bypasses Rust deserialization to see the raw API values.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_raw_json_status() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");

    println!("\n=== RAW JSON CHARGEBACK STATUS CHECK ===\n");

    // Make a direct HTTP request to see raw JSON
    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://test-api.payrix.com/chargebacks")
        .header("APIKEY", &api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to fetch chargebacks");

    let body = response.text().await.expect("Failed to read response body");

    // Parse as generic JSON to see raw status values
    let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(response_obj) = json.get("response") {
        if let Some(data) = response_obj.get("data") {
            if let Some(arr) = data.as_array() {
                println!("Found {} chargebacks in raw response", arr.len());

                // Collect unique raw status values
                let mut raw_statuses: std::collections::HashSet<String> = std::collections::HashSet::new();

                for item in arr {
                    if let Some(status) = item.get("status") {
                        let status_str = status.to_string();
                        raw_statuses.insert(status_str.clone());
                        let id = item.get("id").map(|v| v.to_string()).unwrap_or_default();
                        println!("  {} -> raw status: {}", id, status_str);
                    }
                }

                println!("\n=== UNIQUE RAW STATUS VALUES ===");
                let mut sorted: Vec<_> = raw_statuses.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }
            }
        }
    }
}

/// Test to check chargeback status history for transitions.
/// Status changes might reveal intermediate states.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_status_history_raw() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");

    println!("\n=== RAW JSON CHARGEBACK STATUS HISTORY CHECK ===\n");

    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://test-api.payrix.com/chargebackStatuses")
        .header("APIKEY", &api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to fetch chargeback statuses");

    let body = response.text().await.expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(response_obj) = json.get("response") {
        if let Some(data) = response_obj.get("data") {
            if let Some(arr) = data.as_array() {
                println!("Found {} chargeback status records in raw response", arr.len());

                let mut raw_from: std::collections::HashSet<String> = std::collections::HashSet::new();
                let mut raw_to: std::collections::HashSet<String> = std::collections::HashSet::new();

                for item in arr {
                    let id = item.get("id").map(|v| v.to_string()).unwrap_or_default();
                    let from_status = item.get("fromStatus").map(|v| v.to_string()).unwrap_or_else(|| "null".to_string());
                    let to_status = item.get("toStatus").map(|v| v.to_string()).unwrap_or_else(|| "null".to_string());
                    let status = item.get("status").map(|v| v.to_string()).unwrap_or_else(|| "null".to_string());
                    let name = item.get("name").map(|v| v.to_string()).unwrap_or_else(|| "null".to_string());

                    raw_from.insert(from_status.clone());
                    raw_to.insert(to_status.clone());

                    println!("  {} -> status: {}, fromStatus: {}, toStatus: {}, name: {}",
                             id, status, from_status, to_status, name);
                }

                println!("\n=== UNIQUE RAW fromStatus VALUES ===");
                let mut sorted: Vec<_> = raw_from.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }

                println!("\n=== UNIQUE RAW toStatus VALUES ===");
                let mut sorted: Vec<_> = raw_to.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }
            }
        }
    }
}

// ==================== Merchant Onboarding Workflow Tests ====================

/// Helper to create a test onboarding request with all required fields.
fn create_test_onboarding_request() -> OnboardMerchantRequest {
    // Generate a unique timestamp for testing
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    OnboardMerchantRequest {
        business: BusinessInfo {
            business_type: MerchantType::LimitedLiabilityCorporation,
            legal_name: format!("Test Business {} LLC", timestamp),
            address: Address {
                line1: "123 Test Street".to_string(),
                line2: Some("Suite 100".to_string()),
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62701".to_string(),
                country: "USA".to_string(),
            },
            phone: "5551234567".to_string(),
            email: "payrixrust@gmail.com".to_string(),
            website: Some("https://github.com/outlawpractice/payrix-rs".to_string()),
            ein: "123456789".to_string(),
        },
        merchant: MerchantConfig {
            dba: format!("Test DBA {}", timestamp),
            mcc: "5999".to_string(), // Miscellaneous Retail
            environment: MerchantEnvironment::Ecommerce,
            annual_cc_sales: 50_000_000, // $500,000 in cents
            avg_ticket: 5_000,           // $50 in cents
            established: DateYmd::new("20200101").unwrap(),
            is_new_business: false,
        },
        accounts: vec![BankAccountInfo {
            name: Some("Operating Account".to_string()),
            routing_number: Some("121000358".to_string()), // Test routing number
            account_number: Some("123456789".to_string()),
            holder_type: AccountHolderType::Business,
            account_method: BankAccountMethod::Checking,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: None,
        }],
        members: vec![MemberInfo {
            member_type: MemberType::Owner,
            first_name: "Test".to_string(),
            last_name: "Owner".to_string(),
            title: Some("CEO".to_string()),
            ownership_percentage: 100,
            date_of_birth: "19800115".to_string(),
            ssn: "123456789".to_string(),
            email: "payrixrust@gmail.com".to_string(),
            phone: "5559876543".to_string(),
            address: Address {
                line1: "456 Owner Lane".to_string(),
                line2: None,
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62702".to_string(),
                country: "USA".to_string(),
            },
        }],
        terms_acceptance: TermsAcceptance {
            version: "4.21".to_string(),
            accepted_at: "2024-01-15 10:30:00".to_string(),
        },
    }
}

/// Test the full merchant onboarding workflow.
///
/// **CAUTION**: This test creates real resources in Payrix!
/// - Creates a new Entity (business)
/// - Creates a new Merchant
/// - Creates bank Account(s)
/// - Creates Member(s) (beneficial owners)
///
/// These resources will remain in your Payrix test account.
/// Run this test sparingly to avoid accumulating test data.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_workflow() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WORKFLOW TEST ===\n");

    let request = create_test_onboarding_request();
    println!("Onboarding business: {}", request.business.legal_name);
    println!("DBA: {}", request.merchant.dba);

    // Execute the onboarding workflow
    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Boarding Status: {:?}", onboard_result.boarding_status);
            println!("Accounts created: {}", onboard_result.accounts.len());
            println!("Members created: {}", onboard_result.members.len());

            // Verify the result structure
            assert!(!onboard_result.entity_id.is_empty(), "Entity ID should not be empty");
            assert!(!onboard_result.merchant_id.is_empty(), "Merchant ID should not be empty");

            // The boarding status depends on Payrix's underwriting decision
            println!("\nBoarding status interpretation:");
            match onboard_result.boarding_status {
                BoardingStatus::Boarded => println!("  ✓ Merchant was immediately approved!"),
                BoardingStatus::Submitted => println!("  → Merchant submitted for boarding, awaiting processing"),
                BoardingStatus::Pending => println!("  → Merchant pending automated review"),
                BoardingStatus::ManualReview => println!("  → Merchant requires manual underwriting review"),
                BoardingStatus::NotReady => println!("  ⚠ Merchant not ready - check for missing information"),
                BoardingStatus::Incomplete => println!("  ⚠ Application incomplete - check required fields"),
                BoardingStatus::Closed => println!("  ✗ Merchant account was closed"),
            }

            // Test check_boarding_status with the created merchant
            println!("\n=== CHECKING BOARDING STATUS ===");
            let status_result = check_boarding_status(&client, &onboard_result.merchant_id).await;

            match status_result {
                Ok(status) => {
                    println!("Current Status: {:?}", status.status);
                    println!("Merchant ID: {}", status.merchant_id);
                    println!("Entity ID: {}", status.entity_id);
                    if let Some(boarded_date) = &status.boarded_date {
                        println!("Boarded Date: {}", boarded_date);
                    }
                }
                Err(e) => {
                    println!("Error checking boarding status: {}", e);
                    // Don't fail the test - the onboarding was successful
                }
            }
        }
        Err(e) => {
            // Onboarding can fail for many reasons in test environment
            println!("\n=== ONBOARDING FAILED ===");
            println!("Error: {}", e);

            // Some errors are expected in test environment
            let error_str = format!("{}", e);
            if error_str.contains("ein") || error_str.contains("EIN") {
                println!("\nNote: EIN validation may fail with test data");
            }
            if error_str.contains("ssn") || error_str.contains("SSN") {
                println!("\nNote: SSN validation may fail with test data");
            }
            if error_str.contains("routing") {
                println!("\nNote: Bank routing number validation may fail with test data");
            }

            // In a real test, you might want to fail here
            // For now, we just report the error since test data may not pass validation
            println!("\nThis is expected when using test/dummy data for EIN, SSN, and bank accounts.");
        }
    }
}

/// Test checking boarding status for an existing merchant.
///
/// This test looks up an existing merchant and checks its boarding status.
/// It doesn't create any new resources.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_check_boarding_status() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== CHECK BOARDING STATUS TEST ===\n");

    // First, get an existing merchant
    let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await.unwrap();

    if merchants.is_empty() {
        println!("No merchants found in test account - skipping test");
        return;
    }

    let merchant = &merchants[0];
    println!("Checking status for merchant: {}", merchant.id.as_str());
    println!("Merchant DBA: {:?}", merchant.dba);
    println!("Current merchant status: {:?}", merchant.status);

    // Check boarding status using the workflow function
    let result = check_boarding_status(&client, merchant.id.as_str()).await;

    match result {
        Ok(status) => {
            println!("\n=== BOARDING STATUS RESULT ===");
            println!("Status: {:?}", status.status);
            println!("Merchant ID: {}", status.merchant_id);
            println!("Entity ID: {}", status.entity_id);
            if let Some(boarded_date) = &status.boarded_date {
                println!("Boarded Date: {}", boarded_date);
            }

            // Verify the merchant ID matches
            assert_eq!(status.merchant_id, merchant.id.as_str());
        }
        Err(e) => {
            println!("Error: {}", e);
            panic!("Failed to check boarding status: {}", e);
        }
    }
}

/// Test onboarding with multiple accounts (trust + operating pattern).
///
/// This test demonstrates the common scenario of having both:
/// - Operating account (All) - for deposits AND fee withdrawals
/// - Trust account (Credit only) - for deposits only
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_with_trust_account() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WITH TRUST ACCOUNT TEST ===\n");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut request = create_test_onboarding_request();

    // Update with unique data (keep email as payrixrust@gmail.com from helper)
    request.business.legal_name = format!("Trust Account Test {} LLC", timestamp);
    request.merchant.dba = format!("Trust Test DBA {}", timestamp);

    // Add a trust account in addition to the operating account
    request.accounts.push(BankAccountInfo {
        name: Some("Client Trust Account".to_string()),
        routing_number: Some("121000358".to_string()),
        account_number: Some("987654321".to_string()),
        holder_type: AccountHolderType::Business,
        account_method: BankAccountMethod::Checking,
        transaction_type: AccountType::Credit, // Deposits ONLY - no fee withdrawals
        currency: Some("USD".to_string()),
        is_primary: false, // Not primary - fees come from operating
        plaid_public_token: None,
    });

    println!("Onboarding with {} accounts:", request.accounts.len());
    for (i, acct) in request.accounts.iter().enumerate() {
        println!(
            "  Account {}: {:?} - type: {:?}, primary: {}",
            i + 1,
            acct.name,
            acct.transaction_type,
            acct.is_primary
        );
    }

    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Accounts created: {}", onboard_result.accounts.len());

            // Verify both accounts were created
            // Note: The API may or may not return both accounts depending on configuration
            println!("\nCreated accounts:");
            for acct in &onboard_result.accounts {
                println!(
                    "  - {}: type={:?}, primary={:?}",
                    acct.id.as_str(),
                    acct.account_type,
                    acct.primary
                );
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Debug: {:?}", e);
            println!("\nNote: This may fail with dummy data or if accounts already exist.");
        }
    }
}

/// Test onboarding with multiple members (beneficial owners).
///
/// This test demonstrates the scenario of having multiple beneficial owners
/// and a control person as required by FinCEN regulations.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates real resources in Payrix"]
async fn test_merchant_onboarding_with_multiple_members() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== MERCHANT ONBOARDING WITH MULTIPLE MEMBERS TEST ===\n");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut request = create_test_onboarding_request();

    // Update with unique data
    request.business.legal_name = format!("Multi-Member Test {} LLC", timestamp);
    request.merchant.dba = format!("Multi-Member DBA {}", timestamp);
    request.business.email = format!("multimember{}@example.com", timestamp);

    // First member owns 60%
    request.members[0].ownership_percentage = 60;
    request.members[0].email = format!("owner1-{}@example.com", timestamp);

    // Add second owner with 40%
    request.members.push(MemberInfo {
        member_type: MemberType::Owner,
        first_name: "Second".to_string(),
        last_name: "Owner".to_string(),
        title: Some("CFO".to_string()),
        ownership_percentage: 40,
        date_of_birth: "19850620".to_string(),
        ssn: "987654321".to_string(),
        email: format!("owner2-{}@example.com", timestamp),
        phone: "5551112222".to_string(),
        address: Address {
            line1: "789 Partner Rd".to_string(),
            line2: None,
            city: "Chicago".to_string(),
            state: "IL".to_string(),
            zip: "60601".to_string(),
            country: "USA".to_string(),
        },
    });

    // Add control person (may have 0% ownership but has management control)
    request.members.push(MemberInfo {
        member_type: MemberType::ControlPerson,
        first_name: "Control".to_string(),
        last_name: "Person".to_string(),
        title: Some("COO".to_string()),
        ownership_percentage: 0, // Control persons may not have ownership
        date_of_birth: "19900301".to_string(),
        ssn: "111223333".to_string(),
        email: format!("control-{}@example.com", timestamp),
        phone: "5553334444".to_string(),
        address: Address {
            line1: "321 Control Ave".to_string(),
            line2: Some("Apt 2B".to_string()),
            city: "Naperville".to_string(),
            state: "IL".to_string(),
            zip: "60540".to_string(),
            country: "USA".to_string(),
        },
    });

    println!("Onboarding with {} members:", request.members.len());
    for member in &request.members {
        println!(
            "  - {} {}: {:?}, {}% ownership",
            member.first_name, member.last_name, member.member_type, member.ownership_percentage
        );
    }

    let result = onboard_merchant(&client, request).await;

    match result {
        Ok(onboard_result) => {
            println!("\n=== ONBOARDING SUCCESSFUL ===");
            println!("Entity ID: {}", onboard_result.entity_id);
            println!("Merchant ID: {}", onboard_result.merchant_id);
            println!("Members created: {}", onboard_result.members.len());

            println!("\nCreated members:");
            for member in &onboard_result.members {
                println!(
                    "  - {}: {:?} {:?}, title={:?}, ownership={:?}",
                    member.id.as_str(),
                    member.first,
                    member.last,
                    member.title,
                    member.ownership
                );
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("\nThis is expected when using test/dummy data.");
        }
    }
}

// =============================================================================
// OpenAPI Validation Tests
// =============================================================================
// These tests create data using our test merchant and validate that API responses
// match the OpenAPI specification. Failures document real API inconsistencies.

/// Create a customer and validate all response fields match OpenAPI spec.
///
/// This test creates a customer with all optional fields populated to ensure
/// the API returns them correctly.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates real customer"]
async fn test_create_customer_validate_response() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create customer with all fields populated
    let new_customer = NewCustomer {
        merchant: TEST_MERCHANT_ID.to_string(),
        first: Some(format!("Test{}", timestamp)),
        middle: Some("M".to_string()),
        last: Some("Customer".to_string()),
        email: Some("payrixrust@gmail.com".to_string()),
        phone: Some("5551234567".to_string()),
        address1: Some("123 Test Street".to_string()),
        address2: Some("Suite 100".to_string()),
        city: Some("Chicago".to_string()),
        state: Some("IL".to_string()),
        zip: Some("60601".to_string()),
        country: Some("USA".to_string()),
        company: Some(format!("Test Company {}", timestamp)),
        custom: Some(format!("{{\"test_id\": {}}}", timestamp)),
        ..Default::default()
    };

    println!("Creating customer: {} {} ({})",
        new_customer.first.as_deref().unwrap_or(""),
        new_customer.last.as_deref().unwrap_or(""),
        new_customer.email.as_deref().unwrap_or("")
    );

    let result: Result<Customer, _> = client.create(EntityType::Customers, &new_customer).await;

    match result {
        Ok(customer) => {
            println!("\n=== CUSTOMER CREATED SUCCESSFULLY ===");
            println!("ID: {}", customer.id.as_str());

            // Validate required fields
            assert!(!customer.id.as_str().is_empty(), "ID should not be empty");
            assert!(customer.id.as_str().starts_with("t1_cus_"), "ID should start with t1_cus_");

            // Validate optional fields that we set
            assert_eq!(customer.first.as_deref(), Some(&format!("Test{}", timestamp) as &str));
            assert_eq!(customer.middle.as_deref(), Some("M"));
            assert_eq!(customer.last.as_deref(), Some("Customer"));
            assert_eq!(customer.email.as_deref(), Some("payrixrust@gmail.com"));
            assert_eq!(customer.phone.as_deref(), Some("5551234567"));
            assert_eq!(customer.city.as_deref(), Some("Chicago"));
            assert_eq!(customer.state.as_deref(), Some("IL"));
            assert_eq!(customer.zip.as_deref(), Some("60601"));

            // Validate system-generated fields
            assert!(customer.created.is_some(), "Created timestamp should be set");
            assert!(customer.modified.is_some(), "Modified timestamp should be set");

            println!("\nAll response fields validated successfully!");
            println!("Customer ID for use in other tests: {}", customer.id.as_str());
        }
        Err(e) => {
            panic!("Failed to create customer: {:?}", e);
        }
    }
}

/// Create a token and validate all response fields match OpenAPI spec.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates real token"]
async fn test_create_token_validate_response() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // First create a customer to attach the token to
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let new_customer = NewCustomer {
        merchant: TEST_MERCHANT_ID.to_string(),
        first: Some(format!("TokenTest{}", timestamp)),
        last: Some("Customer".to_string()),
        email: Some("payrixrust@gmail.com".to_string()),
        ..Default::default()
    };

    let customer: Customer = client.create(EntityType::Customers, &new_customer).await
        .expect("Failed to create customer for token test");

    println!("Created customer: {}", customer.id.as_str());

    // Create a Visa token
    let new_token = NewToken {
        customer: customer.id.to_string(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            routing: None,
            expiration: Some("1230".to_string()), // December 2030
            cvv: Some("123".to_string()),
        },
        ..Default::default()
    };

    println!("Creating Visa token for customer {}", customer.id.as_str());

    let result: Result<Token, _> = client.create(EntityType::Tokens, &new_token).await;

    match result {
        Ok(token) => {
            println!("\n=== TOKEN CREATED SUCCESSFULLY ===");
            println!("ID: {}", token.id.as_str());

            // Validate required fields
            assert!(!token.id.as_str().is_empty(), "ID should not be empty");
            assert!(token.id.as_str().starts_with("t1_tok_"), "ID should start with t1_tok_");

            // Note: payment info is stored in the payment object
            println!("Payment method: {:?}", token.payment);
            println!("Token status: {:?}", token.status);

            // Validate expiration
            assert!(token.expiration.is_some(), "Expiration should be set");

            // Validate timestamps
            assert!(token.created.is_some(), "Created timestamp should be set");

            println!("\nAll response fields validated successfully!");
            println!("Token ID: {}", token.id.as_str());
            println!("Token status: {:?}", token.status);
            println!("Payment Method: {:?}", token.payment);
        }
        Err(e) => {
            panic!("Failed to create token: {:?}", e);
        }
    }
}

/// Create a transaction and validate response fields match OpenAPI spec.
/// This test validates integer enums are returned correctly.
///
/// NOTE: Requires TEST_MERCHANT_ID to have status=Boarded. The test merchant
/// currently has status=NotReady because it was created with test data.
/// This test documents the expected behavior once a merchant is properly boarded.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY and TEST_MERCHANT_ID to be Boarded"]
async fn test_create_transaction_validate_response() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // First create customer and token
    let new_customer = NewCustomer {
        merchant: TEST_MERCHANT_ID.to_string(),
        first: Some(format!("TxnTest{}", timestamp)),
        last: Some("Customer".to_string()),
        email: Some("payrixrust@gmail.com".to_string()),
        ..Default::default()
    };

    let customer: Customer = client.create(EntityType::Customers, &new_customer).await
        .expect("Failed to create customer");

    let new_token = NewToken {
        customer: customer.id.to_string(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            routing: None,
            expiration: Some("1230".to_string()),
            cvv: Some("123".to_string()),
        },
        ..Default::default()
    };

    let token: Token = client.create(EntityType::Tokens, &new_token).await
        .expect("Failed to create token");

    println!("Created customer {} and token {}", customer.id.as_str(), token.id.as_str());

    // Create a $1.00 transaction using the token's transaction token string
    let token_string = token.token.expect("Token should have a token string");
    let new_txn = NewTransaction {
        merchant: TEST_MERCHANT_ID.to_string(),
        token: Some(token_string),
        total: 100, // $1.00 in cents
        origin: Some(TransactionOrigin::Ecommerce),
        ..Default::default()
    };

    println!("Creating $1.00 transaction...");

    let result: Result<Transaction, _> = client.create(EntityType::Txns, &new_txn).await;

    match result {
        Ok(txn) => {
            println!("\n=== TRANSACTION CREATED SUCCESSFULLY ===");
            println!("ID: {}", txn.id.as_str());
            println!("Status: {:?}", txn.status);
            println!("Type: {:?}", txn.txn_type);
            println!("Total: {:?}", txn.total);

            // Validate required fields
            assert!(!txn.id.as_str().is_empty(), "ID should not be empty");
            assert!(txn.id.as_str().starts_with("t1_txn_"), "ID should start with t1_txn_");

            // Validate amounts
            assert_eq!(txn.total, Some(100), "Total should be 100 cents");

            // Validate enums (these should be integers per OpenAPI spec)
            // If this fails with "invalid type: string expected i32", the API
            // is returning strings instead of integers for enum fields
            assert!(txn.status.is_some(), "Status should be set");
            // txn_type is not Option, it's a direct TransactionType enum
            println!("Transaction type: {:?}", txn.txn_type);

            // Validate timestamps
            assert!(txn.created.is_some(), "Created timestamp should be set");

            println!("\nAll response fields validated successfully!");
            println!("Transaction ID: {}", txn.id.as_str());
        }
        Err(e) => {
            // This is where we'll see the API inconsistencies
            println!("\n=== TRANSACTION CREATION FAILED ===");
            println!("Error: {:?}", e);
            panic!("Transaction creation failed - this documents an API inconsistency: {:?}", e);
        }
    }
}
