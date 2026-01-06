//! Organization entity types for the Payrix API.
//!
//! Organization entities link entities to organizations.
//!
//! **OpenAPI schema:** `orgEntitiesResponse`

use serde::{Deserialize, Serialize};

use super::PayrixId;

// =============================================================================
// ORG ENTITY STRUCT
// =============================================================================

/// A Payrix organization entity link.
///
/// Links an entity to an organization.
///
/// **OpenAPI schema:** `orgEntitiesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct OrgEntity {
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

    /// The identifier of the Org that this orgEntity is associated with.
    ///
    /// **OpenAPI type:** string (ref: orgEntitiesModelOrg)
    #[serde(default)]
    pub org: Option<PayrixId>,

    /// The identifier of the Entity that this orgEntity is associated with.
    ///
    /// **OpenAPI type:** string (ref: orgEntitiesModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== OrgEntity Struct Tests ====================

    #[test]
    fn org_entity_deserialize_full() {
        let json = r#"{
            "id": "t1_ore_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "org": "t1_org_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123"
        }"#;

        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.id.as_str(), "t1_ore_12345678901234567890123");
        assert_eq!(org_entity.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(org_entity.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(org_entity.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(org_entity.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(org_entity.org.as_ref().map(|o| o.as_str()), Some("t1_org_12345678901234567890123"));
        assert_eq!(org_entity.entity.as_ref().map(|e| e.as_str()), Some("t1_ent_12345678901234567890123"));
    }

    #[test]
    fn org_entity_deserialize_minimal() {
        let json = r#"{"id": "t1_ore_12345678901234567890123"}"#;

        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        assert_eq!(org_entity.id.as_str(), "t1_ore_12345678901234567890123");
        assert!(org_entity.created.is_none());
        assert!(org_entity.modified.is_none());
        assert!(org_entity.creator.is_none());
        assert!(org_entity.modifier.is_none());
        assert!(org_entity.org.is_none());
        assert!(org_entity.entity.is_none());
    }

    #[test]
    fn org_entity_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_ore_12345678901234567890123",
            "org": "t1_org_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123"
        }"#;

        let org_entity: OrgEntity = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&org_entity).unwrap();
        let deserialized: OrgEntity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(org_entity.id, deserialized.id);
        assert_eq!(org_entity.org, deserialized.org);
        assert_eq!(org_entity.entity, deserialized.entity);
    }
}
