//! Payout types for the Payrix API.
//!
//! Payouts represent scheduled or on-demand disbursements of funds
//! from a merchant's operating balance to their bank account.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{
    bool_from_int_default_false, option_bool_from_int, DateYmd, PayrixId, PayoutSchedule,
    PayoutUnit,
};

/// Payout status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PayoutStatus {
    /// Payout is pending
    #[default]
    Pending = 0,
    /// Payout is scheduled
    Scheduled = 1,
    /// Payout is processing
    Processing = 2,
    /// Payout completed successfully
    Completed = 3,
    /// Payout failed
    Failed = 4,
    /// Payout was canceled
    Canceled = 5,
    /// Payout was returned
    Returned = 6,
}

/// A Payrix payout.
///
/// Payouts define how and when funds are disbursed from a merchant's
/// operating balance to their bank account. All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Payout {
    /// Unique identifier (30 characters, e.g., "t1_pay_...")
    pub id: PayrixId,

    /// Entity ID that owns this payout configuration
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID to pay out from
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Account ID to pay out to
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// Login ID that created this payout
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Payout status
    #[serde(default)]
    pub status: Option<PayoutStatus>,

    /// Payout schedule (daily, weekly, monthly, etc.)
    #[serde(default)]
    pub schedule: Option<PayoutSchedule>,

    /// Payout unit (percent or actual amount)
    #[serde(default)]
    pub unit: Option<PayoutUnit>,

    /// Payout amount or percentage (depending on unit)
    /// If unit is Actual, this is in cents
    /// If unit is Percent, this is the percentage (e.g., 100 = 100%)
    #[serde(default)]
    pub amount: Option<i64>,

    /// Minimum balance to maintain after payout (in cents)
    #[serde(default)]
    pub min_balance: Option<i64>,

    /// Minimum amount required to trigger payout (in cents)
    #[serde(default)]
    pub min_payout: Option<i64>,

    /// Maximum payout amount (in cents)
    #[serde(default)]
    pub max_payout: Option<i64>,

    /// Day of week (1=Monday, 7=Sunday) for weekly payouts
    #[serde(default)]
    pub day_of_week: Option<i32>,

    /// Day of month (1-31) for monthly payouts
    #[serde(default)]
    pub day_of_month: Option<i32>,

    /// Next scheduled payout date (YYYYMMDD format)
    #[serde(default)]
    pub next_date: Option<DateYmd>,

    /// Last payout date (YYYYMMDD format)
    #[serde(default)]
    pub last_date: Option<DateYmd>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Payout name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Whether payout is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub enabled: bool,

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

