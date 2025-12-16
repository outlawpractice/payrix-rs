//! Vendor types for the Payrix API.
//!
//! Vendors represent third-party recipients for split payments or payouts.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Vendor status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum VendorStatus {
    /// Vendor is inactive
    Inactive = 0,
    /// Vendor is active
    #[default]
    Active = 1,
    /// Vendor is suspended
    Suspended = 2,
}

/// A Payrix vendor.
///
/// Vendors are recipients for split payments or payouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Vendor {
    /// Unique identifier (30 characters, e.g., "t1_vnd_...")
    pub id: PayrixId,

    /// Entity ID this vendor belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Account ID for payouts
    #[serde(default)]
    pub account: Option<PayrixId>,

    /// Login ID that created this vendor
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Vendor status
    #[serde(default)]
    pub status: Option<VendorStatus>,

    /// Vendor name
    #[serde(default)]
    pub name: Option<String>,

    /// Legal/business name
    #[serde(default)]
    pub legal_name: Option<String>,

    /// DBA name
    #[serde(default)]
    pub dba: Option<String>,

    /// Tax ID / EIN
    #[serde(default)]
    pub tax_id: Option<String>,

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

/// Request to create a new vendor.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewVendor {
    /// Entity ID (required)
    pub entity: String,

    /// Account ID for payouts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Vendor name (required)
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

    // VendorStatus enum tests
    #[test]
    fn test_vendor_status_default() {
        assert_eq!(VendorStatus::default(), VendorStatus::Active);
    }

    #[test]
    fn test_vendor_status_serialize() {
        assert_eq!(serde_json::to_string(&VendorStatus::Inactive).unwrap(), "0");
        assert_eq!(serde_json::to_string(&VendorStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&VendorStatus::Suspended).unwrap(), "2");
    }

    #[test]
    fn test_vendor_status_deserialize() {
        assert_eq!(serde_json::from_str::<VendorStatus>("0").unwrap(), VendorStatus::Inactive);
        assert_eq!(serde_json::from_str::<VendorStatus>("1").unwrap(), VendorStatus::Active);
        assert_eq!(serde_json::from_str::<VendorStatus>("2").unwrap(), VendorStatus::Suspended);
    }

    // Vendor struct tests
    #[test]
    fn test_vendor_deserialize_full() {
        let json = r#"{
            "id": "t1_vnd_12345678901234567890123",
            "entity": "t1_ent_23456789012345678901234",
            "merchant": "t1_mer_34567890123456789012345",
            "account": "t1_acc_45678901234567890123456",
            "login": "t1_tlg_56789012345678901234567",
            "status": 1,
            "name": "Test Vendor",
            "legalName": "Test Vendor Legal Name LLC",
            "dba": "TestVendor",
            "taxId": "12-3456789",
            "email": "vendor@example.com",
            "phone": "555-1234",
            "address1": "123 Vendor St",
            "address2": "Suite 200",
            "city": "Los Angeles",
            "state": "CA",
            "zip": "90001",
            "country": "US",
            "description": "Test vendor description",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.id.as_str(), "t1_vnd_12345678901234567890123");
        assert_eq!(vendor.entity, Some(PayrixId::new("t1_ent_23456789012345678901234").unwrap()));
        assert_eq!(vendor.merchant, Some(PayrixId::new("t1_mer_34567890123456789012345").unwrap()));
        assert_eq!(vendor.account, Some(PayrixId::new("t1_acc_45678901234567890123456").unwrap()));
        assert_eq!(vendor.login, Some(PayrixId::new("t1_tlg_56789012345678901234567").unwrap()));
        assert_eq!(vendor.status, Some(VendorStatus::Active));
        assert_eq!(vendor.name, Some("Test Vendor".to_string()));
        assert_eq!(vendor.legal_name, Some("Test Vendor Legal Name LLC".to_string()));
        assert_eq!(vendor.dba, Some("TestVendor".to_string()));
        assert_eq!(vendor.tax_id, Some("12-3456789".to_string()));
        assert_eq!(vendor.email, Some("vendor@example.com".to_string()));
        assert_eq!(vendor.phone, Some("555-1234".to_string()));
        assert_eq!(vendor.address1, Some("123 Vendor St".to_string()));
        assert_eq!(vendor.address2, Some("Suite 200".to_string()));
        assert_eq!(vendor.city, Some("Los Angeles".to_string()));
        assert_eq!(vendor.state, Some("CA".to_string()));
        assert_eq!(vendor.zip, Some("90001".to_string()));
        assert_eq!(vendor.country, Some("US".to_string()));
        assert_eq!(vendor.description, Some("Test vendor description".to_string()));
        assert_eq!(vendor.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(vendor.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(vendor.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(vendor.inactive, false);
        assert_eq!(vendor.frozen, true);
    }

    #[test]
    fn test_vendor_deserialize_minimal() {
        let json = r#"{
            "id": "t1_vnd_12345678901234567890123"
        }"#;

        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.id.as_str(), "t1_vnd_12345678901234567890123");
        assert_eq!(vendor.entity, None);
        assert_eq!(vendor.merchant, None);
        assert_eq!(vendor.account, None);
        assert_eq!(vendor.login, None);
        assert_eq!(vendor.status, None);
        assert_eq!(vendor.name, None);
        assert_eq!(vendor.legal_name, None);
        assert_eq!(vendor.dba, None);
        assert_eq!(vendor.tax_id, None);
        assert_eq!(vendor.email, None);
        assert_eq!(vendor.phone, None);
        assert_eq!(vendor.address1, None);
        assert_eq!(vendor.address2, None);
        assert_eq!(vendor.city, None);
        assert_eq!(vendor.state, None);
        assert_eq!(vendor.zip, None);
        assert_eq!(vendor.country, None);
        assert_eq!(vendor.description, None);
        assert_eq!(vendor.custom, None);
        assert_eq!(vendor.created, None);
        assert_eq!(vendor.modified, None);
        assert_eq!(vendor.inactive, false);
        assert_eq!(vendor.frozen, false);
    }

    #[test]
    fn test_vendor_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_vnd_12345678901234567890123", "inactive": 1}"#;
        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.inactive, true);

        let json = r#"{"id": "t1_vnd_12345678901234567890123", "inactive": 0}"#;
        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_vnd_12345678901234567890123", "frozen": 1}"#;
        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.frozen, true);

        let json = r#"{"id": "t1_vnd_12345678901234567890123", "frozen": 0}"#;
        let vendor: Vendor = serde_json::from_str(json).unwrap();
        assert_eq!(vendor.frozen, false);
    }

    // NewVendor struct tests
    #[test]
    fn test_new_vendor_serialize_full() {
        let new_vendor = NewVendor {
            entity: "t1_ent_23456789012345678901234".to_string(),
            account: Some("t1_acc_45678901234567890123456".to_string()),
            name: "Test Vendor".to_string(),
            legal_name: Some("Test Vendor Legal Name LLC".to_string()),
            dba: Some("TestVendor".to_string()),
            tax_id: Some("12-3456789".to_string()),
            email: Some("vendor@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            address1: Some("123 Vendor St".to_string()),
            address2: Some("Suite 200".to_string()),
            city: Some("Los Angeles".to_string()),
            state: Some("CA".to_string()),
            zip: Some("90001".to_string()),
            country: Some("US".to_string()),
            description: Some("Test vendor description".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_vendor).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"name\":\"Test Vendor\""));
        assert!(json.contains("\"legalName\":\"Test Vendor Legal Name LLC\""));
        assert!(json.contains("\"account\":\"t1_acc_45678901234567890123456\""));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_vendor_serialize_minimal() {
        let new_vendor = NewVendor {
            entity: "t1_ent_23456789012345678901234".to_string(),
            name: "Test Vendor".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_vendor).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"name\":\"Test Vendor\""));
        assert!(!json.contains("account"));
        assert!(!json.contains("legalName"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_vendor_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_vendor = NewVendor {
            entity: "t1_ent_23456789012345678901234".to_string(),
            name: "Test Vendor".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_vendor).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_vendor = NewVendor {
            entity: "t1_ent_23456789012345678901234".to_string(),
            name: "Test Vendor".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_vendor).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_vendor = NewVendor {
            entity: "t1_ent_23456789012345678901234".to_string(),
            name: "Test Vendor".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_vendor).unwrap();
        assert!(!json.contains("inactive"));
    }
}
