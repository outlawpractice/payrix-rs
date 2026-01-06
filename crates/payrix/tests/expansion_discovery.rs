//! Discovery tests for API expansion responses.
//!
//! These tests help us understand what the Payrix API actually returns when
//! using the `expand` query parameter. This is critical because the API
//! may not match OpenAPI documentation.
//!
//! Run with: `cargo test --test expansion_discovery -- --ignored --nocapture`

mod common;

use common::{create_client, init_logging};
use payrix::EntityType;
use serde_json::Value;

/// Helper to print expansion field info
fn print_expansion_info(obj: &serde_json::Map<String, Value>, keys: &[&str]) {
    println!("\n=== Expansion Field Summary ===");
    for key in keys {
        match obj.get(*key) {
            Some(Value::Null) => println!("  {}: null", key),
            Some(Value::Object(o)) => println!("  {}: OBJECT with {} fields", key, o.len()),
            Some(Value::Array(arr)) => println!("  {}: ARRAY with {} items", key, arr.len()),
            Some(Value::Number(n)) => println!("  {}: NUMBER = {}", key, n),
            Some(Value::String(s)) => println!("  {}: STRING = \"{}\"", key, s),
            Some(Value::Bool(b)) => println!("  {}: BOOL = {}", key, b),
            None => println!("  {}: NOT PRESENT", key),
        }
    }
    println!("================================\n");
}

/// Helper to print first N fields of an object
fn print_object_preview(obj: &serde_json::Map<String, Value>, name: &str) {
    println!("\n--- {} fields ---", name);
    for (key, value) in obj.iter().take(30) {
        let preview = match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => format!("{}", b),
            Value::Number(n) => format!("{}", n),
            Value::String(s) if s.len() > 50 => format!("\"{}...\"", &s[..50]),
            Value::String(s) => format!("\"{}\"", s),
            Value::Array(arr) => format!("[{} items]", arr.len()),
            Value::Object(o) => format!("{{...{} fields}}", o.len()),
        };
        println!("  {}: {}", key, preview);
    }
    if obj.len() > 30 {
        println!("  ... and {} more fields", obj.len() - 30);
    }
}

// =============================================================================
// Transaction Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_transaction_expansions() {
    init_logging();
    let client = create_client();

    // First, find a transaction that has relationships we can expand
    println!("Finding a transaction with token...");

    let txns: Vec<Value> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get transactions");

    // Find a transaction that has a token (for expansion testing)
    let txn_with_token = txns.iter().find(|t| {
        t.get("token").map(|v| !v.is_null()).unwrap_or(false)
    });

    let Some(txn) = txn_with_token else {
        println!("No transaction with token found. Trying first available...");
        if txns.is_empty() {
            println!("No transactions available for testing");
            return;
        }
        return;
    };

    let txn_id = txn["id"].as_str().expect("Transaction should have id");
    println!("Using transaction: {}", txn_id);

    // Now fetch with expansions
    println!("\nFetching with expansions: payment, token, token|customer, subscription, merchant");

    let expanded: Option<Value> = client
        .get_one_expanded(
            EntityType::Txns,
            txn_id,
            &["payment", "token", "token|customer", "subscription", "merchant"],
        )
        .await
        .expect("Failed to fetch expanded transaction");

    let expanded = expanded.expect("Transaction should exist");
    let obj = expanded.as_object().expect("Response should be an object");

    // ASSERTION: Response must have an id field
    assert!(obj.get("id").is_some(), "Expanded response must have id");

    print_expansion_info(obj, &[
        "payment", "token", "customer", "subscription", "merchant"
    ]);

    // ASSERTIONS: Expanded fields should be objects (not strings/numbers)
    if let Some(payment) = obj.get("payment") {
        if !payment.is_null() {
            assert!(
                payment.is_object() || payment.is_number(),
                "payment should be object when expanded, or number when not"
            );
            if let Value::Object(payment_obj) = payment {
                print_object_preview(payment_obj, "payment");
                // Payment should have method field when expanded
                assert!(
                    payment_obj.get("method").is_some(),
                    "Expanded payment should have method"
                );
            }
        }
    }

    if let Some(token) = obj.get("token") {
        if !token.is_null() {
            assert!(
                token.is_object() || token.is_string(),
                "token should be object when expanded, or string when not"
            );
            if let Value::Object(token_obj) = token {
                print_object_preview(token_obj, "token");
                // Token should have id when expanded
                assert!(
                    token_obj.get("id").is_some(),
                    "Expanded token should have id"
                );
            }
        }
    }

    if let Some(Value::Object(merchant)) = obj.get("merchant") {
        print_object_preview(merchant, "merchant");
        // Merchant should have id when expanded
        assert!(
            merchant.get("id").is_some(),
            "Expanded merchant should have id"
        );
    }

    println!("\n=== Transaction expansion test PASSED ===");
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_txn_results_expansion() {
    init_logging();
    let client = create_client();

    // Find a transaction
    let txns: Vec<Value> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get transactions");

    if txns.is_empty() {
        println!("No transactions available");
        return;
    }

    let txn_id = txns[0]["id"].as_str().expect("Transaction should have id");
    println!("Testing txnResults expansion on transaction: {}", txn_id);

    // Try expanding txnResults (might not work)
    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Txns, txn_id, &["txnResults"])
        .await
        .expect("Failed to fetch");

    if let Some(Value::Object(obj)) = expanded {
        match obj.get("txnResults") {
            Some(Value::Array(results)) => {
                println!("txnResults: ARRAY with {} items", results.len());
                if let Some(first) = results.first() {
                    println!("First txnResult: {}", serde_json::to_string_pretty(first).unwrap());
                }
            }
            Some(other) => println!("txnResults: unexpected type {:?}", other),
            None => println!("txnResults: NOT PRESENT (expansion might not be supported)"),
        }
    }
}

