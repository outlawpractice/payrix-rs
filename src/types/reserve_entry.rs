//! Reserve entry types for the Payrix API.
//!
//! Reserve entries track individual fund movements into and out of reserves.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

/// Reserve entry type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ReserveEntryType {
    /// Funds added to reserve
    #[default]
    Hold = 1,
    /// Funds released from reserve
    Release = 2,
    /// Funds transferred within reserve
    Transfer = 3,
}

/// A Payrix reserve entry.
///
/// Reserve entries track movements of funds into and out of reserves.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ReserveEntry {
    /// Unique identifier (30 characters, e.g., "t1_rse_...")
    pub id: PayrixId,

    /// Parent reserve ID
    #[serde(default)]
    pub reserve: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Transaction ID (if applicable)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Entry type
    #[serde(default, rename = "type")]
    pub entry_type: Option<ReserveEntryType>,

    /// Entry amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Balance after this entry in cents
    #[serde(default)]
    pub balance: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

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

    // ==================== ReserveEntryType Tests ====================

    #[test]
    fn reserve_entry_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&ReserveEntryType::Hold).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ReserveEntryType::Release).unwrap(), "2");
        assert_eq!(serde_json::to_string(&ReserveEntryType::Transfer).unwrap(), "3");
    }

    #[test]
    fn reserve_entry_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<ReserveEntryType>("1").unwrap(), ReserveEntryType::Hold);
        assert_eq!(serde_json::from_str::<ReserveEntryType>("2").unwrap(), ReserveEntryType::Release);
        assert_eq!(serde_json::from_str::<ReserveEntryType>("3").unwrap(), ReserveEntryType::Transfer);
    }

    #[test]
    fn reserve_entry_type_default() {
        assert_eq!(ReserveEntryType::default(), ReserveEntryType::Hold);
    }

    #[test]
    fn reserve_entry_type_invalid_value() {
        assert!(serde_json::from_str::<ReserveEntryType>("0").is_err());
        assert!(serde_json::from_str::<ReserveEntryType>("99").is_err());
    }

    // ==================== ReserveEntry Struct Tests ====================

    #[test]
    fn reserve_entry_deserialize_full() {
        let json = r#"{
            "id": "t1_rse_12345678901234567890123",
            "reserve": "t1_rsv_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "type": 1,
            "amount": 5000,
            "balance": 50000,
            "currency": "USD",
            "description": "Reserve hold for transaction",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let entry: ReserveEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_rse_12345678901234567890123");
        assert_eq!(entry.reserve.unwrap().as_str(), "t1_rsv_12345678901234567890123");
        assert_eq!(entry.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entry.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(entry.fund.unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(entry.txn.unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(entry.entry_type, Some(ReserveEntryType::Hold));
        assert_eq!(entry.amount, Some(5000));
        assert_eq!(entry.balance, Some(50000));
        assert_eq!(entry.currency, Some("USD".to_string()));
        assert_eq!(entry.description, Some("Reserve hold for transaction".to_string()));
        assert_eq!(entry.custom, Some("custom data".to_string()));
        assert!(!entry.inactive);
        assert!(entry.frozen);
    }

    #[test]
    fn reserve_entry_deserialize_minimal() {
        let json = r#"{
            "id": "t1_rse_12345678901234567890123"
        }"#;

        let entry: ReserveEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_rse_12345678901234567890123");
        assert!(entry.reserve.is_none());
        assert!(entry.entity.is_none());
        assert!(entry.merchant.is_none());
        assert!(entry.entry_type.is_none());
        assert!(entry.amount.is_none());
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }

    #[test]
    fn reserve_entry_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_rse_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let entry: ReserveEntry = serde_json::from_str(json).unwrap();
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }

    #[test]
    fn reserve_entry_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_rse_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let entry: ReserveEntry = serde_json::from_str(json).unwrap();
        assert!(entry.inactive);
        assert!(entry.frozen);
    }

    #[test]
    fn reserve_entry_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_rse_12345678901234567890123"}"#;
        let entry: ReserveEntry = serde_json::from_str(json).unwrap();
        assert!(!entry.inactive);
        assert!(!entry.frozen);
    }
}
