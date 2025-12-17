//! Account types for the Payrix API.
//!
//! Accounts represent bank accounts used for funding and disbursements.
//!
//! **OpenAPI schema:** `accountsResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// ACCOUNT ENUMS
// =============================================================================

/// Account holder type.
///
/// Used for account creation requests. This field is not part of the
/// OpenAPI response schema but is used when creating accounts via the API.
///
/// Valid values:
/// - `1` - Individual (personal account)
/// - `2` - Business (corporate account)
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
///
/// **OpenAPI schema:** `accountStatus`
///
/// Valid values:
/// - `0` - Not Ready
/// - `1` - Ready
/// - `2` - Challenged
/// - `3` - Verified
/// - `4` - Manual
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
///
/// **OpenAPI schema:** `accountsReserved`
///
/// Valid values:
/// - `0` - No reserve
/// - `1` - Block transaction, will never be processed
/// - `3` - Hold transaction, will not be captured
/// - `4` - Reserve transaction, funds should be reserved
/// - `5` - Block current activity, no change for merchant
/// - `6` - Passed decision(s)
/// - `7` - No policies to process
/// - `8` - Onboard merchant and wait for manual check
/// - `9` - Schedule automatic reserve release
/// - `10` - Hold transaction, auto release when sale done
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountReserved {
    /// No reserve
    #[default]
    NoReserve = 0,
    /// Block transaction, will never be processed. Entity sent to manual review queue.
    BlockTransaction = 1,
    /// Hold transaction, will not be captured
    HoldTransaction = 3,
    /// Reserve transaction, funds should be reserved
    ReserveFunds = 4,
    /// Block current activity, no change for merchant
    BlockCurrentActivity = 5,
    /// Passed decision(s). Used for integration purposes only.
    PassedDecisions = 6,
    /// No policies to process
    NoPolicies = 7,
    /// Onboard merchant and wait for manual check later
    OnboardPendingManual = 8,
    /// Schedule automatic release of the reserve
    ScheduleAutoRelease = 9,
    /// Hold transaction, auto release when associated sale is done
    HoldAutoRelease = 10,
}

/// Account type (credit, debit, or all).
///
/// **OpenAPI schema:** `accountType`
///
/// Valid values:
/// - `all` - Debit/checking and credit
/// - `credit` - Credit only
/// - `debit` - Debit/checking only
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    /// All transaction types
    #[default]
    All,
    /// Credit transactions only
    Credit,
    /// Debit transactions only
    Debit,
}

/// Account check stage.
///
/// **OpenAPI schema:** `accountCheckStage`
///
/// Valid values:
/// - `createAccount` - Create Account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AccountCheckStage {
    /// Create Account stage
    #[default]
    #[serde(rename = "createAccount")]
    CreateAccount,
}

/// Account update method.
///
/// **OpenAPI schema:** `UpdateMethod`
///
/// Valid values:
/// - `NOC` - Notification of change
/// - `PLAID` - Plaid
/// - `MANUAL` - Manual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UpdateMethod {
    /// Notification of change
    #[default]
    Noc,
    /// Plaid
    Plaid,
    /// Manual
    Manual,
}

// =============================================================================
// ACCOUNT STRUCT
// =============================================================================

