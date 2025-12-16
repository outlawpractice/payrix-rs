//! Plan types for the Payrix API.
//!
//! Plans are subscription templates that define billing amounts, frequencies,
//! and other terms for recurring payments.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId, SubscriptionSchedule};

/// Plan status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PlanStatus {
    /// Plan is inactive/draft
    Inactive = 0,
    /// Plan is active and available for subscriptions
    #[default]
    Active = 1,
    /// Plan is archived (no new subscriptions)
    Archived = 2,
}

/// A Payrix subscription plan.
///
/// Plans define the template for subscriptions including the billing
/// amount, frequency, and trial period. The payment amount is in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// Unique identifier (30 characters, e.g., "t1_pln_...")
    pub id: PayrixId,

    /// Entity ID that owns this plan
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID (if merchant-specific plan)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this plan
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Plan name
    #[serde(default)]
    pub name: Option<String>,

    /// Plan description
    #[serde(default)]
    pub description: Option<String>,

    /// Plan status
    #[serde(default)]
    pub status: Option<PlanStatus>,

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

    /// Interval between billings (e.g., 1 = every period, 2 = every other period)
    #[serde(default)]
    pub interval: Option<i32>,

    /// Day of month for billing (1-31)
    #[serde(default)]
    pub day: Option<i32>,

    /// Trial period in days
    #[serde(default)]
    pub trial_days: Option<i32>,

    /// Setup fee in cents (one-time charge on subscription creation)
    #[serde(default)]
    pub setup_fee: Option<i64>,

    /// Number of active subscriptions using this plan
    #[serde(default)]
    pub subscription_count: Option<i32>,

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

/// Request to create a new plan.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPlan {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID (if merchant-specific plan)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Plan name (required)
    pub name: String,

    /// Plan description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Billing schedule/frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<SubscriptionSchedule>,

    /// Payment amount in cents (required)
    pub amount: i64,

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

    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,

    /// Setup fee in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee: Option<i64>,

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

    // ==================== PlanStatus Tests ====================

    #[test]
    fn plan_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PlanStatus::Inactive).unwrap(), "0");
        assert_eq!(serde_json::to_string(&PlanStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PlanStatus::Archived).unwrap(), "2");
    }

    #[test]
    fn plan_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PlanStatus>("0").unwrap(), PlanStatus::Inactive);
        assert_eq!(serde_json::from_str::<PlanStatus>("1").unwrap(), PlanStatus::Active);
        assert_eq!(serde_json::from_str::<PlanStatus>("2").unwrap(), PlanStatus::Archived);
    }

    #[test]
    fn plan_status_default() {
        assert_eq!(PlanStatus::default(), PlanStatus::Active);
    }

    #[test]
    fn plan_status_invalid_value() {
        assert!(serde_json::from_str::<PlanStatus>("99").is_err());
    }

    // ==================== Plan Struct Tests ====================

    #[test]
    fn plan_deserialize_full() {
        let json = r#"{
            "id": "t1_pln_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "name": "Premium Plan",
            "description": "Our best plan",
            "status": 1,
            "schedule": 4,
            "amount": 4999,
            "currency": "USD",
            "cycles": 0,
            "interval": 1,
            "day": 1,
            "trialDays": 14,
            "setupFee": 999,
            "subscriptionCount": 150,
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id.as_str(), "t1_pln_12345678901234567890123");
        assert_eq!(plan.name, Some("Premium Plan".to_string()));
        assert_eq!(plan.status, Some(PlanStatus::Active));
        assert_eq!(plan.schedule, Some(SubscriptionSchedule::Monthly));
        assert_eq!(plan.amount, Some(4999));
        assert_eq!(plan.trial_days, Some(14));
        assert_eq!(plan.setup_fee, Some(999));
        assert!(!plan.inactive);
        assert!(!plan.frozen);
    }

    #[test]
    fn plan_deserialize_minimal() {
        let json = r#"{"id": "t1_pln_12345678901234567890123"}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id.as_str(), "t1_pln_12345678901234567890123");
        assert!(plan.name.is_none());
        assert!(plan.status.is_none());
    }

    #[test]
    fn plan_bool_from_int() {
        let json = r#"{"id": "t1_pln_12345678901234567890123", "inactive": 1, "frozen": 0}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert!(plan.inactive);
        assert!(!plan.frozen);
    }

    // ==================== NewPlan Tests ====================

    #[test]
    fn new_plan_serialize_full() {
        let new_plan = NewPlan {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            name: "Premium Plan".to_string(),
            description: Some("Our best plan".to_string()),
            schedule: Some(SubscriptionSchedule::Monthly),
            amount: 4999,
            currency: Some("USD".to_string()),
            cycles: Some(0),
            interval: Some(1),
            day: Some(1),
            trial_days: Some(14),
            setup_fee: Some(999),
            custom: Some("custom".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_plan).unwrap();
        assert!(json.contains("\"name\":\"Premium Plan\""));
        assert!(json.contains("\"amount\":4999"));
        assert!(json.contains("\"schedule\":4"));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_plan_serialize_minimal() {
        let new_plan = NewPlan {
            name: "Basic Plan".to_string(),
            amount: 999,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_plan).unwrap();
        assert!(json.contains("\"name\":\"Basic Plan\""));
        assert!(json.contains("\"amount\":999"));
        assert!(!json.contains("\"schedule\""));
        assert!(!json.contains("\"inactive\""));
    }
}
