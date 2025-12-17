//! Fund types for the Payrix API.
//!
//! Funds represent account balances for entities. Each entity has a fund
//! that tracks available, pending, reserved, and total balances.
//!
//! **OpenAPI schema:** `fundsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// FUND STRUCT
// =============================================================================

/// A Payrix fund balance.
///
/// Funds track available, pending, reserved, and total balances for entities.
/// All monetary values are in **cents** (up to three decimal points per OpenAPI).
///
/// **OpenAPI schema:** `fundsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Fund {
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

    /// The identifier of the Entity that owns this Fund.
    ///
    /// **OpenAPI type:** string (ref: fundsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The currency of the Fund.
    ///
    /// Example: `"USD"`
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    /// The amount held in this Fund that is marked as reserved.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub reserved: Option<f64>,

    /// The current funds pending for the entity.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub pending: Option<f64>,

    /// The amount held in this Fund that is currently available for disbursement.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub available: Option<f64>,

    /// The total amount held in this Fund.
    ///
    /// This field is specified in cents (up to three decimal points).
    ///
    /// **OpenAPI type:** number
    #[serde(default)]
    pub total: Option<f64>,

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

    // =========================================================================
    // NESTED RELATIONS (expandable via API)
    // =========================================================================

    /// Entity reserves associated with this fund.
    ///
    /// This field is populated when expanding `entityReserves` in API requests.
    ///
    /// **OpenAPI type:** array of entityReservesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entity_reserves: Option<Vec<serde_json::Value>>,

    /// Entries associated with this fund.
    ///
    /// This field is populated when expanding `entries` in API requests.
    ///
    /// **OpenAPI type:** array of entriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub entries: Option<Vec<serde_json::Value>>,

    /// Reserve entries associated with this fund.
    ///
    /// This field is populated when expanding `reserveEntries` in API requests.
    ///
    /// **OpenAPI type:** array of reserveEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserve_entries: Option<Vec<serde_json::Value>>,

    /// Fund origins associated with this fund.
    ///
    /// This field is populated when expanding `fundOrigins` in API requests.
    ///
    /// **OpenAPI type:** array of fundOriginsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub fund_origins: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// ENUMS USED BY OTHER MODULES
// These are defined here but semantically belong to their respective modules.
// They remain here to avoid circular dependencies.
// =============================================================================

// -----------------------------------------------------------------------------
// Disbursement Enums (used by disbursement.rs, disbursement_entry.rs)
// -----------------------------------------------------------------------------

/// Disbursement status values.
///
/// Represents the current state of a disbursement in the payment lifecycle.
///
/// **OpenAPI schema:** `disbursementStatus`
///
/// Valid values:
/// - `1` - Requested
/// - `2` - Processing
/// - `3` - Processed
/// - `4` - Failed
/// - `5` - Denied
/// - `6` - Returned
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum DisbursementStatus {
    /// Disbursement has been requested but not yet started.
    #[default]
    Requested = 1,

    /// Disbursement is currently being processed.
    Processing = 2,

    /// Disbursement has been successfully processed.
    Processed = 3,

    /// Disbursement failed to process.
    Failed = 4,

    /// Disbursement was denied.
    Denied = 5,

    /// Disbursement was returned (e.g., bank rejected).
    Returned = 6,
}

/// Disbursement code values indicating the result or error type.
///
/// **OpenAPI schema:** `disbursementCode`
///
/// Valid values per OpenAPI:
/// - `pending` - Pending processing
/// - `internal` - Internal error
/// - `nsf` - Non-sufficient funds
/// - `badAccount` - Bad account
/// - `unauthorized` - Unauthorized
/// - `general` - General error
/// - `noc` - Notification of change
/// - `parameter` - Parameter error
/// - `sameDay` - Same day disbursement
/// - `transferDetails` - Transfer details issue
/// - `platform` - Platform error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisbursementCode {
    /// Pending processing.
    #[default]
    Pending,

    /// Internal error occurred.
    Internal,

    /// Non-sufficient funds in account.
    #[serde(rename = "nsf")]
    Nsf,

    /// Bad account information.
    BadAccount,

    /// Unauthorized disbursement attempt.
    Unauthorized,

    /// General/unspecified error.
    General,

    /// Notification of change received from bank.
    #[serde(rename = "noc")]
    Noc,

    /// Parameter error in request.
    Parameter,

    /// Same day disbursement.
    SameDay,

    /// Issue with transfer details.
    TransferDetails,

    /// Platform-level error.
    Platform,
}

// -----------------------------------------------------------------------------
// Payout Enums (used by payout.rs)
// -----------------------------------------------------------------------------

/// Payout schedule values indicating disbursement frequency.
///
/// **OpenAPI schema:** `payoutSchedule`
///
/// Valid values:
/// - `1` - Daily
/// - `2` - Weekly
/// - `3` - Monthly
/// - `4` - Annually
/// - `5` - Single (one-time)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum PayoutSchedule {
    /// Daily disbursements.
    Daily = 1,

    /// Weekly disbursements.
    Weekly = 2,

    /// Monthly disbursements.
    Monthly = 3,

    /// Annual disbursements.
    Annually = 4,

    /// Single/one-time disbursement.
    #[default]
    Single = 5,
}

/// Payout unit of measure for calculating disbursement amounts.
///
/// **OpenAPI schema:** `payoutUnit`
///
/// Valid values:
/// - `1` - Percent (percentage of balance)
/// - `2` - Actual (fixed amount)
/// - `3` - PercentNegative (negative percentage)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum PayoutUnit {
    /// Percentage of balance.
    Percent = 1,

    /// Fixed/actual amount.
    #[default]
    Actual = 2,

    /// Negative percentage of balance.
    PercentNegative = 3,
}

