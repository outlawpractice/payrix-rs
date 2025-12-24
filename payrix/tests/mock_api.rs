//! Mock API tests for offline testing using wiremock.
//!
//! These tests use wiremock to create a mock HTTP server that simulates
//! the Payrix API. This enables fast, reliable, offline testing without
//! needing a real API key or network connection.

mod common;

use payrix::{Config, Environment, EntityType, PayrixClient};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a PayrixClient configured to use a mock server.
fn create_mock_client(mock_server: &MockServer) -> PayrixClient {
    // wiremock returns URI without trailing slash, but Payrix API needs it
    let base_url = format!("{}/", mock_server.uri());

    let config = Config::new("test-api-key", Environment::Test)
        .with_base_url(base_url);

    PayrixClient::with_config(config).expect("Failed to create mock client")
}

/// Create a standard Payrix API response wrapper.
fn payrix_response<T: serde::Serialize>(data: Vec<T>) -> serde_json::Value {
    json!({
        "response": {
            "data": data,
            "details": {
                "requestId": 1,
                "totals": { "total": data.len() },
                "page": { "current": 1, "last": 1, "hasMore": false }
            },
            "errors": []
        }
    })
}

/// Create an empty Payrix API response.
fn empty_response() -> serde_json::Value {
    payrix_response::<serde_json::Value>(vec![])
}

/// Create a Payrix API error response.
fn error_response(code: i32, message: &str) -> serde_json::Value {
    json!({
        "response": {
            "data": [],
            "details": { "requestId": 1 },
            "errors": [{ "code": code, "msg": message }]
        }
    })
}

// =============================================================================
// GET Tests
// =============================================================================

/// Test fetching customers from mock server.
#[tokio::test]
async fn test_get_customers_from_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![
            json!({
                "id": "t1_cus_mock12345678901234567",
                "first": "Test",
                "last": "Customer"
            }),
            json!({
                "id": "t1_cus_mock22345678901234567",
                "first": "Another",
                "last": "Customer"
            }),
        ])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let customers: Vec<serde_json::Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    assert_eq!(customers.len(), 2);
    assert_eq!(customers[0]["first"], "Test");
    assert_eq!(customers[1]["first"], "Another");
}

/// Test fetching a single resource by ID.
#[tokio::test]
async fn test_get_one_from_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/merchants/t1_mer_mock12345678901234567"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_mer_mock12345678901234567",
            "dba": "Mock Merchant",
            "name": "Mock Merchant Inc"
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let merchant: Option<serde_json::Value> = client
        .get_one(EntityType::Merchants, "t1_mer_mock12345678901234567")
        .await
        .expect("Failed to get merchant");

    assert!(merchant.is_some());
    let merchant = merchant.unwrap();
    assert_eq!(merchant["dba"], "Mock Merchant");
}

/// Test that non-existent resource returns None.
#[tokio::test]
async fn test_get_one_not_found_returns_none() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers/t1_cus_nonexistent00000000000"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(empty_response()))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let customer: Option<serde_json::Value> = client
        .get_one(EntityType::Customers, "t1_cus_nonexistent00000000000")
        .await
        .expect("Should not error for non-existent resource");

    assert!(customer.is_none(), "Non-existent resource should return None");
}

// =============================================================================
// HTTP Status Code Error Tests
// =============================================================================

/// Test 400 Bad Request error handling.
#[tokio::test]
async fn test_bad_request_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_response(
            400,
            "Invalid request parameters",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Customers, &json!({"invalid": "data"}))
        .await;

    assert!(result.is_err(), "Should return error for 400 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::BadRequest(_)),
        "Should be BadRequest error, got: {:?}",
        err
    );
}

/// Test 401 Unauthorized error handling.
#[tokio::test]
async fn test_unauthorized_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_response(
            401,
            "Invalid API key",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for 401 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::Unauthorized(_)),
        "Should be Unauthorized error, got: {:?}",
        err
    );
}

/// Test 403 Forbidden error handling.
#[tokio::test]
async fn test_forbidden_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(403).set_body_json(error_response(
            403,
            "Access denied to this resource",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for 403 response");
    let err = result.unwrap_err();
    // 403 typically maps to Unauthorized in our client
    assert!(
        matches!(err, payrix::Error::Unauthorized(_)),
        "Should be Unauthorized error for 403, got: {:?}",
        err
    );
}

/// Test 404 Not Found error handling.
#[tokio::test]
async fn test_not_found_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers/t1_cus_doesnotexist12345678"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_response(
            404,
            "Resource not found",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<Option<serde_json::Value>, _> = client
        .get_one(EntityType::Customers, "t1_cus_doesnotexist12345678")
        .await;

    assert!(result.is_err(), "Should return error for 404 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::NotFound(_)),
        "Should be NotFound error, got: {:?}",
        err
    );
}

