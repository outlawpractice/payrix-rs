//! Example: Webhook-driven chargeback dispute handler
//!
//! This example demonstrates how to integrate the webhook server with the
//! chargeback dispute handling workflow to automatically respond to disputes.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐        ┌─────────────────┐        ┌──────────────────┐
//! │  Payrix API     │───────>│  WebhookServer  │───────>│  Dispute Handler │
//! │  (sends POST)   │        │  (axum)         │        │  (this example)  │
//! └─────────────────┘        └─────────────────┘        └──────────────────┘
//!                                                                │
//!                                                                ▼
//!                                                       ┌────────────────────┐
//!                                                       │  ChargebackDispute │
//!                                                       │  (typestate API)   │
//!                                                       └────────────────────┘
//! ```
//!
//! # Running the Example
//!
//! ```bash
//! # Set your API key
//! export PAYRIX_API_KEY="your-api-key"
//!
//! # Run the example
//! cargo run --example webhook_dispute_handler --features webhooks
//! ```
//!
//! # Testing Locally
//!
//! You can test the webhook endpoint with curl:
//!
//! ```bash
//! curl -X POST http://localhost:13847/webhooks/payrix \
//!   -H "Content-Type: application/json" \
//!   -H "X-Webhook-Secret: test-secret" \
//!   -d '{
//!     "event": "chargeback.created",
//!     "resourceType": "chargebacks",
//!     "resourceId": "t1_chb_test123",
//!     "resource": {
//!       "id": "t1_chb_test123",
//!       "cycle": "first",
//!       "status": 1,
//!       "amount": 5000
//!     }
//!   }'
//! ```

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{error, info, warn};

use payrix::{
    webhooks::{ChargebackEvent, WebhookServer, WebhookServerConfig},
    workflows::dispute_handling::{
        ActiveDispute, ChargebackDispute, Evidence, TypedChargeback,
    },
    Environment, PayrixClient,
};

/// Application state shared across handlers.
#[allow(dead_code)]
struct AppState {
    client: PayrixClient,
    /// Track disputes we're currently processing to avoid duplicates.
    processing: RwLock<std::collections::HashSet<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("payrix=debug".parse()?)
                .add_directive("webhook_dispute_handler=debug".parse()?),
        )
        .init();

    // Get API key from environment
    let api_key = std::env::var("PAYRIX_API_KEY").unwrap_or_else(|_| {
        warn!("PAYRIX_API_KEY not set, using demo mode (no API calls)");
        "demo-key".to_string()
    });

    // Create Payrix client
    let client = PayrixClient::new(&api_key, Environment::Test)?;

    // Create shared application state
    let state = Arc::new(AppState {
        client,
        processing: RwLock::new(std::collections::HashSet::new()),
    });

    // Configure webhook server with authentication
    let config = WebhookServerConfig::new()
        .with_auth_header("X-Webhook-Secret", "test-secret")
        .with_stdout_logging(true);

    // Create server and get event receiver
    let (server, mut events) = WebhookServer::with_config(config);

    // Spawn the event handler task
    let handler_state = state.clone();
    tokio::spawn(async move {
        info!("Dispute handler started, waiting for events...");

        while let Some(event) = events.recv().await {
            // Check if this is a chargeback event
            if let Some(cb_event) = event.as_chargeback_event() {
                let chargeback_id = cb_event.chargeback_id().to_string();

                // Skip if already processing
                {
                    let processing = handler_state.processing.read().await;
                    if processing.contains(&chargeback_id) {
                        info!(chargeback_id, "Skipping duplicate event");
                        continue;
                    }
                }

                // Mark as processing
                {
                    let mut processing = handler_state.processing.write().await;
                    processing.insert(chargeback_id.clone());
                }

                // Handle the event
                let state = handler_state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_chargeback_event(&state, cb_event).await {
                        error!(chargeback_id, error = %e, "Failed to handle chargeback event");
                    }

                    // Remove from processing set
                    let mut processing = state.processing.write().await;
                    processing.remove(&chargeback_id);
                });
            }
        }
    });

    // Start the server
    let addr: SocketAddr = "0.0.0.0:13847".parse()?;
    println!();
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          Webhook Dispute Handler Example                    ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Webhook endpoint: POST http://localhost:13847/webhooks/payrix");
    println!("║  Health check:     GET  http://localhost:13847/health      ║");
    println!("║                                                            ║");
    println!("║  Auth header: X-Webhook-Secret: test-secret                ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    server.run(addr).await?;

    Ok(())
}

