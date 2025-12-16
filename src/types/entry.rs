//! Entry types for the Payrix API.
//!
//! Entries represent ledger records for financial transactions.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, DateYmd, PayrixId};

/// Entry type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EntryType {
    /// Transaction credit
    #[default]
    TransactionCredit = 1,
    /// Transaction debit
    TransactionDebit = 2,
    /// Fee
    Fee = 3,
    /// Adjustment
    Adjustment = 4,
    /// Disbursement
    Disbursement = 5,
    /// Chargeback
    Chargeback = 6,
    /// Refund
    Refund = 7,
    /// Reserve hold
    ReserveHold = 8,
    /// Reserve release
    ReserveRelease = 9,
}

/// Event type values (what triggered the entry).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EventType {
    /// Entry triggered by daily event
    #[default]
    Days = 1,
    /// Entry triggered by weekly event
    Weeks = 2,
    /// Entry triggered by monthly event
    Months = 3,
    /// Entry triggered by annual event
    Years = 4,
    /// One-off event
    Single = 5,
    /// Transaction authorization
    Authorization = 6,
    /// Transaction capture
    Capture = 7,
    /// Refund processed
    Refund = 8,
    /// Merchant boarded
    Boarded = 9,
    /// Payout processed
    Payout = 10,
    /// Chargeback occurred
    Chargeback = 11,
    /// Overdraft charge
    Overdraft = 12,
    /// Interchange fee assessed
    Interchange = 13,
    /// Processor fee
    Processor = 14,
    /// ACH failure
    AchFail = 15,
    /// Bank account verification
    AccountVerification = 16,
    /// Fraud score (SIFT)
    Sift = 17,
    /// Adjustment
    Adjustment = 18,
    /// Retrieval request
    Retrieval = 19,
    /// Arbitration chargeback
    Arbitration = 20,
    /// eCheck sale
    ECheckSale = 21,
    /// eCheck refund
    ECheckRefund = 22,
    /// eCheck return
    ECheckReturn = 23,
    /// Transaction settlement
    Settlement = 24,
    /// Misuse of authorization
    Misuse = 25,
    /// Profit sharing
    ProfitShare = 26,
    /// Unauthorized entry
    Unauthorized = 27,
    /// ACH notification of change
    AchNotificationOfChange = 28,
    /// eCheck notification of change
    ECheckNotificationOfChange = 29,
    /// eCheck failure
    ECheckFail = 30,
    /// eCheck non-sufficient funds
    ECheckNonSufficientFunds = 31,
    /// Currency conversion
    CurrencyConversion = 32,
    /// Terminal transaction
    TerminalTransaction = 33,
    /// Payout reversed
    ReversePayout = 34,
    /// Partial payout reversal
    PartialReversePayout = 35,
    /// Reserve entry created
    ReserveEntry = 36,
    /// Reserve entry released
    ReserveEntryRelease = 37,
    /// Pending entry
    PendingEntry = 38,
    /// Pending entry paid
    PendingPaid = 39,
    /// Remainder (non-disbursed funds)
    Remainder = 40,
    /// Remainder used
    RemainderUsed = 41,
    /// Pending refund cancelled
    PendingRefundCancelled = 42,
    /// Payment check (Account Updater)
    PaymentCheck = 43,
    /// Payment update (Account Updater)
    PaymentUpdate = 44,
    /// Payment group check (Account Updater)
    PaymentGroupCheck = 45,
    /// Payment group update (Account Updater)
    PaymentGroupUpdate = 46,
    /// Entry refund
    EntryRefund = 47,
}

/// A Payrix ledger entry.
///
/// Entries track all financial movements in the Payrix system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Unique identifier (30 characters, e.g., "t1_ent_...")
    pub id: PayrixId,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Transaction ID (if related to a transaction)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Fund ID this entry affects
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Batch ID (if part of a batch)
    #[serde(default)]
    pub batch: Option<PayrixId>,

    /// Disbursement ID (if part of a disbursement)
    #[serde(default)]
    pub disbursement: Option<PayrixId>,

    /// Entry type
    #[serde(default, rename = "type")]
    pub entry_type: Option<EntryType>,

    /// Entry category
    #[serde(default)]
    pub category: Option<String>,

    /// Debit amount in cents
    #[serde(default)]
    pub debit: Option<i64>,

    /// Credit amount in cents
    #[serde(default)]
    pub credit: Option<i64>,

    /// Net amount in cents (credit - debit)
    #[serde(default)]
    pub net: Option<i64>,

    /// Running balance in cents after this entry
    #[serde(default)]
    pub balance: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Entry status
    #[serde(default)]
    pub status: Option<i32>,

    /// Effective date (YYYYMMDD format)
    #[serde(default)]
    pub effective: Option<DateYmd>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Reference/external ID
    #[serde(default)]
    pub reference: Option<String>,

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

