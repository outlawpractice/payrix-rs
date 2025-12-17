//! Chargeback types for the Payrix API.
//!
//! Chargebacks represent disputed transactions and their resolution process.
//!
//! **OpenAPI schema:** `chargebacksResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// Enums
// =============================================================================

/// Chargeback cycle/stage values per OpenAPI spec.
///
/// **OpenAPI schema:** `cycle`
///
/// Valid values:
/// - `retrieval` - Initial request from the issuer for more information.
/// - `first` - First Chargeback from issuer for this Transaction.
/// - `arbitration` - Arbitration is being sought for this Chargeback.
/// - `reversal` - Chargeback was reversed.
/// - `representment` - Merchant is being represented to the issuer.
/// - `preArbitration` - Chargeback is no longer representable.
/// - `arbitrationLost` - Arbitration lost.
/// - `arbitrationSplit` - Arbitration split.
/// - `arbitrationWon` - Arbitration won.
/// - `issuerAcceptPreArbitration` - Issuer accepted the pre-arbitration response.
/// - `issuerDeclinedPreArbitration` - Issuer declined pre-arbitration.
/// - `responseToIssuerPreArbitration` - Response to issuer pre-arbitration.
/// - `merchantAcceptedPreArbitration` - Merchant accepted the pre-arbitration response.
/// - `merchantDeclinedPreArbitration` - Merchant declined the pre-arbitration response.
/// - `preCompliance` - Pre-compliance.
/// - `compliance` - Compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargebackCycle {
    /// Initial request from the issuer for more information on this Transaction.
    #[default]
    Retrieval,
    /// First Chargeback from issuer for this Transaction.
    First,
    /// Arbitration is being sought for this Chargeback.
    Arbitration,
    /// Chargeback was reversed.
    Reversal,
    /// Merchant is being represented to the issuer with the Chargeback response posted.
    Representment,
    /// Chargeback is no longer representable. Merchant must choose to accept or arbitrate.
    PreArbitration,
    /// Arbitration lost.
    ArbitrationLost,
    /// Arbitration split.
    ArbitrationSplit,
    /// Arbitration won.
    ArbitrationWon,
    /// Issuer accepted the pre-arbitration response.
    IssuerAcceptPreArbitration,
    /// Issuer declined pre-arbitration.
    IssuerDeclinedPreArbitration,
    /// Response to issuer pre-arbitration.
    ResponseToIssuerPreArbitration,
    /// Merchant accepted the pre-arbitration response.
    MerchantAcceptedPreArbitration,
    /// Merchant declined the pre-arbitration response.
    MerchantDeclinedPreArbitration,
    /// Pre-compliance.
    PreCompliance,
    /// Compliance.
    Compliance,
}

/// Chargeback status values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackStatus`
///
/// Valid values:
/// - `open` - Chargeback is open, responses may be submitted.
/// - `closed` - Chargeback is closed, responses may no longer be submitted.
/// - `won` - Chargeback won.
/// - `lost` - Chargeback lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackStatusValue {
    /// Chargeback is open, responses may be submitted.
    #[default]
    Open,
    /// Chargeback is closed, responses may no longer be submitted.
    Closed,
    /// Chargeback won.
    Won,
    /// Chargeback lost.
    Lost,
}

/// Chargeback platform values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackPlatform`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChargebackPlatform {
    /// VCore platform.
    #[serde(rename = "VCORE")]
    VCore,
    /// TSYS platform.
    Tsys,
    /// First Data platform.
    #[serde(rename = "FIRSTDATA")]
    FirstData,
    /// Elavon platform.
    Elavon,
}

