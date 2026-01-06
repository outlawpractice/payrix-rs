//! Note types for the Payrix API.
//!
//! Notes allow adding comments and annotations to various resources.
//!
//! **OpenAPI schema:** `notesResponse`, `noteDocumentsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// NOTE STRUCT
// =============================================================================

/// A Payrix note.
///
/// Notes are comments or annotations attached to holds, transactions,
/// terminal transactions, or entities.
///
/// **OpenAPI schema:** `notesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Note {
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

    /// The identifier of the Login that owns this notes resource.
    ///
    /// **OpenAPI type:** string (ref: notesModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Hold that relates to this notes resource.
    ///
    /// **OpenAPI type:** string (ref: notesModelHold)
    #[serde(default)]
    pub hold: Option<PayrixId>,

    /// The identifier of the Txn that relates to this notes resource.
    ///
    /// **OpenAPI type:** string (ref: notesModelTxn)
    #[serde(default)]
    pub txn: Option<PayrixId>,

    /// The identifier of the TerminalTxn that relates to this notes resource.
    ///
    /// **OpenAPI type:** string (ref: notesModelTerminalTxn)
    #[serde(default)]
    pub terminal_txn: Option<PayrixId>,

    /// The identifier of the Entity that relates to this notes resource.
    ///
    /// **OpenAPI type:** string (ref: notesModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The desired type to take on the referenced Note.
    ///
    /// **OpenAPI type:** string (ref: noteType)
    ///
    /// Valid values: `note`, `release`, `review`, `reReview`, `amexSales`, `businessSales`,
    /// `consumerSales`, `deliverySchedule`, `immediateDeliveryPercent`, `sevenDayDeliveryPercent`,
    /// `fourteenDayDeliveryPercent`, `thirtyDayDeliveryPercent`, `cardPresentSales`, `motoSales`,
    /// `ecommerceSales`, `siteVisit`, `goodsSold`, `authorizationFlatFee`, `capturePercentFee`,
    /// `captureFlatFee`, `riskApproved`, `riskPending`, `riskCancelled`, `riskDenied`,
    /// `riskClosed`, `riskInvestigation`, `riskPendingData`, `riskFundsReleased`, `riskActivityApproved`
    #[serde(default, rename = "type")]
    pub note_type: Option<String>,

    /// A Message/Note regarding this notes resource.
    ///
    /// This field is stored as a text string.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub note: Option<String>,

    /// Free-form text for adding a message along with the type.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub data: Option<String>,

    /// Flag to determine if a note has been pinned or not.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub pinned: Option<i32>,

    /// The timestamp indicating the date and time when a note was pinned.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$`)
    #[serde(default)]
    pub pinned_date: Option<String>,

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
}

// =============================================================================
// NOTE DOCUMENT STRUCT
// =============================================================================

