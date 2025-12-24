//! Payout types for the Payrix API.
//!
//! Payouts define how and when funds are disbursed from a merchant's
//! operating balance to their bank account.
//!
//! **OpenAPI schema:** `payoutsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId, PayoutSchedule, PayoutUnit};

// =============================================================================
// PAYOUT STRUCT
// =============================================================================

/// A Payrix payout configuration.
///
/// Payouts define the schedule and parameters for disbursing funds from
/// a merchant's operating balance to their bank account.
///
/// **OpenAPI schema:** `payoutsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Payout {
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

    /// The Login that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: payoutsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The token of the Account that this Payout is associated with.
    ///
    /// This account will either receive the funds or be debited for the funds
    /// every time a Disbursement occurs, depending on the direction.
    ///
    /// **OpenAPI type:** string (ref: payoutsModelAccount)
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// The identifier of the Entity that this Payout is associated with.
    ///
    /// **OpenAPI type:** string (ref: payoutsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of a Billing that this Payout is associated with.
    ///
    /// Payout associated with a Billing record will be used to pay for Statements.
    ///
    /// **OpenAPI type:** string (ref: payoutsModelBilling)
    #[serde(default)]
    pub billing: Option<PayrixId>,

    /// The identifier of the PayoutFlow associated with this Payout.
    ///
    /// **OpenAPI type:** string (ref: payoutsModelPayoutFlow)
    #[serde(default)]
    pub payout_flow: Option<PayrixId>,

    /// The name of this Payout.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Payout.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The schedule that determines when the Payout should be triggered.
    ///
    /// - `1` - Daily
    /// - `2` - Weekly
    /// - `3` - Monthly
    /// - `4` - Annually
    /// - `5` - Single (one-off payment)
    ///
    /// **OpenAPI type:** integer (ref: payoutSchedule)
    #[serde(default)]
    pub schedule: Option<PayoutSchedule>,

    /// A multiplier to adjust the schedule set in the 'schedule' field.
    ///
    /// This field is specified as an integer and its value determines how
    /// the interval is multiplied.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub schedule_factor: Option<i32>,

    /// The date on which payment of the Payout should start.
    ///
    /// The date is specified as an eight digit integer in YYYYMMDD format,
    /// for example, 20160120 for January 20, 2016.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub start: Option<i32>,

    /// The currency of the amount in this Payout.
    ///
    /// This field is only required when um is set to ACTUAL.
    /// If not set, disbursements for all currencies will be processed.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The unit of measure for calculating the Payout amount.
    ///
    /// - `1` - Percent (percentage in basis points)
    /// - `2` - Actual (amount in cents)
    /// - `3` - PercentNegative (negative percentage in basis points)
    ///
    /// **OpenAPI type:** integer (ref: payoutUm)
    #[serde(default)]
    pub um: Option<PayoutUnit>,

    /// The total amount of this Payout.
    ///
    /// The units depend on the 'um' field:
    /// - If um is 1 or 3: percentage in basis points
    /// - If um is 2: amount in cents
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub amount: Option<i64>,

    /// The threshold that ensures no disbursement is generated if it
    /// doesn't reach the minimum value.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub minimum: Option<i64>,

    /// The maximum threshold for a disbursement.
    ///
    /// Any amount exceeding this value will not be released and will
    /// roll over to the next disbursement.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub maximum: Option<i64>,

    /// The minimum balance to maintain, despite any Payouts occurring.
    ///
    /// If the Payout would reduce the balance to below this value, it is not processed.
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, rename = "float")]
    pub float_balance: Option<i32>,

    /// The secondary billing descriptor to appear on bank statements for the payout.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub secondary_descriptor: Option<String>,

    /// Whether to skip the creation of disbursements on holidays and weekends.
    ///
    /// - `0` - Do not skip (disbursement will be generated in Requested status
    ///   and process the next business day)
    /// - `1` - Skip (not advised for weekly/monthly/yearly schedules as
    ///   disbursement will not be generated until next scheduled date)
    ///
    /// **OpenAPI type:** integer (ref: SkipOffDays)
    #[serde(default, with = "bool_from_int_default_false")]
    pub skip_off_days: bool,

    /// Whether sameDay payout is enabled.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: SameDay)
    #[serde(default, with = "bool_from_int_default_false")]
    pub same_day: bool,

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

    /// Disbursements associated with this payout.
    ///
    /// **OpenAPI type:** array of disbursementsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub disbursements: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Payout Struct Tests ====================

    #[test]
    fn payout_deserialize_full() {
        let json = r#"{
            "id": "t1_pay_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "account": "t1_acc_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "billing": "t1_bil_12345678901234567890123",
            "payoutFlow": "t1_pfl_12345678901234567890123",
            "name": "Daily Payout",
            "description": "Daily operating payout",
            "schedule": 1,
            "scheduleFactor": 1,
            "start": 20240101,
            "currency": "USD",
            "um": 2,
            "amount": 50000,
            "minimum": 5000,
            "maximum": 100000,
            "float": 10000,
            "secondaryDescriptor": "MERCHANT PAYOUT",
            "skipOffDays": 0,
            "sameDay": 1,
            "inactive": 0,
            "frozen": 0
        }"#;

        let payout: Payout = serde_json::from_str(json).unwrap();
        assert_eq!(payout.id.as_str(), "t1_pay_12345678901234567890123");
        assert_eq!(payout.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(payout.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            payout.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            payout.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            payout.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            payout.account.as_ref().map(|a| a.as_str()),
            Some("t1_acc_12345678901234567890123")
        );
        assert_eq!(
            payout.entity.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(
            payout.billing.as_ref().map(|b| b.as_str()),
            Some("t1_bil_12345678901234567890123")
        );
        assert_eq!(
            payout.payout_flow.as_ref().map(|p| p.as_str()),
            Some("t1_pfl_12345678901234567890123")
        );
        assert_eq!(payout.name, Some("Daily Payout".to_string()));
        assert_eq!(payout.description, Some("Daily operating payout".to_string()));
        assert_eq!(payout.schedule, Some(PayoutSchedule::Daily));
        assert_eq!(payout.schedule_factor, Some(1));
        assert_eq!(payout.start, Some(20240101));
        assert_eq!(payout.currency, Some("USD".to_string()));
        assert_eq!(payout.um, Some(PayoutUnit::Actual));
        assert_eq!(payout.amount, Some(50000));
        assert_eq!(payout.minimum, Some(5000));
        assert_eq!(payout.maximum, Some(100000));
        assert_eq!(payout.float_balance, Some(10000));
        assert_eq!(
            payout.secondary_descriptor,
            Some("MERCHANT PAYOUT".to_string())
        );
        assert!(!payout.skip_off_days);
        assert!(payout.same_day);
        assert!(!payout.inactive);
        assert!(!payout.frozen);
    }

    #[test]
    fn payout_deserialize_minimal() {
        let json = r#"{"id": "t1_pay_12345678901234567890123"}"#;

        let payout: Payout = serde_json::from_str(json).unwrap();
        assert_eq!(payout.id.as_str(), "t1_pay_12345678901234567890123");
        assert!(payout.created.is_none());
        assert!(payout.modified.is_none());
        assert!(payout.creator.is_none());
        assert!(payout.modifier.is_none());
        assert!(payout.login.is_none());
        assert!(payout.account.is_none());
        assert!(payout.entity.is_none());
        assert!(payout.billing.is_none());
        assert!(payout.payout_flow.is_none());
        assert!(payout.name.is_none());
        assert!(payout.description.is_none());
        assert!(payout.schedule.is_none());
        assert!(payout.schedule_factor.is_none());
        assert!(payout.start.is_none());
        assert!(payout.currency.is_none());
        assert!(payout.um.is_none());
        assert!(payout.amount.is_none());
        assert!(payout.minimum.is_none());
        assert!(payout.maximum.is_none());
        assert!(payout.float_balance.is_none());
        assert!(payout.secondary_descriptor.is_none());
        assert!(!payout.skip_off_days);
        assert!(!payout.same_day);
        assert!(!payout.inactive);
        assert!(!payout.frozen);
    }

    #[test]
    fn payout_bool_from_int() {
        let json = r#"{
            "id": "t1_pay_12345678901234567890123",
            "skipOffDays": 1,
            "sameDay": 1,
            "inactive": 1,
            "frozen": 1
        }"#;

        let payout: Payout = serde_json::from_str(json).unwrap();
        assert!(payout.skip_off_days);
        assert!(payout.same_day);
        assert!(payout.inactive);
        assert!(payout.frozen);
    }

    #[test]
    fn payout_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_pay_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "schedule": 1,
            "um": 2,
            "amount": 50000
        }"#;

        let payout: Payout = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&payout).unwrap();
        let deserialized: Payout = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payout.id, deserialized.id);
        assert_eq!(payout.entity, deserialized.entity);
        assert_eq!(payout.schedule, deserialized.schedule);
        assert_eq!(payout.um, deserialized.um);
        assert_eq!(payout.amount, deserialized.amount);
    }

    #[test]
    fn payout_float_field_serialization() {
        // Verify float field uses correct JSON name "float" not "floatBalance"
        let json = r#"{"id": "t1_pay_12345678901234567890123", "float": 5000}"#;
        let payout: Payout = serde_json::from_str(json).unwrap();
        assert_eq!(payout.float_balance, Some(5000));

        let serialized = serde_json::to_string(&payout).unwrap();
        assert!(serialized.contains("\"float\":5000"));
    }
}
