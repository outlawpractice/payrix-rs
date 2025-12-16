//! Contact types for the Payrix API.
//!
//! Contacts represent individuals associated with entities or merchants.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Contact type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ContactType {
    /// Primary contact
    #[default]
    Primary = 1,
    /// Billing contact
    Billing = 2,
    /// Technical contact
    Technical = 3,
    /// Support contact
    Support = 4,
}

/// A Payrix contact.
///
/// Contacts are individuals associated with entities or merchants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// Unique identifier (30 characters, e.g., "t1_con_...")
    pub id: PayrixId,

    /// Entity ID this contact belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this contact
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Contact type
    #[serde(default, rename = "type")]
    pub contact_type: Option<ContactType>,

    /// First name
    #[serde(default)]
    pub first: Option<String>,

    /// Last name
    #[serde(default)]
    pub last: Option<String>,

    /// Title/position
    #[serde(default)]
    pub title: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Primary phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// Alternate phone number
    #[serde(default)]
    pub phone_alt: Option<String>,

    /// Fax number
    #[serde(default)]
    pub fax: Option<String>,

    /// Address line 1
    #[serde(default)]
    pub address1: Option<String>,

    /// Address line 2
    #[serde(default)]
    pub address2: Option<String>,

    /// City
    #[serde(default)]
    pub city: Option<String>,

    /// State/province
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(default)]
    pub zip: Option<String>,

    /// Country code
    #[serde(default)]
    pub country: Option<String>,

    /// Description/notes
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

