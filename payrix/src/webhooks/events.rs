//! Webhook event types for Payrix callbacks.
//!
//! This module defines the event types that are received from Payrix webhooks
//! and provides typed event channels for consuming specific event types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::types::Chargeback;

// =============================================================================
// Raw Webhook Event
// =============================================================================

/// A raw webhook event received from Payrix.
///
/// This represents the unprocessed event as received from the webhook endpoint.
/// Use the typed event enums (e.g., [`ChargebackEvent`]) for type-safe event handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// The event type string (e.g., "chargeback.created", "txn.approved").
    pub event_type: String,

    /// The resource type (e.g., "chargebacks", "txns").
    pub resource_type: String,

    /// The resource ID (e.g., "t1_chb_...").
    pub resource_id: String,

    /// The full event payload as JSON.
    pub data: serde_json::Value,

    /// When the event was received by our server.
    pub received_at: DateTime<Utc>,

    /// The source IP address of the webhook request.
    pub source_ip: IpAddr,
}

impl WebhookEvent {
    /// Create a new webhook event.
    pub fn new(
        event_type: impl Into<String>,
        resource_type: impl Into<String>,
        resource_id: impl Into<String>,
        data: serde_json::Value,
        source_ip: IpAddr,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            resource_type: resource_type.into(),
            resource_id: resource_id.into(),
            data,
            received_at: Utc::now(),
            source_ip,
        }
    }

    /// Check if this is a chargeback event.
    pub fn is_chargeback_event(&self) -> bool {
        self.event_type.starts_with("chargeback.")
    }

    /// Check if this is a transaction event.
    pub fn is_transaction_event(&self) -> bool {
        self.event_type.starts_with("txn.")
    }

    /// Check if this is a merchant event.
    pub fn is_merchant_event(&self) -> bool {
        self.event_type.starts_with("merchant.")
    }

    /// Check if this is a disbursement event.
    pub fn is_disbursement_event(&self) -> bool {
        self.event_type.starts_with("disbursement.")
    }

    /// Try to parse this event as a typed chargeback event.
    pub fn as_chargeback_event(&self) -> Option<ChargebackEvent> {
        if !self.is_chargeback_event() {
            return None;
        }

        let chargeback: Chargeback = serde_json::from_value(self.data.clone()).ok()?;
        let id = self.resource_id.clone();

        match self.event_type.as_str() {
            "chargeback.created" => Some(ChargebackEvent::Created {
                chargeback_id: id,
                data: chargeback,
            }),
            "chargeback.opened" => Some(ChargebackEvent::Opened {
                chargeback_id: id,
                data: chargeback,
            }),
            "chargeback.closed" => Some(ChargebackEvent::Closed {
                chargeback_id: id,
                data: chargeback,
            }),
            "chargeback.won" => Some(ChargebackEvent::Won {
                chargeback_id: id,
                data: chargeback,
            }),
            "chargeback.lost" => Some(ChargebackEvent::Lost {
                chargeback_id: id,
                data: chargeback,
            }),
            _ => Some(ChargebackEvent::Other {
                chargeback_id: id,
                event_type: self.event_type.clone(),
                data: chargeback,
            }),
        }
    }
}

// =============================================================================
// Typed Chargeback Events
// =============================================================================

/// A typed chargeback event.
///
/// These events are parsed from raw webhook events and provide strongly-typed
/// access to chargeback data.
#[derive(Debug, Clone)]
pub enum ChargebackEvent {
    /// A new chargeback was created.
    Created {
        /// The chargeback ID.
        chargeback_id: String,
        /// The chargeback data.
        data: Chargeback,
    },

    /// A chargeback was re-opened.
    Opened {
        /// The chargeback ID.
        chargeback_id: String,
        /// The chargeback data.
        data: Chargeback,
    },

    /// A chargeback was closed.
    Closed {
        /// The chargeback ID.
        chargeback_id: String,
        /// The chargeback data.
        data: Chargeback,
    },

    /// The merchant won the chargeback dispute.
    Won {
        /// The chargeback ID.
        chargeback_id: String,
        /// The chargeback data.
        data: Chargeback,
    },

    /// The merchant lost the chargeback dispute.
    Lost {
        /// The chargeback ID.
        chargeback_id: String,
        /// The chargeback data.
        data: Chargeback,
    },

    /// An unrecognized chargeback event type.
    Other {
        /// The chargeback ID.
        chargeback_id: String,
        /// The event type string.
        event_type: String,
        /// The chargeback data.
        data: Chargeback,
    },
}

