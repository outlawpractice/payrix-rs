//! Disbursement types for the Payrix API.
//!
//! Disbursements represent actual fund transfers from a merchant's
//! operating balance to their bank account.

use serde::{Deserialize, Serialize};

use super::{
    bool_from_int_default_false, option_bool_from_int, DateYmd, DisbursementCode,
    DisbursementStatus, PayrixId,
};

/// A Payrix disbursement.
///
/// Disbursements are the actual fund transfers initiated by payouts
/// or on-demand requests. All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Disbursement {
    /// Unique identifier (30 characters, e.g., "t1_dis_...")
    pub id: PayrixId,

    /// Entity ID that owns this disbursement
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID the disbursement is from
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Account ID the disbursement is sent to
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// Payout ID that triggered this disbursement
    #[serde(default)]
    pub payout: Option<PayrixId>,

    /// Login ID that created this disbursement
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Disbursement status
    #[serde(default)]
    pub status: Option<DisbursementStatus>,

    /// Disbursement result code
    #[serde(default)]
    pub code: Option<DisbursementCode>,

    /// Disbursement amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Fee amount in cents
    #[serde(default)]
    pub fee: Option<i64>,

    /// Net amount after fees in cents
    #[serde(default)]
    pub net: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Scheduled date for disbursement (YYYYMMDD format)
    #[serde(default)]
    pub scheduled: Option<DateYmd>,

    /// Date disbursement was processed (YYYYMMDD format)
    #[serde(default)]
    pub processed: Option<DateYmd>,

    /// ACH trace number
    #[serde(default)]
    pub trace: Option<String>,

    /// Disbursement name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Return reason code (if returned)
    #[serde(default)]
    pub return_code: Option<String>,

    /// Return reason description
    #[serde(default)]
    pub return_reason: Option<String>,

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

/// Request to create a new on-demand disbursement.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDisbursement {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Fund ID to disburse from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund: Option<String>,

    /// Account ID to disburse to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Disbursement amount in cents (required)
    pub amount: i64,

    /// Currency code (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Scheduled date (YYYYMMDD format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<String>,

    /// Disbursement name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description/memo
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

    // ==================== Disbursement Struct Tests ====================

    #[test]
    fn disbursement_deserialize_full() {
        let json = r#"{
            "id": "t1_dis_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "account": "t1_acc_12345678901234567890123",
            "payout": "t1_pyt_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 3,
            "code": "pending",
            "amount": 100000,
            "fee": 500,
            "net": 99500,
            "currency": "USD",
            "scheduled": "20240115",
            "processed": "20240116",
            "trace": "123456789012345",
            "name": "Weekly Payout",
            "description": "Automatic weekly disbursement",
            "returnCode": "R01",
            "returnReason": "Insufficient Funds",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-16 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert_eq!(disbursement.id.as_str(), "t1_dis_12345678901234567890123");
        assert_eq!(disbursement.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(disbursement.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(disbursement.fund.unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(disbursement.account.unwrap().as_str(), "t1_acc_12345678901234567890123");
        assert_eq!(disbursement.payout.unwrap().as_str(), "t1_pyt_12345678901234567890123");
        assert_eq!(disbursement.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(disbursement.status, Some(DisbursementStatus::Processed));
        assert_eq!(disbursement.code, Some(DisbursementCode::Pending));
        assert_eq!(disbursement.amount, Some(100000));
        assert_eq!(disbursement.fee, Some(500));
        assert_eq!(disbursement.net, Some(99500));
        assert_eq!(disbursement.currency, Some("USD".to_string()));
        assert_eq!(disbursement.scheduled.as_ref().unwrap().as_str(), "20240115");
        assert_eq!(disbursement.processed.as_ref().unwrap().as_str(), "20240116");
        assert_eq!(disbursement.trace, Some("123456789012345".to_string()));
        assert_eq!(disbursement.name, Some("Weekly Payout".to_string()));
        assert_eq!(disbursement.description, Some("Automatic weekly disbursement".to_string()));
        assert_eq!(disbursement.return_code, Some("R01".to_string()));
        assert_eq!(disbursement.return_reason, Some("Insufficient Funds".to_string()));
        assert!(!disbursement.inactive);
        assert!(disbursement.frozen);
    }

    #[test]
    fn disbursement_deserialize_minimal() {
        let json = r#"{
            "id": "t1_dis_12345678901234567890123"
        }"#;

        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert_eq!(disbursement.id.as_str(), "t1_dis_12345678901234567890123");
        assert!(disbursement.entity.is_none());
        assert!(disbursement.merchant.is_none());
        assert!(disbursement.status.is_none());
        assert!(disbursement.amount.is_none());
        assert!(!disbursement.inactive);
        assert!(!disbursement.frozen);
    }

    #[test]
    fn disbursement_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_dis_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert!(!disbursement.inactive);
        assert!(!disbursement.frozen);
    }

    #[test]
    fn disbursement_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_dis_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert!(disbursement.inactive);
        assert!(disbursement.frozen);
    }

    #[test]
    fn disbursement_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_dis_12345678901234567890123"}"#;
        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert!(!disbursement.inactive);
        assert!(!disbursement.frozen);
    }

    // ==================== NewDisbursement Tests ====================

    #[test]
    fn new_disbursement_serialize_full() {
        let new_disbursement = NewDisbursement {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            fund: Some("t1_fnd_12345678901234567890123".to_string()),
            account: Some("t1_acc_12345678901234567890123".to_string()),
            amount: 100000,
            currency: Some("USD".to_string()),
            scheduled: Some("20240115".to_string()),
            name: Some("On-demand Payout".to_string()),
            description: Some("Manual disbursement request".to_string()),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_disbursement).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"fund\":\"t1_fnd_12345678901234567890123\""));
        assert!(json.contains("\"account\":\"t1_acc_12345678901234567890123\""));
        assert!(json.contains("\"amount\":100000"));
        assert!(json.contains("\"currency\":\"USD\""));
        assert!(json.contains("\"scheduled\":\"20240115\""));
        assert!(json.contains("\"name\":\"On-demand Payout\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_disbursement_serialize_minimal() {
        let new_disbursement = NewDisbursement {
            amount: 50000,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_disbursement).unwrap();
        assert!(json.contains("\"amount\":50000"));
        // Optional fields should be omitted
        assert!(!json.contains("\"entity\""));
        assert!(!json.contains("\"merchant\""));
        assert!(!json.contains("\"fund\""));
        assert!(!json.contains("\"account\""));
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_disbursement_option_bool_to_int_true() {
        let new_disbursement = NewDisbursement {
            amount: 50000,
            inactive: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_disbursement).unwrap();
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn new_disbursement_option_bool_to_int_false() {
        let new_disbursement = NewDisbursement {
            amount: 50000,
            inactive: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_disbursement).unwrap();
        assert!(json.contains("\"inactive\":0"));
    }
}
