//! Customer integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id, TestContext};
use payrix::{
    CreateCustomer, CreateToken, Customer, CustomerExpanded, EntityType, Environment, PaymentInfo,
    PaymentMethod, PayrixClient, SearchBuilder, Token,
};
use serde_json::{json, Value};
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_customer_crud() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // CREATE
    let new_customer = CreateCustomer {
        merchant: Some(ctx.merchant_id.parse().unwrap()),
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
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_customers() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
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

/// Create a customer and validate all response fields match OpenAPI spec.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY - creates real customer"]
async fn test_create_customer_validate_response() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create customer with all fields populated
    let new_customer = CreateCustomer {
        merchant: Some(test_merchant_id().parse().unwrap()),
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

    println!(
        "Creating customer: {} {} ({})",
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
            assert!(
                customer.id.as_str().starts_with("t1_cus_"),
                "ID should start with t1_cus_"
            );

            // Validate optional fields that we set
            assert_eq!(
                customer.first.as_deref(),
                Some(&format!("Test{}", timestamp) as &str)
            );
            assert_eq!(customer.middle.as_deref(), Some("M"));
            assert_eq!(customer.last.as_deref(), Some("Customer"));
            assert_eq!(customer.email.as_deref(), Some("payrixrust@gmail.com"));
            assert_eq!(customer.phone.as_deref(), Some("5551234567"));
            assert_eq!(customer.city.as_deref(), Some("Chicago"));
            assert_eq!(customer.state.as_deref(), Some("IL"));
            assert_eq!(customer.zip.as_deref(), Some("60601"));

            // Validate system-generated fields
            assert!(customer.created.is_some(), "Created timestamp should be set");
            assert!(
                customer.modified.is_some(),
                "Modified timestamp should be set"
            );

            println!("\nAll response fields validated successfully!");
            println!(
                "Customer ID for use in other tests: {}",
                customer.id.as_str()
            );
        }
        Err(e) => {
            panic!("Failed to create customer: {:?}", e);
        }
    }
}

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_customer_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_customer_expanded() {
    init_logging();
    let client = create_client();

    // Find any customer
    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available for testing");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");
    println!("Testing get_customer_expanded with: {}", customer_id);

    // Fetch with expansions using convenience method
    let result = client.get_customer_expanded(customer_id).await;

    match result {
        Ok(Some(customer)) => {
            println!("Successfully fetched CustomerExpanded:");
            println!("  ID: {}", customer.id.as_str());
            println!("  Name: {} {}",
                customer.first.as_deref().unwrap_or(""),
                customer.last.as_deref().unwrap_or(""));
            println!("  Email: {:?}", customer.email);
            println!("  Merchant: {:?}", customer.merchant);
            println!("  Inactive: {}", customer.inactive);

            // Validate required fields
            assert!(!customer.id.as_str().is_empty());
            assert!(customer.id.as_str().starts_with("t1_cus_"));

            // Check expanded tokens if present
            if let Some(ref tokens) = customer.tokens {
                println!("  Tokens expanded: {} token(s)", tokens.len());
                for (i, token) in tokens.iter().take(5).enumerate() {
                    println!("    [{}] {} - {:?}",
                        i,
                        token.id.as_str(),
                        token.status);

                    // Token may also have nested expansions
                    if let Some(ref payment) = token.payment {
                        println!("        Payment: {}", payment.display());
                    }
                }
                if tokens.len() > 5 {
                    println!("    ... and {} more tokens", tokens.len() - 5);
                }
            } else {
                println!("  Tokens: not expanded (customer may have no tokens)");
            }

            // Check expanded invoices if present
            if let Some(ref invoices) = customer.invoices {
                println!("  Invoices expanded: {} invoice(s)", invoices.len());
            } else {
                println!("  Invoices: not expanded");
            }

            println!("CustomerExpanded test passed!");
        }
        Ok(None) => println!("Customer not found"),
        Err(e) => panic!("Failed to fetch customer: {:?}", e),
    }
}

