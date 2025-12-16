//! Team login types for the Payrix API.
//!
//! Team logins represent user accounts with specific roles and permissions.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// Team login role values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TeamLoginRole {
    /// Admin role - full access
    Admin = 1,
    /// Manager role - limited admin access
    Manager = 2,
    /// User role - basic access
    #[default]
    User = 3,
    /// Read-only role
    ReadOnly = 4,
}

/// Team login status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TeamLoginStatus {
    /// Login is inactive
    Inactive = 0,
    /// Login is active
    #[default]
    Active = 1,
    /// Login is locked
    Locked = 2,
    /// Login is suspended
    Suspended = 3,
}

/// A Payrix team login.
///
/// Team logins are user accounts associated with an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct TeamLogin {
    /// Unique identifier (30 characters, e.g., "t1_tlg_...")
    pub id: PayrixId,

    /// Entity ID this login belongs to
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Parent login ID (if sub-user)
    #[serde(default)]
    pub parent: Option<PayrixId>,

    /// Login status
    #[serde(default)]
    pub status: Option<TeamLoginStatus>,

    /// Login role
    #[serde(default)]
    pub role: Option<TeamLoginRole>,

    /// Username/email
    #[serde(default)]
    pub username: Option<String>,

    /// First name
    #[serde(default)]
    pub first: Option<String>,

    /// Last name
    #[serde(default)]
    pub last: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// Last login timestamp
    #[serde(default)]
    pub last_login: Option<String>,

    /// Two-factor authentication enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub two_factor: bool,

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

