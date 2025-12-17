//! Transaction types for the Payrix API.
//!
//! **OpenAPI schema:** `txnsResponse`
//!
//! Transactions represent payment operations in Payrix including sales, authorizations,
//! captures, refunds, and eCheck transactions. All monetary values are stored as integers
//! in **cents** (e.g., $10.00 = 1000).
//!
//! See API_INCONSISTENCIES.md for known deviations from this spec.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, deserialize_optional_amount, deserialize_optional_i32, DateMmyy, PayrixId};

/// A Payrix transaction.
///
/// **OpenAPI schema:** `txnsResponse`
///
/// Represents a payment transaction including sales, authorizations, captures,
/// refunds, and eCheck transactions. All monetary fields (`total`, `approved`,
/// `refunded`, etc.) are in **cents**.
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    // ==================== Identity Fields ====================

    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS.SSSS
    ///
    /// **OpenAPI type:** string (pattern: ^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$)
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS.SSSS
    ///
    /// **OpenAPI type:** string (pattern: ^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$)
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

    /// The incoming IP address from which this Transaction was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ip_created: Option<String>,

    /// The incoming IP address from which this Transaction was last modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ip_modified: Option<String>,

    // ==================== Relationships ====================

    /// The identifier of the Merchant associated with this Transaction.
    ///
    /// **OpenAPI type:** string (ref: txnsModelMerchant)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The token of the Tokens resource this Transaction is associated with.
    ///
    /// **OpenAPI type:** string (ref: txnsModelToken)
    #[serde(default)]
    pub token: Option<String>,

    /// The payment method associated with this Transaction, including card details.
    ///
    /// **OpenAPI type:** string (ref: txnsModelPayment)
    #[serde(default)]
    pub payment: Option<String>,

    /// If this Transaction is related to another Transaction, the ID of that Transaction.
    ///
    /// For example, if this Transaction is a refund, this field could be set to the
    /// identifier of the original sale Transaction.
    ///
    /// **OpenAPI type:** string (ref: txnsModelFortxn)
    #[serde(default)]
    pub fortxn: Option<PayrixId>,

    /// Reauthorize this referenced Transaction.
    ///
    /// For example, to process a resubmission of a declined Transaction or to
    /// reauthorize an expired Transaction.
    ///
    /// **OpenAPI type:** string (ref: txnsModelFromtxn)
    #[serde(default)]
    pub fromtxn: Option<PayrixId>,

    /// If the Transaction is linked to a Batch, the identifier of the Batch.
    ///
    /// **OpenAPI type:** string (ref: txnsModelBatch)
    #[serde(default)]
    pub batch: Option<PayrixId>,

    /// The identifier of the Subscription associated with this Transaction.
    ///
    /// **OpenAPI type:** string (ref: txnsModelSubscription)
    #[serde(default)]
    pub subscription: Option<PayrixId>,

    /// The statement ID for which this transaction is being processed as payment.
    ///
    /// **OpenAPI type:** string (ref: txnsModelStatement)
    #[serde(default)]
    pub statement: Option<PayrixId>,

    // ==================== Transaction Type and Status ====================

    /// The type of Transaction.
    ///
    /// Valid values:
    /// - `1` - Credit Card Sale (auth + capture)
    /// - `2` - Credit Card Auth (authorize only)
    /// - `3` - Credit Card Capture (finalize auth)
    /// - `4` - Credit Card Reverse Auth (release hold)
    /// - `5` - Credit Card Refund
    /// - `7` - eCheck Sale
    /// - `8` - eCheck Refund
    /// - `11` - eCheck Redeposit (retry failed)
    /// - `12` - eCheck Account Verification
    /// - `14` - Incremental Authorization
    ///
    /// **OpenAPI type:** integer enum (txnType)
    #[serde(rename = "type")]
    pub txn_type: TransactionType,

    /// The status of the Transaction.
    ///
    /// Valid values:
    /// - `0` - Pending (gateway awaiting processor confirmation)
    /// - `1` - Approved (authorized, can be voided)
    /// - `2` - Failed
    /// - `3` - Captured (can be refunded)
    /// - `4` - Settled (can be refunded)
    /// - `5` - Returned (refunded)
    ///
    /// **OpenAPI type:** integer enum (txnStatus)
    ///
    /// **API Inconsistency:** POST returns string `"1"`, GET returns integer `1`.
    /// Uses flexible deserializer to handle both.
    #[serde(default)]
    pub status: Option<TransactionStatus>,

    /// The origin of the transaction.
    ///
    /// Valid values:
    /// - `0` - Unknown
    /// - `1` - Credit card terminal
    /// - `2` - eCommerce system
    /// - `3` - Mail order / telephone order
    /// - `4` - Apple Pay
    /// - `5` - Successful 3D Secure
    /// - `6` - Attempted 3D Secure
    /// - `7` - Recurring (deprecated)
    /// - `8` - PayFrame
    /// - `9` - In writing
    ///
    /// **OpenAPI type:** integer enum (txnOrigin)
    #[serde(default)]
    pub origin: Option<TransactionOrigin>,

    /// The platform used to process transactions.
    ///
    /// Valid values:
    /// - `APPLE` - Apple Payment Processor (Deprecated)
    /// - `ELAVON` - Elavon processor (Deprecated)
    /// - `FIRSTDATA` - FirstData processor (Deprecated)
    /// - `GOOGLE` - Google Payment Processor (Deprecated)
    /// - `VANTIV` - WorldPay eComm (VAP) processor
    /// - `VCORE` - WorldPay Core processor
    /// - `WELLSACH` - Wells Fargo ACH processor
    /// - `WELLSFARGO` - Wells Fargo Merchant Services (Deprecated)
    /// - `WFSINGLE` - WFSINGLE processor (Deprecated)
    ///
    /// **OpenAPI type:** string enum (txnsPlatform)
    #[serde(default)]
    pub platform: Option<TransactionPlatform>,

    // ==================== Amounts (all in cents) ====================

    /// The total amount of this Transaction in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub total: Option<i64>,

    /// The total amount approved by the processor in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub approved: Option<i64>,

    /// The amount originally authorized for the transaction in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub original_approved: Option<i64>,

    /// The amount refunded from this transaction in **cents**.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub refunded: Option<i32>,

    /// The total amount settled in **cents**.
    ///
    /// This field is set automatically.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub settled_total: Option<i64>,

    /// Tax amount in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub tax: Option<i64>,

    /// Surcharge amount in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub surcharge: Option<i64>,

    /// Shipping fee in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub shipping: Option<i64>,

    /// Discount amount in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub discount: Option<i64>,

    /// Duty fee in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub duty: Option<i64>,

    /// Cash back amount in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub cashback: Option<i64>,

    /// Convenience fee amount in **cents**.
    ///
    /// Fee charged when card payment is an alternative form of payment.
    /// Currently not active.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub convenience_fee: Option<i32>,

    /// Optional calculated fee amount indicator in cents (up to 3 decimal points).
    ///
    /// Should be used in conjunction with txnFee setting on Fees resource.
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub fee: Option<f64>,

    /// Tip amount in **cents**.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub tip: Option<i64>,

    // ==================== Dates and Timestamps ====================

    /// The date on which the Transaction was authorized.
    ///
    /// Format: YYYYMMDD (e.g., '20160120' for January 20, 2016).
    /// Must represent a date in the past.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub auth_date: Option<i32>,

    /// A timestamp indicating when this Transaction was captured.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS. Set automatically.
    ///
    /// **OpenAPI type:** string (pattern: ^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})
    #[serde(default)]
    pub captured: Option<String>,

    /// A date indicating when this Transaction was settled.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS. Set automatically.
    ///
    /// **OpenAPI type:** string (pattern: ^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})
    #[serde(default)]
    pub settled: Option<String>,

    /// A date indicating when this Transaction was funded.
    ///
    /// Set automatically.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub funded: Option<i32>,

    /// The transaction has been returned by the receiver.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub returned: Option<String>,

    // ==================== Currency ====================

    /// The currency for this transaction.
    ///
    /// 3-letter ISO currency code (e.g., "USD").
    /// See https://www.iban.com/currency-codes for valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The currency in which the transaction was funded.
    ///
    /// See https://www.iban.com/currency-codes for valid values.
    ///
    /// **OpenAPI type:** string (ref: FundingCurrency)
    #[serde(default)]
    pub funding_currency: Option<String>,

    /// The currency of the settled total.
    ///
    /// Set automatically.
    ///
    /// **OpenAPI type:** string (ref: SettledCurrency)
    #[serde(default)]
    pub settled_currency: Option<String>,

    /// The status of the currency conversion.
    ///
    /// Valid values:
    /// - `customerAccepted` - Customer accepted conversion rate
    /// - `customerRejected` - Customer rejected conversion rate
    /// - `notEligible` - Not eligible for conversion
    ///
    /// **OpenAPI type:** string enum (CurrencyConversion)
    #[serde(default)]
    pub currency_conversion: Option<CurrencyConversion>,

    // ==================== Authorization Codes ====================

    /// Authorization code returned by the network.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub authorization: Option<String>,

    /// The authorization code for this Transaction.
    ///
    /// Stored as text string, 0-20 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub auth_code: Option<String>,

    /// Authentication token returned by the network in a 3DSecure txn.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub authentication: Option<String>,

    /// Optional transaction ID returned by the network in a 3DSecure txn.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub authentication_id: Option<String>,

    // ==================== Card Information ====================

    /// The expiration date of this Transaction.
    ///
    /// Format: MMYY (e.g., '0623' for June 2023).
    /// Must reflect a future date.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub expiration: Option<DateMmyy>,

    /// Service code retrieved from track data for swiped transactions.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub service_code: Option<String>,

    /// Whether correct CVV was sent during this Transaction.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub cvv: Option<i32>,

    /// The status of the CVV on the card.
    ///
    /// Valid values:
    /// - `notPresent` - CVV is not present
    /// - `illegible` - CVV is illegible
    /// - `notProvided` - CVV was not provided
    ///
    /// **OpenAPI type:** string enum (CvvStatus)
    #[serde(default)]
    pub cvv_status: Option<CvvStatus>,

    /// Whether the card was swiped during this Transaction.
    ///
    /// Set to '1' automatically if 'track' data was received.
    /// - `0` - Not swiped
    /// - `1` - Swiped
    ///
    /// **OpenAPI type:** integer enum (Swiped)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub swiped: Option<i32>,

    /// Whether the card was dipped (using EMV chip) during this Transaction.
    ///
    /// - `0` - Not dipped
    /// - `1` - Dipped
    ///
    /// **OpenAPI type:** integer enum (Emv)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub emv: Option<i32>,

    /// Whether a signature was captured during this Transaction.
    ///
    /// Can be set manually or automatically when associating a signature
    /// via 'txnDatas' resource.
    /// - `0` - Not captured
    /// - `1` - Captured
    ///
    /// **OpenAPI type:** integer enum (Signature)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub signature: Option<i32>,

    /// Whether this Transaction was verified with a PIN.
    ///
    /// - `0` - No PIN verification
    /// - `1` - PIN verification
    ///
    /// **OpenAPI type:** integer enum (Pin)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub pin: Option<i32>,

    // ==================== Terminal Information ====================

    /// The identifier of the terminal that processed this Transaction.
    ///
    /// Identifier is taken from the terminal system and varies by terminal type.
    /// Stored as text string, 0-50 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub terminal: Option<String>,

    /// Capabilities of the terminal device.
    ///
    /// Valid values:
    /// - `1` - Key entry only terminal
    /// - `2` - Can read magnetic stripe
    /// - `3` - Integrated circuit reader
    /// - `4` - Can detect contactless payment
    ///
    /// **OpenAPI type:** integer enum (TerminalCapability)
    #[serde(default)]
    pub terminal_capability: Option<TerminalCapability>,

    /// How payment information has been entered.
    ///
    /// Valid values:
    /// - `1` - Manually keyed entry
    /// - `2` - Card swiped, Track 1 received
    /// - `3` - Card swiped, Track 2 received
    /// - `4` - Card swiped, Track 1 & 2 received
    /// - `5` - Card dipped, EMV chip received
    /// - `6` - Contactless card read, Track or EMV data received
    /// - `7` - Track Data from Card Swipe after EMV chip failure
    /// - `8` - Track Data from Manually keyed entry after EMV chip failure
    /// - `9` - ApplePay
    /// - `10` - Google Pay
    /// - `11` - Merchant created transaction
    /// - `12` - Invoice payment
    /// - `13` - Merchant created transaction in payrix portal
    /// - `14` - Invoice payment from payrix portal
    ///
    /// **OpenAPI type:** integer enum (entryMode)
    #[serde(default)]
    pub entry_mode: Option<EntryMode>,

    /// Indicates if transaction is processed through Mobile POS.
    ///
    /// - `0` - Non-mobile POS
    /// - `1` - Mobile POS
    ///
    /// **OpenAPI type:** integer enum (Mobile)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub mobile: Option<i32>,

    /// Indicates the PIN entry capability of the device.
    ///
    /// Valid values:
    /// - `unknown` - Unknown PIN Entry Capability
    /// - `capable` - Terminal can accept PINs
    /// - `notCapable` - Terminal cannot accept entry of PINs
    /// - `pinPadDown` - Terminal PIN Pad is down
    ///
    /// **OpenAPI type:** string enum (PinEntryCapability)
    #[serde(default)]
    pub pin_entry_capability: Option<PinEntryCapability>,

    /// Whether the card was swiped at an unattended terminal.
    ///
    /// Default is '0'.
    /// - `0` - Attended terminal
    /// - `1` - Unattended terminal
    ///
    /// **OpenAPI type:** integer enum (Unattended)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub unattended: Option<i32>,

    // ==================== Card on File ====================

    /// The type of Card On File transaction when using a token.
    ///
    /// Valid values:
    /// - `single` - Individual Transaction
    /// - `scheduled` - Scheduled Transaction
    /// - `unscheduled` - Unscheduled sequential Transaction
    /// - `installment` - Installment Transaction
    ///
    /// **OpenAPI type:** string enum (CofType)
    #[serde(default)]
    pub cof_type: Option<CardOnFileType>,

    // ==================== Order Information ====================

    /// The identifier of the Order associated with this Transaction.
    ///
    /// Stored as text string, 0-1000 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub order: Option<String>,

    /// A description of this Transaction.
    ///
    /// Stored as text string, 0-1000 characters.
    /// Often contains JSON with custom data.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The descriptor used in this Transaction.
    ///
    /// Stored as text string, 1-50 characters.
    /// If not set, defaults to merchant information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub descriptor: Option<String>,

    /// Sequential number that uniquely identifies the txn.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, deserialize_with = "deserialize_optional_amount")]
    pub trace_number: Option<i64>,

    // ==================== Customer Information ====================

    /// First name associated with this Transaction.
    ///
    /// For eCheck transactions, either first or last is required.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub first: Option<String>,

    /// Middle name associated with this Transaction.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub middle: Option<String>,

    /// Last name associated with this Transaction.
    ///
    /// For eCheck transactions, either first or last is required.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last: Option<String>,

    /// Company name associated with this Transaction.
    ///
    /// Especially important when processing an eCheck from a company.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub company: Option<String>,

    /// Email associated with this Transaction.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// Phone number associated with this Transaction.
    ///
    /// Stored as text string, 10-15 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// First line of the address associated with this Transaction.
    ///
    /// Stored as text string, 1-500 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// Second line of the address associated with this Transaction.
    ///
    /// Stored as text string, 1-500 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// City in the address associated with this Transaction.
    ///
    /// Stored as text string, 1-500 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// U.S. state or Canadian province.
    ///
    /// For US/Canada, use 2-character postal abbreviation.
    /// For other locations, provide full state name.
    /// Stored as text string, 2-100 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP code in the address.
    ///
    /// Stored as text string, 1-20 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// 3-letter ISO country code.
    ///
    /// **OpenAPI type:** string (ref: Country)
    #[serde(default)]
    pub country: Option<String>,

    /// The client IP address from which the Transaction was created.
    ///
    /// Valid values are any IPv4 or IPv6 address.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub client_ip: Option<String>,

    // ==================== Merchant/Processor Information ====================

    /// The Merchant ID as set by the processor.
    ///
    /// Stored as text string, 0-50 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mid: Option<String>,

    // ==================== Transaction Flags ====================

    /// Whether to allow partial amount authorizations.
    ///
    /// If the transaction amount is $1000 and the processor only authorizes
    /// a smaller amount, enabling this lets the Transaction proceed.
    /// - `0` - Partial not allowed
    /// - `1` - Partial allowed
    ///
    /// **OpenAPI type:** integer enum (AllowPartial)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub allow_partial: Option<i32>,

    /// Indicates whether the Transaction is reserved and action to be taken.
    ///
    /// Valid values:
    /// - `0` - No reserve
    /// - `1` - Block transaction, Entity sent to manual review queue
    /// - `3` - Hold transaction, will not be captured
    /// - `4` - Reserve transaction, funds should be reserved
    /// - `5` - Block current activity, no change for merchant
    /// - `8` - Onboard merchant, wait for manual check later
    ///
    /// **OpenAPI type:** integer enum (txnsReserved)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub reserved: Option<i32>,

    /// Indicates if authorization was misused by not being captured/reversed in time.
    ///
    /// Timeframe varies per network, MCC, and type of closing.
    /// - `0` - Not misused
    /// - `1` - Misused
    ///
    /// **OpenAPI type:** integer enum (Misused)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub misused: Option<i32>,

    /// The last transaction stage check for risk.
    ///
    /// Valid values:
    /// - `activation` - Terminal activation
    /// - `auth` - Apply during transaction authorization
    /// - `postauth` - Apply after transaction authorization
    /// - `capture` - Apply during transaction capture
    /// - `refund` - Apply when processing a refund
    ///
    /// **OpenAPI type:** string enum (txnCheckStage)
    #[serde(default)]
    pub check_stage: Option<CheckStage>,

    /// The reason for the auth reversal.
    ///
    /// Default is 'customerCancelled'.
    ///
    /// Valid values:
    /// - `incomplete` - Transaction incomplete
    /// - `timeout` - Transaction timeout
    /// - `clerkCancelled` - Cancelled by clerk
    /// - `customerCancelled` - Cancelled by customer
    /// - `misdispense` - Misdispense
    /// - `hardwareFailure` - Hardware failure
    /// - `suspectedFraud` - Suspected fraud
    ///
    /// **OpenAPI type:** string enum (unauthReason)
    #[serde(default)]
    pub unauth_reason: Option<UnauthReason>,

    /// Reason for copying Transaction referenced in fromtxn field.
    ///
    /// Valid values:
    /// - `resubmission` - Resubmission
    /// - `reauthorization` - Reauthorization
    ///
    /// **OpenAPI type:** string enum (CopyReason)
    #[serde(default)]
    pub copy_reason: Option<CopyReason>,

    /// Whether the txn was imported from a report.
    ///
    /// Set automatically.
    /// - `0` - Not imported
    /// - `1` - Imported
    ///
    /// **OpenAPI type:** integer enum (Imported)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub imported: Option<i32>,

    /// Entry creation/deletion job sequencing: current request sequence number.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub request_sequence: Option<i32>,

    /// Entry creation/deletion job sequencing: current processed sequence number.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub processed_sequence: Option<i32>,

    /// Whether this is a debt repayment transaction.
    ///
    /// **OpenAPI type:** integer
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub debt_repayment: Option<i32>,

    /// Whether funding is enabled for this transaction.
    ///
    /// **OpenAPI type:** integer
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub funding_enabled: Option<i32>,

    /// For Benefit Of indicator.
    ///
    /// **OpenAPI type:** integer
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub fbo: Option<i32>,

    /// Transaction session identifier.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub txnsession: Option<String>,

    /// Customer identifier from the AuthToken used during authentication.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub auth_token_customer: Option<String>,

    /// Channel information.
    ///
    /// Stored as text string, 0-1000 characters.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub channel: Option<String>,

    /// Soft POS identifier.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub soft_pos_id: Option<String>,

    /// Soft POS device type indicator.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub soft_pos_device_type_indicator: Option<String>,

    /// Network token indicator.
    ///
    /// **OpenAPI type:** integer
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub network_token_indicator: Option<i32>,

    // ==================== Standard Flags ====================

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer enum (Inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer enum (Frozen)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// ==================== Enums ====================

/// Transaction types per OpenAPI spec.
///
/// **OpenAPI schema:** `txnType`
///
/// Valid values:
/// - `1` - Credit Card Sale (auth + capture)
/// - `2` - Credit Card Auth (authorize only)
/// - `3` - Credit Card Capture (finalize auth)
/// - `4` - Credit Card Reverse Auth
/// - `5` - Credit Card Refund
/// - `7` - eCheck Sale
/// - `8` - eCheck Refund
/// - `11` - eCheck Redeposit
/// - `12` - eCheck Account Verification
/// - `14` - Incremental Authorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
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
    /// Incremental authorization
    IncrementalAuthorization = 14,
}

