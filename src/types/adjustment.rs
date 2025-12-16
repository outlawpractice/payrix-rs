//! Adjustment types for the Payrix API.
//!
//! Adjustments represent manual or automatic balance corrections.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Adjustment type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AdjustmentType {
    /// Credit adjustment (adds funds)
    #[default]
    Credit = 1,
    /// Debit adjustment (removes funds)
    Debit = 2,
}

/// Adjustment status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AdjustmentStatus {
    /// Adjustment is pending
    #[default]
    Pending = 0,
    /// Adjustment is approved
    Approved = 1,
    /// Adjustment is processed
    Processed = 2,
    /// Adjustment is rejected
    Rejected = 3,
}

/// A Payrix adjustment.
///
/// Adjustments are manual or automatic balance corrections.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Adjustment {
    /// Unique identifier (30 characters, e.g., "t1_adj_...")
    pub id: PayrixId,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID being adjusted
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Login ID that created this adjustment
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Transaction ID (if related to a transaction)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Adjustment type
    #[serde(default, rename = "type")]
    pub adjustment_type: Option<AdjustmentType>,

    /// Adjustment status
    #[serde(default)]
    pub status: Option<AdjustmentStatus>,

    /// Adjustment amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Reason code
    #[serde(default)]
    pub reason_code: Option<String>,

    /// Reason description
    #[serde(default)]
    pub reason: Option<String>,

    /// Adjustment name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Description/notes
    #[serde(default)]
    pub description: Option<String>,

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

/// Request to create a new adjustment.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAdjustment {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Fund ID being adjusted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund: Option<String>,

    /// Transaction ID (if related)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn: Option<String>,

    /// Adjustment type (required)
    #[serde(rename = "type")]
    pub adjustment_type: AdjustmentType,

    /// Adjustment amount in cents (required)
    pub amount: i64,

    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Reason code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,

    /// Reason description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Adjustment name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description/notes
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

    // ==================== AdjustmentType Tests ====================

    #[test]
    fn adjustment_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AdjustmentType::Credit).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AdjustmentType::Debit).unwrap(), "2");
    }

    #[test]
    fn adjustment_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AdjustmentType>("1").unwrap(), AdjustmentType::Credit);
        assert_eq!(serde_json::from_str::<AdjustmentType>("2").unwrap(), AdjustmentType::Debit);
    }

    #[test]
    fn adjustment_type_default() {
        assert_eq!(AdjustmentType::default(), AdjustmentType::Credit);
    }

    // ==================== AdjustmentStatus Tests ====================

    #[test]
    fn adjustment_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AdjustmentStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&AdjustmentStatus::Approved).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AdjustmentStatus::Processed).unwrap(), "2");
        assert_eq!(serde_json::to_string(&AdjustmentStatus::Rejected).unwrap(), "3");
    }

    #[test]
    fn adjustment_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AdjustmentStatus>("0").unwrap(), AdjustmentStatus::Pending);
        assert_eq!(serde_json::from_str::<AdjustmentStatus>("1").unwrap(), AdjustmentStatus::Approved);
        assert_eq!(serde_json::from_str::<AdjustmentStatus>("2").unwrap(), AdjustmentStatus::Processed);
        assert_eq!(serde_json::from_str::<AdjustmentStatus>("3").unwrap(), AdjustmentStatus::Rejected);
    }

    #[test]
    fn adjustment_status_default() {
        assert_eq!(AdjustmentStatus::default(), AdjustmentStatus::Pending);
    }

    // ==================== Adjustment Struct Tests ====================

    #[test]
    fn adjustment_deserialize_full() {
        let json = r#"{
            "id": "t1_adj_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "type": 1,
            "status": 2,
            "amount": 5000,
            "currency": "USD",
            "reasonCode": "PROMO",
            "reason": "Promotional credit",
            "name": "Promo Adjustment",
            "description": "Marketing promotion",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.id.as_str(), "t1_adj_12345678901234567890123");
        assert_eq!(adj.adjustment_type, Some(AdjustmentType::Credit));
        assert_eq!(adj.status, Some(AdjustmentStatus::Processed));
        assert_eq!(adj.amount, Some(5000));
        assert_eq!(adj.reason, Some("Promotional credit".to_string()));
        assert!(!adj.inactive);
    }

    #[test]
    fn adjustment_deserialize_minimal() {
        let json = r#"{"id": "t1_adj_12345678901234567890123"}"#;
        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.id.as_str(), "t1_adj_12345678901234567890123");
        assert!(adj.adjustment_type.is_none());
        assert!(adj.status.is_none());
    }

    #[test]
    fn adjustment_bool_from_int() {
        let json = r#"{"id": "t1_adj_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert!(adj.inactive);
        assert!(adj.frozen);
    }

    // ==================== NewAdjustment Tests ====================

    #[test]
    fn new_adjustment_serialize_full() {
        let new_adj = NewAdjustment {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            fund: Some("t1_fnd_12345678901234567890123".to_string()),
            txn: Some("t1_txn_12345678901234567890123".to_string()),
            adjustment_type: AdjustmentType::Credit,
            amount: 5000,
            currency: Some("USD".to_string()),
            reason_code: Some("PROMO".to_string()),
            reason: Some("Promotional credit".to_string()),
            name: Some("Promo Adjustment".to_string()),
            description: Some("Marketing promotion".to_string()),
            custom: Some("custom".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_adj).unwrap();
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"amount\":5000"));
        assert!(json.contains("\"reasonCode\":\"PROMO\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_adjustment_serialize_minimal() {
        let new_adj = NewAdjustment {
            adjustment_type: AdjustmentType::Debit,
            amount: 1000,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_adj).unwrap();
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"amount\":1000"));
        assert!(!json.contains("\"entity\""));
        assert!(!json.contains("\"reason\""));
    }
}