/// Request to create a new team login.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTeamLogin {
    /// Entity ID (required)
    pub entity: String,

    /// Login role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<TeamLoginRole>,

    /// Username/email (required)
    pub username: String,

    /// Password (required for new logins)
    pub password: String,

    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,

    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

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

    // TeamLoginRole enum tests
    #[test]
    fn test_team_login_role_default() {
        assert_eq!(TeamLoginRole::default(), TeamLoginRole::User);
    }

    #[test]
    fn test_team_login_role_serialize() {
        assert_eq!(serde_json::to_string(&TeamLoginRole::Admin).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TeamLoginRole::Manager).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TeamLoginRole::User).unwrap(), "3");
        assert_eq!(serde_json::to_string(&TeamLoginRole::ReadOnly).unwrap(), "4");
    }

    #[test]
    fn test_team_login_role_deserialize() {
        assert_eq!(serde_json::from_str::<TeamLoginRole>("1").unwrap(), TeamLoginRole::Admin);
        assert_eq!(serde_json::from_str::<TeamLoginRole>("2").unwrap(), TeamLoginRole::Manager);
        assert_eq!(serde_json::from_str::<TeamLoginRole>("3").unwrap(), TeamLoginRole::User);
        assert_eq!(serde_json::from_str::<TeamLoginRole>("4").unwrap(), TeamLoginRole::ReadOnly);
    }

    // TeamLoginStatus enum tests
    #[test]
    fn test_team_login_status_default() {
        assert_eq!(TeamLoginStatus::default(), TeamLoginStatus::Active);
    }

    #[test]
    fn test_team_login_status_serialize() {
        assert_eq!(serde_json::to_string(&TeamLoginStatus::Inactive).unwrap(), "0");
        assert_eq!(serde_json::to_string(&TeamLoginStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TeamLoginStatus::Locked).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TeamLoginStatus::Suspended).unwrap(), "3");
    }

    #[test]
    fn test_team_login_status_deserialize() {
        assert_eq!(serde_json::from_str::<TeamLoginStatus>("0").unwrap(), TeamLoginStatus::Inactive);
        assert_eq!(serde_json::from_str::<TeamLoginStatus>("1").unwrap(), TeamLoginStatus::Active);
        assert_eq!(serde_json::from_str::<TeamLoginStatus>("2").unwrap(), TeamLoginStatus::Locked);
        assert_eq!(serde_json::from_str::<TeamLoginStatus>("3").unwrap(), TeamLoginStatus::Suspended);
    }

    // TeamLogin struct tests
    #[test]
    fn test_team_login_deserialize_full() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "entity": "t1_ent_23456789012345678901234",
            "parent": "t1_tlg_34567890123456789012345",
            "status": 1,
            "role": 2,
            "username": "john.doe",
            "first": "John",
            "last": "Doe",
            "email": "john@example.com",
            "phone": "555-1234",
            "lastLogin": "2024-01-02 08:30:00.000",
            "twoFactor": 1,
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.id.as_str(), "t1_tlg_12345678901234567890123");
        assert_eq!(team_login.entity, Some(PayrixId::new("t1_ent_23456789012345678901234").unwrap()));
        assert_eq!(team_login.parent, Some(PayrixId::new("t1_tlg_34567890123456789012345").unwrap()));
        assert_eq!(team_login.status, Some(TeamLoginStatus::Active));
        assert_eq!(team_login.role, Some(TeamLoginRole::Manager));
        assert_eq!(team_login.username, Some("john.doe".to_string()));
        assert_eq!(team_login.first, Some("John".to_string()));
        assert_eq!(team_login.last, Some("Doe".to_string()));
        assert_eq!(team_login.email, Some("john@example.com".to_string()));
        assert_eq!(team_login.phone, Some("555-1234".to_string()));
        assert_eq!(team_login.last_login, Some("2024-01-02 08:30:00.000".to_string()));
        assert_eq!(team_login.two_factor, true);
        assert_eq!(team_login.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(team_login.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(team_login.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(team_login.inactive, false);
        assert_eq!(team_login.frozen, true);
    }

    #[test]
    fn test_team_login_deserialize_minimal() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123"
        }"#;

        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.id.as_str(), "t1_tlg_12345678901234567890123");
        assert_eq!(team_login.entity, None);
        assert_eq!(team_login.parent, None);
        assert_eq!(team_login.status, None);
        assert_eq!(team_login.role, None);
        assert_eq!(team_login.username, None);
        assert_eq!(team_login.first, None);
        assert_eq!(team_login.last, None);
        assert_eq!(team_login.email, None);
        assert_eq!(team_login.phone, None);
        assert_eq!(team_login.last_login, None);
        assert_eq!(team_login.two_factor, false);
        assert_eq!(team_login.custom, None);
        assert_eq!(team_login.created, None);
        assert_eq!(team_login.modified, None);
        assert_eq!(team_login.inactive, false);
        assert_eq!(team_login.frozen, false);
    }

    #[test]
    fn test_team_login_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_tlg_12345678901234567890123", "inactive": 1}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.inactive, true);

        let json = r#"{"id": "t1_tlg_12345678901234567890123", "inactive": 0}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_tlg_12345678901234567890123", "frozen": 1}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.frozen, true);

        let json = r#"{"id": "t1_tlg_12345678901234567890123", "frozen": 0}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.frozen, false);

        // Test two_factor field with int values
        let json = r#"{"id": "t1_tlg_12345678901234567890123", "twoFactor": 1}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.two_factor, true);

        let json = r#"{"id": "t1_tlg_12345678901234567890123", "twoFactor": 0}"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.two_factor, false);
    }

    // NewTeamLogin struct tests
    #[test]
    fn test_new_team_login_serialize_full() {
        let new_team_login = NewTeamLogin {
            entity: "t1_ent_23456789012345678901234".to_string(),
            role: Some(TeamLoginRole::Manager),
            username: "john.doe".to_string(),
            password: "SecurePassword123!".to_string(),
            first: Some("John".to_string()),
            last: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"role\":2"));
        assert!(json.contains("\"username\":\"john.doe\""));
        assert!(json.contains("\"password\":\"SecurePassword123!\""));
        assert!(json.contains("\"first\":\"John\""));
        assert!(json.contains("\"last\":\"Doe\""));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_team_login_serialize_minimal() {
        let new_team_login = NewTeamLogin {
            entity: "t1_ent_23456789012345678901234".to_string(),
            username: "john.doe".to_string(),
            password: "SecurePassword123!".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_23456789012345678901234\""));
        assert!(json.contains("\"username\":\"john.doe\""));
        assert!(json.contains("\"password\":\"SecurePassword123!\""));
        assert!(!json.contains("role"));
        assert!(!json.contains("first"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_team_login_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_team_login = NewTeamLogin {
            entity: "t1_ent_23456789012345678901234".to_string(),
            username: "john.doe".to_string(),
            password: "password".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_team_login = NewTeamLogin {
            entity: "t1_ent_23456789012345678901234".to_string(),
            username: "john.doe".to_string(),
            password: "password".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_team_login = NewTeamLogin {
            entity: "t1_ent_23456789012345678901234".to_string(),
            username: "john.doe".to_string(),
            password: "password".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_team_login_serialize_roles() {
        // Test all role variants
        let variants = vec![
            (TeamLoginRole::Admin, "1"),
            (TeamLoginRole::Manager, "2"),
            (TeamLoginRole::User, "3"),
            (TeamLoginRole::ReadOnly, "4"),
        ];

        for (variant, expected) in variants {
            let new_team_login = NewTeamLogin {
                entity: "t1_ent_23456789012345678901234".to_string(),
                username: "john.doe".to_string(),
                password: "password".to_string(),
                role: Some(variant),
                ..Default::default()
            };
            let json = serde_json::to_string(&new_team_login).unwrap();
            assert!(json.contains(&format!("\"role\":{}", expected)));
        }
    }
}
