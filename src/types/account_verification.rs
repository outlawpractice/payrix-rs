//! Account verification types for the Payrix API.
//!
//! Account verifications track the verification status of bank accounts.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

/// Account verification status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountVerificationStatus {
    /// Verification pending
    #[default]
    Pending = 0,
    /// Micro-deposits sent
    Sent = 1,
    /// Verification in progress
    InProgress = 2,
    /// Verification successful
    Verified = 3,
    /// Verification failed
    Failed = 4,
    /// Verification expired
    Expired = 5,
}

/// Account verification method values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountVerificationMethod {
    /// Micro-deposit verification
    #[default]
    MicroDeposit = 1,
    /// Instant verification (Plaid, etc.)
    Instant = 2,
    /// Manual verification (documents)
    Manual = 3,
}

/// A Payrix account verification.
///
/// Account verifications confirm bank account ownership.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AccountVerification {
    /// Unique identifier (30 characters, e.g., "t1_acv_...")
    pub id: PayrixId,

    /// Account ID being verified
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that initiated verification
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Verification status
    #[serde(default)]
    pub status: Option<AccountVerificationStatus>,

    /// Verification method
    #[serde(default)]
    pub method: Option<AccountVerificationMethod>,

    /// Number of verification attempts
    #[serde(default)]
    pub attempts: Option<i32>,

    /// Maximum allowed attempts
    #[serde(default)]
    pub max_attempts: Option<i32>,

    /// First micro-deposit amount in cents (if applicable)
    #[serde(default)]
    pub amount1: Option<i64>,

    /// Second micro-deposit amount in cents (if applicable)
    #[serde(default)]
    pub amount2: Option<i64>,

    /// Verification expiration date
    #[serde(default)]
    pub expires: Option<String>,

    /// Description/notes
    #[serde(default)]
    pub description: Option<String>,

    /// Custom data field
    #[serde(default)]
    pub custom: Option<String>,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub created: Option<String>,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive (false=active, true=inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AccountVerificationStatus Tests ====================

    #[test]
    fn account_verification_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::Sent).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::InProgress).unwrap(), "2");
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::Verified).unwrap(), "3");
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::Failed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&AccountVerificationStatus::Expired).unwrap(), "5");
    }

    #[test]
    fn account_verification_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("0").unwrap(), AccountVerificationStatus::Pending);
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("1").unwrap(), AccountVerificationStatus::Sent);
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("2").unwrap(), AccountVerificationStatus::InProgress);
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("3").unwrap(), AccountVerificationStatus::Verified);
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("4").unwrap(), AccountVerificationStatus::Failed);
        assert_eq!(serde_json::from_str::<AccountVerificationStatus>("5").unwrap(), AccountVerificationStatus::Expired);
    }

    #[test]
    fn account_verification_status_default() {
        assert_eq!(AccountVerificationStatus::default(), AccountVerificationStatus::Pending);
    }

    #[test]
    fn account_verification_status_invalid_value() {
        assert!(serde_json::from_str::<AccountVerificationStatus>("6").is_err());
        assert!(serde_json::from_str::<AccountVerificationStatus>("99").is_err());
    }

    // ==================== AccountVerificationMethod Tests ====================

    #[test]
    fn account_verification_method_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountVerificationMethod::MicroDeposit).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountVerificationMethod::Instant).unwrap(), "2");
        assert_eq!(serde_json::to_string(&AccountVerificationMethod::Manual).unwrap(), "3");
    }

    #[test]
    fn account_verification_method_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountVerificationMethod>("1").unwrap(), AccountVerificationMethod::MicroDeposit);
        assert_eq!(serde_json::from_str::<AccountVerificationMethod>("2").unwrap(), AccountVerificationMethod::Instant);
        assert_eq!(serde_json::from_str::<AccountVerificationMethod>("3").unwrap(), AccountVerificationMethod::Manual);
    }

    #[test]
    fn account_verification_method_default() {
        assert_eq!(AccountVerificationMethod::default(), AccountVerificationMethod::MicroDeposit);
    }

    #[test]
    fn account_verification_method_invalid_value() {
        assert!(serde_json::from_str::<AccountVerificationMethod>("0").is_err());
        assert!(serde_json::from_str::<AccountVerificationMethod>("99").is_err());
    }

    // ==================== AccountVerification Struct Tests ====================

    #[test]
    fn account_verification_deserialize_full() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123",
            "account": "t1_acc_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 3,
            "method": 1,
            "attempts": 2,
            "maxAttempts": 3,
            "amount1": 12,
            "amount2": 34,
            "expires": "2024-12-31",
            "description": "Bank account verification",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(verification.id.as_str(), "t1_acv_12345678901234567890123");
        assert_eq!(verification.account.unwrap().as_str(), "t1_acc_12345678901234567890123");
        assert_eq!(verification.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(verification.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(verification.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(verification.status, Some(AccountVerificationStatus::Verified));
        assert_eq!(verification.method, Some(AccountVerificationMethod::MicroDeposit));
        assert_eq!(verification.attempts, Some(2));
        assert_eq!(verification.max_attempts, Some(3));
        assert_eq!(verification.amount1, Some(12));
        assert_eq!(verification.amount2, Some(34));
        assert_eq!(verification.expires, Some("2024-12-31".to_string()));
        assert_eq!(verification.description, Some("Bank account verification".to_string()));
        assert_eq!(verification.custom, Some("custom data".to_string()));
        assert_eq!(verification.created, Some("2024-01-01 00:00:00.000".to_string()));
        assert_eq!(verification.modified, Some("2024-04-01 12:00:00.000".to_string()));
        assert!(!verification.inactive);
        assert!(verification.frozen);
    }

    #[test]
    fn account_verification_deserialize_minimal() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123"
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(verification.id.as_str(), "t1_acv_12345678901234567890123");
        assert!(verification.account.is_none());
        assert!(verification.entity.is_none());
        assert!(verification.merchant.is_none());
        assert!(verification.login.is_none());
        assert!(verification.status.is_none());
        assert!(verification.method.is_none());
        assert!(verification.attempts.is_none());
        assert!(verification.max_attempts.is_none());
        assert!(verification.amount1.is_none());
        assert!(verification.amount2.is_none());
        assert!(verification.expires.is_none());
        assert!(verification.description.is_none());
        assert!(verification.custom.is_none());
        assert!(verification.created.is_none());
        assert!(verification.modified.is_none());
        assert!(!verification.inactive);
        assert!(!verification.frozen);
    }

    #[test]
    fn account_verification_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_acv_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert!(!verification.inactive);
        assert!(!verification.frozen);
    }

    #[test]
    fn account_verification_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_acv_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert!(verification.inactive);
        assert!(verification.frozen);
    }

    #[test]
    fn account_verification_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_acv_12345678901234567890123"}"#;
        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert!(!verification.inactive);
        assert!(!verification.frozen);
    }

    #[test]
    fn account_verification_all_status_variants() {
        let test_cases = vec![
            (0, AccountVerificationStatus::Pending),
            (1, AccountVerificationStatus::Sent),
            (2, AccountVerificationStatus::InProgress),
            (3, AccountVerificationStatus::Verified),
            (4, AccountVerificationStatus::Failed),
            (5, AccountVerificationStatus::Expired),
        ];

        for (status_val, expected_status) in test_cases {
            let json = format!(
                r#"{{"id": "t1_acv_12345678901234567890123", "status": {}}}"#,
                status_val
            );
            let verification: AccountVerification = serde_json::from_str(&json).unwrap();
            assert_eq!(verification.status, Some(expected_status));
        }
    }

    #[test]
    fn account_verification_all_method_variants() {
        let test_cases = vec![
            (1, AccountVerificationMethod::MicroDeposit),
            (2, AccountVerificationMethod::Instant),
            (3, AccountVerificationMethod::Manual),
        ];

        for (method_val, expected_method) in test_cases {
            let json = format!(
                r#"{{"id": "t1_acv_12345678901234567890123", "method": {}}}"#,
                method_val
            );
            let verification: AccountVerification = serde_json::from_str(&json).unwrap();
            assert_eq!(verification.method, Some(expected_method));
        }
    }

    #[test]
    fn account_verification_micro_deposit_amounts() {
        let json = r#"{
            "id": "t1_acv_12345678901234567890123",
            "amount1": 17,
            "amount2": 23
        }"#;

        let verification: AccountVerification = serde_json::from_str(json).unwrap();
        assert_eq!(verification.amount1, Some(17));
        assert_eq!(verification.amount2, Some(23));
    }

    #[test]
    fn account_verification_serialize_roundtrip() {
        let verification = AccountVerification {
            id: "t1_acv_12345678901234567890123".parse().unwrap(),
            account: Some("t1_acc_12345678901234567890123".parse().unwrap()),
            entity: Some("t1_ent_12345678901234567890123".parse().unwrap()),
            merchant: Some("t1_mer_12345678901234567890123".parse().unwrap()),
            login: Some("t1_log_12345678901234567890123".parse().unwrap()),
            status: Some(AccountVerificationStatus::Verified),
            method: Some(AccountVerificationMethod::MicroDeposit),
            attempts: Some(1),
            max_attempts: Some(3),
            amount1: Some(12),
            amount2: Some(34),
            expires: Some("2024-12-31".to_string()),
            description: Some("Test verification".to_string()),
            custom: Some("custom".to_string()),
            created: Some("2024-01-01 00:00:00.000".to_string()),
            modified: Some("2024-04-01 12:00:00.000".to_string()),
            inactive: false,
            frozen: true,
        };

        let json = serde_json::to_string(&verification).unwrap();
        let deserialized: AccountVerification = serde_json::from_str(&json).unwrap();
        assert_eq!(verification, deserialized);
    }
}
