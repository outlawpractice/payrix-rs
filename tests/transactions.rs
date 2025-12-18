//! Transaction integration tests.

mod common;

use common::{init_logging, TestContext, TEST_MERCHANT_ID};
use payrix::{
    CreateCustomer, CreateToken, CreateTransaction, EntityType, Environment, PaymentInfo, PaymentMethod,
    PayrixClient, Token, Transaction, TransactionOrigin, TransactionType,
};
use std::env;

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

/// Create a transaction and validate response fields match OpenAPI spec.
/// NOTE: Requires TEST_MERCHANT_ID to have status=Boarded.
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
    let new_customer = CreateCustomer {
        merchant: Some(TEST_MERCHANT_ID.parse().unwrap()),
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
        merchant: TEST_MERCHANT_ID.to_string(),
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
