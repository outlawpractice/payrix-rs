//! Fee types for the Payrix API.
//!
//! Fees represent actual fee charges applied to transactions or accounts.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, FeeType, FeeUnit, PayrixId};

/// A Payrix fee.
///
/// Fees are actual charges applied to transactions or accounts.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    /// Unique identifier (30 characters, e.g., "t1_fee_...")
    pub id: PayrixId,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Transaction ID (if transaction-level fee)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Fee rule ID that generated this fee
    #[serde(default)]
    pub fee_rule: Option<PayrixId>,

    /// Fund ID
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Fee type
    #[serde(default, rename = "type")]
    pub fee_type: Option<FeeType>,

    /// Fee unit
    #[serde(default)]
    pub unit: Option<FeeUnit>,

    /// Fee amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Base amount the fee was calculated on (in cents)
    #[serde(default)]
    pub base: Option<i64>,

    /// Rate/percentage used for calculation
    #[serde(default)]
    pub rate: Option<i32>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Fee name/label
    #[serde(default)]
    pub name: Option<String>,

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

    // ==================== Fee Struct Tests ====================

    #[test]
    fn fee_deserialize_full() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "feeRule": "t1_fer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "type": 1,
            "unit": 2,
            "amount": 250,
            "base": 10000,
            "rate": 250,
            "currency": "USD",
            "name": "Transaction Fee",
            "description": "Standard processing fee",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.id.as_str(), "t1_fee_12345678901234567890123");
        assert_eq!(fee.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(fee.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(fee.txn.unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(fee.fee_rule.unwrap().as_str(), "t1_fer_12345678901234567890123");
        assert_eq!(fee.fund.unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(fee.fee_type, Some(FeeType::Fee));
        assert_eq!(fee.unit, Some(FeeUnit::Fixed));
        assert_eq!(fee.amount, Some(250));
        assert_eq!(fee.base, Some(10000));
        assert_eq!(fee.rate, Some(250));
        assert_eq!(fee.currency, Some("USD".to_string()));
        assert_eq!(fee.name, Some("Transaction Fee".to_string()));
        assert_eq!(fee.description, Some("Standard processing fee".to_string()));
        assert_eq!(fee.custom, Some("custom data".to_string()));
        assert_eq!(fee.created, Some("2024-01-01 00:00:00.000".to_string()));
        assert_eq!(fee.modified, Some("2024-04-01 12:00:00.000".to_string()));
        assert!(!fee.inactive);
        assert!(fee.frozen);
    }

    #[test]
    fn fee_deserialize_minimal() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123"
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.id.as_str(), "t1_fee_12345678901234567890123");
        assert!(fee.entity.is_none());
        assert!(fee.merchant.is_none());
        assert!(fee.txn.is_none());
        assert!(fee.fee_rule.is_none());
        assert!(fee.fund.is_none());
        assert!(fee.fee_type.is_none());
        assert!(fee.unit.is_none());
        assert!(fee.amount.is_none());
        assert!(fee.base.is_none());
        assert!(fee.rate.is_none());
        assert!(fee.currency.is_none());
        assert!(fee.name.is_none());
        assert!(fee.description.is_none());
        assert!(fee.custom.is_none());
        assert!(fee.created.is_none());
        assert!(fee.modified.is_none());
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
        let test_cases = vec![
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
    fn fee_unit_variants() {
        let test_cases = vec![
            (1, FeeUnit::Percent),
            (2, FeeUnit::Fixed),
            (3, FeeUnit::Surcharge),
        ];

        for (unit_val, expected_unit) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fee_12345678901234567890123", "unit": {}}}"#,
                unit_val
            );
            let fee: Fee = serde_json::from_str(&json).unwrap();
            assert_eq!(fee.unit, Some(expected_unit));
        }
    }

    #[test]
    fn fee_fixed_amount() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "unit": 2,
            "amount": 150
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.unit, Some(FeeUnit::Fixed));
        assert_eq!(fee.amount, Some(150));
    }

    #[test]
    fn fee_percentage_calculation() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "unit": 1,
            "base": 10000,
            "rate": 250,
            "amount": 250
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.unit, Some(FeeUnit::Percent));
        assert_eq!(fee.base, Some(10000));
        assert_eq!(fee.rate, Some(250));
        assert_eq!(fee.amount, Some(250));

        // Verify calculation: 10000 * 250 / 10000 = 250 cents
        let calculated = (fee.base.unwrap() * fee.rate.unwrap() as i64) / 10000;
        assert_eq!(calculated, fee.amount.unwrap());
    }

    #[test]
    fn fee_serialize_roundtrip() {
        let fee = Fee {
            id: "t1_fee_12345678901234567890123".parse().unwrap(),
            entity: Some("t1_ent_12345678901234567890123".parse().unwrap()),
            merchant: Some("t1_mer_12345678901234567890123".parse().unwrap()),
            txn: Some("t1_txn_12345678901234567890123".parse().unwrap()),
            fee_rule: Some("t1_fer_12345678901234567890123".parse().unwrap()),
            fund: Some("t1_fnd_12345678901234567890123".parse().unwrap()),
            fee_type: Some(FeeType::Fee),
            unit: Some(FeeUnit::Fixed),
            amount: Some(100),
            base: Some(5000),
            rate: Some(200),
            currency: Some("USD".to_string()),
            name: Some("Test Fee".to_string()),
            description: Some("Test".to_string()),
            custom: Some("custom".to_string()),
            created: Some("2024-01-01 00:00:00.000".to_string()),
            modified: Some("2024-01-02 00:00:00.000".to_string()),
            inactive: false,
            frozen: true,
        };

        let json = serde_json::to_string(&fee).unwrap();
        let deserialized: Fee = serde_json::from_str(&json).unwrap();
        assert_eq!(fee, deserialized);
    }

    #[test]
    fn fee_zero_amount() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "amount": 0
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.amount, Some(0));
    }

    #[test]
    fn fee_large_amount() {
        let json = r#"{
            "id": "t1_fee_12345678901234567890123",
            "amount": 999999999
        }"#;

        let fee: Fee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.amount, Some(999999999));
    }
}
