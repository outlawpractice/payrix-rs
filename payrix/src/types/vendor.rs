//! Vendor types for the Payrix API.
//!
//! Vendors represent third-party recipients for split payments or payouts.
//!
//! **OpenAPI schema:** `vendorsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// VENDOR STRUCT
// =============================================================================

/// A Payrix vendor.
///
/// Vendors are recipients for split payments or payouts.
///
/// **OpenAPI schema:** `vendorsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Vendor {
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

    /// The identifier of the Entity associated with this Vendor resource.
    ///
    /// **OpenAPI type:** string (ref: vendorsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// ID of the division associated with this vendor.
    ///
    /// **OpenAPI type:** string (ref: vendorsModelDivision)
    #[serde(default)]
    pub division: Option<PayrixId>,

    /// Deprecated.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub deal_id: Option<String>,

    /// Different emails the partners want to be reached for onboarding related queries/concerns.
    ///
    /// This is a way of allowing the risk team to manage contacts more efficiently.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub onboarding_contact_emails: Option<String>,

    /// Different emails the partners want to be reached for risk related queries/concerns.
    ///
    /// This is a way of allowing the risk team to manage contacts more efficiently.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub risk_contact_emails: Option<String>,

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

    // ==================== Vendor Struct Tests ====================

    #[test]
    fn vendor_deserialize_full() {
        let json = r#"{
            "id": "t1_vnd_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "division": "t1_div_12345678901234567890123",
            "dealId": "DEAL123",
            "onboardingContactEmails": "onboarding@example.com,support@example.com",
            "riskContactEmails": "risk@example.com",
            "inactive": 0,
            "frozen": 1
        }"#;

        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.id.as_str(), "t1_vnd_12345678901234567890123");
        assert_eq!(vendor.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(vendor.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(vendor.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(vendor.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(vendor.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(vendor.division.as_ref().map(|d| d.as_str()), Some("t1_div_12345678901234567890123"));
        assert_eq!(vendor.deal_id, Some("DEAL123".to_string()));
        assert_eq!(vendor.onboarding_contact_emails, Some("onboarding@example.com,support@example.com".to_string()));
        assert_eq!(vendor.risk_contact_emails, Some("risk@example.com".to_string()));
        assert!(!vendor.inactive);
        assert!(vendor.frozen);
    }

    #[test]
    fn vendor_deserialize_minimal() {
        let json = r#"{"id": "t1_vnd_12345678901234567890123"}"#;

        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.id.as_str(), "t1_vnd_12345678901234567890123");
        assert!(vendor.created.is_none());
        assert!(vendor.modified.is_none());
        assert!(vendor.creator.is_none());
        assert!(vendor.modifier.is_none());
        assert!(vendor.entity.is_none());
        assert!(vendor.division.is_none());
        assert!(vendor.deal_id.is_none());
        assert!(vendor.onboarding_contact_emails.is_none());
        assert!(vendor.risk_contact_emails.is_none());
        assert!(!vendor.inactive);
        assert!(!vendor.frozen);
    }

    #[test]
    fn vendor_bool_from_int() {
        let json = r#"{"id": "t1_vnd_12345678901234567890123", "inactive": 1, "frozen": 0}"#;
        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert!(vendor.inactive);
        assert!(!vendor.frozen);
    }

    #[test]
    fn vendor_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_vnd_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "riskContactEmails": "risk@example.com"
        }"#;

        let vendor: Vendor = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&vendor).unwrap();
        let deserialized: Vendor = serde_json::from_str(&serialized).unwrap();
        assert_eq!(vendor.id, deserialized.id);
        assert_eq!(vendor.entity, deserialized.entity);
        assert_eq!(vendor.risk_contact_emails, deserialized.risk_contact_emails);
    }
}
