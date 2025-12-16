//! Chargeback types for the Payrix API.
//!
//! Chargebacks represent disputed transactions and their resolution process.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, DateYmd, PayrixId};

/// Chargeback cycle/stage values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ChargebackCycle {
    /// Initial request from issuer for more information
    #[default]
    Retrieval,
    /// First chargeback from issuer
    First,
    /// Arbitration being sought
    Arbitration,
    /// Chargeback was reversed
    Reversal,
    /// Merchant being represented with response
    Representment,
    /// Chargeback is no longer representable
    PreArbitration,
    /// Arbitration lost
    ArbitrationLost,
    /// Arbitration split decision
    ArbitrationSplit,
    /// Arbitration won
    ArbitrationWon,
    /// Issuer accepted pre-arbitration
    IssuerAcceptPreArbitration,
    /// Issuer declined pre-arbitration
    IssuerDeclinedPreArbitration,
    /// Response to issuer pre-arbitration
    ResponseToIssuerPreArbitration,
    /// Merchant accepted pre-arbitration
    MerchantAcceptedPreArbitration,
    /// Merchant declined pre-arbitration
    MerchantDeclinedPreArbitration,
    /// Pre-compliance stage
    PreCompliance,
    /// Compliance stage
    Compliance,
}

/// Chargeback message status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackMessageStatus {
    /// Message requested
    #[default]
    Requested,
    /// Message processing
    Processing,
    /// Message failed
    Failed,
    /// Message denied
    Denied,
    /// Message processed
    Processed,
}

/// Chargeback message type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargebackMessageType {
    /// Assign case
    #[default]
    Assign,
    /// Add note
    Notate,
    /// Accept liability
    AcceptLiability,
    /// Create pre-arbitration
    CreatePreArbitration,
    /// Represent case
    Represent,
    /// Respond to case
    Respond,
    /// Request arbitration
    RequestArbitration,
    /// Create arbitration
    CreateArbitration,
    /// Request pre-arbitration
    RequestPreArbitration,
    /// Request resolution to pre-arbitration
    RequestResolutionToPreArbitration,
    /// Respond to dispute
    RespondToDispute,
    /// Respond to pre-arbitration
    RespondToPreArbitration,
    /// Unaccept liability
    Unaccept,
    /// File pre-arbitration
    FilePreArbitration,
}

/// Chargeback message result type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackMessageResultType {
    /// General result
    #[default]
    General,
    /// Platform-specific result
    Platform,
}

/// Chargeback document status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChargebackDocumentStatus {
    /// Document created
    #[default]
    Created,
    /// Document processed
    Processed,
    /// Document processing failed
    Failed,
}

/// Chargeback document source values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ChargebackDocumentSource {
    /// Document from merchant
    #[default]
    Merchant,
    /// Document from issuer
    Issuer,
}

/// Chargeback status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChargebackStatusValue {
    /// New/pending chargeback
    #[default]
    New = 0,
    /// Under review
    UnderReview = 1,
    /// Response submitted
    Responded = 2,
    /// Won by merchant
    Won = 3,
    /// Lost by merchant
    Lost = 4,
    /// Expired (no response)
    Expired = 5,
}

/// Chargeback type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChargebackType {
    /// Retrieval request
    #[default]
    Retrieval = 1,
    /// First chargeback
    FirstChargeback = 2,
    /// Pre-arbitration
    PreArbitration = 3,
    /// Arbitration
    Arbitration = 4,
}

/// Chargeback outcome values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChargebackOutcome {
    /// Pending/undecided
    #[default]
    Pending = 0,
    /// Won by merchant
    Won = 1,
    /// Lost by merchant
    Lost = 2,
    /// Withdrawn
    Withdrawn = 3,
}

/// Message direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MessageDirection {
    /// Inbound message (received)
    #[default]
    Inbound = 1,
    /// Outbound message (sent)
    Outbound = 2,
}

