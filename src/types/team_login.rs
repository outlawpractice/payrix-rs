//! Team login types for the Payrix API.
//!
//! Team logins represent permission assignments for logins within teams.
//! Per OpenAPI spec, these define CRUD permissions for a login on a specific team.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// A Payrix team login (permission assignment).
///
/// Per OpenAPI spec: Team logins define the permissions a login has within a team.
/// This represents CRUD and admin rights, not user account data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct TeamLogin {
    /// Unique identifier (30 characters, e.g., "t1_tlg_...")
    pub id: PayrixId,

    /// Login ID this permission applies to
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Team ID this permission is for
    #[serde(default)]
    pub team: Option<PayrixId>,

    /// Create permission (0/1)
    #[serde(default, with = "bool_from_int_default_false")]
    pub create: bool,

    /// Read permission (0/1)
    #[serde(default, with = "bool_from_int_default_false")]
    pub read: bool,

    /// Update permission (0/1)
    #[serde(default, with = "bool_from_int_default_false")]
    pub update: bool,

    /// Delete permission (0/1)
    #[serde(default, with = "bool_from_int_default_false")]
    pub delete: bool,

    /// Reference permission (0/1) - ability to reference this resource
    #[serde(default, with = "bool_from_int_default_false")]
    pub reference: bool,

    /// Team admin permission (0/1)
    #[serde(default, with = "bool_from_int_default_false")]
    pub team_admin: bool,

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

/// Request to create a new team login (permission assignment).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTeamLogin {
    /// Login ID (required)
    pub login: String,

    /// Team ID (required)
    pub team: String,

    /// Create permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub create: Option<bool>,

    /// Read permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub read: Option<bool>,

    /// Update permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub update: Option<bool>,

    /// Delete permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub delete: Option<bool>,

    /// Reference permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub reference: Option<bool>,

    /// Team admin permission
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub team_admin: Option<bool>,

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

    // TeamLogin struct tests - per OpenAPI, this is a permissions assignment
    #[test]
    fn test_team_login_deserialize_full() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "login": "t1_log_23456789012345678901234",
            "team": "t1_tea_34567890123456789012345",
            "create": 1,
            "read": 1,
            "update": 1,
            "delete": 0,
            "reference": 1,
            "teamAdmin": 1,
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 0
        }"#;

        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.id.as_str(), "t1_tlg_12345678901234567890123");
        assert_eq!(team_login.login, Some(PayrixId::new("t1_log_23456789012345678901234").unwrap()));
        assert_eq!(team_login.team, Some(PayrixId::new("t1_tea_34567890123456789012345").unwrap()));
        assert!(team_login.create);
        assert!(team_login.read);
        assert!(team_login.update);
        assert!(!team_login.delete);
        assert!(team_login.reference);
        assert!(team_login.team_admin);
        assert_eq!(team_login.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(team_login.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(team_login.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert!(!team_login.inactive);
        assert!(!team_login.frozen);
    }

    #[test]
    fn test_team_login_deserialize_minimal() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123"
        }"#;

        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(team_login.id.as_str(), "t1_tlg_12345678901234567890123");
        assert!(team_login.login.is_none());
        assert!(team_login.team.is_none());
        assert!(!team_login.create);
        assert!(!team_login.read);
        assert!(!team_login.update);
        assert!(!team_login.delete);
        assert!(!team_login.reference);
        assert!(!team_login.team_admin);
        assert!(team_login.custom.is_none());
        assert!(team_login.created.is_none());
        assert!(team_login.modified.is_none());
        assert!(!team_login.inactive);
        assert!(!team_login.frozen);
    }

    #[test]
    fn test_team_login_bool_from_int() {
        // Test all permission fields with int values
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "create": 1,
            "read": 0,
            "update": 1,
            "delete": 0,
            "reference": 1,
            "teamAdmin": 0,
            "inactive": 1,
            "frozen": 0
        }"#;
        let team_login: TeamLogin = serde_json::from_str(json).unwrap();
        assert!(team_login.create);
        assert!(!team_login.read);
        assert!(team_login.update);
        assert!(!team_login.delete);
        assert!(team_login.reference);
        assert!(!team_login.team_admin);
        assert!(team_login.inactive);
        assert!(!team_login.frozen);
    }

    // NewTeamLogin struct tests
    #[test]
    fn test_new_team_login_serialize_full() {
        let new_team_login = NewTeamLogin {
            login: "t1_log_23456789012345678901234".to_string(),
            team: "t1_tea_34567890123456789012345".to_string(),
            create: Some(true),
            read: Some(true),
            update: Some(true),
            delete: Some(false),
            reference: Some(true),
            team_admin: Some(true),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"login\":\"t1_log_23456789012345678901234\""));
        assert!(json.contains("\"team\":\"t1_tea_34567890123456789012345\""));
        assert!(json.contains("\"create\":1"));
        assert!(json.contains("\"read\":1"));
        assert!(json.contains("\"update\":1"));
        assert!(json.contains("\"delete\":0"));
        assert!(json.contains("\"reference\":1"));
        assert!(json.contains("\"teamAdmin\":1"));
        assert!(json.contains("\"custom\":\"{\\\"key\\\":\\\"value\\\"}\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn test_new_team_login_serialize_minimal() {
        let new_team_login = NewTeamLogin {
            login: "t1_log_23456789012345678901234".to_string(),
            team: "t1_tea_34567890123456789012345".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"login\":\"t1_log_23456789012345678901234\""));
        assert!(json.contains("\"team\":\"t1_tea_34567890123456789012345\""));
        // Optional fields should be omitted
        assert!(!json.contains("create"));
        assert!(!json.contains("read"));
        assert!(!json.contains("update"));
        assert!(!json.contains("delete"));
        assert!(!json.contains("reference"));
        assert!(!json.contains("teamAdmin"));
        assert!(!json.contains("custom"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_team_login_permissions() {
        // Test with all permissions enabled
        let new_team_login = NewTeamLogin {
            login: "t1_log_23456789012345678901234".to_string(),
            team: "t1_tea_34567890123456789012345".to_string(),
            create: Some(true),
            read: Some(true),
            update: Some(true),
            delete: Some(true),
            reference: Some(true),
            team_admin: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_team_login).unwrap();
        assert!(json.contains("\"create\":1"));
        assert!(json.contains("\"read\":1"));
        assert!(json.contains("\"update\":1"));
        assert!(json.contains("\"delete\":1"));
        assert!(json.contains("\"reference\":1"));
        assert!(json.contains("\"teamAdmin\":1"));
    }
}
