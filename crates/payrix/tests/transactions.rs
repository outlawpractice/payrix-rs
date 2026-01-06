//! Transaction integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id, TestContext};
use payrix::{
    CreateCustomer, CreateToken, CreateTransaction, EntityType, Environment, PaymentInfo,
    PaymentMethod, PayrixClient, Token, Transaction, TransactionExpanded, TransactionOrigin,
    TransactionType,
};
use serde_json::Value;
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and a boarded merchant"]
async fn test_transaction_flow() {
    // Note: This test uses TEST_MERCHANT_ID which must have status=Boarded.
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Override with our known boarded merchant
    ctx.merchant_id = test_merchant_id();
    println!("Using boarded merchant: {}", ctx.merchant_id);

    // Create customer
    let customer: payrix::Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
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
            &CreateToken {
                customer: customer.id.as_str().parse().unwrap(),
                payment: PaymentInfo {
                    method: PaymentMethod::Visa,
                    number: Some("4111111111111111".to_string()),
                    expiration: Some("1229".to_string()),
                    cvv: Some("123".to_string()),
                    routing: None,
                },
                login: None,
                expiration: None,
                name: None,
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token");

    ctx.track_token(token.id.as_str());

    // Create a transaction (auth + capture)
    // Use the token string value, not the token ID
    let token_string = token.token.expect("Token should have a token string");
    let new_txn = CreateTransaction {
        merchant: ctx.merchant_id.clone(),
        token: Some(token_string),
        txn_type: TransactionType::CreditCardSale,
        origin: Some(TransactionOrigin::Ecommerce),
        total: 1000, // $10.00
        cof_type: None,
        description: None,
        fortxn: None,
        fee_id: None,
        allow_partial: None,
        client_ip: None,
        first: None,
        middle: None,
        last: None,
        inactive: None,
        frozen: None,
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
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_transactions() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
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

/// Create a transaction and validate response fields match OpenAPI spec.
/// NOTE: Requires TEST_MERCHANT_ID to have status=Boarded.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and TEST_MERCHANT_ID to be Boarded"]
async fn test_create_transaction_validate_response() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // First create customer and token
    let new_customer = CreateCustomer {
        merchant: Some(test_merchant_id().parse().unwrap()),
        first: Some(format!("TxnTest{}", timestamp)),
        last: Some("Customer".to_string()),
        email: Some("payrixrust@gmail.com".to_string()),
        ..Default::default()
    };

    let customer: payrix::Customer = client
        .create(EntityType::Customers, &new_customer)
        .await
        .expect("Failed to create customer");

    let new_token = CreateToken {
        customer: customer.id.to_string().parse().unwrap(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            routing: None,
            expiration: Some("1230".to_string()),
            cvv: Some("123".to_string()),
        },
        login: None,
        expiration: None,
        name: None,
        description: None,
        custom: None,
        inactive: None,
        frozen: None,
    };

    let token: Token = client
        .create(EntityType::Tokens, &new_token)
        .await
        .expect("Failed to create token");

    println!(
        "Created customer {} and token {}",
        customer.id.as_str(),
        token.id.as_str()
    );

    // Create a $1.00 transaction using the token's transaction token string
    let token_string = token.token.expect("Token should have a token string");
    let new_txn = CreateTransaction {
        merchant: test_merchant_id(),
        token: Some(token_string),
        txn_type: TransactionType::CreditCardSale,
        total: 100, // $1.00 in cents
        origin: Some(TransactionOrigin::Ecommerce),
        cof_type: None,
        description: None,
        fortxn: None,
        fee_id: None,
        allow_partial: None,
        client_ip: None,
        first: None,
        middle: None,
        last: None,
        inactive: None,
        frozen: None,
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
            assert!(
                txn.id.as_str().starts_with("t1_txn_"),
                "ID should start with t1_txn_"
            );

            // Validate amounts
            assert_eq!(txn.total, Some(100), "Total should be 100 cents");

            // Validate enums
            assert!(txn.status.is_some(), "Status should be set");
            println!("Transaction type: {:?}", txn.txn_type);

            // Validate timestamps
            assert!(txn.created.is_some(), "Created timestamp should be set");

            println!("\nAll response fields validated successfully!");
            println!("Transaction ID: {}", txn.id.as_str());
        }
        Err(e) => {
            println!("\n=== TRANSACTION CREATION FAILED ===");
            println!("Error: {:?}", e);
            panic!(
                "Transaction creation failed - this documents an API inconsistency: {:?}",
                e
            );
        }
    }
}

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_transaction_full()` convenience method with all expansions.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_transaction_full() {
    init_logging();
    let client = create_client();

    // First, find any transaction
    let txns: Vec<Value> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get transactions");

    if txns.is_empty() {
        println!("No transactions available for testing");
        return;
    }

    // Prefer a transaction with a token for more complete testing
    let txn_json = txns
        .iter()
        .find(|t| t.get("token").map(|v| !v.is_null()).unwrap_or(false))
        .or_else(|| txns.first())
        .expect("Should have at least one transaction");

    let txn_id = txn_json["id"].as_str().expect("Transaction should have id");
    println!("Testing get_transaction_full with: {}", txn_id);

    // Fetch with expansions using convenience method
    let result = client.get_transaction_full(txn_id).await;

    match result {
        Ok(Some(txn)) => {
            println!("Successfully fetched TransactionExpanded:");
            println!("  ID: {}", txn.id.as_str());
            println!("  Amount: ${:.2}", txn.amount_dollars());
            println!("  Status: {:?}", txn.status);
            println!("  Type: {:?}", txn.txn_type);

            // Validate required fields
            assert!(!txn.id.as_str().is_empty());
            assert!(txn.id.as_str().starts_with("t1_txn_"));

            // Check expanded payment if present
            if let Some(ref payment) = txn.payment {
                println!("  Payment: {}", payment.display());
                println!("    BIN: {:?}", payment.bin);
                println!("    Method: {:?}", payment.method);
                assert!(payment.is_card() || payment.is_bank_account() || payment.method.is_none());
            } else {
                println!("  Payment: not expanded (may not be present on this transaction)");
            }

            // Check expanded token with nested customer
            if let Some(ref token) = txn.token {
                println!("  Token ID: {}", token.id.as_str());
                println!("  Token string: {:?}", token.token);
                println!("  Token status: {:?}", token.status);

                // Check nested customer ID (not expanded)
                if let Some(customer_id) = token.customer_id() {
                    println!("  Customer ID (nested in token): {}", customer_id);
                } else {
                    println!("  Customer: not present in token");
                }
            } else {
                println!("  Token: not expanded (transaction may not have a token)");
            }

            // Check merchant ID (not expanded)
            if let Some(merchant_id) = txn.merchant.as_ref().map(|m| m.as_str()) {
                println!("  Merchant ID: {}", merchant_id);
            }

            // Test convenience methods
            if let Some(display) = txn.payment_display() {
                println!("  payment_display(): {}", display);
            }
            if let Some(name) = txn.customer_name() {
                println!("  customer_name(): {}", name);
            }

            println!("TransactionExpanded test passed!");
        }
        Ok(None) => println!("Transaction not found (deleted?)"),
        Err(e) => panic!("Failed to fetch transaction: {:?}", e),
    }
}

/// Test TransactionExpanded deserialization with a real API response.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_transaction_expanded_all_fields() {
    init_logging();
    let client = create_client();

    // Get a transaction as raw JSON to see what fields exist
    let txns: Vec<Value> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get transactions");

    if txns.is_empty() {
        println!("No transactions available");
        return;
    }

    let txn_id = txns[0]["id"].as_str().expect("Transaction should have id");

    // Fetch with all possible expansions
    let expanded_json: Option<Value> = client
        .get_one_expanded(
            EntityType::Txns,
            txn_id,
            &["payment", "token|customer", "subscription", "merchant"],
        )
        .await
        .expect("Failed to fetch expanded");

    if let Some(ref json) = expanded_json {
        println!("Raw expanded JSON:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());
    }

    // Now deserialize to TransactionExpanded
    let expanded: Option<TransactionExpanded> = client
        .get_one_expanded(
            EntityType::Txns,
            txn_id,
            &["payment", "token|customer", "subscription", "merchant"],
        )
        .await
        .expect("Failed to fetch expanded");

    if let Some(txn) = expanded {
        // Verify we can access all the standard fields
        println!("\nDeserialized TransactionExpanded:");
        println!("  id: {}", txn.id.as_str());
        println!("  created: {:?}", txn.created);
        println!("  status: {:?}", txn.status);
        println!("  total: {:?} (${:.2})", txn.total, txn.amount_dollars());
        println!("  approved: {:?}", txn.approved);
        println!("  currency: {:?}", txn.currency);
        println!("  inactive: {}", txn.inactive);
        println!("  frozen: {}", txn.frozen);

        // Check that expanded fields deserialize correctly
        println!("\nExpanded fields:");
        println!("  payment: {:?}", txn.payment.is_some());
        println!("  token: {:?}", txn.token.is_some());
        println!("  merchant: {:?}", txn.merchant.is_some());
        println!("  subscription: {:?}", txn.subscription.is_some());
    }
}

/// Test creating a transaction and immediately fetching it expanded.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and TEST_MERCHANT_ID to be Boarded"]
async fn test_create_and_fetch_transaction_expanded() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");
    ctx.merchant_id = test_merchant_id();

    // Create customer
    let customer: payrix::Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
                first: Some("ExpandedTest".to_string()),
                last: Some("Customer".to_string()),
                email: Some("expanded-test@example.com".to_string()),
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
            &CreateToken {
                customer: customer.id.as_str().parse().unwrap(),
                payment: PaymentInfo {
                    method: PaymentMethod::Visa,
                    number: Some("4111111111111111".to_string()),
                    expiration: Some("1230".to_string()),
                    cvv: Some("123".to_string()),
                    routing: None,
                },
                login: None,
                expiration: None,
                name: None,
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token");

    ctx.track_token(token.id.as_str());

    // Create transaction
    let token_string = token.token.expect("Token should have a token string");
    let txn: Transaction = ctx
        .client
        .create(
            EntityType::Txns,
            &CreateTransaction {
                merchant: ctx.merchant_id.clone(),
                token: Some(token_string),
                txn_type: TransactionType::CreditCardSale,
                origin: Some(TransactionOrigin::Ecommerce),
                total: 1500, // $15.00
                cof_type: None,
                description: Some("Expanded test transaction".to_string()),
                fortxn: None,
                fee_id: None,
                allow_partial: None,
                client_ip: None,
                first: None,
                middle: None,
                last: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create transaction");

    ctx.track_transaction(txn.id.as_str());
    println!("Created transaction: {}", txn.id.as_str());

    // Now fetch it with expansions
    let expanded = ctx
        .client
        .get_transaction_full(txn.id.as_str())
        .await
        .expect("Failed to fetch expanded")
        .expect("Transaction should exist");

    // Validate that expansions worked
    println!("\n=== Expanded Transaction ===");
    println!("ID: {}", expanded.id.as_str());
    println!("Total: ${:.2}", expanded.amount_dollars());

    // Payment should be expanded
    assert!(expanded.payment.is_some(), "Payment should be expanded");
    let payment = expanded.payment.as_ref().unwrap();
    println!("Payment: {} (BIN: {:?})", payment.display(), payment.bin);
    assert_eq!(payment.method, Some(PaymentMethod::Visa));

    // Token should be expanded
    assert!(expanded.token.is_some(), "Token should be expanded");
    let exp_token = expanded.token.as_ref().unwrap();
    println!("Token: {} - {:?}", exp_token.id.as_str(), exp_token.status);

    // Customer ID should be present in token (not expanded - just an ID)
    assert!(exp_token.customer.is_some(), "Customer ID should be in token");
    let customer_id = exp_token.customer_id().unwrap();
    println!("Customer ID: {}", customer_id);

    // Verify convenience methods (customer_name comes from txn.first/last fields)
    assert!(expanded.customer_name().is_some(), "customer_name() should work from first/last fields");
    assert!(expanded.payment_display().is_some());

    // Cleanup
    ctx.cleanup().await;
}