/// A Payrix chargeback.
///
/// Represents a dispute on a transaction initiated by the cardholder's bank.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Chargeback {
    /// Unique identifier (30 characters, e.g., "t1_cbk_...")
    pub id: PayrixId,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Transaction ID that was disputed
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// Login ID that created this record
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Chargeback case number
    #[serde(default)]
    pub case_number: Option<String>,

    /// Chargeback reason code
    #[serde(default)]
    pub reason_code: Option<String>,

    /// Reason description
    #[serde(default)]
    pub reason: Option<String>,

    /// Chargeback amount in cents
    #[serde(default)]
    pub amount: Option<i64>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Chargeback status
    #[serde(default)]
    pub status: Option<ChargebackStatusValue>,

    /// Chargeback type
    #[serde(default, rename = "type")]
    pub chargeback_type: Option<ChargebackType>,

    /// Due date for response (YYYYMMDD format)
    #[serde(default)]
    pub due_date: Option<DateYmd>,

    /// Date the chargeback was received (YYYYMMDD format)
    #[serde(default)]
    pub received_date: Option<DateYmd>,

    /// Date the chargeback was resolved (YYYYMMDD format)
    #[serde(default)]
    pub resolved_date: Option<DateYmd>,

    /// Outcome of the chargeback
    #[serde(default)]
    pub outcome: Option<ChargebackOutcome>,

    /// ARN (Acquirer Reference Number)
    #[serde(default)]
    pub arn: Option<String>,

    /// Card number (masked)
    #[serde(default)]
    pub card: Option<String>,

    /// Last 4 digits of card
    #[serde(default)]
    pub last4: Option<String>,

    /// First 6 digits of card (BIN)
    #[serde(default)]
    pub first6: Option<String>,

    /// Cardholder name
    #[serde(default)]
    pub cardholder: Option<String>,

    /// Platform (processor)
    #[serde(default)]
    pub platform: Option<String>,

    /// Notes/description
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

/// A Payrix chargeback message.
///
/// Messages are communications related to a chargeback case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackMessage {
    /// Unique identifier (30 characters, e.g., "t1_cbm_...")
    pub id: PayrixId,

    /// Chargeback ID this message belongs to (required)
    pub chargeback: PayrixId,

    /// Login ID that created this message
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Message type
    #[serde(default, rename = "type")]
    pub message_type: Option<ChargebackMessageType>,

    /// Message status
    #[serde(default)]
    pub status: Option<ChargebackMessageStatus>,

    /// Message subject
    #[serde(default)]
    pub subject: Option<String>,

    /// Message body
    #[serde(default)]
    pub message: Option<String>,

    /// Sender information
    #[serde(default)]
    pub sender: Option<String>,

    /// Direction (inbound/outbound)
    #[serde(default)]
    pub direction: Option<MessageDirection>,

    /// Read status
    #[serde(default, with = "bool_from_int_default_false")]
    pub read: bool,

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

/// Request to create a new chargeback message.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChargebackMessage {
    /// Chargeback ID (required)
    pub chargeback: String,

    /// Message type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub message_type: Option<ChargebackMessageType>,

    /// Message subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Message body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Document type values for chargeback documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChargebackDocumentType {
    /// Image file
    #[default]
    Image = 1,
    /// PDF document
    Pdf = 2,
    /// Text file
    Text = 3,
    /// Other/generic
    Other = 4,
}

/// A Payrix chargeback document.
///
/// Documents are evidence files attached to chargeback cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackDocument {
    /// Unique identifier (30 characters, e.g., "t1_cbd_...")
    pub id: PayrixId,

    /// Chargeback ID this document belongs to (required)
    pub chargeback: PayrixId,

    /// Chargeback message ID (if attached to a message)
    #[serde(default)]
    pub chargeback_message: Option<PayrixId>,

    /// Login ID that uploaded this document
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Document name/filename
    #[serde(default)]
    pub name: Option<String>,

    /// Document type/category
    #[serde(default, rename = "type")]
    pub document_type: Option<ChargebackDocumentType>,

    /// MIME type
    #[serde(default)]
    pub mime_type: Option<String>,

    /// File size in bytes
    #[serde(default)]
    pub size: Option<i64>,

    /// Document URL or path
    #[serde(default)]
    pub url: Option<String>,

    /// Document description
    #[serde(default)]
    pub description: Option<String>,

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

/// Request to create a new chargeback document.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewChargebackDocument {
    /// Chargeback ID (required)
    pub chargeback: String,

    /// Chargeback message ID (if attaching to a message)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_message: Option<String>,

    /// Document name/filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Document type/category
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub document_type: Option<ChargebackDocumentType>,

    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Document description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A Payrix chargeback message result.
