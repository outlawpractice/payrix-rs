//! Entry types for the Payrix API.
//!
//! Entries represent ledger records for financial transactions.
//!
//! **OpenAPI schema:** `entriesResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::PayrixId;

// =============================================================================
// ENTRY ENUMS
// =============================================================================

/// Event type values indicating what triggered the entry.
///
/// **OpenAPI schema:** `entryEvent`
///
/// Note: OpenAPI defines many additional event values (up to 608). This enum includes
/// the most commonly used values. Unknown values will fail deserialization per strict
/// OpenAPI alignment policy.
///
/// Common valid values:
/// - `1` - Days (daily event)
/// - `2` - Weeks (weekly event)
/// - `3` - Months (monthly event)
/// - `4` - Years (annual event)
/// - `5` - Single (one-off event)
/// - `6` - Auth (transaction authorization)
/// - `7` - Capture (transaction capture)
/// - `8` - Refund (refund processed)
/// - `9` - Board (merchant boarded)
/// - `10` - Payout (payout processed)
/// - `11` - Chargeback (chargeback occurred)
/// - `12` - Overdraft (overdraft charge)
/// - `13` - Interchange (interchange fee)
/// - `14` - Processor (processor fee)
/// - `15` - AchFail (ACH failure)
/// - `16` - Account (account verification)
/// - `17` - Sift (fraud score)
/// - `18` - Adjustment (balance adjustment)
/// - ... (see OpenAPI spec for full list up to value 608)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EventType {
    /// Entry triggered by daily event.
    #[default]
    Days = 1,
    /// Entry triggered by weekly event.
    Weeks = 2,
    /// Entry triggered by monthly event.
    Months = 3,
    /// Entry triggered by annual event.
    Years = 4,
    /// One-off event.
    Single = 5,
    /// Transaction authorization.
    Authorization = 6,
    /// Transaction capture.
    Capture = 7,
    /// Refund processed.
    Refund = 8,
    /// Merchant boarded.
    Boarded = 9,
    /// Payout processed.
    Payout = 10,
    /// Chargeback occurred.
    Chargeback = 11,
    /// Overdraft charge.
    Overdraft = 12,
    /// Interchange fee assessed.
    Interchange = 13,
    /// Processor fee.
    Processor = 14,
    /// ACH failure.
    AchFail = 15,
    /// Bank account verification.
    AccountVerification = 16,
    /// Fraud score (SIFT).
    Sift = 17,
    /// Adjustment.
    Adjustment = 18,
    /// Retrieval request.
    Retrieval = 19,
    /// Arbitration chargeback.
    Arbitration = 20,
    /// eCheck sale.
    ECheckSale = 21,
    /// eCheck refund.
    ECheckRefund = 22,
    /// eCheck return.
    ECheckReturn = 23,
    /// Transaction settlement.
    Settlement = 24,
    /// Misuse of authorization.
    Misuse = 25,
    /// Profit sharing.
    ProfitShare = 26,
    /// Unauthorized entry.
    Unauthorized = 27,
    /// ACH notification of change.
    AchNotificationOfChange = 28,
    /// eCheck notification of change.
    ECheckNotificationOfChange = 29,
    /// eCheck failure.
    ECheckFail = 30,
    /// eCheck non-sufficient funds.
    ECheckNonSufficientFunds = 31,
    /// Currency conversion.
    CurrencyConversion = 32,
    /// Terminal transaction.
    TerminalTransaction = 33,
    /// Payout reversed.
    ReversePayout = 34,
    /// Partial payout reversal.
    PartialReversePayout = 35,
    /// Reserve entry created.
    ReserveEntry = 36,
    /// Reserve entry released.
    ReserveEntryRelease = 37,
    /// Pending entry.
    PendingEntry = 38,
    /// Pending entry paid.
    PendingPaid = 39,
    /// Remainder (non-disbursed funds).
    Remainder = 40,
    /// Remainder used.
    RemainderUsed = 41,
    /// Pending refund cancelled.
    PendingRefundCancelled = 42,
    /// Payment check (Account Updater).
    PaymentCheck = 43,
    /// Payment update (Account Updater).
    PaymentUpdate = 44,
    /// Payment group check (Account Updater).
    PaymentGroupCheck = 45,
    /// Payment group update (Account Updater).
    PaymentGroupUpdate = 46,
    /// Entry refund.
    EntryRefund = 47,
    /// Statement payment.
    Statement = 51,
    /// Merchant creation fee.
    MerchantCreation = 52,
    /// Real-time business search.
    RealtimeBusinessSearch = 53,
    /// Real-time member search.
    RealtimeMemberSearch = 54,
    /// Mastercard match.
    MasterCardMatch = 55,
    /// Business instant ID.
    BusinessInstantId = 56,
    /// Consumer instant ID.
    ConsumerInstantId = 57,
    /// Threat metrix.
    ThreatMetrix = 58,
    /// Legit script register.
    LegitScriptRegister = 59,
    /// Equifax consumer report.
    EquifaxConsumerReport = 60,
    /// Guidestar.
    GuideStar = 61,
    /// Internal Decision V2.
    PayloadAttribute = 62,
    /// TIN check.
    TinCheck = 63,
    /// Equifax commercial report.
    EquifaxCommercialReport = 64,
    /// Legit script check merchant.
    LegitScriptCheckMerchant = 65,
    /// Plaid.
    Plaid = 66,
    /// Statement reversal.
    StatementReversal = 67,
    /// GIACT eCheck verification.
    GiactEcheck = 68,
    /// GIACT bank account verification.
    GiactBankAccount = 69,
    /// Board decision fee.
    BoardDecision = 70,
    /// Transaction risk decision.
    TxnRiskDecision = 71,
    /// FANF external fees.
    Fanf = 72,
    /// MC Location external fees.
    McLocation = 73,
    /// Visa Integrity external fees.
    VisaIntegrity = 74,
    /// Safer Payments Basic external fees.
    SaferPaymentsBasic = 75,
    /// Safer Payments Managed external fees.
    SaferPaymentsManaged = 76,
    /// Safer Payments PCI non-validation external fees.
    SaferPaymentsPciNonvalidation = 77,
    /// Omnitokens volume external fees.
    OmnitokensVolume = 78,
    /// Payout return.
    PayoutReturn = 79,
    /// Payout partial return.
    PayoutPartialReturn = 80,
    /// Rev share.
    RevShare = 81,
    /// Card settlement.
    CardSettlement = 82,
    /// ECheck settlement.
    EcheckSettlement = 83,
    /// Rev share card.
    RevShareCard = 84,
    /// Rev share eCheck.
    RevShareEcheck = 85,
    /// Rev share DBM.
    RevShareDbm = 86,
    /// Prearbitration chargeback.
    Prearbitration = 87,
    /// Plaid identity match.
    PlaidIdentityMatch = 88,
    /// Transaction Plaid identity match.
    TxnPlaidIdentityMatch = 89,
    /// Plaid get identity.
    PlaidGetIdentity = 90,
    /// Transaction Plaid get identity.
    TxnPlaidGetIdentity = 91,
    /// Plaid get auth.
    PlaidGetAuth = 92,
    /// Transaction Plaid get auth.
    TxnPlaidGetAuth = 93,
    /// Chargeback reversal.
    Reversal = 94,
    /// Chargeback representment.
    Representment = 95,
    /// Omnitokens monthly fee.
    OmnitokensMonthly = 96,
    /// Interchange retain pass on refund.
    IcRetainPassthruRefund = 101,
}

