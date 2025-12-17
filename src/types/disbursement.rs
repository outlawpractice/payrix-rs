//! Disbursement types for the Payrix API.
//!
//! Disbursements represent actual fund transfers to an entity's bank account.
//!
//! **OpenAPI schema:** `disbursementsResponse`

use serde::{Deserialize, Serialize};

use super::{DisbursementStatus, PayrixId};

// =============================================================================
// DISBURSEMENT ENUMS
// =============================================================================

/// Funding status for a disbursement.
///
/// Indicates if entries were processed for this Disbursement.
///
/// **OpenAPI schema:** `FundingStatus`
///
/// Valid values:
/// - `pending` - Pending entry creation and processing
/// - `processed` - Entry created and processed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FundingStatus {
    /// Pending entry creation and processing.
    #[default]
    Pending,

    /// Entry created and processed.
    Processed,
}

/// Disbursement entries status.
///
/// The current status of disbursementEntries creation.
///
/// **OpenAPI schema:** `DisbursementEntriesStatus`
///
/// Valid values:
/// - `pending` - Pending entry creation and processing
/// - `processing` - Entry is still processing
/// - `processed` - Entry created and processed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisbursementEntriesStatus {
    /// Pending entry creation and processing.
    #[default]
    Pending,

    /// Entry is still processing.
    Processing,

    /// Entry created and processed.
    Processed,
}

// =============================================================================
// DISBURSEMENT STRUCT
// =============================================================================

