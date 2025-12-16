//! Note types for the Payrix API.
//!
//! Notes allow adding comments and annotations to various resources.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Note type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum NoteType {
    /// General note
    #[default]
    General = 1,
    /// Customer service note
    CustomerService = 2,
    /// Risk/compliance note
    Risk = 3,
    /// Internal note
    Internal = 4,
    /// System-generated note
    System = 5,
}

/// Note visibility values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum NoteVisibility {
    /// Private (only creator can see)
    #[default]
    Private = 1,
    /// Internal (organization members can see)
    Internal = 2,
    /// Public (visible to all with access to the resource)
    Public = 3,
}

/// A Payrix note.
///
/// Notes are comments or annotations attached to resources like
/// merchants, transactions, chargebacks, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// Unique identifier (30 characters, e.g., "t1_nte_...")
    pub id: PayrixId,

    /// Entity ID this note belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Login ID that created this note
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Resource type this note is attached to (e.g., "merchant", "txn", "chargeback")
    #[serde(default)]
    pub for_type: Option<String>,

    /// Resource ID this note is attached to
    #[serde(default)]
    pub for_id: Option<PayrixId>,

    /// Note type
    #[serde(default, rename = "type")]
    pub note_type: Option<NoteType>,

    /// Note visibility
    #[serde(default)]
    pub visibility: Option<NoteVisibility>,

    /// Note subject/title
    #[serde(default)]
    pub subject: Option<String>,

    /// Note body/content
    #[serde(default)]
    pub body: Option<String>,

    /// Whether the note is pinned/sticky
    #[serde(default, with = "bool_from_int_default_false")]
    pub pinned: bool,

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

/// Request to create a new note.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Resource type this note is attached to (required)
    pub for_type: String,

    /// Resource ID this note is attached to (required)
    pub for_id: String,

    /// Note type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub note_type: Option<NoteType>,

    /// Note visibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<NoteVisibility>,

    /// Note subject/title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Note body/content (required)
    pub body: String,

    /// Whether the note is pinned/sticky
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub pinned: Option<bool>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

/// Document type values for note documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum NoteDocumentType {
    /// Image file
    #[default]
    Image = 1,
    /// PDF document
    Pdf = 2,
    /// Text file
    Text = 3,
    /// Spreadsheet
    Spreadsheet = 4,
    /// Other/generic
    Other = 5,
}

/// A Payrix note document.
///
/// Documents are file attachments for notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct NoteDocument {
    /// Unique identifier (30 characters, e.g., "t1_ntd_...")
    pub id: PayrixId,

    /// Note ID this document belongs to (required)
    pub note: PayrixId,

    /// Login ID that uploaded this document
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Document name/filename
    #[serde(default)]
    pub name: Option<String>,

    /// Document type
    #[serde(default, rename = "type")]
    pub document_type: Option<NoteDocumentType>,

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