/// Chargeback payment method values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackPaymentMethod`
///
/// Valid values:
/// - `0` - No payment method specified.
/// - `1` - American Express.
/// - `2` - Visa.
/// - `3` - MasterCard.
/// - `4` - Diners Club.
/// - `5` - Discover.
/// - `6` - PayPal.
/// - `7` - Debit card.
/// - `8` - Checking account.
/// - `9` - Savings account.
/// - `10` - Corporate checking account.
/// - `11` - Corporate savings account.
/// - `12` - Gift card.
/// - `13` - EBT card.
/// - `14` - WIC card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChargebackPaymentMethod {
    /// No payment method specified.
    #[default]
    None = 0,
    /// American Express.
    AmericanExpress = 1,
    /// Visa.
    Visa = 2,
    /// MasterCard.
    Mastercard = 3,
    /// Diners Club.
    DinersClub = 4,
    /// Discover.
    Discover = 5,
    /// PayPal.
    PayPal = 6,
    /// Debit card.
    Debit = 7,
    /// Checking account.
    Checking = 8,
    /// Savings account.
    Savings = 9,
    /// Corporate checking account.
    CorporateChecking = 10,
    /// Corporate savings account.
    CorporateSavings = 11,
    /// Gift card.
    GiftCard = 12,
    /// EBT card.
    Ebt = 13,
    /// WIC card.
    Wic = 14,
}

/// Chargeback message status values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackMessageStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackMessageStatus {
    /// Message requested.
    #[default]
    Requested,
    /// Message processing.
    Processing,
    /// Message failed.
    Failed,
    /// Message denied.
    Denied,
    /// Message processed.
    Processed,
}

/// Chargeback message type values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackMessageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargebackMessageType {
    /// Assign case.
    #[default]
    Assign,
    /// Add note.
    Notate,
    /// Accept liability.
    AcceptLiability,
    /// Create pre-arbitration.
    CreatePreArbitration,
    /// Represent case.
    Represent,
    /// Respond to case.
    Respond,
    /// Request arbitration.
    RequestArbitration,
    /// Create arbitration.
    CreateArbitration,
    /// Request pre-arbitration.
    RequestPreArbitration,
    /// Request resolution to pre-arbitration.
    RequestResolutionToPreArbitration,
    /// Respond to dispute.
    RespondToDispute,
    /// Respond to pre-arbitration.
    RespondToPreArbitration,
    /// Unaccept liability.
    Unaccept,
    /// File pre-arbitration.
    FilePreArbitration,
}

/// Chargeback message result type values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackMessageResultType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackMessageResultType {
    /// General result.
    #[default]
    General,
    /// Platform-specific result.
    Platform,
}

/// Chargeback document status values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackDocumentStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackDocumentStatus {
    /// Document created.
    #[default]
    Created,
    /// Document processed.
    Processed,
    /// Document processing failed.
    Failed,
}

/// Chargeback document source values per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackDocumentSource`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ChargebackDocumentSource {
    /// Document from merchant.
    #[default]
    Merchant,
    /// Document from issuer.
    Issuer,
}

/// Document type values for chargeback documents per OpenAPI spec.
///
/// **OpenAPI schema:** `chargebackDocumentType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackDocumentType {
    /// Image file.
    #[default]
    Image,
    /// PDF document.
    Pdf,
    /// Text file.
    Text,
    /// TIFF image file.
    Tiff,
    /// PNG image file.
    Png,
    /// JPG/JPEG image file.
    Jpg,
    /// Other/generic.
    Other,
}

/// Message direction values per OpenAPI spec.
///
/// **OpenAPI schema:** `messageDirection`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MessageDirection {
    /// Inbound message (received).
    #[default]
    Inbound = 1,
    /// Outbound message (sent).
    Outbound = 2,
}

// =============================================================================
// Chargeback (Response)
// =============================================================================