/// Test 422 Unprocessable Entity error handling.
#[tokio::test]
async fn test_unprocessable_entity_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(422).set_body_json(error_response(
            422,
            "Cannot process this request",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Customers, &json!({"first": "Test"}))
        .await;

    assert!(result.is_err(), "Should return error for 422 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::UnprocessableEntity(_)),
        "Should be UnprocessableEntity error, got: {:?}",
        err
    );
}

/// Test 429 Rate Limit error handling (HTTP status).
#[tokio::test]
async fn test_rate_limit_http_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(429).set_body_json(error_response(
            429,
            "Rate limit exceeded",
        )))
        .expect(1)
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/", mock_server.uri());
    let mut config = Config::new("test-api-key", Environment::Test)
        .with_base_url(base_url);
    config.max_retries = 0; // Don't retry

    let client = PayrixClient::with_config(config).expect("Failed to create client");
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for 429 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::RateLimited(_)),
        "Should be RateLimited error, got: {:?}",
        err
    );
}

/// Test 500 Internal Server Error handling.
#[tokio::test]
async fn test_internal_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(500).set_body_json(error_response(
            500,
            "Internal server error",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for 500 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::ServiceUnavailable(_)),
        "Should be ServiceUnavailable error for 500, got: {:?}",
        err
    );
}

/// Test 503 Service Unavailable error handling.
#[tokio::test]
async fn test_service_unavailable_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(503).set_body_json(error_response(
            503,
            "Service temporarily unavailable",
        )))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for 503 response");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::ServiceUnavailable(_)),
        "Should be ServiceUnavailable error, got: {:?}",
        err
    );
}

// =============================================================================
// HTTP 200 with Errors in Body Tests (Payrix Quirk)
// =============================================================================

/// Test HTTP 200 with errors at top level of response.
///
/// Payrix sometimes returns HTTP 200 but with errors in the top-level
/// `errors` array: `{ "errors": [...], "response": null }`
#[tokio::test]
async fn test_200_with_top_level_errors() {
    let mock_server = MockServer::start().await;

    // Top-level errors format: { "errors": [...], "response": null }
    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "errors": [
                {
                    "msg": "first is required",
                    "field": "first",
                    "code": 1001,
                    "severity": 2
                }
            ],
            "response": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Customers, &json!({}))
        .await;

    assert!(result.is_err(), "Should return error despite HTTP 200");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::Api(_)),
        "Should be Api error, got: {:?}",
        err
    );

    // Verify error message contains field info
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("first"),
        "Error should mention 'first' field: {}",
        err_msg
    );
}

/// Test HTTP 200 with errors nested inside response object.
///
/// Payrix sometimes returns HTTP 200 with errors inside the response:
/// `{ "response": { "data": [], "errors": [...] } }`
#[tokio::test]
async fn test_200_with_nested_response_errors() {
    let mock_server = MockServer::start().await;

    // Nested errors format: { "response": { "data": [], "errors": [...] } }
    Mock::given(method("POST"))
        .and(path("/tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "response": {
                "data": [],
                "errors": [
                    {
                        "msg": "Invalid card number",
                        "field": "payment.number",
                        "code": 2001,
                        "severity": 2
                    }
                ],
                "details": { "requestId": 12345 }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Tokens, &json!({"payment": {"number": "invalid"}}))
        .await;

    assert!(result.is_err(), "Should return error despite HTTP 200");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::Api(_)),
        "Should be Api error, got: {:?}",
        err
    );

    let err_msg = err.to_string();
    assert!(
        err_msg.contains("Invalid card number"),
        "Error should contain message: {}",
        err_msg
    );
}

/// Test HTTP 200 with multiple errors.
#[tokio::test]
async fn test_200_with_multiple_errors() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "errors": [
                { "msg": "first is required", "field": "first", "code": 1001 },
                { "msg": "last is required", "field": "last", "code": 1002 },
                { "msg": "email format invalid", "field": "email", "code": 1003 }
            ],
            "response": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Customers, &json!({}))
        .await;

    assert!(result.is_err(), "Should return error despite HTTP 200");
    let err = result.unwrap_err();

    if let payrix::Error::Api(errors) = &err {
        assert_eq!(errors.len(), 3, "Should have 3 errors");
        assert_eq!(errors[0].field.as_deref(), Some("first"));
        assert_eq!(errors[1].field.as_deref(), Some("last"));
        assert_eq!(errors[2].field.as_deref(), Some("email"));
    } else {
        panic!("Expected Api error with multiple errors, got: {:?}", err);
    }
}

/// Test HTTP 200 with rate limit error code in body.
///
/// Payrix sometimes returns rate limit as HTTP 200 with a specific error code:
/// `C_RATE_LIMIT_EXCEEDED_TEMP_BLOCK`
///
/// Note: Payrix uses camelCase in JSON responses, so `errorCode` not `error_code`.
#[tokio::test]
async fn test_200_with_rate_limit_in_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "errors": [
                {
                    "msg": "Rate limit exceeded, temporary block",
                    "code": 429,
                    "errorCode": "C_RATE_LIMIT_EXCEEDED_TEMP_BLOCK"
                }
            ],
            "response": null
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/", mock_server.uri());
    let mut config = Config::new("test-api-key", Environment::Test)
        .with_base_url(base_url);
    config.max_retries = 0; // Don't retry

    let client = PayrixClient::with_config(config).expect("Failed to create client");
    let result: Result<Vec<serde_json::Value>, _> = client.get_all(EntityType::Customers).await;

    assert!(result.is_err(), "Should return error for rate limit in body");
    let err = result.unwrap_err();
    assert!(
        matches!(err, payrix::Error::RateLimited(_)),
        "Should be RateLimited error even with HTTP 200, got: {:?}",
        err
    );
}

