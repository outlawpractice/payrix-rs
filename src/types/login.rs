//! Login types for the Payrix API.
//!
//! Logins represent user accounts that can access the Payrix system.
//!
//! **OpenAPI schema:** `loginsResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// LOGIN STRUCT
// =============================================================================

/// A Payrix login/user account.
///
/// Logins represent users who can access the Payrix system.
///
/// **OpenAPI schema:** `loginsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Login {
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

    /// The identifier of the parent Login.
    ///
    /// **OpenAPI type:** string (ref: loginsModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The timestamp when this Login last logged in to the API.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last_login: Option<String>,

    /// The username associated with this Login.
    ///
    /// This field is stored as a text string, all lowercase, and must be
    /// between 0 and 50 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub username: Option<String>,

    /// The password associated with this Login.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub password: Option<String>,

    /// The SSO (Single Sign On) ID setup for the login.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub sso_id: Option<String>,

    /// The first name associated with this Login.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub first: Option<String>,

    /// The middle name associated with this Login.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub middle: Option<String>,

    /// The last name associated with this Login.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last: Option<String>,

    /// The email address associated with this Login.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// The fax number associated with this Login.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fax: Option<String>,

    /// The phone number associated with this Login.
    ///
    /// This field is stored as a text string and must be between 10 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The country associated with this Login.
    ///
    /// Valid values is the 3-letter ISO code for the country.
    ///
    /// **OpenAPI type:** string (ref: Country)
    #[serde(default)]
    pub country: Option<String>,

    /// The ZIP code in the address associated with this Login.
    ///
    /// This field is stored as a text string and must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The state in the address associated with this Login.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The name of the city in the address associated with this Login.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The second line of the address associated with this Login.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The first line of the address associated with this Login.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// Whether the email associated with this Login was confirmed.
    ///
    /// - `0` - Not confirmed
    /// - `1` - Confirmed
    ///
    /// **OpenAPI type:** integer (ref: Confirmed)
    #[serde(default, with = "bool_from_int_default_false")]
    pub confirmed: bool,

    /// The roles associated with this Login, specified as an integer.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub roles: Option<i64>,

    /// The partition that this Login is associated with.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub partition: Option<String>,

    /// The division that this Login belongs to.
    ///
    /// **OpenAPI type:** string (ref: loginsModelDivision)
    #[serde(default)]
    pub division: Option<PayrixId>,

    /// The parent division that this Login belongs to.
    ///
    /// Children of this Login will inherit its parent division.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub parent_division: Option<String>,

    /// The resources a user can access (JSON structure).
    ///
    /// Includes action keys (create, update, read, delete, totals) and lists of resources.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub allowed_resources: Option<String>,

    /// The resources a user is restricted from accessing (JSON structure).
    ///
    /// Includes action keys (create, update, read, delete, totals) and lists of resources.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub restricted_resources: Option<String>,

    /// Whether or not this user should have access to portal functionality.
    ///
    /// - `0` - No access to the portal
    /// - `1` - Has access to the portal
    ///
    /// **OpenAPI type:** integer (ref: PortalAccess)
    #[serde(default, with = "bool_from_int_default_false")]
    pub portal_access: bool,

    /// Whether or not this user should have multi factor authentication (MFA) feature.
    ///
    /// - `0` - Disabled
    /// - `1` - Enabled
    ///
    /// **OpenAPI type:** integer (ref: loginsMfaEnabled)
    #[serde(default, with = "bool_from_int_default_false")]
    pub mfa_enabled: bool,

    /// The MFA secret key used to link the MFA device with this Login.
    ///
    /// This field is stored as a text string and must be between 1 and 128 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mfa_secret: Option<String>,

    /// The datetimestamp when this Login was enabled for MFA.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mfa_enrolled_date: Option<String>,

    /// The Type of the MFA enrolled.
    ///
    /// This field is stored as a text string and must be between 1 and 50 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mfa_type: Option<String>,

    /// The full effective roles of the user (as an integer).
    ///
    /// Broken down by role, for roles that include other roles automatically.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub effective_roles: Option<i64>,

    /// The number of SMS codes generated during the current MFA SMS window.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub mfa_sms_codes_count: Option<i32>,

    /// The most recent SMS Code which has been texted to the user.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub mfa_sms_window: Option<i32>,

    /// Whether the associated login can use loginAs or not.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub login_as_enabled: Option<i32>,

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

    // Nested relations (only when sqlx feature is disabled)

    /// Aggregations associated with this login.
    ///
    /// **OpenAPI type:** array of aggregationsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub aggregations: Option<Vec<serde_json::Value>>,

    /// Customers associated with this login.
    ///
    /// **OpenAPI type:** array of customersResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub customers: Option<Vec<serde_json::Value>>,

    /// Divisions associated with this login.
    ///
    /// **OpenAPI type:** array of divisionsResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub divisions: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Login Struct Tests ====================

    #[test]
    fn login_deserialize_full() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890124",
            "modifier": "t1_lgn_12345678901234567890125",
            "login": "t1_lgn_12345678901234567890126",
            "lastLogin": "2024-04-01 10:30:00",
            "username": "johndoe",
            "password": "hashedpassword",
            "ssoId": "sso123",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "email": "john.doe@example.com",
            "fax": "5551234567",
            "phone": "5559876543",
            "country": "USA",
            "zip": "60601",
            "state": "IL",
            "city": "Chicago",
            "address2": "Suite 100",
            "address1": "123 Main St",
            "confirmed": 1,
            "roles": 15,
            "partition": "partition1",
            "division": "t1_div_12345678901234567890123",
            "parentDivision": "parent_div_123",
            "allowedResources": "{\"read\":[\"txns\"]}",
            "restrictedResources": "{\"delete\":[\"logins\"]}",
            "portalAccess": 1,
            "mfaEnabled": 1,
            "mfaSecret": "secret123",
            "mfaEnrolledDate": "2024-01-15 08:00:00",
            "mfaType": "totp",
            "effectiveRoles": 31,
            "mfaSmsCodesCount": 2,
            "mfaSmsWindow": 300,
            "loginAsEnabled": 1,
            "inactive": 0,
            "frozen": 1
        }"#;

        let login: Login = serde_json::from_str(json).unwrap();
        assert_eq!(login.id.as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(login.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(login.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            login.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            login.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            login.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890126")
        );
        assert_eq!(login.last_login, Some("2024-04-01 10:30:00".to_string()));
        assert_eq!(login.username, Some("johndoe".to_string()));
        assert_eq!(login.password, Some("hashedpassword".to_string()));
        assert_eq!(login.sso_id, Some("sso123".to_string()));
        assert_eq!(login.first, Some("John".to_string()));
        assert_eq!(login.middle, Some("Q".to_string()));
        assert_eq!(login.last, Some("Doe".to_string()));
        assert_eq!(login.email, Some("john.doe@example.com".to_string()));
        assert_eq!(login.fax, Some("5551234567".to_string()));
        assert_eq!(login.phone, Some("5559876543".to_string()));
        assert_eq!(login.country, Some("USA".to_string()));
        assert_eq!(login.zip, Some("60601".to_string()));
        assert_eq!(login.state, Some("IL".to_string()));
        assert_eq!(login.city, Some("Chicago".to_string()));
        assert_eq!(login.address2, Some("Suite 100".to_string()));
        assert_eq!(login.address1, Some("123 Main St".to_string()));
        assert!(login.confirmed);
        assert_eq!(login.roles, Some(15));
        assert_eq!(login.partition, Some("partition1".to_string()));
        assert_eq!(
            login.division.as_ref().map(|d| d.as_str()),
            Some("t1_div_12345678901234567890123")
        );
        assert_eq!(login.parent_division, Some("parent_div_123".to_string()));
        assert!(login.portal_access);
        assert!(login.mfa_enabled);
        assert_eq!(login.mfa_secret, Some("secret123".to_string()));
        assert_eq!(login.mfa_type, Some("totp".to_string()));
        assert_eq!(login.effective_roles, Some(31));
        assert_eq!(login.mfa_sms_codes_count, Some(2));
        assert_eq!(login.mfa_sms_window, Some(300));
        assert_eq!(login.login_as_enabled, Some(1));
        assert!(!login.inactive);
        assert!(login.frozen);
    }

    #[test]
    fn login_deserialize_minimal() {
        let json = r#"{"id": "t1_lgn_12345678901234567890123"}"#;

        let login: Login = serde_json::from_str(json).unwrap();
        assert_eq!(login.id.as_str(), "t1_lgn_12345678901234567890123");
        assert!(login.created.is_none());
        assert!(login.modified.is_none());
        assert!(login.creator.is_none());
        assert!(login.modifier.is_none());
        assert!(login.login.is_none());
        assert!(login.last_login.is_none());
        assert!(login.username.is_none());
        assert!(login.password.is_none());
        assert!(login.sso_id.is_none());
        assert!(login.first.is_none());
        assert!(login.middle.is_none());
        assert!(login.last.is_none());
        assert!(login.email.is_none());
        assert!(login.fax.is_none());
        assert!(login.phone.is_none());
        assert!(login.country.is_none());
        assert!(login.zip.is_none());
        assert!(login.state.is_none());
        assert!(login.city.is_none());
        assert!(login.address2.is_none());
        assert!(login.address1.is_none());
        assert!(!login.confirmed);
        assert!(login.roles.is_none());
        assert!(login.partition.is_none());
        assert!(login.division.is_none());
        assert!(login.parent_division.is_none());
        assert!(login.allowed_resources.is_none());
        assert!(login.restricted_resources.is_none());
        assert!(!login.portal_access);
        assert!(!login.mfa_enabled);
        assert!(login.mfa_secret.is_none());
        assert!(login.mfa_enrolled_date.is_none());
        assert!(login.mfa_type.is_none());
        assert!(login.effective_roles.is_none());
        assert!(login.mfa_sms_codes_count.is_none());
        assert!(login.mfa_sms_window.is_none());
        assert!(login.login_as_enabled.is_none());
        assert!(!login.inactive);
        assert!(!login.frozen);
    }

    #[test]
    fn login_bool_from_int() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "confirmed": 1,
            "portalAccess": 0,
            "mfaEnabled": 1,
            "inactive": 1,
            "frozen": 0
        }"#;
        let login: Login = serde_json::from_str(json).unwrap();
        assert!(login.confirmed);
        assert!(!login.portal_access);
        assert!(login.mfa_enabled);
        assert!(login.inactive);
        assert!(!login.frozen);
    }

    #[test]
    fn login_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "username": "johndoe",
            "email": "john@example.com",
            "roles": 15
        }"#;

        let login: Login = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&login).unwrap();
        let deserialized: Login = serde_json::from_str(&serialized).unwrap();
        assert_eq!(login.id, deserialized.id);
        assert_eq!(login.username, deserialized.username);
        assert_eq!(login.email, deserialized.email);
        assert_eq!(login.roles, deserialized.roles);
    }
}
