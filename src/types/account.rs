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
/// Bank accounts are used for merchant funding (receiving deposits) and
/// debiting (fee withdrawals, refunds). An entity can have multiple accounts
/// for different purposes.
///
/// # Creating an Account
///
/// When creating a new account, the following fields are required:
/// - `entity` - Parent entity ID
/// - `routing` - Bank routing number (9 digits) OR `public_token` for Plaid
/// - `account` - Bank account number OR `public_token` for Plaid
/// - `holder_type` - Individual or Business
///
/// Read-only fields (returned by API, not sent on create):
/// - `id` - Assigned by Payrix
/// - `login` - Set by API based on authentication
/// - `last4` - Derived from account number
/// - `status`, `verified` - Set by verification process
/// - `created`, `modified` - Timestamps set by API
///
/// # Account Types
///
/// The `account_type` field controls what transactions can use this account:
/// - `All` - Both credits (deposits) and debits (withdrawals). Use for primary
///   operating accounts that receive deposits and have fees deducted.
/// - `Credit` - Deposits only, no fee withdrawals. Use for trust accounts,
///   client funds accounts, or accounts that should only receive money.
/// - `Debit` - Withdrawals only. Rarely used, for special fee accounts.
///
/// # Primary Account
///
/// One account should be marked as `primary: true`. The primary account is used
/// for fee deductions and as the default funding destination.
///
/// # Bank Account Entry Methods
///
/// You can add bank accounts two ways:
/// 1. **Manual Entry** - Provide `routing` and `account` numbers directly
/// 2. **Plaid Integration** - Provide a `public_token` from Plaid Link
///
/// # Trust Account + Operating Account Pattern
///
/// Businesses handling client funds (law firms, escrow, property managers)
/// often need two accounts:
/// - **Operating account** (`account_type: All`, `primary: true`) - receives
///   merchant's share of funds, pays processing fees
/// - **Trust account** (`account_type: Credit`, `primary: false`) - receives
///   client funds only, no fee withdrawals allowed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Unique identifier (30 characters, e.g., "t1_act_...").
    ///
    /// **Read-only**: Assigned by Payrix when account is created.
    pub id: PayrixId,

    /// Parent entity ID.
    ///
    /// **Required for creation**. The entity that owns this bank account.
    /// Format: "t1_ent_..." (30 characters).
    pub entity: PayrixId,

    /// Login ID that created this account.
    ///
    /// **Read-only**: Set by Payrix based on the authenticated user.
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Account name/label for identification.
    ///
    /// **Optional**. Use descriptive names like "Operating Account",
    /// "Client Trust Account" to distinguish between multiple accounts.
    #[serde(default)]
    pub name: Option<String>,

    /// Transaction type this account can be used for.
    ///
    /// **Optional** (defaults to `All`).
    /// - `All` - Credits (deposits) and debits (fee withdrawals)
    /// - `Credit` - Deposits only (use for trust accounts)
    /// - `Debit` - Withdrawals only (rare)
    #[serde(default, rename = "type")]
    pub account_type: Option<AccountType>,

    /// Bank routing number (ABA number).
    ///
    /// **Required for creation** (manual entry). 9-digit US bank routing number.
    /// Not needed when using Plaid integration.
    #[serde(default)]
    pub routing: Option<String>,

    /// Bank account number.
    ///
    /// **Required for creation** (manual entry). Full account number.
    /// Not needed when using Plaid integration.
    /// Note: API returns masked value in responses.
    #[serde(default)]
    pub account: Option<String>,

    /// Last 4 digits of account number.
    ///
    /// **Read-only**: Derived from the account number by Payrix.
    #[serde(default)]
    pub last4: Option<String>,

    /// Account holder's first name.
    ///
    /// **Optional**. For individual accounts, the account holder's first name.
    /// Not typically needed for business accounts.
    #[serde(default)]
    pub first: Option<String>,

    /// Account holder's middle name.
    ///
    /// **Optional**.
    #[serde(default)]
    pub middle: Option<String>,

    /// Account holder's last name.
    ///
    /// **Optional**. For individual accounts, the account holder's last name.
    /// Not typically needed for business accounts.
    #[serde(default)]
    pub last: Option<String>,

    /// Bank name.
    ///
    /// **Optional**. Name of the financial institution.
    /// Often auto-populated from routing number.
    #[serde(default)]
    pub bank: Option<String>,

    /// Account holder type.
    ///
    /// **Required for creation**.
    /// - `Individual` - Personal bank account
    /// - `Business` - Business/corporate bank account
    #[serde(default)]
    pub holder_type: Option<AccountHolderType>,

    /// Whether this is the primary account.
    ///
    /// **Optional** (defaults to false). The primary account is used for:
    /// - Default funding destination for deposits
    /// - Fee deductions and other debits
    ///
    /// One account per entity should be marked primary.
    #[serde(default, with = "bool_from_int_default_false")]
    pub primary: bool,

    /// Account verification status.
    ///
    /// **Read-only**: Set by Payrix's verification process.
    /// - `NotReady` - Not yet ready to verify
    /// - `Ready` - Ready to be verified
    /// - `Challenged` - Micro-deposit challenge sent
    /// - `Verified` - Successfully verified
    /// - `Manual` - Needs manual review
    #[serde(default)]
    pub status: Option<AccountStatus>,

    /// Verification status code.
    ///
    /// **Read-only**: Numeric verification state set by Payrix.
    #[serde(default)]
    pub verified: Option<i32>,

    /// Currency code (e.g., "USD").
    ///
    /// **Optional**. ISO 4217 currency code. Defaults to "USD" if not specified.
    #[serde(default)]
    pub currency: Option<String>,

    /// Description/notes for internal reference.
    ///
    /// **Optional**.
    #[serde(default)]
    pub description: Option<String>,

    /// Created timestamp in "YYYY-MM-DD HH:mm:ss.sss" format.
    ///
    /// **Read-only**: Set by Payrix when account is created.
    #[serde(default)]
    pub created: Option<String>,

    /// Last modified timestamp in "YYYY-MM-DD HH:mm:ss.sss" format.
    ///
    /// **Read-only**: Updated by Payrix on changes.
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive.
    ///
    /// **Optional on create**. Set to `true` to create in inactive state.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen.
    ///
    /// **Read-only**: Set by Payrix for compliance/risk reasons.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
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
}