/// A Payrix chargeback.
///
/// Represents a dispute on a transaction initiated by the cardholder's bank.
///
/// **OpenAPI schema:** `chargebacksResponse`
///
/// See `API_INCONSISTENCIES.md` for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Chargeback {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

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
    /// **OpenAPI type:** string (ref: `creator`)
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The ID of the merchant associated with the Chargeback.
    ///
    /// **OpenAPI type:** string (ref: `chargebacksModelMerchant`)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The ID of the Transaction associated with the Chargeback.
    ///
    /// **OpenAPI type:** string (ref: `chargebacksModelTxn`)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Chargeback Details
    // -------------------------------------------------------------------------

    /// The Merchant's processing MID.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mid: Option<String>,

    /// Description of the chargeback.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The total amount of the Chargeback in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total: Option<i64>,

    /// The representedTotal for this Chargeback if it has been represented.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub represented_total: Option<i64>,

    /// The current cycle/stage of the chargeback.
    ///
    /// **OpenAPI type:** string (ref: `cycle`)
    #[serde(default)]
    pub cycle: Option<ChargebackCycle>,

    /// The currency for this chargeback.
    ///
    /// See [Currency codes](https://www.iban.com/currency-codes) for all valid values.
    ///
    /// **OpenAPI type:** string (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// The platform used for this chargeback.
    ///
    /// **OpenAPI type:** string (ref: `chargebackPlatform`)
    #[serde(default)]
    pub platform: Option<ChargebackPlatform>,

    /// The type of payment method used for the transaction.
    ///
    /// **OpenAPI type:** integer (ref: `chargebackPaymentMethod`)
    #[serde(default)]
    pub payment_method: Option<ChargebackPaymentMethod>,

    /// The processing reference number for this Chargeback.
    ///
    /// **OpenAPI type:** string
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,

    /// The reason description for this Chargeback.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub reason: Option<String>,

    /// The reason code for this Chargeback.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub reason_code: Option<String>,

    // -------------------------------------------------------------------------
    // Dates
    // -------------------------------------------------------------------------

    /// The date when the Chargeback was issued.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub issued: Option<i32>,

    /// The date when the Chargeback was received.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub received: Option<i32>,

    /// The deadline to submit a reply for the Chargeback.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub reply: Option<i32>,

    // -------------------------------------------------------------------------
    // Reference Numbers
    // -------------------------------------------------------------------------

    /// The issuing bank's reference number for this Chargeback.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub bank_ref: Option<String>,

    /// Chargeback reference number.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback_ref: Option<String>,

    // -------------------------------------------------------------------------
    // Status
    // -------------------------------------------------------------------------

    /// The Chargeback's status.
    ///
    /// **OpenAPI type:** string (ref: `chargebackStatus`)
    #[serde(default)]
    pub status: Option<ChargebackStatusValue>,

    /// The ChargebackStatus representing the latest status change for this Chargeback.
    ///
    /// **OpenAPI type:** string (ref: `chargebacksModelLastStatusChange`)
    #[serde(default)]
    pub last_status_change: Option<PayrixId>,

    /// Whether the Chargeback is actionable and can be responded to.
    ///
    /// **OpenAPI type:** integer (ref: `Actionable`)
    ///
    /// Valid values:
    /// - `0` - Not actionable
    /// - `1` - Actionable
    #[serde(default, with = "bool_from_int_default_false")]
    pub actionable: bool,

    /// Whether the chargeback is shadowed.
    ///
    /// **OpenAPI type:** integer (ref: `Shadow`)
    ///
    /// Valid values:
    /// - `0` - Not shadowed
    /// - `1` - Shadowed
    #[serde(default, with = "bool_from_int_default_false")]
    pub shadow: bool,

    // -------------------------------------------------------------------------
    // Status Flags
    // -------------------------------------------------------------------------

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    ///
    /// Valid values:
    /// - `0` - Active
    /// - `1` - Inactive
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    ///
    /// Valid values:
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// =============================================================================
// ChargebackMessage (Response)
// =============================================================================