/// Request to create a new contact.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContact {
    /// Entity ID (required)
    pub entity: String,

    /// Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Contact type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub contact_type: Option<ContactType>,

    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Title/position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Primary phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Alternate phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_alt: Option<String>,

    /// Fax number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

    /// Address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,

    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// State/province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// Country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Description/notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether resource is inactive
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // ContactType enum tests
    #[test]
    fn test_contact_type_default() {
        assert_eq!(ContactType::default(), ContactType::Primary);
    }

    #[test]
    fn test_contact_type_serialize() {
        assert_eq!(serde_json::to_string(&ContactType::Primary).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ContactType::Billing).unwrap(), "2");
        assert_eq!(serde_json::to_string(&ContactType::Technical).unwrap(), "3");
        assert_eq!(serde_json::to_string(&ContactType::Support).unwrap(), "4");
    }

    #[test]
    fn test_contact_type_deserialize() {
        assert_eq!(serde_json::from_str::<ContactType>("1").unwrap(), ContactType::Primary);
        assert_eq!(serde_json::from_str::<ContactType>("2").unwrap(), ContactType::Billing);
        assert_eq!(serde_json::from_str::<ContactType>("3").unwrap(), ContactType::Technical);
        assert_eq!(serde_json::from_str::<ContactType>("4").unwrap(), ContactType::Support);
    }

    // Contact struct tests
    #[test]
    fn test_contact_deserialize_full() {
        let json = r#"{
            "id": "t1_con_12345678901234567890123",
            "entity": "t1_ent_23456789012345678901234",
            "merchant": "t1_mer_34567890123456789012345",
            "login": "t1_tlg_45678901234567890123456",
            "type": 1,
            "first": "Jane",
            "last": "Smith",
            "title": "CFO",
            "email": "jane@example.com",
            "phone": "555-1234",
            "phoneAlt": "555-5678",
            "fax": "555-9999",
            "address1": "456 Contact Ave",
            "address2": "Floor 3",
            "city": "Chicago",
            "state": "IL",
            "zip": "60601",
            "country": "US",
            "description": "Primary contact for billing",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.id.as_str(), "t1_con_12345678901234567890123");
        assert_eq!(contact.entity, Some(PayrixId::new("t1_ent_23456789012345678901234").unwrap()));
        assert_eq!(contact.merchant, Some(PayrixId::new("t1_mer_34567890123456789012345").unwrap()));
        assert_eq!(contact.login, Some(PayrixId::new("t1_tlg_45678901234567890123456").unwrap()));
        assert_eq!(contact.contact_type, Some(ContactType::Primary));
        assert_eq!(contact.first, Some("Jane".to_string()));
        assert_eq!(contact.last, Some("Smith".to_string()));
        assert_eq!(contact.title, Some("CFO".to_string()));
        assert_eq!(contact.email, Some("jane@example.com".to_string()));
        assert_eq!(contact.phone, Some("555-1234".to_string()));
        assert_eq!(contact.phone_alt, Some("555-5678".to_string()));
        assert_eq!(contact.fax, Some("555-9999".to_string()));
        assert_eq!(contact.address1, Some("456 Contact Ave".to_string()));
        assert_eq!(contact.address2, Some("Floor 3".to_string()));
        assert_eq!(contact.city, Some("Chicago".to_string()));
        assert_eq!(contact.state, Some("IL".to_string()));
        assert_eq!(contact.zip, Some("60601".to_string()));
        assert_eq!(contact.country, Some("US".to_string()));
        assert_eq!(contact.description, Some("Primary contact for billing".to_string()));
        assert_eq!(contact.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(contact.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(contact.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(contact.inactive, false);
        assert_eq!(contact.frozen, true);
    }

    #[test]
    fn test_contact_deserialize_minimal() {
        let json = r#"{
            "id": "t1_con_12345678901234567890123"
        }"#;

        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.id.as_str(), "t1_con_12345678901234567890123");
        assert_eq!(contact.entity, None);
        assert_eq!(contact.merchant, None);
        assert_eq!(contact.login, None);
        assert_eq!(contact.contact_type, None);
        assert_eq!(contact.first, None);
        assert_eq!(contact.last, None);
        assert_eq!(contact.title, None);
        assert_eq!(contact.email, None);
        assert_eq!(contact.phone, None);
        assert_eq!(contact.phone_alt, None);
        assert_eq!(contact.fax, None);
        assert_eq!(contact.address1, None);
        assert_eq!(contact.address2, None);
        assert_eq!(contact.city, None);
        assert_eq!(contact.state, None);
        assert_eq!(contact.zip, None);
        assert_eq!(contact.country, None);
        assert_eq!(contact.description, None);
        assert_eq!(contact.custom, None);
        assert_eq!(contact.created, None);
        assert_eq!(contact.modified, None);
        assert_eq!(contact.inactive, false);
        assert_eq!(contact.frozen, false);
    }

    #[test]
    fn test_contact_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_con_12345678901234567890123", "inactive": 1}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.inactive, true);

        let json = r#"{"id": "t1_con_12345678901234567890123", "inactive": 0}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_con_12345678901234567890123", "frozen": 1}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.frozen, true);

        let json = r#"{"id": "t1_con_12345678901234567890123", "frozen": 0}"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.frozen, false);
    }

    // NewContact struct tests
    #[test]
    fn test_new_contact_serialize_full() {
        let new_contact = NewContact {
            entity: "t1_ent_23456789012345678901234".to_string(),
            merchant: Some("t1_mer_34567890123456789012345".to_string()),
            contact_type: Some(ContactType::Billing),
            first: Some("Jane".to_string()),
            last: Some("Smith".to_string()),
            title: Some("CFO".to_string()),
            email: Some("jane@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            phone_alt: Some("555-5678".to_string()),
            fax: Some("555-9999".to_string()),
            address1: Some("456 Contact Ave".to_string()),
            address2: Some("Floor 3".to_string()),
            city: Some("Chicago".to_string()),
            state: Some("IL".to_string()),
            zip: Some("60601".to_string()),
            country: Some("US".to_string()),
            description: Some("Primary contact for billing".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_contact).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"merchant\":\"t1_mer_34567890123456789012345\""));
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"first\":\"Jane\""));
        assert!(json.contains("\"last\":\"Smith\""));
        assert!(json.contains("\"phoneAlt\":\"555-5678\""));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_contact_serialize_minimal() {
        let new_contact = NewContact {
            entity: "t1_ent_23456789012345678901234".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_contact).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(!json.contains("merchant"));
        assert!(!json.contains("type"));
        assert!(!json.contains("first"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_contact_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_contact = NewContact {
            entity: "t1_ent_23456789012345678901234".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_contact).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_contact = NewContact {
            entity: "t1_ent_23456789012345678901234".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_contact).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_contact = NewContact {
            entity: "t1_ent_23456789012345678901234".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_contact).unwrap();
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_contact_serialize_contact_type() {
        // Test all contact type variants
        let variants = vec![
            (ContactType::Primary, "1"),
            (ContactType::Billing, "2"),
            (ContactType::Technical, "3"),
            (ContactType::Support, "4"),
        ];

        for (variant, expected) in variants {
            let new_contact = NewContact {
                entity: "t1_ent_23456789012345678901234".to_string(),
                contact_type: Some(variant),
                ..Default::default()
            };
            let json = serde_json::to_string(&new_contact).unwrap();
            assert!(json.contains(&format!("\"type\":{}", expected)));
        }
    }
}