impl ChargebackEvent {
    /// Get the chargeback ID for this event.
    pub fn chargeback_id(&self) -> &str {
        match self {
            Self::Created { chargeback_id, .. }
            | Self::Opened { chargeback_id, .. }
            | Self::Closed { chargeback_id, .. }
            | Self::Won { chargeback_id, .. }
            | Self::Lost { chargeback_id, .. }
            | Self::Other { chargeback_id, .. } => chargeback_id,
        }
    }

    /// Get the chargeback data for this event.
    pub fn data(&self) -> &Chargeback {
        match self {
            Self::Created { data, .. }
            | Self::Opened { data, .. }
            | Self::Closed { data, .. }
            | Self::Won { data, .. }
            | Self::Lost { data, .. }
            | Self::Other { data, .. } => data,
        }
    }

    /// Check if this is a terminal event (dispute is closed).
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Closed { .. } | Self::Won { .. } | Self::Lost { .. })
    }
}

// =============================================================================
// Webhook Event Types (for alert configuration)
// =============================================================================

/// Webhook event types that can be subscribed to.
///
/// Use these when configuring webhook alerts via [`setup_webhooks`](crate::workflows::webhook_setup::setup_webhooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebhookEventType {
    // ===== Generic Events =====
    /// Any resource created.
    Create,
    /// Any resource updated.
    Update,
    /// Any resource deleted.
    Delete,
    /// Ownership changed.
    Ownership,
    /// Batch operation.
    Batch,

    // ===== Account Events =====
    /// Account event (generic).
    Account,
    /// Account created.
    AccountCreated,
    /// Account updated.
    AccountUpdated,

    // ===== Chargeback/Dispute Events =====
    /// Chargeback event (generic).
    Chargeback,
    /// New chargeback created.
    ChargebackCreated,
    /// Chargeback opened/re-opened.
    ChargebackOpened,
    /// Chargeback closed.
    ChargebackClosed,
    /// Merchant won the dispute.
    ChargebackWon,
    /// Merchant lost the dispute.
    ChargebackLost,

    // ===== Transaction Events =====
    /// Transaction created.
    TransactionCreated,
    /// Transaction approved.
    TransactionApproved,
    /// Transaction failed.
    TransactionFailed,
    /// Transaction captured.
    TransactionCaptured,
    /// Transaction settled.
    TransactionSettled,
    /// Transaction returned.
    TransactionReturned,

    // ===== Merchant Events =====
    /// Merchant created.
    MerchantCreated,
    /// Merchant boarding in progress.
    MerchantBoarding,
    /// Merchant successfully boarded.
    MerchantBoarded,
    /// Merchant closed.
    MerchantClosed,
    /// Merchant boarding failed.
    MerchantFailed,
    /// Merchant held.
    MerchantHeld,

    // ===== Disbursement Events =====
    /// Disbursement requested.
    DisbursementRequested,
    /// Disbursement processing.
    DisbursementProcessing,
    /// Disbursement processed.
    DisbursementProcessed,
    /// Disbursement failed.
    DisbursementFailed,
    /// Disbursement denied.
    DisbursementDenied,
    /// Disbursement returned.
    DisbursementReturned,

    // ===== Payout Events =====
    /// Payout event.
    Payout,

    // ===== Fee Events =====
    /// Fee event.
    Fee,

    // ===== Subscription Events =====
    /// Subscription created.
    SubscriptionCreated,
    /// Subscription updated.
    SubscriptionUpdated,
    /// Subscription cancelled.
    SubscriptionCancelled,
}