// -----------------------------------------------------------------------------
// Fee Enums (used by fee.rs)
// -----------------------------------------------------------------------------

/// Fee type values.
///
/// **OpenAPI schema:** `feeType`
///
/// Valid values:
/// - `1` - Fee (standard fee)
/// - `2` - Assessment
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum FeeType {
    /// Standard fee.
    #[default]
    Fee = 1,

    /// Assessment fee.
    Assessment = 2,
}

/// Fee rule type values.
///
/// Determines what criteria a fee rule evaluates against.
///
/// **OpenAPI schema:** `feeRuleType`
///
/// Valid values (44 total):
/// - `AVSRESULT` - AVS result check
/// - `BIN` - BIN-based rule
/// - `BUSINESS` - Business type rule
/// - `CVVRESULT` - CVV result check
/// - `CORPORATE` - Corporate card check
/// - `DYNAMICALLYROUTED` - Dynamically routed transaction
/// - `EMV` - EMV chip transaction
/// - `ENTITY` - Entity-based rule
/// - `EQUAL` - Equal comparison
/// - `FRAUDSIGHTENABLED` - FraudSight enabled check
/// - `FUNDINGCURRENCYEQUAL` - Funding currency equals
/// - `FUNDINGCURRENCYNOTEQUAL` - Funding currency not equal
/// - `FUNDINGCURRENCYMISMATCH` - Funding currency mismatch
/// - `FUNDINGENABLED` - Funding enabled check
/// - `GREATER` - Greater than comparison
/// - `IMPORTED` - Imported transaction
/// - `INTERCHANGE` - Interchange rule
/// - `INTERNATIONAL` - International transaction
/// - `ISSUERCOUNTRY` - Issuer country check
/// - `LESS` - Less than comparison
/// - `MCC` - Merchant Category Code rule
/// - `MERCHANTCOUNTRY` - Merchant country check
/// - `METHOD` - Payment method rule
/// - `METHODTYPE` - Payment method type rule
/// - `MISUSE` - Authorization misuse
/// - `NOTEQUAL` - Not equal comparison
/// - `ORIGIN` - Origin-based rule
/// - `OMNITOKENSENABLED` - Omnitokens enabled check
/// - `PLATFORM` - Platform-based rule
/// - `RELATED` - Related transaction
/// - `RELATEDCEIL` - Related ceiling
/// - `RELATEDDELAY` - Related delay
/// - `RELATEDFLOOR` - Related floor
/// - `SAMEDAY` - Same day transaction
/// - `SETTLEDCURRENCYMISMATCH` - Settled currency mismatch
/// - `SIGNED` - Signed transaction
/// - `SUBSCRIPTION` - Subscription transaction
/// - `SWIPED` - Swiped transaction
/// - `TAXFORM1099K` - 1099K tax form requirement
/// - `TYPE` - Transaction type rule
/// - `3DSRESULT` - 3DS result check
/// - `STATUS` - Status-based rule
/// - `IC_RETAIN_PASSTHRU_REFUND` - IC retain passthru refund
/// - `SOFTPOS` - SoftPOS transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FeeRuleType {
    /// AVS result-based fee rule.
    #[serde(rename = "AVSRESULT")]
    AvsResult,

    /// BIN-based fee rule.
    #[serde(rename = "BIN")]
    Bin,

    /// Business type-based fee rule.
    #[serde(rename = "BUSINESS")]
    Business,

    /// CVV result-based fee rule.
    #[serde(rename = "CVVRESULT")]
    CvvResult,

    /// Corporate card check.
    #[serde(rename = "CORPORATE")]
    Corporate,

    /// Dynamically routed transaction.
    #[serde(rename = "DYNAMICALLYROUTED")]
    DynamicallyRouted,

    /// EMV chip transaction.
    #[serde(rename = "EMV")]
    Emv,

    /// Entity-based rule.
    #[serde(rename = "ENTITY")]
    Entity,

    /// Equal comparison.
    #[serde(rename = "EQUAL")]
    Equal,

    /// FraudSight enabled check.
    #[serde(rename = "FRAUDSIGHTENABLED")]
    FraudSightEnabled,

    /// Funding currency equals.
    #[serde(rename = "FUNDINGCURRENCYEQUAL")]
    FundingCurrencyEqual,

    /// Funding currency not equal.
    #[serde(rename = "FUNDINGCURRENCYNOTEQUAL")]
    FundingCurrencyNotEqual,

    /// Funding currency mismatch.
    #[serde(rename = "FUNDINGCURRENCYMISMATCH")]
    FundingCurrencyMismatch,

    /// Funding enabled check.
    #[serde(rename = "FUNDINGENABLED")]
    FundingEnabled,

    /// Greater than comparison.
    #[serde(rename = "GREATER")]
    Greater,

    /// Imported transaction.
    #[serde(rename = "IMPORTED")]
    Imported,

    /// Interchange rule.
    #[serde(rename = "INTERCHANGE")]
    Interchange,

    /// International transaction.
    #[serde(rename = "INTERNATIONAL")]
    International,

    /// Issuer country check.
    #[serde(rename = "ISSUERCOUNTRY")]
    IssuerCountry,

    /// Less than comparison.
    #[serde(rename = "LESS")]
    Less,

    /// Merchant Category Code rule.
    #[serde(rename = "MCC")]
    Mcc,

    /// Merchant country check.
    #[serde(rename = "MERCHANTCOUNTRY")]
    MerchantCountry,

    /// Payment method rule.
    #[default]
    #[serde(rename = "METHOD")]
    Method,

    /// Payment method type rule.
    #[serde(rename = "METHODTYPE")]
    MethodType,

    /// Authorization misuse.
    #[serde(rename = "MISUSE")]
    Misuse,

    /// Not equal comparison.
    #[serde(rename = "NOTEQUAL")]
    NotEqual,

    /// Origin-based rule.
    #[serde(rename = "ORIGIN")]
    Origin,

    /// Omnitokens enabled check.
    #[serde(rename = "OMNITOKENSENABLED")]
    OmnitokensEnabled,

    /// Platform-based rule.
    #[serde(rename = "PLATFORM")]
    Platform,

    /// Related transaction.
    #[serde(rename = "RELATED")]
    Related,

    /// Related ceiling.
    #[serde(rename = "RELATEDCEIL")]
    RelatedCeil,

    /// Related delay.
    #[serde(rename = "RELATEDDELAY")]
    RelatedDelay,

    /// Related floor.
    #[serde(rename = "RELATEDFLOOR")]
    RelatedFloor,

    /// Same day transaction.
    #[serde(rename = "SAMEDAY")]
    SameDay,

    /// Settled currency mismatch.
    #[serde(rename = "SETTLEDCURRENCYMISMATCH")]
    SettledCurrencyMismatch,

    /// Signed transaction.
    #[serde(rename = "SIGNED")]
    Signed,

    /// Subscription transaction.
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,

    /// Swiped transaction.
    #[serde(rename = "SWIPED")]
    Swiped,

    /// 1099K tax form requirement.
    #[serde(rename = "TAXFORM1099K")]
    TaxForm1099K,

    /// Transaction type rule.
    #[serde(rename = "TYPE")]
    Type,

    /// 3DS result check.
    #[serde(rename = "3DSRESULT")]
    ThreeDsResult,

    /// Status-based rule.
    #[serde(rename = "STATUS")]
    Status,

    /// IC retain passthru refund.
    #[serde(rename = "IC_RETAIN_PASSTHRU_REFUND")]
    IcRetainPassthruRefund,

    /// SoftPOS transaction.
    #[serde(rename = "SOFTPOS")]
    SoftPos,
}

