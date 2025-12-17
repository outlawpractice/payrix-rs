//! Fee rule types for the Payrix API.
//!
//! Fee rules define conditions that determine whether a fee should be applied.
//!
//! **OpenAPI schema:** `feeRulesResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, FeeApplication, FeeRuleType, PayrixId};

// =============================================================================
// FEE RULE STRUCT
// =============================================================================

/// A Payrix fee rule.
///
/// Fee rules define conditions for evaluating whether a fee should apply.
///
/// **OpenAPI schema:** `feeRulesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct FeeRule {
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

    /// The identifier of the Fee that this Fee Rule applies to.
    ///
    /// **OpenAPI type:** string (ref: feeRulesModelFee)
    #[serde(default)]
    pub fee: Option<PayrixId>,

    /// The name of this Fee Rule.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// The description of this Fee Rule.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The type of Fee Rule.
    ///
    /// See `FeeRuleType` for all 44 valid values (AVSRESULT, BIN, BUSINESS, etc.).
    ///
    /// **OpenAPI type:** string (ref: feeRuleType)
    #[serde(default, rename = "type")]
    pub rule_type: Option<FeeRuleType>,

    /// Where the fee rule should apply.
    ///
    /// - `both` - Rule applies to fee and collection calculation
    /// - `fee` - Rule applies only to the fee itself
    /// - `collection` - Rule used only for collection calculation
    ///
    /// **OpenAPI type:** string (ref: feeApplication)
    #[serde(default)]
    pub application: Option<FeeApplication>,

    /// The value to compare against when evaluating this Fee Rule.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub value: Option<String>,

    /// A name for a group of rules to be applied in conjunction.
    ///
    /// When grouping is used, the Fee will be allowed to be processed
    /// if at least one of the rules in the group is matched.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub grouping: Option<String>,

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
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FeeRule Struct Tests ====================

    #[test]
    fn fee_rule_deserialize_full() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "fee": "t1_fee_12345678901234567890123",
            "name": "Visa Credit Rule",
            "description": "Apply fee for Visa credit transactions",
            "type": "METHOD",
            "application": "both",
            "value": "visa",
            "grouping": "card_brand_group",
            "inactive": 0,
            "frozen": 1
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.id.as_str(), "t1_fer_12345678901234567890123");
        assert_eq!(fee_rule.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(fee_rule.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(fee_rule.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(fee_rule.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(fee_rule.fee.as_ref().map(|f| f.as_str()), Some("t1_fee_12345678901234567890123"));
        assert_eq!(fee_rule.name, Some("Visa Credit Rule".to_string()));
        assert_eq!(fee_rule.description, Some("Apply fee for Visa credit transactions".to_string()));
        assert_eq!(fee_rule.rule_type, Some(FeeRuleType::Method));
        assert_eq!(fee_rule.application, Some(FeeApplication::Both));
        assert_eq!(fee_rule.value, Some("visa".to_string()));
        assert_eq!(fee_rule.grouping, Some("card_brand_group".to_string()));
        assert!(!fee_rule.inactive);
        assert!(fee_rule.frozen);
    }

    #[test]
    fn fee_rule_deserialize_minimal() {
        let json = r#"{"id": "t1_fer_12345678901234567890123"}"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.id.as_str(), "t1_fer_12345678901234567890123");
        assert!(fee_rule.created.is_none());
        assert!(fee_rule.modified.is_none());
        assert!(fee_rule.creator.is_none());
        assert!(fee_rule.modifier.is_none());
        assert!(fee_rule.fee.is_none());
        assert!(fee_rule.name.is_none());
        assert!(fee_rule.description.is_none());
        assert!(fee_rule.rule_type.is_none());
        assert!(fee_rule.application.is_none());
        assert!(fee_rule.value.is_none());
        assert!(fee_rule.grouping.is_none());
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
    fn fee_rule_type_common_variants() {
        let test_cases = [
            ("METHOD", FeeRuleType::Method),
            ("BIN", FeeRuleType::Bin),
            ("AVSRESULT", FeeRuleType::AvsResult),
            ("BUSINESS", FeeRuleType::Business),
            ("CVVRESULT", FeeRuleType::CvvResult),
            ("EMV", FeeRuleType::Emv),
            ("INTERCHANGE", FeeRuleType::Interchange),
            ("INTERNATIONAL", FeeRuleType::International),
            ("MCC", FeeRuleType::Mcc),
            ("PLATFORM", FeeRuleType::Platform),
            ("STATUS", FeeRuleType::Status),
            ("SOFTPOS", FeeRuleType::SoftPos),
        ];

        for (type_str, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "type": "{}"}}"#,
                type_str
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.rule_type, Some(expected_type), "Failed for type: {}", type_str);
        }
    }

    #[test]
    fn fee_rule_type_comparison_variants() {
        let test_cases = [
            ("EQUAL", FeeRuleType::Equal),
            ("NOTEQUAL", FeeRuleType::NotEqual),
            ("GREATER", FeeRuleType::Greater),
            ("LESS", FeeRuleType::Less),
        ];

        for (type_str, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "type": "{}"}}"#,
                type_str
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.rule_type, Some(expected_type));
        }
    }

    #[test]
    fn fee_rule_type_special_variants() {
        let test_cases = [
            ("3DSRESULT", FeeRuleType::ThreeDsResult),
            ("IC_RETAIN_PASSTHRU_REFUND", FeeRuleType::IcRetainPassthruRefund),
            ("TAXFORM1099K", FeeRuleType::TaxForm1099K),
        ];

        for (type_str, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "type": "{}"}}"#,
                type_str
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.rule_type, Some(expected_type));
        }
    }

    #[test]
    fn fee_rule_application_variants() {
        let test_cases = [
            ("both", FeeApplication::Both),
            ("fee", FeeApplication::Fee),
            ("collection", FeeApplication::Collection),
        ];

        for (app_str, expected_app) in test_cases {
            let json = format!(
                r#"{{"id": "t1_fer_12345678901234567890123", "application": "{}"}}"#,
                app_str
            );
            let fee_rule: FeeRule = serde_json::from_str(&json).unwrap();
            assert_eq!(fee_rule.application, Some(expected_app));
        }
    }

    #[test]
    fn fee_rule_with_value_and_grouping() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "type": "BIN",
            "value": "411111",
            "grouping": "visa_bins"
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        assert_eq!(fee_rule.rule_type, Some(FeeRuleType::Bin));
        assert_eq!(fee_rule.value, Some("411111".to_string()));
        assert_eq!(fee_rule.grouping, Some("visa_bins".to_string()));
    }

    #[test]
    fn fee_rule_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_fer_12345678901234567890123",
            "type": "METHOD",
            "application": "both",
            "value": "credit",
            "inactive": 0,
            "frozen": 0
        }"#;

        let fee_rule: FeeRule = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&fee_rule).unwrap();
        let deserialized: FeeRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(fee_rule.id, deserialized.id);
        assert_eq!(fee_rule.rule_type, deserialized.rule_type);
        assert_eq!(fee_rule.application, deserialized.application);
        assert_eq!(fee_rule.value, deserialized.value);
    }
}
