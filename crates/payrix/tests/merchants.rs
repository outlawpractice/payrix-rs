//! Merchant integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id};
use payrix::{EntityType, Merchant, MerchantExpanded};
use serde_json::Value;

// =============================================================================
// Basic Merchant Tests
// =============================================================================

/// Test fetching merchants.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_merchants() {
    init_logging();
    let client = create_client();

    let merchants: Vec<Merchant> = client.get_all(EntityType::Merchants).await.unwrap();

    println!("Found {} merchants", merchants.len());
    for merchant in merchants.iter().take(5) {
        println!(
            "  Merchant: {} - DBA: {:?}, status: {:?}",
            merchant.id.as_str(),
            merchant.dba,
            merchant.status
        );
    }
}

/// Test fetching a single merchant by ID.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_merchant_by_id() {
    init_logging();
    let client = create_client();

    // Use the known test merchant
    let merchant: Option<Merchant> = client
        .get_one(EntityType::Merchants, &test_merchant_id())
        .await
        .expect("Failed to fetch merchant");

    if let Some(m) = merchant {
        println!("Fetched merchant: {}", m.id.as_str());
        println!("  DBA: {:?}", m.dba);
        println!("  Status: {:?}", m.status);
        println!("  Created: {:?}", m.created);
    } else {
        println!("Test merchant not found: {}", test_merchant_id());
    }
}

// =============================================================================
// Expanded Type Tests
// =============================================================================

/// Test `get_merchant_expanded()` convenience method.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_merchant_expanded() {
    init_logging();
    let client = create_client();

    // First, find any merchant
    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available for testing");
        return;
    }

    let merchant_id = merchants[0]["id"]
        .as_str()
        .expect("Merchant should have id");
    println!("Testing get_merchant_expanded with: {}", merchant_id);

    // Fetch with expansions using convenience method
    let result = client.get_merchant_expanded(merchant_id).await;

    match result {
        Ok(Some(merchant)) => {
            println!("Successfully fetched MerchantExpanded:");
            println!("  ID: {}", merchant.id.as_str());
            println!("  DBA: {:?}", merchant.dba);
            println!("  Name: {:?}", merchant.name);
            println!("  Status: {:?}", merchant.status);
            println!("  Email: {:?}", merchant.email);

            // Validate required fields
            assert!(!merchant.id.as_str().is_empty());
            assert!(merchant.id.as_str().starts_with("t1_mer_"));

            // Check expanded members
            if let Some(ref members) = merchant.members {
                println!("  Members (expanded): {} items", members.len());
                for (i, member) in members.iter().take(3).enumerate() {
                    println!(
                        "    [{}] {} - {} {}",
                        i,
                        member.id.as_str(),
                        member.first.as_deref().unwrap_or(""),
                        member.last.as_deref().unwrap_or("")
                    );
                }
                if members.len() > 3 {
                    println!("    ... and {} more", members.len() - 3);
                }
            } else {
                println!("  Members: not expanded");
            }

            // Test convenience method
            println!("  display_name(): {}", merchant.display_name());

            println!("MerchantExpanded test passed!");
        }
        Ok(None) => println!("Merchant not found (deleted?)"),
        Err(e) => panic!("Failed to fetch merchant: {:?}", e),
    }
}

/// Test with known test merchant ID.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_merchant_expanded_with_known_id() {
    init_logging();
    let client = create_client();

    let merchant_id = test_merchant_id();
    println!("Testing with known merchant: {}", merchant_id);

    let result = client.get_merchant_expanded(&merchant_id).await;

    match result {
        Ok(Some(merchant)) => {
            println!("Successfully fetched MerchantExpanded:");
            println!("  ID: {}", merchant.id.as_str());
            println!("  DBA: {:?}", merchant.dba);
            println!("  Status: {:?}", merchant.status);
            println!("  display_name(): {}", merchant.display_name());

            // Check members
            if let Some(ref members) = merchant.members {
                println!("  Has {} members", members.len());
            }
        }
        Ok(None) => println!("Test merchant not found"),
        Err(e) => panic!("Failed to fetch test merchant: {:?}", e),
    }
}

