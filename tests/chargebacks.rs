//! Chargeback integration tests.

mod common;

use common::{
    create_client, init_logging, TEST_CLOSED_CHARGEBACK_ID, TEST_OPEN_CHARGEBACK_ID,
};
use payrix::types::ChargebackMessageType;
use payrix::{
    Chargeback, ChargebackDocument, ChargebackMessage, ChargebackMessageResult, ChargebackStatus,
    ChargebackStatusValue, CreateChargebackMessage, EntityType, Environment, PayrixClient,
    SearchBuilder,
};
use std::env;

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargebacks() {
    init_logging();
    let client = create_client();

    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();

    println!("Found {} chargebacks", chargebacks.len());
    // Print ALL chargebacks to capture all status values for documentation
    for chargeback in chargebacks.iter() {
        println!(
            "  Chargeback: {} - status: {:?}, total: {:?}, reason: {:?}",
            chargeback.id.as_str(),
            chargeback.status,
            chargeback.total,
            chargeback.reason_code
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_messages() {
    init_logging();
    let client = create_client();

    let messages: Vec<ChargebackMessage> = client
        .get_all(EntityType::ChargebackMessages)
        .await
        .unwrap();

    println!("Found {} chargeback messages", messages.len());
    for msg in messages.iter().take(5) {
        println!(
            "  ChargebackMessage: {} - chargeback: {:?}, direction: {:?}, status: {:?}",
            msg.id.as_str(),
            msg.chargeback,
            msg.direction,
            msg.status
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_documents() {
    init_logging();
    let client = create_client();

    let docs: Vec<ChargebackDocument> = client
        .get_all(EntityType::ChargebackDocuments)
        .await
        .unwrap();

    println!("Found {} chargeback documents", docs.len());
    for doc in docs.iter().take(5) {
        println!(
            "  ChargebackDocument: {} - chargeback: {:?}, document_type: {:?}, name: {:?}",
            doc.id.as_str(),
            doc.chargeback,
            doc.document_type,
            doc.name
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_message_results() {
    init_logging();
    let client = create_client();

    let results: Vec<ChargebackMessageResult> = client
        .get_all(EntityType::ChargebackMessageResults)
        .await
        .unwrap();

    println!("Found {} chargeback message results", results.len());
    for r in results.iter().take(5) {
        println!(
            "  ChargebackMessageResult: {} - message: {:?}, result_type: {:?}",
            r.id.as_str(),
            r.message,
            r.result_type
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_statuses() {
    init_logging();
    let client = create_client();

    let statuses: Vec<ChargebackStatus> = client
        .get_all(EntityType::ChargebackStatuses)
        .await
        .unwrap();

    println!("Found {} chargeback statuses", statuses.len());
    // Print ALL statuses to capture all possible status values for API documentation
    for s in statuses.iter() {
        println!(
            "  ChargebackStatus: {} - chargeback: {:?}, status: {:?}, message: {:?}",
            s.id.as_str(),
            s.chargeback,
            s.status,
            s.chargeback_message
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_specific_chargeback() {
    // Test reading a specific chargeback by ID and validating response fields
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let chargeback: Option<Chargeback> = client
        .get_one(EntityType::Chargebacks, TEST_OPEN_CHARGEBACK_ID)
        .await
        .expect("Failed to get chargeback");

    assert!(chargeback.is_some(), "Chargeback should exist");
    let cb = chargeback.unwrap();

    println!("=== CHARGEBACK DETAILS ===");
    println!("ID: {}", cb.id.as_str());
    println!("Status: {:?}", cb.status);
    println!("Cycle: {:?}", cb.cycle);
    println!("Reason Code: {:?}", cb.reason_code);
    println!("Reason: {:?}", cb.reason);
    println!("Total: {:?}", cb.total);
    println!("Currency: {:?}", cb.currency);
    println!("Issued: {:?}", cb.issued);
    println!("Received: {:?}", cb.received);
    println!("Merchant: {:?}", cb.merchant);
    println!("Transaction: {:?}", cb.txn);
    println!("Bank Ref: {:?}", cb.bank_ref);
    println!("Chargeback Ref: {:?}", cb.chargeback_ref);
    println!("Payment Method: {:?}", cb.payment_method);

    // Validate the chargeback has expected fields populated
    assert_eq!(cb.id.as_str(), TEST_OPEN_CHARGEBACK_ID);
    assert!(cb.status.is_some(), "Status should be present");
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY - creates data in Payrix"]
async fn test_create_chargeback_message() {
    // Test creating a message on an existing open chargeback.
    // NOTE: This creates real data in the test environment.
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // First verify the chargeback exists and is open
    let chargeback: Option<Chargeback> = client
        .get_one(EntityType::Chargebacks, TEST_OPEN_CHARGEBACK_ID)
        .await
        .expect("Failed to get chargeback");

    let cb = chargeback.expect("Test chargeback should exist");
    println!(
        "Creating message on chargeback: {} (status: {:?})",
        cb.id.as_str(),
        cb.status
    );

    // Note: Some message types require specific chargeback states.
    // "notate" is generally safe for adding notes to any chargeback.
    let new_message = CreateChargebackMessage {
        chargeback: TEST_OPEN_CHARGEBACK_ID.to_string(),
        message_type: Some(ChargebackMessageType::Notate),
        subject: Some("Integration Test Note".to_string()),
        message: Some(format!(
            "Test message created by payrix-rs integration tests at timestamp {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        )),
    };

    let result: Result<ChargebackMessage, _> = client
        .create(EntityType::ChargebackMessages, &new_message)
        .await;

    match result {
        Ok(msg) => {
            println!("=== MESSAGE CREATED SUCCESSFULLY ===");
            println!("ID: {}", msg.id.as_str());
            println!("Chargeback: {:?}", msg.chargeback);
            println!("Type: {:?}", msg.message_type);
            println!("Status: {:?}", msg.status);
            println!("Subject: {:?}", msg.subject);
            println!("Message: {:?}", msg.message);
            println!("Direction: {:?}", msg.direction);
            println!("Created: {:?}", msg.created);
        }
        Err(e) => {
            println!("=== MESSAGE CREATION FAILED ===");
            println!("Error: {:?}", e);
            // Don't panic - document the error for analysis
            println!(
                "NOTE: Message creation may fail if the chargeback state doesn't allow this message type"
            );
        }
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_status_history() {
    // Test getting the status history for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    // Get status records for the closed chargeback (should have status history)
    let search = SearchBuilder::new()
        .field("chargeback", TEST_CLOSED_CHARGEBACK_ID)
        .build();

    let statuses: Vec<ChargebackStatus> = client
        .search(EntityType::ChargebackStatuses, &search)
        .await
        .expect("Failed to get chargeback statuses");

    println!(
        "=== STATUS HISTORY FOR {} ===",
        TEST_CLOSED_CHARGEBACK_ID
    );
    println!("Found {} status records", statuses.len());

    for status in &statuses {
        println!(
            "  {} - status: {:?}, chargeback: {:?}, message: {:?}",
            status.id.as_str(),
            status.status,
            status.chargeback,
            status.chargeback_message
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_messages_for_chargeback() {
    // Test getting messages for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let search = SearchBuilder::new()
        .field("chargeback", TEST_OPEN_CHARGEBACK_ID)
        .build();

    let messages: Vec<ChargebackMessage> = client
        .search(EntityType::ChargebackMessages, &search)
        .await
        .expect("Failed to get chargeback messages");

    println!("=== MESSAGES FOR {} ===", TEST_OPEN_CHARGEBACK_ID);
    println!("Found {} messages", messages.len());

    for msg in &messages {
        println!(
            "  {} - type: {:?}, status: {:?}, subject: {:?}",
            msg.id.as_str(),
            msg.message_type,
            msg.status,
            msg.subject
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_get_chargeback_documents_for_chargeback() {
    // Test getting documents for a specific chargeback
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    let search = SearchBuilder::new()
        .field("chargeback", TEST_OPEN_CHARGEBACK_ID)
        .build();

    let documents: Vec<ChargebackDocument> = client
        .search(EntityType::ChargebackDocuments, &search)
        .await
        .expect("Failed to get chargeback documents");

    println!("=== DOCUMENTS FOR {} ===", TEST_OPEN_CHARGEBACK_ID);
    println!("Found {} documents", documents.len());

    for doc in &documents {
        println!(
            "  {} - name: {:?}, type: {:?}, size: {:?}",
            doc.id.as_str(),
            doc.name,
            doc.document_type,
            doc.size
        );
    }
}

#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_outcomes_by_status() {
    // Test aggregating chargebacks by outcome status
    init_logging();
    let client = create_client();

    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();

    let mut open = 0;
    let mut won = 0;
    let mut lost = 0;
    let mut closed = 0;
    let mut other = 0;

    for cb in &chargebacks {
        match cb.status {
            Some(ChargebackStatusValue::Open) => open += 1,
            Some(ChargebackStatusValue::Won) => won += 1,
            Some(ChargebackStatusValue::Lost) => lost += 1,
            Some(ChargebackStatusValue::Closed) => closed += 1,
            None => other += 1,
        }
    }

    println!("=== CHARGEBACK STATUS SUMMARY ===");
    println!("Total: {}", chargebacks.len());
    println!("  Open: {}", open);
    println!("  Won: {}", won);
    println!("  Lost: {}", lost);
    println!("  Closed: {}", closed);
    println!("  Other/Unknown: {}", other);
}

/// Comprehensive test to discover all ChargebackStatusValue variants in use.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_discover_chargeback_status_values() {
    init_logging();
    let client = create_client();

    println!("\n=== CHARGEBACK STATUS VALUE DISCOVERY ===\n");

    // Collect all unique status values
    let mut unique_statuses: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Get all chargebacks and their status field
    println!("--- Checking Chargeback.status field ---");
    let chargebacks: Vec<Chargeback> = client.get_all(EntityType::Chargebacks).await.unwrap();
    println!("Total chargebacks: {}", chargebacks.len());

    for cb in &chargebacks {
        if let Some(status) = &cb.status {
            let status_str = format!("{:?}", status);
            unique_statuses.insert(status_str.clone());
            println!("  {} -> status: {:?}", cb.id.as_str(), status);
        }
    }

    // 2. Get all chargeback status records (status change history)
    println!("\n--- Checking ChargebackStatus.status field ---");
    let statuses: Vec<ChargebackStatus> = client
        .get_all(EntityType::ChargebackStatuses)
        .await
        .unwrap();
    println!("Total chargeback status records: {}", statuses.len());

    for s in &statuses {
        if let Some(status_val) = &s.status {
            let status_str = format!("{:?}", status_val);
            unique_statuses.insert(status_str);
        }
        println!(
            "  {} -> status: {:?}, chargeback: {:?}, message: {:?}",
            s.id.as_str(),
            s.status,
            s.chargeback,
            s.chargeback_message
        );
    }

    // 3. Summary
    println!("\n=== UNIQUE STATUS VALUES FOUND ===");
    let mut sorted: Vec<_> = unique_statuses.iter().collect();
    sorted.sort();
    for status in sorted {
        println!("  - {}", status);
    }

    println!("\n=== EXPECTED VALUES FROM OPENAPI (4 documented) ===");
    println!("  - Open");
    println!("  - Closed");
    println!("  - Won");
    println!("  - Lost");

    println!("\n=== POTENTIALLY UNDOCUMENTED VALUES TO WATCH FOR ===");
    println!("  - New (lifecycle: initial state)");
    println!("  - UnderReview (lifecycle: being reviewed)");
    println!("  - Responded (lifecycle: merchant responded)");
    println!("  - Expired (lifecycle: response deadline passed)");
}

/// Test to get raw JSON response for chargebacks to see exact status values.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_raw_json_status() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");

    println!("\n=== RAW JSON CHARGEBACK STATUS CHECK ===\n");

    // Make a direct HTTP request to see raw JSON
    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://test-api.payrix.com/chargebacks")
        .header("APIKEY", &api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to fetch chargebacks");

    let body = response.text().await.expect("Failed to read response body");

    // Parse as generic JSON to see raw status values
    let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(response_obj) = json.get("response") {
        if let Some(data) = response_obj.get("data") {
            if let Some(arr) = data.as_array() {
                println!("Found {} chargebacks in raw response", arr.len());

                // Collect unique raw status values
                let mut raw_statuses: std::collections::HashSet<String> =
                    std::collections::HashSet::new();

                for item in arr {
                    if let Some(status) = item.get("status") {
                        let status_str = status.to_string();
                        raw_statuses.insert(status_str.clone());
                        let id = item.get("id").map(|v| v.to_string()).unwrap_or_default();
                        println!("  {} -> raw status: {}", id, status_str);
                    }
                }

                println!("\n=== UNIQUE RAW STATUS VALUES ===");
                let mut sorted: Vec<_> = raw_statuses.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }
            }
        }
    }
}

/// Test to check chargeback status history for transitions.
#[tokio::test]
#[ignore = "requires PAYRIX_API_KEY environment variable"]
async fn test_chargeback_status_history_raw() {
    init_logging();
    let api_key = env::var("PAYRIX_API_KEY").expect("PAYRIX_API_KEY must be set");

    println!("\n=== RAW JSON CHARGEBACK STATUS HISTORY CHECK ===\n");

    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://test-api.payrix.com/chargebackStatuses")
        .header("APIKEY", &api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to fetch chargeback statuses");

    let body = response.text().await.expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

    if let Some(response_obj) = json.get("response") {
        if let Some(data) = response_obj.get("data") {
            if let Some(arr) = data.as_array() {
                println!(
                    "Found {} chargeback status records in raw response",
                    arr.len()
                );

                let mut raw_from: std::collections::HashSet<String> =
                    std::collections::HashSet::new();
                let mut raw_to: std::collections::HashSet<String> =
                    std::collections::HashSet::new();

                for item in arr {
                    let id = item.get("id").map(|v| v.to_string()).unwrap_or_default();
                    let from_status = item
                        .get("fromStatus")
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "null".to_string());
                    let to_status = item
                        .get("toStatus")
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "null".to_string());
                    let status = item
                        .get("status")
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "null".to_string());
                    let name = item
                        .get("name")
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "null".to_string());

                    raw_from.insert(from_status.clone());
                    raw_to.insert(to_status.clone());

                    println!(
                        "  {} -> status: {}, fromStatus: {}, toStatus: {}, name: {}",
                        id, status, from_status, to_status, name
                    );
                }

                println!("\n=== UNIQUE RAW fromStatus VALUES ===");
                let mut sorted: Vec<_> = raw_from.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }

                println!("\n=== UNIQUE RAW toStatus VALUES ===");
                let mut sorted: Vec<_> = raw_to.iter().collect();
                sorted.sort();
                for status in sorted {
                    println!("  - {}", status);
                }
            }
        }
    }
}
