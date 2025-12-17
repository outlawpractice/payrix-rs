//! Disbursement entry types for the Payrix API.
//!
//! Disbursement entries are individual line items within a disbursement,
//! representing specific fund movements.
//!
//! **OpenAPI schema:** `disbursementEntriesResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// DISBURSEMENT ENTRY STRUCT
// =============================================================================

/// A Payrix disbursement entry.
///
/// Disbursement entries track individual fund movements within a disbursement.
///
/// **OpenAPI schema:** `disbursementEntriesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct DisbursementEntry {
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

    /// The identifier of the Disbursement that this DisbursementEntry resource refers to.
    ///
    /// **OpenAPI type:** string (ref: disbursementEntriesModelDisbursement)
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// The identifier of the Entry that this DisbursementEntry resource refers to.
    ///
    /// **OpenAPI type:** string (ref: disbursementEntriesModelEntry)
    #[serde(default)]
    pub entry: Option<PayrixId>,

    /// The identifier of the PendingEntry that this DisbursementEntry resource refers to.
    ///
    /// **OpenAPI type:** string (ref: disbursementEntriesModelPendingEntry)
    #[serde(default)]
    pub pending_entry: Option<PayrixId>,

    /// The identifier of the ReserveEntry that this DisbursementEntry resource refers to.
    ///
    /// **OpenAPI type:** string (ref: disbursementEntriesModelReserveEntry)
    #[serde(default)]
    pub reserve_entry: Option<PayrixId>,

    /// The event that triggered the funding activity.
    ///
    /// This is a large integer enum with values 1-600+ representing various event types
    /// like DAYS, WEEKS, CAPTURE, REFUND, CHARGEBACK, etc.
    ///
    /// **OpenAPI type:** integer (ref: disbursementEntryEvent)
    #[serde(default)]
    pub event: Option<i32>,

    /// The identifier of the record that is associated with this DisbursementEntry resource.
    ///
    /// **OpenAPI type:** string (ref: disbursementEntriesModelEventId)
    #[serde(default)]
    pub event_id: Option<PayrixId>,

    /// The total amount of this DisbursementEntry.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    /// The total amount used of this DisbursementEntry.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount_used: Option<f64>,

    /// A description of this DisbursementEntry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== DisbursementEntry Struct Tests ====================

    #[test]
    fn disbursement_entry_deserialize_full() {
        let json = r#"{
            "id": "t1_dbe_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "disbursement": "t1_dbm_12345678901234567890123",
            "entry": "t1_etr_12345678901234567890123",
            "pendingEntry": "t1_per_12345678901234567890123",
            "reserveEntry": "t1_rer_12345678901234567890123",
            "event": 7,
            "eventId": "t1_txn_12345678901234567890123",
            "amount": 10000.50,
            "amountUsed": 0,
            "description": "CAPTURE fee schedule"
        }"#;

        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_dbe_12345678901234567890123");
        assert_eq!(entry.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(entry.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            entry.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            entry.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            entry.disbursement.as_ref().map(|d| d.as_str()),
            Some("t1_dbm_12345678901234567890123")
        );
        assert_eq!(
            entry.entry.as_ref().map(|e| e.as_str()),
            Some("t1_etr_12345678901234567890123")
        );
        assert_eq!(
            entry.pending_entry.as_ref().map(|p| p.as_str()),
            Some("t1_per_12345678901234567890123")
        );
        assert_eq!(
            entry.reserve_entry.as_ref().map(|r| r.as_str()),
            Some("t1_rer_12345678901234567890123")
        );
        assert_eq!(entry.event, Some(7)); // CAPTURE
        assert_eq!(
            entry.event_id.as_ref().map(|e| e.as_str()),
            Some("t1_txn_12345678901234567890123")
        );
        assert_eq!(entry.amount, Some(10000.50));
        assert_eq!(entry.amount_used, Some(0.0));
        assert_eq!(entry.description, Some("CAPTURE fee schedule".to_string()));
    }

    #[test]
    fn disbursement_entry_deserialize_minimal() {
        let json = r#"{"id": "t1_dbe_12345678901234567890123"}"#;

        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_dbe_12345678901234567890123");
        assert!(entry.created.is_none());
        assert!(entry.modified.is_none());
        assert!(entry.creator.is_none());
        assert!(entry.modifier.is_none());
        assert!(entry.disbursement.is_none());
        assert!(entry.entry.is_none());
        assert!(entry.pending_entry.is_none());
        assert!(entry.reserve_entry.is_none());
        assert!(entry.event.is_none());
        assert!(entry.event_id.is_none());
        assert!(entry.amount.is_none());
        assert!(entry.amount_used.is_none());
        assert!(entry.description.is_none());
    }

    #[test]
    fn disbursement_entry_various_event_values() {
        // Test various event values from the OpenAPI spec
        let test_cases = vec![
            (1, "DAYS"),
            (7, "CAPTURE"),
            (8, "REFUND"),
            (11, "CHARGEBACK"),
            (40, "REMAINDER"),
        ];

        for (event_val, _name) in test_cases {
            let json = format!(
                r#"{{"id": "t1_dbe_12345678901234567890123", "event": {}}}"#,
                event_val
            );
            let entry: DisbursementEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.event, Some(event_val));
        }
    }

    #[test]
    fn disbursement_entry_amount_as_float() {
        // Test that amount handles decimal values (up to 3 decimal points per spec)
        let json = r#"{"id": "t1_dbe_12345678901234567890123", "amount": 1234.567}"#;
        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.amount, Some(1234.567));

        let json = r#"{"id": "t1_dbe_12345678901234567890123", "amount": 5000}"#;
        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.amount, Some(5000.0));
    }

    #[test]
    fn disbursement_entry_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_dbe_12345678901234567890123",
            "disbursement": "t1_dbm_12345678901234567890123",
            "amount": 5000.0,
            "event": 7
        }"#;

        let entry: DisbursementEntry = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: DisbursementEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry.id, deserialized.id);
        assert_eq!(entry.disbursement, deserialized.disbursement);
        assert_eq!(entry.amount, deserialized.amount);
        assert_eq!(entry.event, deserialized.event);
    }
}