/// Handle a chargeback event from the webhook.
async fn handle_chargeback_event(
    state: &AppState,
    event: ChargebackEvent,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let chargeback_id = event.chargeback_id().to_string();
    let event_type = event_type_name(&event);

    info!(
        chargeback_id = chargeback_id.as_str(),
        event_type,
        "Processing chargeback event"
    );

    match event {
        ChargebackEvent::Created { data, .. } | ChargebackEvent::Opened { data, .. } => {
            // New or reopened chargeback - analyze and potentially respond
            info!(
                chargeback_id = chargeback_id.as_str(),
                total = data.total,
                cycle = ?data.cycle,
                "New chargeback received"
            );

            // Convert to typed dispute
            let dispute = ChargebackDispute::from_chargeback(data);

            // Handle based on state
            match dispute {
                ChargebackDispute::Active(active) => {
                    handle_active_dispute(state, active).await?;
                }
                ChargebackDispute::Terminal(terminal) => {
                    info!(
                        chargeback_id = terminal.id().as_str(),
                        "Dispute is already terminal, no action needed"
                    );
                }
            }
        }

        ChargebackEvent::Won { chargeback_id, .. } => {
            info!(chargeback_id, "Dispute WON - merchant prevailed");
            // Could trigger notification, update database, etc.
        }

        ChargebackEvent::Lost { chargeback_id, .. } => {
            warn!(chargeback_id, "Dispute LOST - merchant lost");
            // Could trigger notification, update database, etc.
        }

        ChargebackEvent::Closed { chargeback_id, .. } => {
            info!(chargeback_id, "Dispute closed");
        }

        ChargebackEvent::Other { chargeback_id, event_type, .. } => {
            info!(chargeback_id, event_type, "Other chargeback event");
        }
    }

    Ok(())
}

/// Handle an active dispute that may need action.
async fn handle_active_dispute(
    state: &AppState,
    dispute: ActiveDispute,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match dispute {
        ActiveDispute::First(first) => {
            handle_first_chargeback(state, first).await?;
        }
        ActiveDispute::PreArbitration(pre_arb) => {
            handle_pre_arbitration(state, pre_arb).await?;
        }
        ActiveDispute::SecondChargeback(second) => {
            info!(
                chargeback_id = second.id().as_str(),
                "Second chargeback received - requires manual review"
            );
            // Second chargebacks typically need careful human review
        }
        ActiveDispute::Retrieval(retrieval) => {
            info!(
                chargeback_id = retrieval.id().as_str(),
                "Retrieval request - awaiting first chargeback"
            );
        }
        ActiveDispute::Representment(rep) => {
            info!(
                chargeback_id = rep.id().as_str(),
                "In representment - awaiting issuer decision"
            );
        }
        ActiveDispute::Arbitration(arb) => {
            info!(
                chargeback_id = arb.id().as_str(),
                "In arbitration - awaiting card network decision"
            );
        }
    }

    Ok(())
}

