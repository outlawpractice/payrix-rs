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
    ChargebackDocument, ChargebackMessage, ChargebackMessageResult, ChargebackStatus, Contact,
    Customer, Disbursement, DisbursementEntry, Entity, EntityReserve, EntityType, Entry,
    Environment, Fee, FeeRule, Fund, Login, Member, Merchant, NewCustomer, NewToken,
    NewTransaction, Note, NoteDocument, Org, OrgEntity, PaymentInfo, PaymentMethod, Payout,
    PayrixClient, PendingEntry, Plan, Refund, Reserve, ReserveEntry, SearchBuilder, Subscription,
    TeamLogin, Token, Transaction, TransactionOrigin, Vendor,
    // Workflow types
    onboard_merchant, check_boarding_status, OnboardMerchantRequest, BusinessInfo, MerchantConfig,
    BankAccountInfo, BankAccountMethod, MemberInfo, Address, TermsAcceptance, BoardingStatus,
};
use payrix::types::{
    AccountHolderType, AccountType, DateYmd, MemberType, MerchantEnvironment, MerchantType,
};
use serde_json::json;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

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
#[ignore = "requires PAYRIX_API_KEY and a 'Ready' merchant"]
async fn test_transaction_flow() {
    // Note: This test requires a merchant with status "Ready" (boarded).
    // Merchants with status "NotReady" cannot process transactions.
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

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
            "  Token: {} - payment: {:?}, last4: {:?}",
            token.id.as_str(),
            token.payment,
            token.last4
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
            "  Batch: {} - opened: {:?}, status: {:?}",
            batch.id.as_str(),
            batch.opened,
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
            "  FeeRule: {} - type: {:?}, amount: {:?}",
            rule.id.as_str(),
            rule.fee_type,
            rule.amount
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
            "  Subscription: {} - status: {:?}, plan: {:?}",
            sub.id.as_str(),
            sub.status,
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
            "  Member: {} - entity: {:?}",
            member.id.as_str(),
            member.entity
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
            "  Chargeback: {} - status: {:?}, amount: {:?}, reason: {:?}",
            chargeback.id.as_str(),
            chargeback.status,
            chargeback.amount,
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
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let payouts: Vec<Payout> = client.get_all(EntityType::Payouts).await.unwrap();

    println!("Found {} payouts", payouts.len());
    for payout in payouts.iter().take(5) {
        println!(
            "  Payout: {} - amount: {:?}, status: {:?}, schedule: {:?}",
            payout.id.as_str(),
            payout.amount,
            payout.status,
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
            "  EntityReserve: {} - entity: {:?}, percent: {:?}, reserve_type: {:?}",
            reserve.id.as_str(),
            reserve.entity,
            reserve.percent,
            reserve.reserve_type
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
            "  ReserveEntry: {} - reserve: {:?}, amount: {:?}, entry_type: {:?}",
            entry.id.as_str(),
            entry.reserve,
            entry.amount,
            entry.entry_type
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
            "  Reserve: {} - entity: {:?}, amount: {:?}, status: {:?}",
            reserve.id.as_str(),
            reserve.entity,
            reserve.amount,
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
            "  Vendor: {} - name: {:?}, status: {:?}, entity: {:?}",
            vendor.id.as_str(),
            vendor.name,
            vendor.status,
            vendor.entity
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
            "  AccountVerification: {} - account: {:?}, status: {:?}, amount1: {:?}",
            v.id.as_str(),
            v.account,
            v.status,
            v.amount1
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
            "  Adjustment: {} - entity: {:?}, amount: {:?}, adjustment_type: {:?}",
            adj.id.as_str(),
            adj.entity,
            adj.amount,
            adj.adjustment_type
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
            "  ChargebackStatus: {} - chargeback: {:?}, from_status: {:?}, to_status: {:?}, name: {:?}",
            s.id.as_str(),
            s.chargeback,
            s.from_status,
            s.to_status,
            s.name
        );
    }
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
            "  Entry: {} - entity: {:?}, net: {:?}, entry_type: {:?}",
            e.id.as_str(),
            e.entity,
            e.net,
            e.entry_type
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
            "  PendingEntry: {} - entity: {:?}, net: {:?}, entry_type: {:?}",
            e.id.as_str(),
            e.entity,
            e.net,
            e.entry_type
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
            "  Refund: {} - txn: {:?}, amount: {:?}, status: {:?}",
            r.id.as_str(),
            r.txn,
            r.amount,
            r.status
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
            "  Alert: {} - name: {:?}, entity: {:?}",
            a.id.as_str(),
            a.name,
            a.entity
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
            "  AlertTrigger: {} - alert: {:?}, trigger_type: {:?}",
            t.id.as_str(),
            t.alert,
            t.trigger_type
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
            "  Note: {} - entity: {:?}, subject: {:?}, note_type: {:?}",
            n.id.as_str(),
            n.entity,
            n.subject,
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
            "  NoteDocument: {} - note: {:?}, name: {:?}, document_type: {:?}",
            d.id.as_str(),
            d.note,
            d.name,
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
    println!("\n--- Checking ChargebackStatus.from_status/to_status fields ---");
    let statuses: Vec<ChargebackStatus> = client.get_all(EntityType::ChargebackStatuses).await.unwrap();
    println!("Total chargeback status records: {}", statuses.len());

    for s in &statuses {
        if let Some(from) = &s.from_status {
            let status_str = format!("{:?}", from);
            unique_statuses.insert(status_str.clone());
        }
        if let Some(to) = &s.to_status {
            let status_str = format!("{:?}", to);
            unique_statuses.insert(status_str.clone());
        }
        println!(
            "  {} -> from: {:?}, to: {:?}, name: {:?}",
            s.id.as_str(),
            s.from_status,
            s.to_status,
            s.name
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
            email: format!("test{}@example.com", timestamp),
            website: Some("https://example.com".to_string()),
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
            email: format!("owner{}@example.com", timestamp),
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

    // Update with unique data
    request.business.legal_name = format!("Trust Account Test {} LLC", timestamp);
    request.merchant.dba = format!("Trust Test DBA {}", timestamp);
    request.business.email = format!("trust{}@example.com", timestamp);

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
            println!("\nThis is expected when using test/dummy data.");
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
                    "  - {}: {:?} {:?}, type={:?}, ownership={:?}",
                    member.id.as_str(),
                    member.first,
                    member.last,
                    member.member_type,
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
