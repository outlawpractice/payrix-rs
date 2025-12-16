//! Member types for the Payrix API.
//!
//! Members represent beneficial owners and key individuals associated with merchants.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, DateYmd, PayrixId};

/// Member type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MemberType {
    /// Beneficial owner
    #[default]
    Owner = 1,
    /// Control person / authorized signer
    ControlPerson = 2,
    /// Principal
    Principal = 3,
}

/// A Payrix member.
///
/// Members are individuals associated with a merchant entity (owners, officers, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Member {
    /// Unique identifier (30 characters, e.g., "t1_mem_...")
    pub id: PayrixId,

    /// Entity ID this member belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this member
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Member type
    #[serde(default, rename = "type")]
    pub member_type: Option<MemberType>,

    /// First name
    #[serde(default)]
    pub first: Option<String>,

    /// Middle name
    #[serde(default)]
    pub middle: Option<String>,

    /// Last name
    #[serde(default)]
    pub last: Option<String>,

    /// Title/position
    #[serde(default)]
    pub title: Option<String>,

    /// Ownership percentage (0-100)
    #[serde(default)]
    pub ownership: Option<i32>,

    /// Date of birth (YYYYMMDD format)
    #[serde(default)]
    pub dob: Option<DateYmd>,

    /// SSN (last 4 digits only typically returned)
    #[serde(default)]
    pub ssn: Option<String>,

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

    /// State/province
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(default)]
    pub zip: Option<String>,

    /// Country code
    #[serde(default)]
    pub country: Option<String>,

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

