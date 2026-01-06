//! Batch integration tests.

mod common;

use common::{create_client, init_logging};
use payrix::{Batch, BatchExpanded, EntityType};
use serde_json::Value;

// =============================================================================
// Basic Batch Tests
// =============================================================================

/// Test fetching batches.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_batches() {
    init_logging();
    let client = create_client();

    let batches: Vec<Batch> = client.get_all(EntityType::Batches).await.unwrap();

    println!("Found {} batches", batches.len());
    for batch in batches.iter().take(5) {
        println!(
            "  Batch: {} - status: {:?}, ref: {:?}",
            batch.id.as_str(),
            batch.status,
            batch.reference
        );
    }
}

/// Test fetching a single batch by ID.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_batch_by_id() {
    init_logging();
    let client = create_client();

    // First get any batch
    let batches: Vec<Value> = client.get_all(EntityType::Batches).await.unwrap();

    if batches.is_empty() {
        println!("No batches available for testing");
        return;
    }

    let batch_id = batches[0]["id"].as_str().expect("Batch should have id");
    println!("Testing with batch: {}", batch_id);

    let batch: Option<Batch> = client
        .get_one(EntityType::Batches, batch_id)
        .await
        .expect("Failed to fetch batch");

    if let Some(b) = batch {
        println!("Fetched batch: {}", b.id.as_str());
        println!("  Status: {:?}", b.status);
        println!("  Reference: {:?}", b.reference);
        println!("  Date: {:?}", b.date);
        println!("  Close Time: {:?}", b.close_time);
    }
}

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_batch_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_batch_expanded() {
    init_logging();
    let client = create_client();

    // First, find any batch
    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    if batches.is_empty() {
        println!("No batches available for testing");
        return;
    }

    // Use first batch
    let batch_json = batches
        .first()
        .expect("Should have at least one batch");

    let batch_id = batch_json["id"].as_str().expect("Batch should have id");
    println!("Testing get_batch_expanded with: {}", batch_id);

    // Fetch with expansions using convenience method
    let result = client.get_batch_expanded(batch_id).await;

    match result {
        Ok(Some(batch)) => {
            println!("Successfully fetched BatchExpanded:");
            println!("  ID: {}", batch.id.as_str());
            println!("  Status: {:?}", batch.status);
            println!("  Reference: {:?}", batch.reference);
            println!("  Txn Count: {}", batch.transaction_count());
            println!("  Total: ${:.2}", batch.total_amount_dollars());

            // Validate required fields
            assert!(!batch.id.as_str().is_empty());
            assert!(batch.id.as_str().starts_with("t1_bat_"));

            // Check merchant ID (not expanded)
            if let Some(merchant_id) = batch.merchant_id() {
                println!("  Merchant ID: {}", merchant_id);
            } else {
                println!("  Merchant: not present");
            }

            // Check expanded transactions
            if let Some(ref txns) = batch.txns {
                println!("  Transactions (expanded): {} items", txns.len());
                for (i, txn) in txns.iter().take(3).enumerate() {
                    println!(
                        "    [{}] {} - ${:.2}, status: {:?}",
                        i,
                        txn.id.as_str(),
                        txn.total.unwrap_or(0) as f64 / 100.0,
                        txn.status
                    );
                }
                if txns.len() > 3 {
                    println!("    ... and {} more", txns.len() - 3);
                }
            } else {
                println!("  Transactions: not expanded");
            }

            println!("BatchExpanded test passed!");
        }
        Ok(None) => println!("Batch not found (deleted?)"),
        Err(e) => panic!("Failed to fetch batch: {:?}", e),
    }
}

