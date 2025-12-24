//! Batch types for the Payrix API.
//!
//! Batches represent groups of transactions that are settled together,
//! typically at the end of a business day.
//!
//! **OpenAPI schema:** `batchesResponse`

use payrix_macros::PayrixEntity;
use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, BatchStatus, PayrixId};

// =============================================================================
// BATCH ENUMS
// =============================================================================

/// Payment platform/processor values.
///
/// **OpenAPI schema:** `platformModel`
///
/// Valid values:
/// - `APPLE` - Apple Payment Processor
/// - `ELAVON` - Elavon processor
/// - `FIRSTDATA` - FirstData processor
/// - `GOOGLE` - Google Payment Processor
/// - `VANTIV` - WorldPay (Vantiv/Litle) eComm (VAP) processor
/// - `VCORE` - WorldPay (Vantiv) Core processor
/// - `TDBANKCA` - External funding with TD Bank Canada via Operating Account
/// - `WELLSACH` - Wells Fargo ACH processor
/// - `WELLSFARGO` - Wells Fargo Merchant Services processor
/// - `WFSINGLE` - WFSINGLE processor
/// - `WORLDPAY` - WORLDPAY processor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Platform {
    /// Apple Payment Processor.
    #[default]
    Apple,

    /// Elavon processor.
    Elavon,

    /// FirstData processor.
    #[serde(rename = "FIRSTDATA")]
    FirstData,

    /// Google Payment Processor.
    Google,

    /// WorldPay (Vantiv/Litle) eComm (VAP) processor.
    Vantiv,

    /// WorldPay (Vantiv) Core processor.
    #[serde(rename = "VCORE")]
    VCore,

    /// External funding with TD Bank Canada via Operating Account.
    #[serde(rename = "TDBANKCA")]
    TdBankCa,

    /// Wells Fargo ACH processor.
    #[serde(rename = "WELLSACH")]
    WellsAch,

    /// Wells Fargo Merchant Services processor.
    #[serde(rename = "WELLSFARGO")]
    WellsFargo,

    /// WFSINGLE processor.
    #[serde(rename = "WFSINGLE")]
    WfSingle,

    /// WORLDPAY processor.
    #[serde(rename = "WORLDPAY")]
    WorldPay,
}

// =============================================================================
// BATCH STRUCT
// =============================================================================

