//! Fee types for the Payrix API.
//!
//! Fees represent fee configurations that define when and how fees are charged.
//!
//! **OpenAPI schema:** `feesResponse`

use payrix_macros::PayrixEntity;
use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, FeeCollection, FeeSchedule, FeeType, FeeUnit, PayrixId};

// =============================================================================
// FEE STRUCT
// =============================================================================

/// A Payrix fee configuration.
///
/// Defines when and how fees should be charged.
///
/// **OpenAPI schema:** `feesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateFee, update = UpdateFee)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[payrix(readonly)]
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Entity that will charge this Fee.
    ///
    /// **OpenAPI type:** string (ref: feesModelEntity)
    #[payrix(create_only)]
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Entity that this Fee applies for.
    ///
    /// **OpenAPI type:** string (ref: feesModelForentity)
    #[serde(default)]
    pub forentity: Option<PayrixId>,

    /// The identifier of the Org who should pay this Fee on behalf of the
    /// Entity identified in the `forentity` field.
    ///
    /// This field is optional. If set, the Fee is charged to this Org instead.
    ///
    /// **OpenAPI type:** string (ref: feesModelOrg)
    #[serde(default)]
    pub org: Option<PayrixId>,

    /// The identifier of the Partition who should pay this Fee on behalf of
    /// the Entity identified in the `forentity` field.
    ///
    /// This field is optional. If set, the Fee is charged to the whole Partition.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub partition: Option<String>,

    /// The type of the fee.
    ///
    /// - `1` - Fee (Standard Fee)
    /// - `2` - Assessment (Third-Party Platform Fee)
    ///
    /// **OpenAPI type:** integer (ref: feeType)
    #[serde(default, rename = "type")]
    pub fee_type: Option<FeeType>,

    /// The name of this Fee.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Fee.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub description: Option<String>,

    /// The schedule that determines when this Fee is triggered.
    ///
    /// See `FeeSchedule` enum for all valid values (1-807).
    ///
    /// **OpenAPI type:** integer (ref: feeSchedule)
    #[serde(default)]
    pub schedule: Option<FeeSchedule>,

    /// A multiplier to adjust the schedule for duration-based triggers
    /// (daily, weekly, monthly, annually).
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub schedule_factor: Option<i32>,

    /// The date on which charging of the Fee should start.
    ///
    /// Format: YYYYMMDD (e.g., `20160120` for January 20, 2016).
    /// Value must be a date in the future or present.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub start: Option<i32>,

    /// The date on which charging of the Fee should end.
    ///
    /// Format: YYYYMMDD (e.g., `20160120` for January 20, 2016).
    /// Value must be a date in the future.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub finish: Option<i32>,

    /// Applies the fee based on the volume of a resource.
    ///
    /// - `1` - Txn (total amount of all transactions)
    /// - `2` - Txn-TaxID (total per entity EIN/tax ID)
    /// - `3` - Txn-Merchant (total per entity)
    ///
    /// **OpenAPI type:** integer (ref: feeCollection)
    #[serde(default)]
    pub collection: Option<FeeCollection>,

    /// A multiplier to adjust the data set used in collection calculation.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub collection_factor: Option<i32>,

    /// The number of days, weeks, months or years to go back when
    /// selecting data for collection calculation.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub collection_offset: Option<i32>,

    /// Whether to include the current period for the collection calculation.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub collection_include_current: Option<i32>,

    /// The unit of measure for this Fee.
    ///
    /// - `1` - Percentage (fee as percentage of event amount in basis points)
    /// - `2` - Fixed Amount (fee as fixed amount in cents)
    /// - `3` - Surcharge (percentage assuming event amount includes fee)
    ///
    /// Note: Percentage and Surcharge only apply to monetary event schedules.
    ///
    /// **OpenAPI type:** integer (ref: feeUm)
    #[serde(default)]
    pub um: Option<FeeUnit>,

    /// The total amount of this Fee.
    ///
    /// The units depend on the `um` field:
    /// - If `um` is percentage: basis points
    /// - If `um` is amount: cents (up to three decimal points)
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    /// The maximum amount to charge for this fee (percent-based fees only).
    ///
    /// Will be `null` for all other units.
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub maximum: Option<f64>,

    /// The currency of the amount.
    ///
    /// See [Currency codes](https://www.iban.com/currency-codes) for valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// Indicator to extract fee from transaction-supplied fee.
    ///
    /// When set, amount corresponds to the fee amount in the transaction
    /// and only that amount will be extractable.
    ///
    /// - `0` - Disabled (fee calculated normally)
    /// - `1` - Enabled (fee based on transaction fee)
    ///
    /// **OpenAPI type:** integer (ref: TxnFee)
    #[serde(default)]
    pub txn_fee: Option<i32>,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Fee modifiers associated with this fee.
    ///
    /// **OpenAPI type:** array of feeModifiersResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub fee_modifiers: Option<Vec<serde_json::Value>>,

    /// Fee rules associated with this fee.
    ///
    /// **OpenAPI type:** array of feeRulesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub fee_rules: Option<Vec<serde_json::Value>>,

    /// Assessments associated with this fee.
    ///
    /// **OpenAPI type:** array of assessmentsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub assessments: Option<Vec<serde_json::Value>>,

    /// Billing events associated with this fee.
    ///
    /// **OpenAPI type:** array of billingEventsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub billing_events: Option<Vec<serde_json::Value>>,

    /// Entries associated with this fee.
    ///
    /// **OpenAPI type:** array
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Fee Struct Tests ====================

    #[test]
    fn fee_deserialize_full() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "forentity": "t1_ent_12345678901234567890124",
            "org": "t1_org_12345678901234567890123",
            "partition": "partition123",
            "type": 1,
            "name": "Transaction Fee",
            "description": "Standard processing fee",
            "schedule": 7,
            "scheduleFactor": 1,
            "start": 20240101,
            "finish": 20251231,
            "collection": 1,
            "collectionFactor": 1,
            "collectionOffset": 0,
            "collectionIncludeCurrent": 1,
            "um": 2,
            "amount": 150.5,
            "maximum": 1000.0,
            "currency": "USD",
            "txnFee": 0,
            "inactive": 0,
            "frozen": 1
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.id.as_str(), "t1_fee_12345678901234567890123");
        assert_eq!(fee.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(fee.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(fee.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(fee.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(fee.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(fee.forentity.as_ref().map(|f| f.as_str()), Some("t1_ent_12345678901234567890124"));
        assert_eq!(fee.org.as_ref().map(|o| o.as_str()), Some("t1_org_12345678901234567890123"));
        assert_eq!(fee.partition, Some("partition123".to_string()));
        assert_eq!(fee.fee_type, Some(FeeType::Fee));
        assert_eq!(fee.name, Some("Transaction Fee".to_string()));
        assert_eq!(fee.description, Some("Standard processing fee".to_string()));
        assert_eq!(fee.schedule, Some(FeeSchedule::Capture));
        assert_eq!(fee.schedule_factor, Some(1));
        assert_eq!(fee.start, Some(20240101));
        assert_eq!(fee.finish, Some(20251231));
        assert_eq!(fee.collection, Some(FeeCollection::Transaction));
        assert_eq!(fee.collection_factor, Some(1));
        assert_eq!(fee.collection_offset, Some(0));
        assert_eq!(fee.collection_include_current, Some(1));
        assert_eq!(fee.um, Some(FeeUnit::Fixed));
        assert_eq!(fee.amount, Some(150.5));
        assert_eq!(fee.maximum, Some(1000.0));
        assert_eq!(fee.currency, Some("USD".to_string()));
        assert_eq!(fee.txn_fee, Some(0));
        assert!(!fee.inactive);
        assert!(fee.frozen);
    }

    #[test]
    fn fee_deserialize_minimal() {
        let json = r#"{"id": "t1_fee_12345678901234567890123"}"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.id.as_str(), "t1_fee_12345678901234567890123");
        assert!(fee.created.is_none());
        assert!(fee.modified.is_none());
        assert!(fee.creator.is_none());
        assert!(fee.modifier.is_none());
        assert!(fee.entity.is_none());
        assert!(fee.forentity.is_none());
        assert!(fee.org.is_none());
        assert!(fee.partition.is_none());
        assert!(fee.fee_type.is_none());
        assert!(fee.name.is_none());
        assert!(fee.description.is_none());
        assert!(fee.schedule.is_none());
        assert!(fee.schedule_factor.is_none());
        assert!(fee.start.is_none());
        assert!(fee.finish.is_none());
        assert!(fee.collection.is_none());
        assert!(fee.collection_factor.is_none());
        assert!(fee.collection_offset.is_none());
        assert!(fee.collection_include_current.is_none());
        assert!(fee.um.is_none());
        assert!(fee.amount.is_none());
        assert!(fee.maximum.is_none());
        assert!(fee.currency.is_none());
        assert!(fee.txn_fee.is_none());
        assert!(!fee.inactive);
        assert!(!fee.frozen);
    }

    #[test]
    fn fee_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_fee_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let fee: Fee = serde_json::from_str(json).unwrap();
        assert!(!fee.inactive);
        assert!(!fee.frozen);
    }

    #[test]
    fn fee_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_fee_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let fee: Fee = serde_json::from_str(json).unwrap();
        assert!(fee.inactive);
        assert!(fee.frozen);
    }

    #[test]
    fn fee_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_fee_12345678901234567890123"}"#;
        let fee: Fee = serde_json::from_str(json).unwrap();
        assert!(!fee.inactive);
        assert!(!fee.frozen);
    }

    #[test]
    fn fee_type_variants() {
        let test_cases = [
            (1, FeeType::Fee),
            (2, FeeType::Assessment),
        ];

        for (type_val, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fee_12345678901234567890123", "type": {}}}"#,
                type_val
            );
            let fee: Fee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.fee_type, Some(expected_type));
        }
    }

    #[test]
    fn fee_schedule_common_variants() {
        let test_cases = [
            (1, FeeSchedule::Days),
            (2, FeeSchedule::Weeks),
            (3, FeeSchedule::Months),
            (4, FeeSchedule::Years),
            (5, FeeSchedule::Single),
            (6, FeeSchedule::Auth),
            (7, FeeSchedule::Capture),
            (8, FeeSchedule::Refund),
            (9, FeeSchedule::Board),
            (10, FeeSchedule::Payout),
            (11, FeeSchedule::Chargeback),
            (24, FeeSchedule::Settlement),
        ];

        for (schedule_val, expected_schedule) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fee_12345678901234567890123", "schedule": {}}}"#,
                schedule_val
            );
            let fee: Fee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.schedule, Some(expected_schedule));
        }
    }

    #[test]
    fn fee_um_variants() {
        let test_cases = [
            (1, FeeUnit::Percent),
            (2, FeeUnit::Fixed),
            (3, FeeUnit::Surcharge),
        ];

        for (um_val, expected_um) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fee_12345678901234567890123", "um": {}}}"#,
                um_val
            );
            let fee: Fee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.um, Some(expected_um));
        }
    }

    #[test]
    fn fee_collection_variants() {
        let test_cases = [
            (1, FeeCollection::Transaction),
            (2, FeeCollection::TransactionTaxId),
            (3, FeeCollection::TransactionMerchant),
        ];

        for (coll_val, expected_coll) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fee_12345678901234567890123", "collection": {}}}"#,
                coll_val
            );
            let fee: Fee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.collection, Some(expected_coll));
        }
    }

    #[test]
    fn fee_percentage_based() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "um": 1,
            "amount": 250,
            "maximum": 5000
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.um, Some(FeeUnit::Percent));
        assert_eq!(fee.amount, Some(250.0)); // 250 basis points = 2.5%
        assert_eq!(fee.maximum, Some(5000.0)); // max $50.00
    }

    #[test]
    fn fee_fixed_amount() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "um": 2,
            "amount": 150
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.um, Some(FeeUnit::Fixed));
        assert_eq!(fee.amount, Some(150.0)); // $1.50
        assert!(fee.maximum.is_none()); // not applicable for fixed
    }

    #[test]
    fn fee_date_format() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "start": 20240115,
            "finish": 20241231
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.start, Some(20240115));
        assert_eq!(fee.finish, Some(20241231));
    }

    #[test]
    #[cfg(not(feature = "sqlx"))]
    fn fee_with_nested_relations() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "feeRules": [{"id": "t1_fer_12345678901234567890123"}],
            "assessments": [],
            "billingEvents": []
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert!(fee.fee_rules.is_some());
        assert_eq!(fee.fee_rules.as_ref().unwrap().len(), 1);
        assert!(fee.assessments.is_some());
        assert!(fee.billing_events.is_some());
    }
}
