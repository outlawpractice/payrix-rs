//! Customer types for the Payrix API.
//!
//! Customers represent payment contacts associated with a merchant.
//! Each customer can have multiple tokens (saved payment methods).
//!
//! **OpenAPI schema:** `customersResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId, Token};

// =============================================================================
// Customer (Response)
// =============================================================================

/// A Payrix customer.
///
/// Customers are associated with a merchant and can have multiple payment
/// tokens. The `custom` field can store your application's identifier.
///
/// **OpenAPI schema:** `customersResponse`
///
/// See `API_INCONSISTENCIES.md` for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Customer {
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

    /// The ID of the Login that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: `customersModelLogin`)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The ID of the Merchant associated with this Customer.
    ///
    /// **OpenAPI type:** string (ref: `customersModelMerchant`)
    ///
    /// Note: API may return null for some customers.
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The ID of the Entity that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: `customersModelEntity`)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Customer Name & Company
    // -------------------------------------------------------------------------

    /// The first name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub first: Option<String>,

    /// The middle name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub middle: Option<String>,

    /// The last name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last: Option<String>,

    /// The name of the company associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub company: Option<String>,

    // -------------------------------------------------------------------------
    // Contact Information
    // -------------------------------------------------------------------------

    /// The email address of this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// The phone number associated with this Transaction.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The fax number associated with this Customer.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fax: Option<String>,

    // -------------------------------------------------------------------------
    // Billing Address
    // -------------------------------------------------------------------------

    /// The first line of the address associated with this Customer.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// The second line of the address associated with this Customer.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The name of the city in the address associated with this Customer.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The U.S. state or Canadian province relevant to the address provided here.
    ///
    /// If the location is within the U.S. and Canada, specify the 2-character postal
    /// abbreviation for the state. If the location is outside of the U.S. and Canada,
    /// provide the full state name.
    ///
    /// This field is stored as a text string and must be between 2 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The ZIP code in the address associated with this Customer.
    ///
    /// This field is stored as a text string and must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The country associated with this Customer.
    ///
    /// Valid values for this field is the 3-letter ISO code for the country
    /// (e.g., `USA`, `CAN`).
    ///
    /// **OpenAPI type:** string (ref: `Country`)
    #[serde(default)]
    pub country: Option<String>,

    // -------------------------------------------------------------------------
    // Shipping Address
    // -------------------------------------------------------------------------

    /// The first name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_first: Option<String>,

    /// The middle name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_middle: Option<String>,

    /// The last name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_last: Option<String>,

    /// The name of the company associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_company: Option<String>,

    /// The first line of the address associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_address1: Option<String>,

    /// The second line of the address associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_address2: Option<String>,

    /// The name of the city associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_city: Option<String>,

    /// The U.S. state or Canadian province relevant to the shipping address.
    ///
    /// If the location is within the U.S. and Canada, specify the 2-character postal
    /// abbreviation for the state. If the location is outside of the U.S. and Canada,
    /// provide the full state name.
    ///
    /// This field is stored as a text string and must be between 2 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_state: Option<String>,

    /// The ZIP code associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_zip: Option<String>,

    /// The country associated with this Customer's shipping information.
    ///
    /// Valid values for this field is the 3-letter ISO code for the country
    /// (e.g., `USA`, `CAN`).
    ///
    /// **OpenAPI type:** string (ref: `ShippingCountry`)
    #[serde(default)]
    pub shipping_country: Option<String>,

    /// The phone number associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_phone: Option<String>,

    /// The fax number associated with this Customer's shipping information.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shipping_fax: Option<String>,

    // -------------------------------------------------------------------------
    // Custom & Authentication Fields
    // -------------------------------------------------------------------------

    /// Custom, free-form field for client-supplied text.
    ///
    /// Must be between 0 and 1,000 characters long.
    /// Use this to store your application's ID for this contact.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub custom: Option<String>,

    /// The customer reference from the authToken used for user authentication, if available.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub auth_token_customer: Option<String>,

    // -------------------------------------------------------------------------
    // Status Flags
    // -------------------------------------------------------------------------

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    ///
    /// Valid values:
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    ///
    /// Valid values:
    /// - `0` - Active
    /// - `1` - Inactive
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    // -------------------------------------------------------------------------
    // Nested Relations (expanded via `expand` query parameter)
    // -------------------------------------------------------------------------

    /// Array of invoices associated with this Customer.
    ///
    /// Only populated when using the `expand` query parameter.
    ///
    /// **OpenAPI type:** array of `invoicesResponse`
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub invoices: Option<Vec<serde_json::Value>>,

    /// Array of tokens (saved payment methods) associated with this Customer.
    ///
    /// Only populated when using the `expand` query parameter.
    ///
    /// **OpenAPI type:** array of `tokensResponse`
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub tokens: Option<Vec<Token>>,
}

// =============================================================================
// NewCustomer (Request)
// =============================================================================

