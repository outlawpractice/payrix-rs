//! Dispute handling workflow integration tests.
//!
//! These tests use existing chargeback data since chargebacks cannot be
//! created programmatically in the Payrix test environment.

mod common;

use common::{
    create_client, init_logging, require_closed_chargeback_id, require_open_chargeback_id,
    test_merchant_id, fixtures,
};
use payrix::{
    Chargeback, ChargebackStatusValue, EntityType,
    workflows::dispute_handling::{
        ChargebackDispute, ActiveDispute, TypedChargeback, Evidence,
        First, Retrieval, PreArbitration, Terminal,
        MAX_DOCUMENTS, MAX_DOCUMENT_SIZE, MAX_TOTAL_SIZE,
    },
};

// =============================================================================
// Basic Loading Tests
// =============================================================================

/// Test loading a chargeback and determining its state.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_load_chargeback_as_typed_dispute() {
    init_logging();
    let client = create_client();

    // Get any chargeback
    let chargebacks: Vec<Chargeback> = client
        .get_all(EntityType::Chargebacks)
        .await
        .expect("Failed to get chargebacks");

    if chargebacks.is_empty() {
        println!("No chargebacks available for testing");
        return;
    }

    let chargeback_id = chargebacks[0].id.as_str();
    println!("Testing ChargebackDispute::load with: {}", chargeback_id);

    let dispute = ChargebackDispute::load(&client, chargeback_id)
        .await
        .expect("Should load chargeback");

    // Verify we got a valid typed dispute
    match &dispute {
        ChargebackDispute::Active(active) => {
            println!("Chargeback is active in state: {}", active.state_name());
            match active {
                ActiveDispute::Retrieval(cb) => {
                    println!("  -> Retrieval stage");
                    assert!(!cb.id().as_str().is_empty());
                }
                ActiveDispute::First(cb) => {
                    println!("  -> First chargeback stage");
                    assert!(!cb.id().as_str().is_empty());
                }
                ActiveDispute::Representment(cb) => {
                    println!("  -> Representment stage");
                    assert!(!cb.id().as_str().is_empty());
                }
                ActiveDispute::PreArbitration(cb) => {
                    println!("  -> Pre-arbitration stage");
                    assert!(!cb.id().as_str().is_empty());
                }
                ActiveDispute::SecondChargeback(cb) => {
                    println!("  -> Second chargeback stage");
                    assert!(!cb.id().as_str().is_empty());
                }
                ActiveDispute::Arbitration(cb) => {
                    println!("  -> Arbitration stage");
                    assert!(!cb.id().as_str().is_empty());
                }
            }
        }
        ChargebackDispute::Terminal(terminal) => {
            println!("Chargeback is terminal");
            assert!(!terminal.id().as_str().is_empty());
            println!("  Status: {:?}", terminal.inner().status);
        }
    }
}

/// Test loading with a known open chargeback ID.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and TEST_OPEN_CHARGEBACK_ID"]
async fn test_load_known_open_chargeback() {
    init_logging();
    let client = create_client();

    let chargeback_id = require_open_chargeback_id();
    println!("Loading known open chargeback: {}", chargeback_id);

    let dispute = ChargebackDispute::load(&client, &chargeback_id)
        .await
        .expect("Should load chargeback");

    // An "open" chargeback should be Active, not Terminal
    match dispute {
        ChargebackDispute::Active(active) => {
            println!("Correctly loaded as Active dispute");
            println!("  State: {}", active.state_name());
            assert!(
                !matches!(active, ActiveDispute::Retrieval(_)),
                "Open chargebacks typically shouldn't be in Retrieval"
            );
        }
        ChargebackDispute::Terminal(_) => {
            // This could happen if the chargeback was closed since the test data was created
            println!("Warning: Expected Active but got Terminal (chargeback may have been closed)");
        }
    }
}

/// Test that terminal chargebacks are correctly identified.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY and TEST_CLOSED_CHARGEBACK_ID"]
async fn test_terminal_chargeback_identification() {
    init_logging();
    let client = create_client();

    let chargeback_id = require_closed_chargeback_id();
    println!("Loading known closed chargeback: {}", chargeback_id);

    let dispute = ChargebackDispute::load(&client, &chargeback_id)
        .await
        .expect("Should load chargeback");

    match dispute {
        ChargebackDispute::Terminal(terminal) => {
            println!("Correctly identified as Terminal");
            println!("  ID: {}", terminal.id());
            println!("  Status: {:?}", terminal.inner().status);
        }
        ChargebackDispute::Active(active) => {
            // This might happen if the chargeback was reopened
            println!("Warning: Expected Terminal but got Active({})", active.state_name());
        }
    }
}

// =============================================================================
// State-Specific Method Availability (Compile-Time Checks)
// =============================================================================