/// A Payrix pending entry.
///
/// Pending entries represent ledger entries that haven't been finalized yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct PendingEntry {
    /// Unique identifier (30 characters, e.g., "t1_pen_...")
    pub id: PayrixId,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Transaction ID (if related to a transaction)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Fund ID this entry will affect
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Batch ID (if part of a batch)
    #[serde(default)]
    pub batch: Option<PayrixId>,

    /// Entry type
    #[serde(default, rename = "type")]
    pub entry_type: Option<EntryType>,

    /// Entry category
    #[serde(default)]
    pub category: Option<String>,

    /// Debit amount in cents
    #[serde(default)]
    pub debit: Option<i64>,

    /// Credit amount in cents
    #[serde(default)]
    pub credit: Option<i64>,

    /// Net amount in cents (credit - debit)
    #[serde(default)]
    pub net: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Entry status
    #[serde(default)]
    pub status: Option<i32>,

    /// Expected effective date (YYYYMMDD format)
    #[serde(default)]
    pub effective: Option<DateYmd>,

    /// Reason the entry is pending
    #[serde(default)]
    pub pending_reason: Option<String>,

    /// Description/memo
    #[serde(default)]
    pub description: Option<String>,

    /// Reference/external ID
    #[serde(default)]
    pub reference: Option<String>,

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
    use serde_json;

    // ==================== Enum Tests ====================

    #[test]
    fn entry_type_default() {
        assert_eq!(EntryType::default(), EntryType::TransactionCredit);
    }

    #[test]
    fn entry_type_all_variants_serialize() {
        let test_cases = [
            (EntryType::TransactionCredit, "1"),
            (EntryType::TransactionDebit, "2"),
            (EntryType::Fee, "3"),
            (EntryType::Adjustment, "4"),
            (EntryType::Disbursement, "5"),
            (EntryType::Chargeback, "6"),
            (EntryType::Refund, "7"),
            (EntryType::ReserveHold, "8"),
            (EntryType::ReserveRelease, "9"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn entry_type_all_variants_deserialize() {
        let test_cases = [
            ("1", EntryType::TransactionCredit),
            ("2", EntryType::TransactionDebit),
            ("3", EntryType::Fee),
            ("4", EntryType::Adjustment),
            ("5", EntryType::Disbursement),
            ("6", EntryType::Chargeback),
            ("7", EntryType::Refund),
            ("8", EntryType::ReserveHold),
            ("9", EntryType::ReserveRelease),
        ];

        for (json, expected_variant) in test_cases {
            let variant: EntryType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn event_type_default() {
        assert_eq!(EventType::default(), EventType::Days);
    }

    #[test]
    fn event_type_all_variants_serialize() {
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
            (EventType::Overdraft, "12"),
            (EventType::Interchange, "13"),
            (EventType::Processor, "14"),
            (EventType::AchFail, "15"),
            (EventType::AccountVerification, "16"),
            (EventType::Sift, "17"),
            (EventType::Adjustment, "18"),
            (EventType::Retrieval, "19"),
            (EventType::Arbitration, "20"),
            (EventType::ECheckSale, "21"),
            (EventType::ECheckRefund, "22"),
            (EventType::ECheckReturn, "23"),
            (EventType::Settlement, "24"),
            (EventType::Misuse, "25"),
            (EventType::ProfitShare, "26"),
            (EventType::Unauthorized, "27"),
            (EventType::AchNotificationOfChange, "28"),
            (EventType::ECheckNotificationOfChange, "29"),
            (EventType::ECheckFail, "30"),
            (EventType::ECheckNonSufficientFunds, "31"),
            (EventType::CurrencyConversion, "32"),
            (EventType::TerminalTransaction, "33"),
            (EventType::ReversePayout, "34"),
            (EventType::PartialReversePayout, "35"),
            (EventType::ReserveEntry, "36"),
            (EventType::ReserveEntryRelease, "37"),
            (EventType::PendingEntry, "38"),
            (EventType::PendingPaid, "39"),
            (EventType::Remainder, "40"),
            (EventType::RemainderUsed, "41"),
            (EventType::PendingRefundCancelled, "42"),
            (EventType::PaymentCheck, "43"),
            (EventType::PaymentUpdate, "44"),
            (EventType::PaymentGroupCheck, "45"),
            (EventType::PaymentGroupUpdate, "46"),
            (EventType::EntryRefund, "47"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn event_type_all_variants_deserialize() {
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
            ("12", EventType::Overdraft),
            ("13", EventType::Interchange),
            ("14", EventType::Processor),
            ("15", EventType::AchFail),
            ("16", EventType::AccountVerification),
            ("17", EventType::Sift),
            ("18", EventType::Adjustment),
            ("19", EventType::Retrieval),
            ("20", EventType::Arbitration),
            ("21", EventType::ECheckSale),
            ("22", EventType::ECheckRefund),
            ("23", EventType::ECheckReturn),
            ("24", EventType::Settlement),
            ("25", EventType::Misuse),
            ("26", EventType::ProfitShare),
            ("27", EventType::Unauthorized),
            ("28", EventType::AchNotificationOfChange),
            ("29", EventType::ECheckNotificationOfChange),
            ("30", EventType::ECheckFail),
            ("31", EventType::ECheckNonSufficientFunds),
            ("32", EventType::CurrencyConversion),
            ("33", EventType::TerminalTransaction),
            ("34", EventType::ReversePayout),
            ("35", EventType::PartialReversePayout),
            ("36", EventType::ReserveEntry),
            ("37", EventType::ReserveEntryRelease),
            ("38", EventType::PendingEntry),
            ("39", EventType::PendingPaid),
            ("40", EventType::Remainder),
            ("41", EventType::RemainderUsed),
            ("42", EventType::PendingRefundCancelled),
            ("43", EventType::PaymentCheck),
            ("44", EventType::PaymentUpdate),
            ("45", EventType::PaymentGroupCheck),
            ("46", EventType::PaymentGroupUpdate),
            ("47", EventType::EntryRefund),
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
            "entity": "t1_ety_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "batch": "t1_bat_12345678901234567890123",
            "disbursement": "t1_dis_12345678901234567890123",
            "type": 1,
            "category": "transaction",
            "debit": 0,
            "credit": 10000,
            "net": 10000,
            "balance": 50000,
            "currency": "USD",
            "status": 1,
            "effective": "20240115",
            "description": "Credit card payment",
            "reference": "REF-12345",
            "custom": "custom_data",
            "created": "2024-01-01 10:00:00.000",
            "modified": "2024-01-01 15:30:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entry.entity.as_ref().unwrap().as_str(), "t1_ety_12345678901234567890123");
        assert_eq!(entry.merchant.as_ref().unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(entry.txn.as_ref().unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(entry.fund.as_ref().unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(entry.batch.as_ref().unwrap().as_str(), "t1_bat_12345678901234567890123");
        assert_eq!(entry.disbursement.as_ref().unwrap().as_str(), "t1_dis_12345678901234567890123");
        assert_eq!(entry.entry_type.unwrap(), EntryType::TransactionCredit);
        assert_eq!(entry.category.as_ref().unwrap(), "transaction");
        assert_eq!(entry.debit.unwrap(), 0);
        assert_eq!(entry.credit.unwrap(), 10000);
        assert_eq!(entry.net.unwrap(), 10000);
        assert_eq!(entry.balance.unwrap(), 50000);
        assert_eq!(entry.currency.as_ref().unwrap(), "USD");
        assert_eq!(entry.status.unwrap(), 1);
        assert_eq!(entry.effective.as_ref().unwrap().as_str(), "20240115");
        assert_eq!(entry.description.as_ref().unwrap(), "Credit card payment");
        assert_eq!(entry.reference.as_ref().unwrap(), "REF-12345");
        assert_eq!(entry.custom.as_ref().unwrap(), "custom_data");
        assert_eq!(entry.created.as_ref().unwrap(), "2024-01-01 10:00:00.000");
        assert_eq!(entry.modified.as_ref().unwrap(), "2024-01-01 15:30:00.000");
        assert_eq!(entry.inactive, false);
        assert_eq!(entry.frozen, true);
    }

    #[test]
    fn entry_deserialize_minimal() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123"
        }"#;

        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id.as_str(), "t1_ent_12345678901234567890123");
        assert!(entry.entity.is_none());
        assert!(entry.merchant.is_none());
        assert!(entry.txn.is_none());
        assert!(entry.fund.is_none());
        assert!(entry.batch.is_none());
        assert!(entry.disbursement.is_none());
        assert!(entry.entry_type.is_none());
        assert!(entry.category.is_none());
        assert!(entry.debit.is_none());
        assert!(entry.credit.is_none());
        assert!(entry.net.is_none());
        assert!(entry.balance.is_none());
        assert!(entry.currency.is_none());
        assert!(entry.status.is_none());
        assert!(entry.effective.is_none());
        assert!(entry.description.is_none());
        assert!(entry.reference.is_none());
        assert!(entry.custom.is_none());
        assert!(entry.created.is_none());
        assert!(entry.modified.is_none());
        assert_eq!(entry.inactive, false);
        assert_eq!(entry.frozen, false);
    }

    #[test]
    fn entry_deserialize_bool_from_int() {
        // Test bool_from_int deserialization
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "inactive": 1,
            "frozen": 0
        }"#;

        let entry: Entry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.inactive, true);
        assert_eq!(entry.frozen, false);
    }
}
