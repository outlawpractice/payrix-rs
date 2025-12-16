//! Customer types for the Payrix API.
//!
//! Customers represent payment contacts associated with a merchant.
//! Each customer can have multiple tokens (saved payment methods).

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// A Payrix customer.
///
/// Customers are associated with a merchant and can have multiple payment
/// tokens. The `custom` field can store your application's identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// Unique identifier (30 characters, e.g., "t1_cus_...")
    pub id: PayrixId,

    /// The ID of the Merchant (not the Merchant's entity)
    pub merchant: PayrixId,

    /// Login ID that created this entity
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Customer's first name
    #[serde(default)]
    pub first: Option<String>,

    /// Customer's middle name
    #[serde(default)]
    pub middle: Option<String>,

    /// Customer's last name
    #[serde(default)]
    pub last: Option<String>,

    /// Company name
    #[serde(default)]
    pub company: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// Fax number
    #[serde(default)]
    pub fax: Option<String>,

    /// Billing address line 1
    #[serde(default)]
    pub address1: Option<String>,

    /// Billing address line 2
    #[serde(default)]
    pub address2: Option<String>,

    /// Billing city
    #[serde(default)]
    pub city: Option<String>,

    /// Two-letter state/province code
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(default)]
    pub zip: Option<String>,

    /// Country code (USA or CAN)
    #[serde(default)]
    pub country: Option<String>,

    // Shipping address fields
    /// Shipping first name
    #[serde(default)]
    pub shipping_first: Option<String>,

    /// Shipping middle name
    #[serde(default)]
    pub shipping_middle: Option<String>,

    /// Shipping last name
    #[serde(default)]
    pub shipping_last: Option<String>,

    /// Shipping company
    #[serde(default)]
    pub shipping_company: Option<String>,

    /// Shipping address line 1
    #[serde(default)]
    pub shipping_address1: Option<String>,

    /// Shipping address line 2
    #[serde(default)]
    pub shipping_address2: Option<String>,

    /// Shipping city
    #[serde(default)]
    pub shipping_city: Option<String>,

    /// Shipping state/province code
    #[serde(default)]
    pub shipping_state: Option<String>,

    /// Shipping ZIP/postal code
    #[serde(default)]
    pub shipping_zip: Option<String>,

    /// Shipping country code
    #[serde(default)]
    pub shipping_country: Option<String>,

    /// Shipping phone
    #[serde(default)]
    pub shipping_phone: Option<String>,

    /// Shipping fax
    #[serde(default)]
    pub shipping_fax: Option<String>,

    /// Custom field for client data (0-1000 chars).
    /// Use this to store your application's ID for this contact.
    #[serde(default)]
    pub custom: Option<String>,

    /// Entity ID that owns this customer
    #[serde(default)]
    pub entity: Option<PayrixId>,

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