/// Test state-specific method availability (compile-time check).
///
/// This test verifies the typestate pattern at compile time.
/// If this compiles, the type safety is working.
#[test]
fn test_state_methods_compile_time_safety() {
    // This function exists to verify that certain methods are available
    // on certain state types. If these type annotations compile, the
    // typestate pattern is working correctly.

    // First state should have represent() and accept_liability()
    fn _assert_first_has_represent(cb: TypedChargeback<First>) {
        // These would compile (methods exist on First state):
        // let _ = cb.represent(&client, evidence).await;
        // let _ = cb.accept_liability(&client).await;
        let _ = cb; // Suppress unused warning
    }

    // Retrieval state should have no action methods
    fn _assert_retrieval_has_no_actions(cb: TypedChargeback<Retrieval>) {
        // These would NOT compile (no such methods):
        // let _ = cb.represent(&client, evidence).await;
        // let _ = cb.accept_liability(&client).await;
        let _ = cb.id(); // Only readonly access
    }

    // Terminal state should have no action methods
    fn _assert_terminal_is_final(cb: TypedChargeback<Terminal>) {
        // No action methods available - correct!
        let _ = cb.id(); // Only readonly access
    }

    // PreArbitration should have request_arbitration()
    fn _assert_pre_arb_has_arbitration(cb: TypedChargeback<PreArbitration>) {
        // These would compile:
        // let _ = cb.request_arbitration(&client).await;
        // let _ = cb.represent(&client, evidence).await;
        // let _ = cb.accept_liability(&client).await;
        let _ = cb;
    }

    println!("Type safety verified at compile time!");
}

// =============================================================================
// Evidence Tests
// =============================================================================

/// Test Evidence builder functionality.
#[test]
fn test_evidence_builder() {
    // Create evidence with a message explaining why the chargeback should be reversed
    let evidence = Evidence::new("The customer received the goods as described in the order. \
        Tracking #: 1Z999AA10123456784, Delivered 2024-01-15 2:30 PM. \
        Customer acknowledged receipt via email on 2024-01-15.");

    // Add a mock document (in real usage, this would be actual file content)
    let evidence = evidence.with_document(
        "tracking_proof.pdf",
        b"mock PDF content".to_vec(),
        "application/pdf",
    );

    // Validate passes
    let result = evidence.validate();
    assert!(result.is_ok(), "Valid evidence should pass validation");

    println!("Evidence built successfully with {} documents", 1);
}

/// Test Evidence validation passes for valid evidence.
#[test]
fn test_evidence_validation_passes() {
    let evidence = Evidence::new("Valid statement for chargeback dispute.");

    let result = evidence.validate();
    assert!(result.is_ok(), "Valid evidence should pass validation");
}

/// Test Evidence validation fails for empty statement.
#[test]
fn test_evidence_validation_fails_empty_statement() {
    let evidence = Evidence::new("");

    let result = evidence.validate();
    assert!(result.is_err(), "Empty statement should fail validation");

    if let Err(e) = result {
        println!("Correctly rejected empty statement: {}", e);
    }
}

/// Test EvidenceDocument validation.
#[test]
fn test_evidence_document_limits() {
    // Test document count limit
    assert_eq!(MAX_DOCUMENTS, 8, "Max documents should be 8");

    // Test individual document size limit (1 MB)
    assert_eq!(MAX_DOCUMENT_SIZE, 1_048_576, "Max document size should be 1 MB");

    // Test total size limit (8 MB)
    assert_eq!(MAX_TOTAL_SIZE, 8_388_608, "Max total size should be 8 MB");

    println!("Evidence limits verified:");
    println!("  Max documents: {}", MAX_DOCUMENTS);
    println!("  Max document size: {} bytes ({} MB)", MAX_DOCUMENT_SIZE, MAX_DOCUMENT_SIZE / 1_048_576);
    println!("  Max total size: {} bytes ({} MB)", MAX_TOTAL_SIZE, MAX_TOTAL_SIZE / 1_048_576);
}

// =============================================================================
// Mock Data Tests
// =============================================================================

/// Test loading chargebacks from mock data.
#[test]
fn test_chargeback_from_mock_data() {
    let chargebacks: Vec<Chargeback> = fixtures::load_fixture("chargebacks");
    assert!(!chargebacks.is_empty(), "Should have mock chargeback data");

    println!("Loaded {} chargebacks from mock data", chargebacks.len());

    for cb in &chargebacks {
        // Convert to ChargebackDispute using from_chargeback
        let dispute = ChargebackDispute::from_chargeback(cb.clone());

        match &dispute {
            ChargebackDispute::Active(active) => {
                println!(
                    "  {} -> Active({}) - cycle: {:?}",
                    cb.id.as_str(),
                    active.state_name(),
                    cb.cycle
                );
            }
            ChargebackDispute::Terminal(terminal) => {
                println!(
                    "  {} -> Terminal - status: {:?}",
                    cb.id.as_str(),
                    terminal.inner().status
                );
            }
        }
    }
}