// =============================================================================
// Token Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_token_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a token...");

    let tokens: Vec<Value> = client
        .get_all(EntityType::Tokens)
        .await
        .expect("Failed to get tokens");

    if tokens.is_empty() {
        println!("No tokens available for testing");
        return;
    }

    let token_id = tokens[0]["id"].as_str().expect("Token should have id");
    println!("Using token: {}", token_id);

    // Fetch with expansions
    println!("\nFetching with expansions: payment, customer");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Tokens, token_id, &["payment", "customer"])
        .await
        .expect("Failed to fetch expanded token");

    let expanded = expanded.expect("Token should exist");
    let obj = expanded.as_object().expect("Response should be an object");

    // ASSERTION: Response must have an id field
    assert!(obj.get("id").is_some(), "Expanded response must have id");

    // ASSERTION: ID should have proper format
    let id = obj["id"].as_str().expect("id should be string");
    assert!(id.starts_with("t1_tok_"), "Token ID should start with t1_tok_");

    print_expansion_info(obj, &["payment", "customer"]);

    // ASSERTIONS: Check payment expansion
    if let Some(payment) = obj.get("payment") {
        if !payment.is_null() {
            if let Value::Object(payment_obj) = payment {
                print_object_preview(payment_obj, "payment");
                // Expanded payment should have method field
                assert!(
                    payment_obj.get("method").is_some(),
                    "Expanded payment should have method"
                );
            } else {
                // payment might be just an integer when not expanded
                println!("\npayment (non-object): {:?}", payment);
            }
        }
    }

    // ASSERTIONS: Check customer expansion
    if let Some(customer) = obj.get("customer") {
        if !customer.is_null() && customer.is_object() {
            let customer_obj = customer.as_object().unwrap();
            print_object_preview(customer_obj, "customer");
            // Expanded customer should have id
            assert!(
                customer_obj.get("id").is_some(),
                "Expanded customer should have id"
            );
        }
    }

    println!("\n=== Token expansion test PASSED ===");
}