/// A Payrix disbursement.
///
/// Disbursements represent actual fund transfers to an entity's bank account.
///
/// **OpenAPI schema:** `disbursementsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Disbursement {
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

    /// The identifier of the Entity that owns this Disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The token of the accounts resource used for this Disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelAccount)
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// A reference to the actual account or card data used for this disbursement.
    ///
    /// If someone changes the details in their bank account within Payrix Pro,
    /// the account token will point to a new account but the payment will
    /// always point to the data used for this disbursement.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub payment: Option<PayrixId>,

    /// The identifier of the Payout that represents the schedule for this Disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelPayout)
    #[serde(default)]
    pub payout: Option<PayrixId>,

    /// The settlement record for this disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelSettlement)
    #[serde(default)]
    pub settlement: Option<PayrixId>,

    /// The identifier of the Statement being paid by this Disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelStatement)
    #[serde(default)]
    pub statement: Option<PayrixId>,

    /// A description of this Disbursement.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// A secondary descriptor for the ACH transaction sent to the receiving bank.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub secondary_descriptor: Option<String>,

    /// The total amount of this Disbursement in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub amount: Option<i64>,

    /// The amount that has been returned within the disbursement, in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub returned_amount: Option<i64>,

    /// The current status of this Disbursement.
    ///
    /// - `1` - Requested: The request for this Disbursement has been received.
    /// - `2` - Processing: This Disbursement is being processed to be paid out.
    /// - `3` - Processed: This Disbursement has been paid by ACH to the bank account.
    /// - `4` - Failed: A problem occurred and the payment processor has failed.
    /// - `5` - Denied: The disbursement was denied.
    /// - `6` - Returned: This Disbursement has been returned.
    ///
    /// **OpenAPI type:** integer (ref: disbursementStatus)
    #[serde(default)]
    pub status: Option<DisbursementStatus>,

    /// Indicates if entries were processed for this Disbursement.
    ///
    /// - `pending` - Pending entry creation and processing.
    /// - `processed` - Entry created and processed.
    ///
    /// **OpenAPI type:** string (ref: FundingStatus)
    #[serde(default)]
    pub funding_status: Option<FundingStatus>,

    /// A timestamp indicating when the Disbursement was processed.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}`)
    #[serde(default)]
    pub processed: Option<String>,

    /// The current status of disbursementEntries creation.
    ///
    /// - `pending` - Pending entry creation and processing.
    /// - `processing` - Entry is still processing.
    /// - `processed` - Entry created and processed.
    ///
    /// **OpenAPI type:** string (ref: DisbursementEntriesStatus)
    #[serde(default)]
    pub disbursement_entries_status: Option<DisbursementEntriesStatus>,

    /// The last negative Entry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastNegativeEntry)
    #[serde(default)]
    pub last_negative_entry: Option<PayrixId>,

    /// The last negative PendingEntry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastNegativePendingEntry)
    #[serde(default)]
    pub last_negative_pending_entry: Option<PayrixId>,

    /// The last positive ReserveEntry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastPositiveReserveEntry)
    #[serde(default)]
    pub last_positive_reserve_entry: Option<PayrixId>,

    /// The last positive Entry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastPositiveEntry)
    #[serde(default)]
    pub last_positive_entry: Option<PayrixId>,

    /// The last positive PendingEntry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastPositivePendingEntry)
    #[serde(default)]
    pub last_positive_pending_entry: Option<PayrixId>,

    /// The last negative ReserveEntry processed included in the disbursement.
    ///
    /// **OpenAPI type:** string (ref: disbursementsModelLastNegativeReserveEntry)
    #[serde(default)]
    pub last_negative_reserve_entry: Option<PayrixId>,

    /// The currency of this Disbursement.
    ///
    /// Example: `"USD"`
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The expiration date of the related debit account.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub expiration: Option<String>,

    /// Whether sameDay funding is enabled or disabled for this disbursement.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: disbursementsSameDay)
    #[serde(default)]
    pub same_day: Option<i32>,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Adjustments associated with this disbursement.
    ///
    /// This field is populated when expanding `adjustments` in API requests.
    ///
    /// **OpenAPI type:** array of adjustmentsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub adjustments: Option<Vec<serde_json::Value>>,

    /// Disbursement entries associated with this disbursement.
    ///
    /// This field is populated when expanding `disbursementEntries` in API requests.
    ///
    /// **OpenAPI type:** array of disbursementEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub disbursement_entries: Option<Vec<serde_json::Value>>,

    /// Disbursement results associated with this disbursement.
    ///
    /// This field is populated when expanding `disbursementResults` in API requests.
    ///
    /// **OpenAPI type:** array of disbursementResultsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub disbursement_results: Option<Vec<serde_json::Value>>,

    /// Entity returns associated with this disbursement.
    ///
    /// This field is populated when expanding `entityReturns` in API requests.
    ///
    /// **OpenAPI type:** array of entityReturnsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entity_returns: Option<Vec<serde_json::Value>>,

    /// Entries associated with this disbursement.
    ///
    /// This field is populated when expanding `entries` in API requests.
    ///
    /// **OpenAPI type:** array of entriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,

    /// Entry origins associated with this disbursement.
    ///
    /// This field is populated when expanding `entryOrigins` in API requests.
    ///
    /// **OpenAPI type:** array of entryOriginsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entry_origins: Option<Vec<serde_json::Value>>,

    /// Funding references associated with this disbursement.
    ///
    /// This field is populated when expanding `funding` in API requests.
    ///
    /// **OpenAPI type:** array of entityRefsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub funding: Option<Vec<serde_json::Value>>,

    /// Pending entries associated with this disbursement.
    ///
    /// This field is populated when expanding `pendingEntries` in API requests.
    ///
    /// **OpenAPI type:** array of pendingEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub pending_entries: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FundingStatus Tests ====================

    #[test]
    fn funding_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FundingStatus::Pending).unwrap(), "\"pending\"");
        assert_eq!(serde_json::to_string(&FundingStatus::Processed).unwrap(), "\"processed\"");
    }

    #[test]
    fn funding_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FundingStatus>("\"pending\"").unwrap(), FundingStatus::Pending);
        assert_eq!(serde_json::from_str::<FundingStatus>("\"processed\"").unwrap(), FundingStatus::Processed);
    }

    #[test]
    fn funding_status_default() {
        assert_eq!(FundingStatus::default(), FundingStatus::Pending);
    }

    // ==================== DisbursementEntriesStatus Tests ====================

    #[test]
    fn disbursement_entries_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&DisbursementEntriesStatus::Pending).unwrap(), "\"pending\"");
        assert_eq!(serde_json::to_string(&DisbursementEntriesStatus::Processing).unwrap(), "\"processing\"");
        assert_eq!(serde_json::to_string(&DisbursementEntriesStatus::Processed).unwrap(), "\"processed\"");
    }

    #[test]
    fn disbursement_entries_status_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<DisbursementEntriesStatus>("\"pending\"").unwrap(),
            DisbursementEntriesStatus::Pending
        );
        assert_eq!(
            serde_json::from_str::<DisbursementEntriesStatus>("\"processing\"").unwrap(),
            DisbursementEntriesStatus::Processing
        );
        assert_eq!(
            serde_json::from_str::<DisbursementEntriesStatus>("\"processed\"").unwrap(),
            DisbursementEntriesStatus::Processed
        );
    }

    #[test]
    fn disbursement_entries_status_default() {
        assert_eq!(DisbursementEntriesStatus::default(), DisbursementEntriesStatus::Pending);
    }

    // ==================== Disbursement Struct Tests ====================

    #[test]
    fn disbursement_deserialize_full() {
        let json = r#"{
            "id": "t1_dis_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-16 12:00:00.0000",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "account": "t1_acc_12345678901234567890123",
            "payment": "t1_pay_12345678901234567890123",
            "payout": "t1_pyt_12345678901234567890123",
            "settlement": "t1_stl_12345678901234567890123",
            "statement": "t1_sta_12345678901234567890123",
            "description": "Weekly payout disbursement",
            "secondaryDescriptor": "PAYRIX PAYOUT",
            "amount": 100000,
            "returnedAmount": 0,
            "status": 3,
            "fundingStatus": "processed",
            "processed": "2024-01-16 10:30:00",
            "disbursementEntriesStatus": "processed",
            "lastNegativeEntry": "t1_ent_neg12345678901234567890",
            "lastPositiveEntry": "t1_ent_pos12345678901234567890",
            "currency": "USD",
            "expiration": "2025-12",
            "sameDay": 0
        }"#;

        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert_eq!(disbursement.id.as_str(), "t1_dis_12345678901234567890123");
        assert_eq!(disbursement.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(disbursement.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(disbursement.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(disbursement.account.as_ref().map(|a| a.as_str()), Some("t1_acc_12345678901234567890123"));
        assert_eq!(disbursement.payment.as_ref().map(|p| p.as_str()), Some("t1_pay_12345678901234567890123"));
        assert_eq!(disbursement.payout.as_ref().map(|p| p.as_str()), Some("t1_pyt_12345678901234567890123"));
        assert_eq!(disbursement.settlement.as_ref().map(|s| s.as_str()), Some("t1_stl_12345678901234567890123"));
        assert_eq!(disbursement.statement.as_ref().map(|s| s.as_str()), Some("t1_sta_12345678901234567890123"));
        assert_eq!(disbursement.description, Some("Weekly payout disbursement".to_string()));
        assert_eq!(disbursement.secondary_descriptor, Some("PAYRIX PAYOUT".to_string()));
        assert_eq!(disbursement.amount, Some(100000));
        assert_eq!(disbursement.returned_amount, Some(0));
        assert_eq!(disbursement.status, Some(DisbursementStatus::Processed));
        assert_eq!(disbursement.funding_status, Some(FundingStatus::Processed));
        assert_eq!(disbursement.processed, Some("2024-01-16 10:30:00".to_string()));
        assert_eq!(disbursement.disbursement_entries_status, Some(DisbursementEntriesStatus::Processed));
        assert_eq!(disbursement.currency, Some("USD".to_string()));
        assert_eq!(disbursement.expiration, Some("2025-12".to_string()));
        assert_eq!(disbursement.same_day, Some(0));
    }

    #[test]
    fn disbursement_deserialize_minimal() {
        let json = r#"{"id": "t1_dis_12345678901234567890123"}"#;

        let disbursement: Disbursement = serde_json::from_str(json).unwrap();
        assert_eq!(disbursement.id.as_str(), "t1_dis_12345678901234567890123");
        assert!(disbursement.creator.is_none());
        assert!(disbursement.entity.is_none());
        assert!(disbursement.status.is_none());
        assert!(disbursement.amount.is_none());
    }

    #[test]
    fn disbursement_status_values() {
        // Test all disbursement status values from OpenAPI
        let json_requested = r#"{"id": "t1_dis_12345678901234567890123", "status": 1}"#;
        let json_processing = r#"{"id": "t1_dis_12345678901234567890123", "status": 2}"#;
        let json_processed = r#"{"id": "t1_dis_12345678901234567890123", "status": 3}"#;
        let json_failed = r#"{"id": "t1_dis_12345678901234567890123", "status": 4}"#;
        let json_denied = r#"{"id": "t1_dis_12345678901234567890123", "status": 5}"#;
        let json_returned = r#"{"id": "t1_dis_12345678901234567890123", "status": 6}"#;

        assert_eq!(
            serde_json::from_str::<Disbursement>(json_requested).unwrap().status,
            Some(DisbursementStatus::Requested)
        );
        assert_eq!(
            serde_json::from_str::<Disbursement>(json_processing).unwrap().status,
            Some(DisbursementStatus::Processing)
        );
        assert_eq!(
            serde_json::from_str::<Disbursement>(json_processed).unwrap().status,
            Some(DisbursementStatus::Processed)
        );
        assert_eq!(
            serde_json::from_str::<Disbursement>(json_failed).unwrap().status,
            Some(DisbursementStatus::Failed)
        );
        assert_eq!(
            serde_json::from_str::<Disbursement>(json_denied).unwrap().status,
            Some(DisbursementStatus::Denied)
        );
        assert_eq!(
            serde_json::from_str::<Disbursement>(json_returned).unwrap().status,
            Some(DisbursementStatus::Returned)
        );
    }

    #[test]
    fn disbursement_same_day_flag() {
        let json_disabled = r#"{"id": "t1_dis_12345678901234567890123", "sameDay": 0}"#;
        let json_enabled = r#"{"id": "t1_dis_12345678901234567890123", "sameDay": 1}"#;

        assert_eq!(serde_json::from_str::<Disbursement>(json_disabled).unwrap().same_day, Some(0));
        assert_eq!(serde_json::from_str::<Disbursement>(json_enabled).unwrap().same_day, Some(1));
    }
}
