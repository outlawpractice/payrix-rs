//! Transaction types for the Payrix API.
//!
//! Transactions represent payment operations in Payrix. All monetary values
//! are stored as integers in **cents** (e.g., $10.00 = 1000).

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use super::{bool_from_int_default_false, deserialize_optional_amount, deserialize_optional_i32, DateMmyy, DateYmd, PayrixId};

/// A Payrix transaction.
///
/// Represents a payment transaction including sales, authorizations, captures,
/// refunds, and eCheck transactions. All monetary fields (`total`, `approved`,
/// `refunded`, etc.) are in **cents**.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Unique identifier (30 characters, e.g., "t1_txn_...")
    pub id: PayrixId,

    /// The ID of the Merchant (not the Merchant's entity)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Token string for the payment method (if using tokenized payment).
    /// This is the token string value, not the token ID.
    #[serde(default)]
    pub token: Option<String>,

    /// Transaction type
    #[serde(rename = "type")]
    pub txn_type: TransactionType,

    /// The total amount of this transaction, in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub total: Option<i64>,

    /// The total amount approved by the processor, in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub approved: Option<i64>,

    /// Authorization code returned by the network
    #[serde(default)]
    pub authorization: Option<String>,

    /// Authorization code for this transaction
    #[serde(default)]
    pub auth_code: Option<String>,

    /// Date of authorization in YYYYMMDD format
    #[serde(default)]
    pub auth_date: Option<DateYmd>,

    /// Transaction status
    #[serde(default)]
    pub status: Option<TransactionStatus>,

    /// Description/memo field (up to 1000 characters).
    /// Often contains JSON with custom data.
    #[serde(default)]
    pub description: Option<String>,

    /// First 6 digits of card (BIN)
    #[serde(default)]
    pub first6: Option<String>,

    /// Last 4 digits of card/account number
    #[serde(default)]
    pub last4: Option<String>,

    /// If linked to another transaction, the ID of that transaction.
    /// Used for refunds referencing the original sale.
    #[serde(default)]
    pub fortxn: Option<PayrixId>,

    /// Transaction being reauthorized (for resubmission/reauthorization)
    #[serde(default)]
    pub fromtxn: Option<PayrixId>,

    /// Batch ID if transaction is linked to a batch
    #[serde(default)]
    pub batch: Option<PayrixId>,

    /// Amount refunded from this transaction, in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub refunded: Option<i64>,

    /// Date settled in YYYYMMDD format
    #[serde(default)]
    pub settled: Option<i64>,

    /// Settled amount in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub settled_total: Option<i64>,

    /// Date funded in YYYYMMDD format
    #[serde(default)]
    pub funded: Option<i64>,

    /// Date returned in YYYYMMDD format
    #[serde(default)]
    pub returned: Option<DateYmd>,

    /// Transaction origin
    #[serde(default)]
    pub origin: Option<TransactionOrigin>,

    /// Card-on-file type for tokenized transactions
    #[serde(default)]
    pub cof_type: Option<CardOnFileType>,

    /// Whether partial payment is allowed (0 or 1)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub allow_partial: Option<i32>,

    /// Subscription ID if from a recurring payment
    #[serde(default)]
    pub subscription: Option<PayrixId>,

    /// Processing platform
    #[serde(default)]
    pub platform: Option<TransactionPlatform>,

    /// 3-letter currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Client IP address from which transaction was created
    #[serde(default)]
    pub client_ip: Option<String>,

    /// Card expiration in MMYY format
    #[serde(default)]
    pub expiration: Option<DateMmyy>,

    /// Customer's first name
    #[serde(default)]
    pub first: Option<String>,

    /// Customer's middle name
    #[serde(default)]
    pub middle: Option<String>,

    /// Customer's last name
    #[serde(default)]
    pub last: Option<String>,

    /// Company name
    #[serde(default)]
    pub company: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// Address line 1
    #[serde(default)]
    pub address1: Option<String>,

    /// Address line 2
    #[serde(default)]
    pub address2: Option<String>,

    /// City
    #[serde(default)]
    pub city: Option<String>,

    /// Two-letter state/province code
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(default)]
    pub zip: Option<String>,

    /// 3-character country code
    #[serde(default)]
    pub country: Option<String>,

    /// Tax amount in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub tax: Option<i64>,

    /// Shipping fee in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub shipping: Option<i64>,

    /// Discount amount in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub discount: Option<i64>,

    /// Surcharge amount in **cents**
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub surcharge: Option<i64>,

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

/// Transaction types.
/// NOTE: API may return values as strings (e.g., "1") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TransactionType {
    /// Credit card sale (auth + capture)
    #[default]
    CreditCardSale = 1,
    /// Credit card authorization only
    CreditCardAuth = 2,
    /// Capture a previous authorization
    CreditCardCapture = 3,
    /// Reverse a prior auth or sale
    CreditCardReverseAuth = 4,
    /// Refund a prior capture or sale
    CreditCardRefund = 5,
    /// eCheck sale
    ECheckSale = 7,
    /// eCheck refund
    ECheckRefund = 8,
    /// Redeposit a failed eCheck
    ECheckRedeposit = 11,
    /// Verify eCheck payment details
    ECheckAccountVerification = 12,
}

