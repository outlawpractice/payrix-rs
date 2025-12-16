//! Batch types for the Payrix API.
//!
//! Batches represent groups of transactions that are settled together,
//! typically at the end of a business day.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, BatchStatus, DateYmd, PayrixId};

/// A Payrix batch.
///
/// Batches group transactions for settlement. All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    /// Unique identifier (30 characters, e.g., "t1_bat_...")
    pub id: PayrixId,

    /// Entity ID that owns this batch
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Terminal ID (if applicable)
    #[serde(default)]
    pub terminal: Option<PayrixId>,

    /// Login ID that closed this batch
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Batch status
    #[serde(default)]
    pub status: Option<BatchStatus>,

    /// Batch number/sequence
    #[serde(default)]
    pub number: Option<i64>,

    /// Total number of transactions in batch
    #[serde(default)]
    pub count: Option<i32>,

    /// Total sales amount in cents
    #[serde(default)]
    pub sales_amount: Option<i64>,

    /// Number of sales transactions
    #[serde(default)]
    pub sales_count: Option<i32>,

    /// Total refund amount in cents
    #[serde(default)]
    pub refund_amount: Option<i64>,

    /// Number of refund transactions
    #[serde(default)]
    pub refund_count: Option<i32>,

    /// Net batch amount in cents (sales - refunds)
    #[serde(default)]
    pub net_amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Date batch was opened (YYYYMMDD format)
    #[serde(default)]
    pub opened: Option<DateYmd>,

    /// Date batch was closed (YYYYMMDD format)
    #[serde(default)]
    pub closed: Option<DateYmd>,

    /// Batch name/label
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

    // ==================== Batch Struct Tests ====================

    #[test]
    fn batch_deserialize_full() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "terminal": "t1_trm_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": "open",
            "number": 42,
            "count": 150,
            "salesAmount": 150000,
            "salesCount": 125,
            "refundAmount": 5000,
            "refundCount": 25,
            "netAmount": 145000,
            "currency": "USD",
            "opened": "20240101",
            "closed": "20240102",
            "name": "Daily Batch",
            "description": "End of day batch",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-02 23:59:59.999",
            "inactive": 0,
            "frozen": 1
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.id.as_str(), "t1_bat_12345678901234567890123");
        assert_eq!(batch.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(batch.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(batch.terminal.unwrap().as_str(), "t1_trm_12345678901234567890123");
        assert_eq!(batch.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(batch.status, Some(BatchStatus::Open));
        assert_eq!(batch.number, Some(42));
        assert_eq!(batch.count, Some(150));
        assert_eq!(batch.sales_amount, Some(150000));
        assert_eq!(batch.sales_count, Some(125));
        assert_eq!(batch.refund_amount, Some(5000));
        assert_eq!(batch.refund_count, Some(25));
        assert_eq!(batch.net_amount, Some(145000));
        assert_eq!(batch.currency, Some("USD".to_string()));
        assert_eq!(batch.opened.as_ref().unwrap().as_str(), "20240101");
        assert_eq!(batch.closed.as_ref().unwrap().as_str(), "20240102");
        assert_eq!(batch.name, Some("Daily Batch".to_string()));
        assert_eq!(batch.description, Some("End of day batch".to_string()));
        assert_eq!(batch.custom, Some("custom data".to_string()));
        assert_eq!(batch.created, Some("2024-01-01 00:00:00.000".to_string()));
        assert_eq!(batch.modified, Some("2024-01-02 23:59:59.999".to_string()));
        assert!(!batch.inactive);
        assert!(batch.frozen);
    }

    #[test]
    fn batch_deserialize_minimal() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123"
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.id.as_str(), "t1_bat_12345678901234567890123");
        assert!(batch.entity.is_none());
        assert!(batch.merchant.is_none());
        assert!(batch.terminal.is_none());
        assert!(batch.login.is_none());
        assert!(batch.status.is_none());
        assert!(batch.number.is_none());
        assert!(batch.count.is_none());
        assert!(batch.sales_amount.is_none());
        assert!(batch.sales_count.is_none());
        assert!(batch.refund_amount.is_none());
        assert!(batch.refund_count.is_none());
        assert!(batch.net_amount.is_none());
        assert!(batch.currency.is_none());
        assert!(batch.opened.is_none());
        assert!(batch.closed.is_none());
        assert!(batch.name.is_none());
        assert!(batch.description.is_none());
        assert!(batch.custom.is_none());
        assert!(batch.created.is_none());
        assert!(batch.modified.is_none());
        assert!(!batch.inactive);
        assert!(!batch.frozen);
    }

    #[test]
    fn batch_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_bat_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert!(!batch.inactive);
        assert!(!batch.frozen);
    }

    #[test]
    fn batch_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_bat_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert!(batch.inactive);
        assert!(batch.frozen);
    }

    #[test]
    fn batch_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_bat_12345678901234567890123"}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert!(!batch.inactive);
        assert!(!batch.frozen);
    }

    #[test]
    fn batch_status_open() {
        let json = r#"{"id": "t1_bat_12345678901234567890123", "status": "open"}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.status, Some(BatchStatus::Open));
    }

    #[test]
    fn batch_status_processed() {
        let json = r#"{"id": "t1_bat_12345678901234567890123", "status": "processed"}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.status, Some(BatchStatus::Processed));
    }

    #[test]
    fn batch_date_ymd_fields() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "opened": "20240615",
            "closed": "20240616"
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert!(batch.opened.is_some());
        assert!(batch.closed.is_some());
        assert_eq!(batch.opened.unwrap().as_str(), "20240615");
        assert_eq!(batch.closed.unwrap().as_str(), "20240616");
    }

    #[test]
    fn batch_amounts_calculation() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "salesAmount": 100000,
            "refundAmount": 10000,
            "netAmount": 90000
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.sales_amount, Some(100000));
        assert_eq!(batch.refund_amount, Some(10000));
        assert_eq!(batch.net_amount, Some(90000));

        // Verify that net amount equals sales - refunds
        let net = batch.sales_amount.unwrap() - batch.refund_amount.unwrap();
        assert_eq!(net, batch.net_amount.unwrap());
    }

    #[test]
    fn batch_transaction_counts() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "count": 100,
            "salesCount": 80,
            "refundCount": 20
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.count, Some(100));
        assert_eq!(batch.sales_count, Some(80));
        assert_eq!(batch.refund_count, Some(20));

        // Verify that total count equals sales + refunds
        let total = batch.sales_count.unwrap() + batch.refund_count.unwrap();
        assert_eq!(total, batch.count.unwrap());
    }

    #[test]
    fn batch_serialize_roundtrip() {
        let batch = Batch {
            id: "t1_bat_12345678901234567890123".parse().unwrap(),
            entity: Some("t1_ent_12345678901234567890123".parse().unwrap()),
            merchant: Some("t1_mer_12345678901234567890123".parse().unwrap()),
            terminal: Some("t1_trm_12345678901234567890123".parse().unwrap()),
            login: Some("t1_log_12345678901234567890123".parse().unwrap()),
            status: Some(BatchStatus::Open),
            number: Some(1),
            count: Some(50),
            sales_amount: Some(50000),
            sales_count: Some(45),
            refund_amount: Some(2500),
            refund_count: Some(5),
            net_amount: Some(47500),
            currency: Some("USD".to_string()),
            opened: Some("20240101".parse().unwrap()),
            closed: Some("20240102".parse().unwrap()),
            name: Some("Test Batch".to_string()),
            description: Some("Test".to_string()),
            custom: Some("custom".to_string()),
            created: Some("2024-01-01 00:00:00.000".to_string()),
            modified: Some("2024-01-02 00:00:00.000".to_string()),
            inactive: false,
            frozen: false,
        };

        let json = serde_json::to_string(&batch).unwrap();
        let deserialized: Batch = serde_json::from_str(&json).unwrap();
        assert_eq!(batch, deserialized);
    }

    #[test]
    fn batch_empty_batch() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "count": 0,
            "salesAmount": 0,
            "salesCount": 0,
            "refundAmount": 0,
            "refundCount": 0,
            "netAmount": 0
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.count, Some(0));
        assert_eq!(batch.sales_amount, Some(0));
        assert_eq!(batch.net_amount, Some(0));
    }
}