/// Request to create a new note document.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNoteDocument {
    /// Note ID (required)
    pub note: String,

    /// Document name/filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Document type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub document_type: Option<NoteDocumentType>,

    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Document description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // ==================== Enum Tests ====================

    #[test]
    fn note_type_default() {
        assert_eq!(NoteType::default(), NoteType::General);
    }

    #[test]
    fn note_type_all_variants_serialize() {
        let test_cases = [
            (NoteType::General, "1"),
            (NoteType::CustomerService, "2"),
            (NoteType::Risk, "3"),
            (NoteType::Internal, "4"),
            (NoteType::System, "5"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn note_type_all_variants_deserialize() {
        let test_cases = [
            ("1", NoteType::General),
            ("2", NoteType::CustomerService),
            ("3", NoteType::Risk),
            ("4", NoteType::Internal),
            ("5", NoteType::System),
        ];

        for (json, expected_variant) in test_cases {
            let variant: NoteType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn note_visibility_default() {
        assert_eq!(NoteVisibility::default(), NoteVisibility::Private);
    }

    #[test]
    fn note_visibility_all_variants_serialize() {
        let test_cases = [
            (NoteVisibility::Private, "1"),
            (NoteVisibility::Internal, "2"),
            (NoteVisibility::Public, "3"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn note_visibility_all_variants_deserialize() {
        let test_cases = [
            ("1", NoteVisibility::Private),
            ("2", NoteVisibility::Internal),
            ("3", NoteVisibility::Public),
        ];

        for (json, expected_variant) in test_cases {
            let variant: NoteVisibility = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    #[test]
    fn note_document_type_default() {
        assert_eq!(NoteDocumentType::default(), NoteDocumentType::Image);
    }

    #[test]
    fn note_document_type_all_variants_serialize() {
        let test_cases = [
            (NoteDocumentType::Image, "1"),
            (NoteDocumentType::Pdf, "2"),
            (NoteDocumentType::Text, "3"),
            (NoteDocumentType::Spreadsheet, "4"),
            (NoteDocumentType::Other, "5"),
        ];

        for (variant, expected_json) in test_cases {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
        }
    }

    #[test]
    fn note_document_type_all_variants_deserialize() {
        let test_cases = [
            ("1", NoteDocumentType::Image),
            ("2", NoteDocumentType::Pdf),
            ("3", NoteDocumentType::Text),
            ("4", NoteDocumentType::Spreadsheet),
            ("5", NoteDocumentType::Other),
        ];

        for (json, expected_variant) in test_cases {
            let variant: NoteDocumentType = serde_json::from_str(json).unwrap();
            assert_eq!(variant, expected_variant);
        }
    }

    // ==================== Note Struct Tests ====================

    #[test]
    fn note_deserialize_full() {
        let json = r#"{
            "id": "t1_nte_12345678901234567890123",
            "entity": "t1_ety_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "forType": "merchant",
            "forId": "t1_mer_12345678901234567890123",
            "type": 2,
            "visibility": 3,
            "subject": "Account Review",
            "body": "Reviewed account for compliance",
            "pinned": 1,
            "custom": "custom_data",
            "created": "2024-01-01 10:00:00.000",
            "modified": "2024-01-01 15:30:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id.as_str(), "t1_nte_12345678901234567890123");
        assert_eq!(note.entity.as_ref().unwrap().as_str(), "t1_ety_12345678901234567890123");
        assert_eq!(note.login.as_ref().unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(note.for_type.as_ref().unwrap(), "merchant");
        assert_eq!(note.for_id.as_ref().unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(note.note_type.unwrap(), NoteType::CustomerService);
        assert_eq!(note.visibility.unwrap(), NoteVisibility::Public);
        assert_eq!(note.subject.as_ref().unwrap(), "Account Review");
        assert_eq!(note.body.as_ref().unwrap(), "Reviewed account for compliance");
        assert_eq!(note.pinned, true);
        assert_eq!(note.custom.as_ref().unwrap(), "custom_data");
        assert_eq!(note.created.as_ref().unwrap(), "2024-01-01 10:00:00.000");
        assert_eq!(note.modified.as_ref().unwrap(), "2024-01-01 15:30:00.000");
        assert_eq!(note.inactive, false);
        assert_eq!(note.frozen, true);
    }

    #[test]
    fn note_deserialize_minimal() {
        let json = r#"{
            "id": "t1_nte_12345678901234567890123"
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id.as_str(), "t1_nte_12345678901234567890123");
        assert!(note.entity.is_none());
        assert!(note.login.is_none());
        assert!(note.for_type.is_none());
        assert!(note.for_id.is_none());
        assert!(note.note_type.is_none());
        assert!(note.visibility.is_none());
        assert!(note.subject.is_none());
        assert!(note.body.is_none());
        assert_eq!(note.pinned, false);
        assert!(note.custom.is_none());
        assert!(note.created.is_none());
        assert!(note.modified.is_none());
        assert_eq!(note.inactive, false);
        assert_eq!(note.frozen, false);
    }

    #[test]
    fn note_deserialize_bool_from_int() {
        // Test bool_from_int deserialization
        let json = r#"{
            "id": "t1_nte_12345678901234567890123",
            "pinned": 1,
            "inactive": 1,
            "frozen": 0
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.pinned, true);
        assert_eq!(note.inactive, true);
        assert_eq!(note.frozen, false);
    }

    // ==================== NewNote Tests ====================

    #[test]
    fn new_note_serialize_full() {
        let note = NewNote {
            entity: Some("t1_ety_12345678901234567890123".to_string()),
            for_type: "merchant".to_string(),
            for_id: "t1_mer_12345678901234567890123".to_string(),
            note_type: Some(NoteType::Risk),
            visibility: Some(NoteVisibility::Internal),
            subject: Some("Risk Assessment".to_string()),
            body: "Risk assessment completed".to_string(),
            pinned: Some(true),
            custom: Some("custom_data".to_string()),
        };

        let json = serde_json::to_string(&note).unwrap();
        assert!(json.contains("\"entity\":\"t1_ety_12345678901234567890123\""));
        assert!(json.contains("\"forType\":\"merchant\""));
        assert!(json.contains("\"forId\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"type\":3"));
        assert!(json.contains("\"visibility\":2"));
        assert!(json.contains("\"subject\":\"Risk Assessment\""));
        assert!(json.contains("\"body\":\"Risk assessment completed\""));
        assert!(json.contains("\"pinned\":1"));
        assert!(json.contains("\"custom\":\"custom_data\""));
    }

    #[test]
    fn new_note_serialize_minimal() {
        let note = NewNote {
            entity: None,
            for_type: "merchant".to_string(),
            for_id: "t1_mer_12345678901234567890123".to_string(),
            note_type: None,
            visibility: None,
            subject: None,
            body: "Simple note".to_string(),
            pinned: None,
            custom: None,
        };

        let json = serde_json::to_string(&note).unwrap();
        assert!(!json.contains("\"entity\""));
        assert!(json.contains("\"forType\":\"merchant\""));
        assert!(json.contains("\"forId\":\"t1_mer_12345678901234567890123\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"visibility\""));
        assert!(!json.contains("\"subject\""));
        assert!(json.contains("\"body\":\"Simple note\""));
        assert!(!json.contains("\"pinned\""));
        assert!(!json.contains("\"custom\""));
    }

    #[test]
    fn new_note_serialize_pinned_false() {
        let note = NewNote {
            entity: None,
            for_type: "merchant".to_string(),
            for_id: "t1_mer_12345678901234567890123".to_string(),
            note_type: None,
            visibility: None,
            subject: None,
            body: "Note".to_string(),
            pinned: Some(false),
            custom: None,
        };

        let json = serde_json::to_string(&note).unwrap();
        assert!(json.contains("\"pinned\":0"));
    }

    #[test]
    fn new_note_default() {
        let note = NewNote::default();
        assert!(note.entity.is_none());
        assert_eq!(note.for_type, "");
        assert_eq!(note.for_id, "");
        assert!(note.note_type.is_none());
        assert!(note.visibility.is_none());
        assert!(note.subject.is_none());
        assert_eq!(note.body, "");
        assert!(note.pinned.is_none());
        assert!(note.custom.is_none());
    }

    // ==================== NoteDocument Tests ====================

    #[test]
    fn note_document_deserialize_full() {
        let json = r#"{
            "id": "t1_ntd_12345678901234567890123",
            "note": "t1_nte_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "name": "attachment.pdf",
            "type": 2,
            "mimeType": "application/pdf",
            "size": 102400,
            "url": "https://example.com/files/attachment.pdf",
            "description": "Supporting document",
            "created": "2024-01-01 10:00:00.000",
            "modified": "2024-01-01 15:30:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let doc: NoteDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.id.as_str(), "t1_ntd_12345678901234567890123");
        assert_eq!(doc.note.as_str(), "t1_nte_12345678901234567890123");
        assert_eq!(doc.login.as_ref().unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(doc.name.as_ref().unwrap(), "attachment.pdf");
        assert_eq!(doc.document_type.unwrap(), NoteDocumentType::Pdf);
        assert_eq!(doc.mime_type.as_ref().unwrap(), "application/pdf");
        assert_eq!(doc.size.unwrap(), 102400);
        assert_eq!(doc.url.as_ref().unwrap(), "https://example.com/files/attachment.pdf");
        assert_eq!(doc.description.as_ref().unwrap(), "Supporting document");
        assert_eq!(doc.created.as_ref().unwrap(), "2024-01-01 10:00:00.000");
        assert_eq!(doc.modified.as_ref().unwrap(), "2024-01-01 15:30:00.000");
        assert_eq!(doc.inactive, false);
        assert_eq!(doc.frozen, true);
    }

    #[test]
    fn note_document_deserialize_minimal() {
        let json = r#"{
            "id": "t1_ntd_12345678901234567890123",
            "note": "t1_nte_12345678901234567890123"
        }"#;

        let doc: NoteDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.id.as_str(), "t1_ntd_12345678901234567890123");
        assert_eq!(doc.note.as_str(), "t1_nte_12345678901234567890123");
        assert!(doc.login.is_none());
        assert!(doc.name.is_none());
        assert!(doc.document_type.is_none());
        assert!(doc.mime_type.is_none());
        assert!(doc.size.is_none());
        assert!(doc.url.is_none());
        assert!(doc.description.is_none());
        assert!(doc.created.is_none());
        assert!(doc.modified.is_none());
        assert_eq!(doc.inactive, false);
        assert_eq!(doc.frozen, false);
    }

    // ==================== NewNoteDocument Tests ====================

    #[test]
    fn new_note_document_serialize_full() {
        let doc = NewNoteDocument {
            note: "t1_nte_12345678901234567890123".to_string(),
            name: Some("document.xlsx".to_string()),
            document_type: Some(NoteDocumentType::Spreadsheet),
            mime_type: Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()),
            description: Some("Financial data".to_string()),
        };

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("\"note\":\"t1_nte_12345678901234567890123\""));
        assert!(json.contains("\"name\":\"document.xlsx\""));
        assert!(json.contains("\"type\":4"));
        assert!(json.contains("\"mimeType\":\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\""));
        assert!(json.contains("\"description\":\"Financial data\""));
    }

    #[test]
    fn new_note_document_serialize_minimal() {
        let doc = NewNoteDocument {
            note: "t1_nte_12345678901234567890123".to_string(),
            name: None,
            document_type: None,
            mime_type: None,
            description: None,
        };

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("\"note\":\"t1_nte_12345678901234567890123\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"mimeType\""));
        assert!(!json.contains("\"description\""));
    }

    #[test]
    fn new_note_document_default() {
        let doc = NewNoteDocument::default();
        assert_eq!(doc.note, "");
        assert!(doc.name.is_none());
        assert!(doc.document_type.is_none());
        assert!(doc.mime_type.is_none());
        assert!(doc.description.is_none());
    }
}
