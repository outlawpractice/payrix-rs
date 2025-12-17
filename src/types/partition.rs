//! Partition types for the Payrix API.
//!
//! Partitions represent isolated environments within the Payrix hierarchy,
//! providing separation of data and configuration.
//!
//! **OpenAPI schema:** `partitionsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// PARTITION STRUCT
// =============================================================================

/// A Payrix partition.
///
/// Partitions represent isolated environments within the Payrix hierarchy,
/// providing separation of data and configuration.
///
/// **OpenAPI schema:** `partitionsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Partition {
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
    /// **OpenAPI type:** string
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The name of this Partition.
    ///
    /// This field is stored as a text string and must be between 0 and 50 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// The email address that the API should send email notifications from when processing
    /// requests in this Partition.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// The minimum number of characters that all passwords used in this Partition must have.
    ///
    /// This field is specified as an integer. The value must be between 8 and 16.
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

    /// The minimum number of previous passwords that the current password must be distinct from.
    ///
    /// This field is specified as an integer. The value must be between 3 and 12.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub min_password_history: Option<i32>,

    /// Whether to disable the email address confirmation process for this Partition.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: NoEmailConfirmation)
    #[serde(default, with = "bool_from_int_default_false")]
    pub no_email_confirmation: bool,

    /// Whether to disable email notification for automated hold messages.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: NoHoldEmail)
    #[serde(default, with = "bool_from_int_default_false")]
    pub no_hold_email: bool,

    /// Whether change management is enabled.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: changeManagementEnabled)
    #[serde(default, with = "bool_from_int_default_false")]
    pub change_management_enabled: bool,

    /// The currency in which an entity should board.
    ///
    /// See ISO 4217 currency codes for valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_deserialize_full() {
        let json = r#"{
            "id": "t1_prt_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "name": "Test Partition",
            "email": "partition@example.com",
            "minPasswordLength": 12,
            "minPasswordComplexity": 3,
            "minPasswordHistory": 5,
            "noEmailConfirmation": 0,
            "noHoldEmail": 1,
            "changeManagementEnabled": 1,
            "currency": "USD"
        }"#;

        let partition: Partition = serde_json::from_str(json).unwrap();
        assert_eq!(partition.id.as_str(), "t1_prt_12345678901234567890123");
        assert_eq!(partition.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(partition.name, Some("Test Partition".to_string()));
        assert_eq!(partition.email, Some("partition@example.com".to_string()));
        assert_eq!(partition.min_password_length, Some(12));
        assert_eq!(partition.min_password_complexity, Some(3));
        assert_eq!(partition.min_password_history, Some(5));
        assert!(!partition.no_email_confirmation);
        assert!(partition.no_hold_email);
        assert!(partition.change_management_enabled);
        assert_eq!(partition.currency, Some("USD".to_string()));
    }

    #[test]
    fn partition_deserialize_minimal() {
        let json = r#"{"id": "t1_prt_12345678901234567890123"}"#;

        let partition: Partition = serde_json::from_str(json).unwrap();
        assert_eq!(partition.id.as_str(), "t1_prt_12345678901234567890123");
        assert!(partition.created.is_none());
        assert!(partition.name.is_none());
        assert!(!partition.change_management_enabled);
    }

    #[test]
    fn partition_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_prt_12345678901234567890123",
            "name": "Test Partition",
            "currency": "CAD"
        }"#;

        let partition: Partition = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&partition).unwrap();
        let deserialized: Partition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(partition.id, deserialized.id);
        assert_eq!(partition.name, deserialized.name);
        assert_eq!(partition.currency, deserialized.currency);
    }
}