// =============================================================================
// ENTRY STRUCT
// =============================================================================

/// A Payrix ledger entry.
///
/// Entries track all financial movements in the Payrix system.
///
/// **OpenAPI schema:** `entriesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Entry {
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

    /// The identifier of the Entity that owns this Entry.
    ///
    /// **OpenAPI type:** string (ref: entriesModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Entity that this Entry refers to.
    ///
    /// This is the owner of the record that triggered the charge.
    ///
    /// **OpenAPI type:** string (ref: entriesModelOnentity)
    #[serde(default)]
    pub onentity: Option<PayrixId>,

    /// The identifier of the Entity that the charge or activity is for.
    ///
    /// If the activity involves two parties with one paying a charge,
    /// this stores the identifier of the paying Entity.
    ///
    /// **OpenAPI type:** string (ref: entriesModelFromentity)
    #[serde(default)]
    pub fromentity: Option<PayrixId>,

    /// The identifier of the Entry that this Entry was created based on.
    ///
    /// **OpenAPI type:** string (ref: entriesModelOpposingEntry)
    #[serde(default)]
    pub opposing_entry: Option<PayrixId>,

    /// The identifier of the Fund that this Entry refers to.
    ///
    /// **OpenAPI type:** string (ref: entriesModelFund)
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// The identifier of the Fee if this Entry is for a fee charge.
    ///
    /// **OpenAPI type:** string (ref: entriesModelFee)
    #[serde(default)]
    pub fee: Option<PayrixId>,

    /// The identifier of the Disbursement if this Entry is for a disbursement.
    ///
    /// **OpenAPI type:** string (ref: entriesModelDisbursement)
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// The identifier of the Refund if this Entry is for a refund.
    ///
    /// **OpenAPI type:** string (ref: entriesModelRefund)
    #[serde(default)]
    pub refund: Option<PayrixId>,

    /// The identifier of the Settlement if this Entry is for a settlement.
    ///
    /// **OpenAPI type:** string (ref: entriesModelSettlement)
    #[serde(default)]
    pub settlement: Option<PayrixId>,

    /// The identifier of the Transaction if this Entry is for a transaction.
    ///
    /// **OpenAPI type:** string (ref: entriesModelTxn)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// The identifier of the Chargeback if this Entry is for a chargeback.
    ///
    /// **OpenAPI type:** string (ref: entriesModelChargeback)
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// The adjustment ID from which the funding activity was triggered.
    ///
    /// **OpenAPI type:** string (ref: entriesModelAdjustment)
    #[serde(default)]
    pub adjustment: Option<PayrixId>,

    /// The identifier of the ProfitShare if this Entry is for profit sharing.
    ///
    /// **OpenAPI type:** string (ref: entriesModelProfitShare)
    #[serde(default)]
    pub profit_share: Option<PayrixId>,

    /// The identifier of the Statement if this Entry is for a statement.
    ///
    /// **OpenAPI type:** string (ref: entriesModelStatement)
    #[serde(default)]
    pub statement: Option<PayrixId>,

    /// The type of event that triggered this Entry.
    ///
    /// See EventType enum for valid values.
    ///
    /// **OpenAPI type:** integer (ref: entryEvent)
    #[serde(default)]
    pub event: Option<EventType>,

    /// The identifier of the record associated with this Entry.
    ///
    /// **OpenAPI type:** string (ref: entriesModelEventId)
    #[serde(default)]
    pub event_id: Option<PayrixId>,

    /// The identifier of the RevShareStatement if this Entry is for revenue share.
    ///
    /// **OpenAPI type:** string (ref: entriesModelRevShareStatement)
    #[serde(default)]
    pub rev_share_statement: Option<PayrixId>,

    /// ID of the original event.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub original_event_id: Option<String>,

    /// The event that originally triggered the entry.
    ///
    /// **OpenAPI type:** integer (ref: originalEntryEvent)
    #[serde(default)]
    pub original_event: Option<EventType>,

    /// A description of this Entry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// Original Currency of the entry.
    ///
    /// Set automatically based on the settled total. Default is 'USD'.
    ///
    /// **OpenAPI type:** string (ref: OriginalCurrency)
    #[serde(default)]
    pub original_currency: Option<String>,

    /// Currency conversion rate (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub currency_rate: Option<f64>,

    /// The amount involved in this Entry in cents (up to three decimal points).
    ///
    /// Refers to the amount charged, transferred, or disbursed.
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    /// Integer boolean indicating whether this entry is pending.
    ///
    /// - `0` - Not pending
    /// - `1` - Pending
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub pending: Option<i32>,

    /// Integer boolean indicating whether the funding activity was triggered by a fee.
    ///
    /// - `0` - Not a fee
    /// - `1` - Is a fee
    ///
    /// **OpenAPI type:** integer (ref: entriesIsFee)
    #[serde(default)]
    pub is_fee: Option<i32>,

    /// Summary record for archived entries.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub archive_summary: Option<i32>,

    /// The ID of the pending entry associated with this entry.
    ///
    /// **OpenAPI type:** string (ref: entriesModelPendingEntry)
    #[serde(default)]
    pub pending_entry: Option<PayrixId>,

    /// The date/time when the associated pendingEntry was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}`)
    #[serde(default)]
    pub pending_entry_created: Option<String>,

    /// Integer boolean indicating whether this is an External Fee Entry (EFE).
    ///
    /// - `0` - Not EFE
    /// - `1` - Is EFE
    ///
    /// **OpenAPI type:** integer (ref: IsEFE)
    #[serde(default, rename = "isEFE")]
    pub is_efe: Option<i32>,

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Cancellation entries associated with this entry.
    ///
    /// **OpenAPI type:** array of entryOriginsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub cancellation: Option<Vec<serde_json::Value>>,

    /// Disbursement entries associated with this entry.
    ///
    /// **OpenAPI type:** array of disbursementEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub disbursement_entries: Option<Vec<serde_json::Value>>,

    /// Entry origins associated with this entry.
    ///
    /// **OpenAPI type:** array of entryOriginsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entry_origins: Option<Vec<serde_json::Value>>,

    /// Last negative disbursement of this entry.
    ///
    /// **OpenAPI type:** disbursementsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub last_negative_of_disbursement: Option<serde_json::Value>,

    /// Last positive disbursement of this entry.
    ///
    /// **OpenAPI type:** disbursementsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub last_positive_of_disbursement: Option<serde_json::Value>,

    /// Opposing entries.
    ///
    /// **OpenAPI type:** entriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub opposing_entries: Option<serde_json::Value>,

    /// Profit share results associated with this entry.
    ///
    /// **OpenAPI type:** array of profitShareResultsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub profit_share_results: Option<Vec<serde_json::Value>>,

    /// Refunds associated with this entry.
    ///
    /// **OpenAPI type:** array of refundsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub refunds: Option<Vec<serde_json::Value>>,

    /// Reserve entries associated with this entry.
    ///
    /// **OpenAPI type:** array of reserveEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserve_entries: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// PENDING ENTRY STRUCT
