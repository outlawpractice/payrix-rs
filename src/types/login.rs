//! Login types for the Payrix API.
//!
//! Logins represent user accounts that can access the Payrix system.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Login role values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LoginRole {
    /// Administrator with full access
    Admin = 1,
    /// Manager with limited admin access
    Manager = 2,
    /// Standard user
    #[default]
    User = 3,
    /// Read-only access
    ReadOnly = 4,
    /// API-only access
    Api = 5,
}

/// Login status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LoginStatus {
    /// Active login
    #[default]
    Active = 1,
    /// Locked out (too many failed attempts)
    Locked = 2,
    /// Disabled by administrator
    Disabled = 3,
    /// Pending activation
    Pending = 4,
}

/// A Payrix login/user account.
///
/// Logins represent users who can access the Payrix system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Login {
    /// Unique identifier (30 characters, e.g., "t1_lgn_...")
    pub id: PayrixId,

    /// Entity ID this login belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Username/login name
    #[serde(default)]
    pub username: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// First name
    #[serde(default)]
    pub first: Option<String>,

    /// Middle name
    #[serde(default)]
    pub middle: Option<String>,

    /// Last name
    #[serde(default)]
    pub last: Option<String>,

    /// Phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// User role
    #[serde(default)]
    pub role: Option<LoginRole>,

    /// Login status
    #[serde(default)]
    pub status: Option<LoginStatus>,

    /// Whether two-factor authentication is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub tfa_enabled: bool,

    /// Last login timestamp
    #[serde(default)]
    pub last_login: Option<String>,

    /// Number of failed login attempts
    #[serde(default)]
    pub failed_attempts: Option<i32>,

    /// Timestamp when login was locked
    #[serde(default)]
    pub locked_at: Option<String>,

    /// Password reset token (if pending)
    #[serde(default)]
    pub reset_token: Option<String>,

    /// Password reset token expiry
    #[serde(default)]
    pub reset_expires: Option<String>,

    /// API key (for API-only logins)
    #[serde(default)]
    pub api_key: Option<String>,

    /// Timezone preference
    #[serde(default)]
    pub timezone: Option<String>,

    /// Locale/language preference
    #[serde(default)]
    pub locale: Option<String>,

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