crate::impl_flexible_i32_enum_deserialize!(TransactionType, [
    (1, CreditCardSale),
    (2, CreditCardAuth),
    (3, CreditCardCapture),
    (4, CreditCardReverseAuth),
    (5, CreditCardRefund),
    (7, ECheckSale),
    (8, ECheckRefund),
    (11, ECheckRedeposit),
    (12, ECheckAccountVerification),
]);

/// Transaction status values.
/// NOTE: API may return values as strings (e.g., "1") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TransactionStatus {
    /// Gateway awaiting processor confirmation
    #[default]
    Pending = 0,
    /// Transaction approved (authorized)
    Approved = 1,
    /// Transaction failed
    Failed = 2,
    /// Transaction captured, funds not yet deposited
    Captured = 3,
    /// Processing complete, funds ready for distribution
    Settled = 4,
    /// Transaction returned
    Returned = 5,
}

crate::impl_flexible_i32_enum_deserialize!(TransactionStatus, [
    (0, Pending),
    (1, Approved),
    (2, Failed),
    (3, Captured),
    (4, Settled),
    (5, Returned),
]);

/// Transaction origin values.
/// NOTE: API may return values as strings (e.g., "2") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TransactionOrigin {
    /// Unknown origin
    Unknown = 0,
    /// Terminal/retail transaction
    #[default]
    Terminal = 1,
    /// Ecommerce transaction
    Ecommerce = 2,
    /// Mail or telephone order
    MailOrTelephoneOrder = 3,
    /// Apple Pay transaction
    ApplePay = 4,
    /// Successful 3DS authentication
    Success3DS = 5,
    /// Attempted 3DS authentication
    Attempted3DS = 6,
    /// Recurring transaction
    Recurring = 7,
    /// Payframe transaction
    Payframe = 8,
    /// Google Pay transaction
    GooglePay = 9,
}

crate::impl_flexible_i32_enum_deserialize!(TransactionOrigin, [
    (0, Unknown),
    (1, Terminal),
    (2, Ecommerce),
    (3, MailOrTelephoneOrder),
    (4, ApplePay),
    (5, Success3DS),
    (6, Attempted3DS),
    (7, Recurring),
    (8, Payframe),
    (9, GooglePay),
]);

/// Transaction platform values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionPlatform {
    /// Apple Pay
    Apple,
    /// Elavon processor
    Elavon,
    /// First Data processor
    #[serde(rename = "FIRSTDATA")]
    FirstData,
    /// Google Pay
    Google,
    /// Vantiv processor
    Vantiv,
    /// Vcore processor
    Vcore,
    /// Wells Fargo ACH
    #[serde(rename = "WELLSACH")]
    WellsAch,
    /// Wells Fargo processor
    #[serde(rename = "WELLSFARGO")]
    WellsFargo,
    /// Wells Fargo Single
    #[serde(rename = "WFSINGLE")]
    WfSingle,
    /// Worldpay processor
    #[default]
    Worldpay,
    /// TD Bank Canada
    #[serde(rename = "TDBANKCA")]
    TdBankCanada,
}

/// Card-on-file type for tokenized transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardOnFileType {
    /// Single/one-time transaction
    #[default]
    Single,
    /// Scheduled recurring transaction
    Scheduled,
    /// Unscheduled merchant-initiated transaction
    Unscheduled,
}

/// Terminal capability values.
/// NOTE: API may return values as strings (e.g., "1") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TerminalCapability {
    /// Key entry only
    #[default]
    KeyEntryOnly = 1,
    /// Magnetic stripe reader
    MagneticStripe = 2,
    /// Integrated circuit (chip) reader
    IntegratedCircuitReader = 3,
}

crate::impl_flexible_i32_enum_deserialize!(TerminalCapability, [
    (1, KeyEntryOnly),
    (2, MagneticStripe),
    (3, IntegratedCircuitReader),
]);

/// Entry mode values for how payment information was entered.
/// NOTE: API may return values as strings (e.g., "1") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum EntryMode {
    /// Terminal used with key entry
    #[default]
    TerminalUsedWithKey = 1,
    /// Track 1 read
    Track1Read = 2,
    /// Track 2 read
    Track2Read = 3,
    /// Complete magnetic stripe read
    CompleteMagneticStripeRead = 4,
    /// EMV chip read
    EmvChipRead = 5,
}

crate::impl_flexible_i32_enum_deserialize!(EntryMode, [
    (1, TerminalUsedWithKey),
    (2, Track1Read),
    (3, Track2Read),
    (4, CompleteMagneticStripeRead),
    (5, EmvChipRead),
]);

/// Unauthorized transaction reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnauthReason {
    /// Transaction incomplete
    #[default]
    Incomplete,
    /// Transaction timed out
    Timeout,
    /// Clerk cancelled transaction
    ClerkCancelled,
    /// Customer cancelled transaction
    CustomerCancelled,
    /// Mis-dispense
    #[serde(rename = "misdispense")]
    MisDispense,
    /// Hardware failure
    HardwareFailure,
    /// Suspected fraud
    SuspectedFraud,
}

/// Transaction result type.
/// NOTE: API may return values as strings (e.g., "1") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TxnResultType {
    /// General result
    #[default]
    General = 1,
    /// Fraud prevention alert
    FraudPrevention = 2,
    /// Processor error
    Processor = 3,
    /// CVV mismatch alert
    CvvMismatch = 4,
    /// AVS check alert
    AvsCheck = 5,
    /// AAVS check alert
    AavsCheck = 6,
    /// Network error
    NetworkError = 7,
    /// 3DS check alert
    ThreeDsAlert = 8,
}