/// Transaction status values per OpenAPI spec.
///
/// **OpenAPI schema:** `txnStatus`
///
/// Valid values:
/// - `0` - Pending
/// - `1` - Approved
/// - `2` - Failed
/// - `3` - Captured
/// - `4` - Settled
/// - `5` - Returned
///
/// **API Inconsistency:** POST returns string `"1"`, GET returns integer `1`.
/// Uses flexible deserializer to handle both formats.
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

/// Transaction origin values per OpenAPI spec.
///
/// **OpenAPI schema:** `txnOrigin`
///
/// Valid values:
/// - `0` - Unknown
/// - `1` - Terminal
/// - `2` - eCommerce
/// - `3` - Mail order / telephone order
/// - `4` - Apple Pay
/// - `5` - Successful 3D Secure
/// - `6` - Attempted 3D Secure
/// - `7` - Recurring (deprecated)
/// - `8` - PayFrame
/// - `9` - In writing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
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
    /// Recurring transaction (deprecated)
    Recurring = 7,
    /// Payframe transaction
    Payframe = 8,
    /// In writing
    InWriting = 9,
}

/// Transaction platform values per OpenAPI spec.
///
/// **OpenAPI schema:** `txnsPlatform`
///
/// Valid values:
/// - `APPLE` - Apple Payment Processor (Deprecated)
/// - `ELAVON` - Elavon processor (Deprecated)
/// - `FIRSTDATA` - FirstData processor (Deprecated)
/// - `GOOGLE` - Google Payment Processor (Deprecated)
/// - `VANTIV` - WorldPay eComm (VAP) processor
/// - `VCORE` - WorldPay Core processor
/// - `WELLSACH` - Wells Fargo ACH processor
/// - `WELLSFARGO` - Wells Fargo Merchant Services (Deprecated)
/// - `WFSINGLE` - WFSINGLE processor (Deprecated)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionPlatform {
    /// Apple Pay (Deprecated)
    Apple,
    /// Elavon processor (Deprecated)
    Elavon,
    /// First Data processor (Deprecated)
    #[serde(rename = "FIRSTDATA")]
    FirstData,
    /// Google Pay (Deprecated)
    Google,
    /// WorldPay eComm (VAP) processor
    #[default]
    Vantiv,
    /// WorldPay Core processor
    Vcore,
    /// Wells Fargo ACH processor
    #[serde(rename = "WELLSACH")]
    WellsAch,
    /// Wells Fargo Merchant Services (Deprecated)
    #[serde(rename = "WELLSFARGO")]
    WellsFargo,
    /// WFSINGLE processor (Deprecated)
    #[serde(rename = "WFSINGLE")]
    WfSingle,
}

