//! Entity reserve types for the Payrix API.
//!
//! Entity reserves represent reserve accounts associated with entities.
//!
//! **OpenAPI schema:** `entityReservesResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ENTITY RESERVE STRUCT
// =============================================================================

/// A Payrix entity reserve.
///
/// Entity reserves represent reserve accounts where funds are held for an entity.
///
/// **OpenAPI schema:** `entityReservesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct EntityReserve {
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
    /// **OpenAPI type:** string (ref: entityReservesModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Fund that this entityReserves resource relates to.
    ///
    /// **OpenAPI type:** string (ref: entityReservesModelFund)
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// The amount held in this entityReserve.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total: Option<i64>,

    /// The current sequentially numbered activity requested for this entityReserve.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub request_sequence: Option<i64>,

    /// The current sequentially numbered activity processed for this entityReserve.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub processed_sequence: Option<i64>,

    /// The name of this EntityReserve.
    ///
    /// This field is stored as a text string and must be between 0 and 50 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this EntityReserve.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// Reserve entries associated with this entity reserve.
    ///
    /// **OpenAPI type:** array of reserveEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserve_entries: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== EntityReserve Struct Tests ====================

    #[test]
    fn entity_reserve_deserialize_full() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "fund": "t1_fnd_12345678901234567890123",
            "total": 5000000,
            "requestSequence": 100,
            "processedSequence": 99,
            "name": "Merchant Reserve",
            "description": "Reserve account for merchant"
        }"#;

        let er: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(er.id.as_str(), "t1_ers_12345678901234567890123");
        assert_eq!(er.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(er.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            er.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            er.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            er.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            er.fund.as_ref().map(|f| f.as_str()),
            Some("t1_fnd_12345678901234567890123")
        );
        assert_eq!(er.total, Some(5000000));
        assert_eq!(er.request_sequence, Some(100));
        assert_eq!(er.processed_sequence, Some(99));
        assert_eq!(er.name, Some("Merchant Reserve".to_string()));
        assert_eq!(er.description, Some("Reserve account for merchant".to_string()));
    }

    #[test]
    fn entity_reserve_deserialize_minimal() {
        let json = r#"{"id": "t1_ers_12345678901234567890123"}"#;

        let er: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(er.id.as_str(), "t1_ers_12345678901234567890123");
        assert!(er.created.is_none());
        assert!(er.modified.is_none());
        assert!(er.creator.is_none());
        assert!(er.modifier.is_none());
        assert!(er.login.is_none());
        assert!(er.fund.is_none());
        assert!(er.total.is_none());
        assert!(er.request_sequence.is_none());
        assert!(er.processed_sequence.is_none());
        assert!(er.name.is_none());
        assert!(er.description.is_none());
    }

    #[test]
    fn entity_reserve_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "total": 1000000,
            "name": "Test Reserve"
        }"#;

        let er: EntityReserve = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&er).unwrap();
        let deserialized: EntityReserve = serde_json::from_str(&serialized).unwrap();
        assert_eq!(er.id, deserialized.id);
        assert_eq!(er.fund, deserialized.fund);
        assert_eq!(er.total, deserialized.total);
        assert_eq!(er.name, deserialized.name);
    }

    #[test]
    fn entity_reserve_large_total() {
        let json = r#"{"id": "t1_ers_12345678901234567890123", "total": 9999999999}"#;
        let er: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(er.total, Some(9999999999));
    }

    #[test]
    fn entity_reserve_zero_total() {
        let json = r#"{"id": "t1_ers_12345678901234567890123", "total": 0}"#;
        let er: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(er.total, Some(0));
    }
}