crate::impl_flexible_i32_enum_deserialize!(TxnResultType, [
    (1, General),
    (2, FraudPrevention),
    (3, Processor),
    (4, CvvMismatch),
    (5, AvsCheck),
    (6, AavsCheck),
    (7, NetworkError),
    (8, ThreeDsAlert),
]);

/// Transaction result code.
/// NOTE: API may return values as strings (e.g., "0") instead of integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr)]
#[repr(i32)]
pub enum TxnResultCode {
    /// Transaction approved
    #[default]
    Approved = 0,
    /// Partially approved
    PartiallyApproved = 1,
    /// Declined
    Declined = 2,
    /// Verification successful
    VerificationSuccessful = 3,
    /// Verification unsuccessful
    VerificationUnsuccessful = 4,
    /// ZIP code mismatch
    ZipCodeMismatch = 5,
    /// Address mismatch
    AddressMismatch = 6,
    /// Name mismatch
    NameMismatch = 7,
    /// Name and phone mismatch
    NameAndPhoneMismatch = 8,
    /// Name and email mismatch
    NameAndEmailMismatch = 9,
    /// Phone mismatch
    PhoneMismatch = 10,
    /// Phone and email mismatch
    PhoneAndEmailMismatch = 11,
    /// Email mismatch
    EmailMismatch = 12,
    /// Name not in transaction data
    NameNotInTxnData = 13,
    /// Name and phone not in transaction data
    NameAndPhoneNotInTxnData = 14,
    /// Name and email not in transaction data
    NameAndEmailNotInTxnData = 15,
    /// Phone not in transaction data
    PhoneNotInTxnData = 16,
    /// Phone and email not in transaction data
    PhoneAndEmailNotInTxnData = 17,
    /// Email not in transaction data
    EmailNotInTxnData = 18,
    /// Customer info not in transaction data
    CustomerNotInTxnData = 19,
    /// Non-sufficient funds
    NonSufficientFunds = 20,
    /// Account invalid
    AccountInvalid = 21,
    /// Account unauthorized
    AccountUnauthorized = 22,
    /// General error
    GeneralError = 23,
    /// ZIP not in transaction data
    ZipNotInTxnData = 24,
    /// ZIP and address not in transaction data
    ZipAndAddressNotInTxnData = 25,
    /// Address not in transaction data
    AddressNotInTxnData = 26,
    /// Transaction not captured
    NotCaptured = 27,
    /// 3DS authentication passed
    ThreeDsPassed = 28,
    /// 3DS authentication invalid
    ThreeDsInvalid = 29,
    /// 3DS authentication failed
    ThreeDsFailed = 30,
    /// 3DS not validated
    ThreeDsNotValidated = 31,
    /// 3DS passed but liability not shifted
    ThreeDsAuthPassedLiabilityShifted = 32,
}

crate::impl_flexible_i32_enum_deserialize!(TxnResultCode, [
    (0, Approved),
    (1, PartiallyApproved),
    (2, Declined),
    (3, VerificationSuccessful),
    (4, VerificationUnsuccessful),
    (5, ZipCodeMismatch),
    (6, AddressMismatch),
    (7, NameMismatch),
    (8, NameAndPhoneMismatch),
    (9, NameAndEmailMismatch),
    (10, PhoneMismatch),
    (11, PhoneAndEmailMismatch),
    (12, EmailMismatch),
    (13, NameNotInTxnData),
    (14, NameAndPhoneNotInTxnData),
    (15, NameAndEmailNotInTxnData),
    (16, PhoneNotInTxnData),
    (17, PhoneAndEmailNotInTxnData),
    (18, EmailNotInTxnData),
    (19, CustomerNotInTxnData),
    (20, NonSufficientFunds),
    (21, AccountInvalid),
    (22, AccountUnauthorized),
    (23, GeneralError),
    (24, ZipNotInTxnData),
    (25, ZipAndAddressNotInTxnData),
    (26, AddressNotInTxnData),
    (27, NotCaptured),
    (28, ThreeDsPassed),
    (29, ThreeDsInvalid),
    (30, ThreeDsFailed),
    (31, ThreeDsNotValidated),
    (32, ThreeDsAuthPassedLiabilityShifted),
]);

/// Custom data stored in transaction description field.
///
/// This is typically serialized as JSON in the `description` field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCustom {
    /// Your application's transaction ID
    #[serde(default)]
    pub id: Option<String>,
    /// Whether this is a trust/retainer payment
    #[serde(default)]
    pub is_trust: Option<bool>,
    /// Free-form description text
    #[serde(default)]
    pub description: Option<String>,
}

