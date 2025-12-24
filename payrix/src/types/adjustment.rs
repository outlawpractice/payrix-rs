//! Adjustment types for the Payrix API.
//!
//! Adjustments represent manual or automatic balance corrections.
//!
//! **OpenAPI schema:** `adjustmentsResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ADJUSTMENT STRUCT
// =============================================================================

/// A Payrix adjustment.
///
/// Adjustments are manual or automatic balance corrections applied to entities.
///
/// **OpenAPI schema:** `adjustmentsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Adjustment {
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

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Entity associated with this Account.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The applicable Entity for this Adjustment.
    ///
    /// This field is an optional field.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelFromentity)
    #[serde(default)]
    pub fromentity: Option<PayrixId>,

    /// The identifier of the Entity associated with this Adjustment.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelOnentity)
    #[serde(default)]
    pub onentity: Option<PayrixId>,

    /// The identifier of the Disbursement associated with this Adjustment.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelDisbursement)
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// The description of this Adjustment.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The amount of the Adjustment.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    /// The currency for the amount of this resource.
    ///
    /// See [Currency codes](https://www.iban.com/currency-codes) for all valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The ID of the fund which the disbursement's movement applies.
    ///
    /// **OpenAPI type:** string (ref: adjustmentsModelFunding)
    #[serde(default)]
    pub funding: Option<PayrixId>,

    /// The processor that issued this adjustment.
    ///
    /// **OpenAPI type:** string
    ///
    /// Valid values: `APPLE`, `ELAVON`, `FIRSTDATA`, `GOOGLE`, `VANTIV`, `VCORE`,
    /// `WELLSACH`, `WELLSFARGO`, `WFSINGLE`, `WORLDPAY`, `TDBANKCA`
    #[serde(default)]
    pub platform: Option<String>,

    /// FBO account (For-Benefit-Of account) identifier.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fbo: Option<String>,

    /// Entry associated with this adjustment.
    ///
    /// **OpenAPI type:** object (ref: entriesResponse)
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entry: Option<serde_json::Value>,

    /// Entry origins associated with this adjustment.
    ///
    /// **OpenAPI type:** array of entryOriginsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entry_origins: Option<Vec<serde_json::Value>>,

    /// Pending entry associated with this adjustment.
    ///
    /// **OpenAPI type:** object (ref: pendingEntriesResponse)
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub pending_entry: Option<serde_json::Value>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Adjustment Struct Tests ====================

    #[test]
    fn adjustment_deserialize_full() {
        let json = r#"{
            "id": "t1_adj_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "entity": "t1_ent_12345678901234567890123",
            "fromentity": "t1_ent_12345678901234567890124",
            "onentity": "t1_ent_12345678901234567890125",
            "disbursement": "t1_dsb_12345678901234567890123",
            "description": "Balance adjustment",
            "amount": 5000.50,
            "currency": "USD",
            "funding": "t1_fnd_12345678901234567890123",
            "platform": "VANTIV",
            "fbo": "FBO123456"
        }"#;

        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.id.as_str(), "t1_adj_12345678901234567890123");
        assert_eq!(adj.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(adj.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            adj.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            adj.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            adj.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            adj.entity.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(
            adj.fromentity.as_ref().map(|f| f.as_str()),
            Some("t1_ent_12345678901234567890124")
        );
        assert_eq!(
            adj.onentity.as_ref().map(|o| o.as_str()),
            Some("t1_ent_12345678901234567890125")
        );
        assert_eq!(
            adj.disbursement.as_ref().map(|d| d.as_str()),
            Some("t1_dsb_12345678901234567890123")
        );
        assert_eq!(adj.description, Some("Balance adjustment".to_string()));
        assert_eq!(adj.amount, Some(5000.50));
        assert_eq!(adj.currency, Some("USD".to_string()));
        assert_eq!(
            adj.funding.as_ref().map(|f| f.as_str()),
            Some("t1_fnd_12345678901234567890123")
        );
        assert_eq!(adj.platform, Some("VANTIV".to_string()));
        assert_eq!(adj.fbo, Some("FBO123456".to_string()));
    }

    #[test]
    fn adjustment_deserialize_minimal() {
        let json = r#"{"id": "t1_adj_12345678901234567890123"}"#;

        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.id.as_str(), "t1_adj_12345678901234567890123");
        assert!(adj.created.is_none());
        assert!(adj.modified.is_none());
        assert!(adj.creator.is_none());
        assert!(adj.modifier.is_none());
        assert!(adj.login.is_none());
        assert!(adj.entity.is_none());
        assert!(adj.fromentity.is_none());
        assert!(adj.onentity.is_none());
        assert!(adj.disbursement.is_none());
        assert!(adj.description.is_none());
        assert!(adj.amount.is_none());
        assert!(adj.currency.is_none());
        assert!(adj.funding.is_none());
        assert!(adj.platform.is_none());
        assert!(adj.fbo.is_none());
    }

    #[test]
    fn adjustment_amount_as_float() {
        let json = r#"{"id": "t1_adj_12345678901234567890123", "amount": 1234.567}"#;
        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.amount, Some(1234.567));

        let json = r#"{"id": "t1_adj_12345678901234567890123", "amount": 5000}"#;
        let adj: Adjustment = serde_json::from_str(json).unwrap();
        assert_eq!(adj.amount, Some(5000.0));
    }

    #[test]
    fn adjustment_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_adj_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "amount": 5000.0,
            "currency": "USD"
        }"#;

        let adj: Adjustment = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&adj).unwrap();
        let deserialized: Adjustment = serde_json::from_str(&serialized).unwrap();
        assert_eq!(adj.id, deserialized.id);
        assert_eq!(adj.entity, deserialized.entity);
        assert_eq!(adj.amount, deserialized.amount);
        assert_eq!(adj.currency, deserialized.currency);
    }
}