/// Test MerchantExpanded deserializes the same data as raw JSON.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_merchant_expanded_vs_raw_json() {
    init_logging();
    let client = create_client();

    // Get any merchant
    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available for testing");
        return;
    }

    let merchant_id = merchants[0]["id"]
        .as_str()
        .expect("Merchant should have id");
    println!("Testing with merchant: {}", merchant_id);

    // Fetch as raw JSON
    let raw: Option<Value> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &["members"])
        .await
        .expect("Failed to fetch as JSON");

    // Fetch as MerchantExpanded
    let typed: Option<MerchantExpanded> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &["members"])
        .await
        .expect("Failed to fetch as MerchantExpanded");

    if let (Some(raw), Some(typed)) = (raw, typed) {
        // Compare IDs
        assert_eq!(
            raw["id"].as_str().unwrap(),
            typed.id.as_str(),
            "IDs should match"
        );

        // Compare DBA
        if let Some(raw_dba) = raw.get("dba").and_then(|v| v.as_str()) {
            assert_eq!(
                typed.dba.as_deref(),
                Some(raw_dba),
                "DBA should match"
            );
        }

        // Compare members array if present
        if let Some(raw_members) = raw.get("members").filter(|v| v.is_array()) {
            let raw_members = raw_members.as_array().unwrap();
            if !raw_members.is_empty() {
                assert!(
                    typed.members.is_some(),
                    "Typed members should be Some when JSON has array"
                );
                let typed_members = typed.members.as_ref().unwrap();
                assert_eq!(
                    raw_members.len(),
                    typed_members.len(),
                    "Member counts should match"
                );
            }
        }

        println!("Raw JSON and MerchantExpanded match correctly!");
    }
}

/// Test multiple merchants with expanded data.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_multiple_merchants_expanded() {
    init_logging();
    let client = create_client();

    // Get multiple merchants as raw JSON first
    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    println!("Found {} merchants total", merchants.len());

    // Test expanded fetch for up to 5 merchants
    let mut success_count = 0;
    for merchant_json in merchants.iter().take(5) {
        let merchant_id = match merchant_json["id"].as_str() {
            Some(id) => id,
            None => continue,
        };

        match client.get_merchant_expanded(merchant_id).await {
            Ok(Some(merchant)) => {
                let member_count = merchant.members.as_ref().map(|m| m.len()).unwrap_or(0);
                println!(
                    "Merchant {}: {} - status: {:?}, {} members",
                    merchant.id.as_str(),
                    merchant.display_name(),
                    merchant.status,
                    member_count
                );
                success_count += 1;
            }
            Ok(None) => println!("Merchant {} not found", merchant_id),
            Err(e) => println!("Failed to fetch merchant {}: {:?}", merchant_id, e),
        }
    }

    println!("\nSuccessfully expanded {}/5 merchants", success_count);
    assert!(
        success_count > 0,
        "Should successfully expand at least one merchant"
    );
}

/// Test merchant expanded fields are optional (don't fail on missing data).
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_merchant_expanded_optional_fields() {
    init_logging();
    let client = create_client();

    // Get any merchant
    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available");
        return;
    }

    let merchant_id = merchants[0]["id"]
        .as_str()
        .expect("Merchant should have id");

    // Fetch WITHOUT expansions - expanded fields should be None
    let merchant: Option<MerchantExpanded> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &[])
        .await
        .expect("Failed to fetch");

    if let Some(merchant) = merchant {
        // Should still deserialize even without expansions
        println!(
            "Fetched merchant without expansions: {}",
            merchant.id.as_str()
        );

        // Expanded fields might be None when not requested
        println!("  members present: {}", merchant.members.is_some());

        // Core fields should still work
        println!("  DBA: {:?}", merchant.dba);
        println!("  Status: {:?}", merchant.status);
        println!("  display_name(): {}", merchant.display_name());
    }
}

/// Test MerchantExpanded with all available fields.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_merchant_expanded_all_fields() {
    init_logging();
    let client = create_client();

    // Get a merchant as raw JSON to see what fields exist
    let merchants: Vec<Value> = client
        .get_all(EntityType::Merchants)
        .await
        .expect("Failed to get merchants");

    if merchants.is_empty() {
        println!("No merchants available");
        return;
    }

    let merchant_id = merchants[0]["id"]
        .as_str()
        .expect("Merchant should have id");

    // Fetch with all possible expansions
    let expanded_json: Option<Value> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &["members"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(ref json) = expanded_json {
        println!("Raw expanded JSON:");
        println!("{}", serde_json::to_string_pretty(json).unwrap());
    }

    // Now deserialize to MerchantExpanded
    let expanded: Option<MerchantExpanded> = client
        .get_one_expanded(EntityType::Merchants, merchant_id, &["members"])
        .await
        .expect("Failed to fetch expanded");

    if let Some(merchant) = expanded {
        println!("\nDeserialized MerchantExpanded:");
        println!("  id: {}", merchant.id.as_str());
        println!("  created: {:?}", merchant.created);
        println!("  dba: {:?}", merchant.dba);
        println!("  name: {:?}", merchant.name);
        println!("  status: {:?}", merchant.status);
        println!("  email: {:?}", merchant.email);
        println!("  phone: {:?}", merchant.phone);
        println!("  inactive: {}", merchant.inactive);
        println!("  frozen: {}", merchant.frozen);

        // Check expanded fields
        println!("\nExpanded fields:");
        if let Some(ref members) = merchant.members {
            println!("  members: {} items", members.len());
        } else {
            println!("  members: None");
        }
    }
}