/// A Payrix note document.
///
/// Documents are file attachments for notes.
///
/// **OpenAPI schema:** `noteDocumentsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct NoteDocument {
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

    /// The identifier of the Note that owns this note documents resource.
    ///
    /// **OpenAPI type:** string (ref: noteDocumentsModelNote)
    #[serde(default)]
    pub note: Option<PayrixId>,

    /// The identifier of the Custom that relates to this notes resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub custom: Option<String>,

    /// The desired type to take on the referenced Note Document.
    ///
    /// **OpenAPI type:** string (ref: noteDocumentType)
    ///
    /// Valid values: `jpg`, `jpeg`, `gif`, `png`, `pdf`, `tif`, `tiff`, `txt`, `xml`, `asc`
    #[serde(default, rename = "type")]
    pub document_type: Option<String>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Note Struct Tests ====================

    #[test]
    fn note_deserialize_full() {
        let json = r#"{
            "id": "t1_nte_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "hold": "t1_hld_12345678901234567890123",
            "txn": "t1_txn_12345678901234567890123",
            "terminalTxn": "t1_ttx_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "type": "note",
            "note": "Account reviewed for compliance",
            "data": "Additional details here",
            "pinned": 1,
            "pinnedDate": "2024-01-01 10:00:00",
            "inactive": 0,
            "frozen": 1
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id.as_str(), "t1_nte_12345678901234567890123");
        assert_eq!(note.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(note.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            note.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            note.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            note.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            note.hold.as_ref().map(|h| h.as_str()),
            Some("t1_hld_12345678901234567890123")
        );
        assert_eq!(
            note.txn.as_ref().map(|t| t.as_str()),
            Some("t1_txn_12345678901234567890123")
        );
        assert_eq!(
            note.terminal_txn.as_ref().map(|t| t.as_str()),
            Some("t1_ttx_12345678901234567890123")
        );
        assert_eq!(
            note.entity.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(note.note_type, Some("note".to_string()));
        assert_eq!(
            note.note,
            Some("Account reviewed for compliance".to_string())
        );
        assert_eq!(note.data, Some("Additional details here".to_string()));
        assert_eq!(note.pinned, Some(1));
        assert_eq!(note.pinned_date, Some("2024-01-01 10:00:00".to_string()));
        assert!(!note.inactive);
        assert!(note.frozen);
    }

    #[test]
    fn note_deserialize_minimal() {
        let json = r#"{"id": "t1_nte_12345678901234567890123"}"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id.as_str(), "t1_nte_12345678901234567890123");
        assert!(note.created.is_none());
        assert!(note.modified.is_none());
        assert!(note.creator.is_none());
        assert!(note.modifier.is_none());
        assert!(note.login.is_none());
        assert!(note.hold.is_none());
        assert!(note.txn.is_none());
        assert!(note.terminal_txn.is_none());
        assert!(note.entity.is_none());
        assert!(note.note_type.is_none());
        assert!(note.note.is_none());
        assert!(note.data.is_none());
        assert!(note.pinned.is_none());
        assert!(note.pinned_date.is_none());
        assert!(!note.inactive);
        assert!(!note.frozen);
    }

    #[test]
    fn note_various_types() {
        let types = vec![
            "note",
            "release",
            "review",
            "reReview",
            "riskApproved",
            "riskPending",
            "riskDenied",
        ];

        for note_type in types {
            let json = format!(
                r#"{{"id": "t1_nte_12345678901234567890123", "type": "{}"}}"#,
                note_type
            );
            let note: Note = serde_json::from_str(&json).unwrap();
            assert_eq!(note.note_type, Some(note_type.to_string()));
        }
    }

    #[test]
    fn note_bool_from_int() {
        let json = r#"{"id": "t1_nte_12345678901234567890123", "inactive": 1, "frozen": 0}"#;
        let note: Note = serde_json::from_str(json).unwrap();
        assert!(note.inactive);
        assert!(!note.frozen);
    }

    #[test]
    fn note_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_nte_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "type": "note",
            "note": "Test note"
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&note).unwrap();
        let deserialized: Note = serde_json::from_str(&serialized).unwrap();
        assert_eq!(note.id, deserialized.id);
        assert_eq!(note.entity, deserialized.entity);
        assert_eq!(note.note_type, deserialized.note_type);
        assert_eq!(note.note, deserialized.note);
    }

    // ==================== NoteDocument Tests ====================

    #[test]
    fn note_document_deserialize_full() {
        let json = r#"{
            "id": "t1_ntd_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "note": "t1_nte_12345678901234567890123",
            "custom": "custom data",
            "type": "pdf"
        }"#;

        let doc: NoteDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.id.as_str(), "t1_ntd_12345678901234567890123");
        assert_eq!(doc.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(doc.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            doc.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            doc.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            doc.note.as_ref().map(|n| n.as_str()),
            Some("t1_nte_12345678901234567890123")
        );
        assert_eq!(doc.custom, Some("custom data".to_string()));
        assert_eq!(doc.document_type, Some("pdf".to_string()));
    }

    #[test]
    fn note_document_deserialize_minimal() {
        let json = r#"{"id": "t1_ntd_12345678901234567890123"}"#;

        let doc: NoteDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.id.as_str(), "t1_ntd_12345678901234567890123");
        assert!(doc.created.is_none());
        assert!(doc.modified.is_none());
        assert!(doc.creator.is_none());
        assert!(doc.modifier.is_none());
        assert!(doc.note.is_none());
        assert!(doc.custom.is_none());
        assert!(doc.document_type.is_none());
    }

    #[test]
    fn note_document_various_types() {
        let types = vec!["jpg", "jpeg", "gif", "png", "pdf", "tif", "tiff", "txt", "xml", "asc"];

        for doc_type in types {
            let json = format!(
                r#"{{"id": "t1_ntd_12345678901234567890123", "type": "{}"}}"#,
                doc_type
            );
            let doc: NoteDocument = serde_json::from_str(&json).unwrap();
            assert_eq!(doc.document_type, Some(doc_type.to_string()));
        }
    }

    #[test]
    fn note_document_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_ntd_12345678901234567890123",
            "note": "t1_nte_12345678901234567890123",
            "type": "pdf"
        }"#;

        let doc: NoteDocument = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&doc).unwrap();
        let deserialized: NoteDocument = serde_json::from_str(&serialized).unwrap();
        assert_eq!(doc.id, deserialized.id);
        assert_eq!(doc.note, deserialized.note);
        assert_eq!(doc.document_type, deserialized.document_type);
    }
}