/// Test BatchExpanded deserializes the same data as raw JSON.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_batch_expanded_vs_raw_json() {
    init_logging();
    let client = create_client();

    // Get any batch
    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    if batches.is_empty() {
        println!("No batches available for testing");
        return;
    }

    let batch_id = batches[0]["id"].as_str().expect("Batch should have id");
    println!("Testing with batch: {}", batch_id);

    // Fetch as raw JSON
    let raw: Option<Value> = client
        .get_one_expanded(EntityType::Batches, batch_id, &["merchant", "txns"])
        .await
        .expect("Failed to fetch as JSON");

    // Fetch as BatchExpanded
    let typed: Option<BatchExpanded> = client
        .get_one_expanded(EntityType::Batches, batch_id, &["merchant", "txns"])
        .await
        .expect("Failed to fetch as BatchExpanded");

    if let (Some(raw), Some(typed)) = (raw, typed) {
        // Compare IDs
        assert_eq!(
            raw["id"].as_str().unwrap(),
            typed.id.as_str(),
            "IDs should match"
        );

        // Compare merchant if present (merchant is always an ID, not expanded)
        if let Some(raw_merchant) = raw.get("merchant").filter(|v| !v.is_null()) {
            let raw_merchant_id = if raw_merchant.is_object() {
                raw_merchant["id"].as_str().unwrap()
            } else {
                raw_merchant.as_str().unwrap()
            };

            assert!(
                typed.merchant.is_some(),
                "Typed merchant should be Some when JSON has merchant"
            );
            let typed_merchant_id = typed.merchant_id().unwrap();
            assert_eq!(
                raw_merchant_id,
                typed_merchant_id,
                "Merchant IDs should match"
            );
        }

        // Compare txns array if present
        if let Some(raw_txns) = raw.get("txns").filter(|v| v.is_array()) {
            let raw_txns = raw_txns.as_array().unwrap();
            if !raw_txns.is_empty() {
                assert!(
                    typed.txns.is_some(),
                    "Typed txns should be Some when JSON has array"
                );
                let typed_txns = typed.txns.as_ref().unwrap();
                assert_eq!(
                    raw_txns.len(),
                    typed_txns.len(),
                    "Transaction counts should match"
                );
            }
        }

        println!("Raw JSON and BatchExpanded match correctly!");
    }
}

/// Test multiple batches with expanded data.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_multiple_batches_expanded() {
    init_logging();
    let client = create_client();

    // Get multiple batches as raw JSON first
    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    println!("Found {} batches total", batches.len());

    // Test expanded fetch for up to 5 batches
    let mut success_count = 0;
    for batch_json in batches.iter().take(5) {
        let batch_id = match batch_json["id"].as_str() {
            Some(id) => id,
            None => continue,
        };

        match client.get_batch_expanded(batch_id).await {
            Ok(Some(batch)) => {
                println!(
                    "Batch {}: ${:.2}, {} txns, merchant ID: {}",
                    batch.id.as_str(),
                    batch.total_amount_dollars(),
                    batch.transaction_count(),
                    batch.merchant_id().unwrap_or("N/A")
                );
                success_count += 1;
            }
            Ok(None) => println!("Batch {} not found", batch_id),
            Err(e) => println!("Failed to fetch batch {}: {:?}", batch_id, e),
        }
    }

    println!("\nSuccessfully expanded {}/5 batches", success_count);
    assert!(success_count > 0, "Should successfully expand at least one batch");
}

/// Test batch expanded fields are optional (don't fail on missing data).
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_batch_expanded_optional_fields() {
    init_logging();
    let client = create_client();

    // Get any batch
    let batches: Vec<Value> = client
        .get_all(EntityType::Batches)
        .await
        .expect("Failed to get batches");

    if batches.is_empty() {
        println!("No batches available");
        return;
    }

    let batch_id = batches[0]["id"].as_str().expect("Batch should have id");

    // Fetch WITHOUT expansions - expanded fields should be None
    let batch: Option<BatchExpanded> = client
        .get_one_expanded(EntityType::Batches, batch_id, &[])
        .await
        .expect("Failed to fetch");

    if let Some(batch) = batch {
        // Should still deserialize even without expansions
        println!("Fetched batch without expansions: {}", batch.id.as_str());

        // Expanded fields should be None when not requested
        // (The API might still return them as IDs, but our type expects objects)
        println!("  merchant present: {}", batch.merchant.is_some());
        println!("  txns present: {}", batch.txns.is_some());

        // Core fields should still work
        println!("  Status: {:?}", batch.status);
        println!("  Total: ${:.2}", batch.total_amount_dollars());
    }
}