/// Request to create a new payout configuration.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPayout {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Fund ID to pay out from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund: Option<String>,

    /// Account ID to pay out to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Payout schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<PayoutSchedule>,

    /// Payout unit (percent or actual amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<PayoutUnit>,

    /// Payout amount or percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Minimum balance to maintain (in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_balance: Option<i64>,

    /// Minimum amount to trigger payout (in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_payout: Option<i64>,

    /// Maximum payout amount (in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payout: Option<i64>,

    /// Day of week for weekly payouts (1=Monday, 7=Sunday)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,

    /// Day of month for monthly payouts (1-31)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,

    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Payout name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description/memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether payout is enabled
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub enabled: Option<bool>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether resource is inactive
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PayoutStatus Tests ====================

    #[test]
    fn payout_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PayoutStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&PayoutStatus::Scheduled).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PayoutStatus::Processing).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PayoutStatus::Completed).unwrap(), "3");
        assert_eq!(serde_json::to_string(&PayoutStatus::Failed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&PayoutStatus::Canceled).unwrap(), "5");
        assert_eq!(serde_json::to_string(&PayoutStatus::Returned).unwrap(), "6");
    }

    #[test]
    fn payout_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PayoutStatus>("0").unwrap(), PayoutStatus::Pending);
        assert_eq!(serde_json::from_str::<PayoutStatus>("1").unwrap(), PayoutStatus::Scheduled);
        assert_eq!(serde_json::from_str::<PayoutStatus>("2").unwrap(), PayoutStatus::Processing);
        assert_eq!(serde_json::from_str::<PayoutStatus>("3").unwrap(), PayoutStatus::Completed);
        assert_eq!(serde_json::from_str::<PayoutStatus>("4").unwrap(), PayoutStatus::Failed);
        assert_eq!(serde_json::from_str::<PayoutStatus>("5").unwrap(), PayoutStatus::Canceled);
        assert_eq!(serde_json::from_str::<PayoutStatus>("6").unwrap(), PayoutStatus::Returned);
    }

    #[test]
    fn payout_status_default() {
        assert_eq!(PayoutStatus::default(), PayoutStatus::Pending);
    }

    #[test]
    fn payout_status_invalid_value() {
        assert!(serde_json::from_str::<PayoutStatus>("99").is_err());
    }

    // ==================== Payout Struct Tests ====================

    #[test]
    fn payout_deserialize_full() {
        let json = r#"{
            "id": "t1_pay_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "account": "t1_acc_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 3,
            "schedule": 1,
            "unit": 2,
            "amount": 50000,
            "minBalance": 10000,
            "minPayout": 5000,
            "maxPayout": 100000,
            "dayOfWeek": 1,
            "dayOfMonth": 15,
            "nextDate": "20240501",
            "lastDate": "20240401",
            "currency": "USD",
            "name": "Daily Payout",
            "description": "Daily operating payout",
            "enabled": 1,
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let payout: Payout = serde_json::from_str(json).unwrap();
        assert_eq!(payout.id.as_str(), "t1_pay_12345678901234567890123");
        assert_eq!(payout.status, Some(PayoutStatus::Completed));
        assert_eq!(payout.schedule, Some(PayoutSchedule::Daily));
        assert_eq!(payout.unit, Some(PayoutUnit::Actual));
        assert_eq!(payout.amount, Some(50000));
        assert_eq!(payout.min_balance, Some(10000));
        assert!(payout.enabled);
        assert!(!payout.inactive);
    }

    #[test]
    fn payout_deserialize_minimal() {
        let json = r#"{"id": "t1_pay_12345678901234567890123"}"#;
        let payout: Payout = serde_json::from_str(json).unwrap();
        assert_eq!(payout.id.as_str(), "t1_pay_12345678901234567890123");
        assert!(payout.status.is_none());
        assert!(!payout.enabled);
    }

    #[test]
    fn payout_bool_from_int() {
        let json = r#"{"id": "t1_pay_12345678901234567890123", "enabled": 1, "inactive": 0, "frozen": 1}"#;
        let payout: Payout = serde_json::from_str(json).unwrap();
        assert!(payout.enabled);
        assert!(!payout.inactive);
        assert!(payout.frozen);
    }

    // ==================== NewPayout Tests ====================

    #[test]
    fn new_payout_serialize_full() {
        let new_payout = NewPayout {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            fund: Some("t1_fnd_12345678901234567890123".to_string()),
            account: Some("t1_acc_12345678901234567890123".to_string()),
            schedule: Some(PayoutSchedule::Daily),
            unit: Some(PayoutUnit::Actual),
            amount: Some(50000),
            min_balance: Some(10000),
            min_payout: Some(5000),
            max_payout: Some(100000),
            day_of_week: Some(1),
            day_of_month: Some(15),
            currency: Some("USD".to_string()),
            name: Some("Daily Payout".to_string()),
            description: Some("Daily operating payout".to_string()),
            enabled: Some(true),
            custom: Some("custom".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_payout).unwrap();
        assert!(json.contains("\"schedule\":1"));
        assert!(json.contains("\"unit\":2"));
        assert!(json.contains("\"amount\":50000"));
        assert!(json.contains("\"enabled\":1"));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_payout_serialize_minimal() {
        let new_payout = NewPayout::default();
        let json = serde_json::to_string(&new_payout).unwrap();
        assert_eq!(json, "{}");
    }
}