/// A Payrix chargeback message.
///
/// Messages are communications related to a chargeback case.
///
/// **OpenAPI schema:** `chargebackMessagesResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackMessage {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// Chargeback ID this message belongs to.
    ///
    /// **OpenAPI type:** string
    pub chargeback: PayrixId,

    /// Login ID that created this message.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Message type.
    ///
    /// **OpenAPI type:** string (ref: `chargebackMessageType`)
    #[serde(default, rename = "type")]
    pub message_type: Option<ChargebackMessageType>,

    /// Message status.
    ///
    /// **OpenAPI type:** string (ref: `chargebackMessageStatus`)
    #[serde(default)]
    pub status: Option<ChargebackMessageStatus>,

    /// Message subject.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub subject: Option<String>,

    /// Message body.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub message: Option<String>,

    /// Sender information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub sender: Option<String>,

    /// Direction (inbound/outbound).
    ///
    /// **OpenAPI type:** integer (ref: `messageDirection`)
    #[serde(default)]
    pub direction: Option<MessageDirection>,

    /// Read status.
    ///
    /// **OpenAPI type:** integer
    #[serde(default, with = "bool_from_int_default_false")]
    pub read: bool,

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

/// Request to create a new chargeback message.
///
/// **OpenAPI schema:** `chargebackMessagesRequest`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChargebackMessage {
    /// Chargeback ID (required).
    ///
    /// **OpenAPI type:** string
    pub chargeback: String,

    /// Message type.
    ///
    /// **OpenAPI type:** string (ref: `chargebackMessageType`)
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub message_type: Option<ChargebackMessageType>,

    /// Message subject.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Message body.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// =============================================================================
// ChargebackDocument (Response)
// =============================================================================

/// A Payrix chargeback document.
///
/// Documents are evidence files attached to chargeback cases.
///
/// **OpenAPI schema:** `chargebackDocumentsResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackDocument {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// Chargeback ID this document belongs to.
    ///
    /// **OpenAPI type:** string
    pub chargeback: PayrixId,

    /// Chargeback message ID (if attached to a message).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback_message: Option<PayrixId>,

    /// Login ID that uploaded this document.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Document name/filename.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// Document type/category.
    ///
    /// **OpenAPI type:** string (ref: `chargebackDocumentType`)
    #[serde(default, rename = "type")]
    pub document_type: Option<ChargebackDocumentType>,

    /// MIME type.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mime_type: Option<String>,

    /// File size in bytes.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub size: Option<i64>,

    /// Document URL or path.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub url: Option<String>,

    /// Document description.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

