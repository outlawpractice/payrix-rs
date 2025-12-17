//! Contact types for the Payrix API.
//!
//! Contacts represent individuals associated with entities.
//!
//! **OpenAPI schema:** `contactsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// CONTACT STRUCT
// =============================================================================

/// A Payrix contact.
///
/// Contacts are individuals associated with entities.
///
/// **OpenAPI schema:** `contactsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Contact {
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

    /// The identifier of the Entity that this Contact relates to.
    ///
    /// **OpenAPI type:** string (ref: contactsModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The first name associated with this Contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub first: Option<String>,

    /// The middle name associated with this Contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub middle: Option<String>,

    /// The last name associated with this Contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last: Option<String>,

    /// A description of this Contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The email address of this Contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// The fax number associated with this Contact.
    ///
    /// This field is stored as a text string (10-15 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fax: Option<String>,

    /// The phone number associated with this Contact.
    ///
    /// This field is stored as a text string (10-15 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The country being used for this Contact.
    ///
    /// Default value: `USA`
    ///
    /// **OpenAPI type:** string (ref: Country)
    #[serde(default)]
    pub country: Option<String>,

    /// The ZIP code in the address associated with this Contact.
    ///
    /// This field is stored as a text string (1-20 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The U.S. state or Canadian province relevant to the address.
    ///
    /// Use 2-character postal abbreviation for US/Canada (e.g., "IL", "CA", "ON").
    /// For locations outside US/Canada, provide the full state name.
    /// This field is stored as a text string (2-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The name of the city in the address associated with this Contact.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The second line of the address associated with this Contact.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The first line of the address associated with this Contact.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Contact Struct Tests ====================

    #[test]
    fn contact_deserialize_full() {
        let json = r#"{
            "id": "t1_con_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "entity": "t1_ent_12345678901234567890123",
            "first": "John",
            "middle": "Q",
            "last": "Smith",
            "description": "Primary contact",
            "email": "john@example.com",
            "fax": "5551234567",
            "phone": "5559876543",
            "country": "USA",
            "zip": "60601",
            "state": "IL",
            "city": "Chicago",
            "address2": "Suite 100",
            "address1": "123 Main St",
            "frozen": 0,
            "inactive": 1
        }"#;

        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.id.as_str(), "t1_con_12345678901234567890123");
        assert_eq!(contact.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(contact.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(contact.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(contact.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(contact.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
        assert_eq!(contact.first, Some("John".to_string()));
        assert_eq!(contact.middle, Some("Q".to_string()));
        assert_eq!(contact.last, Some("Smith".to_string()));
        assert_eq!(contact.description, Some("Primary contact".to_string()));
        assert_eq!(contact.email, Some("john@example.com".to_string()));
        assert_eq!(contact.fax, Some("5551234567".to_string()));
        assert_eq!(contact.phone, Some("5559876543".to_string()));
        assert_eq!(contact.country, Some("USA".to_string()));
        assert_eq!(contact.zip, Some("60601".to_string()));
        assert_eq!(contact.state, Some("IL".to_string()));
        assert_eq!(contact.city, Some("Chicago".to_string()));
        assert_eq!(contact.address2, Some("Suite 100".to_string()));
        assert_eq!(contact.address1, Some("123 Main St".to_string()));
        assert!(!contact.frozen);
        assert!(contact.inactive);
    }

    #[test]
    fn contact_deserialize_minimal() {
        let json = r#"{"id": "t1_con_12345678901234567890123"}"#;

        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.id.as_str(), "t1_con_12345678901234567890123");
        assert!(contact.created.is_none());
        assert!(contact.modified.is_none());
        assert!(contact.creator.is_none());
        assert!(contact.modifier.is_none());
        assert!(contact.entity.is_none());
        assert!(contact.first.is_none());
        assert!(contact.middle.is_none());
        assert!(contact.last.is_none());
        assert!(contact.description.is_none());
        assert!(contact.email.is_none());
        assert!(contact.fax.is_none());
        assert!(contact.phone.is_none());
        assert!(contact.country.is_none());
        assert!(contact.zip.is_none());
        assert!(contact.state.is_none());
        assert!(contact.city.is_none());
        assert!(contact.address2.is_none());
        assert!(contact.address1.is_none());
        assert!(!contact.frozen);
        assert!(!contact.inactive);
    }

    #[test]
    fn contact_bool_from_int() {
        let json = r#"{"id": "t1_con_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert!(contact.inactive);
        assert!(contact.frozen);

        let json = r#"{"id": "t1_con_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert!(!contact.inactive);
        assert!(!contact.frozen);
    }

    #[test]
    fn contact_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_con_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "first": "Jane",
            "last": "Doe"
        }"#;

        let contact: Contact = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact.id, deserialized.id);
        assert_eq!(contact.entity, deserialized.entity);
        assert_eq!(contact.first, deserialized.first);
        assert_eq!(contact.last, deserialized.last);
    }
}
