//! Subscription types for the Payrix API.
//!
//! Subscriptions represent recurring payment schedules for customers.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, DateYmd, PayrixId};

/// Subscription status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SubscriptionStatus {
    /// Subscription is pending activation
    #[default]
    Pending = 0,
    /// Subscription is active and billing
    Active = 1,
    /// Subscription is paused (temporarily stopped)
    Paused = 2,
    /// Subscription is canceled
    Canceled = 3,
    /// Subscription has past due payments
    PastDue = 4,
    /// Subscription trial period
    Trial = 5,
    /// Subscription completed (finite subscriptions)
    Completed = 6,
}

/// Subscription schedule/frequency values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SubscriptionSchedule {
    /// Daily billing
    Daily = 1,
    /// Weekly billing
    Weekly = 2,
    /// Bi-weekly billing (every 2 weeks)
    BiWeekly = 3,
    /// Monthly billing
    #[default]
    Monthly = 4,
    /// Quarterly billing (every 3 months)
    Quarterly = 5,
    /// Semi-annual billing (every 6 months)
    SemiAnnual = 6,
    /// Annual billing
    Annual = 7,
}

/// A Payrix subscription.
///
/// Subscriptions define recurring payment schedules for customers.
/// The payment amount is in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    /// Unique identifier (30 characters, e.g., "t1_sub_...")
    pub id: PayrixId,

    /// Merchant ID that owns this subscription
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Customer ID for this subscription
    #[serde(default)]
    pub customer: Option<PayrixId>,

    /// Token ID for the payment method
    #[serde(default)]
    pub token: Option<PayrixId>,

    /// Plan ID (if using a predefined plan)
    #[serde(default)]
    pub plan: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Login ID that created this subscription
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Subscription status
    #[serde(default)]
    pub status: Option<SubscriptionStatus>,

    /// Billing schedule/frequency
    #[serde(default)]
    pub schedule: Option<SubscriptionSchedule>,

    /// Payment amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Number of billing cycles (0 = infinite)
    #[serde(default)]
    pub cycles: Option<i32>,

    /// Number of cycles completed
    #[serde(default)]
    pub cycles_completed: Option<i32>,

    /// Interval between billings (e.g., 1 = every period, 2 = every other period)
    #[serde(default)]
    pub interval: Option<i32>,

    /// Day of month for billing (1-31)
    #[serde(default)]
    pub day: Option<i32>,

    /// Start date (YYYYMMDD format)
    #[serde(default)]
    pub start: Option<DateYmd>,

    /// Next billing date (YYYYMMDD format)
    #[serde(default)]
    pub next: Option<DateYmd>,

    /// End date (YYYYMMDD format)
    #[serde(default)]
    pub end: Option<DateYmd>,

    /// Trial end date (YYYYMMDD format)
    #[serde(default)]
    pub trial_end: Option<DateYmd>,

    /// Subscription name/description
    #[serde(default)]
    pub name: Option<String>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Number of failed payment attempts
    #[serde(default)]
    pub failed_attempts: Option<i32>,

    /// Maximum failed attempts before suspension
    #[serde(default)]
    pub max_failed_attempts: Option<i32>,

    /// Last transaction ID
    #[serde(default)]
    pub last_txn: Option<PayrixId>,

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