/// Card-on-file type for tokenized transactions per OpenAPI spec.
///
/// **OpenAPI schema:** `CofType`
///
/// Valid values:
/// - `single` - Individual Transaction
/// - `scheduled` - Scheduled Transaction
/// - `unscheduled` - Unscheduled sequential Transaction
/// - `installment` - Installment Transaction
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
    /// Installment transaction
    Installment,
}

/// Terminal capability values per OpenAPI spec.
///
/// **OpenAPI schema:** `TerminalCapability`
///
/// Valid values:
/// - `1` - Key entry only terminal
/// - `2` - Can read magnetic stripe
/// - `3` - Integrated circuit reader
/// - `4` - Can detect contactless payment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TerminalCapability {
    /// Key entry only
    #[default]
    KeyEntryOnly = 1,
    /// Magnetic stripe reader
    MagneticStripe = 2,
    /// Integrated circuit (chip) reader
    IntegratedCircuitReader = 3,
    /// Contactless payment detection
    ContactlessPayment = 4,
}

/// Entry mode values per OpenAPI spec.
///
/// **OpenAPI schema:** `entryMode`
///
/// Valid values:
/// - `1` - Manually keyed entry
/// - `2` - Card swiped, Track 1 received
/// - `3` - Card swiped, Track 2 received
/// - `4` - Card swiped, Track 1 & 2 received
/// - `5` - Card dipped, EMV chip received
/// - `6` - Contactless card read
/// - `7` - Track Data after EMV chip failure
/// - `8` - Manually keyed after EMV chip failure
/// - `9` - ApplePay
/// - `10` - Google Pay
/// - `11` - Merchant created transaction
/// - `12` - Invoice payment
/// - `13` - Merchant created in payrix portal
/// - `14` - Invoice payment from payrix portal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EntryMode {
    /// Terminal used with key entry
    #[default]
    ManuallyKeyed = 1,
    /// Track 1 read from card swipe
    Track1Read = 2,
    /// Track 2 read from card swipe
    Track2Read = 3,
    /// Track 1 & 2 read from card swipe
    Track1And2Read = 4,
    /// EMV chip read from card dip
    EmvChipRead = 5,
    /// Contactless card read (Track or EMV data)
    ContactlessRead = 6,
    /// Track data from swipe after EMV failure
    SwipeAfterEmvFailure = 7,
    /// Manual key entry after EMV failure
    ManualAfterEmvFailure = 8,
    /// ApplePay
    ApplePay = 9,
    /// Google Pay
    GooglePay = 10,
    /// Merchant created transaction
    MerchantCreated = 11,
    /// Invoice payment
    InvoicePayment = 12,
    /// Merchant created in Payrix portal
    MerchantCreatedPortal = 13,
    /// Invoice payment from Payrix portal
    InvoicePaymentPortal = 14,
}

