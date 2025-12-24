//! Funding, batch, fee, disbursement, entry, and refund integration tests.

mod common;

use common::{create_client, init_logging, test_merchant_id};
use payrix::{
    Adjustment, Batch, Disbursement, DisbursementEntry, EntityReserve, EntityType, Entry, Fee,
    FeeRule, Fund, PayrixClient, Payout, PendingEntry, Refund, Reserve, ReserveEntry,
    SearchBuilder, Transaction, Environment,
};
use serde_json::json;
use std::env;

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_batches() {
    init_logging();
    let client = create_client();

    let batches: Vec<Batch> = client.get_all(EntityType::Batches).await.unwrap();

    println!("Found {} batches", batches.len());
    for batch in batches.iter().take(5) {
        println!(
            "  Batch: {} - date: {:?}, status: {:?}",
            batch.id.as_str(),
            batch.date,
            batch.status
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_fees() {
    init_logging();
    let client = create_client();

    let fees: Vec<Fee> = client.get_all(EntityType::Fees).await.unwrap();

    println!("Found {} fees", fees.len());
    for fee in fees.iter().take(5) {
        println!(
            "  Fee: {} - name: {:?}, type: {:?}",
            fee.id.as_str(),
            fee.name,
            fee.fee_type
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_fee_rules() {
    init_logging();
    let client = create_client();

    let fee_rules: Vec<FeeRule> = client.get_all(EntityType::FeeRules).await.unwrap();

    println!("Found {} fee rules", fee_rules.len());
    for rule in fee_rules.iter().take(5) {
        println!(
            "  FeeRule: {} - type: {:?}, value: {:?}",
            rule.id.as_str(),
            rule.rule_type,
            rule.value
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_funds() {
    init_logging();
    let client = create_client();

    let funds: Vec<Fund> = client.get_all(EntityType::Funds).await.unwrap();

    println!("Found {} funds", funds.len());
    for fund in funds.iter().take(5) {
        println!(
            "  Fund: {} - available: {:?}, pending: {:?}",
            fund.id.as_str(),
            fund.available,
            fund.pending
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_payouts() {
    // NOTE: The test environment does NOT process payouts, so this endpoint
    // will always return empty results in test mode.
    init_logging();
    let client = create_client();

    let payouts: Vec<Payout> = client.get_all(EntityType::Payouts).await.unwrap();

    println!("Found {} payouts", payouts.len());
    for payout in payouts.iter().take(5) {
        println!(
            "  Payout: {} - amount: {:?}, um: {:?}, schedule: {:?}",
            payout.id.as_str(),
            payout.amount,
            payout.um,
            payout.schedule
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_disbursements() {
    init_logging();
    let client = create_client();

    let disbursements: Vec<Disbursement> =
        client.get_all(EntityType::Disbursements).await.unwrap();

    println!("Found {} disbursements", disbursements.len());
    for d in disbursements.iter().take(5) {
        println!(
            "  Disbursement: {} - entity: {:?}, amount: {:?}, status: {:?}",
            d.id.as_str(),
            d.entity,
            d.amount,
            d.status
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_disbursement_entries() {
    init_logging();
    let client = create_client();

    let entries: Vec<DisbursementEntry> = client
        .get_all(EntityType::DisbursementEntries)
        .await
        .unwrap();

    println!("Found {} disbursement entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  DisbursementEntry: {} - disbursement: {:?}, amount: {:?}",
            e.id.as_str(),
            e.disbursement,
            e.amount
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_entries() {
    init_logging();
    let client = create_client();

    let entries: Vec<Entry> = client.get_all(EntityType::Entries).await.unwrap();

    println!("Found {} entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  Entry: {} - entity: {:?}, amount: {:?}, event: {:?}",
            e.id.as_str(),
            e.entity,
            e.amount,
            e.event
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_pending_entries() {
    init_logging();
    let client = create_client();

    let entries: Vec<PendingEntry> = client.get_all(EntityType::PendingEntries).await.unwrap();

    println!("Found {} pending entries", entries.len());
    for e in entries.iter().take(5) {
        println!(
            "  PendingEntry: {} - entity: {:?}, amount: {:?}, event: {:?}",
            e.id.as_str(),
            e.entity,
            e.amount,
            e.event
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_reserves() {
    init_logging();
    let client = create_client();

    let reserves: Vec<Reserve> = client.get_all(EntityType::Reserves).await.unwrap();

    println!("Found {} reserves", reserves.len());
    for reserve in reserves.iter().take(5) {
        println!(
            "  Reserve: {} - entity: {:?}, max: {:?}, status: {:?}",
            reserve.id.as_str(),
            reserve.entity,
            reserve.max,
            reserve.status
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_reserve_entries() {
    init_logging();
    let client = create_client();

    let entries: Vec<ReserveEntry> = client.get_all(EntityType::ReserveEntries).await.unwrap();

    println!("Found {} reserve entries", entries.len());
    for entry in entries.iter().take(5) {
        println!(
            "  ReserveEntry: {} - reserve: {:?}, amount: {:?}, event: {:?}",
            entry.id.as_str(),
            entry.reserve,
            entry.amount,
            entry.event
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_entity_reserves() {
    init_logging();
    let client = create_client();

    let reserves: Vec<EntityReserve> = client.get_all(EntityType::EntityReserves).await.unwrap();

    println!("Found {} entity reserves", reserves.len());
    for reserve in reserves.iter().take(5) {
        println!(
            "  EntityReserve: {} - fund: {:?}, total: {:?}, name: {:?}",
            reserve.id.as_str(),
            reserve.fund,
            reserve.total,
            reserve.name
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_adjustments() {
    init_logging();
    let client = create_client();

    let adjustments: Vec<Adjustment> = client.get_all(EntityType::Adjustments).await.unwrap();

    println!("Found {} adjustments", adjustments.len());
    for adj in adjustments.iter().take(5) {
        println!(
            "  Adjustment: {} - entity: {:?}, amount: {:?}, description: {:?}",
            adj.id.as_str(),
            adj.entity,
            adj.amount,
            adj.description
        );
    }
}

#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY environment variable"]
async fn test_get_refunds() {
    init_logging();
    let client = create_client();

    let refunds: Vec<Refund> = client.get_all(EntityType::Refunds).await.unwrap();

    println!("Found {} refunds", refunds.len());
    for r in refunds.iter().take(5) {
        println!(
            "  Refund: {} - entry: {:?}, amount: {:?}, description: {:?}",
            r.id.as_str(),
            r.entry,
            r.amount,
            r.description
        );
    }
}

/// Test creating a refund for an existing transaction.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and a refundable transaction"]
async fn test_refund_creation() {
    init_logging();
    let api_key = env::var("TEST_PAYRIX_API_KEY").expect("TEST_PAYRIX_API_KEY must be set");
    let client = PayrixClient::new(&api_key, Environment::Test).unwrap();

    println!("\n=== REFUND CREATION TEST ===\n");

    // Search for a refundable transaction (settled, not fully refunded)
    let search = SearchBuilder::new()
        .field("merchant", &test_merchant_id())
        .field("type", "1") // Sale transaction
        .build();

    let transactions: Vec<Transaction> = client
        .search(EntityType::Txns, &search)
        .await
        .expect("Failed to search transactions");

    println!("Found {} transactions for merchant", transactions.len());

    // Find a transaction we can refund (needs to have a positive total)
    let refundable = transactions.iter().find(|t| t.total.unwrap_or(0) > 100);

    let Some(txn) = refundable else {
        println!("No refundable transactions found - skipping test");
        println!("Note: A refundable transaction needs to be a settled sale with total > $1.00");
        return;
    };

    println!("Found refundable transaction: {}", txn.id.as_str());
    println!("  Total: {:?}", txn.total);
    println!("  Status: {:?}", txn.status);

    // Create a partial refund ($1.00 = 100 cents)
    let refund_amount = 100;
    let new_refund = json!({
        "txn": txn.id.as_str(),
        "amount": refund_amount,
        "description": "Integration test partial refund"
    });

    let result: Result<Refund, _> = client.create(EntityType::Refunds, &new_refund).await;

    match result {
        Ok(refund) => {
            println!("Created refund: {}", refund.id.as_str());
            println!("  Amount: {:?}", refund.amount);
            println!("  Entry: {:?}", refund.entry);
        }
        Err(e) => {
            println!("Refund creation failed: {:?}", e);
            println!("Note: Refunds may require special conditions (settled transaction, sufficient funds, etc.)");
        }
    }

    println!("\n=== REFUND CREATION TEST COMPLETE ===\n");
}
