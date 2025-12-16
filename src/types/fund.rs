//! Fund types for the Payrix API.
//!
//! Funds represent account balances and funding sources.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

/// Fund type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FundType {
    /// Operating fund (main merchant balance)
    #[default]
    Operating = 1,
    /// Reserve fund (held for chargebacks/risk)
    Reserve = 2,
    /// Fee fund (collected fees)
    Fee = 3,
    /// Adjustment fund
    Adjustment = 4,
}

/// Disbursement status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DisbursementStatus {
    /// Disbursement requested
    #[default]
    Requested = 1,
    /// Disbursement processing
    Processing = 2,
    /// Disbursement processed successfully
    Processed = 3,
    /// Disbursement failed
    Failed = 4,
    /// Disbursement denied
    Denied = 5,
    /// Disbursement returned
    Returned = 6,
}

/// Disbursement code values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisbursementCode {
    /// Pending
    #[default]
    Pending,
    /// Internal error
    Internal,
    /// Non-sufficient funds
    #[serde(rename = "nsf")]
    Nsf,
    /// Bad account
    BadAccount,
    /// Unauthorized
    Unauthorized,
    /// General error
    General,
    /// Notification of change
    #[serde(rename = "noc")]
    Noc,
    /// Parameter error
    Parameter,
    /// Same day disbursement
    SameDay,
    /// Transfer details issue
    TransferDetails,
    /// Platform error
    Platform,
}

/// Payout schedule values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PayoutSchedule {
    /// Daily payouts
    Daily = 1,
    /// Weekly payouts
    Weekly = 2,
    /// Monthly payouts
    Monthly = 3,
    /// Annual payouts
    Annually = 4,
    /// Single/one-time payout
    #[default]
    Single = 5,
}

/// Payout unit of measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PayoutUnit {
    /// Percentage of balance
    Percent = 1,
    /// Actual/fixed amount
    #[default]
    Actual = 2,
    /// Negative percentage
    PercentNegative = 3,
}

/// Fee type values.
/// Per OpenAPI spec: integer enum [1, 2]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FeeType {
    /// Standard fee
    #[default]
    Fee = 1,
    /// Assessment fee
    Assessment = 2,
}

/// Fee rule type values.
/// Per OpenAPI spec: string enum (used for FeeRule.type, different from Fee.type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeeRuleType {
    /// Method-based fee rule
    #[default]
    Method,
    /// BIN-based fee rule
    Bin,
    /// AVS result-based fee rule
    AvsResult,
    /// Business type-based fee rule
    Business,
}

/// Fee unit of measure (feeUm in OpenAPI).
/// Per OpenAPI spec: integer enum [1, 2, 3]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FeeUnit {
    /// Percentage
    Percent = 1,
    /// Fixed amount
    #[default]
    Fixed = 2,
    /// Surcharge
    Surcharge = 3,
}

/// Fee collection scope.
/// Per OpenAPI spec: integer enum [1, 2, 3]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum FeeCollection {
    /// Per transaction
    #[default]
    Transaction = 1,
    /// Per entity tax ID
    TransactionTaxId = 2,
    /// Per merchant
    TransactionMerchant = 3,
}

/// Batch status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchStatus {
    /// Batch is open
    #[default]
    Open,
    /// Batch has been processed
    Processed,
    /// Batch is closed
    Closed,
}