/// Request to create a new transaction.
///
/// All monetary values are in **cents**.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTransaction {
    /// Merchant ID (required)
    pub merchant: String,

    /// Token ID for payment method (required for tokenized payments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Transaction type (required)
    #[serde(rename = "type")]
    pub txn_type: TransactionType,

    /// Transaction origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TransactionOrigin>,

    /// Card-on-file type for tokenized transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cof_type: Option<CardOnFileType>,

    /// Total amount in **cents** (required)
    pub total: i64,

    /// Description/memo (JSON string recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Related transaction ID (for refunds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fortxn: Option<String>,

    /// Fee ID to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,

    /// Allow partial payment (0 or 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial: Option<i32>,

    /// Client IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,

    /// Customer's first name (for eCheck refunds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Customer's middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// Customer's last name (for eCheck refunds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== TransactionType Tests ====================

    #[test]
    fn transaction_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TransactionType::CreditCardSale).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TransactionType::CreditCardAuth).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TransactionType::CreditCardCapture).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TransactionType::CreditCardReverseAuth).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TransactionType::CreditCardRefund).unwrap(), "5");
        assert_eq!(serde_json::to_string(&TransactionType::ECheckSale).unwrap(), "7");
        assert_eq!(serde_json::to_string(&TransactionType::ECheckRefund).unwrap(), "8");
        assert_eq!(serde_json::to_string(&TransactionType::ECheckRedeposit).unwrap(), "11");
        assert_eq!(serde_json::to_string(&TransactionType::ECheckAccountVerification).unwrap(), "12");
    }

    #[test]
    fn transaction_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TransactionType>("1").unwrap(), TransactionType::CreditCardSale);
        assert_eq!(serde_json::from_str::<TransactionType>("2").unwrap(), TransactionType::CreditCardAuth);
        assert_eq!(serde_json::from_str::<TransactionType>("3").unwrap(), TransactionType::CreditCardCapture);
        assert_eq!(serde_json::from_str::<TransactionType>("4").unwrap(), TransactionType::CreditCardReverseAuth);
        assert_eq!(serde_json::from_str::<TransactionType>("5").unwrap(), TransactionType::CreditCardRefund);
        assert_eq!(serde_json::from_str::<TransactionType>("7").unwrap(), TransactionType::ECheckSale);
        assert_eq!(serde_json::from_str::<TransactionType>("8").unwrap(), TransactionType::ECheckRefund);
        assert_eq!(serde_json::from_str::<TransactionType>("11").unwrap(), TransactionType::ECheckRedeposit);
        assert_eq!(serde_json::from_str::<TransactionType>("12").unwrap(), TransactionType::ECheckAccountVerification);
    }

    #[test]
    fn transaction_type_default() {
        assert_eq!(TransactionType::default(), TransactionType::CreditCardSale);
    }

    #[test]
    fn transaction_type_invalid_value() {
        assert!(serde_json::from_str::<TransactionType>("99").is_err());
    }

    // ==================== TransactionStatus Tests ====================

    #[test]
    fn transaction_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TransactionStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&TransactionStatus::Approved).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TransactionStatus::Failed).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TransactionStatus::Captured).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TransactionStatus::Settled).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TransactionStatus::Returned).unwrap(), "5");
    }

    #[test]
    fn transaction_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TransactionStatus>("0").unwrap(), TransactionStatus::Pending);
        assert_eq!(serde_json::from_str::<TransactionStatus>("1").unwrap(), TransactionStatus::Approved);
        assert_eq!(serde_json::from_str::<TransactionStatus>("2").unwrap(), TransactionStatus::Failed);
        assert_eq!(serde_json::from_str::<TransactionStatus>("3").unwrap(), TransactionStatus::Captured);
        assert_eq!(serde_json::from_str::<TransactionStatus>("4").unwrap(), TransactionStatus::Settled);
        assert_eq!(serde_json::from_str::<TransactionStatus>("5").unwrap(), TransactionStatus::Returned);
    }

    #[test]
    fn transaction_status_default() {
        assert_eq!(TransactionStatus::default(), TransactionStatus::Pending);
    }

    #[test]
    fn transaction_status_invalid_value() {
        assert!(serde_json::from_str::<TransactionStatus>("99").is_err());
    }

    // ==================== TransactionOrigin Tests ====================

    #[test]
    fn transaction_origin_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TransactionOrigin::Terminal).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Ecommerce).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TransactionOrigin::MailOrTelephoneOrder).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TransactionOrigin::ApplePay).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Success3DS).unwrap(), "5");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Attempted3DS).unwrap(), "6");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Recurring).unwrap(), "7");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Payframe).unwrap(), "8");
    }

    #[test]
    fn transaction_origin_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TransactionOrigin>("1").unwrap(), TransactionOrigin::Terminal);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("2").unwrap(), TransactionOrigin::Ecommerce);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("3").unwrap(), TransactionOrigin::MailOrTelephoneOrder);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("4").unwrap(), TransactionOrigin::ApplePay);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("5").unwrap(), TransactionOrigin::Success3DS);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("6").unwrap(), TransactionOrigin::Attempted3DS);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("7").unwrap(), TransactionOrigin::Recurring);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("8").unwrap(), TransactionOrigin::Payframe);
    }

    #[test]
    fn transaction_origin_default() {
        assert_eq!(TransactionOrigin::default(), TransactionOrigin::Terminal);
    }

    #[test]
    fn transaction_origin_invalid_value() {
        // 0 is now valid (Unknown), so test with values outside the valid range
        assert!(serde_json::from_str::<TransactionOrigin>("99").is_err());
        assert!(serde_json::from_str::<TransactionOrigin>("100").is_err());
    }

    #[test]
    fn transaction_origin_unknown_is_valid() {
        // OpenAPI spec includes 0 as a valid origin value
        let origin: TransactionOrigin = serde_json::from_str("0").unwrap();
        assert_eq!(origin, TransactionOrigin::Unknown);
    }

    // ==================== TransactionPlatform Tests ====================

    #[test]
    fn transaction_platform_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TransactionPlatform::Apple).unwrap(), "\"APPLE\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::Elavon).unwrap(), "\"ELAVON\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::FirstData).unwrap(), "\"FIRSTDATA\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::Google).unwrap(), "\"GOOGLE\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::Vantiv).unwrap(), "\"VANTIV\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::Vcore).unwrap(), "\"VCORE\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::WellsAch).unwrap(), "\"WELLSACH\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::WellsFargo).unwrap(), "\"WELLSFARGO\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::WfSingle).unwrap(), "\"WFSINGLE\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::Worldpay).unwrap(), "\"WORLDPAY\"");
        assert_eq!(serde_json::to_string(&TransactionPlatform::TdBankCanada).unwrap(), "\"TDBANKCA\"");
    }

    #[test]
    fn transaction_platform_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"APPLE\"").unwrap(), TransactionPlatform::Apple);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"ELAVON\"").unwrap(), TransactionPlatform::Elavon);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"FIRSTDATA\"").unwrap(), TransactionPlatform::FirstData);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"GOOGLE\"").unwrap(), TransactionPlatform::Google);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"VANTIV\"").unwrap(), TransactionPlatform::Vantiv);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"VCORE\"").unwrap(), TransactionPlatform::Vcore);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"WELLSACH\"").unwrap(), TransactionPlatform::WellsAch);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"WELLSFARGO\"").unwrap(), TransactionPlatform::WellsFargo);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"WFSINGLE\"").unwrap(), TransactionPlatform::WfSingle);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"WORLDPAY\"").unwrap(), TransactionPlatform::Worldpay);
        assert_eq!(serde_json::from_str::<TransactionPlatform>("\"TDBANKCA\"").unwrap(), TransactionPlatform::TdBankCanada);
    }

    #[test]
    fn transaction_platform_default() {
        assert_eq!(TransactionPlatform::default(), TransactionPlatform::Worldpay);
    }

    #[test]
    fn transaction_platform_invalid_value() {
        assert!(serde_json::from_str::<TransactionPlatform>("\"INVALID\"").is_err());
    }

    // ==================== CardOnFileType Tests ====================

    #[test]
    fn card_on_file_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&CardOnFileType::Single).unwrap(), "\"single\"");
        assert_eq!(serde_json::to_string(&CardOnFileType::Scheduled).unwrap(), "\"scheduled\"");
        assert_eq!(serde_json::to_string(&CardOnFileType::Unscheduled).unwrap(), "\"unscheduled\"");
    }

    #[test]
    fn card_on_file_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"single\"").unwrap(), CardOnFileType::Single);
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"scheduled\"").unwrap(), CardOnFileType::Scheduled);
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"unscheduled\"").unwrap(), CardOnFileType::Unscheduled);
    }

    #[test]
    fn card_on_file_type_default() {
        assert_eq!(CardOnFileType::default(), CardOnFileType::Single);
    }

    #[test]
    fn card_on_file_type_invalid_value() {
        assert!(serde_json::from_str::<CardOnFileType>("\"invalid\"").is_err());
    }

    // ==================== TerminalCapability Tests ====================

    #[test]
    fn terminal_capability_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TerminalCapability::KeyEntryOnly).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TerminalCapability::MagneticStripe).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TerminalCapability::IntegratedCircuitReader).unwrap(), "3");
    }

    #[test]
    fn terminal_capability_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TerminalCapability>("1").unwrap(), TerminalCapability::KeyEntryOnly);
        assert_eq!(serde_json::from_str::<TerminalCapability>("2").unwrap(), TerminalCapability::MagneticStripe);
        assert_eq!(serde_json::from_str::<TerminalCapability>("3").unwrap(), TerminalCapability::IntegratedCircuitReader);
    }

    #[test]
    fn terminal_capability_default() {
        assert_eq!(TerminalCapability::default(), TerminalCapability::KeyEntryOnly);
    }

    #[test]
    fn terminal_capability_invalid_value() {
        assert!(serde_json::from_str::<TerminalCapability>("0").is_err());
        assert!(serde_json::from_str::<TerminalCapability>("99").is_err());
    }

    // ==================== EntryMode Tests ====================

    #[test]
    fn entry_mode_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&EntryMode::TerminalUsedWithKey).unwrap(), "1");
        assert_eq!(serde_json::to_string(&EntryMode::Track1Read).unwrap(), "2");
        assert_eq!(serde_json::to_string(&EntryMode::Track2Read).unwrap(), "3");
        assert_eq!(serde_json::to_string(&EntryMode::CompleteMagneticStripeRead).unwrap(), "4");
        assert_eq!(serde_json::to_string(&EntryMode::EmvChipRead).unwrap(), "5");
    }

    #[test]
    fn entry_mode_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<EntryMode>("1").unwrap(), EntryMode::TerminalUsedWithKey);
        assert_eq!(serde_json::from_str::<EntryMode>("2").unwrap(), EntryMode::Track1Read);
        assert_eq!(serde_json::from_str::<EntryMode>("3").unwrap(), EntryMode::Track2Read);
        assert_eq!(serde_json::from_str::<EntryMode>("4").unwrap(), EntryMode::CompleteMagneticStripeRead);
        assert_eq!(serde_json::from_str::<EntryMode>("5").unwrap(), EntryMode::EmvChipRead);
    }

    #[test]
    fn entry_mode_default() {
        assert_eq!(EntryMode::default(), EntryMode::TerminalUsedWithKey);
    }

    #[test]
    fn entry_mode_invalid_value() {
        assert!(serde_json::from_str::<EntryMode>("0").is_err());
        assert!(serde_json::from_str::<EntryMode>("99").is_err());
    }

    // ==================== UnauthReason Tests ====================

    #[test]
    fn unauth_reason_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&UnauthReason::Incomplete).unwrap(), "\"incomplete\"");
        assert_eq!(serde_json::to_string(&UnauthReason::Timeout).unwrap(), "\"timeout\"");
        assert_eq!(serde_json::to_string(&UnauthReason::ClerkCancelled).unwrap(), "\"clerkCancelled\"");
        assert_eq!(serde_json::to_string(&UnauthReason::CustomerCancelled).unwrap(), "\"customerCancelled\"");
        assert_eq!(serde_json::to_string(&UnauthReason::MisDispense).unwrap(), "\"misdispense\"");
        assert_eq!(serde_json::to_string(&UnauthReason::HardwareFailure).unwrap(), "\"hardwareFailure\"");
        assert_eq!(serde_json::to_string(&UnauthReason::SuspectedFraud).unwrap(), "\"suspectedFraud\"");
    }

    #[test]
    fn unauth_reason_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<UnauthReason>("\"incomplete\"").unwrap(), UnauthReason::Incomplete);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"timeout\"").unwrap(), UnauthReason::Timeout);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"clerkCancelled\"").unwrap(), UnauthReason::ClerkCancelled);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"customerCancelled\"").unwrap(), UnauthReason::CustomerCancelled);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"misdispense\"").unwrap(), UnauthReason::MisDispense);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"hardwareFailure\"").unwrap(), UnauthReason::HardwareFailure);
        assert_eq!(serde_json::from_str::<UnauthReason>("\"suspectedFraud\"").unwrap(), UnauthReason::SuspectedFraud);
    }

    #[test]
    fn unauth_reason_default() {
        assert_eq!(UnauthReason::default(), UnauthReason::Incomplete);
    }

    #[test]
    fn unauth_reason_invalid_value() {
        assert!(serde_json::from_str::<UnauthReason>("\"invalid\"").is_err());
    }

    // ==================== TxnResultType Tests ====================

    #[test]
    fn txn_result_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TxnResultType::General).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TxnResultType::FraudPrevention).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TxnResultType::Processor).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TxnResultType::CvvMismatch).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TxnResultType::AvsCheck).unwrap(), "5");
        assert_eq!(serde_json::to_string(&TxnResultType::AavsCheck).unwrap(), "6");
        assert_eq!(serde_json::to_string(&TxnResultType::NetworkError).unwrap(), "7");
        assert_eq!(serde_json::to_string(&TxnResultType::ThreeDsAlert).unwrap(), "8");
    }

    #[test]
    fn txn_result_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TxnResultType>("1").unwrap(), TxnResultType::General);
        assert_eq!(serde_json::from_str::<TxnResultType>("2").unwrap(), TxnResultType::FraudPrevention);
        assert_eq!(serde_json::from_str::<TxnResultType>("3").unwrap(), TxnResultType::Processor);
        assert_eq!(serde_json::from_str::<TxnResultType>("4").unwrap(), TxnResultType::CvvMismatch);
        assert_eq!(serde_json::from_str::<TxnResultType>("5").unwrap(), TxnResultType::AvsCheck);
        assert_eq!(serde_json::from_str::<TxnResultType>("6").unwrap(), TxnResultType::AavsCheck);
        assert_eq!(serde_json::from_str::<TxnResultType>("7").unwrap(), TxnResultType::NetworkError);
        assert_eq!(serde_json::from_str::<TxnResultType>("8").unwrap(), TxnResultType::ThreeDsAlert);
    }

    #[test]
    fn txn_result_type_default() {
        assert_eq!(TxnResultType::default(), TxnResultType::General);
    }

    #[test]
    fn txn_result_type_invalid_value() {
        assert!(serde_json::from_str::<TxnResultType>("0").is_err());
        assert!(serde_json::from_str::<TxnResultType>("99").is_err());
    }

    // ==================== TxnResultCode Tests ====================

    #[test]
    fn txn_result_code_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TxnResultCode::Approved).unwrap(), "0");
        assert_eq!(serde_json::to_string(&TxnResultCode::PartiallyApproved).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TxnResultCode::Declined).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TxnResultCode::VerificationSuccessful).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TxnResultCode::VerificationUnsuccessful).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TxnResultCode::ZipCodeMismatch).unwrap(), "5");
        assert_eq!(serde_json::to_string(&TxnResultCode::AddressMismatch).unwrap(), "6");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameMismatch).unwrap(), "7");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameAndPhoneMismatch).unwrap(), "8");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameAndEmailMismatch).unwrap(), "9");
        assert_eq!(serde_json::to_string(&TxnResultCode::PhoneMismatch).unwrap(), "10");
        assert_eq!(serde_json::to_string(&TxnResultCode::PhoneAndEmailMismatch).unwrap(), "11");
        assert_eq!(serde_json::to_string(&TxnResultCode::EmailMismatch).unwrap(), "12");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameNotInTxnData).unwrap(), "13");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameAndPhoneNotInTxnData).unwrap(), "14");
        assert_eq!(serde_json::to_string(&TxnResultCode::NameAndEmailNotInTxnData).unwrap(), "15");
        assert_eq!(serde_json::to_string(&TxnResultCode::PhoneNotInTxnData).unwrap(), "16");
        assert_eq!(serde_json::to_string(&TxnResultCode::PhoneAndEmailNotInTxnData).unwrap(), "17");
        assert_eq!(serde_json::to_string(&TxnResultCode::EmailNotInTxnData).unwrap(), "18");
        assert_eq!(serde_json::to_string(&TxnResultCode::CustomerNotInTxnData).unwrap(), "19");
        assert_eq!(serde_json::to_string(&TxnResultCode::NonSufficientFunds).unwrap(), "20");
        assert_eq!(serde_json::to_string(&TxnResultCode::AccountInvalid).unwrap(), "21");
        assert_eq!(serde_json::to_string(&TxnResultCode::AccountUnauthorized).unwrap(), "22");
        assert_eq!(serde_json::to_string(&TxnResultCode::GeneralError).unwrap(), "23");
        assert_eq!(serde_json::to_string(&TxnResultCode::ZipNotInTxnData).unwrap(), "24");
        assert_eq!(serde_json::to_string(&TxnResultCode::ZipAndAddressNotInTxnData).unwrap(), "25");
        assert_eq!(serde_json::to_string(&TxnResultCode::AddressNotInTxnData).unwrap(), "26");
        assert_eq!(serde_json::to_string(&TxnResultCode::NotCaptured).unwrap(), "27");
        assert_eq!(serde_json::to_string(&TxnResultCode::ThreeDsPassed).unwrap(), "28");
        assert_eq!(serde_json::to_string(&TxnResultCode::ThreeDsInvalid).unwrap(), "29");
        assert_eq!(serde_json::to_string(&TxnResultCode::ThreeDsFailed).unwrap(), "30");
        assert_eq!(serde_json::to_string(&TxnResultCode::ThreeDsNotValidated).unwrap(), "31");
        assert_eq!(serde_json::to_string(&TxnResultCode::ThreeDsAuthPassedLiabilityShifted).unwrap(), "32");
    }

    #[test]
    fn txn_result_code_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TxnResultCode>("0").unwrap(), TxnResultCode::Approved);
        assert_eq!(serde_json::from_str::<TxnResultCode>("1").unwrap(), TxnResultCode::PartiallyApproved);
        assert_eq!(serde_json::from_str::<TxnResultCode>("2").unwrap(), TxnResultCode::Declined);
        assert_eq!(serde_json::from_str::<TxnResultCode>("3").unwrap(), TxnResultCode::VerificationSuccessful);
        assert_eq!(serde_json::from_str::<TxnResultCode>("4").unwrap(), TxnResultCode::VerificationUnsuccessful);
        assert_eq!(serde_json::from_str::<TxnResultCode>("5").unwrap(), TxnResultCode::ZipCodeMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("6").unwrap(), TxnResultCode::AddressMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("7").unwrap(), TxnResultCode::NameMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("8").unwrap(), TxnResultCode::NameAndPhoneMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("9").unwrap(), TxnResultCode::NameAndEmailMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("10").unwrap(), TxnResultCode::PhoneMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("11").unwrap(), TxnResultCode::PhoneAndEmailMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("12").unwrap(), TxnResultCode::EmailMismatch);
        assert_eq!(serde_json::from_str::<TxnResultCode>("13").unwrap(), TxnResultCode::NameNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("14").unwrap(), TxnResultCode::NameAndPhoneNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("15").unwrap(), TxnResultCode::NameAndEmailNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("16").unwrap(), TxnResultCode::PhoneNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("17").unwrap(), TxnResultCode::PhoneAndEmailNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("18").unwrap(), TxnResultCode::EmailNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("19").unwrap(), TxnResultCode::CustomerNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("20").unwrap(), TxnResultCode::NonSufficientFunds);
        assert_eq!(serde_json::from_str::<TxnResultCode>("21").unwrap(), TxnResultCode::AccountInvalid);
        assert_eq!(serde_json::from_str::<TxnResultCode>("22").unwrap(), TxnResultCode::AccountUnauthorized);
        assert_eq!(serde_json::from_str::<TxnResultCode>("23").unwrap(), TxnResultCode::GeneralError);
        assert_eq!(serde_json::from_str::<TxnResultCode>("24").unwrap(), TxnResultCode::ZipNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("25").unwrap(), TxnResultCode::ZipAndAddressNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("26").unwrap(), TxnResultCode::AddressNotInTxnData);
        assert_eq!(serde_json::from_str::<TxnResultCode>("27").unwrap(), TxnResultCode::NotCaptured);
        assert_eq!(serde_json::from_str::<TxnResultCode>("28").unwrap(), TxnResultCode::ThreeDsPassed);
        assert_eq!(serde_json::from_str::<TxnResultCode>("29").unwrap(), TxnResultCode::ThreeDsInvalid);
        assert_eq!(serde_json::from_str::<TxnResultCode>("30").unwrap(), TxnResultCode::ThreeDsFailed);
        assert_eq!(serde_json::from_str::<TxnResultCode>("31").unwrap(), TxnResultCode::ThreeDsNotValidated);
        assert_eq!(serde_json::from_str::<TxnResultCode>("32").unwrap(), TxnResultCode::ThreeDsAuthPassedLiabilityShifted);
    }

    #[test]
    fn txn_result_code_default() {
        assert_eq!(TxnResultCode::default(), TxnResultCode::Approved);
    }

    #[test]
    fn txn_result_code_invalid_value() {
        assert!(serde_json::from_str::<TxnResultCode>("99").is_err());
    }

    // ==================== Transaction Struct Tests ====================

    #[test]
    fn transaction_deserialize_full() {
        let json = r#"{
            "id": "t1_txn_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "token": "t1_tok_12345678901234567890123",
            "type": 1,
            "total": 10000,
            "approved": 10000,
            "authorization": "AUTH123",
            "authCode": "123456",
            "authDate": "20240101",
            "status": 1,
            "description": "Test transaction",
            "first6": "424242",
            "last4": "4242",
            "fortxn": "t1_txn_98765432109876543210987",
            "fromtxn": "t1_txn_11111111111111111111111",
            "batch": "t1_bat_12345678901234567890123",
            "refunded": 0,
            "settled": 20240102,
            "settledTotal": 10000,
            "funded": 20240103,
            "returned": "20240104",
            "origin": 2,
            "cofType": "scheduled",
            "allowPartial": 1,
            "subscription": "t1_sub_12345678901234567890123",
            "platform": "WORLDPAY",
            "currency": "USD",
            "clientIp": "192.168.1.1",
            "expiration": "1225",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "company": "Acme Inc",
            "email": "john@example.com",
            "phone": "555-1234",
            "address1": "123 Main St",
            "address2": "Apt 4",
            "city": "New York",
            "state": "NY",
            "zip": "10001",
            "country": "USA",
            "tax": 100,
            "shipping": 500,
            "discount": 200,
            "surcharge": 50,
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(txn.id.as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(txn.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        // Token is now a string, not a PayrixId
        assert_eq!(txn.token.unwrap(), "t1_tok_12345678901234567890123");
        assert_eq!(txn.txn_type, TransactionType::CreditCardSale);
        assert_eq!(txn.total, Some(10000));
        assert_eq!(txn.status, Some(TransactionStatus::Approved));
        assert_eq!(txn.origin, Some(TransactionOrigin::Ecommerce));
        assert_eq!(txn.cof_type, Some(CardOnFileType::Scheduled));
        assert_eq!(txn.platform, Some(TransactionPlatform::Worldpay));
        assert!(!txn.inactive);
        assert!(txn.frozen);
    }

    #[test]
    fn transaction_deserialize_minimal() {
        let json = r#"{
            "id": "t1_txn_12345678901234567890123",
            "type": 1
        }"#;

        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(txn.id.as_str(), "t1_txn_12345678901234567890123");
        assert!(txn.merchant.is_none());
        assert_eq!(txn.txn_type, TransactionType::CreditCardSale);
        assert!(txn.token.is_none());
        assert!(txn.status.is_none());
        assert!(!txn.inactive);
        assert!(!txn.frozen);
    }

    #[test]
    fn transaction_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_txn_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123", "type": 1, "inactive": 0, "frozen": 0}"#;
        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert!(!txn.inactive);
        assert!(!txn.frozen);
    }

    #[test]
    fn transaction_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_txn_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123", "type": 1, "inactive": 1, "frozen": 1}"#;
        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert!(txn.inactive);
        assert!(txn.frozen);
    }

    #[test]
    fn transaction_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_txn_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123", "type": 1}"#;
        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert!(!txn.inactive);
        assert!(!txn.frozen);
    }

    // ==================== NewTransaction Tests ====================

    #[test]
    fn new_transaction_serialize_full() {
        let new_txn = NewTransaction {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            token: Some("t1_tok_12345678901234567890123".to_string()),
            txn_type: TransactionType::CreditCardSale,
            origin: Some(TransactionOrigin::Ecommerce),
            cof_type: Some(CardOnFileType::Scheduled),
            total: 10000,
            description: Some("Test transaction".to_string()),
            fortxn: Some("t1_txn_98765432109876543210987".to_string()),
            fee: Some("t1_fee_12345678901234567890123".to_string()),
            allow_partial: Some(1),
            client_ip: Some("192.168.1.1".to_string()),
            first: Some("John".to_string()),
            middle: Some("Q".to_string()),
            last: Some("Doe".to_string()),
        };

        let json = serde_json::to_string(&new_txn).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"total\":10000"));
        assert!(json.contains("\"origin\":2"));
        assert!(json.contains("\"cofType\":\"scheduled\""));
    }

    #[test]
    fn new_transaction_serialize_minimal() {
        let new_txn = NewTransaction {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            txn_type: TransactionType::CreditCardSale,
            total: 10000,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_txn).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"total\":10000"));
        // Optional fields should be omitted
        assert!(!json.contains("\"token\""));
        assert!(!json.contains("\"origin\""));
        assert!(!json.contains("\"description\""));
    }
}
