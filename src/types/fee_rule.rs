//! Fee rule types for the Payrix API.
//!
//! Fee rules define how and when fees are calculated and applied to transactions.

use serde::{Deserialize, Serialize};

use super::{
    bool_from_int_default_false, option_bool_from_int, FeeCollection, FeeRuleType, FeeUnit, PayrixId,
};

/// A Payrix fee rule.
///
/// Fee rules configure automatic fee calculation for transactions.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct FeeRule {
    /// Unique identifier (30 characters, e.g., "t1_fer_...")
    pub id: PayrixId,

    /// Entity ID that owns this rule
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID (if merchant-specific)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this rule
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Fee rule type (different from Fee.type - uses string enum, not integer)
    #[serde(default, rename = "type")]
    pub fee_type: Option<FeeRuleType>,

    /// Fee unit (percentage or fixed)
    #[serde(default)]
    pub unit: Option<FeeUnit>,

    /// Fee collection scope
    #[serde(default)]
    pub collection: Option<FeeCollection>,

    /// Fee amount (in cents if fixed, percentage if percent)
    #[serde(default)]
    pub amount: Option<i64>,

    /// Minimum fee amount in cents
    #[serde(default)]
    pub min: Option<i64>,

    /// Maximum fee amount in cents
    #[serde(default)]
    pub max: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Card brand filter (e.g., "visa", "mastercard")
    #[serde(default)]
    pub card_brand: Option<String>,

    /// Card type filter (e.g., "credit", "debit")
    #[serde(default)]
    pub card_type: Option<String>,

    /// Transaction type filter
    #[serde(default)]
    pub txn_type: Option<String>,

    /// Fee rule name/label
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

