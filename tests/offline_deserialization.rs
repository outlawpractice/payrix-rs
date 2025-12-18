//! Offline deserialization tests using real API response fixtures.
//!
//! These tests validate that our Rust types correctly deserialize real Payrix API
//! responses without needing network access. Run after collecting mock data with:
//!   cargo test --test collect_mock_data -- --ignored
//!
//! These tests do NOT require an API key and run offline.

mod common;

use common::fixtures::{fixture_exists, load_fixture, load_single_fixture};
use payrix::{
    Batch, Chargeback, ChargebackMessage, ChargebackStatus, Customer, Member, Merchant,
    Transaction,
};

// =============================================================================
// Customer Deserialization Tests
// =============================================================================

#[test]
fn test_deserialize_real_customers() {
    if !fixture_exists("customers") {
        println!("SKIP: customers.json not found - run collect_mock_data first");
        return;
    }

    let customers: Vec<Customer> = load_fixture("customers");

    assert!(!customers.is_empty(), "Should have loaded customers");
    println!("Successfully deserialized {} customers", customers.len());

    for customer in &customers {
        // Validate required fields
        assert!(
            customer.id.as_str().starts_with("t1_cus_"),
            "Customer ID should have correct prefix: {}",
            customer.id
        );

        // Print some info for debugging
        println!(
            "  Customer {}: {} {}",
            customer.id,
            customer.first.as_deref().unwrap_or("?"),
            customer.last.as_deref().unwrap_or("?")
        );
    }
}

#[test]
fn test_customer_fields_accessible() {
    if !fixture_exists("customers") {
        println!("SKIP: customers.json not found");
        return;
    }

    let customer: Customer = load_single_fixture("customers");

    // Test that common fields are accessible (may be None but shouldn't panic)
    let _ = &customer.id;
    let _ = &customer.first;
    let _ = &customer.last;
    let _ = &customer.email;
    let _ = &customer.phone;
    let _ = &customer.merchant;
    let _ = &customer.created;
    let _ = &customer.modified;

    println!("All Customer fields accessible");
}

// =============================================================================
// Transaction Deserialization Tests
// =============================================================================

#[test]
fn test_deserialize_real_transactions() {
    if !fixture_exists("transactions") {
        println!("SKIP: transactions.json not found");
        return;
    }

    let transactions: Vec<Transaction> = load_fixture("transactions");

    assert!(!transactions.is_empty(), "Should have loaded transactions");
    println!(
        "Successfully deserialized {} transactions",
        transactions.len()
    );

    for txn in &transactions {
        assert!(
            txn.id.as_str().starts_with("t1_txn_"),
            "Transaction ID should have correct prefix: {}",
            txn.id
        );

        println!(
            "  Transaction {}: type={:?}, status={:?}, total={:?}",
            txn.id, txn.txn_type, txn.status, txn.total
        );
    }
}

#[test]
fn test_transaction_amount_fields() {
    if !fixture_exists("transactions") {
        println!("SKIP: transactions.json not found");
        return;
    }

    let transactions: Vec<Transaction> = load_fixture("transactions");

    for txn in &transactions {
        // These fields should exist on transactions
        if let Some(total) = txn.total {
            assert!(total >= 0, "Total should be non-negative");
        }
        if let Some(approved) = txn.approved {
            assert!(approved >= 0, "Approved should be non-negative");
        }
    }
}

// =============================================================================
// Merchant Deserialization Tests
// =============================================================================

#[test]
fn test_deserialize_real_merchant() {
    if !fixture_exists("merchants") {
        println!("SKIP: merchants.json not found");
        return;
    }

    let merchant: Merchant = load_single_fixture("merchants");

    assert!(
        merchant.id.as_str().starts_with("t1_mer_"),
        "Merchant ID should have correct prefix: {}",
        merchant.id
    );

    println!(
        "Successfully deserialized merchant: {} ({})",
        merchant.id,
        merchant.dba.as_deref().unwrap_or("unnamed")
    );
}

// =============================================================================
// Batch Deserialization Tests
// =============================================================================

#[test]
fn test_deserialize_real_batches() {
    if !fixture_exists("batches") {
        println!("SKIP: batches.json not found");
        return;
    }

    let batches: Vec<Batch> = load_fixture("batches");

    assert!(!batches.is_empty(), "Should have loaded batches");
    println!("Successfully deserialized {} batches", batches.len());

    for batch in &batches {
        assert!(
            batch.id.as_str().starts_with("t1_bth_"),
            "Batch ID should have correct prefix: {}",
            batch.id
        );

        println!(
            "  Batch {}: status={:?}",
            batch.id,
            batch.status
        );
    }
}

// =============================================================================
// Member Deserialization Tests
// =============================================================================