///
/// Results track the outcome of chargeback message submissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackMessageResult {
    /// Unique identifier (30 characters, e.g., "t1_cmr_...")
    pub id: PayrixId,

    /// Chargeback ID
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// Chargeback message ID
    #[serde(default)]
    pub chargeback_message: Option<PayrixId>,

    /// Result type
    #[serde(default, rename = "type")]
    pub result_type: Option<ChargebackMessageResultType>,

    /// Result message/description
    #[serde(default)]
    pub message: Option<String>,

    /// Platform response
    #[serde(default)]
    pub response: Option<String>,

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

/// A Payrix chargeback status record.
///
/// Tracks status changes for chargebacks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct ChargebackStatus {
    /// Unique identifier (30 characters, e.g., "t1_cbs_...")
    pub id: PayrixId,

    /// Chargeback ID
    #[serde(default)]
    pub chargeback: Option<PayrixId>,

    /// Login ID that made the status change
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Previous status
    #[serde(default)]
    pub from_status: Option<ChargebackStatusValue>,

    /// New status
    #[serde(default)]
    pub to_status: Option<ChargebackStatusValue>,

    /// Status name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Status description/reason for change
    #[serde(default)]
    pub description: Option<String>,

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
    fn chargeback_cycle_default() {
        assert_eq!(ChargebackCycle::default(), ChargebackCycle::Retrieval);
    }

    #[test]
    fn chargeback_cycle_all_variants_serialize() {
        let test_cases = [
            (ChargebackCycle::Retrieval, "\"Retrieval\""),
            (ChargebackCycle::First, "\"First\""),
            (ChargebackCycle::Arbitration, "\"Arbitration\""),
            (ChargebackCycle::Reversal, "\"Reversal\""),
            (ChargebackCycle::Representment, "\"Representment\""),
            (ChargebackCycle::PreArbitration, "\"PreArbitration\""),
            (ChargebackCycle::ArbitrationLost, "\"ArbitrationLost\""),
            (ChargebackCycle::ArbitrationSplit, "\"ArbitrationSplit\""),
            (ChargebackCycle::ArbitrationWon, "\"ArbitrationWon\""),
            (ChargebackCycle::IssuerAcceptPreArbitration, "\"IssuerAcceptPreArbitration\""),
            (ChargebackCycle::IssuerDeclinedPreArbitration, "\"IssuerDeclinedPreArbitration\""),
            (ChargebackCycle::ResponseToIssuerPreArbitration, "\"ResponseToIssuerPreArbitration\""),
            (ChargebackCycle::MerchantAcceptedPreArbitration, "\"MerchantAcceptedPreArbitration\""),
            (ChargebackCycle::MerchantDeclinedPreArbitration, "\"MerchantDeclinedPreArbitration\""),
            (ChargebackCycle::PreCompliance, "\"PreCompliance\""),
            (ChargebackCycle::Compliance, "\"Compliance\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_cycle_all_variants_deserialize() {
        let test_cases = [
            ("\"Retrieval\"", ChargebackCycle::Retrieval),
            ("\"First\"", ChargebackCycle::First),
            ("\"Arbitration\"", ChargebackCycle::Arbitration),
            ("\"Reversal\"", ChargebackCycle::Reversal),
            ("\"Representment\"", ChargebackCycle::Representment),
            ("\"PreArbitration\"", ChargebackCycle::PreArbitration),
            ("\"ArbitrationLost\"", ChargebackCycle::ArbitrationLost),
            ("\"ArbitrationSplit\"", ChargebackCycle::ArbitrationSplit),
            ("\"ArbitrationWon\"", ChargebackCycle::ArbitrationWon),
            ("\"IssuerAcceptPreArbitration\"", ChargebackCycle::IssuerAcceptPreArbitration),
            ("\"IssuerDeclinedPreArbitration\"", ChargebackCycle::IssuerDeclinedPreArbitration),
            ("\"ResponseToIssuerPreArbitration\"", ChargebackCycle::ResponseToIssuerPreArbitration),
            ("\"MerchantAcceptedPreArbitration\"", ChargebackCycle::MerchantAcceptedPreArbitration),
            ("\"MerchantDeclinedPreArbitration\"", ChargebackCycle::MerchantDeclinedPreArbitration),
            ("\"PreCompliance\"", ChargebackCycle::PreCompliance),
            ("\"Compliance\"", ChargebackCycle::Compliance),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackCycle = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_message_status_default() {
        assert_eq!(ChargebackMessageStatus::default(), ChargebackMessageStatus::Requested);
    }

    #[test]
    fn chargeback_message_status_all_variants_serialize() {
        let test_cases = [
            (ChargebackMessageStatus::Requested, "\"requested\""),
            (ChargebackMessageStatus::Processing, "\"processing\""),
            (ChargebackMessageStatus::Failed, "\"failed\""),
            (ChargebackMessageStatus::Denied, "\"denied\""),
            (ChargebackMessageStatus::Processed, "\"processed\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_message_status_all_variants_deserialize() {
        let test_cases = [
            ("\"requested\"", ChargebackMessageStatus::Requested),
            ("\"processing\"", ChargebackMessageStatus::Processing),
            ("\"failed\"", ChargebackMessageStatus::Failed),
            ("\"denied\"", ChargebackMessageStatus::Denied),
            ("\"processed\"", ChargebackMessageStatus::Processed),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackMessageStatus = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_message_type_default() {
        assert_eq!(ChargebackMessageType::default(), ChargebackMessageType::Assign);
    }

    #[test]
    fn chargeback_message_type_all_variants_serialize() {
        let test_cases = [
            (ChargebackMessageType::Assign, "\"assign\""),
            (ChargebackMessageType::Notate, "\"notate\""),
            (ChargebackMessageType::AcceptLiability, "\"acceptLiability\""),
            (ChargebackMessageType::CreatePreArbitration, "\"createPreArbitration\""),
            (ChargebackMessageType::Represent, "\"represent\""),
            (ChargebackMessageType::Respond, "\"respond\""),
            (ChargebackMessageType::RequestArbitration, "\"requestArbitration\""),
            (ChargebackMessageType::CreateArbitration, "\"createArbitration\""),
            (ChargebackMessageType::RequestPreArbitration, "\"requestPreArbitration\""),
            (ChargebackMessageType::RequestResolutionToPreArbitration, "\"requestResolutionToPreArbitration\""),
            (ChargebackMessageType::RespondToDispute, "\"respondToDispute\""),
            (ChargebackMessageType::RespondToPreArbitration, "\"respondToPreArbitration\""),
            (ChargebackMessageType::Unaccept, "\"unaccept\""),
            (ChargebackMessageType::FilePreArbitration, "\"filePreArbitration\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_message_type_all_variants_deserialize() {
        let test_cases = [
            ("\"assign\"", ChargebackMessageType::Assign),
            ("\"notate\"", ChargebackMessageType::Notate),
            ("\"acceptLiability\"", ChargebackMessageType::AcceptLiability),
            ("\"createPreArbitration\"", ChargebackMessageType::CreatePreArbitration),
            ("\"represent\"", ChargebackMessageType::Represent),
            ("\"respond\"", ChargebackMessageType::Respond),
            ("\"requestArbitration\"", ChargebackMessageType::RequestArbitration),
            ("\"createArbitration\"", ChargebackMessageType::CreateArbitration),
            ("\"requestPreArbitration\"", ChargebackMessageType::RequestPreArbitration),
            ("\"requestResolutionToPreArbitration\"", ChargebackMessageType::RequestResolutionToPreArbitration),
            ("\"respondToDispute\"", ChargebackMessageType::RespondToDispute),
            ("\"respondToPreArbitration\"", ChargebackMessageType::RespondToPreArbitration),
            ("\"unaccept\"", ChargebackMessageType::Unaccept),
            ("\"filePreArbitration\"", ChargebackMessageType::FilePreArbitration),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackMessageType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_message_result_type_default() {
        assert_eq!(ChargebackMessageResultType::default(), ChargebackMessageResultType::General);
    }

    #[test]
    fn chargeback_message_result_type_all_variants_serialize() {
        let test_cases = [
            (ChargebackMessageResultType::General, "\"general\""),
            (ChargebackMessageResultType::Platform, "\"platform\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_message_result_type_all_variants_deserialize() {
        let test_cases = [
            ("\"general\"", ChargebackMessageResultType::General),
            ("\"platform\"", ChargebackMessageResultType::Platform),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackMessageResultType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_document_status_default() {
        assert_eq!(ChargebackDocumentStatus::default(), ChargebackDocumentStatus::Created);
    }

    #[test]
    fn chargeback_document_status_all_variants_serialize() {
        let test_cases = [
            (ChargebackDocumentStatus::Created, "\"created\""),
            (ChargebackDocumentStatus::Processed, "\"processed\""),
            (ChargebackDocumentStatus::Failed, "\"failed\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_document_status_all_variants_deserialize() {
        let test_cases = [
            ("\"created\"", ChargebackDocumentStatus::Created),
            ("\"processed\"", ChargebackDocumentStatus::Processed),
            ("\"failed\"", ChargebackDocumentStatus::Failed),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackDocumentStatus = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_document_source_default() {
        assert_eq!(ChargebackDocumentSource::default(), ChargebackDocumentSource::Merchant);
    }

    #[test]
    fn chargeback_document_source_all_variants_serialize() {
        let test_cases = [
            (ChargebackDocumentSource::Merchant, "\"Merchant\""),
            (ChargebackDocumentSource::Issuer, "\"Issuer\""),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_document_source_all_variants_deserialize() {
        let test_cases = [
            ("\"Merchant\"", ChargebackDocumentSource::Merchant),
            ("\"Issuer\"", ChargebackDocumentSource::Issuer),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackDocumentSource = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_status_value_default() {
        assert_eq!(ChargebackStatusValue::default(), ChargebackStatusValue::New);
    }

    #[test]
    fn chargeback_status_value_all_variants_serialize() {
        let test_cases = [
            (ChargebackStatusValue::New, "0"),
            (ChargebackStatusValue::UnderReview, "1"),
            (ChargebackStatusValue::Responded, "2"),
            (ChargebackStatusValue::Won, "3"),
            (ChargebackStatusValue::Lost, "4"),
            (ChargebackStatusValue::Expired, "5"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_status_value_all_variants_deserialize() {
        let test_cases = [
            ("0", ChargebackStatusValue::New),
            ("1", ChargebackStatusValue::UnderReview),
            ("2", ChargebackStatusValue::Responded),
            ("3", ChargebackStatusValue::Won),
            ("4", ChargebackStatusValue::Lost),
            ("5", ChargebackStatusValue::Expired),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackStatusValue = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_type_default() {
        assert_eq!(ChargebackType::default(), ChargebackType::Retrieval);
    }

    #[test]
    fn chargeback_type_all_variants_serialize() {
        let test_cases = [
            (ChargebackType::Retrieval, "1"),
            (ChargebackType::FirstChargeback, "2"),
            (ChargebackType::PreArbitration, "3"),
            (ChargebackType::Arbitration, "4"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_type_all_variants_deserialize() {
        let test_cases = [
            ("1", ChargebackType::Retrieval),
            ("2", ChargebackType::FirstChargeback),
            ("3", ChargebackType::PreArbitration),
            ("4", ChargebackType::Arbitration),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_outcome_default() {
        assert_eq!(ChargebackOutcome::default(), ChargebackOutcome::Pending);
    }

    #[test]
    fn chargeback_outcome_all_variants_serialize() {
        let test_cases = [
            (ChargebackOutcome::Pending, "0"),
            (ChargebackOutcome::Won, "1"),
            (ChargebackOutcome::Lost, "2"),
            (ChargebackOutcome::Withdrawn, "3"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_outcome_all_variants_deserialize() {
        let test_cases = [
            ("0", ChargebackOutcome::Pending),
            ("1", ChargebackOutcome::Won),
            ("2", ChargebackOutcome::Lost),
            ("3", ChargebackOutcome::Withdrawn),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackOutcome = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn message_direction_default() {
        assert_eq!(MessageDirection::default(), MessageDirection::Inbound);
    }

    #[test]
    fn message_direction_all_variants_serialize() {
        let test_cases = [
            (MessageDirection::Inbound, "1"),
            (MessageDirection::Outbound, "2"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn message_direction_all_variants_deserialize() {
        let test_cases = [
            ("1", MessageDirection::Inbound),
            ("2", MessageDirection::Outbound),
        ];

        for (json, expected_variant) in test_cases {
            let variant: MessageDirection = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn chargeback_document_type_default() {
        assert_eq!(ChargebackDocumentType::default(), ChargebackDocumentType::Image);
    }

    #[test]
    fn chargeback_document_type_all_variants_serialize() {
        let test_cases = [
            (ChargebackDocumentType::Image, "1"),
            (ChargebackDocumentType::Pdf, "2"),
            (ChargebackDocumentType::Text, "3"),
            (ChargebackDocumentType::Other, "4"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn chargeback_document_type_all_variants_deserialize() {
        let test_cases = [
            ("1", ChargebackDocumentType::Image),
            ("2", ChargebackDocumentType::Pdf),
            ("3", ChargebackDocumentType::Text),
            ("4", ChargebackDocumentType::Other),
        ];

        for (json, expected_variant) in test_cases {
            let variant: ChargebackDocumentType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    // ==================== Chargeback Struct Tests ====================

    #[test]
    fn chargeback_deserialize_full() {
        let json = r#"{
            "id": "t1_cbk_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "caseNumber": "CB-2024-001",
            "reasonCode": "4853",
            "reason": "Cardholder Dispute",
            "amount": 10000,
            "currency": "USD",
            "status": 1,
            "type": 2,
            "dueDate": "20240115",
            "receivedDate": "20240101",
            "resolvedDate": "20240120",
            "outcome": 1,
            "arn": "74537604221111111111111",
            "card": "************1234",
            "last4": "1234",
            "first6": "411111",
            "cardholder": "John Doe",
            "platform": "tsys",
            "description": "Disputed transaction",
            "custom": "custom_data",
            "created": "2024-01-01 10:00:00.000",
            "modified": "2024-01-01 15:30:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(chargeback.id.as_str(), "t1_cbk_12345678901234567890123");
        assert_eq!(chargeback.merchant.as_ref().unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(chargeback.entity.as_ref().unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(chargeback.txn.as_ref().unwrap().as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(chargeback.login.as_ref().unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(chargeback.case_number.as_ref().unwrap(), "CB-2024-001");
        assert_eq!(chargeback.reason_code.as_ref().unwrap(), "4853");
        assert_eq!(chargeback.reason.as_ref().unwrap(), "Cardholder Dispute");
        assert_eq!(chargeback.amount.unwrap(), 10000);
        assert_eq!(chargeback.currency.as_ref().unwrap(), "USD");
        assert_eq!(chargeback.status.unwrap(), ChargebackStatusValue::UnderReview);
        assert_eq!(chargeback.chargeback_type.unwrap(), ChargebackType::FirstChargeback);
        assert_eq!(chargeback.due_date.as_ref().unwrap().as_str(), "20240115");
        assert_eq!(chargeback.received_date.as_ref().unwrap().as_str(), "20240101");
        assert_eq!(chargeback.resolved_date.as_ref().unwrap().as_str(), "20240120");
        assert_eq!(chargeback.outcome.unwrap(), ChargebackOutcome::Won);
        assert_eq!(chargeback.arn.as_ref().unwrap(), "74537604221111111111111");
        assert_eq!(chargeback.card.as_ref().unwrap(), "************1234");
        assert_eq!(chargeback.last4.as_ref().unwrap(), "1234");
        assert_eq!(chargeback.first6.as_ref().unwrap(), "411111");
        assert_eq!(chargeback.cardholder.as_ref().unwrap(), "John Doe");
        assert_eq!(chargeback.platform.as_ref().unwrap(), "tsys");
        assert_eq!(chargeback.description.as_ref().unwrap(), "Disputed transaction");
        assert_eq!(chargeback.custom.as_ref().unwrap(), "custom_data");
        assert_eq!(chargeback.created.as_ref().unwrap(), "2024-01-01 10:00:00.000");
        assert_eq!(chargeback.modified.as_ref().unwrap(), "2024-01-01 15:30:00.000");
        assert_eq!(chargeback.inactive, false);
        assert_eq!(chargeback.frozen, true);
    }

    #[test]
    fn chargeback_deserialize_minimal() {
        let json = r#"{
            "id": "t1_cbk_12345678901234567890123"
        }"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(chargeback.id.as_str(), "t1_cbk_12345678901234567890123");
        assert!(chargeback.merchant.is_none());
        assert!(chargeback.entity.is_none());
        assert!(chargeback.txn.is_none());
        assert!(chargeback.login.is_none());
        assert!(chargeback.case_number.is_none());
        assert!(chargeback.reason_code.is_none());
        assert!(chargeback.reason.is_none());
        assert!(chargeback.amount.is_none());
        assert!(chargeback.currency.is_none());
        assert!(chargeback.status.is_none());
        assert!(chargeback.chargeback_type.is_none());
        assert!(chargeback.due_date.is_none());
        assert!(chargeback.received_date.is_none());
        assert!(chargeback.resolved_date.is_none());
        assert!(chargeback.outcome.is_none());
        assert!(chargeback.arn.is_none());
        assert!(chargeback.card.is_none());
        assert!(chargeback.last4.is_none());
        assert!(chargeback.first6.is_none());
        assert!(chargeback.cardholder.is_none());
        assert!(chargeback.platform.is_none());
        assert!(chargeback.description.is_none());
        assert!(chargeback.custom.is_none());
        assert!(chargeback.created.is_none());
        assert!(chargeback.modified.is_none());
        assert_eq!(chargeback.inactive, false);
        assert_eq!(chargeback.frozen, false);
    }

    #[test]
    fn chargeback_deserialize_bool_from_int() {
        // Test bool_from_int deserialization
        let json = r#"{
            "id": "t1_cbk_12345678901234567890123",
            "inactive": 1,
            "frozen": 0
        }"#;

        let chargeback: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(chargeback.inactive, true);
        assert_eq!(chargeback.frozen, false);
    }

    // ==================== NewChargebackMessage Tests ====================

    #[test]
    fn new_chargeback_message_serialize_full() {
        let msg = NewChargebackMessage {
            chargeback: "t1_cbk_12345678901234567890123".to_string(),
            message_type: Some(ChargebackMessageType::Represent),
            subject: Some("Response to chargeback".to_string()),
            message: Some("We are representing this transaction".to_string()),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_cbk_12345678901234567890123\""));
        assert!(json.contains("\"type\":\"represent\""));
        assert!(json.contains("\"subject\":\"Response to chargeback\""));
        assert!(json.contains("\"message\":\"We are representing this transaction\""));
    }

    #[test]
    fn new_chargeback_message_serialize_minimal() {
        let msg = NewChargebackMessage {
            chargeback: "t1_cbk_12345678901234567890123".to_string(),
            message_type: None,
            subject: None,
            message: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_cbk_12345678901234567890123\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"subject\""));
        assert!(!json.contains("\"message\""));
    }

    #[test]
    fn new_chargeback_message_default() {
        let msg = NewChargebackMessage::default();
        assert_eq!(msg.chargeback, "");
        assert!(msg.message_type.is_none());
        assert!(msg.subject.is_none());
        assert!(msg.message.is_none());
    }

    // ==================== NewChargebackDocument Tests ====================

    #[test]
    fn new_chargeback_document_serialize_full() {
        let doc = NewChargebackDocument {
            chargeback: "t1_cbk_12345678901234567890123".to_string(),
            chargeback_message: Some("t1_cbm_12345678901234567890123".to_string()),
            name: Some("evidence.pdf".to_string()),
            document_type: Some(ChargebackDocumentType::Pdf),
            mime_type: Some("application/pdf".to_string()),
            description: Some("Supporting evidence".to_string()),
        };

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_cbk_12345678901234567890123\""));
        assert!(json.contains("\"chargebackMessage\":\"t1_cbm_12345678901234567890123\""));
        assert!(json.contains("\"name\":\"evidence.pdf\""));
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"mimeType\":\"application/pdf\""));
        assert!(json.contains("\"description\":\"Supporting evidence\""));
    }

    #[test]
    fn new_chargeback_document_serialize_minimal() {
        let doc = NewChargebackDocument {
            chargeback: "t1_cbk_12345678901234567890123".to_string(),
            chargeback_message: None,
            name: None,
            document_type: None,
            mime_type: None,
            description: None,
        };

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("\"chargeback\":\"t1_cbk_12345678901234567890123\""));
        assert!(!json.contains("\"chargebackMessage\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"mimeType\""));
        assert!(!json.contains("\"description\""));
    }

    #[test]
    fn new_chargeback_document_default() {
        let doc = NewChargebackDocument::default();
        assert_eq!(doc.chargeback, "");
        assert!(doc.chargeback_message.is_none());
        assert!(doc.name.is_none());
        assert!(doc.document_type.is_none());
        assert!(doc.mime_type.is_none());
        assert!(doc.description.is_none());
    }
}
