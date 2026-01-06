//! Settlement types for the Payrix API.
//!
//! Settlements represent batched transaction settlements with payment processors.
//!
//! **OpenAPI schema:** `settlementsResponse`
//!
//! This type is only available when the `financial` feature is enabled.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId, Platform};

// =============================================================================
// ENUMS
// =============================================================================

/// Settlement status values per OpenAPI spec.
///
/// **OpenAPI schema:** `settlementStatus`
///
/// Valid values:
/// - `cancelled` - Cancelled
/// - `failed` - Failed
/// - `pending` - Pending
/// - `processed` - Processed
/// - `processing` - Processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementStatus {
    /// Cancelled.
    Cancelled,
    /// Failed.
    Failed,
    /// Pending.
    Pending,
    /// Processed.
    Processed,
    /// Processing.
    Processing,
}

// =============================================================================
// SETTLEMENT STRUCT
// =============================================================================

/// A Payrix settlement.
///
/// Settlements represent batched transaction settlements with payment processors.
///
/// **OpenAPI schema:** `settlementsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Settlement {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Login that owns this settlement.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Payment used with this settlement.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub payment: Option<PayrixId>,

    /// The platform used to process this resource.
    ///
    /// **OpenAPI type:** string (ref: platformModel)
    #[serde(default)]
    pub platform: Option<Platform>,

    /// The reference code used to identify the settlement in the platform.
    ///
    /// **OpenAPI type:** string
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,

    /// The current status of the settlement.
    ///
    /// **OpenAPI type:** string (ref: settlementStatus)
    #[serde(default)]
    pub status: Option<SettlementStatus>,

    /// The amount of the settlement.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub amount: Option<i32>,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_deserialize_full() {
        let json = r#"{
            "id": "t1_stl_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "payment": "t1_pmt_12345678901234567890123",
            "platform": "VANTIV",
            "ref": "STL123456",
            "status": "processed",
            "amount": 100000,
            "inactive": 0,
            "frozen": 0
        }"#;

        let settlement: Settlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.id.as_str(), "t1_stl_12345678901234567890123");
        assert_eq!(settlement.platform, Some(Platform::Vantiv));
        assert_eq!(settlement.status, Some(SettlementStatus::Processed));
        assert_eq!(settlement.amount, Some(100000));
        assert!(!settlement.inactive);
    }

    #[test]
    fn settlement_deserialize_minimal() {
        let json = r#"{"id": "t1_stl_12345678901234567890123"}"#;

        let settlement: Settlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.id.as_str(), "t1_stl_12345678901234567890123");
        assert!(settlement.status.is_none());
        assert!(!settlement.inactive);
    }

    #[test]
    fn settlement_status_values() {
        let test_cases = vec![
            ("cancelled", SettlementStatus::Cancelled),
            ("failed", SettlementStatus::Failed),
            ("pending", SettlementStatus::Pending),
            ("processed", SettlementStatus::Processed),
            ("processing", SettlementStatus::Processing),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_stl_12345678901234567890123", "status": "{}"}}"#,
                val
            );
            let settlement: Settlement = serde_json::from_str(&json).unwrap();
            assert_eq!(settlement.status, Some(expected));
        }
    }
}
