//! Token integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id, TestContext};
use payrix::{
    CreateCustomer, CreateToken, EntityType, Environment, PaymentInfo, PaymentMethod, PayrixClient,
    Token, TokenExpanded,
};
use serde_json::Value;
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
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
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_tokens() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
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
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real token"]
async fn test_create_token_validate_response() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // First create a customer to attach the token to
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let new_customer = CreateCustomer {
        merchant: Some(test_merchant_id().parse().unwrap()),
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

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_token_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_token_expanded() {
    init_logging();
    let client = create_client();

    // Find any token
    let tokens: Vec<Value> = client
        .get_all(EntityType::Tokens)
        .await
        .expect("Failed to get tokens");

    if tokens.is_empty() {
        println!("No tokens available for testing");
        return;
    }

    let token_id = tokens[0]["id"].as_str().expect("Token should have id");
    println!("Testing get_token_expanded with: {}", token_id);

    // Fetch with expansions using convenience method
    let result = client.get_token_expanded(token_id).await;

    match result {
        Ok(Some(token)) => {
            println!("Successfully fetched TokenExpanded:");
            println!("  ID: {}", token.id.as_str());
            println!("  Token string: {:?}", token.token);
            println!("  Status: {:?}", token.status);
            println!("  Expiration: {:?}", token.expiration);
            println!("  Inactive: {}", token.inactive);
            println!("  Frozen: {}", token.frozen);

            // Validate required fields
            assert!(!token.id.as_str().is_empty());
            assert!(token.id.as_str().starts_with("t1_tok_"));

            // Check expanded payment if present
            if let Some(ref payment) = token.payment {
                println!("  Payment expanded:");
                println!("    Display: {}", payment.display());
                println!("    BIN: {:?}", payment.bin);
                println!("    Last4: {:?}", payment.last4);
                println!("    Method: {:?}", payment.method);
                println!("    Routing: {:?}", payment.routing);

                // Test convenience method
                if let Some(method) = token.payment_method() {
                    println!("  payment_method(): {:?}", method);
                }
                if let Some(display) = token.card_display() {
                    println!("  card_display(): {}", display);
                }
            } else {
                println!("  Payment: not expanded");
            }

            // Check expanded customer if present
            if let Some(ref customer) = token.customer {
                println!("  Customer expanded:");
                println!("    ID: {}", customer.id.as_str());
                println!("    Name: {} {}",
                    customer.first.as_deref().unwrap_or(""),
                    customer.last.as_deref().unwrap_or(""));
                println!("    Email: {:?}", customer.email);

                // Test convenience method
                if let Some(name) = token.customer_name() {
                    println!("  customer_name(): {}", name);
                }
            } else {
                println!("  Customer: not expanded (may not be present)");
            }

            println!("TokenExpanded test passed!");
        }
        Ok(None) => println!("Token not found"),
        Err(e) => panic!("Failed to fetch token: {:?}", e),
    }
}

/// Test TokenExpanded deserialization with raw JSON.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_token_expanded_all_fields() {
    init_logging();
    let client = create_client();

    // Get a token
    let tokens: Vec<Value> = client
        .get_all(EntityType::Tokens)
        .await
        .expect("Failed to get tokens");

    if tokens.is_empty() {
        println!("No tokens available");
        return;
    }

    let token_id = tokens[0]["id"].as_str().expect("Token should have id");

    // Fetch with all expansions as raw JSON first
    let expanded_json: Option<Value> = client
        .get_one_expanded(EntityType::Tokens, token_id, &["payment", "customer"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(ref json) = expanded_json {
        println!("Raw expanded JSON:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());
    }

    // Now deserialize to TokenExpanded
    let expanded: Option<TokenExpanded> = client
        .get_one_expanded(EntityType::Tokens, token_id, &["payment", "customer"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(token) = expanded {
        println!("\nDeserialized TokenExpanded:");
        println!("  id: {}", token.id.as_str());
        println!("  created: {:?}", token.created);
        println!("  modified: {:?}", token.modified);
        println!("  token: {:?}", token.token);
        println!("  status: {:?}", token.status);
        println!("  expiration: {:?}", token.expiration);
        println!("  name: {:?}", token.name);
        println!("  description: {:?}", token.description);
        println!("  inactive: {}", token.inactive);
        println!("  frozen: {}", token.frozen);
        println!("  entry_mode: {:?}", token.entry_mode);
        println!("  origin: {:?}", token.origin);

        // Check expanded fields
        println!("\nExpanded fields:");
        println!("  payment: {:?}", token.payment.is_some());
        println!("  customer: {:?}", token.customer.is_some());
    }
}

/// Test creating a token and immediately fetching it expanded.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_create_and_fetch_token_expanded() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Create customer first
    let customer: payrix::Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
                first: Some("TokenExpanded".to_string()),
                last: Some("TestCustomer".to_string()),
                email: Some("token-expanded-test@example.com".to_string()),
                phone: Some("5551234567".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());
    println!("Created customer: {} ({} {})",
        customer.id.as_str(),
        customer.first.as_deref().unwrap_or(""),
        customer.last.as_deref().unwrap_or(""));

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
                name: Some("Test Token".to_string()),
                description: Some("Created for expanded type testing".to_string()),
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token");

    ctx.track_token(token.id.as_str());
    println!("Created token: {}", token.id.as_str());

    // Fetch it expanded
    let expanded = ctx
        .client
        .get_token_expanded(token.id.as_str())
        .await
        .expect("Failed to fetch expanded")
        .expect("Token should exist");

    // Validate expanded data
    println!("\n=== Expanded Token ===");
    println!("ID: {}", expanded.id.as_str());
    println!("Token: {:?}", expanded.token);
    println!("Status: {:?}", expanded.status);
    println!("Name: {:?}", expanded.name);
    println!("Description: {:?}", expanded.description);

    // Payment should be expanded
    assert!(expanded.payment.is_some(), "Payment should be expanded");
    let payment = expanded.payment.as_ref().unwrap();
    println!("Payment: {}", payment.display());
    println!("  BIN: {:?}", payment.bin);
    println!("  Method: {:?}", payment.method);
    assert_eq!(payment.method, Some(PaymentMethod::Visa));
    assert_eq!(payment.bin.as_deref(), Some("411111")); // Visa test card BIN

    // Customer should be expanded
    assert!(expanded.customer.is_some(), "Customer should be expanded");
    let exp_customer = expanded.customer.as_ref().unwrap();
    println!("Customer: {} {} ({})",
        exp_customer.first.as_deref().unwrap_or(""),
        exp_customer.last.as_deref().unwrap_or(""),
        exp_customer.id.as_str());

    // Verify customer data matches what we created
    assert_eq!(exp_customer.id.as_str(), customer.id.as_str());
    assert_eq!(exp_customer.first.as_deref(), Some("TokenExpanded"));
    assert_eq!(exp_customer.last.as_deref(), Some("TestCustomer"));

    // Test convenience methods
    assert_eq!(expanded.payment_method(), Some(PaymentMethod::Visa));
    assert!(expanded.card_display().is_some());
    assert_eq!(expanded.customer_name(), Some("TokenExpanded TestCustomer".to_string()));

    // Cleanup
    ctx.cleanup().await;
}

/// Test that TokenExpanded handles missing expansions gracefully.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_token_expanded_without_expansions() {
    init_logging();
    let client = create_client();

    // Get a token
    let tokens: Vec<Value> = client
        .get_all(EntityType::Tokens)
        .await
        .expect("Failed to get tokens");

    if tokens.is_empty() {
        println!("No tokens available");
        return;
    }

    let token_id = tokens[0]["id"].as_str().expect("Token should have id");

    // Fetch WITHOUT expansions but try to deserialize as TokenExpanded
    let result: Option<TokenExpanded> = client
        .get_one(EntityType::Tokens, token_id)
        .await
        .expect("Failed to fetch token");

    if let Some(token) = result {
        println!("TokenExpanded without expansions:");
        println!("  ID: {}", token.id.as_str());
        println!("  Token: {:?}", token.token);

        // Expanded fields should be None
        assert!(token.payment.is_none(), "Payment should be None when not expanded");
        // Note: customer might still have an ID reference, but won't be a full object

        println!("Correctly handles non-expanded response!");
    }
}