/// Request to create a new customer.
///
/// **OpenAPI schema:** `customersRequest` (POST /customers)
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    /// The ID of the Merchant associated with this Customer.
    ///
    /// **Required.**
    ///
    /// **OpenAPI type:** string
    pub merchant: String,

    /// The ID of the Login that owns this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,

    /// The first name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// The middle name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// The last name associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// The name of the company associated with this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    /// The email address of this Customer.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The phone number associated with this Customer.
    ///
    /// This field must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The fax number associated with this Customer.
    ///
    /// This field must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

    /// The first line of the address associated with this Customer.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,

    /// The second line of the address associated with this Customer.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    /// The name of the city in the address associated with this Customer.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// The U.S. state or Canadian province relevant to the address.
    ///
    /// Use 2-character postal abbreviation for U.S./Canada, or full state name otherwise.
    /// This field must be between 2 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The ZIP code in the address associated with this Customer.
    ///
    /// This field must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// The country associated with this Customer.
    ///
    /// Use the 3-letter ISO code (e.g., `USA`, `CAN`).
    ///
    /// **OpenAPI type:** string (ref: `Country`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// The first name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_first: Option<String>,

    /// The middle name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_middle: Option<String>,

    /// The last name associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_last: Option<String>,

    /// The name of the company associated with this Customer's shipping information.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_company: Option<String>,

    /// The first line of the address associated with this Customer's shipping information.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address1: Option<String>,

    /// The second line of the address associated with this Customer's shipping information.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address2: Option<String>,

    /// The name of the city associated with this Customer's shipping information.
    ///
    /// This field must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<String>,

    /// The U.S. state or Canadian province relevant to the shipping address.
    ///
    /// Use 2-character postal abbreviation for U.S./Canada, or full state name otherwise.
    /// This field must be between 2 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<String>,

    /// The ZIP code associated with this Customer's shipping information.
    ///
    /// This field must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<String>,

    /// The country associated with this Customer's shipping information.
    ///
    /// Use the 3-letter ISO code (e.g., `USA`, `CAN`).
    ///
    /// **OpenAPI type:** string (ref: `ShippingCountry`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<String>,

    /// The phone number associated with this Customer's shipping information.
    ///
    /// This field must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_phone: Option<String>,

    /// The fax number associated with this Customer's shipping information.
    ///
    /// This field must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_fax: Option<String>,

    /// Custom, free-form field for client-supplied text.
    ///
    /// Must be between 0 and 1,000 characters long.
    /// Use this to store your application's ID for this contact.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (`0` = Active, `1` = Inactive)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (`0` = Not Frozen, `1` = Frozen)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub frozen: Option<bool>,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Customer Struct Tests
    // =========================================================================

    #[test]
    fn customer_deserialize_full() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-04-01 12:00:00.0000",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345",
            "login": "t1_lgn_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "company": "Acme Corp",
            "email": "john.doe@example.com",
            "phone": "555-1234567",
            "fax": "555-9876543",
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
            "shippingPhone": "555-1111111",
            "shippingFax": "555-2222222",
            "custom": "my-app-customer-12345",
            "authTokenCustomer": "auth-customer-ref",
            "frozen": 0,
            "inactive": 1
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();

        // Core identifiers
        assert_eq!(customer.id.as_str(), "t1_cus_12345678901234567890123");
        assert_eq!(customer.created.as_deref(), Some("2024-01-01 00:00:00.0000"));
        assert_eq!(customer.modified.as_deref(), Some("2024-04-01 12:00:00.0000"));
        assert_eq!(customer.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(customer.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
        assert_eq!(customer.login.as_ref().unwrap().as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(customer.merchant.as_ref().unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(customer.entity.as_ref().unwrap().as_str(), "t1_ent_12345678901234567890123");

        // Customer name & company
        assert_eq!(customer.first.as_deref(), Some("John"));
        assert_eq!(customer.middle.as_deref(), Some("Q"));
        assert_eq!(customer.last.as_deref(), Some("Doe"));
        assert_eq!(customer.company.as_deref(), Some("Acme Corp"));

        // Contact information
        assert_eq!(customer.email.as_deref(), Some("john.doe@example.com"));
        assert_eq!(customer.phone.as_deref(), Some("555-1234567"));
        assert_eq!(customer.fax.as_deref(), Some("555-9876543"));

        // Billing address
        assert_eq!(customer.address1.as_deref(), Some("123 Main St"));
        assert_eq!(customer.address2.as_deref(), Some("Suite 100"));
        assert_eq!(customer.city.as_deref(), Some("Springfield"));
        assert_eq!(customer.state.as_deref(), Some("IL"));
        assert_eq!(customer.zip.as_deref(), Some("62701"));
        assert_eq!(customer.country.as_deref(), Some("USA"));

        // Shipping address
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
        assert_eq!(customer.shipping_phone.as_deref(), Some("555-1111111"));
        assert_eq!(customer.shipping_fax.as_deref(), Some("555-2222222"));

        // Custom & auth fields
        assert_eq!(customer.custom.as_deref(), Some("my-app-customer-12345"));
        assert_eq!(customer.auth_token_customer.as_deref(), Some("auth-customer-ref"));

        // Status flags
        assert!(!customer.frozen);
        assert!(customer.inactive);

        // Nested relations (not expanded)
        assert!(customer.invoices.is_none());
        assert!(customer.tokens.is_none());
    }

    #[test]
    fn customer_deserialize_minimal() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123"
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();

        assert_eq!(customer.id.as_str(), "t1_cus_12345678901234567890123");
        assert!(customer.created.is_none());
        assert!(customer.modified.is_none());
        assert!(customer.creator.is_none());
        assert!(customer.modifier.is_none());
        assert!(customer.login.is_none());
        assert!(customer.merchant.is_none());
        assert!(customer.entity.is_none());
        assert!(customer.first.is_none());
        assert!(customer.last.is_none());
        assert!(customer.email.is_none());
        assert!(!customer.frozen);
        assert!(!customer.inactive);
    }

    #[test]
    fn customer_deserialize_with_nested_tokens() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "tokens": [
                {
                    "id": "t1_tok_12345678901234567890123",
                    "customer": "t1_cus_12345678901234567890123",
                    "inactive": 0,
                    "frozen": 0
                }
            ]
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(customer.tokens.is_some());
        let tokens = customer.tokens.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].id.as_str(), "t1_tok_12345678901234567890123");
    }

    #[test]
    fn customer_deserialize_with_nested_invoices() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "invoices": [
                {"id": "t1_inv_12345678901234567890123", "status": "pending"}
            ]
        }"#;

        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(customer.invoices.is_some());
        let invoices = customer.invoices.unwrap();
        assert_eq!(invoices.len(), 1);
    }

    #[test]
    fn customer_bool_from_int_zero_is_false() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
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
            "id": "t1_cus_12345678901234567890123"
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert!(!customer.inactive);
        assert!(!customer.frozen);
    }

    #[test]
    fn customer_creator_modifier_fields() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345"
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert_eq!(customer.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(customer.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
    }

    #[test]
    fn customer_auth_token_customer_field() {
        let json = r#"{
            "id": "t1_cus_12345678901234567890123",
            "authTokenCustomer": "customer-auth-ref-12345"
        }"#;
        let customer: Customer = serde_json::from_str(json).unwrap();
        assert_eq!(customer.auth_token_customer.as_deref(), Some("customer-auth-ref-12345"));
    }

    // =========================================================================
    // NewCustomer Tests
    // =========================================================================

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
            phone: Some("5551234567".to_string()),
            fax: Some("5559876543".to_string()),
            address1: Some("123 Main St".to_string()),
            address2: Some("Suite 100".to_string()),
            city: Some("Springfield".to_string()),
            state: Some("IL".to_string()),
            zip: Some("62701".to_string()),
            country: Some("USA".to_string()),
            shipping_first: Some("Jane".to_string()),
            shipping_middle: Some("R".to_string()),
            shipping_last: Some("Smith".to_string()),
            shipping_company: Some("Shipping Co".to_string()),
            shipping_address1: Some("456 Oak Ave".to_string()),
            shipping_address2: Some("Apt 5".to_string()),
            shipping_city: Some("Chicago".to_string()),
            shipping_state: Some("IL".to_string()),
            shipping_zip: Some("60601".to_string()),
            shipping_country: Some("USA".to_string()),
            shipping_phone: Some("5551111111".to_string()),
            shipping_fax: Some("5552222222".to_string()),
            custom: Some("my-app-customer-id".to_string()),
            inactive: Some(false),
            frozen: Some(true),
        };

        let json = serde_json::to_string(&new_customer).unwrap();

        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"last\":\"Doe\""));
        assert!(json.contains("\"email\":\"john.doe@example.com\""));
        assert!(json.contains("\"shippingFirst\":\"Jane\""));
        assert!(json.contains("\"shippingCity\":\"Chicago\""));
        assert!(json.contains("\"custom\":\"my-app-customer-id\""));
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
        assert!(!json.contains("\"shippingFirst\""));
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

    #[test]
    fn new_customer_with_shipping_fields() {
        let new_customer = NewCustomer {
            merchant: "t1_mer_12345678901234567890123".to_string(),
            shipping_first: Some("Jane".to_string()),
            shipping_last: Some("Doe".to_string()),
            shipping_address1: Some("789 Elm St".to_string()),
            shipping_city: Some("New York".to_string()),
            shipping_state: Some("NY".to_string()),
            shipping_zip: Some("10001".to_string()),
            shipping_country: Some("USA".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_customer).unwrap();
        assert!(json.contains("\"shippingFirst\":\"Jane\""));
        assert!(json.contains("\"shippingLast\":\"Doe\""));
        assert!(json.contains("\"shippingAddress1\":\"789 Elm St\""));
        assert!(json.contains("\"shippingCity\":\"New York\""));
        assert!(json.contains("\"shippingState\":\"NY\""));
        assert!(json.contains("\"shippingZip\":\"10001\""));
        assert!(json.contains("\"shippingCountry\":\"USA\""));
    }
}
