//! Entity reserve types for the Payrix API.
//!
//! Entity reserves configure reserve requirements at the entity level.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId, ReserveType};

/// A Payrix entity reserve configuration.
///
/// Entity reserves define default reserve settings for merchants under an entity.
/// All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct EntityReserve {
    /// Unique identifier (30 characters, e.g., "t1_ers_...")
    pub id: PayrixId,

    /// Entity ID that owns this configuration
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Login ID that created this configuration
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Reserve type
    #[serde(default, rename = "type")]
    pub reserve_type: Option<ReserveType>,

    /// Default reserve percentage
    #[serde(default)]
    pub percent: Option<i32>,

    /// Default cap amount in cents
    #[serde(default)]
    pub cap: Option<i64>,

    /// Default hold days
    #[serde(default)]
    pub hold_days: Option<i32>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Configuration name/label
    #[serde(default)]
    pub name: Option<String>,

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

/// Request to create a new entity reserve configuration.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewEntityReserve {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Reserve type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub reserve_type: Option<ReserveType>,

    /// Default reserve percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,

    /// Default cap amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<i64>,

    /// Default hold days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_days: Option<i32>,

    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Configuration name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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

    // ==================== EntityReserve Struct Tests ====================

    #[test]
    fn entity_reserve_deserialize_full() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "type": 1,
            "percent": 10,
            "cap": 100000,
            "holdDays": 30,
            "currency": "USD",
            "name": "Default Reserve",
            "description": "Default reserve configuration",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.id.as_str(), "t1_ers_12345678901234567890123");
        assert_eq!(entity_reserve.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entity_reserve.login.unwrap().as_str(), "t1_log_12345678901234567890123");
        assert_eq!(entity_reserve.reserve_type, Some(ReserveType::Rolling));
        assert_eq!(entity_reserve.percent, Some(10));
        assert_eq!(entity_reserve.cap, Some(100000));
        assert_eq!(entity_reserve.hold_days, Some(30));
        assert_eq!(entity_reserve.currency, Some("USD".to_string()));
        assert_eq!(entity_reserve.name, Some("Default Reserve".to_string()));
        assert_eq!(entity_reserve.description, Some("Default reserve configuration".to_string()));
        assert_eq!(entity_reserve.custom, Some("custom data".to_string()));
        assert_eq!(entity_reserve.created, Some("2024-01-01 00:00:00.000".to_string()));
        assert_eq!(entity_reserve.modified, Some("2024-04-01 12:00:00.000".to_string()));
        assert!(!entity_reserve.inactive);
        assert!(entity_reserve.frozen);
    }

    #[test]
    fn entity_reserve_deserialize_minimal() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123"
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.id.as_str(), "t1_ers_12345678901234567890123");
        assert!(entity_reserve.entity.is_none());
        assert!(entity_reserve.login.is_none());
        assert!(entity_reserve.reserve_type.is_none());
        assert!(entity_reserve.percent.is_none());
        assert!(entity_reserve.cap.is_none());
        assert!(entity_reserve.hold_days.is_none());
        assert!(entity_reserve.currency.is_none());
        assert!(entity_reserve.name.is_none());
        assert!(entity_reserve.description.is_none());
        assert!(entity_reserve.custom.is_none());
        assert!(entity_reserve.created.is_none());
        assert!(entity_reserve.modified.is_none());
        assert!(!entity_reserve.inactive);
        assert!(!entity_reserve.frozen);
    }

    #[test]
    fn entity_reserve_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_ers_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert!(!entity_reserve.inactive);
        assert!(!entity_reserve.frozen);
    }

    #[test]
    fn entity_reserve_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_ers_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert!(entity_reserve.inactive);
        assert!(entity_reserve.frozen);
    }

    #[test]
    fn entity_reserve_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_ers_12345678901234567890123"}"#;
        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert!(!entity_reserve.inactive);
        assert!(!entity_reserve.frozen);
    }

    #[test]
    fn entity_reserve_all_reserve_type_variants() {
        let test_cases = vec![
            (1, ReserveType::Rolling),
            (2, ReserveType::Fixed),
            (3, ReserveType::Upfront),
        ];

        for (type_val, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ers_12345678901234567890123", "type": {}}}"#,
                type_val
            );
            let entity_reserve: EntityReserve = serde_json::from_str(&json).unwrap();
            assert_eq!(entity_reserve.reserve_type, Some(expected_type));
        }
    }

    #[test]
    fn entity_reserve_rolling_reserve_config() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "type": 1,
            "percent": 10,
            "cap": 50000,
            "holdDays": 30
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.reserve_type, Some(ReserveType::Rolling));
        assert_eq!(entity_reserve.percent, Some(10));
        assert_eq!(entity_reserve.cap, Some(50000));
        assert_eq!(entity_reserve.hold_days, Some(30));
    }

    #[test]
    fn entity_reserve_fixed_reserve_config() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "type": 2,
            "cap": 100000
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.reserve_type, Some(ReserveType::Fixed));
        assert_eq!(entity_reserve.cap, Some(100000));
    }

    #[test]
    fn entity_reserve_serialize_roundtrip() {
        let entity_reserve = EntityReserve {
            id: "t1_ers_12345678901234567890123".parse().unwrap(),
            entity: Some("t1_ent_12345678901234567890123".parse().unwrap()),
            login: Some("t1_log_12345678901234567890123".parse().unwrap()),
            reserve_type: Some(ReserveType::Rolling),
            percent: Some(15),
            cap: Some(75000),
            hold_days: Some(45),
            currency: Some("USD".to_string()),
            name: Some("Test Reserve".to_string()),
            description: Some("Test".to_string()),
            custom: Some("custom".to_string()),
            created: Some("2024-01-01 00:00:00.000".to_string()),
            modified: Some("2024-01-02 00:00:00.000".to_string()),
            inactive: false,
            frozen: true,
        };

        let json = serde_json::to_string(&entity_reserve).unwrap();
        let deserialized: EntityReserve = serde_json::from_str(&json).unwrap();
        assert_eq!(entity_reserve, deserialized);
    }

    #[test]
    fn entity_reserve_zero_percent() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "percent": 0
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.percent, Some(0));
    }

    #[test]
    fn entity_reserve_max_percent() {
        let json = r#"{
            "id": "t1_ers_12345678901234567890123",
            "percent": 100
        }"#;

        let entity_reserve: EntityReserve = serde_json::from_str(json).unwrap();
        assert_eq!(entity_reserve.percent, Some(100));
    }

    // ==================== NewEntityReserve Tests ====================

    #[test]
    fn new_entity_reserve_serialize_full() {
        let new_entity_reserve = NewEntityReserve {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            reserve_type: Some(ReserveType::Rolling),
            percent: Some(10),
            cap: Some(100000),
            hold_days: Some(30),
            currency: Some("USD".to_string()),
            name: Some("Default Reserve".to_string()),
            description: Some("Default reserve configuration".to_string()),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"percent\":10"));
        assert!(json.contains("\"cap\":100000"));
        assert!(json.contains("\"holdDays\":30"));
        assert!(json.contains("\"currency\":\"USD\""));
        assert!(json.contains("\"name\":\"Default Reserve\""));
        assert!(json.contains("\"description\":\"Default reserve configuration\""));
        assert!(json.contains("\"custom\":\"custom data\""));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_entity_reserve_serialize_minimal() {
        let new_entity_reserve = NewEntityReserve {
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        // All optional fields should be omitted
        assert!(!json.contains("\"entity\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"percent\""));
        assert!(!json.contains("\"cap\""));
        assert!(!json.contains("\"holdDays\""));
        assert!(!json.contains("\"currency\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"description\""));
        assert!(!json.contains("\"custom\""));
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_entity_reserve_option_bool_to_int_true() {
        let new_entity_reserve = NewEntityReserve {
            inactive: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn new_entity_reserve_option_bool_to_int_false() {
        let new_entity_reserve = NewEntityReserve {
            inactive: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_entity_reserve_option_bool_none_omitted() {
        let new_entity_reserve = NewEntityReserve {
            inactive: None,
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_entity_reserve_all_reserve_type_variants() {
        let test_cases = vec![
            (ReserveType::Rolling, 1),
            (ReserveType::Fixed, 2),
            (ReserveType::Upfront, 3),
        ];

        for (reserve_type, expected_val) in test_cases {
            let new_entity_reserve = NewEntityReserve {
                reserve_type: Some(reserve_type),
                ..Default::default()
            };

            let json = serde_json::to_string(&new_entity_reserve).unwrap();
            assert!(json.contains(&format!("\"type\":{}", expected_val)));
        }
    }

    #[test]
    fn new_entity_reserve_rolling_config() {
        let new_entity_reserve = NewEntityReserve {
            reserve_type: Some(ReserveType::Rolling),
            percent: Some(15),
            cap: Some(50000),
            hold_days: Some(60),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"percent\":15"));
        assert!(json.contains("\"cap\":50000"));
        assert!(json.contains("\"holdDays\":60"));
    }

    #[test]
    fn new_entity_reserve_fixed_config() {
        let new_entity_reserve = NewEntityReserve {
            reserve_type: Some(ReserveType::Fixed),
            cap: Some(100000),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"cap\":100000"));
        // Percent should not be present
        assert!(!json.contains("\"percent\""));
    }

    #[test]
    fn new_entity_reserve_upfront_config() {
        let new_entity_reserve = NewEntityReserve {
            reserve_type: Some(ReserveType::Upfront),
            cap: Some(25000),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"type\":3"));
        assert!(json.contains("\"cap\":25000"));
    }

    #[test]
    fn new_entity_reserve_zero_percent() {
        let new_entity_reserve = NewEntityReserve {
            percent: Some(0),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"percent\":0"));
    }

    #[test]
    fn new_entity_reserve_max_percent() {
        let new_entity_reserve = NewEntityReserve {
            percent: Some(100),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"percent\":100"));
    }

    #[test]
    fn new_entity_reserve_zero_hold_days() {
        let new_entity_reserve = NewEntityReserve {
            hold_days: Some(0),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"holdDays\":0"));
    }

    #[test]
    fn new_entity_reserve_large_cap() {
        let new_entity_reserve = NewEntityReserve {
            cap: Some(999999999),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_entity_reserve).unwrap();
        assert!(json.contains("\"cap\":999999999"));
    }
}