// =============================================================================

/// A Payrix pending entry.
///
/// Pending entries represent ledger entries that haven't been finalized yet.
///
/// **OpenAPI schema:** `pendingEntriesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct PendingEntry {
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

    /// The identifier of the Entity that owns this PendingEntry.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Entity that this PendingEntry refers to.
    ///
    /// This is the owner of the record that triggered the charge.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelOnentity)
    #[serde(default)]
    pub onentity: Option<PayrixId>,

    /// The identifier of the Entity that the charge or activity is for.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelFromentity)
    #[serde(default)]
    pub fromentity: Option<PayrixId>,

    /// The identifier of the Fund that this PendingEntry refers to.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelFund)
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// The identifier of the Entry that this entry was created based on.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub opposing_entry: Option<PayrixId>,

    /// The entry ID associated with this pendingEntry.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelEntry)
    #[serde(default)]
    pub entry: Option<PayrixId>,

    /// The adjustment ID from which the funding activity was triggered.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelAdjustment)
    #[serde(default)]
    pub adjustment: Option<PayrixId>,

    /// The identifier of the Chargeback if this PendingEntry is for a chargeback.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelChargeback)
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// The identifier of the Disbursement if this PendingEntry is for a disbursement.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelDisbursement)
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// The identifier of the Fee if this PendingEntry is for a fee charge.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelFee)
    #[serde(default)]
    pub fee: Option<PayrixId>,

    /// The identifier of the Refund if this PendingEntry is for a refund.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelRefund)
    #[serde(default)]
    pub refund: Option<PayrixId>,

    /// The identifier of the Transaction if this PendingEntry is for a transaction.
    ///
    /// **OpenAPI type:** string (ref: pendingEntriesModelTxn)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// The type of event that triggered this PendingEntry.
    ///
    /// See EventType enum for valid values.
    ///
    /// **OpenAPI type:** integer (ref: pendingEntryEvent)
    #[serde(default)]
    pub event: Option<EventType>,

    /// The amount involved in this PendingEntry in cents.
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub amount: Option<f64>,

    /// A description of this PendingEntry.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== EventType Tests ====================

    #[test]
    fn event_type_default() {
        assert_eq!(EventType::default(), EventType::Days);
    }

    #[test]
    fn event_type_common_variants_serialize() {
        let test_cases = [
            (EventType::Days, "1"),
            (EventType::Weeks, "2"),
            (EventType::Months, "3"),
            (EventType::Years, "4"),
            (EventType::Single, "5"),
            (EventType::Authorization, "6"),
            (EventType::Capture, "7"),
            (EventType::Refund, "8"),
            (EventType::Boarded, "9"),
            (EventType::Payout, "10"),
            (EventType::Chargeback, "11"),
            (EventType::Settlement, "24"),
            (EventType::RevShare, "81"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn event_type_common_variants_deserialize() {
        let test_cases = [
            ("1", EventType::Days),
            ("2", EventType::Weeks),
            ("3", EventType::Months),
            ("4", EventType::Years),
            ("5", EventType::Single),
            ("6", EventType::Authorization),
            ("7", EventType::Capture),
            ("8", EventType::Refund),
            ("9", EventType::Boarded),
            ("10", EventType::Payout),
            ("11", EventType::Chargeback),
            ("24", EventType::Settlement),
            ("81", EventType::RevShare),
        ];

        for (json, expected_variant) in test_cases {
            let variant: EventType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    // ==================== Entry Struct Tests ====================

    #[test]
    fn entry_deserialize_full() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "created": "2024-01-01 10:00:00.0000",
            "modified": "2024-01-01 15:30:00.0000",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ety_12345678901234567890123",
            "onentity": "t1_ety_12345678901234567890124",
            "fromentity": "t1_ety_12345678901234567890125",
            "opposingEntry": "t1_ent_12345678901234567890124",
            "fund": "t1_fnd_12345678901234567890123",
            "fee": "t1_fee_12345678901234567890123",
            "disbursement": "t1_dis_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "event": 7,
            "description": "Capture transaction",
            "originalCurrency": "USD",
            "currencyRate": 1.0,
            "amount": 10000.0,
            "pending": 0,
            "isFee": 0,
            "isEFE": 0
        }"#;

        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entry.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(entry.entity.as_ref().map(|e| e.as_str()), Some("t1_ety_12345678901234567890123"));
        assert_eq!(entry.onentity.as_ref().map(|o| o.as_str()), Some("t1_ety_12345678901234567890124"));
        assert_eq!(entry.fromentity.as_ref().map(|f| f.as_str()), Some("t1_ety_12345678901234567890125"));
        assert_eq!(entry.event, Some(EventType::Capture));
        assert_eq!(entry.amount, Some(10000.0));
        assert_eq!(entry.pending, Some(0));
        assert_eq!(entry.is_fee, Some(0));
        assert_eq!(entry.is_efe, Some(0));
    }

    #[test]
    fn entry_deserialize_minimal() {
        let json = r#"{"id": "t1_ent_12345678901234567890123"}"#;

        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_ent_12345678901234567890123");
        assert!(entry.creator.is_none());
        assert!(entry.entity.is_none());
        assert!(entry.event.is_none());
        assert!(entry.amount.is_none());
    }

    #[test]
    fn entry_event_values() {
        let test_cases = [
            (1, EventType::Days),
            (7, EventType::Capture),
            (11, EventType::Chargeback),
            (24, EventType::Settlement),
        ];

        for (event_val, expected_type) in test_cases {
            let json = format!(r#"{{"id": "t1_ent_12345678901234567890123", "event": {}}}"#, event_val);
            let entry: Entry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.event, Some(expected_type));
        }
    }

    // ==================== PendingEntry Struct Tests ====================

    #[test]
    fn pending_entry_deserialize_full() {
        let json = r#"{
            "id": "t1_pen_12345678901234567890123",
            "created": "2024-01-01 10:00:00.0000",
            "modified": "2024-01-01 15:30:00.0000",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ety_12345678901234567890123",
            "onentity": "t1_ety_12345678901234567890124",
            "fund": "t1_fnd_12345678901234567890123",
            "entry": "t1_ent_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "event": 6,
            "amount": 5000.0,
            "description": "Pending authorization"
        }"#;

        let pending: PendingEntry = serde_json::from_str(json).unwrap();
        assert_eq!(pending.id.as_str(), "t1_pen_12345678901234567890123");
        assert_eq!(pending.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(pending.entity.as_ref().map(|e| e.as_str()), Some("t1_ety_12345678901234567890123"));
        assert_eq!(pending.event, Some(EventType::Authorization));
        assert_eq!(pending.amount, Some(5000.0));
        assert_eq!(pending.description, Some("Pending authorization".to_string()));
    }

    #[test]
    fn pending_entry_deserialize_minimal() {
        let json = r#"{"id": "t1_pen_12345678901234567890123"}"#;

        let pending: PendingEntry = serde_json::from_str(json).unwrap();
        assert_eq!(pending.id.as_str(), "t1_pen_12345678901234567890123");
        assert!(pending.creator.is_none());
        assert!(pending.entity.is_none());
        assert!(pending.event.is_none());
    }
}