/// Request to create a new chargeback document.
///
/// **OpenAPI schema:** `chargebackDocumentsRequest`
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChargebackDocument {
    /// Chargeback ID (required).
    ///
    /// **OpenAPI type:** string
    pub chargeback: String,

    /// Chargeback message ID (if attaching to a message).
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_message: Option<String>,

    /// Document name/filename.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Document type/category.
    ///
    /// **OpenAPI type:** string (ref: `chargebackDocumentType`)
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub document_type: Option<ChargebackDocumentType>,

    /// MIME type.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Document description.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// =============================================================================
// ChargebackMessageResult (Response)
// =============================================================================

/// A Payrix chargeback message result.
///
/// Results track the outcome of chargeback message submissions.
///
/// **OpenAPI schema:** `chargebackMessageResultsResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackMessageResult {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modified: Option<String>,

    /// Chargeback ID.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// Chargeback message ID.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback_message: Option<PayrixId>,

    /// Result type.
    ///
    /// **OpenAPI type:** string (ref: `chargebackMessageResultType`)
    #[serde(default, rename = "type")]
    pub result_type: Option<ChargebackMessageResultType>,

    /// Result message/description.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub message: Option<String>,

    /// Platform response.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub response: Option<String>,

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// =============================================================================
// ChargebackStatus (Response)
// =============================================================================

/// A Payrix chargeback status record.
///
/// This tracks status changes/history for a chargeback case.
///
/// **OpenAPI schema:** `chargebackStatusesResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackStatus {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// Chargeback ID this status record belongs to.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// Current status value.
    ///
    /// **OpenAPI type:** string (ref: `chargebackStatus`)
    #[serde(default)]
    pub status: Option<ChargebackStatusValue>,

    /// Associated chargeback message ID (if any).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub chargeback_message: Option<PayrixId>,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ChargebackCycle Tests
    // =========================================================================

    #[test]
    fn chargeback_cycle_default() {
        assert_eq!(ChargebackCycle::default(), ChargebackCycle::Retrieval);
    }

    #[test]
    fn chargeback_cycle_serialize_all_variants() {
        let test_cases = [
            (ChargebackCycle::Retrieval, "\"retrieval\""),
            (ChargebackCycle::First, "\"first\""),
            (ChargebackCycle::Arbitration, "\"arbitration\""),
            (ChargebackCycle::Reversal, "\"reversal\""),
            (ChargebackCycle::Representment, "\"representment\""),
            (ChargebackCycle::PreArbitration, "\"preArbitration\""),
            (ChargebackCycle::ArbitrationLost, "\"arbitrationLost\""),
            (ChargebackCycle::ArbitrationSplit, "\"arbitrationSplit\""),
            (ChargebackCycle::ArbitrationWon, "\"arbitrationWon\""),
            (ChargebackCycle::IssuerAcceptPreArbitration, "\"issuerAcceptPreArbitration\""),
            (ChargebackCycle::IssuerDeclinedPreArbitration, "\"issuerDeclinedPreArbitration\""),
            (ChargebackCycle::ResponseToIssuerPreArbitration, "\"responseToIssuerPreArbitration\""),
            (ChargebackCycle::MerchantAcceptedPreArbitration, "\"merchantAcceptedPreArbitration\""),
            (ChargebackCycle::MerchantDeclinedPreArbitration, "\"merchantDeclinedPreArbitration\""),
            (ChargebackCycle::PreCompliance, "\"preCompliance\""),
            (ChargebackCycle::Compliance, "\"compliance\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_cycle_deserialize_all_variants() {
        let test_cases = [
            ("\"retrieval\"", ChargebackCycle::Retrieval),
            ("\"first\"", ChargebackCycle::First),
            ("\"arbitration\"", ChargebackCycle::Arbitration),
            ("\"reversal\"", ChargebackCycle::Reversal),
            ("\"representment\"", ChargebackCycle::Representment),
            ("\"preArbitration\"", ChargebackCycle::PreArbitration),
            ("\"arbitrationLost\"", ChargebackCycle::ArbitrationLost),
            ("\"arbitrationSplit\"", ChargebackCycle::ArbitrationSplit),
            ("\"arbitrationWon\"", ChargebackCycle::ArbitrationWon),
            ("\"issuerAcceptPreArbitration\"", ChargebackCycle::IssuerAcceptPreArbitration),
            ("\"issuerDeclinedPreArbitration\"", ChargebackCycle::IssuerDeclinedPreArbitration),
            ("\"responseToIssuerPreArbitration\"", ChargebackCycle::ResponseToIssuerPreArbitration),
            ("\"merchantAcceptedPreArbitration\"", ChargebackCycle::MerchantAcceptedPreArbitration),
            ("\"merchantDeclinedPreArbitration\"", ChargebackCycle::MerchantDeclinedPreArbitration),
            ("\"preCompliance\"", ChargebackCycle::PreCompliance),
            ("\"compliance\"", ChargebackCycle::Compliance),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackCycle = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    // =========================================================================
    // ChargebackStatusValue Tests
    // =========================================================================

    #[test]
    fn chargeback_status_value_default() {
        assert_eq!(ChargebackStatusValue::default(), ChargebackStatusValue::Open);
    }

    #[test]
    fn chargeback_status_value_serialize_all_variants() {
        let test_cases = [
            (ChargebackStatusValue::Open, "\"open\""),
            (ChargebackStatusValue::Closed, "\"closed\""),
            (ChargebackStatusValue::Won, "\"won\""),
            (ChargebackStatusValue::Lost, "\"lost\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_status_value_deserialize_all_variants() {
        let test_cases = [
            ("\"open\"", ChargebackStatusValue::Open),
            ("\"closed\"", ChargebackStatusValue::Closed),
            ("\"won\"", ChargebackStatusValue::Won),
            ("\"lost\"", ChargebackStatusValue::Lost),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackStatusValue = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    // =========================================================================
    // ChargebackPaymentMethod Tests
    // =========================================================================

    #[test]
    fn chargeback_payment_method_default() {
        assert_eq!(ChargebackPaymentMethod::default(), ChargebackPaymentMethod::None);
    }

    #[test]
    fn chargeback_payment_method_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::None).unwrap(), "0");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::AmericanExpress).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Visa).unwrap(), "2");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Mastercard).unwrap(), "3");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::DinersClub).unwrap(), "4");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Discover).unwrap(), "5");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::PayPal).unwrap(), "6");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Debit).unwrap(), "7");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Checking).unwrap(), "8");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Savings).unwrap(), "9");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::CorporateChecking).unwrap(), "10");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::CorporateSavings).unwrap(), "11");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::GiftCard).unwrap(), "12");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Ebt).unwrap(), "13");
        assert_eq!(serde_json::to_string(&ChargebackPaymentMethod::Wic).unwrap(), "14");
    }

    // =========================================================================
    // Chargeback Struct Tests
    // =========================================================================

    #[test]
    fn chargeback_deserialize_full() {
        let json = r#"{
            "id": "t1_chb_12345678901234567890123",
            "created": "2024-01-01 10:00:00.0000",
            "modified": "2024-01-01 15:30:00.0000",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345",
            "merchant": "t1_mer_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "mid": "123456789012345",
            "description": "Disputed transaction",
            "total": 10000,
            "representedTotal": 10000,
            "cycle": "first",
            "currency": "USD",
            "platform": "VCORE",
            "paymentMethod": 2,
            "ref": "REF123456",
            "reason": "Cardholder Dispute",
            "reasonCode": "4853",
            "issued": 20240101,
            "received": 20240102,
            "reply": 20240115,
            "bankRef": "BANK123",
            "chargebackRef": "CB123",
            "status": "open",
            "lastStatusChange": "t1_chs_12345678901234567890123",
            "actionable": 1,
            "shadow": 0,
            "inactive": 0,
            "frozen": 1
        }"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(chargeback.id.as_str(), "t1_chb_12345678901234567890123");
        assert_eq!(chargeback.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(chargeback.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
        assert_eq!(chargeback.merchant.as_ref().unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(chargeback.txn.as_ref().unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(chargeback.mid.as_deref(), Some("123456789012345"));
        assert_eq!(chargeback.description.as_deref(), Some("Disputed transaction"));
        assert_eq!(chargeback.total, Some(10000));
        assert_eq!(chargeback.represented_total, Some(10000));
        assert_eq!(chargeback.cycle, Some(ChargebackCycle::First));
        assert_eq!(chargeback.currency.as_deref(), Some("USD"));
        assert_eq!(chargeback.platform, Some(ChargebackPlatform::VCore));
        assert_eq!(chargeback.payment_method, Some(ChargebackPaymentMethod::Visa));
        assert_eq!(chargeback.reference.as_deref(), Some("REF123456"));
        assert_eq!(chargeback.reason.as_deref(), Some("Cardholder Dispute"));
        assert_eq!(chargeback.reason_code.as_deref(), Some("4853"));
        assert_eq!(chargeback.issued, Some(20240101));
        assert_eq!(chargeback.received, Some(20240102));
        assert_eq!(chargeback.reply, Some(20240115));
        assert_eq!(chargeback.bank_ref.as_deref(), Some("BANK123"));
        assert_eq!(chargeback.chargeback_ref.as_deref(), Some("CB123"));
        assert_eq!(chargeback.status, Some(ChargebackStatusValue::Open));
        assert_eq!(chargeback.last_status_change.as_ref().unwrap().as_str(), "t1_chs_12345678901234567890123");
        assert!(chargeback.actionable);
        assert!(!chargeback.shadow);
        assert!(!chargeback.inactive);
        assert!(chargeback.frozen);
    }

    #[test]
    fn chargeback_deserialize_minimal() {
        let json = r#"{"id": "t1_chb_12345678901234567890123"}"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(chargeback.id.as_str(), "t1_chb_12345678901234567890123");
        assert!(chargeback.creator.is_none());
        assert!(chargeback.merchant.is_none());
        assert!(chargeback.txn.is_none());
        assert!(chargeback.total.is_none());
        assert!(chargeback.cycle.is_none());
        assert!(chargeback.status.is_none());
        assert!(!chargeback.actionable);
        assert!(!chargeback.shadow);
        assert!(!chargeback.inactive);
        assert!(!chargeback.frozen);
    }

    #[test]
    fn chargeback_bool_from_int() {
        let json = r#"{
            "id": "t1_chb_12345678901234567890123",
            "actionable": 1,
            "shadow": 1,
            "inactive": 0,
            "frozen": 1
        }"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert!(chargeback.actionable);
        assert!(chargeback.shadow);
        assert!(!chargeback.inactive);
        assert!(chargeback.frozen);
    }

    // =========================================================================
    // ChargebackMessage Tests
    // =========================================================================

    #[test]
    fn chargeback_message_status_default() {
        assert_eq!(ChargebackMessageStatus::default(), ChargebackMessageStatus::Requested);
    }

    #[test]
    fn chargeback_message_type_default() {
        assert_eq!(ChargebackMessageType::default(), ChargebackMessageType::Assign);
    }

    #[test]
    fn new_chargeback_message_serialize() {
        let msg = NewChargebackMessage {
            chargeback: "t1_chb_12345678901234567890123".to_string(),
            message_type: Some(ChargebackMessageType::Represent),
            subject: Some("Response".to_string()),
            message: Some("We are representing this transaction".to_string()),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_chb_12345678901234567890123\""));
        assert!(json.contains("\"type\":\"represent\""));
        assert!(json.contains("\"subject\":\"Response\""));
    }

    // =========================================================================
    // ChargebackDocument Tests
    // =========================================================================

    #[test]
    fn chargeback_document_status_default() {
        assert_eq!(ChargebackDocumentStatus::default(), ChargebackDocumentStatus::Created);
    }

    #[test]
    fn chargeback_document_type_default() {
        assert_eq!(ChargebackDocumentType::default(), ChargebackDocumentType::Image);
    }

    #[test]
    fn new_chargeback_document_serialize() {
        let doc = NewChargebackDocument {
            chargeback: "t1_chb_12345678901234567890123".to_string(),
            chargeback_message: Some("t1_cbm_12345678901234567890123".to_string()),
            name: Some("evidence.pdf".to_string()),
            document_type: Some(ChargebackDocumentType::Pdf),
            mime_type: Some("application/pdf".to_string()),
            description: Some("Supporting evidence".to_string()),
        };

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_chb_12345678901234567890123\""));
        assert!(json.contains("\"type\":\"pdf\""));
        assert!(json.contains("\"name\":\"evidence.pdf\""));
    }

    // =========================================================================
    // ChargebackMessageResult Tests
    // =========================================================================

    #[test]
    fn chargeback_message_result_type_default() {
        assert_eq!(ChargebackMessageResultType::default(), ChargebackMessageResultType::General);
    }

    // =========================================================================
    // MessageDirection Tests
    // =========================================================================

    #[test]
    fn message_direction_default() {
        assert_eq!(MessageDirection::default(), MessageDirection::Inbound);
    }

    #[test]
    fn message_direction_serialize() {
        assert_eq!(serde_json::to_string(&MessageDirection::Inbound).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MessageDirection::Outbound).unwrap(), "2");
    }
}