/// A Payrix bank account.
///
/// Bank accounts are used for merchant funding (receiving deposits) and
/// debiting (fee withdrawals, refunds).
///
/// **OpenAPI schema:** `accountsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Account {
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

    /// The identifier of the Entity associated with this Account.
    ///
    /// **OpenAPI type:** string (ref: accountsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Payment associated with this Account.
    ///
    /// **OpenAPI type:** string (ref: accountsModelAccount)
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// A unique token that can be used to refer to this Account in other parts of the API.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub token: Option<String>,

    /// A client-supplied name for this bank account.
    ///
    /// This field is stored as a string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A client-supplied description for this bank account.
    ///
    /// This field is stored as a string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// Whether the Account is the primary Account for the associated Entity.
    ///
    /// Only one Account per Entity can be the primary Account.
    ///
    /// - `0` - Not primary
    /// - `1` - Primary
    ///
    /// **OpenAPI type:** integer (ref: accountPrimary)
    #[serde(default, with = "bool_from_int_default_false")]
    pub primary: bool,

    /// The type of financial account: debit, credit, or both.
    ///
    /// - `all` - Debit/checking and credit
    /// - `credit` - Credit only
    /// - `debit` - Debit/checking only
    ///
    /// **OpenAPI type:** string (ref: accountType)
    #[serde(default, rename = "type")]
    pub account_type: Option<AccountType>,

    /// The status of the Account.
    ///
    /// - `0` - Not Ready
    /// - `1` - Ready
    /// - `2` - Challenged
    /// - `3` - Verified
    /// - `4` - Manual
    ///
    /// **OpenAPI type:** integer (ref: accountStatus)
    #[serde(default)]
    pub status: Option<AccountStatus>,

    /// Whether the Account is reserved and the action to be taken.
    ///
    /// **OpenAPI type:** integer (ref: accountsReserved)
    #[serde(default)]
    pub reserved: Option<AccountReserved>,

    /// The last stage completed for risk.
    ///
    /// - `createAccount` - Create Account
    ///
    /// **OpenAPI type:** string (ref: accountCheckStage)
    #[serde(default)]
    pub check_stage: Option<AccountCheckStage>,

    /// The expiration date of the related debit account.
    ///
    /// Format: MMYY (e.g., `0118` for January 2018).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub expiration: Option<String>,

    /// The currency of this Account.
    ///
    /// Default: `USD`. See ISO 4217 currency codes.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The token received from the Plaid account connection process.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub public_token: Option<String>,

    /// The account number mask, showing the last four digits.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mask: Option<String>,

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

    /// The account identifier returned by the Plaid flow.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub plaid_account_id: Option<String>,

    /// The method used to update the account.
    ///
    /// - `NOC` - Notification of change
    /// - `PLAID` - Plaid
    /// - `MANUAL` - Manual
    ///
    /// **OpenAPI type:** string (ref: UpdateMethod)
    #[serde(default)]
    pub update_method: Option<UpdateMethod>,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Change requests associated with this account.
    ///
    /// **OpenAPI type:** array of changeRequest
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub change_requests: Option<Vec<serde_json::Value>>,

    /// Payment updates associated with this account.
    ///
    /// **OpenAPI type:** array of paymentUpdatesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub payment_updates: Option<Vec<serde_json::Value>>,

    /// Payouts associated with this account.
    ///
    /// **OpenAPI type:** array of payoutsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub payouts: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

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

    // ==================== AccountReserved Tests ====================

    #[test]
    fn account_reserved_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountReserved::NoReserve).unwrap(), "0");
        assert_eq!(serde_json::to_string(&AccountReserved::BlockTransaction).unwrap(), "1");
        assert_eq!(serde_json::to_string(&AccountReserved::HoldTransaction).unwrap(), "3");
        assert_eq!(serde_json::to_string(&AccountReserved::ReserveFunds).unwrap(), "4");
        assert_eq!(serde_json::to_string(&AccountReserved::BlockCurrentActivity).unwrap(), "5");
        assert_eq!(serde_json::to_string(&AccountReserved::PassedDecisions).unwrap(), "6");
        assert_eq!(serde_json::to_string(&AccountReserved::NoPolicies).unwrap(), "7");
        assert_eq!(serde_json::to_string(&AccountReserved::OnboardPendingManual).unwrap(), "8");
        assert_eq!(serde_json::to_string(&AccountReserved::ScheduleAutoRelease).unwrap(), "9");
        assert_eq!(serde_json::to_string(&AccountReserved::HoldAutoRelease).unwrap(), "10");
    }

    #[test]
    fn account_reserved_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountReserved>("0").unwrap(), AccountReserved::NoReserve);
        assert_eq!(serde_json::from_str::<AccountReserved>("1").unwrap(), AccountReserved::BlockTransaction);
        assert_eq!(serde_json::from_str::<AccountReserved>("3").unwrap(), AccountReserved::HoldTransaction);
        assert_eq!(serde_json::from_str::<AccountReserved>("4").unwrap(), AccountReserved::ReserveFunds);
        assert_eq!(serde_json::from_str::<AccountReserved>("5").unwrap(), AccountReserved::BlockCurrentActivity);
        assert_eq!(serde_json::from_str::<AccountReserved>("6").unwrap(), AccountReserved::PassedDecisions);
        assert_eq!(serde_json::from_str::<AccountReserved>("7").unwrap(), AccountReserved::NoPolicies);
        assert_eq!(serde_json::from_str::<AccountReserved>("8").unwrap(), AccountReserved::OnboardPendingManual);
        assert_eq!(serde_json::from_str::<AccountReserved>("9").unwrap(), AccountReserved::ScheduleAutoRelease);
        assert_eq!(serde_json::from_str::<AccountReserved>("10").unwrap(), AccountReserved::HoldAutoRelease);
    }

    #[test]
    fn account_reserved_default() {
        assert_eq!(AccountReserved::default(), AccountReserved::NoReserve);
    }

    // ==================== AccountType Tests ====================

    #[test]
    fn account_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&AccountType::All).unwrap(), "\"all\"");
        assert_eq!(serde_json::to_string(&AccountType::Credit).unwrap(), "\"credit\"");
        assert_eq!(serde_json::to_string(&AccountType::Debit).unwrap(), "\"debit\"");
    }

    #[test]
    fn account_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<AccountType>("\"all\"").unwrap(), AccountType::All);
        assert_eq!(serde_json::from_str::<AccountType>("\"credit\"").unwrap(), AccountType::Credit);
        assert_eq!(serde_json::from_str::<AccountType>("\"debit\"").unwrap(), AccountType::Debit);
    }

    #[test]
    fn account_type_default() {
        assert_eq!(AccountType::default(), AccountType::All);
    }

    // ==================== UpdateMethod Tests ====================

    #[test]
    fn update_method_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&UpdateMethod::Noc).unwrap(), "\"NOC\"");
        assert_eq!(serde_json::to_string(&UpdateMethod::Plaid).unwrap(), "\"PLAID\"");
        assert_eq!(serde_json::to_string(&UpdateMethod::Manual).unwrap(), "\"MANUAL\"");
    }

    #[test]
    fn update_method_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<UpdateMethod>("\"NOC\"").unwrap(), UpdateMethod::Noc);
        assert_eq!(serde_json::from_str::<UpdateMethod>("\"PLAID\"").unwrap(), UpdateMethod::Plaid);
        assert_eq!(serde_json::from_str::<UpdateMethod>("\"MANUAL\"").unwrap(), UpdateMethod::Manual);
    }

    // ==================== Account Struct Tests ====================

    #[test]
    fn account_deserialize_full() {
        let json = r#"{
            "id": "t1_act_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "account": "t1_pmt_12345678901234567890123",
            "token": "tok_12345",
            "name": "Primary Checking",
            "description": "Primary business account",
            "primary": 1,
            "type": "credit",
            "status": 3,
            "reserved": 0,
            "checkStage": "createAccount",
            "expiration": "0125",
            "currency": "USD",
            "publicToken": "public-sandbox-xxx",
            "mask": "1234",
            "inactive": 0,
            "frozen": 1,
            "plaidAccountId": "plaid_acc_123",
            "updateMethod": "PLAID"
        }"#;

        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.id.as_str(), "t1_act_12345678901234567890123");
        assert_eq!(account.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(account.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(account.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(account.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(account.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(account.account.as_ref().map(|a| a.as_str()), Some("t1_pmt_12345678901234567890123"));
        assert_eq!(account.token, Some("tok_12345".to_string()));
        assert_eq!(account.name, Some("Primary Checking".to_string()));
        assert_eq!(account.description, Some("Primary business account".to_string()));
        assert!(account.primary);
        assert_eq!(account.account_type, Some(AccountType::Credit));
        assert_eq!(account.status, Some(AccountStatus::Verified));
        assert_eq!(account.reserved, Some(AccountReserved::NoReserve));
        assert_eq!(account.check_stage, Some(AccountCheckStage::CreateAccount));
        assert_eq!(account.expiration, Some("0125".to_string()));
        assert_eq!(account.currency, Some("USD".to_string()));
        assert_eq!(account.public_token, Some("public-sandbox-xxx".to_string()));
        assert_eq!(account.mask, Some("1234".to_string()));
        assert!(!account.inactive);
        assert!(account.frozen);
        assert_eq!(account.plaid_account_id, Some("plaid_acc_123".to_string()));
        assert_eq!(account.update_method, Some(UpdateMethod::Plaid));
    }

    #[test]
    fn account_deserialize_minimal() {
        let json = r#"{"id": "t1_act_12345678901234567890123"}"#;

        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.id.as_str(), "t1_act_12345678901234567890123");
        assert!(account.created.is_none());
        assert!(account.modified.is_none());
        assert!(account.creator.is_none());
        assert!(account.modifier.is_none());
        assert!(account.entity.is_none());
        assert!(account.account.is_none());
        assert!(account.token.is_none());
        assert!(account.name.is_none());
        assert!(account.description.is_none());
        assert!(!account.primary);
        assert!(account.account_type.is_none());
        assert!(account.status.is_none());
        assert!(account.reserved.is_none());
        assert!(account.check_stage.is_none());
        assert!(account.expiration.is_none());
        assert!(account.currency.is_none());
        assert!(account.public_token.is_none());
        assert!(account.mask.is_none());
        assert!(!account.inactive);
        assert!(!account.frozen);
        assert!(account.plaid_account_id.is_none());
        assert!(account.update_method.is_none());
    }

    #[test]
    fn account_bool_from_int() {
        let json = r#"{"id": "t1_act_12345678901234567890123", "primary": 1, "inactive": 1, "frozen": 1}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert!(account.primary);
        assert!(account.inactive);
        assert!(account.frozen);

        let json = r#"{"id": "t1_act_12345678901234567890123", "primary": 0, "inactive": 0, "frozen": 0}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert!(!account.primary);
        assert!(!account.inactive);
        assert!(!account.frozen);
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn account_with_nested_relations() {
        let json = r#"{
            "id": "t1_act_12345678901234567890123",
            "changeRequests": [{"id": "req1"}],
            "paymentUpdates": [{"id": "upd1"}],
            "payouts": [{"id": "t1_pyt_12345678901234567890123"}]
        }"#;

        let account: Account = serde_json::from_str(json).unwrap();
        assert!(account.change_requests.is_some());
        assert_eq!(account.change_requests.as_ref().unwrap().len(), 1);
        assert!(account.payment_updates.is_some());
        assert_eq!(account.payment_updates.as_ref().unwrap().len(), 1);
        assert!(account.payouts.is_some());
        assert_eq!(account.payouts.as_ref().unwrap().len(), 1);
    }
}