/// Test CustomerExpanded deserialization with raw JSON.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_customer_expanded_all_fields() {
    init_logging();
    let client = create_client();

    // Get a customer
    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");

    // Fetch with expansions as raw JSON first
    let expanded_json: Option<Value> = client
        .get_one_expanded(EntityType::Customers, customer_id, &["tokens", "invoices"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(ref json) = expanded_json {
        println!("Raw expanded JSON:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());
    }

    // Now deserialize to CustomerExpanded
    let expanded: Option<CustomerExpanded> = client
        .get_one_expanded(EntityType::Customers, customer_id, &["tokens", "invoices"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(customer) = expanded {
        println!("\nDeserialized CustomerExpanded:");
        println!("  id: {}", customer.id.as_str());
        println!("  created: {:?}", customer.created);
        println!("  first: {:?}", customer.first);
        println!("  last: {:?}", customer.last);
        println!("  email: {:?}", customer.email);
        println!("  merchant: {:?}", customer.merchant);
        println!("  inactive: {}", customer.inactive);

        // Check expanded fields
        println!("\nExpanded fields:");
        println!("  tokens: {:?}", customer.tokens.as_ref().map(|t| t.len()));
        println!("  invoices: {:?}", customer.invoices.as_ref().map(|i| i.len()));
    }
}

/// Test creating a customer with tokens and fetching it expanded.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_create_customer_with_tokens_and_fetch_expanded() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Create customer
    let customer: Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
                first: Some("CustomerExpanded".to_string()),
                last: Some("TestUser".to_string()),
                email: Some("customer-expanded-test@example.com".to_string()),
                phone: Some("5559876543".to_string()),
                address1: Some("456 Test Avenue".to_string()),
                city: Some("Test City".to_string()),
                state: Some("CA".to_string()),
                zip: Some("90210".to_string()),
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

    // Create first token
    let token1: Token = ctx
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
                name: Some("Primary Card".to_string()),
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token 1");

    ctx.track_token(token1.id.as_str());
    println!("Created token 1: {}", token1.id.as_str());

    // Create second token
    let token2: Token = ctx
        .client
        .create(
            EntityType::Tokens,
            &CreateToken {
                customer: customer.id.as_str().parse().unwrap(),
                payment: PaymentInfo {
                    method: PaymentMethod::Mastercard,
                    number: Some("5555555555554444".to_string()),
                    expiration: Some("1231".to_string()),
                    cvv: Some("456".to_string()),
                    routing: None,
                },
                login: None,
                expiration: None,
                name: Some("Backup Card".to_string()),
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            },
        )
        .await
        .expect("Failed to create token 2");

    ctx.track_token(token2.id.as_str());
    println!("Created token 2: {}", token2.id.as_str());

    // Now fetch customer expanded
    let expanded = ctx
        .client
        .get_customer_expanded(customer.id.as_str())
        .await
        .expect("Failed to fetch expanded")
        .expect("Customer should exist");

    // Validate expanded data
    println!("\n=== Expanded Customer ===");
    println!("ID: {}", expanded.id.as_str());
    println!("Name: {} {}",
        expanded.first.as_deref().unwrap_or(""),
        expanded.last.as_deref().unwrap_or(""));

    // Verify customer data
    assert_eq!(expanded.first.as_deref(), Some("CustomerExpanded"));
    assert_eq!(expanded.last.as_deref(), Some("TestUser"));
    assert_eq!(expanded.email.as_deref(), Some("customer-expanded-test@example.com"));

    // Tokens should be expanded
    assert!(expanded.tokens.is_some(), "Tokens should be expanded");
    let tokens = expanded.tokens.as_ref().unwrap();
    println!("Tokens: {} token(s)", tokens.len());
    assert!(tokens.len() >= 2, "Should have at least 2 tokens");

    // Verify both tokens are present
    let token_ids: Vec<&str> = tokens.iter().map(|t| t.id.as_str()).collect();
    assert!(token_ids.contains(&token1.id.as_str()), "Should contain token 1");
    assert!(token_ids.contains(&token2.id.as_str()), "Should contain token 2");

    // Check token details
    for token in tokens {
        println!("  Token: {} - {:?} - {:?}",
            token.id.as_str(),
            token.name,
            token.status);

        // Tokens should have payment expanded too (nested expansion)
        if let Some(ref payment) = token.payment {
            println!("    Payment: {} ({})",
                payment.display(),
                payment.method.map(|m| format!("{:?}", m)).unwrap_or_default());
        }
    }

    // Cleanup
    ctx.cleanup().await;
}

/// Test that CustomerExpanded handles customers with no tokens.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_customer_expanded_no_tokens() {
    init_logging();
    let mut ctx = TestContext::new().await.expect("Failed to create test context");

    // Create customer without tokens
    let customer: Customer = ctx
        .client
        .create(
            EntityType::Customers,
            &CreateCustomer {
                merchant: Some(ctx.merchant_id.parse().unwrap()),
                first: Some("NoTokens".to_string()),
                last: Some("Customer".to_string()),
                email: Some("no-tokens@example.com".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("Failed to create customer");

    ctx.track_customer(customer.id.as_str());
    println!("Created customer without tokens: {}", customer.id.as_str());

    // Fetch expanded
    let expanded = ctx
        .client
        .get_customer_expanded(customer.id.as_str())
        .await
        .expect("Failed to fetch expanded")
        .expect("Customer should exist");

    println!("CustomerExpanded with no tokens:");
    println!("  ID: {}", expanded.id.as_str());
    println!("  Name: {} {}",
        expanded.first.as_deref().unwrap_or(""),
        expanded.last.as_deref().unwrap_or(""));

    // Tokens should be empty array or None
    if let Some(ref tokens) = expanded.tokens {
        println!("  Tokens: {} (empty array)", tokens.len());
        assert!(tokens.is_empty(), "Should have no tokens");
    } else {
        println!("  Tokens: None");
    }

    // Cleanup
    ctx.cleanup().await;
}

/// Test that CustomerExpanded handles missing expansions gracefully.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_customer_expanded_without_expansions() {
    init_logging();
    let client = create_client();

    // Get a customer
    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");

    // Fetch WITHOUT expansions but deserialize as CustomerExpanded
    let result: Option<CustomerExpanded> = client
        .get_one(EntityType::Customers, customer_id)
        .await
        .expect("Failed to fetch customer");

    if let Some(customer) = result {
        println!("CustomerExpanded without expansions:");
        println!("  ID: {}", customer.id.as_str());
        println!("  Name: {} {}",
            customer.first.as_deref().unwrap_or(""),
            customer.last.as_deref().unwrap_or(""));

        // Expanded fields should be None
        assert!(customer.tokens.is_none(), "Tokens should be None when not expanded");
        assert!(customer.invoices.is_none(), "Invoices should be None when not expanded");

        println!("Correctly handles non-expanded response!");
    }
}
