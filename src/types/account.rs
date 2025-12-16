//! Account types for the Payrix API.
//!
//! Accounts represent bank accounts used for funding and disbursements.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

/// Account holder type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountHolderType {
    /// Individual/personal account
    #[default]
    Individual = 1,
    /// Business/corporate account
    Business = 2,
}

/// Account status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountStatus {
    /// Account holder is not yet ready to verify
    NotReady = 0,
    /// Account is ready to be verified
    #[default]
    Ready = 1,
    /// Account has processed the challenge
    Challenged = 2,
    /// Account has been verified
    Verified = 3,
    /// Manual intervention required for verification
    Manual = 4,
}

/// Account reserved status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountReserved {
    /// No reserve
    #[default]
    NoReserve = 0,
    /// Account withheld
    AccountWithheld = 1,
    /// Account usage pending manual review
    AccountUsagePendingManualReview = 3,
    /// Move all funds into reserve
    MoveAllFundsIntoReserve = 4,
}

/// Account type (credit, debit, or all).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    /// Credit transactions only
    Credit,
    /// Debit transactions only
    Debit,
    /// All transaction types
    #[default]
    All,
}

/// A Payrix bank account.
///
/// Accounts are used for merchant funding and disbursements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Unique identifier (30 characters, e.g., "t1_act_...")
    pub id: PayrixId,

    /// Entity ID that owns this account (required)
    pub entity: PayrixId,

    /// Login ID that created this account
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Account name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Account type (credit, debit, or all)
    #[serde(default, rename = "type")]
    pub account_type: Option<AccountType>,

    /// Bank routing number
    #[serde(default)]
    pub routing: Option<String>,

    /// Bank account number (masked)
    #[serde(default)]
    pub account: Option<String>,

    /// Last 4 digits of account number
    #[serde(default)]
    pub last4: Option<String>,

    /// Account holder's first name
    #[serde(default)]
    pub first: Option<String>,

    /// Account holder's middle name
    #[serde(default)]
    pub middle: Option<String>,

    /// Account holder's last name
    #[serde(default)]
    pub last: Option<String>,

    /// Bank name
    #[serde(default)]
    pub bank: Option<String>,

    /// Account holder type
    #[serde(default)]
    pub holder_type: Option<AccountHolderType>,

    /// Whether this is the primary account
    #[serde(default, with = "bool_from_int_default_false")]
    pub primary: bool,

    /// Account status
    #[serde(default)]
    pub status: Option<AccountStatus>,

    /// Verification status
    #[serde(default)]
    pub verified: Option<i32>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Description/notes
    #[serde(default)]
    pub description: Option<String>,

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