/// CVV status values per OpenAPI spec.
///
/// **OpenAPI schema:** `CvvStatus`
///
/// Valid values:
/// - `notPresent` - CVV is not present
/// - `illegible` - CVV is illegible
/// - `notProvided` - CVV was not provided
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CvvStatus {
    /// CVV is not present
    #[default]
    NotPresent,
    /// CVV is illegible
    Illegible,
    /// CVV was not provided
    NotProvided,
}

/// PIN entry capability values per OpenAPI spec.
///
/// **OpenAPI schema:** `PinEntryCapability`
///
/// Valid values:
/// - `unknown` - Unknown PIN Entry Capability
/// - `capable` - Terminal can accept PINs
/// - `notCapable` - Terminal cannot accept PINs
/// - `pinPadDown` - Terminal PIN Pad is down
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PinEntryCapability {
    /// Unknown PIN entry capability
    #[default]
    Unknown,
    /// Terminal can accept PINs
    Capable,
    /// Terminal cannot accept PINs
    NotCapable,
    /// Terminal PIN pad is down
    PinPadDown,
}

/// Currency conversion status per OpenAPI spec.
///
/// **OpenAPI schema:** `CurrencyConversion`
///
/// Valid values:
/// - `customerAccepted` - Customer accepted the conversion rate
/// - `customerRejected` - Customer rejected the conversion rate
/// - `notEligible` - Transaction not eligible for conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrencyConversion {
    /// Customer accepted the currency conversion rate
    #[default]
    CustomerAccepted,
    /// Customer rejected the currency conversion rate
    CustomerRejected,
    /// Transaction not eligible for currency conversion
    NotEligible,
}

