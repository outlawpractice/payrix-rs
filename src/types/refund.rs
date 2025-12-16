//! Refund types for the Payrix API.
//!
//! Refunds represent the return of funds from a previously completed transaction
//! back to the customer's payment method.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Refund status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum RefundStatus {
    /// Refund is pending
    #[default]
    Pending = 0,
    /// Refund is approved
    Approved = 1,
    /// Refund is processing
    Processing = 2,
    /// Refund completed successfully
    Completed = 3,
    /// Refund failed
    Failed = 4,
    /// Refund was voided
    Voided = 5,
}

/// A Payrix refund.
///
/// Refunds return funds from a completed transaction back to the customer.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    /// Unique identifier (30 characters, e.g., "t1_ref_...")
    pub id: PayrixId,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID that owns this refund
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Original transaction ID being refunded
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Customer ID
    #[serde(default)]
    pub customer: Option<PayrixId>,

    /// Login ID that created this refund
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Refund status
    #[serde(default)]
    pub status: Option<RefundStatus>,

    /// Refund amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Reason for refund
    #[serde(default)]
    pub reason: Option<String>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Reference number
    #[serde(default)]
    pub reference: Option<String>,

    /// Response code from processor
    #[serde(default)]
    pub response_code: Option<String>,

    /// Response message from processor
    #[serde(default)]
    pub response_message: Option<String>,

    /// Custom data field
    #[serde(default)]
    pub custom: Option<String>,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub created: Option<String>,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive (false=active, true=inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

/// Request to create a new refund.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRefund {
    /// Original transaction ID to refund (required)
    pub txn: String,

    /// Refund amount in cents (if not provided, full refund)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Reason for refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Description/memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether resource is inactive
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== RefundStatus Tests ====================

    #[test]
    fn refund_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&RefundStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&RefundStatus::Approved).unwrap(), "1");
        assert_eq!(serde_json::to_string(&RefundStatus::Processing).unwrap(), "2");
        assert_eq!(serde_json::to_string(&RefundStatus::Completed).unwrap(), "3");
        assert_eq!(serde_json::to_string(&RefundStatus::Failed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&RefundStatus::Voided).unwrap(), "5");
    }

    #[test]
    fn refund_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<RefundStatus>("0").unwrap(), RefundStatus::Pending);
        assert_eq!(serde_json::from_str::<RefundStatus>("1").unwrap(), RefundStatus::Approved);
        assert_eq!(serde_json::from_str::<RefundStatus>("2").unwrap(), RefundStatus::Processing);
        assert_eq!(serde_json::from_str::<RefundStatus>("3").unwrap(), RefundStatus::Completed);
        assert_eq!(serde_json::from_str::<RefundStatus>("4").unwrap(), RefundStatus::Failed);
        assert_eq!(serde_json::from_str::<RefundStatus>("5").unwrap(), RefundStatus::Voided);
    }

    #[test]
    fn refund_status_default() {
        assert_eq!(RefundStatus::default(), RefundStatus::Pending);
    }

    #[test]
    fn refund_status_invalid_value() {
        assert!(serde_json::from_str::<RefundStatus>("99").is_err());
    }

    // ==================== Refund Struct Tests ====================

    #[test]
    fn refund_deserialize_full() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "customer": "t1_cus_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 3,
            "amount": 2500,
            "currency": "USD",
            "reason": "Customer request",
            "description": "Partial refund",
            "reference": "REF123",
            "responseCode": "00",
            "responseMessage": "Approved",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.id.as_str(), "t1_ref_12345678901234567890123");
        assert_eq!(refund.txn.unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(refund.status, Some(RefundStatus::Completed));
        assert_eq!(refund.amount, Some(2500));
        assert_eq!(refund.reason, Some("Customer request".to_string()));
        assert!(!refund.inactive);
    }

    #[test]
    fn refund_deserialize_minimal() {
        let json = r#"{"id": "t1_ref_12345678901234567890123"}"#;
        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.id.as_str(), "t1_ref_12345678901234567890123");
        assert!(refund.status.is_none());
        assert!(refund.amount.is_none());
    }

    #[test]
    fn refund_bool_from_int() {
        let json = r#"{"id": "t1_ref_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let refund: Refund = serde_json::from_str(json).unwrap();
        assert!(refund.inactive);
        assert!(refund.frozen);
    }

    // ==================== NewRefund Tests ====================

    #[test]
    fn new_refund_serialize_full() {
        let new_refund = NewRefund {
            txn: "t1_txn_12345678901234567890123".to_string(),
            amount: Some(2500),
            reason: Some("Customer request".to_string()),
            description: Some("Partial refund".to_string()),
            custom: Some("custom".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_refund).unwrap();
        assert!(json.contains("\"txn\":\"t1_txn_12345678901234567890123\""));
        assert!(json.contains("\"amount\":2500"));
        assert!(json.contains("\"reason\":\"Customer request\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_refund_serialize_minimal() {
        let new_refund = NewRefund {
            txn: "t1_txn_12345678901234567890123".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_refund).unwrap();
        assert!(json.contains("\"txn\":\"t1_txn_12345678901234567890123\""));
        assert!(!json.contains("\"amount\""));
        assert!(!json.contains("\"reason\""));
    }
}
