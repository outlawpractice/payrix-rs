//! Organization entity types for the Payrix API.
//!
//! Organization entities link entities to organizations.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// A Payrix organization entity link.
///
/// Links an entity to an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct OrgEntity {
    /// Unique identifier (30 characters, e.g., "t1_ore_...")
    pub id: PayrixId,

    /// Organization ID
    #[serde(default)]
    pub org: Option<PayrixId>,

    /// Entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Login ID that created this link
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Role within the organization
    #[serde(default)]
    pub role: Option<String>,

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

/// Request to create a new organization entity link.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrgEntity {
    /// Organization ID (required)
    pub org: String,

    /// Entity ID (required)
    pub entity: String,

    /// Role within the organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

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

    // OrgEntity struct tests
    #[test]
    fn test_org_entity_deserialize_full() {
        let json = r#"{
            "id": "t1_ore_12345678901234567890123",
            "org": "t1_org_23456789012345678901234",
            "entity": "t1_ent_34567890123456789012345",
            "login": "t1_tlg_45678901234567890123456",
            "role": "admin",
            "description": "Test description",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.id.as_str(), "t1_ore_12345678901234567890123");
        assert_eq!(org_entity.org, Some(PayrixId::new("t1_org_23456789012345678901234").unwrap()));
        assert_eq!(org_entity.entity, Some(PayrixId::new("t1_ent_34567890123456789012345").unwrap()));
        assert_eq!(org_entity.login, Some(PayrixId::new("t1_tlg_45678901234567890123456").unwrap()));
        assert_eq!(org_entity.role, Some("admin".to_string()));
        assert_eq!(org_entity.description, Some("Test description".to_string()));
        assert_eq!(org_entity.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(org_entity.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(org_entity.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(org_entity.inactive, false);
        assert_eq!(org_entity.frozen, true);
    }

    #[test]
    fn test_org_entity_deserialize_minimal() {
        let json = r#"{
            "id": "t1_ore_12345678901234567890123"
        }"#;

        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.id.as_str(), "t1_ore_12345678901234567890123");
        assert_eq!(org_entity.org, None);
        assert_eq!(org_entity.entity, None);
        assert_eq!(org_entity.login, None);
        assert_eq!(org_entity.role, None);
        assert_eq!(org_entity.description, None);
        assert_eq!(org_entity.custom, None);
        assert_eq!(org_entity.created, None);
        assert_eq!(org_entity.modified, None);
        assert_eq!(org_entity.inactive, false);
        assert_eq!(org_entity.frozen, false);
    }

    #[test]
    fn test_org_entity_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_ore_12345678901234567890123", "inactive": 1}"#;
        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.inactive, true);

        let json = r#"{"id": "t1_ore_12345678901234567890123", "inactive": 0}"#;
        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_ore_12345678901234567890123", "frozen": 1}"#;
        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.frozen, true);

        let json = r#"{"id": "t1_ore_12345678901234567890123", "frozen": 0}"#;
        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.frozen, false);
    }

    // NewOrgEntity struct tests
    #[test]
    fn test_new_org_entity_serialize_full() {
        let new_org_entity = NewOrgEntity {
            org: "t1_org_23456789012345678901234".to_string(),
            entity: "t1_ent_34567890123456789012345".to_string(),
            role: Some("admin".to_string()),
            description: Some("Test description".to_string()),
            custom: Some("{\"key\":\"value\"}".to_string()),
            inactive: Some(true),
        };

        let json = serde_json::to_string(&new_org_entity).unwrap();
        assert!(json.contains("\"org\":\"t1_org_23456789012345678901234\""));
        assert!(json.contains("\"entity\":\"t1_ent_34567890123456789012345\""));
        assert!(json.contains("\"role\":\"admin\""));
        assert!(json.contains("\"description\":\"Test description\""));
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn test_new_org_entity_serialize_minimal() {
        let new_org_entity = NewOrgEntity {
            org: "t1_org_23456789012345678901234".to_string(),
            entity: "t1_ent_34567890123456789012345".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_org_entity).unwrap();
        assert!(json.contains("\"org\":\"t1_org_23456789012345678901234\""));
        assert!(json.contains("\"entity\":\"t1_ent_34567890123456789012345\""));
        assert!(!json.contains("role"));
        assert!(!json.contains("description"));
        assert!(!json.contains("inactive"));
    }

    #[test]
    fn test_new_org_entity_serialize_with_inactive() {
        // Test with inactive = true (should serialize as 1)
        let new_org_entity = NewOrgEntity {
            org: "t1_org_23456789012345678901234".to_string(),
            entity: "t1_ent_34567890123456789012345".to_string(),
            inactive: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org_entity).unwrap();
        assert!(json.contains("\"inactive\":1"));

        // Test with inactive = false (should serialize as 0)
        let new_org_entity = NewOrgEntity {
            org: "t1_org_23456789012345678901234".to_string(),
            entity: "t1_ent_34567890123456789012345".to_string(),
            inactive: Some(false),
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org_entity).unwrap();
        assert!(json.contains("\"inactive\":0"));

        // Test with inactive = None (should not serialize)
        let new_org_entity = NewOrgEntity {
            org: "t1_org_23456789012345678901234".to_string(),
            entity: "t1_ent_34567890123456789012345".to_string(),
            inactive: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&new_org_entity).unwrap();
        assert!(!json.contains("inactive"));
    }
}
