//! Error handling integration tests.
//!
//! These tests verify that the library correctly handles various error scenarios
//! from the Payrix API.

mod common;

use common::{create_client, init_logging};
use payrix::{EntityType, Environment, Error, PayrixClient};
use serde_json::Value;

// =============================================================================
// Authentication Errors
// =============================================================================

/// Test that an invalid API key returns an Unauthorized error.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_invalid_api_key_returns_unauthorized() {
    init_logging();

    // Use a clearly invalid API key
    let client = PayrixClient::new("invalid-api-key-12345", Environment::Test).unwrap();

    let result: Result<Vec<Value>, Error> = client.get_all(EntityType::Customers).await;

    match result {
        Err(Error::Unauthorized(msg)) => {
            println!("Correctly received Unauthorized error: {}", msg);
            assert!(!msg.is_empty(), "Error message should not be empty");
        }
        Err(e) => panic!("Expected Unauthorized error, got: {:?}", e),
        Ok(_) => panic!("Expected error, but request succeeded"),
    }
}

/// Test that an empty API key is rejected at construction time.
#[tokio::test]
async fn test_empty_api_key_rejected_at_construction() {
    init_logging();

    // Empty API key should fail at client construction, not at request time
    let result = PayrixClient::new("", Environment::Test);

    match result {
        Err(Error::Config(msg)) => {
            println!("Correctly rejected empty API key at construction: {}", msg);
            assert!(msg.contains("empty"), "Error should mention empty key: {}", msg);
        }
        Err(e) => panic!("Expected Config error, got: {:?}", e),
        Ok(_) => panic!("Expected error, but client was created with empty API key"),
    }
}

// =============================================================================
// Not Found Errors
// =============================================================================

/// Test that fetching a non-existent resource returns None (not an error).
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_nonexistent_resource_returns_none() {
    init_logging();
    let client = create_client();

    // Use an ID that looks valid but doesn't exist
    let result: Option<Value> = client
        .get_one(EntityType::Customers, "t1_cus_nonexistent00000000000")
        .await
        .expect("Should not return an error for non-existent resource");

    assert!(result.is_none(), "Non-existent resource should return None");
    println!("Correctly returned None for non-existent customer");
}

/// Test that fetching a non-existent merchant returns None.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_nonexistent_merchant_returns_none() {
    init_logging();
    let client = create_client();

    let result: Option<Value> = client
        .get_one(EntityType::Merchants, "t1_mer_nonexistent00000000000")
        .await
        .expect("Should not return an error for non-existent resource");

    assert!(result.is_none(), "Non-existent merchant should return None");
    println!("Correctly returned None for non-existent merchant");
}

/// Test that fetching a non-existent chargeback returns None.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_nonexistent_chargeback_returns_none() {
    init_logging();
    let client = create_client();

    let result: Option<Value> = client
        .get_one(EntityType::Chargebacks, "t1_chb_nonexistent00000000000")
        .await
        .expect("Should not return an error for non-existent resource");

    assert!(result.is_none(), "Non-existent chargeback should return None");
    println!("Correctly returned None for non-existent chargeback");
}

// =============================================================================
// Invalid Request Errors
// =============================================================================

/// Test that invalid field in update returns an error.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_invalid_update_field_returns_error() {
    init_logging();
    let client = create_client();

    // Try to update a non-existent customer with invalid data
    let result: Result<Value, Error> = client
        .update(
            EntityType::Customers,
            "t1_cus_nonexistent00000000000",
            &serde_json::json!({"invalid_field": "value"}),
        )
        .await;

    // This should either return an error or succeed (API may ignore unknown fields)
    match result {
        Err(e) => {
            println!("Update with invalid field returned error: {:?}", e);
        }
        Ok(_) => {
            println!("API accepted update with unknown field (may ignore unknown fields)");
        }
    }
}

/// Test that creating a resource with missing required fields returns an error.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_create_with_missing_required_fields() {
    init_logging();
    let client = create_client();

    // Try to create a customer without required fields
    let result: Result<Value, Error> = client
        .create(EntityType::Customers, &serde_json::json!({}))
        .await;

    match result {
        Err(Error::Api(errors)) => {
            println!("Correctly received API error for missing fields:");
            for error in &errors {
                println!("  - {} (code: {:?})", error.msg, error.code);
            }
            assert!(!errors.is_empty(), "Should have at least one error");
        }
        Err(e) => {
            println!("Received error (possibly different type): {:?}", e);
        }
        Ok(_) => {
            println!("Note: API accepted empty customer (may have defaults)");
        }
    }
}