/// Fee application scope.
///
/// Determines where a fee rule should apply.
///
/// **OpenAPI schema:** `feeApplication`
///
/// Valid values:
/// - `both` - Rule applies to fee and collection calculation
/// - `fee` - Rule applies only to the fee itself
/// - `collection` - Rule used only for collection calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeeApplication {
    /// Rule applies to both fee and collection calculation.
    #[default]
    Both,

    /// Rule applies only to the fee itself.
    Fee,

    /// Rule used only when calculating a collection.
    Collection,
}

/// Fee unit of measure (feeUm in OpenAPI).
///
/// **OpenAPI schema:** `feeUnit`
///
/// Valid values:
/// - `1` - Percent (percentage-based fee)
/// - `2` - Fixed (fixed amount fee)
/// - `3` - Surcharge
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum FeeUnit {
    /// Percentage-based fee.
    Percent = 1,

    /// Fixed amount fee.
    #[default]
    Fixed = 2,

    /// Surcharge fee.
    Surcharge = 3,
}

/// Fee collection scope indicating when fees are charged.
///
/// **OpenAPI schema:** `feeCollection`
///
/// Valid values:
/// - `1` - Transaction (per transaction)
/// - `2` - TransactionTaxId (per entity tax ID)
/// - `3` - TransactionMerchant (per merchant)
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum FeeCollection {
    /// Per transaction fee collection.
    #[default]
    Transaction = 1,

    /// Per entity tax ID fee collection.
    TransactionTaxId = 2,

    /// Per merchant fee collection.
    TransactionMerchant = 3,
}