impl WebhookEventType {
    /// Convert to the Payrix API event string.
    pub fn as_event_str(&self) -> &'static str {
        match self {
            // Generic
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Ownership => "ownership",
            Self::Batch => "batch",

            // Account
            Self::Account => "account",
            Self::AccountCreated => "account.created",
            Self::AccountUpdated => "account.updated",

            // Chargeback
            Self::Chargeback => "chargeback",
            Self::ChargebackCreated => "chargeback.created",
            Self::ChargebackOpened => "chargeback.opened",
            Self::ChargebackClosed => "chargeback.closed",
            Self::ChargebackWon => "chargeback.won",
            Self::ChargebackLost => "chargeback.lost",

            // Transaction
            Self::TransactionCreated => "txn.created",
            Self::TransactionApproved => "txn.approved",
            Self::TransactionFailed => "txn.failed",
            Self::TransactionCaptured => "txn.captured",
            Self::TransactionSettled => "txn.settled",
            Self::TransactionReturned => "txn.returned",

            // Merchant
            Self::MerchantCreated => "merchant.created",
            Self::MerchantBoarding => "merchant.boarding",
            Self::MerchantBoarded => "merchant.boarded",
            Self::MerchantClosed => "merchant.closed",
            Self::MerchantFailed => "merchant.failed",
            Self::MerchantHeld => "merchant.held",

            // Disbursement
            Self::DisbursementRequested => "disbursement.requested",
            Self::DisbursementProcessing => "disbursement.processing",
            Self::DisbursementProcessed => "disbursement.processed",
            Self::DisbursementFailed => "disbursement.failed",
            Self::DisbursementDenied => "disbursement.denied",
            Self::DisbursementReturned => "disbursement.returned",

            // Other
            Self::Payout => "payout",
            Self::Fee => "fee",
            Self::SubscriptionCreated => "subscription.created",
            Self::SubscriptionUpdated => "subscription.updated",
            Self::SubscriptionCancelled => "subscription.cancelled",
        }
    }

    /// Get all chargeback event types.
    pub fn all_chargeback_events() -> Vec<Self> {
        vec![
            Self::ChargebackCreated,
            Self::ChargebackOpened,
            Self::ChargebackClosed,
            Self::ChargebackWon,
            Self::ChargebackLost,
        ]
    }

    /// Get all transaction event types.
    pub fn all_transaction_events() -> Vec<Self> {
        vec![
            Self::TransactionCreated,
            Self::TransactionApproved,
            Self::TransactionFailed,
            Self::TransactionCaptured,
            Self::TransactionSettled,
            Self::TransactionReturned,
        ]
    }

    /// Get all merchant event types.
    pub fn all_merchant_events() -> Vec<Self> {
        vec![
            Self::MerchantCreated,
            Self::MerchantBoarding,
            Self::MerchantBoarded,
            Self::MerchantClosed,
            Self::MerchantFailed,
            Self::MerchantHeld,
        ]
    }

    /// Get all disbursement event types.
    pub fn all_disbursement_events() -> Vec<Self> {
        vec![
            Self::DisbursementRequested,
            Self::DisbursementProcessing,
            Self::DisbursementProcessed,
            Self::DisbursementFailed,
            Self::DisbursementDenied,
            Self::DisbursementReturned,
        ]
    }
}

impl std::fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_event_str())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_webhook_event_type_chargeback() {
        let event = WebhookEvent::new(
            "chargeback.created",
            "chargebacks",
            "t1_chb_123",
            serde_json::json!({}),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        );

        assert!(event.is_chargeback_event());
        assert!(!event.is_transaction_event());
    }

    #[test]
    fn test_webhook_event_type_transaction() {
        let event = WebhookEvent::new(
            "txn.approved",
            "txns",
            "t1_txn_123",
            serde_json::json!({}),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        );

        assert!(!event.is_chargeback_event());
        assert!(event.is_transaction_event());
    }

    #[test]
    fn test_webhook_event_type_as_str() {
        assert_eq!(WebhookEventType::ChargebackCreated.as_event_str(), "chargeback.created");
        assert_eq!(WebhookEventType::TransactionApproved.as_event_str(), "txn.approved");
        assert_eq!(WebhookEventType::MerchantBoarded.as_event_str(), "merchant.boarded");
    }

    #[test]
    fn test_all_chargeback_events() {
        let events = WebhookEventType::all_chargeback_events();
        assert_eq!(events.len(), 5);
        assert!(events.contains(&WebhookEventType::ChargebackCreated));
        assert!(events.contains(&WebhookEventType::ChargebackLost));
    }

    #[test]
    fn test_chargeback_event_terminal() {
        // Won is terminal
        let won = ChargebackEvent::Won {
            chargeback_id: "123".to_string(),
            data: create_test_chargeback(),
        };
        assert!(won.is_terminal());

        // Created is not terminal
        let created = ChargebackEvent::Created {
            chargeback_id: "123".to_string(),
            data: create_test_chargeback(),
        };
        assert!(!created.is_terminal());
    }

    fn create_test_chargeback() -> Chargeback {
        serde_json::from_value(serde_json::json!({
            "id": "t1_chb_12345678901234567890123"
        }))
        .unwrap()
    }
}