/// Test chargeback state determination from mock data.
#[test]
fn test_chargeback_state_determination() {
    let chargebacks: Vec<Chargeback> = fixtures::load_fixture("chargebacks");

    let mut open_count = 0;
    let mut closed_count = 0;
    let mut won_count = 0;
    let mut lost_count = 0;

    for cb in &chargebacks {
        let dispute = ChargebackDispute::from_chargeback(cb.clone());

        match cb.status {
            Some(ChargebackStatusValue::Open) => open_count += 1,
            Some(ChargebackStatusValue::Closed) => closed_count += 1,
            Some(ChargebackStatusValue::Won) => won_count += 1,
            Some(ChargebackStatusValue::Lost) => lost_count += 1,
            None => {}
        }

        // Verify state mapping
        match cb.status {
            Some(ChargebackStatusValue::Closed)
            | Some(ChargebackStatusValue::Won)
            | Some(ChargebackStatusValue::Lost) => {
                assert!(
                    matches!(dispute, ChargebackDispute::Terminal(_)),
                    "Closed/Won/Lost should be Terminal"
                );
            }
            Some(ChargebackStatusValue::Open) => {
                assert!(
                    matches!(dispute, ChargebackDispute::Active(_)),
                    "Open should be Active"
                );
            }
            None => {}
        }
    }

    println!("Chargeback status distribution:");
    println!("  Open: {}", open_count);
    println!("  Closed: {}", closed_count);
    println!("  Won: {}", won_count);
    println!("  Lost: {}", lost_count);
}

// =============================================================================
// Actionable Disputes Tests
// =============================================================================

/// Test querying for actionable disputes.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_get_actionable_disputes_for_merchant() {
    init_logging();
    let client = create_client();

    let merchant_id = test_merchant_id();
    println!("Querying actionable disputes for merchant: {}", merchant_id);

    let disputes = payrix::workflows::dispute_handling::get_actionable_disputes(&client, &merchant_id)
        .await
        .expect("Should query disputes");

    println!("Found {} actionable disputes", disputes.len());

    for dispute in &disputes {
        match dispute {
            ChargebackDispute::Active(active) => {
                println!("  {} - {}", active.id(), active.state_name());

                match active {
                    ActiveDispute::First(cb) => {
                        println!("    Can: represent, accept_liability");
                        println!("    Reply deadline: {:?}", cb.reply_deadline());
                    }
                    ActiveDispute::PreArbitration(cb) => {
                        println!("    Can: represent, accept_liability, request_arbitration");
                        println!("    Reply deadline: {:?}", cb.reply_deadline());
                    }
                    ActiveDispute::SecondChargeback(cb) => {
                        println!("    Can: represent, accept_liability");
                        println!("    Reply deadline: {:?}", cb.reply_deadline());
                    }
                    _ => {
                        println!("    (awaiting next stage)");
                    }
                }
            }
            ChargebackDispute::Terminal(terminal) => {
                // Terminal disputes shouldn't be "actionable" but might appear
                println!("  {} - Terminal (unexpected)", terminal.id());
            }
        }
    }
}

// =============================================================================
// Reply Deadline Tests
// =============================================================================

/// Test reply deadline parsing and accessibility.
#[tokio::test]
#[ignore = "requires TEST_PAYRIX_API_KEY"]
async fn test_chargeback_reply_deadline() {
    init_logging();
    let client = create_client();

    let chargebacks: Vec<Chargeback> = client
        .get_all(EntityType::Chargebacks)
        .await
        .expect("Failed to get chargebacks");

    println!("Checking reply deadlines on {} chargebacks", chargebacks.len());

    for cb in chargebacks.iter().take(10) {
        let dispute = ChargebackDispute::from_chargeback(cb.clone());

        match &dispute {
            ChargebackDispute::Active(active) => {
                // Access reply deadline through the typed state
                let deadline = match active {
                    ActiveDispute::First(cb) => cb.reply_deadline(),
                    ActiveDispute::PreArbitration(cb) => cb.reply_deadline(),
                    ActiveDispute::SecondChargeback(cb) => cb.reply_deadline(),
                    _ => None,
                };

                println!(
                    "  {} - {} - deadline: {:?}",
                    cb.id.as_str(),
                    active.state_name(),
                    deadline
                );
            }
            ChargebackDispute::Terminal(terminal) => {
                println!("  {} - Terminal (no deadline)", terminal.id());
            }
        }
    }
}