/// Request to create a new login.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLogin {
    /// Entity ID (required)
    pub entity: String,

    /// Username/login name (required)
    pub username: String,

    /// Email address (required)
    pub email: String,

    /// Password (required for non-API logins)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,

    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// User role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<LoginRole>,

    /// Whether two-factor authentication is enabled
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub tfa_enabled: Option<bool>,

    /// Timezone preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// Locale/language preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LoginRole Tests ====================

    #[test]
    fn login_role_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&LoginRole::Admin).unwrap(), "1");
        assert_eq!(serde_json::to_string(&LoginRole::Manager).unwrap(), "2");
        assert_eq!(serde_json::to_string(&LoginRole::User).unwrap(), "3");
        assert_eq!(serde_json::to_string(&LoginRole::ReadOnly).unwrap(), "4");
        assert_eq!(serde_json::to_string(&LoginRole::Api).unwrap(), "5");
    }

    #[test]
    fn login_role_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<LoginRole>("1").unwrap(), LoginRole::Admin);
        assert_eq!(serde_json::from_str::<LoginRole>("2").unwrap(), LoginRole::Manager);
        assert_eq!(serde_json::from_str::<LoginRole>("3").unwrap(), LoginRole::User);
        assert_eq!(serde_json::from_str::<LoginRole>("4").unwrap(), LoginRole::ReadOnly);
        assert_eq!(serde_json::from_str::<LoginRole>("5").unwrap(), LoginRole::Api);
    }

    #[test]
    fn login_role_default() {
        assert_eq!(LoginRole::default(), LoginRole::User);
    }

    #[test]
    fn login_role_invalid_value() {
        assert!(serde_json::from_str::<LoginRole>("0").is_err());
        assert!(serde_json::from_str::<LoginRole>("99").is_err());
    }

    // ==================== LoginStatus Tests ====================

    #[test]
    fn login_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&LoginStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&LoginStatus::Locked).unwrap(), "2");
        assert_eq!(serde_json::to_string(&LoginStatus::Disabled).unwrap(), "3");
        assert_eq!(serde_json::to_string(&LoginStatus::Pending).unwrap(), "4");
    }

    #[test]
    fn login_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<LoginStatus>("1").unwrap(), LoginStatus::Active);
        assert_eq!(serde_json::from_str::<LoginStatus>("2").unwrap(), LoginStatus::Locked);
        assert_eq!(serde_json::from_str::<LoginStatus>("3").unwrap(), LoginStatus::Disabled);
        assert_eq!(serde_json::from_str::<LoginStatus>("4").unwrap(), LoginStatus::Pending);
    }

    #[test]
    fn login_status_default() {
        assert_eq!(LoginStatus::default(), LoginStatus::Active);
    }

    #[test]
    fn login_status_invalid_value() {
        assert!(serde_json::from_str::<LoginStatus>("0").is_err());
        assert!(serde_json::from_str::<LoginStatus>("99").is_err());
    }

    // ==================== Login Struct Tests ====================

    #[test]
    fn login_deserialize_full() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "username": "johndoe",
            "email": "john.doe@example.com",
            "first": "John",
            "middle": "Q",
            "last": "Doe",
            "phone": "555-1234",
            "role": 1,
            "status": 1,
            "tfaEnabled": 1,
            "lastLogin": "2024-04-01 10:30:00.000",
            "failedAttempts": 0,
            "lockedAt": "2024-01-15 08:00:00.000",
            "resetToken": "reset_token_abc123",
            "resetExpires": "2024-04-02 10:30:00.000",
            "apiKey": "api_key_xyz789",
            "timezone": "America/New_York",
            "locale": "en_US",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let login: Login = serde_json::from_str(json).unwrap();
        assert_eq!(login.id.as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(login.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(login.username.as_deref(), Some("johndoe"));
        assert_eq!(login.email.as_deref(), Some("john.doe@example.com"));
        assert_eq!(login.first.as_deref(), Some("John"));
        assert_eq!(login.middle.as_deref(), Some("Q"));
        assert_eq!(login.last.as_deref(), Some("Doe"));
        assert_eq!(login.phone.as_deref(), Some("555-1234"));
        assert_eq!(login.role, Some(LoginRole::Admin));
        assert_eq!(login.status, Some(LoginStatus::Active));
        assert!(login.tfa_enabled);
        assert_eq!(login.last_login.as_deref(), Some("2024-04-01 10:30:00.000"));
        assert_eq!(login.failed_attempts, Some(0));
        assert_eq!(login.locked_at.as_deref(), Some("2024-01-15 08:00:00.000"));
        assert_eq!(login.reset_token.as_deref(), Some("reset_token_abc123"));
        assert_eq!(login.reset_expires.as_deref(), Some("2024-04-02 10:30:00.000"));
        assert_eq!(login.api_key.as_deref(), Some("api_key_xyz789"));
        assert_eq!(login.timezone.as_deref(), Some("America/New_York"));
        assert_eq!(login.locale.as_deref(), Some("en_US"));
        assert_eq!(login.custom.as_deref(), Some("custom data"));
        assert_eq!(login.created.as_deref(), Some("2024-01-01 00:00:00.000"));
        assert_eq!(login.modified.as_deref(), Some("2024-04-01 12:00:00.000"));
        assert!(!login.inactive);
        assert!(login.frozen);
    }

    #[test]
    fn login_deserialize_minimal() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123"
        }"#;

        let login: Login = serde_json::from_str(json).unwrap();
        assert_eq!(login.id.as_str(), "t1_lgn_12345678901234567890123");
        assert!(login.entity.is_none());
        assert!(login.username.is_none());
        assert!(login.role.is_none());
        assert!(login.status.is_none());
        assert!(!login.tfa_enabled);
        assert!(!login.inactive);
        assert!(!login.frozen);
    }

    #[test]
    fn login_bool_from_int_zero_is_false() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "tfaEnabled": 0,
            "inactive": 0,
            "frozen": 0
        }"#;
        let login: Login = serde_json::from_str(json).unwrap();
        assert!(!login.tfa_enabled);
        assert!(!login.inactive);
        assert!(!login.frozen);
    }

    #[test]
    fn login_bool_from_int_one_is_true() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123",
            "tfaEnabled": 1,
            "inactive": 1,
            "frozen": 1
        }"#;
        let login: Login = serde_json::from_str(json).unwrap();
        assert!(login.tfa_enabled);
        assert!(login.inactive);
        assert!(login.frozen);
    }

    #[test]
    fn login_bool_from_int_missing_defaults_false() {
        let json = r#"{
            "id": "t1_lgn_12345678901234567890123"
        }"#;
        let login: Login = serde_json::from_str(json).unwrap();
        assert!(!login.tfa_enabled);
        assert!(!login.inactive);
        assert!(!login.frozen);
    }

    // ==================== NewLogin Tests ====================

    #[test]
    fn new_login_serialize_full() {
        let new_login = NewLogin {
            entity: "t1_ent_12345678901234567890123".to_string(),
            username: "johndoe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: Some("secure_password_123".to_string()),
            first: Some("John".to_string()),
            middle: Some("Q".to_string()),
            last: Some("Doe".to_string()),
            phone: Some("555-1234".to_string()),
            role: Some(LoginRole::Manager),
            tfa_enabled: Some(true),
            timezone: Some("America/New_York".to_string()),
            locale: Some("en_US".to_string()),
            custom: Some("custom data".to_string()),
        };

        let json = serde_json::to_string(&new_login).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"username\":\"johndoe\""));
        assert!(json.contains("\"email\":\"john.doe@example.com\""));
        assert!(json.contains("\"password\":\"secure_password_123\""));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"role\":2"));
        assert!(json.contains("\"tfaEnabled\":1"));
    }

    #[test]
    fn new_login_serialize_minimal() {
        let new_login = NewLogin {
            entity: "t1_ent_12345678901234567890123".to_string(),
            username: "johndoe".to_string(),
            email: "john.doe@example.com".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_login).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"username\":\"johndoe\""));
        assert!(json.contains("\"email\":\"john.doe@example.com\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"password\""));
        assert!(!json.contains("\"first\""));
        assert!(!json.contains("\"role\""));
        assert!(!json.contains("\"tfaEnabled\""));
    }

    #[test]
    fn new_login_option_bool_to_int_true() {
        let new_login = NewLogin {
            entity: "t1_ent_12345678901234567890123".to_string(),
            username: "johndoe".to_string(),
            email: "john.doe@example.com".to_string(),
            tfa_enabled: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_login).unwrap();
        assert!(json.contains("\"tfaEnabled\":1"));
    }

    #[test]
    fn new_login_option_bool_to_int_false() {
        let new_login = NewLogin {
            entity: "t1_ent_12345678901234567890123".to_string(),
            username: "johndoe".to_string(),
            email: "john.doe@example.com".to_string(),
            tfa_enabled: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_login).unwrap();
        assert!(json.contains("\"tfaEnabled\":0"));
    }
}
