//! Customer integration tests.

mod common;

use common::{init_logging, TestContext, TEST_MERCHANT_ID};
use payrix::{CreateCustomer, Customer, EntityType, PayrixClient, SearchBuilder, Environment};
use serde_json::json;
use std::env;

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
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

/// Create a customer and validate all response fields match OpenAPI spec.
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
    let new_customer = CreateCustomer {
        merchant: Some(TEST_MERCHANT_ID.parse().unwrap()),
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
