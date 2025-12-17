//! Reserve entry types for the Payrix API.
//!
//! Reserve entries track individual fund movements into and out of reserves.
//!
//! **OpenAPI schema:** `reserveEntriesResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// RESERVE ENTRY STRUCT
// =============================================================================

/// A Payrix reserve entry.
///
/// Reserve entries track movements of funds into and out of reserves.
///
/// **OpenAPI schema:** `reserveEntriesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ReserveEntry {
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

    /// The Login that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Fund that this reserveEntries resource relates to.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelFund)
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// This field indicates that this reserveEntry was triggered from a Transaction.
    ///
    /// It stores the identifier of the Transaction.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelTxn)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// The ID of the hold that generated this reserveEntry.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelHold)
    #[serde(default)]
    pub hold: Option<PayrixId>,

    /// This field indicates that this reserveEntry was triggered from an automatic reserve.
    ///
    /// It stores the identifier of the Reserve resource.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelReserve)
    #[serde(default)]
    pub reserve: Option<PayrixId>,

    /// This field indicates that this reserveEntry was triggered from a manual change
    /// to an entityReserve.
    ///
    /// It stores the identifier of the entityReserve resource.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelEntityReserve)
    #[serde(default)]
    pub entity_reserve: Option<PayrixId>,

    /// This field indicates that this reserveEntry shows funds moving out of or into reserve.
    ///
    /// This field stores the identifier of the reserveEntry resource that moved
    /// the funds into or out of the reserve.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelReserveEntry)
    #[serde(default)]
    pub reserve_entry: Option<PayrixId>,

    /// The identifier of the Entity that this reserveEntry refers to.
    ///
    /// This is the owner of the record that triggered the charge.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelOnentity)
    #[serde(default)]
    pub onentity: Option<PayrixId>,

    /// The entry ID associated with this pendingEntry.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelEntry)
    #[serde(default)]
    pub entry: Option<PayrixId>,

    /// The bucket of rev share fees.
    ///
    /// This is a large integer enum with values 1-100+ representing various event types
    /// like DAYS, WEEKS, MONTHS, CAPTURE, REFUND, CHARGEBACK, etc.
    ///
    /// **OpenAPI type:** integer (ref: reserveEntryEvent)
    #[serde(default)]
    pub event: Option<i32>,

    /// The identifier of the record that is associated with this PendingEntry.
    ///
    /// **OpenAPI type:** string (ref: reserveEntriesModelEventId)
    #[serde(default)]
    pub event_id: Option<PayrixId>,

    /// The current status of the reserveEntry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub status: Option<String>,

    /// Message that provides details about failure of the reserveEntry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub status_message: Option<String>,

    /// A description of this reserveEntries resource.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The date on which the funds in reserve should be released.
    ///
    /// Specified as an eight-digit string in YYYYMMDD format.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub release: Option<String>,

    /// The amount held in reserve in this reserveEntries resource.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub amount: Option<i64>,

    /// The date and time on which the reserveEntry was processed.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub processed: Option<String>,

    /// The processing ID for the reserveEntry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub processing_id: Option<String>,

    /// Last positive disbursement associated with this entry.
    ///
    /// **OpenAPI type:** object (ref: disbursementsResponse)
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub last_positive_of_disbursement: Option<serde_json::Value>,

    /// Last negative disbursement associated with this entry.
    ///
    /// **OpenAPI type:** object (ref: disbursementsResponse)
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub last_negative_of_disbursement: Option<serde_json::Value>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ReserveEntry Struct Tests ====================

    #[test]
    fn reserve_entry_deserialize_full() {
        let json = r#"{
            "id": "t1_rse_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "fund": "t1_fnd_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "hold": "t1_hld_12345678901234567890123",
            "reserve": "t1_rsv_12345678901234567890123",
            "entityReserve": "t1_ers_12345678901234567890123",
            "reserveEntry": "t1_rse_12345678901234567890124",
            "onentity": "t1_ent_12345678901234567890123",
            "entry": "t1_ent_12345678901234567890124",
            "event": 7,
            "eventId": "t1_evn_12345678901234567890123",
            "status": "processed",
            "statusMessage": null,
            "description": "Reserve hold for capture",
            "release": "20240630",
            "amount": 5000,
            "processed": "2024-01-01 12:00:00.0000",
            "processingId": "proc_123456"
        }"#;

        let re: ReserveEntry = serde_json::from_str(json).unwrap();
        assert_eq!(re.id.as_str(), "t1_rse_12345678901234567890123");
        assert_eq!(re.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(re.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            re.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            re.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            re.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            re.fund.as_ref().map(|f| f.as_str()),
            Some("t1_fnd_12345678901234567890123")
        );
        assert_eq!(
            re.txn.as_ref().map(|t| t.as_str()),
            Some("t1_txn_12345678901234567890123")
        );
        assert_eq!(
            re.hold.as_ref().map(|h| h.as_str()),
            Some("t1_hld_12345678901234567890123")
        );
        assert_eq!(
            re.reserve.as_ref().map(|r| r.as_str()),
            Some("t1_rsv_12345678901234567890123")
        );
        assert_eq!(
            re.entity_reserve.as_ref().map(|e| e.as_str()),
            Some("t1_ers_12345678901234567890123")
        );
        assert_eq!(
            re.reserve_entry.as_ref().map(|r| r.as_str()),
            Some("t1_rse_12345678901234567890124")
        );
        assert_eq!(
            re.onentity.as_ref().map(|o| o.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(
            re.entry.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890124")
        );
        assert_eq!(re.event, Some(7)); // CAPTURE
        assert_eq!(
            re.event_id.as_ref().map(|e| e.as_str()),
            Some("t1_evn_12345678901234567890123")
        );
        assert_eq!(re.status, Some("processed".to_string()));
        assert!(re.status_message.is_none());
        assert_eq!(re.description, Some("Reserve hold for capture".to_string()));
        assert_eq!(re.release, Some("20240630".to_string()));
        assert_eq!(re.amount, Some(5000));
        assert_eq!(re.processed, Some("2024-01-01 12:00:00.0000".to_string()));
        assert_eq!(re.processing_id, Some("proc_123456".to_string()));
    }

    #[test]
    fn reserve_entry_deserialize_minimal() {
        let json = r#"{"id": "t1_rse_12345678901234567890123"}"#;

        let re: ReserveEntry = serde_json::from_str(json).unwrap();
        assert_eq!(re.id.as_str(), "t1_rse_12345678901234567890123");
        assert!(re.created.is_none());
        assert!(re.modified.is_none());
        assert!(re.creator.is_none());
        assert!(re.modifier.is_none());
        assert!(re.login.is_none());
        assert!(re.fund.is_none());
        assert!(re.txn.is_none());
        assert!(re.hold.is_none());
        assert!(re.reserve.is_none());
        assert!(re.entity_reserve.is_none());
        assert!(re.reserve_entry.is_none());
        assert!(re.onentity.is_none());
        assert!(re.entry.is_none());
        assert!(re.event.is_none());
        assert!(re.event_id.is_none());
        assert!(re.status.is_none());
        assert!(re.status_message.is_none());
        assert!(re.description.is_none());
        assert!(re.release.is_none());
        assert!(re.amount.is_none());
        assert!(re.processed.is_none());
        assert!(re.processing_id.is_none());
    }

    #[test]
    fn reserve_entry_various_event_values() {
        // Test various event values from the OpenAPI spec
        let test_cases = vec![
            (1, "DAYS"),
            (7, "CAPTURE"),
            (8, "REFUND"),
            (11, "CHARGEBACK"),
            (37, "RESERVE_ENTRY_RELEASE"),
        ];

        for (event_val, _name) in test_cases {
            let json = format!(
                r#"{{"id": "t1_rse_12345678901234567890123", "event": {}}}"#,
                event_val
            );
            let re: ReserveEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(re.event, Some(event_val));
        }
    }

    #[test]
    fn reserve_entry_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_rse_12345678901234567890123",
            "reserve": "t1_rsv_12345678901234567890123",
            "amount": 5000,
            "event": 7
        }"#;

        let re: ReserveEntry = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&re).unwrap();
        let deserialized: ReserveEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(re.id, deserialized.id);
        assert_eq!(re.reserve, deserialized.reserve);
        assert_eq!(re.amount, deserialized.amount);
        assert_eq!(re.event, deserialized.event);
    }
}