/// Fee schedule trigger values.
///
/// Determines when a fee is triggered to be charged.
///
/// **OpenAPI schema:** `feeSchedule`
///
/// Valid values:
/// - `1` - Days (fee charged daily)
/// - `2` - Weeks (fee charged weekly)
/// - `3` - Months (fee charged monthly)
/// - `4` - Years (fee charged annually)
/// - `5` - Single (one-off charge)
/// - `6` - Auth (at authorization time)
/// - `7` - Capture (at capture time)
/// - `8` - Refund (when refund processed)
/// - `9` - Board (when merchant boarded)
/// - `10` - Payout (when payout processed)
/// - `11` - Chargeback (when chargeback occurs)
/// - `12` - Overdraft (on overdraft)
/// - `13` - Interchange (on interchange assessment)
/// - `14` - Processor (on processor fee)
/// - `15` - ACH failure
/// - `16` - Account (bank account verification)
/// - `17` - Sift (fraud checking)
/// - `18` - Adjustment (on adjustment)
/// - `19` - Retrieval (chargeback retrieval)
/// - `20` - Arbitration (chargeback arbitration)
/// - `21` - eCheck Sale
/// - `22` - eCheck Refund
/// - `23` - eCheck Return
/// - `24` - Settlement
/// - `25` - Misuse (auth misuse)
/// - `26` - Profit share
/// - `27` - Unauth (auth reversal)
/// - `28` - Disbursement NOC
/// - `29` - Transaction NOC
/// - `30` - eCheck failure return
/// - `31` - eCheck NSF return
/// - `32` - Currency conversion
/// - `33` - Terminal transaction
/// - `34` - Reverse payout
/// - `35` - Partial reverse payout
/// - `43` - Payment check
/// - `44` - Payment update
/// - `45` - Payment group check
/// - `46` - Payment group update
/// - `47` - Entry refund
/// - `51` - Statement Payment
/// - `52` - Merchant Created
/// - `53`-`93` - Various verification and compliance fees
/// - `96` - Omnitokens Monthly
/// - `200`-`284` - Valutec external fees
/// - `400`-`410` - RevBoost and verification fees
/// - `601`-`608` - Tier-based fees
/// - `701`-`705` - Network annual fees
/// - `801`-`807` - Equipment fees
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(i32)]
pub enum FeeSchedule {
    /// Fee charged every day.
    #[default]
    Days = 1,
    /// Fee charged every week.
    Weeks = 2,
    /// Fee charged every month.
    Months = 3,
    /// Fee charged every year.
    Years = 4,
    /// One-off fee charge.
    Single = 5,
    /// Fee triggered at authorization time.
    Auth = 6,
    /// Fee triggered at capture time.
    Capture = 7,
    /// Fee triggered when refund processed.
    Refund = 8,
    /// Fee triggered when merchant boarded.
    Board = 9,
    /// Fee triggered when payout processed.
    Payout = 10,
    /// Fee triggered on chargeback.
    Chargeback = 11,
    /// Fee triggered on overdraft.
    Overdraft = 12,
    /// Fee triggered on interchange assessment.
    Interchange = 13,
    /// Fee triggered on processor fee.
    Processor = 14,
    /// Fee triggered on ACH failure.
    AchFailure = 15,
    /// Fee triggered on bank account verification.
    Account = 16,
    /// Fee triggered on Sift fraud check.
    Sift = 17,
    /// Fee triggered on adjustment.
    Adjustment = 18,
    /// Fee triggered on chargeback retrieval.
    Retrieval = 19,
    /// Fee triggered on chargeback arbitration.
    Arbitration = 20,
    /// Fee triggered on eCheck sale.
    EcheckSale = 21,
    /// Fee triggered on eCheck refund.
    EcheckRefund = 22,
    /// Fee triggered on eCheck return.
    EcheckReturn = 23,
    /// Fee triggered on settlement.
    Settlement = 24,
    /// Fee triggered on auth misuse.
    Misuse = 25,
    /// Fee triggered on profit share.
    ProfitShare = 26,
    /// Fee triggered on auth reversal.
    Unauth = 27,
    /// Fee triggered on disbursement NOC.
    DisbursementNoc = 28,
    /// Fee triggered on transaction NOC.
    TransactionNoc = 29,
    /// Fee triggered on eCheck failure return.
    EcheckFailureReturn = 30,
    /// Fee triggered on eCheck NSF return.
    EcheckNsfReturn = 31,
    /// Fee triggered on currency conversion.
    CurrencyConversion = 32,
    /// Fee triggered on terminal transaction.
    TerminalTransaction = 33,
    /// Fee triggered on payout reversal.
    ReversePayout = 34,
    /// Fee triggered on partial payout reversal.
    PartialReversePayout = 35,
    /// Fee triggered on payment check.
    PaymentCheck = 43,
    /// Fee triggered on payment update.
    PaymentUpdate = 44,
    /// Fee triggered on payment group check.
    PaymentGroupCheck = 45,
    /// Fee triggered on payment group update.
    PaymentGroupUpdate = 46,
    /// Fee triggered on entry refund.
    EntryRefund = 47,
    /// Fee triggered on statement payment.
    StatementPayment = 51,
    /// Fee triggered when merchant created.
    MerchantCreated = 52,
    /// Realtime Business OFAC Search.
    RealtimeBusinessSearch = 53,
    /// Realtime Member OFAC Search.
    RealtimeMemberSearch = 54,
    /// MasterCard MATCH check.
    MasterCardMatch = 55,
    /// Business Instant ID (KYB).
    BusinessInstantId = 56,
    /// Consumer Instant ID (KYC).
    ConsumerInstantId = 57,
    /// ThreatMetrix check.
    ThreatMetrix = 58,
    /// LegitScript Registration.
    LegitScriptRegistration = 59,
    /// Equifax Consumer Report.
    EquifaxConsumerReport = 60,
    /// CharityCheck verification.
    CharityCheck = 61,
    /// Internal Decision V2.
    InternalDecisionV2 = 62,
    /// TIN Check.
    TinCheck = 63,
    /// Equifax Commercial Report.
    EquifaxCommercialReport = 64,
    /// LegitScript Merchant Check.
    LegitScriptMerchantCheck = 65,
    /// Plaid services.
    Plaid = 66,
    /// Statement Reversal.
    StatementReversal = 67,
    /// GIACT eCheck Verification.
    GiactEcheckVerification = 68,
    /// GIACT Account Verification.
    GiactAccountVerification = 69,
    /// Boarding Decision.
    BoardingDecision = 70,
    /// Transaction Risk Decision.
    TransactionRiskDecision = 71,
    /// FANF (Fixed Acquirer Network Fee).
    Fanf = 72,
    /// MCLocation (Mastercard Location Fee).
    McLocation = 73,
    /// Visa Integrity Fee.
    VisaIntegrity = 74,
    /// SaferPayments Basic.
    SaferPaymentsBasic = 75,
    /// SaferPayments Managed.
    SaferPaymentsManaged = 76,
    /// SaferPayments PCI Non-Validation.
    SaferPaymentsPciNonValidation = 77,
    /// Omnitokens Volume.
    OmnitokensVolume = 78,
    /// Payout Return.
    PayoutReturn = 79,
    /// Payout Partial Return.
    PayoutPartialReturn = 80,
    /// Revenue Share.
    RevenueShare = 81,
    /// Card Settlement.
    CardSettlement = 82,
    /// eCheck Settlement.
    EcheckSettlement = 83,
    /// Revenue Share from Card.
    RevenueShareFromCard = 84,
    /// Revenue Share from eCheck.
    RevenueShareFromEcheck = 85,
    /// Revenue Share Payout.
    RevenueSharePayout = 86,
    /// Plaid Identity Match.
    PlaidIdentityMatch = 88,
    /// Transaction Plaid Identity Match.
    TxnPlaidIdentityMatch = 89,
    /// Plaid Get Identity.
    PlaidGetIdentity = 90,
    /// Transaction Plaid Get Identity.
    TxnPlaidGetIdentity = 91,
    /// Plaid Get Auth.
    PlaidGetAuth = 92,
    /// Transaction Plaid Get Auth.
    TxnPlaidGetAuth = 93,
    /// Omnitokens Monthly Fee.
    OmnitokensMonthly = 96,
    /// Valutec Essential Gift.
    ValutecEssentialGift = 200,
    /// Valutec Essential Monthly Transaction.
    ValutecEssentialMonthlyTxn = 201,
    /// Valutec Digital Gift Plus Package.
    ValutecDigitalGiftPlusPackage = 202,
    /// Valutec Digital Gift Plus Package Transaction.
    ValutecDigitalGiftPlusPackageTxn = 203,
    /// Valutec Loyalty Plus Package.
    ValutecLoyaltyPlusPackage = 204,
    /// Valutec Loyalty Plus Package Transaction.
    ValutecLoyaltyPlusPackageTxn = 205,
    /// Valutec Transaction Fee.
    ValutecTransactionFee = 206,
    /// Valutec Setup Fee.
    ValutecSetupFee = 207,
    /// Valutec Gift ACH Pooling.
    ValutecGiftAchPooling = 208,
    /// Valutec Jump Start Kit.
    ValutecJumpStartKit = 209,
    /// Valutec Launch Box Kit.
    ValutecLaunchBoxKit = 210,
    /// Valutec 500 Custom Cards.
    Valutec500CustomCards = 211,
    /// Valutec Maintenance Fee.
    ValutecMaintenanceFee = 212,
    /// Valutec Digital Gift Plus Package ME.
    ValutecDigitalGiftPlusPackageMe = 213,
    /// EFE MWC Residual Atelio.
    EfeMwcResidualAtelio = 214,
    /// EFE MWC Residual Parafin.
    EfeMwcResidualParafin = 215,
    /// EFE MWC Billing.
    EfeMwcBilling = 216,
    /// FraudSight CNP Decision.
    FraudsightCnpDecision = 217,
    /// FraudSight CP Decision.
    FraudsightCpDecision = 218,
    /// Valutec Overage Transaction Fee.
    ValutecOverageTransactionFee = 230,
    /// Valutec Standard Mobile Pass Monthly Apple.
    ValutecStandardMobilePassMonthlyApple = 231,
    /// Valutec NFC Mobile Pass Monthly.
    ValutecNfcMobilePassMonthly = 232,
    /// Valutec Standard Mobile Pass Monthly Google.
    ValutecStandardMobilePassMonthlyGoogle = 233,
    /// Valutec Online Gift Website Monthly.
    ValutecOnlineGiftWebsiteMonthly = 234,
    /// Valutec Marketing 360 Monthly.
    ValutecMarketing360Monthly = 235,
    /// Valutec Online Gift Card Volume Fee.
    ValutecOnlineGiftCardVolumeFee = 236,
    /// Valutec Social Sharing Monthly.
    ValutecSocialSharingMonthly = 237,
    /// Valutec Digicard Monthly.
    ValutecDigicardMonthly = 238,
    /// Valutec Loyalty Standard Monthly.
    ValutecLoyaltyStandardMonthly = 239,
    /// Valutec Auto Rewards LPR Monthly.
    ValutecAutoRewardsLprMonthly = 240,
    /// Valutec OneCard Setup Fee.
    ValutecOnecardSetupFee = 241,
    /// Valutec OneCard Monthly Fee.
    ValutecOnecardMonthlyFee = 242,
    /// Valutec Monthly Fee Choice.
    ValutecMonthlyFeeChoice = 243,
    /// Valutec System Access.
    ValutecSystemAccess = 244,
    /// Valutec Transaction.
    ValutecTransaction = 245,
    /// Valutec Monthly Fee Launchbox.
    ValutecMonthlyFeeLaunchbox = 246,
    /// Valutec Monthly Fee Jumpstart.
    ValutecMonthlyFeeJumpstart = 247,
    /// Valutec OneCard Physical Fulfillment Fee.
    ValutecOnecardPhysicalFulfillmentFee = 248,
    /// Valutec OneCard Virtual Fulfillment Fee.
    ValutecOnecardVirtualFulfillmentFee = 249,
    /// Valutec Customized Reporting.
    ValutecCustomizedReporting = 250,
    /// Valutec Transaction File Feed.
    ValutecTransactionFileFeed = 251,
    /// Valutec 250 Custom Cards.
    Valutec250CustomCards = 252,
    /// Valutec 1000 Custom Cards.
    Valutec1000CustomCards = 253,
    /// Valutec 2500 Custom Cards.
    Valutec2500CustomCards = 254,
    /// Valutec 5000 Custom Cards.
    Valutec5000CustomCards = 255,
    /// Valutec 10000 Custom Cards.
    Valutec10000CustomCards = 256,
    /// Valutec 15000 Custom Cards.
    Valutec15000CustomCards = 257,
    /// Valutec 20000 Custom Cards.
    Valutec20000CustomCards = 258,
    /// Valutec 25000 Custom Cards.
    Valutec25000CustomCards = 259,
    /// Valutec 250 Express Cards.
    Valutec250ExpressCards = 260,
    /// Valutec 500 Express Cards.
    Valutec500ExpressCards = 261,
    /// Valutec 1000 Express Cards.
    Valutec1000ExpressCards = 262,
    /// Valutec 2500 Express Cards.
    Valutec2500ExpressCards = 263,
    /// Valutec 5000 Express Cards.
    Valutec5000ExpressCards = 264,
    /// Valutec 10000 Express Cards.
    Valutec10000ExpressCards = 265,
    /// Valutec Specialty Cards.
    ValutecSpecialtyCards = 266,
    /// Valutec Keytags.
    ValutecKeytags = 267,
    /// Valutec Sleeves Carriers Hangers.
    ValutecSleevesCarriersHangers = 268,
    /// Valutec Signature Panel.
    ValutecSignaturePanel = 269,
    /// Valutec 4x4 Printing.
    Valutec4x4Printing = 270,
    /// Valutec Specialty Finish.
    ValutecSpecialtyFinish = 271,
    /// Valutec Pantone.
    ValutecPantone = 272,
    /// Valutec PIN Scratch.
    ValutecPinScratch = 273,
    /// Valutec Merchandise.
    ValutecMerchandise = 274,
    /// Valutec Gift Card Design Setup Fee.
    ValutecGiftCardDesignSetupFee = 275,
    /// Valutec Shipping Handling.
    ValutecShippingHandling = 276,
    /// Valutec Rush Fees.
    ValutecRushFees = 277,
    /// Valutec Sequencing.
    ValutecSequencing = 278,
    /// Valutec Wholesale Fulfillment Fee.
    ValutecWholesaleFulfillmentFee = 279,
    /// Valutec Miscellaneous Fees.
    ValutecMiscellaneousFees = 280,
    /// Valutec Hosting Fee.
    ValutecHostingFee = 281,
    /// Valutec Storecard Monthly Fee.
    ValutecStorecardMonthlyFee = 282,
    /// Valutec Gift ACH Fee Monthly.
    ValutecGiftAchFeeMonthly = 283,
    /// Valutec OneCard Monthly Fee Storecard.
    ValutecOnecardMonthlyFeeStorecard = 284,
    /// RevBoost Embedded TMS Monthly.
    RevboostEmbeddedTmsMonthly = 400,
    /// RevBoost Embedded TMS PPT.
    RevboostEmbeddedTmsPpt = 401,
    /// Transaction ThreatMetrix.
    TxnThreatMetrix = 402,
    /// ThreatMetrix Emailage.
    ThreatMetrixEmailage = 403,
    /// ThreatMetrix Fraud Point.
    ThreatMetrixFraudPoint = 404,
    /// GIACT Inquiry.
    GiactInquiry = 405,
    /// GIACT Transaction gAuthenticate.
    GiactTxnGauthenticate = 406,
    /// Trulioo IDV.
    TruliooIdv = 407,
    /// Trulioo AML LDV.
    TruliooAmlldv = 408,
    /// Trulioo Business Verification.
    TruliooBusinessVerification = 409,
    /// ThreatMetrix Phone Finder.
    ThreatMetrixPhoneFinder = 410,
    /// Tier Non-Qualified Count.
    TierNonQualifiedCount = 601,
    /// Tier Qualified Count.
    TierQualifiedCount = 602,
    /// Tier Mid-Qualified Count.
    TierMidQualifiedCount = 603,
    /// Tier High Risk Count.
    TierHighRiskCount = 604,
    /// Tier Non-Qualified Volume.
    TierNonQualifiedVolume = 605,
    /// Tier Qualified Volume.
    TierQualifiedVolume = 606,
    /// Tier Mid-Qualified Volume.
    TierMidQualifiedVolume = 607,
    /// Tier High Risk Volume.
    TierHighRiskVolume = 608,
    /// NYCE Annual Fee.
    NyceAnnualFee = 701,
    /// PULSE Annual Fee.
    PulseAnnualFee = 702,
    /// CU24 Annual Fee.
    Cu24AnnualFee = 703,
    /// STAR Annual Fee.
    StarAnnualFee = 704,
    /// ACCEL Annual Fee.
    AccelAnnualFee = 705,
    /// Ingenico Link 2500 (Canada).
    IngenicoLink2500 = 801,
    /// Ingenico Lane 3600 Standard (Canada).
    IngenicoLane3600Std = 802,
    /// Ingenico Lane 3600 Deluxe (Canada).
    IngenicoLane3600Dlx = 803,
    /// Ingenico Lane 7000 Standard (Canada).
    IngenicoLane7000Std = 804,
    /// Ingenico Lane 7000 Deluxe (Canada).
    IngenicoLane7000Dlx = 805,
    /// Ingenico Move 5000 (Canada).
    IngenicoMove5000 = 806,
    /// Payrix Equipment Setup Fee.
    PayrixEquipmentSetupFee = 807,
}

