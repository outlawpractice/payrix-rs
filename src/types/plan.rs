//! Plan types for the Payrix API.
//!
//! Plans define billing terms for recurring payments or installment payments.
//!
//! **OpenAPI schema:** `plansResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// PLAN ENUMS
// =============================================================================

/// Plan type values.
///
/// Determines the type of payment plan.
///
/// **OpenAPI schema:** `planType`
///
/// Valid values:
/// - `recurring` - A recurring payment plan (subscription)
/// - `installment` - A deferred payment installment plan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    /// A recurring payment plan (subscription).
    #[default]
    Recurring,

    /// A deferred payment installment plan.
    Installment,
}

/// Plan schedule values.
///
/// Determines when the subscription related to this Plan is triggered.
///
/// **OpenAPI schema:** `planSchedule`
///
/// Valid values:
/// - `1` - Daily
/// - `2` - Weekly
/// - `3` - Monthly
/// - `4` - Annually
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PlanSchedule {
    /// Daily billing.
    Daily = 1,

    /// Weekly billing.
    Weekly = 2,

    /// Monthly billing.
    #[default]
    Monthly = 3,

    /// Annual billing.
    Annually = 4,
}

/// Plan unit of measure.
///
/// Determines how the amount is interpreted.
///
/// **OpenAPI schema:** `planUm`
///
/// Valid values:
/// - `actual` - An actual amount to charge, in cents
/// - `percent` - A percentage of another amount, in basis points
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanUm {
    /// An actual amount to charge, in cents.
    #[default]
    Actual,

    /// A percentage of another amount, in basis points.
    Percent,
}

// =============================================================================
// PLAN STRUCT
// =============================================================================