/// Test HTTP 200 with errorCode (camelCase) but no numeric code.
///
/// Payrix returns errorCode as a string classification (e.g., "Decline", "CARD_DECLINED").
#[tokio::test]
async fn test_200_with_string_error_code() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/txns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "errors": [
                {
                    "msg": "Transaction declined",
                    "errorCode": "CARD_DECLINED",
                    "severity": 2
                }
            ],
            "response": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let result: Result<serde_json::Value, _> = client
        .create(EntityType::Txns, &json!({"merchant": "m1", "type": 1, "total": 1000}))
        .await;

    assert!(result.is_err(), "Should return error despite HTTP 200");
    let err = result.unwrap_err();

    if let payrix::Error::Api(errors) = &err {
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_code.as_deref(), Some("CARD_DECLINED"));
        assert!(errors[0].code.is_none(), "Numeric code should be None");
    } else {
        panic!("Expected Api error, got: {:?}", err);
    }
}

// =============================================================================
// POST (Create) Tests
// =============================================================================

/// Test creating a customer.
#[tokio::test]
async fn test_create_customer_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/customers"))
        .and(header("apikey", "test-api-key"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_cus_newcustomer1234567890",
            "first": "New",
            "last": "Customer"
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let new_customer = json!({
        "first": "New",
        "last": "Customer"
    });

    let result: serde_json::Value = client
        .create(EntityType::Customers, &new_customer)
        .await
        .expect("Failed to create customer");

    assert_eq!(result["first"], "New");
    assert!(result["id"].as_str().unwrap().starts_with("t1_cus_"));
}

// =============================================================================
// Expansion Tests
// =============================================================================

/// Test expanding relationships.
#[tokio::test]
async fn test_expand_token_with_customer() {
    let mock_server = MockServer::start().await;

    // The API uses expand[customer] query parameter
    Mock::given(method("GET"))
        .and(path("/tokens/t1_tok_mock12345678901234567"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_tok_mock12345678901234567",
            "token": "abc123def456",
            "status": "ready",
            "customer": {
                "id": "t1_cus_mock12345678901234567",
                "first": "Expanded",
                "last": "Customer"
            }
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let token: Option<serde_json::Value> = client
        .get_one_expanded(EntityType::Tokens, "t1_tok_mock12345678901234567", &["customer"])
        .await
        .expect("Failed to get expanded token");

    assert!(token.is_some());
    let token = token.unwrap();

    // Verify the expanded customer is an object, not just an ID
    let customer = &token["customer"];
    assert!(customer.is_object(), "customer should be expanded to object");
    assert_eq!(customer["first"], "Expanded");
}

// =============================================================================
// Search Tests
// =============================================================================

/// Test search with filters.
#[tokio::test]
async fn test_search_with_filters() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_cus_searchresult123456789",
            "first": "John",
            "last": "Doe",
            "email": "john@example.com"
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let search = payrix::SearchBuilder::new()
        .field("first", "John")
        .build();

    let customers: Vec<serde_json::Value> = client
        .search(EntityType::Customers, &search)
        .await
        .expect("Failed to search customers");

    assert_eq!(customers.len(), 1);
    assert_eq!(customers[0]["first"], "John");
}

// =============================================================================
// Typed Response Tests
// =============================================================================

/// Test deserializing into typed structures.
#[tokio::test]
async fn test_typed_customer_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/customers"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_cus_typed1234567890123456",
            "first": "Typed",
            "last": "Customer",
            "email": "typed@example.com",
            "inactive": 0,
            "frozen": 0
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let customers: Vec<payrix::Customer> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get typed customers");

    assert_eq!(customers.len(), 1);
    let customer = &customers[0];
    assert_eq!(customer.first.as_deref(), Some("Typed"));
    assert_eq!(customer.last.as_deref(), Some("Customer"));
    assert!(!customer.inactive);
}

/// Test deserializing transaction with required type field.
#[tokio::test]
async fn test_typed_transaction_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/txns"))
        .and(header("apikey", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(payrix_response(vec![json!({
            "id": "t1_txn_typed1234567890123456",
            "type": 1,
            "status": 3,
            "total": 1000,
            "currency": "USD"
        })])))
        .mount(&mock_server)
        .await;

    let client = create_mock_client(&mock_server);
    let transactions: Vec<payrix::Transaction> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get typed transactions");

    assert_eq!(transactions.len(), 1);
    let txn = &transactions[0];
    assert_eq!(txn.total, Some(1000));
}