/// Request to create a new member.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMember {
    /// Entity ID (required)
    pub entity: String,

    /// Member type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub member_type: Option<MemberType>,

    /// First name (required)
    pub first: String,

    /// Middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// Last name (required)
    pub last: String,

    /// Title/position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Ownership percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<i32>,

    /// Date of birth (YYYYMMDD format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<String>,

    /// SSN (full SSN for new members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

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

    // MemberType enum tests
    #[test]
    fn test_member_type_default() {
        assert_eq!(MemberType::default(), MemberType::Owner);
    }

    #[test]
    fn test_member_type_serialize() {
        assert_eq!(serde_json::to_string(&MemberType::Owner).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MemberType::ControlPerson).unwrap(), "2");
        assert_eq!(serde_json::to_string(&MemberType::Principal).unwrap(), "3");
    }

    #[test]
    fn test_member_type_deserialize() {
        assert_eq!(serde_json::from_str::<MemberType>("1").unwrap(), MemberType::Owner);
        assert_eq!(serde_json::from_str::<MemberType>("2").unwrap(), MemberType::ControlPerson);
        assert_eq!(serde_json::from_str::<MemberType>("3").unwrap(), MemberType::Principal);
    }

    // Member struct tests
    #[test]
    fn test_member_deserialize_full() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "entity": "t1_ent_23456789012345678901234",
            "merchant": "t1_mer_34567890123456789012345",
            "login": "t1_tlg_45678901234567890123456",
            "type": 1,
            "first": "John",
            "middle": "A",
            "last": "Doe",
            "title": "CEO",
            "ownership": 50,
            "dob": "20000115",
            "ssn": "1234",
            "email": "john@example.com",
            "phone": "555-1234",
            "address1": "123 Main St",
            "address2": "Apt 4B",
            "city": "New York",
            "state": "NY",
            "zip": "10001",
            "country": "US",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert_eq!(member.entity, Some(PayrixId::new("t1_ent_23456789012345678901234").unwrap()));
        assert_eq!(member.merchant, Some(PayrixId::new("t1_mer_34567890123456789012345").unwrap()));
        assert_eq!(member.login, Some(PayrixId::new("t1_tlg_45678901234567890123456").unwrap()));
        assert_eq!(member.member_type, Some(MemberType::Owner));
        assert_eq!(member.first, Some("John".to_string()));
        assert_eq!(member.middle, Some("A".to_string()));
        assert_eq!(member.last, Some("Doe".to_string()));
        assert_eq!(member.title, Some("CEO".to_string()));
        assert_eq!(member.ownership, Some(50));
        assert_eq!(member.dob, Some(DateYmd::new("20000115").unwrap()));
        assert_eq!(member.ssn, Some("1234".to_string()));
        assert_eq!(member.email, Some("john@example.com".to_string()));
        assert_eq!(member.phone, Some("555-1234".to_string()));
        assert_eq!(member.address1, Some("123 Main St".to_string()));
        assert_eq!(member.address2, Some("Apt 4B".to_string()));
        assert_eq!(member.city, Some("New York".to_string()));
        assert_eq!(member.state, Some("NY".to_string()));
        assert_eq!(member.zip, Some("10001".to_string()));
        assert_eq!(member.country, Some("US".to_string()));
        assert_eq!(member.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(member.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(member.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(member.inactive, false);
        assert_eq!(member.frozen, true);
    }

    #[test]
    fn test_member_deserialize_minimal() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123"
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert_eq!(member.entity, None);
        assert_eq!(member.merchant, None);
        assert_eq!(member.login, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.first, None);
        assert_eq!(member.middle, None);
        assert_eq!(member.last, None);
        assert_eq!(member.title, None);
        assert_eq!(member.ownership, None);
        assert_eq!(member.dob, None);
        assert_eq!(member.ssn, None);
        assert_eq!(member.email, None);
        assert_eq!(member.phone, None);
        assert_eq!(member.address1, None);
        assert_eq!(member.address2, None);
        assert_eq!(member.city, None);
        assert_eq!(member.state, None);
        assert_eq!(member.zip, None);
        assert_eq!(member.country, None);
        assert_eq!(member.custom, None);
        assert_eq!(member.created, None);
        assert_eq!(member.modified, None);
        assert_eq!(member.inactive, false);
        assert_eq!(member.frozen, false);
    }

    #[test]
    fn test_member_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_mem_12345678901234567890123", "inactive": 1}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.inactive, true);

        let json = r#"{"id": "t1_mem_12345678901234567890123", "inactive": 0}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_mem_12345678901234567890123", "frozen": 1}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.frozen, true);

        let json = r#"{"id": "t1_mem_12345678901234567890123", "frozen": 0}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.frozen, false);
    }

    // NewMember struct tests
    #[test]
    fn test_new_member_serialize_full() {
        let new_member = NewMember {
            entity: "t1_ent_23456789012345678901234".to_string(),
            member_type: Some(MemberType::Owner),
            first: "John".to_string(),
            middle: Some("A".to_string()),
            last: "Doe".to_string(),
            title: Some("CEO".to_string()),
            ownership: Some(50),
            dob: Some("20000115".to_string()),
            ssn: Some("123456789".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            address1: Some("123 Main St".to_string()),
            address2: Some("Apt 4B".to_string()),
            city: Some("New York".to_string()),
            state: Some("NY".to_string()),
            zip: Some("10001".to_string()),
            country: Some("US".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_member).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"last\":\"Doe\""));
        assert!(json.contains("\"ownership\":50"));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_member_serialize_minimal() {
        let new_member = NewMember {
            entity: "t1_ent_23456789012345678901234".to_string(),
            first: "John".to_string(),
            last: "Doe".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_member).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"last\":\"Doe\""));
        assert!(!json.contains("type"));
        assert!(!json.contains("middle"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_member_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_member = NewMember {
            entity: "t1_ent_23456789012345678901234".to_string(),
            first: "John".to_string(),
            last: "Doe".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_member).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_member = NewMember {
            entity: "t1_ent_23456789012345678901234".to_string(),
            first: "John".to_string(),
            last: "Doe".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_member).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_member = NewMember {
            entity: "t1_ent_23456789012345678901234".to_string(),
            first: "John".to_string(),
            last: "Doe".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_member).unwrap();
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_member_serialize_member_type() {
        // Test all member type variants
        let variants = vec![
            (MemberType::Owner, "1"),
            (MemberType::ControlPerson, "2"),
            (MemberType::Principal, "3"),
        ];

        for (variant, expected) in variants {
            let new_member = NewMember {
                entity: "t1_ent_23456789012345678901234".to_string(),
                first: "John".to_string(),
                last: "Doe".to_string(),
                member_type: Some(variant),
                ..Default::default()
            };
            let json = serde_json::to_string(&new_member).unwrap();
            assert!(json.contains(&format!("\"type\":{}", expected)));
        }
    }
}