// =============================================================================
// Customer Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_customer_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a customer with tokens...");

    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available for testing");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");
    println!("Using customer: {}", customer_id);

    // Fetch with expansions
    println!("\nFetching with expansions: tokens, invoices");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Customers, customer_id, &["tokens", "invoices"])
        .await
        .expect("Failed to fetch expanded customer");

    let expanded = expanded.expect("Customer should exist");
    let obj = expanded.as_object().expect("Response should be an object");

    // ASSERTION: Response must have an id field
    assert!(obj.get("id").is_some(), "Expanded response must have id");

    // ASSERTION: ID should have proper format
    let id = obj["id"].as_str().expect("id should be string");
    assert!(id.starts_with("t1_cus_"), "Customer ID should start with t1_cus_");

    print_expansion_info(obj, &["tokens", "invoices"]);

    // ASSERTIONS: Check tokens expansion
    if let Some(tokens) = obj.get("tokens") {
        if !tokens.is_null() {
            if let Value::Array(tokens_arr) = tokens {
                println!("\ntokens: ARRAY with {} items", tokens_arr.len());
                if let Some(first) = tokens_arr.first() {
                    if let Value::Object(t) = first {
                        print_object_preview(t, "tokens[0]");
                        // Each expanded token should have an id
                        assert!(t.get("id").is_some(), "Expanded token should have id");
                    }
                }
            }
        }
    }

    println!("\n=== Customer expansion test PASSED ===");
}

// =============================================================================
// Subscription Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_subscription_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a subscription...");

    let subs: Vec<Value> = client
        .get_all(EntityType::Subscriptions)
        .await
        .expect("Failed to get subscriptions");

    if subs.is_empty() {
        println!("No subscriptions available for testing");
        return;
    }

    let sub_id = subs[0]["id"].as_str().expect("Subscription should have id");
    println!("Using subscription: {}", sub_id);

    // Fetch with expansions - try common relationships
    println!("\nFetching with expansions: token, customer, merchant, plan");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Subscriptions, sub_id, &["token", "customer", "merchant", "plan"])
        .await
        .expect("Failed to fetch expanded subscription");

    if let Some(Value::Object(obj)) = expanded {
        println!("\n=== FULL EXPANDED SUBSCRIPTION ===");
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());

        print_expansion_info(&obj, &["token", "customer", "merchant", "plan"]);
    } else {
        println!("Subscription not found or empty response");
    }
}

// =============================================================================
// Merchant Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_merchant_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a merchant...");

    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available for testing");
        return;
    }

    let merchant_id = merchants[0]["id"].as_str().expect("Merchant should have id");
    println!("Using merchant: {}", merchant_id);

    // Try various expansions
    println!("\nFetching with expansions: entities, members, org, parent");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &["entities", "members", "org", "parent"])
        .await
        .expect("Failed to fetch expanded merchant");

    if let Some(Value::Object(obj)) = expanded {
        println!("\n=== FULL EXPANDED MERCHANT ===");
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());

        print_expansion_info(&obj, &["entities", "members", "org", "parent"]);
    } else {
        println!("Merchant not found or empty response");
    }
}

// =============================================================================
// Chargeback Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_chargeback_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a chargeback...");

    let chargebacks: Vec<Value> = client
        .get_all(EntityType::Chargebacks)
        .await
        .expect("Failed to get chargebacks");

    if chargebacks.is_empty() {
        println!("No chargebacks available for testing");
        return;
    }

    let chargeback_id = chargebacks[0]["id"].as_str().expect("Chargeback should have id");
    println!("Using chargeback: {}", chargeback_id);

    // Try various expansions
    println!("\nFetching with expansions: txn, merchant, token");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Chargebacks, chargeback_id, &["txn", "merchant", "token"])
        .await
        .expect("Failed to fetch expanded chargeback");

    if let Some(Value::Object(obj)) = expanded {
        println!("\n=== FULL EXPANDED CHARGEBACK ===");
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());

        print_expansion_info(&obj, &["txn", "merchant", "token"]);
    } else {
        println!("Chargeback not found or empty response");
    }
}

// =============================================================================
// Plan Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_plan_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a plan...");

    let plans: Vec<Value> = client
        .get_all(EntityType::Plans)
        .await
        .expect("Failed to get plans");

    if plans.is_empty() {
        println!("No plans available for testing");
        return;
    }

    let plan_id = plans[0]["id"].as_str().expect("Plan should have id");
    println!("Using plan: {}", plan_id);

    // Try various expansions
    println!("\nFetching with expansions: merchant, subscriptions");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Plans, plan_id, &["merchant", "subscriptions"])
        .await
        .expect("Failed to fetch expanded plan");

    if let Some(Value::Object(obj)) = expanded {
        println!("\n=== FULL EXPANDED PLAN ===");
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());

        print_expansion_info(&obj, &["merchant", "subscriptions"]);
    } else {
        println!("Plan not found or empty response");
    }
}