/// Request to create a new fee rule.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFeeRule {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID (if merchant-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Fee rule type (different from Fee.type - uses string enum, not integer)
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub fee_type: Option<FeeRuleType>,

    /// Fee unit (percentage or fixed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<FeeUnit>,

    /// Fee collection scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<FeeCollection>,

    /// Fee amount (required)
    pub amount: i64,

    /// Minimum fee amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,

    /// Maximum fee amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Card brand filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,

    /// Card type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,

    /// Transaction type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_type: Option<String>,

    /// Fee rule name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description/notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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

    // ==================== FeeRule Struct Tests ====================

    #[test]
    fn fee_rule_deserialize_full() {
        // Per OpenAPI: type is string enum, unit/collection are integers
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "type": "method",
            "unit": 2,
            "collection": 1,
            "amount": 250,
            "min": 50,
            "max": 1000,
            "currency": "USD",
            "cardBrand": "visa",
            "cardType": "credit",
            "txnType": "sale",
            "name": "Standard Fee Rule",
            "description": "Standard processing fee",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.id.as_str(), "t1_fer_12345678901234567890123");
        assert_eq!(fee_rule.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(fee_rule.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(fee_rule.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(fee_rule.fee_type, Some(FeeRuleType::Method));
        assert_eq!(fee_rule.unit, Some(FeeUnit::Fixed));
        assert_eq!(fee_rule.collection, Some(FeeCollection::Transaction));
        assert_eq!(fee_rule.amount, Some(250));
        assert_eq!(fee_rule.min, Some(50));
        assert_eq!(fee_rule.max, Some(1000));
        assert_eq!(fee_rule.currency, Some("USD".to_string()));
        assert_eq!(fee_rule.card_brand, Some("visa".to_string()));
        assert_eq!(fee_rule.card_type, Some("credit".to_string()));
        assert_eq!(fee_rule.txn_type, Some("sale".to_string()));
        assert_eq!(fee_rule.name, Some("Standard Fee Rule".to_string()));
        assert_eq!(fee_rule.description, Some("Standard processing fee".to_string()));
        assert_eq!(fee_rule.custom, Some("custom data".to_string()));
        assert_eq!(fee_rule.created, Some("2024-01-01 00:00:00.000".to_string()));
        assert_eq!(fee_rule.modified, Some("2024-04-01 12:00:00.000".to_string()));
        assert!(!fee_rule.inactive);
        assert!(fee_rule.frozen);
    }

    #[test]
    fn fee_rule_deserialize_minimal() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123"
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.id.as_str(), "t1_fer_12345678901234567890123");
        assert!(fee_rule.entity.is_none());
        assert!(fee_rule.merchant.is_none());
        assert!(fee_rule.login.is_none());
        assert!(fee_rule.fee_type.is_none());
        assert!(fee_rule.unit.is_none());
        assert!(fee_rule.collection.is_none());
        assert!(fee_rule.amount.is_none());
        assert!(fee_rule.min.is_none());
        assert!(fee_rule.max.is_none());
        assert!(fee_rule.currency.is_none());
        assert!(fee_rule.card_brand.is_none());
        assert!(fee_rule.card_type.is_none());
        assert!(fee_rule.txn_type.is_none());
        assert!(fee_rule.name.is_none());
        assert!(fee_rule.description.is_none());
        assert!(fee_rule.custom.is_none());
        assert!(fee_rule.created.is_none());
        assert!(fee_rule.modified.is_none());
        assert!(!fee_rule.inactive);
        assert!(!fee_rule.frozen);
    }

    #[test]
    fn fee_rule_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_fer_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert!(!fee_rule.inactive);
        assert!(!fee_rule.frozen);
    }

    #[test]
    fn fee_rule_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_fer_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert!(fee_rule.inactive);
        assert!(fee_rule.frozen);
    }

    #[test]
    fn fee_rule_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_fer_12345678901234567890123"}"#;
        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert!(!fee_rule.inactive);
        assert!(!fee_rule.frozen);
    }

    #[test]
    fn fee_rule_all_fee_type_variants() {
        // Per OpenAPI: type is string enum (different from Fee.type)
        let test_cases = vec![
            ("method", FeeRuleType::Method),
            ("bin", FeeRuleType::Bin),
            ("avsresult", FeeRuleType::AvsResult),
            ("business", FeeRuleType::Business),
        ];

        for (type_val, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "type": "{}"}}"#,
                type_val
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.fee_type, Some(expected_type));
        }
    }

    #[test]
    fn fee_rule_all_unit_variants() {
        // Per OpenAPI: unit (feeUm) is integer enum [1, 2, 3]
        let test_cases = vec![
            (1, FeeUnit::Percent),
            (2, FeeUnit::Fixed),
            (3, FeeUnit::Surcharge),
        ];

        for (unit_val, expected_unit) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "unit": {}}}"#,
                unit_val
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.unit, Some(expected_unit));
        }
    }

    #[test]
    fn fee_rule_all_collection_variants() {
        // Per OpenAPI: collection is integer enum [1, 2, 3]
        let test_cases = vec![
            (1, FeeCollection::Transaction),
            (2, FeeCollection::TransactionTaxId),
            (3, FeeCollection::TransactionMerchant),
        ];

        for (collection_val, expected_collection) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "collection": {}}}"#,
                collection_val
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.collection, Some(expected_collection));
        }
    }

    #[test]
    fn fee_rule_min_max_constraints() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "min": 100,
            "max": 500
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.min, Some(100));
        assert_eq!(fee_rule.max, Some(500));
    }

    #[test]
    fn fee_rule_card_filters() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "cardBrand": "mastercard",
            "cardType": "debit"
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.card_brand, Some("mastercard".to_string()));
        assert_eq!(fee_rule.card_type, Some("debit".to_string()));
    }

    #[test]
    fn fee_rule_serialize_roundtrip() {
        let fee_rule = FeeRule {
            id: "t1_fer_12345678901234567890123".parse().unwrap(),
            entity: Some("t1_ent_12345678901234567890123".parse().unwrap()),
            merchant: Some("t1_mer_12345678901234567890123".parse().unwrap()),
            login: Some("t1_log_12345678901234567890123".parse().unwrap()),
            fee_type: Some(FeeRuleType::Method),
            unit: Some(FeeUnit::Fixed),
            collection: Some(FeeCollection::Transaction),
            amount: Some(100),
            min: Some(50),
            max: Some(500),
            currency: Some("USD".to_string()),
            card_brand: Some("visa".to_string()),
            card_type: Some("credit".to_string()),
            txn_type: Some("sale".to_string()),
            name: Some("Test Fee Rule".to_string()),
            description: Some("Test".to_string()),
            custom: Some("custom".to_string()),
            created: Some("2024-01-01 00:00:00.000".to_string()),
            modified: Some("2024-01-02 00:00:00.000".to_string()),
            inactive: false,
            frozen: true,
        };

        let json = serde_json::to_string(&fee_rule).unwrap();
        let deserialized: FeeRule = serde_json::from_str(&json).unwrap();
        assert_eq!(fee_rule, deserialized);
    }

    // ==================== NewFeeRule Tests ====================

    #[test]
    fn new_fee_rule_serialize_full() {
        let new_fee_rule = NewFeeRule {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            fee_type: Some(FeeRuleType::Method),
            unit: Some(FeeUnit::Fixed),
            collection: Some(FeeCollection::Transaction),
            amount: 250,
            min: Some(50),
            max: Some(1000),
            currency: Some("USD".to_string()),
            card_brand: Some("visa".to_string()),
            card_type: Some("credit".to_string()),
            txn_type: Some("sale".to_string()),
            name: Some("Standard Fee Rule".to_string()),
            description: Some("Standard processing fee".to_string()),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        // Per OpenAPI: type is string, unit/collection are integers
        assert!(json.contains("\"type\":\"method\""));
        assert!(json.contains("\"unit\":2"));
        assert!(json.contains("\"collection\":1"));
        assert!(json.contains("\"amount\":250"));
        assert!(json.contains("\"min\":50"));
        assert!(json.contains("\"max\":1000"));
        assert!(json.contains("\"currency\":\"USD\""));
        assert!(json.contains("\"cardBrand\":\"visa\""));
        assert!(json.contains("\"cardType\":\"credit\""));
        assert!(json.contains("\"txnType\":\"sale\""));
        assert!(json.contains("\"name\":\"Standard Fee Rule\""));
        assert!(json.contains("\"description\":\"Standard processing fee\""));
        assert!(json.contains("\"custom\":\"custom data\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_fee_rule_serialize_minimal() {
        let new_fee_rule = NewFeeRule {
            amount: 100,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"amount\":100"));
        // Optional fields should be omitted
        assert!(!json.contains("\"entity\""));
        assert!(!json.contains("\"merchant\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"unit\""));
        assert!(!json.contains("\"collection\""));
        assert!(!json.contains("\"min\""));
        assert!(!json.contains("\"max\""));
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_fee_rule_option_bool_to_int_true() {
        let new_fee_rule = NewFeeRule {
            amount: 100,
            inactive: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn new_fee_rule_option_bool_to_int_false() {
        let new_fee_rule = NewFeeRule {
            amount: 100,
            inactive: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_fee_rule_option_bool_none_omitted() {
        let new_fee_rule = NewFeeRule {
            amount: 100,
            inactive: None,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_fee_rule_with_all_enum_variants() {
        let new_fee_rule = NewFeeRule {
            amount: 500,
            fee_type: Some(FeeRuleType::Bin),
            unit: Some(FeeUnit::Percent),
            collection: Some(FeeCollection::TransactionMerchant),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        // Per OpenAPI: type="bin" (string), unit=1 (Percent), collection=3 (TransactionMerchant)
        assert!(json.contains("\"type\":\"bin\""));
        assert!(json.contains("\"unit\":1"));
        assert!(json.contains("\"collection\":3"));
    }

    #[test]
    fn new_fee_rule_zero_amount() {
        let new_fee_rule = NewFeeRule {
            amount: 0,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"amount\":0"));
    }

    #[test]
    fn new_fee_rule_large_amount() {
        let new_fee_rule = NewFeeRule {
            amount: 999999999,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"amount\":999999999"));
    }

    #[test]
    fn new_fee_rule_with_constraints() {
        let new_fee_rule = NewFeeRule {
            amount: 300,
            min: Some(100),
            max: Some(500),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_fee_rule).unwrap();
        assert!(json.contains("\"amount\":300"));
        assert!(json.contains("\"min\":100"));
        assert!(json.contains("\"max\":500"));
    }
}
