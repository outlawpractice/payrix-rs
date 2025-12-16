//! Disbursement entry types for the Payrix API.
//!
//! Disbursement entries are individual line items within a disbursement,
//! representing specific fund movements.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, DisbursementStatus, PayrixId};

/// A Payrix disbursement entry.
///
/// Entries track individual fund movements within a disbursement.
/// NOTE: Monetary values may be floating point despite documentation suggesting cents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct DisbursementEntry {
    /// Unique identifier (30 characters, e.g., "t1_dse_...")
    pub id: PayrixId,

    /// Parent disbursement ID
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID the entry affects
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Entry status
    #[serde(default)]
    pub status: Option<DisbursementStatus>,

    /// Entry amount in cents per OpenAPI spec
    #[serde(default)]
    pub amount: Option<i64>,

    /// Fee amount in cents per OpenAPI spec
    #[serde(default)]
    pub fee: Option<i64>,

    /// Net amount after fees in cents per OpenAPI spec
    #[serde(default)]
    pub net: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Entry type/category
    #[serde(default, rename = "type")]
    pub entry_type: Option<String>,

    /// Reference transaction ID (if applicable)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Description/memo
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== DisbursementEntry Struct Tests ====================

    #[test]
    fn disbursement_entry_deserialize_full() {
        let json = r#"{
            "id": "t1_dse_12345678901234567890123",
            "disbursement": "t1_dis_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "status": 3,
            "amount": 10000,
            "fee": 100,
            "net": 9900,
            "currency": "USD",
            "type": "payment",
            "txn": "t1_txn_12345678901234567890123",
            "description": "Payment for transaction",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-16 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_dse_12345678901234567890123");
        assert_eq!(entry.disbursement.unwrap().as_str(), "t1_dis_12345678901234567890123");
        assert_eq!(entry.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entry.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(entry.fund.unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(entry.status, Some(DisbursementStatus::Processed));
        assert_eq!(entry.amount, Some(10000));
        assert_eq!(entry.fee, Some(100));
        assert_eq!(entry.net, Some(9900));
        assert_eq!(entry.currency, Some("USD".to_string()));
        assert_eq!(entry.entry_type, Some("payment".to_string()));
        assert_eq!(entry.txn.unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(entry.description, Some("Payment for transaction".to_string()));
        assert_eq!(entry.custom, Some("custom data".to_string()));
        assert!(!entry.inactive);
        assert!(entry.frozen);
    }

    #[test]
    fn disbursement_entry_deserialize_minimal() {
        let json = r#"{
            "id": "t1_dse_12345678901234567890123"
        }"#;

        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_dse_12345678901234567890123");
        assert!(entry.disbursement.is_none());
        assert!(entry.entity.is_none());
        assert!(entry.merchant.is_none());
        assert!(entry.status.is_none());
        assert!(entry.amount.is_none());
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }

    #[test]
    fn disbursement_entry_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_dse_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }

    #[test]
    fn disbursement_entry_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_dse_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert!(entry.inactive);
        assert!(entry.frozen);
    }

    #[test]
    fn disbursement_entry_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_dse_12345678901234567890123"}"#;
        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }
}
