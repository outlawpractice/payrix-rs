//! Refund types for the Payrix API.
//!
//! Refunds represent the return of funds from a previously completed entry
//! back to the customer.
//!
//! **OpenAPI schema:** `refundsResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// REFUND STRUCT
// =============================================================================

/// A Payrix refund.
///
/// Refunds return funds from an entry back to the customer.
///
/// **OpenAPI schema:** `refundsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Refund {
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

    /// The identifier of the Entry that is being refunded.
    ///
    /// **OpenAPI type:** string (ref: refundsModelEntry)
    #[serde(default)]
    pub entry: Option<PayrixId>,

    /// A description of this Refund.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The amount of this Refund.
    ///
    /// This field is specified in cents (up to three decimal points).
    /// If not set, the API uses the amount from the related Entry resource.
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Entry associated with this refund.
    ///
    /// **OpenAPI type:** entriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entries: Option<serde_json::Value>,

    /// Pending entries associated with this refund.
    ///
    /// **OpenAPI type:** pendingEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub pending_entries: Option<serde_json::Value>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Refund Struct Tests ====================

    #[test]
    fn refund_deserialize_full() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entry": "t1_ent_12345678901234567890123",
            "description": "Customer refund request",
            "amount": 2500.5
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.id.as_str(), "t1_ref_12345678901234567890123");
        assert_eq!(refund.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(refund.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(refund.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(refund.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(refund.entry.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(refund.description, Some("Customer refund request".to_string()));
        assert_eq!(refund.amount, Some(2500.5));
    }

    #[test]
    fn refund_deserialize_minimal() {
        let json = r#"{"id": "t1_ref_12345678901234567890123"}"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.id.as_str(), "t1_ref_12345678901234567890123");
        assert!(refund.created.is_none());
        assert!(refund.modified.is_none());
        assert!(refund.creator.is_none());
        assert!(refund.modifier.is_none());
        assert!(refund.entry.is_none());
        assert!(refund.description.is_none());
        assert!(refund.amount.is_none());
    }

    #[test]
    fn refund_amount_decimal() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "amount": 1234.567
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.amount, Some(1234.567));
    }

    #[test]
    fn refund_amount_integer() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "amount": 2500
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.amount, Some(2500.0));
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn refund_with_nested_relations() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "entries": {"id": "t1_ent_12345678901234567890123"},
            "pendingEntries": {"id": "t1_pen_12345678901234567890123"}
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        assert!(refund.entries.is_some());
        assert!(refund.pending_entries.is_some());
    }

    #[test]
    fn refund_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_ref_12345678901234567890123",
            "entry": "t1_ent_12345678901234567890123",
            "description": "Test refund",
            "amount": 1000.0
        }"#;

        let refund: Refund = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&refund).unwrap();
        let deserialized: Refund = serde_json::from_str(&serialized).unwrap();
        assert_eq!(refund.id, deserialized.id);
        assert_eq!(refund.entry, deserialized.entry);
        assert_eq!(refund.description, deserialized.description);
        assert_eq!(refund.amount, deserialized.amount);
    }
}