/// A Payrix plan.
///
/// Plans define billing terms for subscriptions and installment payments.
///
/// **OpenAPI schema:** `plansResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Plan {
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

    /// The identifier of the Merchant associated with this Plan.
    ///
    /// **OpenAPI type:** string (ref: plansModelMerchant1)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The attached billing for which recurring payments should be made
    /// to pay off statements.
    ///
    /// **OpenAPI type:** string (ref: plansModelBilling)
    #[serde(default)]
    pub billing: Option<PayrixId>,

    /// The type of plan.
    ///
    /// - `recurring` - A recurring payment plan (subscription)
    /// - `installment` - A deferred payment installment plan
    ///
    /// **OpenAPI type:** string (ref: planType)
    #[serde(default, rename = "type")]
    pub plan_type: Option<PlanType>,

    /// The name of this Plan.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Plan.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The description of the Txn that will be created through this Plan.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub txn_description: Option<String>,

    /// The order of the Txn that will be created through this Plan.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub order: Option<String>,

    /// The schedule that determines when the subscription is triggered.
    ///
    /// - `1` - Daily
    /// - `2` - Weekly
    /// - `3` - Monthly
    /// - `4` - Annually
    ///
    /// **OpenAPI type:** integer (ref: planSchedule)
    #[serde(default)]
    pub schedule: Option<PlanSchedule>,

    /// A multiplier to adjust the schedule.
    ///
    /// For example, with schedule=3 (Monthly) and scheduleFactor=2,
    /// the plan triggers every 2 months.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub schedule_factor: Option<i32>,

    /// The unit of measure for the amount on the plan.
    ///
    /// - `actual` - An actual amount to charge, in cents
    /// - `percent` - A percentage of another amount, in basis points
    ///
    /// **OpenAPI type:** string (ref: planUm)
    #[serde(default)]
    pub um: Option<PlanUm>,

    /// The amount to charge with each payment under this Plan.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub amount: Option<i64>,

    /// The maximum consecutive payment failures to allow for a subscription
    /// before inactivating it.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub max_failures: Option<i32>,

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

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Subscriptions associated with this plan.
    ///
    /// **OpenAPI type:** array of subscriptionsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub subscriptions: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PlanType Tests ====================

    #[test]
    fn plan_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PlanType::Recurring).unwrap(), "\"recurring\"");
        assert_eq!(serde_json::to_string(&PlanType::Installment).unwrap(), "\"installment\"");
    }

    #[test]
    fn plan_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PlanType>("\"recurring\"").unwrap(), PlanType::Recurring);
        assert_eq!(serde_json::from_str::<PlanType>("\"installment\"").unwrap(), PlanType::Installment);
    }

    #[test]
    fn plan_type_default() {
        assert_eq!(PlanType::default(), PlanType::Recurring);
    }

    // ==================== PlanSchedule Tests ====================

    #[test]
    fn plan_schedule_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PlanSchedule::Daily).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PlanSchedule::Weekly).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PlanSchedule::Monthly).unwrap(), "3");
        assert_eq!(serde_json::to_string(&PlanSchedule::Annually).unwrap(), "4");
    }

    #[test]
    fn plan_schedule_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PlanSchedule>("1").unwrap(), PlanSchedule::Daily);
        assert_eq!(serde_json::from_str::<PlanSchedule>("2").unwrap(), PlanSchedule::Weekly);
        assert_eq!(serde_json::from_str::<PlanSchedule>("3").unwrap(), PlanSchedule::Monthly);
        assert_eq!(serde_json::from_str::<PlanSchedule>("4").unwrap(), PlanSchedule::Annually);
    }

    #[test]
    fn plan_schedule_default() {
        assert_eq!(PlanSchedule::default(), PlanSchedule::Monthly);
    }

    // ==================== PlanUm Tests ====================

    #[test]
    fn plan_um_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PlanUm::Actual).unwrap(), "\"actual\"");
        assert_eq!(serde_json::to_string(&PlanUm::Percent).unwrap(), "\"percent\"");
    }

    #[test]
    fn plan_um_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PlanUm>("\"actual\"").unwrap(), PlanUm::Actual);
        assert_eq!(serde_json::from_str::<PlanUm>("\"percent\"").unwrap(), PlanUm::Percent);
    }

    #[test]
    fn plan_um_default() {
        assert_eq!(PlanUm::default(), PlanUm::Actual);
    }

    // ==================== Plan Struct Tests ====================

    #[test]
    fn plan_deserialize_full() {
        let json = r#"{
            "id": "t1_pln_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "merchant": "t1_mer_12345678901234567890123",
            "billing": "t1_bil_12345678901234567890123",
            "type": "recurring",
            "name": "Premium Plan",
            "description": "Our best plan",
            "txnDescription": "Monthly premium subscription",
            "order": "ORD-12345",
            "schedule": 3,
            "scheduleFactor": 1,
            "um": "actual",
            "amount": 4999,
            "maxFailures": 3,
            "inactive": 0,
            "frozen": 1
        }"#;

        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id.as_str(), "t1_pln_12345678901234567890123");
        assert_eq!(plan.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(plan.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(plan.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(plan.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(plan.merchant.as_ref().map(|m| m.as_str()), Some("t1_mer_12345678901234567890123"));
        assert_eq!(plan.billing.as_ref().map(|b| b.as_str()), Some("t1_bil_12345678901234567890123"));
        assert_eq!(plan.plan_type, Some(PlanType::Recurring));
        assert_eq!(plan.name, Some("Premium Plan".to_string()));
        assert_eq!(plan.description, Some("Our best plan".to_string()));
        assert_eq!(plan.txn_description, Some("Monthly premium subscription".to_string()));
        assert_eq!(plan.order, Some("ORD-12345".to_string()));
        assert_eq!(plan.schedule, Some(PlanSchedule::Monthly));
        assert_eq!(plan.schedule_factor, Some(1));
        assert_eq!(plan.um, Some(PlanUm::Actual));
        assert_eq!(plan.amount, Some(4999));
        assert_eq!(plan.max_failures, Some(3));
        assert!(!plan.inactive);
        assert!(plan.frozen);
    }

    #[test]
    fn plan_deserialize_minimal() {
        let json = r#"{"id": "t1_pln_12345678901234567890123"}"#;

        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id.as_str(), "t1_pln_12345678901234567890123");
        assert!(plan.created.is_none());
        assert!(plan.modified.is_none());
        assert!(plan.creator.is_none());
        assert!(plan.modifier.is_none());
        assert!(plan.merchant.is_none());
        assert!(plan.billing.is_none());
        assert!(plan.plan_type.is_none());
        assert!(plan.name.is_none());
        assert!(plan.description.is_none());
        assert!(plan.txn_description.is_none());
        assert!(plan.order.is_none());
        assert!(plan.schedule.is_none());
        assert!(plan.schedule_factor.is_none());
        assert!(plan.um.is_none());
        assert!(plan.amount.is_none());
        assert!(plan.max_failures.is_none());
        assert!(!plan.inactive);
        assert!(!plan.frozen);
    }

    #[test]
    fn plan_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_pln_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert!(!plan.inactive);
        assert!(!plan.frozen);
    }

    #[test]
    fn plan_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_pln_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert!(plan.inactive);
        assert!(plan.frozen);
    }

    #[test]
    fn plan_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_pln_12345678901234567890123"}"#;
        let plan: Plan = serde_json::from_str(json).unwrap();
        assert!(!plan.inactive);
        assert!(!plan.frozen);
    }

    #[test]
    fn plan_schedule_variants() {
        let test_cases = [
            (1, PlanSchedule::Daily),
            (2, PlanSchedule::Weekly),
            (3, PlanSchedule::Monthly),
            (4, PlanSchedule::Annually),
        ];

        for (schedule_val, expected_schedule) in test_cases {
            let json = format!(
                r#"{{"id": "t1_pln_12345678901234567890123", "schedule": {}}}"#,
                schedule_val
            );
            let plan: Plan = serde_json::from_str(&json).unwrap();
            assert_eq!(plan.schedule, Some(expected_schedule));
        }
    }

    #[test]
    fn plan_installment_type() {
        let json = r#"{
            "id": "t1_pln_12345678901234567890123",
            "type": "installment"
        }"#;

        let plan: Plan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.plan_type, Some(PlanType::Installment));
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn plan_with_nested_subscriptions() {
        let json = r#"{
            "id": "t1_pln_12345678901234567890123",
            "subscriptions": [{"id": "t1_sub_12345678901234567890123"}]
        }"#;

        let plan: Plan = serde_json::from_str(json).unwrap();
        assert!(plan.subscriptions.is_some());
        assert_eq!(plan.subscriptions.as_ref().unwrap().len(), 1);
    }
}