// =============================================================================
// Batch Expansion Discovery
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn discover_batch_expansions() {
    init_logging();
    let client = create_client();

    println!("Finding a batch...");

    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    if batches.is_empty() {
        println!("No batches available for testing");
        return;
    }

    let batch_id = batches[0]["id"].as_str().expect("Batch should have id");
    println!("Using batch: {}", batch_id);

    // Try various expansions
    println!("\nFetching with expansions: merchant, txns");

    let expanded: Option<Value> = client
        .get_one_expanded(EntityType::Batches, batch_id, &["merchant", "txns"])
        .await
        .expect("Failed to fetch expanded batch");

    if let Some(Value::Object(obj)) = expanded {
        println!("\n=== FULL EXPANDED BATCH ===");
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());

        print_expansion_info(&obj, &["merchant", "txns"]);
    } else {
        println!("Batch not found or empty response");
    }
}

// =============================================================================
// Integration Tests for Expanded Types
// =============================================================================

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_transaction_full() {
    init_logging();
    let client = create_client();

    // Find a transaction with token
    let txns: Vec<Value> = client
        .get_all(EntityType::Txns)
        .await
        .expect("Failed to get transactions");

    let txn_with_token = txns.iter().find(|t| {
        t.get("token").map(|v| !v.is_null()).unwrap_or(false)
    });

    let Some(txn) = txn_with_token else {
        println!("No transaction with token found");
        return;
    };

    let txn_id = txn["id"].as_str().expect("Transaction should have id");
    println!("Testing get_transaction_full with: {}", txn_id);

    // Use the convenience method
    let result = client.get_transaction_full(txn_id).await;

    match result {
        Ok(Some(txn)) => {
            println!("Successfully fetched TransactionExpanded");
            println!("  ID: {}", txn.id.as_str());
            println!("  Amount: ${:.2}", txn.amount_dollars());
            println!("  Status: {:?}", txn.status);

            if let Some(ref payment) = txn.payment {
                println!("  Payment: {}", payment.display());
            }

            if let Some(ref token) = txn.token {
                println!("  Token: {}", token.token.as_deref().unwrap_or("N/A"));
                if let Some(customer_id) = token.customer_id() {
                    println!("  Customer ID: {}", customer_id);
                }
            }

            if let Some(merchant_id) = txn.merchant.as_ref().map(|m| m.as_str()) {
                println!("  Merchant ID: {}", merchant_id);
            }
        }
        Ok(None) => println!("Transaction not found"),
        Err(e) => panic!("Failed to fetch transaction: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_token_expanded() {
    init_logging();
    let client = create_client();

    let tokens: Vec<Value> = client
        .get_all(EntityType::Tokens)
        .await
        .expect("Failed to get tokens");

    if tokens.is_empty() {
        println!("No tokens available");
        return;
    }

    let token_id = tokens[0]["id"].as_str().expect("Token should have id");
    println!("Testing get_token_expanded with: {}", token_id);

    let result = client.get_token_expanded(token_id).await;

    match result {
        Ok(Some(token)) => {
            println!("Successfully fetched TokenExpanded");
            println!("  ID: {}", token.id.as_str());
            println!("  Token: {}", token.token.as_deref().unwrap_or("N/A"));
            println!("  Status: {:?}", token.status);

            if let Some(ref payment) = token.payment {
                println!("  Payment: {}", payment.display());
                println!("  BIN: {}", payment.bin.as_deref().unwrap_or("N/A"));
            }

            if let Some(customer_id) = token.customer_id() {
                println!("  Customer ID: {}", customer_id);
            }
        }
        Ok(None) => println!("Token not found"),
        Err(e) => panic!("Failed to fetch token: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_customer_expanded() {
    init_logging();
    let client = create_client();

    let customers: Vec<Value> = client
        .get_all(EntityType::Customers)
        .await
        .expect("Failed to get customers");

    if customers.is_empty() {
        println!("No customers available");
        return;
    }

    let customer_id = customers[0]["id"].as_str().expect("Customer should have id");
    println!("Testing get_customer_expanded with: {}", customer_id);

    let result = client.get_customer_expanded(customer_id).await;

    match result {
        Ok(Some(customer)) => {
            println!("Successfully fetched CustomerExpanded");
            println!("  ID: {}", customer.id.as_str());
            println!("  Name: {} {}",
                customer.first.as_deref().unwrap_or(""),
                customer.last.as_deref().unwrap_or(""));

            if let Some(ref tokens) = customer.tokens {
                println!("  Tokens: {} token(s)", tokens.len());
                for (i, token) in tokens.iter().take(3).enumerate() {
                    println!("    [{}] {} - {:?}",
                        i,
                        token.id.as_str(),
                        token.status);
                }
            }
        }
        Ok(None) => println!("Customer not found"),
        Err(e) => panic!("Failed to fetch customer: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_subscription_expanded() {
    init_logging();
    let client = create_client();

    let subs: Vec<Value> = client
        .get_all(EntityType::Subscriptions)
        .await
        .expect("Failed to get subscriptions");

    if subs.is_empty() {
        println!("No subscriptions available");
        return;
    }

    let sub_id = subs[0]["id"].as_str().expect("Subscription should have id");
    println!("Testing get_subscription_expanded with: {}", sub_id);

    let result = client.get_subscription_expanded(sub_id).await;

    match result {
        Ok(Some(sub)) => {
            println!("Successfully fetched SubscriptionExpanded");
            println!("  ID: {}", sub.id.as_str());
            println!("  Start: {:?}", sub.start);
            println!("  Failures: {:?}", sub.failures);

            if let Some(ref plan) = sub.plan {
                println!("  Plan expanded:");
                println!("    ID: {}", plan.id.as_str());
                println!("    Name: {:?}", plan.name);
                println!("    Amount: ${:.2}", plan.amount.unwrap_or(0) as f64 / 100.0);
                println!("    Schedule: {:?}", plan.schedule);
            }

            // Test convenience methods
            if let Some(dollars) = sub.plan_amount_dollars() {
                println!("  plan_amount_dollars(): ${:.2}", dollars);
            }
            if let Some(name) = sub.plan_name() {
                println!("  plan_name(): {}", name);
            }
        }
        Ok(None) => println!("Subscription not found"),
        Err(e) => panic!("Failed to fetch subscription: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_plan_expanded() {
    init_logging();
    let client = create_client();

    let plans: Vec<Value> = client
        .get_all(EntityType::Plans)
        .await
        .expect("Failed to get plans");

    if plans.is_empty() {
        println!("No plans available");
        return;
    }

    let plan_id = plans[0]["id"].as_str().expect("Plan should have id");
    println!("Testing get_plan_expanded with: {}", plan_id);

    let result = client.get_plan_expanded(plan_id).await;

    match result {
        Ok(Some(plan)) => {
            println!("Successfully fetched PlanExpanded");
            println!("  ID: {}", plan.id.as_str());
            println!("  Name: {:?}", plan.name);
            println!("  Amount: ${:.2}", plan.amount_dollars());
            println!("  Schedule: {:?}", plan.schedule);
            println!("  Type: {:?}", plan.plan_type);

            // Merchant is an ID, not expanded
            if let Some(ref merchant) = plan.merchant {
                println!("  Merchant ID: {}", merchant.as_str());
            }

            if let Some(ref subs) = plan.subscriptions {
                println!("  Subscriptions: {} total", subs.len());
                println!("  Active subscriptions: {}", plan.subscription_count());
            }
        }
        Ok(None) => println!("Plan not found"),
        Err(e) => panic!("Failed to fetch plan: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_chargeback_expanded() {
    init_logging();
    let client = create_client();

    let chargebacks: Vec<Value> = client
        .get_all(EntityType::Chargebacks)
        .await
        .expect("Failed to get chargebacks");

    if chargebacks.is_empty() {
        println!("No chargebacks available");
        return;
    }

    let cb_id = chargebacks[0]["id"].as_str().expect("Chargeback should have id");
    println!("Testing get_chargeback_expanded with: {}", cb_id);

    let result = client.get_chargeback_expanded(cb_id).await;

    match result {
        Ok(Some(cb)) => {
            println!("Successfully fetched ChargebackExpanded");
            println!("  ID: {}", cb.id.as_str());
            println!("  Amount: ${:.2}", cb.amount_dollars());
            println!("  Status: {:?}", cb.status);
            println!("  Cycle: {:?}", cb.cycle);
            println!("  Reason: {:?}", cb.reason);
            println!("  Actionable: {}", cb.is_actionable());

            if let Some(ref txn) = cb.txn {
                println!("  Transaction expanded:");
                println!("    ID: {}", txn.id.as_str());
                println!("    Amount: ${:.2}", txn.total.unwrap_or(0) as f64 / 100.0);
                println!("    Status: {:?}", txn.status);
            }

            if let Some(merchant_id) = cb.merchant_id() {
                println!("  Merchant ID: {}", merchant_id);
            }

            // Test convenience method
            if let Some(original) = cb.original_transaction_amount() {
                println!("  original_transaction_amount(): ${:.2}", original);
            }
        }
        Ok(None) => println!("Chargeback not found"),
        Err(e) => panic!("Failed to fetch chargeback: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_batch_expanded() {
    init_logging();
    let client = create_client();

    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    if batches.is_empty() {
        println!("No batches available");
        return;
    }

    let batch_id = batches[0]["id"].as_str().expect("Batch should have id");
    println!("Testing get_batch_expanded with: {}", batch_id);

    let result = client.get_batch_expanded(batch_id).await;

    match result {
        Ok(Some(batch)) => {
            println!("Successfully fetched BatchExpanded");
            println!("  ID: {}", batch.id.as_str());
            println!("  Date: {:?}", batch.date);
            println!("  Status: {:?}", batch.status);
            println!("  Is Open: {}", batch.is_open());
            println!("  Reference: {:?}", batch.reference);

            // Merchant is just an ID (not expanded)
            if let Some(merchant_id) = batch.merchant_id() {
                println!("  Merchant ID: {}", merchant_id);
            }

            if batch.txns.is_some() {
                println!("  Transactions: {} total", batch.transaction_count());
                println!("  Total amount: ${:.2}", batch.total_amount_dollars());
            }
        }
        Ok(None) => println!("Batch not found"),
        Err(e) => panic!("Failed to fetch batch: {:?}", e),
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_merchant_expanded() {
    init_logging();
    let client = create_client();

    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available");
        return;
    }

    let merchant_id = merchants[0]["id"].as_str().expect("Merchant should have id");
    println!("Testing get_merchant_expanded with: {}", merchant_id);

    let result = client.get_merchant_expanded(merchant_id).await;

    match result {
        Ok(Some(merchant)) => {
            println!("Successfully fetched MerchantExpanded");
            println!("  ID: {}", merchant.id.as_str());
            println!("  DBA: {:?}", merchant.dba);
            println!("  Name: {:?}", merchant.name);
            println!("  Email: {:?}", merchant.email);
            println!("  Phone: {:?}", merchant.phone);

            if let Some(ref members) = merchant.members {
                println!("  Members: {} total", members.len());
                for (i, member) in members.iter().take(5).enumerate() {
                    println!("    [{}] {} {} - {:?} (ownership: {:?}%)",
                        i,
                        member.first.as_deref().unwrap_or(""),
                        member.last.as_deref().unwrap_or(""),
                        member.title,
                        member.ownership.map(|o| o as f64 / 100.0));
                }
            }

            // Test convenience methods
            println!("  member_count(): {}", merchant.member_count());
            println!("  total_ownership_percent(): {:.1}%", merchant.total_ownership_percent());

            if let Some(primary) = merchant.primary_member() {
                println!("  Primary member: {} {}",
                    primary.first.as_deref().unwrap_or(""),
                    primary.last.as_deref().unwrap_or(""));
            }
        }
        Ok(None) => println!("Merchant not found"),
        Err(e) => panic!("Failed to fetch merchant: {:?}", e),
    }
}