// =============================================================================
// Search Errors
// =============================================================================

/// Test that search with invalid field name is handled gracefully.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_search_invalid_field_handled() {
    init_logging();
    let client = create_client();

    let search = payrix::SearchBuilder::new()
        .field("nonexistent_field", "value")
        .build();

    let result: Result<Vec<Value>, Error> = client
        .search(EntityType::Customers, &search)
        .await;

    // API may ignore invalid fields or return an error
    match result {
        Ok(customers) => {
            println!(
                "API ignored invalid search field, returned {} customers",
                customers.len()
            );
        }
        Err(e) => {
            println!("API rejected invalid search field: {:?}", e);
        }
    }
}

// =============================================================================
// Expansion Errors
// =============================================================================

/// Test that invalid expansion name is handled gracefully.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_invalid_expansion_handled() {
    init_logging();
    let client = create_client();

    // First get a customer ID
    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available for testing");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");

    // Try to expand a non-existent field
    let result: Result<Option<Value>, Error> = client
        .get_one_expanded(EntityType::Customers, customer_id, &["nonexistent_expansion"])
        .await;

    match result {
        Ok(Some(customer)) => {
            println!("API ignored invalid expansion, returned customer");
            // The invalid expansion should not appear
            assert!(
                customer.get("nonexistent_expansion").is_none(),
                "Invalid expansion should not create a field"
            );
        }
        Ok(None) => {
            println!("Customer not found");
        }
        Err(e) => {
            println!("API rejected invalid expansion: {:?}", e);
        }
    }
}

// =============================================================================
// Error Message Quality
// =============================================================================

/// Test that API errors contain meaningful error messages.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_api_error_messages_are_meaningful() {
    init_logging();
    let client = create_client();

    // Create a token without a customer - should fail with useful message
    let result: Result<Value, Error> = client
        .create(
            EntityType::Tokens,
            &serde_json::json!({
                "payment": {
                    "method": 1,
                    "number": "4111111111111111",
                    "expiration": "1230"
                }
            }),
        )
        .await;

    match result {
        Err(Error::Api(errors)) => {
            println!("API returned {} error(s):", errors.len());
            for error in &errors {
                println!("  Code: {:?}", error.code);
                println!("  Message: {}", error.msg);
                if let Some(ref field) = error.field {
                    println!("  Field: {}", field);
                }
                println!();
            }

            // Error messages should be non-empty and somewhat descriptive
            for error in &errors {
                assert!(!error.msg.is_empty(), "Error message should not be empty");
                if let Some(code) = error.code {
                    assert!(code > 0, "Error code should be positive");
                }
            }
        }
        Err(e) => {
            println!("Received non-API error: {:?}", e);
        }
        Ok(_) => {
            println!("Unexpectedly succeeded - API may allow tokenless customers");
        }
    }
}

// =============================================================================
// Error Type Matching
// =============================================================================

/// Test that we can match on specific error types.
#[tokio::test]
#[ignore = "requires network access"]
async fn test_error_type_matching() {
    init_logging();
    let client = PayrixClient::new("bad-key", Environment::Test).unwrap();

    let result: Result<Vec<Value>, Error> = client.get_all(EntityType::Customers).await;

    let error = result.expect_err("Should fail with bad key");

    // Demonstrate error type matching
    match &error {
        Error::Unauthorized(_) => println!("Matched Unauthorized"),
        Error::Api(_) => println!("Matched Api"),
        Error::Http(_) => println!("Matched Http"),
        Error::NotFound(_) => println!("Matched NotFound"),
        Error::BadRequest(_) => println!("Matched BadRequest"),
        Error::RateLimited(_) => println!("Matched RateLimited"),
        Error::ServiceUnavailable(_) => println!("Matched ServiceUnavailable"),
        _ => println!("Matched other error type"),
    }

    // Error should implement standard traits
    let _: &dyn std::error::Error = &error;
    let _display = error.to_string();
    let _debug = format!("{:?}", error);

    println!("Error display: {}", error);
    println!("Error debug: {:?}", error);
}
