//! Division types for the Payrix API.
//!
//! Divisions represent organizational units within the Payrix hierarchy,
//! used for grouping and managing entities.
//!
//! **OpenAPI schema:** `divisionsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// DIVISION STRUCT
// =============================================================================

/// A Payrix division.
///
/// Divisions represent organizational units within the Payrix hierarchy,
/// used for grouping and managing entities.
///
/// **OpenAPI schema:** `divisionsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Division {
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

    /// The login ID of the user that owns this division record.
    ///
    /// **OpenAPI type:** string (ref: divisionsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The name of the division, which may be used for some white-label purposes.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// The white-labeled outgoing email for all automated emails generated throughout this division.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// Whether change management is enabled.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: changeManagementEnabled)
    #[serde(default, with = "bool_from_int_default_false")]
    pub change_management_enabled: bool,

    /// Minimum password length.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub min_password_length: Option<i32>,

    /// The minimum level of complexity that all passwords used in this Partition must have.
    ///
    /// There are four complexity 'factors': lowercase, uppercase, integers, and special characters.
    ///
    /// - `1` - Needs to include one type
    /// - `2` - Needs to include two types
    /// - `3` - Needs to include three types
    /// - `4` - Needs to include four types
    ///
    /// **OpenAPI type:** integer (ref: minPasswordComplexity)
    #[serde(default)]
    pub min_password_complexity: Option<i32>,

    /// Describes if the user can use the Plaid Wrapper Microservice or not.
    ///
    /// - `0` - No
    /// - `1` - Yes
    ///
    /// **OpenAPI type:** integer (ref: CanUsePlaidWrapperMicroservice)
    #[serde(default, with = "bool_from_int_default_false")]
    pub can_use_plaid_wrapper_microservice: bool,

    /// Allows automatic enablement of simplified deposits for new boardings.
    ///
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: SimplifiedDepositEnabled)
    #[serde(default, with = "bool_from_int_default_false")]
    pub simplified_deposit_enabled: bool,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_deserialize_full() {
        let json = r#"{
            "id": "t1_div_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "name": "Main Division",
            "email": "support@example.com",
            "changeManagementEnabled": 1,
            "minPasswordLength": 12,
            "minPasswordComplexity": 3,
            "canUsePlaidWrapperMicroservice": 1,
            "simplifiedDepositEnabled": 0
        }"#;

        let division: Division = serde_json::from_str(json).unwrap();
        assert_eq!(division.id.as_str(), "t1_div_12345678901234567890123");
        assert_eq!(division.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(division.name, Some("Main Division".to_string()));
        assert_eq!(division.email, Some("support@example.com".to_string()));
        assert!(division.change_management_enabled);
        assert_eq!(division.min_password_length, Some(12));
        assert_eq!(division.min_password_complexity, Some(3));
        assert!(division.can_use_plaid_wrapper_microservice);
        assert!(!division.simplified_deposit_enabled);
    }

    #[test]
    fn division_deserialize_minimal() {
        let json = r#"{"id": "t1_div_12345678901234567890123"}"#;

        let division: Division = serde_json::from_str(json).unwrap();
        assert_eq!(division.id.as_str(), "t1_div_12345678901234567890123");
        assert!(division.created.is_none());
        assert!(division.name.is_none());
        assert!(!division.change_management_enabled);
    }

    #[test]
    fn division_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_div_12345678901234567890123",
            "name": "Test Division",
            "minPasswordLength": 10
        }"#;

        let division: Division = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&division).unwrap();
        let deserialized: Division = serde_json::from_str(&serialized).unwrap();
        assert_eq!(division.id, deserialized.id);
        assert_eq!(division.name, deserialized.name);
        assert_eq!(division.min_password_length, deserialized.min_password_length);
    }
}
