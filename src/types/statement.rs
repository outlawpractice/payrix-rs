//! Statement types for the Payrix API.
//!
//! Statements represent billing statements for entities.
//!
//! **OpenAPI schema:** `statementsResponse`
//!
//! This type is only available when the `financial` feature is enabled.

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ENUMS
// =============================================================================

/// Statement status values per OpenAPI spec.
///
/// **OpenAPI schema:** `statementStatus`
///
/// Valid values:
/// - `pending` - Statement amount is owed and is pending payment
/// - `processing` - A payment is processing for this statement
/// - `partiallyPaid` - The statement was partially paid
/// - `paid` - The statement was paid in full
/// - `partiallyCancelled` - The statement was partially cancelled
/// - `cancelled` - The statement was completely cancelled
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatementStatus {
    /// Statement amount is owed and is pending payment.
    Pending,
    /// A payment is processing for this statement, pending completion.
    Processing,
    /// The statement was partially paid, some amount is still outstanding.
    PartiallyPaid,
    /// The statement was paid in full.
    Paid,
    /// The statement was partially cancelled, some amount is still outstanding.
    PartiallyCancelled,
    /// The statement was completely cancelled and is no longer due for payment.
    Cancelled,
}

// =============================================================================
// STATEMENT STRUCT
// =============================================================================

/// A Payrix statement.
///
/// Statements represent billing statements for entities.
///
/// **OpenAPI schema:** `statementsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Statement {
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

    /// The identifier of the Billing of this statement resource.
    ///
    /// **OpenAPI type:** string (ref: statementsModelBilling)
    #[serde(default)]
    pub billing: Option<PayrixId>,

    /// The paying entity for which this statement applies.
    ///
    /// **OpenAPI type:** string (ref: statementsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The date on which this Statement period should start.
    ///
    /// Specified as an eight digit integer in YYYYMMDD format.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub start: Option<i32>,

    /// The date on which this Statement period should finish.
    ///
    /// Specified as an eight digit integer in YYYYMMDD format.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub finish: Option<i32>,

    /// The current status of the statement.
    ///
    /// **OpenAPI type:** string (ref: statementStatus)
    #[serde(default)]
    pub status: Option<StatementStatus>,

    /// The total paid amount for this statement, specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total_paid: Option<i64>,

    /// The total amount for this statement, specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total: Option<i64>,

    /// The currency for this statement.
    ///
    /// See ISO 4217 currency codes for valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The payee entity of the statement.
    ///
    /// **OpenAPI type:** string (ref: statementsModelForentity)
    #[serde(default)]
    pub forentity: Option<PayrixId>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_deserialize_full() {
        let json = r#"{
            "id": "t1_stm_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "billing": "t1_bil_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "start": 20240101,
            "finish": 20240131,
            "status": "pending",
            "totalPaid": 0,
            "total": 50000,
            "currency": "USD",
            "forentity": "t1_ent_12345678901234567890124"
        }"#;

        let statement: Statement = serde_json::from_str(json).unwrap();
        assert_eq!(statement.id.as_str(), "t1_stm_12345678901234567890123");
        assert_eq!(statement.start, Some(20240101));
        assert_eq!(statement.finish, Some(20240131));
        assert_eq!(statement.status, Some(StatementStatus::Pending));
        assert_eq!(statement.total_paid, Some(0));
        assert_eq!(statement.total, Some(50000));
        assert_eq!(statement.currency, Some("USD".to_string()));
    }

    #[test]
    fn statement_deserialize_minimal() {
        let json = r#"{"id": "t1_stm_12345678901234567890123"}"#;

        let statement: Statement = serde_json::from_str(json).unwrap();
        assert_eq!(statement.id.as_str(), "t1_stm_12345678901234567890123");
        assert!(statement.status.is_none());
        assert!(statement.total.is_none());
    }

    #[test]
    fn statement_status_values() {
        let test_cases = vec![
            ("pending", StatementStatus::Pending),
            ("processing", StatementStatus::Processing),
            ("partiallyPaid", StatementStatus::PartiallyPaid),
            ("paid", StatementStatus::Paid),
            ("partiallyCancelled", StatementStatus::PartiallyCancelled),
            ("cancelled", StatementStatus::Cancelled),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_stm_12345678901234567890123", "status": "{}"}}"#,
                val
            );
            let statement: Statement = serde_json::from_str(&json).unwrap();
            assert_eq!(statement.status, Some(expected));
        }
    }

    #[test]
    fn statement_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_stm_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "status": "paid",
            "total": 50000
        }"#;

        let statement: Statement = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&statement).unwrap();
        let deserialized: Statement = serde_json::from_str(&serialized).unwrap();
        assert_eq!(statement.id, deserialized.id);
        assert_eq!(statement.status, deserialized.status);
        assert_eq!(statement.total, deserialized.total);
    }
}