/// A Payrix batch.
///
/// Batches group transactions for settlement.
///
/// **OpenAPI schema:** `batchesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateBatch, update = UpdateBatch)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Batch {
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

    /// The identifier of the Merchant that is associated with this Batch.
    ///
    /// **OpenAPI type:** string (ref: batchesModelMerchant)
    #[payrix(create_only)]
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The date the batch was first opened.
    ///
    /// Format: date (YYYY-MM-DD)
    ///
    /// **OpenAPI type:** string (date)
    #[serde(default)]
    pub date: Option<String>,

    /// The date the batch was sent to the processor for processing.
    ///
    /// Format: date (YYYY-MM-DD)
    ///
    /// **OpenAPI type:** string (date)
    #[serde(default)]
    pub processing_date: Option<String>,

    /// Internal ID set for processing.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub processing_id: Option<String>,

    /// The platform used to process this resource.
    ///
    /// **OpenAPI type:** string (ref: platformModel)
    #[serde(default)]
    pub platform: Option<Platform>,

    /// The current status of this Batch.
    ///
    /// - `open` - This Batch can accept more transactions.
    /// - `closed` - This Batch is closed to new transactions and ready to be sent to processor.
    ///
    /// **OpenAPI type:** string (ref: batchStatus)
    #[serde(default)]
    pub status: Option<BatchStatus>,

    /// The reference code of the batch.
    ///
    /// This field is automatically generated and stored as a text string
    /// (0-50 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,

    /// The merchant's reference code of the batch.
    ///
    /// This field is stored as a text string (0-50 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub client_ref: Option<String>,

    /// The default batches close time.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}`)
    #[serde(default)]
    pub close_time: Option<String>,

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

    /// Batch references associated with this batch.
    ///
    /// **OpenAPI type:** array
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub batch_refs: Option<Vec<serde_json::Value>>,

    /// Transactions associated with this batch.
    ///
    /// **OpenAPI type:** array of txnsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub txns: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Platform Tests ====================

    #[test]
    fn platform_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&Platform::Apple).unwrap(), "\"APPLE\"");
        assert_eq!(serde_json::to_string(&Platform::Elavon).unwrap(), "\"ELAVON\"");
        assert_eq!(serde_json::to_string(&Platform::FirstData).unwrap(), "\"FIRSTDATA\"");
        assert_eq!(serde_json::to_string(&Platform::Google).unwrap(), "\"GOOGLE\"");
        assert_eq!(serde_json::to_string(&Platform::Vantiv).unwrap(), "\"VANTIV\"");
        assert_eq!(serde_json::to_string(&Platform::VCore).unwrap(), "\"VCORE\"");
        assert_eq!(serde_json::to_string(&Platform::TdBankCa).unwrap(), "\"TDBANKCA\"");
        assert_eq!(serde_json::to_string(&Platform::WellsAch).unwrap(), "\"WELLSACH\"");
        assert_eq!(serde_json::to_string(&Platform::WellsFargo).unwrap(), "\"WELLSFARGO\"");
        assert_eq!(serde_json::to_string(&Platform::WfSingle).unwrap(), "\"WFSINGLE\"");
        assert_eq!(serde_json::to_string(&Platform::WorldPay).unwrap(), "\"WORLDPAY\"");
    }

    #[test]
    fn platform_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<Platform>("\"APPLE\"").unwrap(), Platform::Apple);
        assert_eq!(serde_json::from_str::<Platform>("\"ELAVON\"").unwrap(), Platform::Elavon);
        assert_eq!(serde_json::from_str::<Platform>("\"FIRSTDATA\"").unwrap(), Platform::FirstData);
        assert_eq!(serde_json::from_str::<Platform>("\"GOOGLE\"").unwrap(), Platform::Google);
        assert_eq!(serde_json::from_str::<Platform>("\"VANTIV\"").unwrap(), Platform::Vantiv);
        assert_eq!(serde_json::from_str::<Platform>("\"VCORE\"").unwrap(), Platform::VCore);
        assert_eq!(serde_json::from_str::<Platform>("\"TDBANKCA\"").unwrap(), Platform::TdBankCa);
        assert_eq!(serde_json::from_str::<Platform>("\"WELLSACH\"").unwrap(), Platform::WellsAch);
        assert_eq!(serde_json::from_str::<Platform>("\"WELLSFARGO\"").unwrap(), Platform::WellsFargo);
        assert_eq!(serde_json::from_str::<Platform>("\"WFSINGLE\"").unwrap(), Platform::WfSingle);
        assert_eq!(serde_json::from_str::<Platform>("\"WORLDPAY\"").unwrap(), Platform::WorldPay);
    }

    #[test]
    fn platform_default() {
        assert_eq!(Platform::default(), Platform::Apple);
    }

    // ==================== Batch Struct Tests ====================

    #[test]
    fn batch_deserialize_full() {
        let json = r#"{
            "id": "t1_bat_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "merchant": "t1_mer_12345678901234567890123",
            "date": "2024-01-01",
            "processingDate": "2024-01-02",
            "processingId": "proc_123456",
            "platform": "VANTIV",
            "status": "open",
            "ref": "REF-12345",
            "clientRef": "CLIENT-REF-001",
            "closeTime": "2024-01-01 23:59:59",
            "inactive": 0,
            "frozen": 1
        }"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.id.as_str(), "t1_bat_12345678901234567890123");
        assert_eq!(batch.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(batch.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(batch.merchant.as_ref().map(|m| m.as_str()), Some("t1_mer_12345678901234567890123"));
        assert_eq!(batch.date, Some("2024-01-01".to_string()));
        assert_eq!(batch.processing_date, Some("2024-01-02".to_string()));
        assert_eq!(batch.processing_id, Some("proc_123456".to_string()));
        assert_eq!(batch.platform, Some(Platform::Vantiv));
        assert_eq!(batch.status, Some(BatchStatus::Open));
        assert_eq!(batch.reference, Some("REF-12345".to_string()));
        assert_eq!(batch.client_ref, Some("CLIENT-REF-001".to_string()));
        assert_eq!(batch.close_time, Some("2024-01-01 23:59:59".to_string()));
        assert!(!batch.inactive);
        assert!(batch.frozen);
    }

    #[test]
    fn batch_deserialize_minimal() {
        let json = r#"{"id": "t1_bat_12345678901234567890123"}"#;

        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.id.as_str(), "t1_bat_12345678901234567890123");
        assert!(batch.creator.is_none());
        assert!(batch.merchant.is_none());
        assert!(batch.date.is_none());
        assert!(batch.platform.is_none());
        assert!(batch.status.is_none());
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
    fn batch_status_closed() {
        let json = r#"{"id": "t1_bat_12345678901234567890123", "status": "closed"}"#;
        let batch: Batch = serde_json::from_str(json).unwrap();
        assert_eq!(batch.status, Some(BatchStatus::Closed));
    }

    #[test]
    fn batch_platform_variants() {
        let test_cases = [
            ("VANTIV", Platform::Vantiv),
            ("VCORE", Platform::VCore),
            ("WORLDPAY", Platform::WorldPay),
            ("WELLSFARGO", Platform::WellsFargo),
        ];

        for (platform_str, expected_platform) in test_cases {
            let json = format!(r#"{{"id": "t1_bat_12345678901234567890123", "platform": "{}"}}"#, platform_str);
            let batch: Batch = serde_json::from_str(&json).unwrap();
            assert_eq!(batch.platform, Some(expected_platform));
        }
    }
}
