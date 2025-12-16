//! Organization types for the Payrix API.
//!
//! Organizations group entities together for administrative purposes.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Organization status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OrgStatus {
    /// Organization is inactive
    Inactive = 0,
    /// Organization is active
    #[default]
    Active = 1,
    /// Organization is suspended
    Suspended = 2,
}

/// A Payrix organization.
///
/// Organizations group entities for management and reporting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Org {
    /// Unique identifier (30 characters, e.g., "t1_org_...")
    pub id: PayrixId,

    /// Parent organization ID (for hierarchies)
    #[serde(default)]
    pub parent: Option<PayrixId>,

    /// Login ID that created this org
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Organization status
    #[serde(default)]
    pub status: Option<OrgStatus>,

    /// Organization name
    #[serde(default)]
    pub name: Option<String>,

    /// Legal/business name
    #[serde(default)]
    pub legal_name: Option<String>,

    /// DBA (doing business as) name
    #[serde(default)]
    pub dba: Option<String>,

    /// Tax ID / EIN
    #[serde(default)]
    pub tax_id: Option<String>,

    /// Website URL
    #[serde(default)]
    pub website: Option<String>,

    /// Primary email
    #[serde(default)]
    pub email: Option<String>,

    /// Primary phone
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

/// Request to create a new organization.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrg {
    /// Parent organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Organization name (required)
    pub name: String,

    /// Legal/business name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,

    /// DBA name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba: Option<String>,

    /// Tax ID / EIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// Website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Primary email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Primary phone
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

    // OrgStatus enum tests
    #[test]
    fn test_org_status_default() {
        assert_eq!(OrgStatus::default(), OrgStatus::Active);
    }

    #[test]
    fn test_org_status_serialize() {
        assert_eq!(serde_json::to_string(&OrgStatus::Inactive).unwrap(), "0");
        assert_eq!(serde_json::to_string(&OrgStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&OrgStatus::Suspended).unwrap(), "2");
    }

    #[test]
    fn test_org_status_deserialize() {
        assert_eq!(serde_json::from_str::<OrgStatus>("0").unwrap(), OrgStatus::Inactive);
        assert_eq!(serde_json::from_str::<OrgStatus>("1").unwrap(), OrgStatus::Active);
        assert_eq!(serde_json::from_str::<OrgStatus>("2").unwrap(), OrgStatus::Suspended);
    }

    // Org struct tests
    #[test]
    fn test_org_deserialize_full() {
        let json = r#"{
            "id": "t1_org_12345678901234567890123",
            "parent": "t1_org_23456789012345678901234",
            "login": "t1_tlg_34567890123456789012345",
            "status": 1,
            "name": "Test Organization",
            "legalName": "Test Org Legal Name LLC",
            "dba": "TestOrg",
            "taxId": "12-3456789",
            "website": "https://example.com",
            "email": "info@example.com",
            "phone": "555-1234",
            "address1": "123 Main St",
            "address2": "Suite 100",
            "city": "New York",
            "state": "NY",
            "zip": "10001",
            "country": "US",
            "description": "Test description",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.id.as_str(), "t1_org_12345678901234567890123");
        assert_eq!(org.parent, Some(PayrixId::new("t1_org_23456789012345678901234").unwrap()));
        assert_eq!(org.login, Some(PayrixId::new("t1_tlg_34567890123456789012345").unwrap()));
        assert_eq!(org.status, Some(OrgStatus::Active));
        assert_eq!(org.name, Some("Test Organization".to_string()));
        assert_eq!(org.legal_name, Some("Test Org Legal Name LLC".to_string()));
        assert_eq!(org.dba, Some("TestOrg".to_string()));
        assert_eq!(org.tax_id, Some("12-3456789".to_string()));
        assert_eq!(org.website, Some("https://example.com".to_string()));
        assert_eq!(org.email, Some("info@example.com".to_string()));
        assert_eq!(org.phone, Some("555-1234".to_string()));
        assert_eq!(org.address1, Some("123 Main St".to_string()));
        assert_eq!(org.address2, Some("Suite 100".to_string()));
        assert_eq!(org.city, Some("New York".to_string()));
        assert_eq!(org.state, Some("NY".to_string()));
        assert_eq!(org.zip, Some("10001".to_string()));
        assert_eq!(org.country, Some("US".to_string()));
        assert_eq!(org.description, Some("Test description".to_string()));
        assert_eq!(org.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(org.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(org.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(org.inactive, false);
        assert_eq!(org.frozen, true);
    }

    #[test]
    fn test_org_deserialize_minimal() {
        let json = r#"{
            "id": "t1_org_12345678901234567890123"
        }"#;

        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.id.as_str(), "t1_org_12345678901234567890123");
        assert_eq!(org.parent, None);
        assert_eq!(org.login, None);
        assert_eq!(org.status, None);
        assert_eq!(org.name, None);
        assert_eq!(org.legal_name, None);
        assert_eq!(org.dba, None);
        assert_eq!(org.tax_id, None);
        assert_eq!(org.website, None);
        assert_eq!(org.email, None);
        assert_eq!(org.phone, None);
        assert_eq!(org.address1, None);
        assert_eq!(org.address2, None);
        assert_eq!(org.city, None);
        assert_eq!(org.state, None);
        assert_eq!(org.zip, None);
        assert_eq!(org.country, None);
        assert_eq!(org.description, None);
        assert_eq!(org.custom, None);
        assert_eq!(org.created, None);
        assert_eq!(org.modified, None);
        assert_eq!(org.inactive, false);
        assert_eq!(org.frozen, false);
    }

    #[test]
    fn test_org_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_org_12345678901234567890123", "inactive": 1}"#;
        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.inactive, true);

        let json = r#"{"id": "t1_org_12345678901234567890123", "inactive": 0}"#;
        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_org_12345678901234567890123", "frozen": 1}"#;
        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.frozen, true);

        let json = r#"{"id": "t1_org_12345678901234567890123", "frozen": 0}"#;
        let org: Org = serde_json::from_str(json).unwrap();
        assert_eq!(org.frozen, false);
    }

    // NewOrg struct tests
    #[test]
    fn test_new_org_serialize_full() {
        let new_org = NewOrg {
            parent: Some("t1_org_23456789012345678901234".to_string()),
            name: "Test Organization".to_string(),
            legal_name: Some("Test Org Legal Name LLC".to_string()),
            dba: Some("TestOrg".to_string()),
            tax_id: Some("12-3456789".to_string()),
            website: Some("https://example.com".to_string()),
            email: Some("info@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            address1: Some("123 Main St".to_string()),
            address2: Some("Suite 100".to_string()),
            city: Some("New York".to_string()),
            state: Some("NY".to_string()),
            zip: Some("10001".to_string()),
            country: Some("US".to_string()),
            description: Some("Test description".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_org).unwrap();
        assert!(json.contains("\"name\":\"Test Organization\""));
        assert!(json.contains("\"parent\":\"t1_org_23456789012345678901234\""));
        assert!(json.contains("\"legalName\":\"Test Org Legal Name LLC\""));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_org_serialize_minimal() {
        let new_org = NewOrg {
            name: "Test Organization".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_org).unwrap();
        assert!(json.contains("\"name\":\"Test Organization\""));
        assert!(!json.contains("parent"));
        assert!(!json.contains("legalName"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_org_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_org = NewOrg {
            name: "Test Org".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_org = NewOrg {
            name: "Test Org".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_org = NewOrg {
            name: "Test Org".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org).unwrap();
        assert!(!json.contains("inactive"));
    }
}
