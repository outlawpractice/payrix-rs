//! Subscription types for the Payrix API.
//!
//! Subscriptions represent recurring payment schedules for customers,
//! associated with a Plan that defines the payment terms.
//!
//! **OpenAPI schema:** `subscriptionsResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// SUBSCRIPTION ENUMS
// =============================================================================

/// Subscription transaction origin values.
///
/// Indicates how the subscription transaction was originated.
///
/// **OpenAPI schema:** `subscriptionOrigin`
///
/// Valid values:
/// - `2` - eCommerce (customer subscribing online)
/// - `3` - Mail Order/Telephone (MOTO)
/// - `4` - Apple Pay
/// - `5` - Successful 3D Secure transaction
/// - `6` - Attempted 3D Secure transaction
/// - `8` - Payframe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SubscriptionOrigin {
    /// Customer subscribing through eCommerce.
    #[default]
    ECommerce = 2,

    /// Customer subscribing by Mail Order/Telephone.
    MailOrderTelephone = 3,

    /// Originated with Apple Pay.
    ApplePay = 4,

    /// Originated as a Successful 3D Secure transaction.
    ThreeDsSuccessful = 5,

    /// Originated as an Attempted 3D Secure transaction.
    ThreeDsAttempted = 6,

    /// Originated in a Payframe.
    Payframe = 8,
}

// =============================================================================
// SUBSCRIPTION STRUCT
// =============================================================================

