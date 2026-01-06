//! Mock data collection script.
//!
//! Run these tests manually to capture real API responses for offline testing.
//! This creates JSON fixtures in tests/mock_data/ that can be used for unit tests.
//!
//! Usage:
//!   TEST_PAYRIX_API_KEY="your-key" cargo test --test collect_mock_data -- --ignored --nocapture
//!
//! The collected data will be from merchant t1_mer_65f097a2848a4ceae39b6ee.

mod common;

use common::{create_client, init_logging};
use payrix::EntityType;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const MOCK_DATA_DIR: &str = "tests/mock_data";
const TARGET_MERCHANT: &str = "t1_mer_65f097a2848a4ceae39b6ee";

/// Helper to save fixture data in standard Payrix response format.
fn save_fixture(name: &str, data: Vec<Value>) {
    let path = Path::new(MOCK_DATA_DIR).join(format!("{}.json", name));

    let fixture = json!({
        "response": {
            "data": data,
            "details": {
                "requestId": 1,
                "totals": {
                    "count": data.len(),
                    "page": 1,
                    "pages": 1
                }
            }
        }
    });

    let content = serde_json::to_string_pretty(&fixture).unwrap();
    fs::write(&path, content).unwrap();
    println!("Saved {} items to {}", data.len(), path.display());
}

/// Helper to sample items with variety (not just first N).
fn sample_with_variety(items: Vec<Value>, max: usize) -> Vec<Value> {
    if items.len() <= max {
        return items;
    }

    // Take first, last, and evenly spaced items
    let mut sampled = Vec::with_capacity(max);
    let step = items.len() / max;

    for i in 0..max {
        let idx = (i * step).min(items.len() - 1);
        sampled.push(items[idx].clone());
    }

    sampled
}

