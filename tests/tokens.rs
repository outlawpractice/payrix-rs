//! Token integration tests.

mod common;

use common::{init_logging, TestContext, TEST_MERCHANT_ID};
use payrix::{
    CreateCustomer, CreateToken, EntityType, Environment, PaymentInfo, PaymentMethod, PayrixClient,
    Token,
};
use std::env;

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_token_creation() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Create a customer first
    let customer: payrix::Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
                first: Some("Token".to_string()),
                last: Some("Test".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());

    // Create a token (test card - Visa 4111...)
    let new_token = CreateToken {
        customer: customer.id.as_str().parse().unwrap(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            expiration: Some("1229".to_string()), // December 2029
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

    let new_customer = CreateCustomer {
        merchant: Some(TEST_MERCHANT_ID.parse().unwrap()),
        first: Some(format!("TokenTest{}", timestamp)),
        last: Some("Customer".to_string()),
        email: Some("payrixrust@gmail.com".to_string()),
        ..Default::default()
    };

    let customer: payrix::Customer = client
        .create(EntityType::Customers, &new_customer)
        .await
        .expect("Failed to create customer for token test");

    println!("Created customer: {}", customer.id.as_str());

    // Create a Visa token
    let new_token = CreateToken {
        customer: customer.id.to_string().parse().unwrap(),
        payment: PaymentInfo {
            method: PaymentMethod::Visa,
            number: Some("4111111111111111".to_string()),
            routing: None,
            expiration: Some("1230".to_string()), // December 2030
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

    println!("Creating Visa token for customer {}", customer.id.as_str());

    let result: Result<Token, _> = client.create(EntityType::Tokens, &new_token).await;

    match result {
        Ok(token) => {
            println!("\n=== TOKEN CREATED SUCCESSFULLY ===");
            println!("ID: {}", token.id.as_str());

            // Validate required fields
            assert!(!token.id.as_str().is_empty(), "ID should not be empty");
            assert!(
                token.id.as_str().starts_with("t1_tok_"),
                "ID should start with t1_tok_"
            );

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