/// Request to create a new account.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAccount {
    /// Entity ID (required)
    pub entity: String,

    /// Account name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Account type (credit, debit, or all)
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub account_type: Option<AccountType>,

    /// Bank routing number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,

    /// Bank account number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Account holder's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Account holder's middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// Account holder's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Bank name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,

    /// Account holder type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_type: Option<AccountHolderType>,

    /// Whether this is the primary account
    #[serde(skip_serializing_if = "Option::is_none", with = "super::option_bool_from_int")]
    pub primary: Option<bool>,

    /// Description/notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AccountHolderType Tests ====================

    #[test]
    fn account_holder_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountHolderType::Individual).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountHolderType::Business).unwrap(), "2");
    }

    #[test]
    fn account_holder_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountHolderType>("1").unwrap(), AccountHolderType::Individual);
        assert_eq!(serde_json::from_str::<AccountHolderType>("2").unwrap(), AccountHolderType::Business);
    }

    #[test]
    fn account_holder_type_default() {
        assert_eq!(AccountHolderType::default(), AccountHolderType::Individual);
    }

    #[test]
    fn account_holder_type_invalid_value() {
        assert!(serde_json::from_str::<AccountHolderType>("0").is_err());
        assert!(serde_json::from_str::<AccountHolderType>("99").is_err());
    }

    // ==================== AccountStatus Tests ====================

    #[test]
    fn account_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountStatus::NotReady).unwrap(), "0");
        assert_eq!(serde_json::to_string(&AccountStatus::Ready).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountStatus::Challenged).unwrap(), "2");
        assert_eq!(serde_json::to_string(&AccountStatus::Verified).unwrap(), "3");
        assert_eq!(serde_json::to_string(&AccountStatus::Manual).unwrap(), "4");
    }

    #[test]
    fn account_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountStatus>("0").unwrap(), AccountStatus::NotReady);
        assert_eq!(serde_json::from_str::<AccountStatus>("1").unwrap(), AccountStatus::Ready);
        assert_eq!(serde_json::from_str::<AccountStatus>("2").unwrap(), AccountStatus::Challenged);
        assert_eq!(serde_json::from_str::<AccountStatus>("3").unwrap(), AccountStatus::Verified);
        assert_eq!(serde_json::from_str::<AccountStatus>("4").unwrap(), AccountStatus::Manual);
    }

    #[test]
    fn account_status_default() {
        assert_eq!(AccountStatus::default(), AccountStatus::Ready);
    }

    #[test]
    fn account_status_invalid_value() {
        assert!(serde_json::from_str::<AccountStatus>("99").is_err());
    }

    // ==================== AccountReserved Tests ====================

    #[test]
    fn account_reserved_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountReserved::NoReserve).unwrap(), "0");
        assert_eq!(serde_json::to_string(&AccountReserved::AccountWithheld).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountReserved::AccountUsagePendingManualReview).unwrap(), "3");
        assert_eq!(serde_json::to_string(&AccountReserved::MoveAllFundsIntoReserve).unwrap(), "4");
    }

    #[test]
    fn account_reserved_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountReserved>("0").unwrap(), AccountReserved::NoReserve);
        assert_eq!(serde_json::from_str::<AccountReserved>("1").unwrap(), AccountReserved::AccountWithheld);
        assert_eq!(serde_json::from_str::<AccountReserved>("3").unwrap(), AccountReserved::AccountUsagePendingManualReview);
        assert_eq!(serde_json::from_str::<AccountReserved>("4").unwrap(), AccountReserved::MoveAllFundsIntoReserve);
    }

    #[test]
    fn account_reserved_default() {
        assert_eq!(AccountReserved::default(), AccountReserved::NoReserve);
    }

    #[test]
    fn account_reserved_invalid_value() {
        assert!(serde_json::from_str::<AccountReserved>("2").is_err());
        assert!(serde_json::from_str::<AccountReserved>("99").is_err());
    }

    // ==================== AccountType Tests ====================

    #[test]
    fn account_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountType::Credit).unwrap(), "\"credit\"");
        assert_eq!(serde_json::to_string(&AccountType::Debit).unwrap(), "\"debit\"");
        assert_eq!(serde_json::to_string(&AccountType::All).unwrap(), "\"all\"");
    }

    #[test]
    fn account_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountType>("\"credit\"").unwrap(), AccountType::Credit);
        assert_eq!(serde_json::from_str::<AccountType>("\"debit\"").unwrap(), AccountType::Debit);
        assert_eq!(serde_json::from_str::<AccountType>("\"all\"").unwrap(), AccountType::All);
    }

    #[test]
    fn account_type_default() {
        assert_eq!(AccountType::default(), AccountType::All);
    }

    #[test]
    fn account_type_invalid_value() {
        assert!(serde_json::from_str::<AccountType>("\"invalid\"").is_err());
    }

    // ==================== Account Struct Tests ====================

    #[test]
    fn account_deserialize_full() {
        // NOTE: API returns string values for type (e.g., "credit", "debit", "all")
        let json = r#"{
            "id": "t1_act_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "name": "Primary Checking",
            "type": "credit",
            "routing": "123456789",
            "account": "****1234",
            "last4": "1234",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "bank": "Chase Bank",
            "holderType": 1,
            "primary": 1,
            "status": 3,
            "verified": 1,
            "currency": "USD",
            "description": "Primary business account",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.id.as_str(), "t1_act_12345678901234567890123");
        assert_eq!(account.entity.as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(account.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(account.name.unwrap(), "Primary Checking");
        assert_eq!(account.account_type, Some(AccountType::Credit));
        assert_eq!(account.routing.unwrap(), "123456789");
        assert_eq!(account.last4.unwrap(), "1234");
        assert_eq!(account.first.unwrap(), "John");
        assert_eq!(account.last.unwrap(), "Doe");
        assert_eq!(account.bank.unwrap(), "Chase Bank");
        assert_eq!(account.holder_type, Some(AccountHolderType::Individual));
        assert!(account.primary);
        assert_eq!(account.status, Some(AccountStatus::Verified));
        assert!(!account.inactive);
        assert!(account.frozen);
    }

    #[test]
    fn account_deserialize_minimal() {
        let json = r#"{
            "id": "t1_act_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123"
        }"#;

        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.id.as_str(), "t1_act_12345678901234567890123");
        assert_eq!(account.entity.as_str(), "t1_ent_12345678901234567890123");
        assert!(account.login.is_none());
        assert!(account.name.is_none());
        assert!(account.status.is_none());
        assert!(!account.primary);
        assert!(!account.inactive);
        assert!(!account.frozen);
    }

    #[test]
    fn account_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_act_12345678901234567890123", "entity": "t1_ent_12345678901234567890123", "primary": 0, "inactive": 0, "frozen": 0}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert!(!account.primary);
        assert!(!account.inactive);
        assert!(!account.frozen);
    }

    #[test]
    fn account_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_act_12345678901234567890123", "entity": "t1_ent_12345678901234567890123", "primary": 1, "inactive": 1, "frozen": 1}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert!(account.primary);
        assert!(account.inactive);
        assert!(account.frozen);
    }

    #[test]
    fn account_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_act_12345678901234567890123", "entity": "t1_ent_12345678901234567890123"}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert!(!account.primary);
        assert!(!account.inactive);
        assert!(!account.frozen);
    }

    // ==================== NewAccount Tests ====================

    #[test]
    fn new_account_serialize_full() {
        let new_account = NewAccount {
            entity: "t1_ent_12345678901234567890123".to_string(),
            name: Some("Primary Checking".to_string()),
            account_type: Some(AccountType::Credit),
            routing: Some("123456789".to_string()),
            account: Some("987654321".to_string()),
            first: Some("John".to_string()),
            middle: Some("Q".to_string()),
            last: Some("Doe".to_string()),
            bank: Some("Chase Bank".to_string()),
            holder_type: Some(AccountHolderType::Individual),
            primary: Some(true),
            description: Some("Primary business account".to_string()),
        };

        let json = serde_json::to_string(&new_account).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"name\":\"Primary Checking\""));
        assert!(json.contains("\"type\":\"credit\""));
        assert!(json.contains("\"holderType\":1"));
        assert!(json.contains("\"primary\":1"));
    }

    #[test]
    fn new_account_serialize_minimal() {
        let new_account = NewAccount {
            entity: "t1_ent_12345678901234567890123".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_account).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"primary\""));
    }

    #[test]
    fn new_account_option_bool_to_int_true() {
        let new_account = NewAccount {
            entity: "t1_ent_12345678901234567890123".to_string(),
            primary: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_account).unwrap();
        assert!(json.contains("\"primary\":1"));
    }

    #[test]
    fn new_account_option_bool_to_int_false() {
        let new_account = NewAccount {
            entity: "t1_ent_12345678901234567890123".to_string(),
            primary: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_account).unwrap();
        assert!(json.contains("\"primary\":0"));
    }
}
