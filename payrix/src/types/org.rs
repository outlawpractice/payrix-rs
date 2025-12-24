//! Organization types for the Payrix API.
//!
//! Organizations group entities together for administrative purposes.
//!
//! **OpenAPI schema:** `orgsResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ORG STRUCT
// =============================================================================

/// A Payrix organization.
///
/// Organizations group entities for management and reporting.
///
/// **OpenAPI schema:** `orgsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Org {
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

    /// The Login that owns this Org.
    ///
    /// **OpenAPI type:** string (ref: orgsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The name of this Org.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Org.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// This is used to specify a default fee for a group.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub default_fee_from_entity: Option<String>,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Aggregations associated with this org.
    ///
    /// **OpenAPI type:** array of aggregationsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub aggregations: Option<Vec<serde_json::Value>>,

    /// Billing modifiers associated with this org.
    ///
    /// **OpenAPI type:** array of billingModifiersResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub billing_modifiers: Option<Vec<serde_json::Value>>,

    /// Billings associated with this org.
    ///
    /// **OpenAPI type:** array of billingsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub billings: Option<Vec<serde_json::Value>>,

    /// Decisions associated with this org.
    ///
    /// **OpenAPI type:** array of decisionsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub decisions: Option<Vec<serde_json::Value>>,

    /// Embedded finance configuration associated with this org.
    ///
    /// **OpenAPI type:** array of embeddedFinanceResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub embedded_finance: Option<Vec<serde_json::Value>>,

    /// Fee modifiers associated with this org.
    ///
    /// **OpenAPI type:** array of feeModifiersResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub fee_modifiers: Option<Vec<serde_json::Value>>,

    /// Fees associated with this org.
    ///
    /// **OpenAPI type:** array of feesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub fees: Option<Vec<serde_json::Value>>,

    /// Invoice parameters associated with this org.
    ///
    /// **OpenAPI type:** array of invoiceParametersResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub invoice_parameters: Option<Vec<serde_json::Value>>,

    /// Org entities associated with this org.
    ///
    /// **OpenAPI type:** array of orgEntitiesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub org_entities: Option<Vec<serde_json::Value>>,

    /// Omni tokens associated with this org.
    ///
    /// **OpenAPI type:** array of omniTokensResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub omni_tokens: Option<Vec<serde_json::Value>>,

    /// Payout flows associated with this org.
    ///
    /// **OpenAPI type:** array of payoutFlowsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub payout_flows: Option<Vec<serde_json::Value>>,

    /// Profit shares associated with this org.
    ///
    /// **OpenAPI type:** array of profitSharesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub profit_shares: Option<Vec<serde_json::Value>>,

    /// Reserves associated with this org.
    ///
    /// **OpenAPI type:** array of reservesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserves: Option<Vec<serde_json::Value>>,

    /// Revenue boosts associated with this org.
    ///
    /// **OpenAPI type:** array of revenueBoostsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub revenue_boosts: Option<Vec<serde_json::Value>>,

    /// Secrets associated with this org.
    ///
    /// **OpenAPI type:** array of secretsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub secrets: Option<Vec<serde_json::Value>>,

    /// Safer payments configuration associated with this org.
    ///
    /// **OpenAPI type:** array of saferPaymentsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub safer_payments: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Org Struct Tests ====================

    #[test]
    fn org_deserialize_full() {
        let json = r#"{
            "id": "t1_org_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "name": "Test Organization",
            "description": "An organization for testing",
            "defaultFeeFromEntity": "t1_ent_12345678901234567890123"
        }"#;

        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.id.as_str(), "t1_org_12345678901234567890123");
        assert_eq!(org.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(org.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(org.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(org.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(org.login.as_ref().map(|l| l.as_str()), Some("t1_lgn_12345678901234567890125"));
        assert_eq!(org.name, Some("Test Organization".to_string()));
        assert_eq!(org.description, Some("An organization for testing".to_string()));
        assert_eq!(org.default_fee_from_entity, Some("t1_ent_12345678901234567890123".to_string()));
    }

    #[test]
    fn org_deserialize_minimal() {
        let json = r#"{"id": "t1_org_12345678901234567890123"}"#;

        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.id.as_str(), "t1_org_12345678901234567890123");
        assert!(org.created.is_none());
        assert!(org.modified.is_none());
        assert!(org.creator.is_none());
        assert!(org.modifier.is_none());
        assert!(org.login.is_none());
        assert!(org.name.is_none());
        assert!(org.description.is_none());
        assert!(org.default_fee_from_entity.is_none());
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn org_with_nested_relations() {
        let json = r#"{
            "id": "t1_org_12345678901234567890123",
            "fees": [{"id": "t1_fee_12345678901234567890123"}],
            "reserves": [{"id": "t1_rsv_12345678901234567890123"}],
            "billings": []
        }"#;

        let org: Org = serde_json::from_str(json).unwrap();
        assert!(org.fees.is_some());
        assert_eq!(org.fees.as_ref().unwrap().len(), 1);
        assert!(org.reserves.is_some());
        assert_eq!(org.reserves.as_ref().unwrap().len(), 1);
        assert!(org.billings.is_some());
        assert_eq!(org.billings.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn org_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_org_12345678901234567890123",
            "name": "Test Org",
            "description": "Test description"
        }"#;

        let org: Org = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&org).unwrap();
        let deserialized: Org = serde_json::from_str(&serialized).unwrap();
        assert_eq!(org.id, deserialized.id);
        assert_eq!(org.name, deserialized.name);
        assert_eq!(org.description, deserialized.description);
    }
}
