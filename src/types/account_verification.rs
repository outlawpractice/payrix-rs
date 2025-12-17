//! Account verification types for the Payrix API.
//!
//! Account verifications track the verification status of bank accounts.
//!
//! **OpenAPI schema:** `accountVerificationResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ENUMS
// =============================================================================

/// Account verification type values per OpenAPI spec.
///
/// **OpenAPI schema:** `accountVerificationType`
///
/// Valid values:
/// - `debit` - Makes two challenge debits (debit1 and debit2)
/// - `credentials` - Uses a bank account credential (username/password)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountVerificationType {
    /// Makes two challenge debits (debit1 and debit2).
    #[default]
    Debit,
    /// Uses a bank account credential (username/password).
    Credentials,
}

// =============================================================================
// ACCOUNT VERIFICATION STRUCT
// =============================================================================

/// A Payrix account verification.
///
/// Account verifications confirm bank account ownership through challenge debits
/// or credential verification.
///
/// **OpenAPI schema:** `accountVerificationResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AccountVerification {
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

    /// The identifier of the Account that you want to verify.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// The type of account verification you want to perform.
    ///
    /// - `debit` - Makes two challenge debits (debit1 and debit2)
    /// - `credentials` - Uses a bank account credential (username/password)
    ///
    /// **OpenAPI type:** string (ref: accountVerificationType)
    #[serde(default, rename = "type")]
    pub verification_type: Option<AccountVerificationType>,

    /// The first verification amount debited in cents.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub debit1: Option<i32>,

    /// The second verification amount debited in cents.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub debit2: Option<i32>,

    /// The number of verification challenge responses attempted.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub trials: Option<i32>,

    /// Account that will be deposited with both amounts: debit1 and debit2.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub toaccount: Option<PayrixId>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AccountVerificationType Tests ====================

    #[test]
    fn account_verification_type_serialize_all_variants() {
        assert_eq!(
            serde_json::to_string(&AccountVerificationType::Debit).unwrap(),
            "\"debit\""
        );
        assert_eq!(
            serde_json::to_string(&AccountVerificationType::Credentials).unwrap(),
            "\"credentials\""
        );
    }

    #[test]
    fn account_verification_type_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<AccountVerificationType>("\"debit\"").unwrap(),
            AccountVerificationType::Debit
        );
        assert_eq!(
            serde_json::from_str::<AccountVerificationType>("\"credentials\"").unwrap(),
            AccountVerificationType::Credentials
        );
    }

    #[test]
    fn account_verification_type_default() {
        assert_eq!(
            AccountVerificationType::default(),
            AccountVerificationType::Debit
        );
    }

    #[test]
    fn account_verification_type_invalid_value() {
        assert!(serde_json::from_str::<AccountVerificationType>("\"invalid\"").is_err());
        assert!(serde_json::from_str::<AccountVerificationType>("\"DEBIT\"").is_err());
    }

    // ==================== AccountVerification Struct Tests ====================

    #[test]
    fn account_verification_deserialize_full() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "account": "t1_acc_12345678901234567890123",
            "type": "debit",
            "debit1": 12,
            "debit2": 34,
            "trials": 2,
            "toaccount": "t1_acc_98765432109876543210987"
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(verification.id.as_str(), "t1_acv_12345678901234567890123");
        assert_eq!(
            verification.created,
            Some("2024-01-01 00:00:00.0000".to_string())
        );
        assert_eq!(
            verification.modified,
            Some("2024-01-02 23:59:59.9999".to_string())
        );
        assert_eq!(
            verification.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            verification.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            verification.account.as_ref().map(|a| a.as_str()),
            Some("t1_acc_12345678901234567890123")
        );
        assert_eq!(
            verification.verification_type,
            Some(AccountVerificationType::Debit)
        );
        assert_eq!(verification.debit1, Some(12));
        assert_eq!(verification.debit2, Some(34));
        assert_eq!(verification.trials, Some(2));
        assert_eq!(
            verification.toaccount.as_ref().map(|t| t.as_str()),
            Some("t1_acc_98765432109876543210987")
        );
    }

    #[test]
    fn account_verification_deserialize_credentials_type() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123",
            "type": "credentials"
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(
            verification.verification_type,
            Some(AccountVerificationType::Credentials)
        );
    }

    #[test]
    fn account_verification_deserialize_minimal() {
        let json = r#"{"id": "t1_acv_12345678901234567890123"}"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(verification.id.as_str(), "t1_acv_12345678901234567890123");
        assert!(verification.created.is_none());
        assert!(verification.modified.is_none());
        assert!(verification.creator.is_none());
        assert!(verification.modifier.is_none());
        assert!(verification.account.is_none());
        assert!(verification.verification_type.is_none());
        assert!(verification.debit1.is_none());
        assert!(verification.debit2.is_none());
        assert!(verification.trials.is_none());
        assert!(verification.toaccount.is_none());
    }

    #[test]
    fn account_verification_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123",
            "account": "t1_acc_12345678901234567890123",
            "type": "debit",
            "debit1": 17,
            "debit2": 23
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&verification).unwrap();
        let deserialized: AccountVerification = serde_json::from_str(&serialized).unwrap();
        assert_eq!(verification.id, deserialized.id);
        assert_eq!(verification.account, deserialized.account);
        assert_eq!(verification.verification_type, deserialized.verification_type);
        assert_eq!(verification.debit1, deserialized.debit1);
        assert_eq!(verification.debit2, deserialized.debit2);
    }
}