/// Request to create a new customer.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    /// Merchant ID (required)
    pub merchant: String,

    /// Login ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,

    /// Customer's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Customer's middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// Customer's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Billing address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,

    /// Billing address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    /// Billing city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter state/province code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// Country code (USA or CAN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Custom field for client data (0-1000 chars).
    /// Use this to store your application's ID for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether resource is inactive (false=active, true=inactive)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,

    /// Whether resource is frozen
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub frozen: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Customer Struct Tests ====================

    #[test]
    fn customer_deserialize_full() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "login": "t1_lgn_12345678901234567890123",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "company": "Acme Corp",
            "email": "john.doe@example.com",
            "phone": "555-1234",
            "fax": "555-5678",
            "address1": "123 Main St",
            "address2": "Suite 100",
            "city": "Springfield",
            "state": "IL",
            "zip": "62701",
            "country": "USA",
            "shippingFirst": "Jane",
            "shippingMiddle": "R",
            "shippingLast": "Smith",
            "shippingCompany": "Shipping Co",
            "shippingAddress1": "456 Oak Ave",
            "shippingAddress2": "Apt 5",
            "shippingCity": "Chicago",
            "shippingState": "IL",
            "shippingZip": "60601",
            "shippingCountry": "USA",
            "shippingPhone": "555-9999",
            "shippingFax": "555-8888",
            "custom": "custom data",
            "entity": "t1_ent_12345678901234567890123",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();
        assert_eq!(customer.id.as_str(), "t1_cus_12345678901234567890123");
        assert_eq!(customer.merchant.as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(customer.login.unwrap().as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(customer.first.as_deref(), Some("John"));
        assert_eq!(customer.middle.as_deref(), Some("Q"));
        assert_eq!(customer.last.as_deref(), Some("Doe"));
        assert_eq!(customer.company.as_deref(), Some("Acme Corp"));
        assert_eq!(customer.email.as_deref(), Some("john.doe@example.com"));
        assert_eq!(customer.phone.as_deref(), Some("555-1234"));
        assert_eq!(customer.fax.as_deref(), Some("555-5678"));
        assert_eq!(customer.address1.as_deref(), Some("123 Main St"));
        assert_eq!(customer.address2.as_deref(), Some("Suite 100"));
        assert_eq!(customer.city.as_deref(), Some("Springfield"));
        assert_eq!(customer.state.as_deref(), Some("IL"));
        assert_eq!(customer.zip.as_deref(), Some("62701"));
        assert_eq!(customer.country.as_deref(), Some("USA"));
        assert_eq!(customer.shipping_first.as_deref(), Some("Jane"));
        assert_eq!(customer.shipping_middle.as_deref(), Some("R"));
        assert_eq!(customer.shipping_last.as_deref(), Some("Smith"));
        assert_eq!(customer.shipping_company.as_deref(), Some("Shipping Co"));
        assert_eq!(customer.shipping_address1.as_deref(), Some("456 Oak Ave"));
        assert_eq!(customer.shipping_address2.as_deref(), Some("Apt 5"));
        assert_eq!(customer.shipping_city.as_deref(), Some("Chicago"));
        assert_eq!(customer.shipping_state.as_deref(), Some("IL"));
        assert_eq!(customer.shipping_zip.as_deref(), Some("60601"));
        assert_eq!(customer.shipping_country.as_deref(), Some("USA"));
        assert_eq!(customer.shipping_phone.as_deref(), Some("555-9999"));
        assert_eq!(customer.shipping_fax.as_deref(), Some("555-8888"));
        assert_eq!(customer.custom.as_deref(), Some("custom data"));
        assert_eq!(customer.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(customer.created.as_deref(), Some("2024-01-01 00:00:00.000"));
        assert_eq!(customer.modified.as_deref(), Some("2024-04-01 12:00:00.000"));
        assert!(!customer.inactive);
        assert!(customer.frozen);
    }

    #[test]
    fn customer_deserialize_minimal() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123"
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();
        assert_eq!(customer.id.as_str(), "t1_cus_12345678901234567890123");
        assert_eq!(customer.merchant.as_str(), "t1_mer_12345678901234567890123");
        assert!(customer.login.is_none());
        assert!(customer.first.is_none());
        assert!(customer.email.is_none());
        assert!(!customer.inactive);
        assert!(!customer.frozen);
    }

    #[test]
    fn customer_bool_from_int_zero_is_false() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "inactive": 0,
            "frozen": 0
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(!customer.inactive);
        assert!(!customer.frozen);
    }

    #[test]
    fn customer_bool_from_int_one_is_true() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "inactive": 1,
            "frozen": 1
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(customer.inactive);
        assert!(customer.frozen);
    }

    #[test]
    fn customer_bool_from_int_missing_defaults_false() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123"
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(!customer.inactive);
        assert!(!customer.frozen);
    }

    // ==================== NewCustomer Tests ====================

    #[test]
    fn new_customer_serialize_full() {
        let new_customer = NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            login: Some("t1_lgn_12345678901234567890123".to_string()),
            first: Some("John".to_string()),
            middle: Some("Q".to_string()),
            last: Some("Doe".to_string()),
            company: Some("Acme Corp".to_string()),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            address1: Some("123 Main St".to_string()),
            address2: Some("Suite 100".to_string()),
            city: Some("Springfield".to_string()),
            state: Some("IL".to_string()),
            zip: Some("62701".to_string()),
            country: Some("USA".to_string()),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
            frozen: Some(true),
        };

        let json = serde_json::to_string(&new_customer).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"last\":\"Doe\""));
        assert!(json.contains("\"email\":\"john.doe@example.com\""));
        assert!(json.contains("\"inactive\":0"));
        assert!(json.contains("\"frozen\":1"));
    }

    #[test]
    fn new_customer_serialize_minimal() {
        let new_customer = NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_customer).unwrap();
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"first\""));
        assert!(!json.contains("\"email\""));
        assert!(!json.contains("\"inactive\""));
        assert!(!json.contains("\"frozen\""));
    }

    #[test]
    fn new_customer_option_bool_to_int_true() {
        let new_customer = NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            inactive: Some(true),
            frozen: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_customer).unwrap();
        assert!(json.contains("\"inactive\":1"));
        assert!(json.contains("\"frozen\":1"));
    }

    #[test]
    fn new_customer_option_bool_to_int_false() {
        let new_customer = NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            inactive: Some(false),
            frozen: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_customer).unwrap();
        assert!(json.contains("\"inactive\":0"));
        assert!(json.contains("\"frozen\":0"));
    }
}