/// A Payrix fund balance.
///
/// Funds track available and pending balances for entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Fund {
    /// Unique identifier (30 characters, e.g., "t1_fnd_...")
    pub id: PayrixId,

    /// Entity ID that owns this fund
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID (if merchant-level fund)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this fund
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Fund name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Fund type
    #[serde(default, rename = "type")]
    pub fund_type: Option<FundType>,

    /// Available balance (may be in dollars with fractional cents from API)
    #[serde(default)]
    pub available: Option<f64>,

    /// Pending balance (not yet available)
    #[serde(default)]
    pub pending: Option<f64>,

    /// Reserved/held balance
    #[serde(default)]
    pub reserved: Option<f64>,

    /// Total balance (available + pending + reserved)
    #[serde(default)]
    pub total: Option<f64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Whether this is the default fund
    #[serde(default, rename = "default", with = "bool_from_int_default_false")]
    pub is_default: bool,

    /// Fund status
    #[serde(default)]
    pub status: Option<i32>,

    /// Minimum balance threshold in cents
    #[serde(default)]
    pub minimum: Option<i64>,

    /// Maximum balance threshold in cents
    #[serde(default)]
    pub maximum: Option<i64>,

    /// Description/notes
    #[serde(default)]
    pub description: Option<String>,

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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FundType Tests ====================

    #[test]
    fn fund_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FundType::Operating).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FundType::Reserve).unwrap(), "2");
        assert_eq!(serde_json::to_string(&FundType::Fee).unwrap(), "3");
        assert_eq!(serde_json::to_string(&FundType::Adjustment).unwrap(), "4");
    }

    #[test]
    fn fund_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FundType>("1").unwrap(), FundType::Operating);
        assert_eq!(serde_json::from_str::<FundType>("2").unwrap(), FundType::Reserve);
        assert_eq!(serde_json::from_str::<FundType>("3").unwrap(), FundType::Fee);
        assert_eq!(serde_json::from_str::<FundType>("4").unwrap(), FundType::Adjustment);
    }

    #[test]
    fn fund_type_default() {
        assert_eq!(FundType::default(), FundType::Operating);
    }

    // ==================== DisbursementStatus Tests ====================

    #[test]
    fn disbursement_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&DisbursementStatus::Requested).unwrap(), "1");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Processing).unwrap(), "2");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Processed).unwrap(), "3");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Failed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Denied).unwrap(), "5");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Returned).unwrap(), "6");
    }

    #[test]
    fn disbursement_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<DisbursementStatus>("1").unwrap(), DisbursementStatus::Requested);
        assert_eq!(serde_json::from_str::<DisbursementStatus>("2").unwrap(), DisbursementStatus::Processing);
        assert_eq!(serde_json::from_str::<DisbursementStatus>("3").unwrap(), DisbursementStatus::Processed);
        assert_eq!(serde_json::from_str::<DisbursementStatus>("4").unwrap(), DisbursementStatus::Failed);
        assert_eq!(serde_json::from_str::<DisbursementStatus>("5").unwrap(), DisbursementStatus::Denied);
        assert_eq!(serde_json::from_str::<DisbursementStatus>("6").unwrap(), DisbursementStatus::Returned);
    }

    #[test]
    fn disbursement_status_default() {
        assert_eq!(DisbursementStatus::default(), DisbursementStatus::Requested);
    }

    // ==================== DisbursementCode Tests ====================

    #[test]
    fn disbursement_code_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&DisbursementCode::Pending).unwrap(), "\"pending\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Internal).unwrap(), "\"internal\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Nsf).unwrap(), "\"nsf\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::BadAccount).unwrap(), "\"badAccount\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Unauthorized).unwrap(), "\"unauthorized\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::General).unwrap(), "\"general\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Noc).unwrap(), "\"noc\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Parameter).unwrap(), "\"parameter\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::SameDay).unwrap(), "\"sameDay\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::TransferDetails).unwrap(), "\"transferDetails\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Platform).unwrap(), "\"platform\"");
    }

    #[test]
    fn disbursement_code_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"pending\"").unwrap(), DisbursementCode::Pending);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"internal\"").unwrap(), DisbursementCode::Internal);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"nsf\"").unwrap(), DisbursementCode::Nsf);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"badAccount\"").unwrap(), DisbursementCode::BadAccount);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"unauthorized\"").unwrap(), DisbursementCode::Unauthorized);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"general\"").unwrap(), DisbursementCode::General);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"noc\"").unwrap(), DisbursementCode::Noc);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"parameter\"").unwrap(), DisbursementCode::Parameter);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"sameDay\"").unwrap(), DisbursementCode::SameDay);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"transferDetails\"").unwrap(), DisbursementCode::TransferDetails);
        assert_eq!(serde_json::from_str::<DisbursementCode>("\"platform\"").unwrap(), DisbursementCode::Platform);
    }

    #[test]
    fn disbursement_code_default() {
        assert_eq!(DisbursementCode::default(), DisbursementCode::Pending);
    }

    // ==================== PayoutSchedule Tests ====================

    #[test]
    fn payout_schedule_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PayoutSchedule::Daily).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Weekly).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Monthly).unwrap(), "3");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Annually).unwrap(), "4");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Single).unwrap(), "5");
    }

    #[test]
    fn payout_schedule_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PayoutSchedule>("1").unwrap(), PayoutSchedule::Daily);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("2").unwrap(), PayoutSchedule::Weekly);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("3").unwrap(), PayoutSchedule::Monthly);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("4").unwrap(), PayoutSchedule::Annually);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("5").unwrap(), PayoutSchedule::Single);
    }

    #[test]
    fn payout_schedule_default() {
        assert_eq!(PayoutSchedule::default(), PayoutSchedule::Single);
    }

    // ==================== PayoutUnit Tests ====================

    #[test]
    fn payout_unit_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PayoutUnit::Percent).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PayoutUnit::Actual).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PayoutUnit::PercentNegative).unwrap(), "3");
    }

    #[test]
    fn payout_unit_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PayoutUnit>("1").unwrap(), PayoutUnit::Percent);
        assert_eq!(serde_json::from_str::<PayoutUnit>("2").unwrap(), PayoutUnit::Actual);
        assert_eq!(serde_json::from_str::<PayoutUnit>("3").unwrap(), PayoutUnit::PercentNegative);
    }

    #[test]
    fn payout_unit_default() {
        assert_eq!(PayoutUnit::default(), PayoutUnit::Actual);
    }

    // ==================== FeeType Tests ====================
    // Per OpenAPI spec: integer enum [1, 2]

    #[test]
    fn fee_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeType::Fee).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeType::Assessment).unwrap(), "2");
    }

    #[test]
    fn fee_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeType>("1").unwrap(), FeeType::Fee);
        assert_eq!(serde_json::from_str::<FeeType>("2").unwrap(), FeeType::Assessment);
    }

    #[test]
    fn fee_type_default() {
        assert_eq!(FeeType::default(), FeeType::Fee);
    }

    // ==================== FeeUnit Tests ====================
    // Per OpenAPI spec: integer enum [1, 2, 3]

    #[test]
    fn fee_unit_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeUnit::Percent).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeUnit::Fixed).unwrap(), "2");
        assert_eq!(serde_json::to_string(&FeeUnit::Surcharge).unwrap(), "3");
    }

    #[test]
    fn fee_unit_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeUnit>("1").unwrap(), FeeUnit::Percent);
        assert_eq!(serde_json::from_str::<FeeUnit>("2").unwrap(), FeeUnit::Fixed);
        assert_eq!(serde_json::from_str::<FeeUnit>("3").unwrap(), FeeUnit::Surcharge);
    }

    #[test]
    fn fee_unit_default() {
        assert_eq!(FeeUnit::default(), FeeUnit::Fixed);
    }

    // ==================== FeeCollection Tests ====================
    // Per OpenAPI spec: integer enum [1, 2, 3]

    #[test]
    fn fee_collection_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeCollection::Transaction).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeCollection::TransactionTaxId).unwrap(), "2");
        assert_eq!(serde_json::to_string(&FeeCollection::TransactionMerchant).unwrap(), "3");
    }

    #[test]
    fn fee_collection_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeCollection>("1").unwrap(), FeeCollection::Transaction);
        assert_eq!(serde_json::from_str::<FeeCollection>("2").unwrap(), FeeCollection::TransactionTaxId);
        assert_eq!(serde_json::from_str::<FeeCollection>("3").unwrap(), FeeCollection::TransactionMerchant);
    }

    #[test]
    fn fee_collection_default() {
        assert_eq!(FeeCollection::default(), FeeCollection::Transaction);
    }

    // ==================== BatchStatus Tests ====================

    #[test]
    fn batch_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&BatchStatus::Open).unwrap(), "\"open\"");
        assert_eq!(serde_json::to_string(&BatchStatus::Processed).unwrap(), "\"processed\"");
    }

    #[test]
    fn batch_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<BatchStatus>("\"open\"").unwrap(), BatchStatus::Open);
        assert_eq!(serde_json::from_str::<BatchStatus>("\"processed\"").unwrap(), BatchStatus::Processed);
    }

    #[test]
    fn batch_status_default() {
        assert_eq!(BatchStatus::default(), BatchStatus::Open);
    }

    // ==================== Fund Struct Tests ====================

    #[test]
    fn fund_deserialize_full() {
        let json = r#"{
            "id": "t1_fnd_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "name": "Operating Fund",
            "type": 1,
            "available": 100000,
            "pending": 25000,
            "reserved": 5000,
            "total": 130000,
            "currency": "USD",
            "default": 1,
            "status": 1,
            "minimum": 0,
            "maximum": 1000000,
            "description": "Main operating fund",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let fund: Fund = serde_json::from_str(json).unwrap();
        assert_eq!(fund.id.as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(fund.name, Some("Operating Fund".to_string()));
        assert_eq!(fund.fund_type, Some(FundType::Operating));
        // NOTE: API returns floats for balance fields (e.g., 3524255.258)
        assert_eq!(fund.available, Some(100000.0));
        assert_eq!(fund.pending, Some(25000.0));
        assert_eq!(fund.total, Some(130000.0));
        assert!(fund.is_default);
        assert!(!fund.inactive);
    }

    #[test]
    fn fund_deserialize_minimal() {
        let json = r#"{"id": "t1_fnd_12345678901234567890123"}"#;
        let fund: Fund = serde_json::from_str(json).unwrap();
        assert_eq!(fund.id.as_str(), "t1_fnd_12345678901234567890123");
        assert!(fund.fund_type.is_none());
        assert!(!fund.is_default);
    }

    #[test]
    fn fund_bool_from_int() {
        let json = r#"{"id": "t1_fnd_12345678901234567890123", "default": 1, "inactive": 1, "frozen": 0}"#;
        let fund: Fund = serde_json::from_str(json).unwrap();
        assert!(fund.is_default);
        assert!(fund.inactive);
        assert!(!fund.frozen);
    }
}