/// Request to create a new subscription.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSubscription {
    /// Merchant ID (required)
    pub merchant: String,

    /// Customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Token ID for the payment method (required for automatic billing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Plan ID (if using a predefined plan)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// Billing schedule/frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<SubscriptionSchedule>,

    /// Payment amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Number of billing cycles (0 = infinite)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles: Option<i32>,

    /// Interval between billings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,

    /// Day of month for billing (1-31)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,

    /// Start date (YYYYMMDD format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// Trial end date (YYYYMMDD format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<String>,

    /// Subscription name/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description/memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Maximum failed attempts before suspension
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_attempts: Option<i32>,

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

    // ==================== SubscriptionStatus Tests ====================

    #[test]
    fn subscription_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Paused).unwrap(), "2");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Canceled).unwrap(), "3");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::PastDue).unwrap(), "4");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Trial).unwrap(), "5");
        assert_eq!(serde_json::to_string(&SubscriptionStatus::Completed).unwrap(), "6");
    }

    #[test]
    fn subscription_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("0").unwrap(), SubscriptionStatus::Pending);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("1").unwrap(), SubscriptionStatus::Active);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("2").unwrap(), SubscriptionStatus::Paused);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("3").unwrap(), SubscriptionStatus::Canceled);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("4").unwrap(), SubscriptionStatus::PastDue);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("5").unwrap(), SubscriptionStatus::Trial);
        assert_eq!(serde_json::from_str::<SubscriptionStatus>("6").unwrap(), SubscriptionStatus::Completed);
    }

    #[test]
    fn subscription_status_default() {
        assert_eq!(SubscriptionStatus::default(), SubscriptionStatus::Pending);
    }

    #[test]
    fn subscription_status_invalid_value() {
        assert!(serde_json::from_str::<SubscriptionStatus>("99").is_err());
    }

    // ==================== SubscriptionSchedule Tests ====================

    #[test]
    fn subscription_schedule_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::Daily).unwrap(), "1");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::Weekly).unwrap(), "2");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::BiWeekly).unwrap(), "3");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::Monthly).unwrap(), "4");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::Quarterly).unwrap(), "5");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::SemiAnnual).unwrap(), "6");
        assert_eq!(serde_json::to_string(&SubscriptionSchedule::Annual).unwrap(), "7");
    }

    #[test]
    fn subscription_schedule_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("1").unwrap(), SubscriptionSchedule::Daily);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("2").unwrap(), SubscriptionSchedule::Weekly);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("3").unwrap(), SubscriptionSchedule::BiWeekly);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("4").unwrap(), SubscriptionSchedule::Monthly);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("5").unwrap(), SubscriptionSchedule::Quarterly);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("6").unwrap(), SubscriptionSchedule::SemiAnnual);
        assert_eq!(serde_json::from_str::<SubscriptionSchedule>("7").unwrap(), SubscriptionSchedule::Annual);
    }

    #[test]
    fn subscription_schedule_default() {
        assert_eq!(SubscriptionSchedule::default(), SubscriptionSchedule::Monthly);
    }

    #[test]
    fn subscription_schedule_invalid_value() {
        assert!(serde_json::from_str::<SubscriptionSchedule>("0").is_err());
        assert!(serde_json::from_str::<SubscriptionSchedule>("99").is_err());
    }

    // ==================== Subscription Struct Tests ====================

    #[test]
    fn subscription_deserialize_full() {
        let json = r#"{
            "id": "t1_sub_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "customer": "t1_cus_12345678901234567890123",
            "token": "t1_tok_12345678901234567890123",
            "plan": "t1_pln_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 1,
            "schedule": 4,
            "amount": 2999,
            "currency": "USD",
            "cycles": 12,
            "cyclesCompleted": 3,
            "interval": 1,
            "day": 15,
            "start": "20240101",
            "next": "20240415",
            "end": "20241231",
            "trialEnd": "20240115",
            "name": "Monthly Premium",
            "description": "Premium subscription",
            "failedAttempts": 0,
            "maxFailedAttempts": 3,
            "lastTxn": "t1_txn_12345678901234567890123",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id.as_str(), "t1_sub_12345678901234567890123");
        assert_eq!(sub.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(sub.customer.unwrap().as_str(), "t1_cus_12345678901234567890123");
        assert_eq!(sub.status, Some(SubscriptionStatus::Active));
        assert_eq!(sub.schedule, Some(SubscriptionSchedule::Monthly));
        assert_eq!(sub.amount, Some(2999));
        assert_eq!(sub.currency, Some("USD".to_string()));
        assert_eq!(sub.cycles, Some(12));
        assert_eq!(sub.cycles_completed, Some(3));
        assert_eq!(sub.start.as_ref().unwrap().as_str(), "20240101");
        assert_eq!(sub.next.as_ref().unwrap().as_str(), "20240415");
        assert!(!sub.inactive);
        assert!(sub.frozen);
    }

    #[test]
    fn subscription_deserialize_minimal() {
        let json = r#"{
            "id": "t1_sub_12345678901234567890123"
        }"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id.as_str(), "t1_sub_12345678901234567890123");
        assert!(sub.merchant.is_none());
        assert!(sub.status.is_none());
        assert!(!sub.inactive);
        assert!(!sub.frozen);
    }

    #[test]
    fn subscription_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_sub_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert!(!sub.inactive);
        assert!(!sub.frozen);
    }

    #[test]
    fn subscription_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_sub_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert!(sub.inactive);
        assert!(sub.frozen);
    }

    #[test]
    fn subscription_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_sub_12345678901234567890123"}"#;
        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert!(!sub.inactive);
        assert!(!sub.frozen);
    }

    // ==================== NewSubscription Tests ====================

    #[test]
    fn new_subscription_serialize_full() {
        let new_sub = NewSubscription {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            customer: Some("t1_cus_12345678901234567890123".to_string()),
            token: Some("t1_tok_12345678901234567890123".to_string()),
            plan: Some("t1_pln_12345678901234567890123".to_string()),
            schedule: Some(SubscriptionSchedule::Monthly),
            amount: Some(2999),
            currency: Some("USD".to_string()),
            cycles: Some(12),
            interval: Some(1),
            day: Some(15),
            start: Some("20240101".to_string()),
            trial_end: Some("20240115".to_string()),
            name: Some("Monthly Premium".to_string()),
            description: Some("Premium subscription".to_string()),
            max_failed_attempts: Some(3),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_sub).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"schedule\":4"));
        assert!(json.contains("\"amount\":2999"));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_subscription_serialize_minimal() {
        let new_sub = NewSubscription {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_sub).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"customer\""));
        assert!(!json.contains("\"schedule\""));
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_subscription_option_bool_to_int_true() {
        let new_sub = NewSubscription {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            inactive: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_sub).unwrap();
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn new_subscription_option_bool_to_int_false() {
        let new_sub = NewSubscription {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            inactive: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_sub).unwrap();
        assert!(json.contains("\"inactive\":0"));
    }
}