// =============================================================================
// Data Collection Tests (run manually)
// =============================================================================

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_customers() {
    init_logging();
    let client = create_client();

    println!("Fetching customers for merchant {}...", TARGET_MERCHANT);

    let customers: Vec<Value> = client
        .search(EntityType::Customers, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch customers");

    println!("Found {} customers", customers.len());

    // Sample up to 10 customers with variety
    let samples = sample_with_variety(customers, 10);
    save_fixture("customers", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_tokens() {
    init_logging();
    let client = create_client();

    println!("Fetching tokens for merchant {}...", TARGET_MERCHANT);

    let tokens: Vec<Value> = client
        .search(EntityType::Tokens, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch tokens");

    println!("Found {} tokens", tokens.len());

    // Sample up to 10 tokens
    let samples = sample_with_variety(tokens, 10);
    save_fixture("tokens", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_transactions() {
    init_logging();
    let client = create_client();

    println!("Fetching transactions for merchant {}...", TARGET_MERCHANT);

    let txns: Vec<Value> = client
        .search(EntityType::Txns, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch transactions");

    println!("Found {} transactions", txns.len());

    // Try to get variety: approved, declined, different types
    let mut samples: Vec<Value> = Vec::new();

    // Get some approved transactions
    let approved: Vec<_> = txns.iter()
        .filter(|t| t.get("status").and_then(|s| s.as_i64()) == Some(1))
        .take(3)
        .cloned()
        .collect();
    samples.extend(approved);

    // Get some declined transactions
    let declined: Vec<_> = txns.iter()
        .filter(|t| t.get("status").and_then(|s| s.as_i64()) == Some(2))
        .take(3)
        .cloned()
        .collect();
    samples.extend(declined);

    // Fill rest with variety
    let remaining = 10 - samples.len();
    let others: Vec<_> = txns.iter()
        .filter(|t| !samples.iter().any(|s| s["id"] == t["id"]))
        .take(remaining)
        .cloned()
        .collect();
    samples.extend(others);

    save_fixture("transactions", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_merchants() {
    init_logging();
    let client = create_client();

    println!("Fetching merchant {}...", TARGET_MERCHANT);

    // Get the specific merchant
    let merchant: Option<Value> = client
        .get_one(EntityType::Merchants, TARGET_MERCHANT)
        .await
        .expect("Failed to fetch merchant");

    if let Some(m) = merchant {
        save_fixture("merchants", vec![m]);
    } else {
        println!("WARNING: Merchant {} not found", TARGET_MERCHANT);
    }
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_subscriptions() {
    init_logging();
    let client = create_client();

    println!("Fetching subscriptions for merchant {}...", TARGET_MERCHANT);

    let subs: Vec<Value> = client
        .search(EntityType::Subscriptions, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch subscriptions");

    println!("Found {} subscriptions", subs.len());

    let samples = sample_with_variety(subs, 10);
    save_fixture("subscriptions", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_plans() {
    init_logging();
    let client = create_client();

    println!("Fetching plans for merchant {}...", TARGET_MERCHANT);

    let plans: Vec<Value> = client
        .search(EntityType::Plans, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch plans");

    println!("Found {} plans", plans.len());

    let samples = sample_with_variety(plans, 10);
    save_fixture("plans", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_batches() {
    init_logging();
    let client = create_client();

    println!("Fetching batches for merchant {}...", TARGET_MERCHANT);

    let batches: Vec<Value> = client
        .search(EntityType::Batches, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch batches");

    println!("Found {} batches", batches.len());

    // Try to get both open and closed batches
    let mut samples: Vec<Value> = Vec::new();

    // Open batches (status = 0)
    let open: Vec<_> = batches.iter()
        .filter(|b| b.get("status").and_then(|s| s.as_i64()) == Some(0))
        .take(3)
        .cloned()
        .collect();
    samples.extend(open);

    // Closed batches (status = 1)
    let closed: Vec<_> = batches.iter()
        .filter(|b| b.get("status").and_then(|s| s.as_i64()) == Some(1))
        .take(5)
        .cloned()
        .collect();
    samples.extend(closed);

    save_fixture("batches", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_entities() {
    init_logging();
    let client = create_client();

    println!("Fetching entities for merchant {}...", TARGET_MERCHANT);

    let entities: Vec<Value> = client
        .search(EntityType::Entities, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch entities");

    println!("Found {} entities", entities.len());

    let samples = sample_with_variety(entities, 10);
    save_fixture("entities", samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_members() {
    init_logging();
    let client = create_client();

    println!("Fetching members for merchant {}...", TARGET_MERCHANT);

    let members: Vec<Value> = client
        .search(EntityType::Members, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch members");

    println!("Found {} members", members.len());

    let samples = sample_with_variety(members, 10);
    save_fixture("members", samples);
}

// =============================================================================
// Expanded Data Collection (with relationships)
// =============================================================================

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_transactions_expanded() {
    init_logging();
    let client = create_client();

    println!("Fetching expanded transactions for merchant {}...", TARGET_MERCHANT);

    // First get transaction IDs
    let txns: Vec<Value> = client
        .search(EntityType::Txns, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch transactions");

    if txns.is_empty() {
        println!("No transactions found");
        return;
    }

    // Fetch a few with expansions
    let mut expanded_samples = Vec::new();
    for txn in txns.iter().take(5) {
        let id = txn["id"].as_str().unwrap();
        let expanded: Option<Value> = client
            .get_one_expanded(EntityType::Txns, id, &["payment", "token", "customer"])
            .await
            .expect("Failed to fetch expanded transaction");

        if let Some(e) = expanded {
            expanded_samples.push(e);
        }
    }

    save_fixture("transactions_expanded", expanded_samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_tokens_expanded() {
    init_logging();
    let client = create_client();

    println!("Fetching expanded tokens for merchant {}...", TARGET_MERCHANT);

    let tokens: Vec<Value> = client
        .search(EntityType::Tokens, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch tokens");

    if tokens.is_empty() {
        println!("No tokens found");
        return;
    }

    let mut expanded_samples = Vec::new();
    for token in tokens.iter().take(5) {
        let id = token["id"].as_str().unwrap();
        let expanded: Option<Value> = client
            .get_one_expanded(EntityType::Tokens, id, &["payment", "customer"])
            .await
            .expect("Failed to fetch expanded token");

        if let Some(e) = expanded {
            expanded_samples.push(e);
        }
    }

    save_fixture("tokens_expanded", expanded_samples);
}

#[tokio::test]
#[ignore = "Run manually to collect mock data"]
async fn collect_customers_expanded() {
    init_logging();
    let client = create_client();

    println!("Fetching expanded customers for merchant {}...", TARGET_MERCHANT);

    let customers: Vec<Value> = client
        .search(EntityType::Customers, &format!("merchant[equals]={}", TARGET_MERCHANT))
        .await
        .expect("Failed to fetch customers");

    if customers.is_empty() {
        println!("No customers found");
        return;
    }

    let mut expanded_samples = Vec::new();
    for customer in customers.iter().take(5) {
        let id = customer["id"].as_str().unwrap();
        let expanded: Option<Value> = client
            .get_one_expanded(EntityType::Customers, id, &["tokens"])
            .await
            .expect("Failed to fetch expanded customer");

        if let Some(e) = expanded {
            expanded_samples.push(e);
        }
    }

    save_fixture("customers_expanded", expanded_samples);
}

// =============================================================================
// Collect All (convenience function)
// =============================================================================

#[tokio::test]
#[ignore = "Run manually to collect ALL mock data"]
async fn collect_all_mock_data() {
    init_logging();
    let client = create_client();

    println!("=== Collecting ALL mock data for merchant {} ===\n", TARGET_MERCHANT);

    // Basic entities
    println!("\n--- Merchants ---");
    if let Ok(Some(m)) = client.get_one::<Value>(EntityType::Merchants, TARGET_MERCHANT).await {
        save_fixture("merchants", vec![m]);
    }

    println!("\n--- Customers ---");
    if let Ok(customers) = client.search::<Value>(EntityType::Customers, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("customers", sample_with_variety(customers, 10));
    }

    println!("\n--- Tokens ---");
    if let Ok(tokens) = client.search::<Value>(EntityType::Tokens, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("tokens", sample_with_variety(tokens, 10));
    }

    println!("\n--- Transactions ---");
    if let Ok(txns) = client.search::<Value>(EntityType::Txns, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("transactions", sample_with_variety(txns, 10));
    }

    println!("\n--- Subscriptions ---");
    if let Ok(subs) = client.search::<Value>(EntityType::Subscriptions, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("subscriptions", sample_with_variety(subs, 10));
    }

    println!("\n--- Plans ---");
    if let Ok(plans) = client.search::<Value>(EntityType::Plans, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("plans", sample_with_variety(plans, 10));
    }

    println!("\n--- Batches ---");
    if let Ok(batches) = client.search::<Value>(EntityType::Batches, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("batches", sample_with_variety(batches, 10));
    }

    println!("\n--- Entities ---");
    if let Ok(entities) = client.search::<Value>(EntityType::Entities, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("entities", sample_with_variety(entities, 10));
    }

    println!("\n--- Members ---");
    if let Ok(members) = client.search::<Value>(EntityType::Members, &format!("merchant[equals]={}", TARGET_MERCHANT)).await {
        save_fixture("members", sample_with_variety(members, 10));
    }

    println!("\n=== Collection complete! ===");
}