/// Transaction check stage values per OpenAPI spec.
///
/// **OpenAPI schema:** `txnCheckStage`
///
/// Valid values:
/// - `activation` - Terminal activation
/// - `auth` - During transaction authorization
/// - `postauth` - After transaction authorization
/// - `capture` - During transaction capture
/// - `refund` - When processing a refund
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CheckStage {
    /// Terminal activation
    #[default]
    Activation,
    /// During transaction authorization
    Auth,
    /// After transaction authorization
    Postauth,
    /// During transaction capture
    Capture,
    /// When processing a refund
    Refund,
}

/// Unauthorized transaction reason per OpenAPI spec.
///
/// **OpenAPI schema:** `unauthReason`
///
/// Valid values:
/// - `incomplete` - Transaction incomplete
/// - `timeout` - Transaction timeout
/// - `clerkCancelled` - Cancelled by clerk
/// - `customerCancelled` - Cancelled by customer
/// - `misdispense` - Misdispense
/// - `hardwareFailure` - Hardware failure
/// - `suspectedFraud` - Suspected fraud
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

/// Copy reason values per OpenAPI spec.
///
/// **OpenAPI schema:** `CopyReason`
///
/// Valid values:
/// - `resubmission` - Resubmission of transaction
/// - `reauthorization` - Reauthorization of transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CopyReason {
    /// Resubmission
    #[default]
    Resubmission,
    /// Reauthorization
    Reauthorization,
}