// -----------------------------------------------------------------------------
// Batch Enums (used by batch.rs)
// -----------------------------------------------------------------------------

/// Batch status values.
///
/// **OpenAPI schema:** `batchStatus`
///
/// Valid values:
/// - `open` - Batch is open for new transactions
/// - `closed` - Batch is closed to new transactions and ready to be sent to processor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchStatus {
    /// Batch is open and can accept more transactions.
    #[default]
    Open,

    /// Batch is closed to new transactions and ready to be sent to processor.
    Closed,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Fund Struct Tests ====================

    #[test]
    fn fund_deserialize_full() {
        let json = r#"{
            "id": "t1_fnd_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-04-01 12:00:00.0000",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "currency": "USD",
            "reserved": 5000.0,
            "pending": 25000.5,
            "available": 100000.0,
            "total": 130000.5,
            "inactive": 0,
            "frozen": 0
        }"#;

        let fund: Fund = serde_json::from_str(json).unwrap();
        assert_eq!(fund.id.as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(fund.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(fund.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(fund.currency, Some("USD".to_string()));
        assert_eq!(fund.available, Some(100000.0));
        assert_eq!(fund.pending, Some(25000.5));
        assert_eq!(fund.reserved, Some(5000.0));
        assert_eq!(fund.total, Some(130000.5));
        assert!(!fund.inactive);
        assert!(!fund.frozen);
    }

    #[test]
    fn fund_deserialize_minimal() {
        let json = r#"{"id": "t1_fnd_12345678901234567890123"}"#;
        let fund: Fund = serde_json::from_str(json).unwrap();
        assert_eq!(fund.id.as_str(), "t1_fnd_12345678901234567890123");
        assert!(fund.creator.is_none());
        assert!(fund.available.is_none());
        assert!(!fund.inactive);
    }

    #[test]
    fn fund_bool_from_int() {
        let json = r#"{"id": "t1_fnd_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let fund: Fund = serde_json::from_str(json).unwrap();
        assert!(fund.inactive);
        assert!(fund.frozen);
    }

    #[test]
    fn fund_fractional_cents() {
        // OpenAPI says amounts support "up to three decimal points"
        let json = r#"{
            "id": "t1_fnd_12345678901234567890123",
            "available": 100000.123,
            "pending": 25000.456,
            "total": 125000.579
        }"#;
        let fund: Fund = serde_json::from_str(json).unwrap();
        assert_eq!(fund.available, Some(100000.123));
        assert_eq!(fund.pending, Some(25000.456));
        assert_eq!(fund.total, Some(125000.579));
    }

    // ==================== DisbursementStatus Tests ====================

    #[test]
    fn disbursement_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&DisbursementStatus::Requested).unwrap(), "1");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Processing).unwrap(), "2");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Processed).unwrap(), "3");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Failed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Denied).unwrap(), "5");
        assert_eq!(serde_json::to_string(&DisbursementStatus::Returned).unwrap(), "6");
    }

    #[test]
    fn disbursement_status_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("1").unwrap(),
            DisbursementStatus::Requested
        );
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("2").unwrap(),
            DisbursementStatus::Processing
        );
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("3").unwrap(),
            DisbursementStatus::Processed
        );
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("4").unwrap(),
            DisbursementStatus::Failed
        );
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("5").unwrap(),
            DisbursementStatus::Denied
        );
        assert_eq!(
            serde_json::from_str::<DisbursementStatus>("6").unwrap(),
            DisbursementStatus::Returned
        );
    }

    #[test]
    fn disbursement_status_default() {
        assert_eq!(DisbursementStatus::default(), DisbursementStatus::Requested);
    }

    // ==================== DisbursementCode Tests ====================

    #[test]
    fn disbursement_code_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&DisbursementCode::Pending).unwrap(), "\"pending\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Internal).unwrap(), "\"internal\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Nsf).unwrap(), "\"nsf\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::BadAccount).unwrap(), "\"badAccount\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Unauthorized).unwrap(), "\"unauthorized\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::General).unwrap(), "\"general\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Noc).unwrap(), "\"noc\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Parameter).unwrap(), "\"parameter\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::SameDay).unwrap(), "\"sameDay\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::TransferDetails).unwrap(), "\"transferDetails\"");
        assert_eq!(serde_json::to_string(&DisbursementCode::Platform).unwrap(), "\"platform\"");
    }

    #[test]
    fn disbursement_code_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"pending\"").unwrap(),
            DisbursementCode::Pending
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"internal\"").unwrap(),
            DisbursementCode::Internal
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"nsf\"").unwrap(),
            DisbursementCode::Nsf
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"badAccount\"").unwrap(),
            DisbursementCode::BadAccount
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"unauthorized\"").unwrap(),
            DisbursementCode::Unauthorized
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"general\"").unwrap(),
            DisbursementCode::General
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"noc\"").unwrap(),
            DisbursementCode::Noc
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"parameter\"").unwrap(),
            DisbursementCode::Parameter
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"sameDay\"").unwrap(),
            DisbursementCode::SameDay
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"transferDetails\"").unwrap(),
            DisbursementCode::TransferDetails
        );
        assert_eq!(
            serde_json::from_str::<DisbursementCode>("\"platform\"").unwrap(),
            DisbursementCode::Platform
        );
    }

    #[test]
    fn disbursement_code_default() {
        assert_eq!(DisbursementCode::default(), DisbursementCode::Pending);
    }

    // ==================== PayoutSchedule Tests ====================

    #[test]
    fn payout_schedule_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PayoutSchedule::Daily).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Weekly).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Monthly).unwrap(), "3");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Annually).unwrap(), "4");
        assert_eq!(serde_json::to_string(&PayoutSchedule::Single).unwrap(), "5");
    }

    #[test]
    fn payout_schedule_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PayoutSchedule>("1").unwrap(), PayoutSchedule::Daily);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("2").unwrap(), PayoutSchedule::Weekly);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("3").unwrap(), PayoutSchedule::Monthly);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("4").unwrap(), PayoutSchedule::Annually);
        assert_eq!(serde_json::from_str::<PayoutSchedule>("5").unwrap(), PayoutSchedule::Single);
    }

    #[test]
    fn payout_schedule_default() {
        assert_eq!(PayoutSchedule::default(), PayoutSchedule::Single);
    }

    // ==================== PayoutUnit Tests ====================

    #[test]
    fn payout_unit_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&PayoutUnit::Percent).unwrap(), "1");
        assert_eq!(serde_json::to_string(&PayoutUnit::Actual).unwrap(), "2");
        assert_eq!(serde_json::to_string(&PayoutUnit::PercentNegative).unwrap(), "3");
    }

    #[test]
    fn payout_unit_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<PayoutUnit>("1").unwrap(), PayoutUnit::Percent);
        assert_eq!(serde_json::from_str::<PayoutUnit>("2").unwrap(), PayoutUnit::Actual);
        assert_eq!(serde_json::from_str::<PayoutUnit>("3").unwrap(), PayoutUnit::PercentNegative);
    }

    #[test]
    fn payout_unit_default() {
        assert_eq!(PayoutUnit::default(), PayoutUnit::Actual);
    }

    // ==================== FeeType Tests ====================

    #[test]
    fn fee_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeType::Fee).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeType::Assessment).unwrap(), "2");
    }

    #[test]
    fn fee_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeType>("1").unwrap(), FeeType::Fee);
        assert_eq!(serde_json::from_str::<FeeType>("2").unwrap(), FeeType::Assessment);
    }

    #[test]
    fn fee_type_default() {
        assert_eq!(FeeType::default(), FeeType::Fee);
    }

    // ==================== FeeRuleType Tests ====================

    #[test]
    fn fee_rule_type_serialize_common_variants() {
        // Per OpenAPI: FeeRuleType values are UPPERCASE
        assert_eq!(serde_json::to_string(&FeeRuleType::Method).unwrap(), "\"METHOD\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::Bin).unwrap(), "\"BIN\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::AvsResult).unwrap(), "\"AVSRESULT\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::Business).unwrap(), "\"BUSINESS\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::CvvResult).unwrap(), "\"CVVRESULT\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::Interchange).unwrap(), "\"INTERCHANGE\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::International).unwrap(), "\"INTERNATIONAL\"");
        assert_eq!(serde_json::to_string(&FeeRuleType::Mcc).unwrap(), "\"MCC\"");
    }

    #[test]
    fn fee_rule_type_deserialize_common_variants() {
        // Per OpenAPI: FeeRuleType values are UPPERCASE
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"METHOD\"").unwrap(), FeeRuleType::Method);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"BIN\"").unwrap(), FeeRuleType::Bin);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"AVSRESULT\"").unwrap(), FeeRuleType::AvsResult);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"BUSINESS\"").unwrap(), FeeRuleType::Business);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"CVVRESULT\"").unwrap(), FeeRuleType::CvvResult);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"INTERCHANGE\"").unwrap(), FeeRuleType::Interchange);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"INTERNATIONAL\"").unwrap(), FeeRuleType::International);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"MCC\"").unwrap(), FeeRuleType::Mcc);
    }

    #[test]
    fn fee_rule_type_deserialize_special_variants() {
        // Special cases with numbers and underscores
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"3DSRESULT\"").unwrap(), FeeRuleType::ThreeDsResult);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"IC_RETAIN_PASSTHRU_REFUND\"").unwrap(), FeeRuleType::IcRetainPassthruRefund);
        assert_eq!(serde_json::from_str::<FeeRuleType>("\"TAXFORM1099K\"").unwrap(), FeeRuleType::TaxForm1099K);
    }

    #[test]
    fn fee_rule_type_default() {
        assert_eq!(FeeRuleType::default(), FeeRuleType::Method);
    }

    // ==================== FeeApplication Tests ====================

    #[test]
    fn fee_application_serialize_all_variants() {
        // Per OpenAPI: FeeApplication values are lowercase
        assert_eq!(serde_json::to_string(&FeeApplication::Both).unwrap(), "\"both\"");
        assert_eq!(serde_json::to_string(&FeeApplication::Fee).unwrap(), "\"fee\"");
        assert_eq!(serde_json::to_string(&FeeApplication::Collection).unwrap(), "\"collection\"");
    }

    #[test]
    fn fee_application_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeApplication>("\"both\"").unwrap(), FeeApplication::Both);
        assert_eq!(serde_json::from_str::<FeeApplication>("\"fee\"").unwrap(), FeeApplication::Fee);
        assert_eq!(serde_json::from_str::<FeeApplication>("\"collection\"").unwrap(), FeeApplication::Collection);
    }

    #[test]
    fn fee_application_default() {
        assert_eq!(FeeApplication::default(), FeeApplication::Both);
    }

    // ==================== FeeUnit Tests ====================

    #[test]
    fn fee_unit_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeUnit::Percent).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeUnit::Fixed).unwrap(), "2");
        assert_eq!(serde_json::to_string(&FeeUnit::Surcharge).unwrap(), "3");
    }

    #[test]
    fn fee_unit_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeUnit>("1").unwrap(), FeeUnit::Percent);
        assert_eq!(serde_json::from_str::<FeeUnit>("2").unwrap(), FeeUnit::Fixed);
        assert_eq!(serde_json::from_str::<FeeUnit>("3").unwrap(), FeeUnit::Surcharge);
    }

    #[test]
    fn fee_unit_default() {
        assert_eq!(FeeUnit::default(), FeeUnit::Fixed);
    }

    // ==================== FeeCollection Tests ====================

    #[test]
    fn fee_collection_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&FeeCollection::Transaction).unwrap(), "1");
        assert_eq!(serde_json::to_string(&FeeCollection::TransactionTaxId).unwrap(), "2");
        assert_eq!(serde_json::to_string(&FeeCollection::TransactionMerchant).unwrap(), "3");
    }

    #[test]
    fn fee_collection_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<FeeCollection>("1").unwrap(), FeeCollection::Transaction);
        assert_eq!(serde_json::from_str::<FeeCollection>("2").unwrap(), FeeCollection::TransactionTaxId);
        assert_eq!(serde_json::from_str::<FeeCollection>("3").unwrap(), FeeCollection::TransactionMerchant);
    }

    #[test]
    fn fee_collection_default() {
        assert_eq!(FeeCollection::default(), FeeCollection::Transaction);
    }

    // ==================== BatchStatus Tests ====================

    #[test]
    fn batch_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&BatchStatus::Open).unwrap(), "\"open\"");
        assert_eq!(serde_json::to_string(&BatchStatus::Closed).unwrap(), "\"closed\"");
    }

    #[test]
    fn batch_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<BatchStatus>("\"open\"").unwrap(), BatchStatus::Open);
        assert_eq!(serde_json::from_str::<BatchStatus>("\"closed\"").unwrap(), BatchStatus::Closed);
    }

    #[test]
    fn batch_status_default() {
        assert_eq!(BatchStatus::default(), BatchStatus::Open);
    }
}