/// A Payrix subscription.
///
/// Subscriptions define recurring payment relationships with Plans.
///
/// **OpenAPI schema:** `subscriptionsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
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

    /// The identifier of the Plan that this Subscription is associated with.
    ///
    /// The Plan determines the frequency and amount of each payment.
    ///
    /// **OpenAPI type:** string (ref: subscriptionsModelPlan)
    #[serde(default)]
    pub plan: Option<PayrixId>,

    /// For a plan attached to a billing, this is the paying entity to match
    /// to the generated statements for which recurring payments will be made.
    ///
    /// **OpenAPI type:** string (ref: subscriptionsModelStatementEntity)
    #[serde(default)]
    pub statement_entity: Option<PayrixId>,

    /// The identification of the first transaction processed through this subscription.
    ///
    /// Used internally to process subsequent transactions.
    ///
    /// **OpenAPI type:** string (ref: subscriptionsModelFirstTxn)
    #[serde(default)]
    pub first_txn: Option<PayrixId>,

    /// The date on which the Subscription should start.
    ///
    /// Format: YYYYMMDD (e.g., `20160120` for January 20, 2016).
    /// Value must represent a date in the future.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub start: Option<i32>,

    /// The date on which the Subscription should finish.
    ///
    /// Format: YYYYMMDD (e.g., `20160120` for January 20, 2016).
    /// Value must represent a date in the future.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub finish: Option<i32>,

    /// The amount of the total sum of this Subscription that is made up of tax.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub tax: Option<i64>,

    /// The descriptor used in this Subscription.
    ///
    /// This field is stored as a text string (1-50 characters).
    /// If not set, defaults from merchant information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub descriptor: Option<String>,

    /// The description of the Txn that will be created through this Subscription.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub txn_description: Option<String>,

    /// The order of the Txn that will be created through this Subscription.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub order: Option<String>,

    /// The origin of the Txn that will be created through this Subscription.
    ///
    /// - `2` - eCommerce
    /// - `3` - Mail Order/Telephone
    /// - `4` - Apple Pay
    /// - `5` - Successful 3D Secure
    /// - `6` - Attempted 3D Secure
    /// - `8` - Payframe
    ///
    /// **OpenAPI type:** integer (ref: subscriptionOrigin)
    #[serde(default)]
    pub origin: Option<SubscriptionOrigin>,

    /// Authentication token for 3D Secure transactions.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub authentication: Option<String>,

    /// Authentication reference ID for 3D Secure transactions.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub authentication_id: Option<String>,

    /// The current count of consecutive payment failures for this subscription.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub failures: Option<i32>,

    /// The maximum consecutive payment failures to allow before inactivating.
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

    /// Invoices associated with this subscription.
    ///
    /// **OpenAPI type:** array of invoicesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub invoices: Option<Vec<serde_json::Value>>,

    /// Subscription tokens associated with this subscription.
    ///
    /// **OpenAPI type:** array of subscriptionTokensResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub subscription_tokens: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== SubscriptionOrigin Tests ====================

    #[test]
    fn subscription_origin_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::ECommerce).unwrap(), "2");
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::MailOrderTelephone).unwrap(), "3");
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::ApplePay).unwrap(), "4");
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::ThreeDsSuccessful).unwrap(), "5");
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::ThreeDsAttempted).unwrap(), "6");
        assert_eq!(serde_json::to_string(&SubscriptionOrigin::Payframe).unwrap(), "8");
    }

    #[test]
    fn subscription_origin_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("2").unwrap(), SubscriptionOrigin::ECommerce);
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("3").unwrap(), SubscriptionOrigin::MailOrderTelephone);
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("4").unwrap(), SubscriptionOrigin::ApplePay);
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("5").unwrap(), SubscriptionOrigin::ThreeDsSuccessful);
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("6").unwrap(), SubscriptionOrigin::ThreeDsAttempted);
        assert_eq!(serde_json::from_str::<SubscriptionOrigin>("8").unwrap(), SubscriptionOrigin::Payframe);
    }

    #[test]
    fn subscription_origin_default() {
        assert_eq!(SubscriptionOrigin::default(), SubscriptionOrigin::ECommerce);
    }

    // ==================== Subscription Struct Tests ====================

    #[test]
    fn subscription_deserialize_full() {
        let json = r#"{
            "id": "t1_sub_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "plan": "t1_pln_12345678901234567890123",
            "statementEntity": "t1_ent_12345678901234567890123",
            "firstTxn": "t1_txn_12345678901234567890123",
            "start": 20240101,
            "finish": 20241231,
            "tax": 500,
            "descriptor": "My Store",
            "txnDescription": "Monthly subscription",
            "order": "ORD-12345",
            "origin": 2,
            "authentication": "auth_token_123",
            "authenticationId": "auth_id_123",
            "failures": 0,
            "maxFailures": 3,
            "inactive": 0,
            "frozen": 1
        }"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id.as_str(), "t1_sub_12345678901234567890123");
        assert_eq!(sub.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(sub.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(sub.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(sub.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(sub.plan.as_ref().map(|p| p.as_str()), Some("t1_pln_12345678901234567890123"));
        assert_eq!(sub.statement_entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(sub.first_txn.as_ref().map(|t| t.as_str()), Some("t1_txn_12345678901234567890123"));
        assert_eq!(sub.start, Some(20240101));
        assert_eq!(sub.finish, Some(20241231));
        assert_eq!(sub.tax, Some(500));
        assert_eq!(sub.descriptor, Some("My Store".to_string()));
        assert_eq!(sub.txn_description, Some("Monthly subscription".to_string()));
        assert_eq!(sub.order, Some("ORD-12345".to_string()));
        assert_eq!(sub.origin, Some(SubscriptionOrigin::ECommerce));
        assert_eq!(sub.authentication, Some("auth_token_123".to_string()));
        assert_eq!(sub.authentication_id, Some("auth_id_123".to_string()));
        assert_eq!(sub.failures, Some(0));
        assert_eq!(sub.max_failures, Some(3));
        assert!(!sub.inactive);
        assert!(sub.frozen);
    }

    #[test]
    fn subscription_deserialize_minimal() {
        let json = r#"{"id": "t1_sub_12345678901234567890123"}"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id.as_str(), "t1_sub_12345678901234567890123");
        assert!(sub.created.is_none());
        assert!(sub.modified.is_none());
        assert!(sub.creator.is_none());
        assert!(sub.modifier.is_none());
        assert!(sub.plan.is_none());
        assert!(sub.statement_entity.is_none());
        assert!(sub.first_txn.is_none());
        assert!(sub.start.is_none());
        assert!(sub.finish.is_none());
        assert!(sub.tax.is_none());
        assert!(sub.descriptor.is_none());
        assert!(sub.txn_description.is_none());
        assert!(sub.order.is_none());
        assert!(sub.origin.is_none());
        assert!(sub.authentication.is_none());
        assert!(sub.authentication_id.is_none());
        assert!(sub.failures.is_none());
        assert!(sub.max_failures.is_none());
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

    #[test]
    fn subscription_origin_variants() {
        let test_cases = [
            (2, SubscriptionOrigin::ECommerce),
            (3, SubscriptionOrigin::MailOrderTelephone),
            (4, SubscriptionOrigin::ApplePay),
            (5, SubscriptionOrigin::ThreeDsSuccessful),
            (6, SubscriptionOrigin::ThreeDsAttempted),
            (8, SubscriptionOrigin::Payframe),
        ];

        for (origin_val, expected_origin) in test_cases {
            let json = format!(
                r#"{{"id": "t1_sub_12345678901234567890123", "origin": {}}}"#,
                origin_val
            );
            let sub: Subscription = serde_json::from_str(&json).unwrap();
            assert_eq!(sub.origin, Some(expected_origin));
        }
    }

    #[test]
    fn subscription_with_dates() {
        let json = r#"{
            "id": "t1_sub_12345678901234567890123",
            "start": 20240115,
            "finish": 20250115
        }"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.start, Some(20240115));
        assert_eq!(sub.finish, Some(20250115));
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn subscription_with_nested_relations() {
        let json = r#"{
            "id": "t1_sub_12345678901234567890123",
            "invoices": [{"id": "t1_inv_12345678901234567890123"}],
            "subscriptionTokens": [{"id": "t1_stk_12345678901234567890123"}]
        }"#;

        let sub: Subscription = serde_json::from_str(json).unwrap();
        assert!(sub.invoices.is_some());
        assert_eq!(sub.invoices.as_ref().unwrap().len(), 1);
        assert!(sub.subscription_tokens.is_some());
        assert_eq!(sub.subscription_tokens.as_ref().unwrap().len(), 1);
    }
}