/// Handle a first chargeback - decide whether to represent or accept.
async fn handle_first_chargeback(
    _state: &AppState,
    first: TypedChargeback<payrix::workflows::dispute_handling::First>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let chargeback_id = first.id().as_str();
    let total = first.inner().total.unwrap_or(0);
    let reason_code = first.inner().reason_code.clone();

    info!(
        chargeback_id,
        total,
        reason_code = reason_code.as_deref().unwrap_or("unknown"),
        "Evaluating first chargeback for response"
    );

    // Example decision logic (customize for your business):
    // - Auto-accept small disputes under $25 (cost of fighting exceeds value)
    // - Auto-represent with evidence if we have transaction proof
    // - Flag for manual review otherwise

    if total < 2500 {
        // $25.00 threshold
        info!(
            chargeback_id,
            total,
            "Amount below threshold, accepting liability"
        );

        // In a real implementation, you would call:
        // let terminal = first.accept_liability(&state.client).await?;

        // For demo, just log
        info!(chargeback_id, "Would accept liability (demo mode)");
    } else {
        // Check if we should represent
        if should_represent(&first) {
            info!(chargeback_id, "Decision: REPRESENT with evidence");

            // Build evidence
            let _evidence = Evidence::new(format!(
                "Customer received goods/services. Transaction ID: {}. \
                 Order was fulfilled on the delivery date.",
                chargeback_id
            ));

            // In a real implementation:
            // let represented = first.represent(&state.client, evidence).await?;

            info!(chargeback_id, "Would submit representment (demo mode)");
        } else {
            info!(chargeback_id, "Decision: FLAG FOR MANUAL REVIEW");
            // Could send to a queue, email, Slack, etc.
        }
    }

    Ok(())
}

/// Handle pre-arbitration - decide whether to escalate or accept.
async fn handle_pre_arbitration(
    _state: &AppState,
    pre_arb: TypedChargeback<payrix::workflows::dispute_handling::PreArbitration>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let chargeback_id = pre_arb.id().as_str();
    let total = pre_arb.inner().total.unwrap_or(0);

    info!(
        chargeback_id,
        total,
        "Pre-arbitration received - issuer rejected representment"
    );

    // Pre-arbitration is expensive (fees apply). Only escalate if:
    // 1. Amount is significant
    // 2. We have strong evidence
    // 3. Business decision supports it

    if total >= 50000 {
        // $500+ threshold for arbitration
        info!(
            chargeback_id,
            "High value dispute - consider arbitration"
        );
        // In real implementation:
        // let arbitrating = pre_arb.request_arbitration(&state.client).await?;
    } else {
        info!(
            chargeback_id,
            "Below arbitration threshold - accepting liability"
        );
        // In real implementation:
        // let terminal = pre_arb.accept_liability(&state.client).await?;
    }

    Ok(())
}

/// Determine if we should represent a chargeback.
fn should_represent(
    chargeback: &TypedChargeback<payrix::workflows::dispute_handling::First>,
) -> bool {
    let inner = chargeback.inner();

    // Example logic - customize for your business:

    // 1. Check reason code - some are easier to win
    if let Some(ref reason) = inner.reason_code {
        // Common winnable reason codes (varies by card network):
        // - "4837" - No cardholder authorization (if we have AVS/CVV match)
        // - "4853" - Cardholder dispute (if we have delivery proof)
        let winnable_codes = ["4837", "4853", "10.4", "13.1"];
        if winnable_codes.iter().any(|c| reason.contains(c)) {
            return true;
        }
    }

    // 2. Check if we have transaction data that suggests we can win
    // In real implementation, you'd check your database for:
    // - Delivery confirmation
    // - Customer signature
    // - AVS/CVV match
    // - Prior customer communication

    // Default: represent amounts over $50
    inner.total.unwrap_or(0) >= 5000
}

/// Get the event type name for logging.
fn event_type_name(event: &ChargebackEvent) -> &'static str {
    match event {
        ChargebackEvent::Created { .. } => "created",
        ChargebackEvent::Opened { .. } => "opened",
        ChargebackEvent::Closed { .. } => "closed",
        ChargebackEvent::Won { .. } => "won",
        ChargebackEvent::Lost { .. } => "lost",
        ChargebackEvent::Other { .. } => "other",
    }
}