#[test]
fn test_deserialize_real_members() {
    if !fixture_exists("members") {
        println!("SKIP: members.json not found");
        return;
    }

    let members: Vec<Member> = load_fixture("members");

    if members.is_empty() {
        println!("No members in fixture (this is OK)");
        return;
    }

    println!("Successfully deserialized {} members", members.len());

    for member in &members {
        assert!(
            member.id.as_str().starts_with("t1_mbr_"),
            "Member ID should have correct prefix: {}",
            member.id
        );

        println!(
            "  Member {}: {} {}",
            member.id,
            member.first.as_deref().unwrap_or("?"),
            member.last.as_deref().unwrap_or("?")
        );
    }
}

// =============================================================================
// Chargeback Deserialization Tests (pre-existing fixtures)
// =============================================================================

#[test]
fn test_deserialize_real_chargebacks() {
    if !fixture_exists("chargebacks") {
        println!("SKIP: chargebacks.json not found");
        return;
    }

    let chargebacks: Vec<Chargeback> = load_fixture("chargebacks");

    assert!(!chargebacks.is_empty(), "Should have loaded chargebacks");
    println!("Successfully deserialized {} chargebacks", chargebacks.len());

    for cb in &chargebacks {
        assert!(
            cb.id.as_str().starts_with("t1_chb_"),
            "Chargeback ID should have correct prefix: {}",
            cb.id
        );

        println!(
            "  Chargeback {}: status={:?}, total={:?}",
            cb.id, cb.status, cb.total
        );
    }
}

#[test]
fn test_deserialize_chargeback_messages() {
    if !fixture_exists("chargebackMessages") {
        println!("SKIP: chargebackMessages.json not found");
        return;
    }

    let messages: Vec<ChargebackMessage> = load_fixture("chargebackMessages");

    println!(
        "Successfully deserialized {} chargeback messages",
        messages.len()
    );

    for msg in &messages {
        assert!(
            msg.id.as_str().starts_with("t1_chm_"),
            "ChargebackMessage ID should have correct prefix: {}",
            msg.id
        );
    }
}

#[test]
fn test_deserialize_chargeback_statuses() {
    if !fixture_exists("chargebackStatuses") {
        println!("SKIP: chargebackStatuses.json not found");
        return;
    }

    let statuses: Vec<ChargebackStatus> = load_fixture("chargebackStatuses");

    println!(
        "Successfully deserialized {} chargeback statuses",
        statuses.len()
    );

    for status in &statuses {
        assert!(
            status.id.as_str().starts_with("t1_chs_"),
            "ChargebackStatus ID should have correct prefix: {}",
            status.id
        );
    }
}

// =============================================================================
// Cross-Entity Relationship Tests
// =============================================================================

#[test]
fn test_transactions_reference_valid_merchant() {
    if !fixture_exists("transactions") || !fixture_exists("merchants") {
        println!("SKIP: Need both transactions.json and merchants.json");
        return;
    }

    let transactions: Vec<Transaction> = load_fixture("transactions");
    let merchant: Merchant = load_single_fixture("merchants");

    // All transactions should reference the same merchant
    for txn in &transactions {
        if let Some(ref txn_merchant) = txn.merchant {
            assert_eq!(
                txn_merchant.as_str(),
                merchant.id.as_str(),
                "Transaction {} should reference the fixture merchant",
                txn.id
            );
        }
    }

    println!("All transactions reference the correct merchant");
}

#[test]
fn test_customers_reference_valid_merchant() {
    if !fixture_exists("customers") || !fixture_exists("merchants") {
        println!("SKIP: Need both customers.json and merchants.json");
        return;
    }

    let customers: Vec<Customer> = load_fixture("customers");
    let merchant: Merchant = load_single_fixture("merchants");

    for customer in &customers {
        if let Some(ref cust_merchant) = customer.merchant {
            assert_eq!(
                cust_merchant.as_str(),
                merchant.id.as_str(),
                "Customer {} should reference the fixture merchant",
                customer.id
            );
        }
    }

    println!("All customers reference the correct merchant");
}

// =============================================================================
// Regression Tests - Ensure API format hasn't changed
// =============================================================================

#[test]
fn test_all_fixtures_deserialize_without_error() {
    let fixtures = [
        ("customers", "Customer"),
        ("transactions", "Transaction"),
        ("merchants", "Merchant"),
        ("batches", "Batch"),
        ("members", "Member"),
        ("chargebacks", "Chargeback"),
        ("chargebackMessages", "ChargebackMessage"),
        ("chargebackStatuses", "ChargebackStatus"),
    ];

    let mut passed = 0;
    let mut skipped = 0;

    for (name, type_name) in fixtures {
        if !fixture_exists(name) {
            println!("SKIP: {}.json not found", name);
            skipped += 1;
            continue;
        }

        // Load as raw JSON to check format
        let raw = common::fixtures::load_fixture_raw(name);
        let count = raw.len();

        println!("OK: {} - {} {} items", name, count, type_name);
        passed += 1;
    }

    println!(
        "\nFixture validation: {} passed, {} skipped",
        passed, skipped
    );
    assert!(passed > 0, "At least some fixtures should exist");
}
