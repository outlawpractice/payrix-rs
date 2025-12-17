//! Team login types for the Payrix API.
//!
//! Team logins represent permission assignments for logins within teams.
//!
//! **OpenAPI schema:** `teamLoginResponse`

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// TEAM LOGIN STRUCT
// =============================================================================

/// A Payrix team login (permission assignment).
///
/// Represents the link between a Login and a Team as well as the Login's
/// rights on the Team. The Login resource identified in its 'login' field
/// is considered part of the Team.
///
/// **OpenAPI schema:** `teamLoginResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct TeamLogin {
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

    /// The Login that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: teamLoginModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Team resource that the Login identified in the
    /// 'login' field should be marked as part of.
    ///
    /// **OpenAPI type:** string (ref: teamLoginModelTeam)
    #[serde(default)]
    pub team: Option<PayrixId>,

    /// Create rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: Create)
    #[serde(default, with = "bool_from_int_default_false")]
    pub create: bool,

    /// Read rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: teamLoginRead)
    #[serde(default, with = "bool_from_int_default_false")]
    pub read: bool,

    /// Update rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: Update)
    #[serde(default, with = "bool_from_int_default_false")]
    pub update: bool,

    /// Delete rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: Delete)
    #[serde(default, with = "bool_from_int_default_false")]
    pub delete: bool,

    /// Reference use rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: Reference)
    #[serde(default, with = "bool_from_int_default_false")]
    pub reference: bool,

    /// Team administration rights for this Login on this Team.
    ///
    /// - `0` - None
    /// - `1` - Allow
    ///
    /// **OpenAPI type:** integer (ref: TeamAdmin)
    #[serde(default, with = "bool_from_int_default_false")]
    pub team_admin: bool,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== TeamLogin Struct Tests ====================

    #[test]
    fn team_login_deserialize_full() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_23456789012345678901234",
            "team": "t1_tea_34567890123456789012345",
            "create": 1,
            "read": 1,
            "update": 1,
            "delete": 0,
            "reference": 1,
            "teamAdmin": 1
        }"#;

        let tl: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(tl.id.as_str(), "t1_tlg_12345678901234567890123");
        assert_eq!(tl.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(tl.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            tl.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            tl.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            tl.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_23456789012345678901234")
        );
        assert_eq!(
            tl.team.as_ref().map(|t| t.as_str()),
            Some("t1_tea_34567890123456789012345")
        );
        assert!(tl.create);
        assert!(tl.read);
        assert!(tl.update);
        assert!(!tl.delete);
        assert!(tl.reference);
        assert!(tl.team_admin);
    }

    #[test]
    fn team_login_deserialize_minimal() {
        let json = r#"{"id": "t1_tlg_12345678901234567890123"}"#;

        let tl: TeamLogin = serde_json::from_str(json).unwrap();
        assert_eq!(tl.id.as_str(), "t1_tlg_12345678901234567890123");
        assert!(tl.created.is_none());
        assert!(tl.modified.is_none());
        assert!(tl.creator.is_none());
        assert!(tl.modifier.is_none());
        assert!(tl.login.is_none());
        assert!(tl.team.is_none());
        assert!(!tl.create);
        assert!(!tl.read);
        assert!(!tl.update);
        assert!(!tl.delete);
        assert!(!tl.reference);
        assert!(!tl.team_admin);
    }

    #[test]
    fn team_login_bool_from_int() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "create": 1,
            "read": 0,
            "update": 1,
            "delete": 0,
            "reference": 1,
            "teamAdmin": 0
        }"#;

        let tl: TeamLogin = serde_json::from_str(json).unwrap();
        assert!(tl.create);
        assert!(!tl.read);
        assert!(tl.update);
        assert!(!tl.delete);
        assert!(tl.reference);
        assert!(!tl.team_admin);
    }

    #[test]
    fn team_login_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_tlg_12345678901234567890123",
            "login": "t1_lgn_23456789012345678901234",
            "team": "t1_tea_34567890123456789012345"
        }"#;

        let tl: TeamLogin = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&tl).unwrap();
        let deserialized: TeamLogin = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tl.id, deserialized.id);
        assert_eq!(tl.login, deserialized.login);
        assert_eq!(tl.team, deserialized.team);
    }
}