/// Transaction result type per OpenAPI spec.
///
/// **OpenAPI schema:** `txnResultType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
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

/// Transaction result code per OpenAPI spec.
///
/// **OpenAPI schema:** `txnResultCode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
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
    /// 3DS passed but liability shifted
    ThreeDsAuthPassedLiabilityShifted = 32,
}

// ==================== Request Types ====================

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

/// Custom data stored in transaction description field.
///
/// Typically serialized as JSON in the `description` field.
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

// ==================== Tests ====================

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
        assert_eq!(serde_json::to_string(&TransactionType::IncrementalAuthorization).unwrap(), "14");
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
        assert_eq!(serde_json::from_str::<TransactionType>("14").unwrap(), TransactionType::IncrementalAuthorization);
    }

    #[test]
    fn transaction_type_default() {
        assert_eq!(TransactionType::default(), TransactionType::CreditCardSale);
    }

    #[test]
    fn transaction_type_invalid_value() {
        assert!(serde_json::from_str::<TransactionType>("99").is_err());
        // Value 6 is not in OpenAPI spec
        assert!(serde_json::from_str::<TransactionType>("6").is_err());
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
    fn transaction_status_deserialize_from_string() {
        // API returns strings on POST
        assert_eq!(serde_json::from_str::<TransactionStatus>("\"0\"").unwrap(), TransactionStatus::Pending);
        assert_eq!(serde_json::from_str::<TransactionStatus>("\"1\"").unwrap(), TransactionStatus::Approved);
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
        assert_eq!(serde_json::to_string(&TransactionOrigin::Unknown).unwrap(), "0");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Terminal).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Ecommerce).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TransactionOrigin::MailOrTelephoneOrder).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TransactionOrigin::ApplePay).unwrap(), "4");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Success3DS).unwrap(), "5");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Attempted3DS).unwrap(), "6");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Recurring).unwrap(), "7");
        assert_eq!(serde_json::to_string(&TransactionOrigin::Payframe).unwrap(), "8");
        assert_eq!(serde_json::to_string(&TransactionOrigin::InWriting).unwrap(), "9");
    }

    #[test]
    fn transaction_origin_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TransactionOrigin>("0").unwrap(), TransactionOrigin::Unknown);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("1").unwrap(), TransactionOrigin::Terminal);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("2").unwrap(), TransactionOrigin::Ecommerce);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("3").unwrap(), TransactionOrigin::MailOrTelephoneOrder);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("4").unwrap(), TransactionOrigin::ApplePay);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("5").unwrap(), TransactionOrigin::Success3DS);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("6").unwrap(), TransactionOrigin::Attempted3DS);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("7").unwrap(), TransactionOrigin::Recurring);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("8").unwrap(), TransactionOrigin::Payframe);
        assert_eq!(serde_json::from_str::<TransactionOrigin>("9").unwrap(), TransactionOrigin::InWriting);
    }

    #[test]
    fn transaction_origin_default() {
        assert_eq!(TransactionOrigin::default(), TransactionOrigin::Terminal);
    }

    #[test]
    fn transaction_origin_invalid_value() {
        assert!(serde_json::from_str::<TransactionOrigin>("99").is_err());
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
    }

    #[test]
    fn transaction_platform_default() {
        assert_eq!(TransactionPlatform::default(), TransactionPlatform::Vantiv);
    }

    #[test]
    fn transaction_platform_invalid_value() {
        // WORLDPAY and TDBANKCA are NOT in OpenAPI spec
        assert!(serde_json::from_str::<TransactionPlatform>("\"WORLDPAY\"").is_err());
        assert!(serde_json::from_str::<TransactionPlatform>("\"TDBANKCA\"").is_err());
        assert!(serde_json::from_str::<TransactionPlatform>("\"INVALID\"").is_err());
    }

    // ==================== CardOnFileType Tests ====================

    #[test]
    fn card_on_file_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&CardOnFileType::Single).unwrap(), "\"single\"");
        assert_eq!(serde_json::to_string(&CardOnFileType::Scheduled).unwrap(), "\"scheduled\"");
        assert_eq!(serde_json::to_string(&CardOnFileType::Unscheduled).unwrap(), "\"unscheduled\"");
        assert_eq!(serde_json::to_string(&CardOnFileType::Installment).unwrap(), "\"installment\"");
    }

    #[test]
    fn card_on_file_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"single\"").unwrap(), CardOnFileType::Single);
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"scheduled\"").unwrap(), CardOnFileType::Scheduled);
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"unscheduled\"").unwrap(), CardOnFileType::Unscheduled);
        assert_eq!(serde_json::from_str::<CardOnFileType>("\"installment\"").unwrap(), CardOnFileType::Installment);
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
        assert_eq!(serde_json::to_string(&TerminalCapability::ContactlessPayment).unwrap(), "4");
    }

    #[test]
    fn terminal_capability_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TerminalCapability>("1").unwrap(), TerminalCapability::KeyEntryOnly);
        assert_eq!(serde_json::from_str::<TerminalCapability>("2").unwrap(), TerminalCapability::MagneticStripe);
        assert_eq!(serde_json::from_str::<TerminalCapability>("3").unwrap(), TerminalCapability::IntegratedCircuitReader);
        assert_eq!(serde_json::from_str::<TerminalCapability>("4").unwrap(), TerminalCapability::ContactlessPayment);
    }

    #[test]
    fn terminal_capability_default() {
        assert_eq!(TerminalCapability::default(), TerminalCapability::KeyEntryOnly);
    }

    #[test]
    fn terminal_capability_invalid_value() {
        assert!(serde_json::from_str::<TerminalCapability>("0").is_err());
        assert!(serde_json::from_str::<TerminalCapability>("5").is_err());
    }

    // ==================== EntryMode Tests ====================

    #[test]
    fn entry_mode_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&EntryMode::ManuallyKeyed).unwrap(), "1");
        assert_eq!(serde_json::to_string(&EntryMode::Track1Read).unwrap(), "2");
        assert_eq!(serde_json::to_string(&EntryMode::Track2Read).unwrap(), "3");
        assert_eq!(serde_json::to_string(&EntryMode::Track1And2Read).unwrap(), "4");
        assert_eq!(serde_json::to_string(&EntryMode::EmvChipRead).unwrap(), "5");
        assert_eq!(serde_json::to_string(&EntryMode::ContactlessRead).unwrap(), "6");
        assert_eq!(serde_json::to_string(&EntryMode::SwipeAfterEmvFailure).unwrap(), "7");
        assert_eq!(serde_json::to_string(&EntryMode::ManualAfterEmvFailure).unwrap(), "8");
        assert_eq!(serde_json::to_string(&EntryMode::ApplePay).unwrap(), "9");
        assert_eq!(serde_json::to_string(&EntryMode::GooglePay).unwrap(), "10");
        assert_eq!(serde_json::to_string(&EntryMode::MerchantCreated).unwrap(), "11");
        assert_eq!(serde_json::to_string(&EntryMode::InvoicePayment).unwrap(), "12");
        assert_eq!(serde_json::to_string(&EntryMode::MerchantCreatedPortal).unwrap(), "13");
        assert_eq!(serde_json::to_string(&EntryMode::InvoicePaymentPortal).unwrap(), "14");
    }

    #[test]
    fn entry_mode_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<EntryMode>("1").unwrap(), EntryMode::ManuallyKeyed);
        assert_eq!(serde_json::from_str::<EntryMode>("2").unwrap(), EntryMode::Track1Read);
        assert_eq!(serde_json::from_str::<EntryMode>("3").unwrap(), EntryMode::Track2Read);
        assert_eq!(serde_json::from_str::<EntryMode>("4").unwrap(), EntryMode::Track1And2Read);
        assert_eq!(serde_json::from_str::<EntryMode>("5").unwrap(), EntryMode::EmvChipRead);
        assert_eq!(serde_json::from_str::<EntryMode>("6").unwrap(), EntryMode::ContactlessRead);
        assert_eq!(serde_json::from_str::<EntryMode>("7").unwrap(), EntryMode::SwipeAfterEmvFailure);
        assert_eq!(serde_json::from_str::<EntryMode>("8").unwrap(), EntryMode::ManualAfterEmvFailure);
        assert_eq!(serde_json::from_str::<EntryMode>("9").unwrap(), EntryMode::ApplePay);
        assert_eq!(serde_json::from_str::<EntryMode>("10").unwrap(), EntryMode::GooglePay);
        assert_eq!(serde_json::from_str::<EntryMode>("11").unwrap(), EntryMode::MerchantCreated);
        assert_eq!(serde_json::from_str::<EntryMode>("12").unwrap(), EntryMode::InvoicePayment);
        assert_eq!(serde_json::from_str::<EntryMode>("13").unwrap(), EntryMode::MerchantCreatedPortal);
        assert_eq!(serde_json::from_str::<EntryMode>("14").unwrap(), EntryMode::InvoicePaymentPortal);
    }

    #[test]
    fn entry_mode_default() {
        assert_eq!(EntryMode::default(), EntryMode::ManuallyKeyed);
    }

    #[test]
    fn entry_mode_invalid_value() {
        assert!(serde_json::from_str::<EntryMode>("0").is_err());
        assert!(serde_json::from_str::<EntryMode>("15").is_err());
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
            "authDate": 20240101,
            "status": 1,
            "description": "Test transaction",
            "fortxn": "t1_txn_98765432109876543210987",
            "fromtxn": "t1_txn_11111111111111111111111",
            "batch": "t1_bat_12345678901234567890123",
            "refunded": 0,
            "settled": "2024-01-02 12:00:00",
            "settledTotal": 10000,
            "funded": 20240103,
            "returned": "20240104",
            "origin": 2,
            "cofType": "scheduled",
            "allowPartial": 1,
            "subscription": "t1_sub_12345678901234567890123",
            "platform": "VANTIV",
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
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-01 12:00:00.0000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(txn.id.as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(txn.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(txn.token.unwrap(), "t1_tok_12345678901234567890123");
        assert_eq!(txn.txn_type, TransactionType::CreditCardSale);
        assert_eq!(txn.total, Some(10000));
        assert_eq!(txn.status, Some(TransactionStatus::Approved));
        assert_eq!(txn.origin, Some(TransactionOrigin::Ecommerce));
        assert_eq!(txn.cof_type, Some(CardOnFileType::Scheduled));
        assert_eq!(txn.platform, Some(TransactionPlatform::Vantiv));
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
        let json = r#"{"id": "t1_txn_12345678901234567890123", "type": 1, "inactive": 0, "frozen": 0}"#;
        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert!(!txn.inactive);
        assert!(!txn.frozen);
    }

    #[test]
    fn transaction_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_txn_12345678901234567890123", "type": 1, "inactive": 1, "frozen": 1}"#;
        let txn: Transaction = serde_json::from_str(json).unwrap();
        assert!(txn.inactive);
        assert!(txn.frozen);
    }

    #[test]
    fn transaction_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_txn_12345678901234567890123", "type": 1}"#;
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
