//! Reserve types for the Payrix API.
//!
//! Reserves hold funds aside for risk management purposes.
//!
//! **OpenAPI schema:** `reservesResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// ENUMS
// =============================================================================

/// Reserve status values per OpenAPI spec.
///
/// **OpenAPI schema:** `reserveStatus`
///
/// Valid values:
/// - `1` - The reserve is active
/// - `2` - The reserve is under review
/// - `3` - The reserve is inactive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ReserveStatus {
    /// The reserve is active.
    #[default]
    Active = 1,
    /// The reserve is under review.
    UnderReview = 2,
    /// The reserve is inactive.
    Inactive = 3,
}

/// Reserve level values per OpenAPI spec.
///
/// The hierarchical level for this reserve setting.
///
/// **OpenAPI schema:** `Level`
///
/// Valid values:
/// - `admin` - Admin User
/// - `division` - Division-level User
/// - `merchant` - Merchant-level User
/// - `partition` - Partition-level User
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReserveLevel {
    /// Admin User.
    #[default]
    Admin,
    /// Division-level User.
    Division,
    /// Merchant-level User.
    Merchant,
    /// Partition-level User.
    Partition,
}

/// Reserve release schedule values per OpenAPI spec.
///
/// The schedule that determines when the funds in this Reserve should be released.
///
/// **OpenAPI schema:** `Release`
///
/// Valid values:
/// - `never` - Never
/// - `days` - Daily (funds released every day)
/// - `weeks` - Weekly (funds released every week)
/// - `months` - Monthly (funds released every month)
/// - `years` - Annually (funds released every year)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseSchedule {
    /// Never release the funds.
    #[default]
    Never,
    /// Daily. The funds are released every day.
    Days,
    /// Weekly. The funds are released every week.
    Weeks,
    /// Monthly. The funds are released every month.
    Months,
    /// Annually. The funds are released every year.
    Years,
}

// =============================================================================
// RESERVE STRUCT
// =============================================================================

/// A Payrix reserve.
///
/// Reserves hold funds for risk management purposes.
///
/// **OpenAPI schema:** `reservesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Reserve {
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

    /// The identifier of the Login that owns this Reserve.
    ///
    /// **OpenAPI type:** string (ref: reservesModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Org that this Reserve's resource applies to.
    ///
    /// If you set this field, then the Reserve applies to all Entities in the Org.
    ///
    /// **OpenAPI type:** string (ref: reservesModelOrg)
    #[serde(default)]
    pub org: Option<PayrixId>,

    /// The identifier of the Division that this Reserve's resource applies to.
    ///
    /// **OpenAPI type:** string (ref: reservesModelDivision)
    #[serde(default)]
    pub division: Option<PayrixId>,

    /// The identifier of the Partition that this Reserve's resource applies to.
    ///
    /// **OpenAPI type:** string (ref: reservesModelPartition)
    #[serde(default)]
    pub partition: Option<PayrixId>,

    /// The hierarchical level for this reserve setting.
    ///
    /// **OpenAPI type:** string (ref: Level)
    #[serde(default)]
    pub level: Option<ReserveLevel>,

    /// The identifier of the Entity that this Reserve applies to.
    ///
    /// **OpenAPI type:** string (ref: reservesModelEntity)
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// The identifier of the Entity Hold from which this Reserve was generated.
    ///
    /// **OpenAPI type:** string (ref: reservesModelHold)
    #[serde(default)]
    pub hold: Option<PayrixId>,

    /// The name of this Reserve.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Reserve.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The percentage of funds to reserve, expressed in basis points.
    ///
    /// For example, 25.3% is expressed as '2530'.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub percent: Option<i32>,

    /// The schedule that determines when the funds in this Reserve should be released.
    ///
    /// **OpenAPI type:** string (ref: Release)
    #[serde(default)]
    pub release: Option<ReleaseSchedule>,

    /// A multiplier to adjust the schedule set in the 'release' field.
    ///
    /// Its value determines how the schedule is multiplied.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub release_factor: Option<i32>,

    /// The date on which this Reserve resource should start reserving funds.
    ///
    /// The date is specified as an eight digit integer in YYYYMMDD format,
    /// for example, 20160120 for January 20, 2016.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub start: Option<i32>,

    /// The date on which this Reserve resource should stop reserving funds.
    ///
    /// The date is specified as an eight digit integer in YYYYMMDD format,
    /// for example, 20160120 for January 20, 2016.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub finish: Option<i32>,

    /// The maximum amount to reserve.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub max: Option<i64>,

    /// The status of reserve.
    ///
    /// - `1` - The reserve is active
    /// - `2` - The reserve is under review
    /// - `3` - The reserve is inactive
    ///
    /// **OpenAPI type:** integer (ref: reserveStatus)
    #[serde(default)]
    pub status: Option<ReserveStatus>,

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

    /// Reserve entries associated with this reserve.
    ///
    /// **OpenAPI type:** array of reserveEntriesResponse
    #[cfg(not(feature = "sqlx"))]
    #[serde(default)]
    pub reserve_entries: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ReserveStatus Tests ====================

    #[test]
    fn reserve_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&ReserveStatus::Active).unwrap(), "1");
        assert_eq!(
            serde_json::to_string(&ReserveStatus::UnderReview).unwrap(),
            "2"
        );
        assert_eq!(
            serde_json::to_string(&ReserveStatus::Inactive).unwrap(),
            "3"
        );
    }

    #[test]
    fn reserve_status_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<ReserveStatus>("1").unwrap(),
            ReserveStatus::Active
        );
        assert_eq!(
            serde_json::from_str::<ReserveStatus>("2").unwrap(),
            ReserveStatus::UnderReview
        );
        assert_eq!(
            serde_json::from_str::<ReserveStatus>("3").unwrap(),
            ReserveStatus::Inactive
        );
    }

    #[test]
    fn reserve_status_default() {
        assert_eq!(ReserveStatus::default(), ReserveStatus::Active);
    }

    #[test]
    fn reserve_status_invalid_value() {
        assert!(serde_json::from_str::<ReserveStatus>("0").is_err());
        assert!(serde_json::from_str::<ReserveStatus>("99").is_err());
    }

    // ==================== ReserveLevel Tests ====================

    #[test]
    fn reserve_level_serialize_all_variants() {
        assert_eq!(
            serde_json::to_string(&ReserveLevel::Admin).unwrap(),
            "\"admin\""
        );
        assert_eq!(
            serde_json::to_string(&ReserveLevel::Division).unwrap(),
            "\"division\""
        );
        assert_eq!(
            serde_json::to_string(&ReserveLevel::Merchant).unwrap(),
            "\"merchant\""
        );
        assert_eq!(
            serde_json::to_string(&ReserveLevel::Partition).unwrap(),
            "\"partition\""
        );
    }

    #[test]
    fn reserve_level_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<ReserveLevel>("\"admin\"").unwrap(),
            ReserveLevel::Admin
        );
        assert_eq!(
            serde_json::from_str::<ReserveLevel>("\"division\"").unwrap(),
            ReserveLevel::Division
        );
        assert_eq!(
            serde_json::from_str::<ReserveLevel>("\"merchant\"").unwrap(),
            ReserveLevel::Merchant
        );
        assert_eq!(
            serde_json::from_str::<ReserveLevel>("\"partition\"").unwrap(),
            ReserveLevel::Partition
        );
    }

    // ==================== ReleaseSchedule Tests ====================

    #[test]
    fn release_schedule_serialize_all_variants() {
        assert_eq!(
            serde_json::to_string(&ReleaseSchedule::Never).unwrap(),
            "\"never\""
        );
        assert_eq!(
            serde_json::to_string(&ReleaseSchedule::Days).unwrap(),
            "\"days\""
        );
        assert_eq!(
            serde_json::to_string(&ReleaseSchedule::Weeks).unwrap(),
            "\"weeks\""
        );
        assert_eq!(
            serde_json::to_string(&ReleaseSchedule::Months).unwrap(),
            "\"months\""
        );
        assert_eq!(
            serde_json::to_string(&ReleaseSchedule::Years).unwrap(),
            "\"years\""
        );
    }

    #[test]
    fn release_schedule_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<ReleaseSchedule>("\"never\"").unwrap(),
            ReleaseSchedule::Never
        );
        assert_eq!(
            serde_json::from_str::<ReleaseSchedule>("\"days\"").unwrap(),
            ReleaseSchedule::Days
        );
        assert_eq!(
            serde_json::from_str::<ReleaseSchedule>("\"weeks\"").unwrap(),
            ReleaseSchedule::Weeks
        );
        assert_eq!(
            serde_json::from_str::<ReleaseSchedule>("\"months\"").unwrap(),
            ReleaseSchedule::Months
        );
        assert_eq!(
            serde_json::from_str::<ReleaseSchedule>("\"years\"").unwrap(),
            ReleaseSchedule::Years
        );
    }

    // ==================== Reserve Struct Tests ====================

    #[test]
    fn reserve_deserialize_full() {
        let json = r#"{
            "id": "t1_rsv_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "org": "t1_org_12345678901234567890123",
            "division": "t1_div_12345678901234567890123",
            "partition": "t1_par_12345678901234567890123",
            "level": "merchant",
            "entity": "t1_ent_12345678901234567890123",
            "hold": "t1_hld_12345678901234567890123",
            "name": "Risk Reserve",
            "description": "Rolling reserve for merchant",
            "percent": 2530,
            "release": "months",
            "releaseFactor": 6,
            "start": 20240101,
            "finish": 20241231,
            "max": 1000000,
            "status": 1,
            "inactive": 0,
            "frozen": 1
        }"#;

        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert_eq!(reserve.id.as_str(), "t1_rsv_12345678901234567890123");
        assert_eq!(reserve.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(
            reserve.modified,
            Some("2024-01-02 23:59:59.9999".to_string())
        );
        assert_eq!(
            reserve.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            reserve.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            reserve.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            reserve.org.as_ref().map(|o| o.as_str()),
            Some("t1_org_12345678901234567890123")
        );
        assert_eq!(
            reserve.division.as_ref().map(|d| d.as_str()),
            Some("t1_div_12345678901234567890123")
        );
        assert_eq!(
            reserve.partition.as_ref().map(|p| p.as_str()),
            Some("t1_par_12345678901234567890123")
        );
        assert_eq!(reserve.level, Some(ReserveLevel::Merchant));
        assert_eq!(
            reserve.entity.as_ref().map(|e| e.as_str()),
            Some("t1_ent_12345678901234567890123")
        );
        assert_eq!(
            reserve.hold.as_ref().map(|h| h.as_str()),
            Some("t1_hld_12345678901234567890123")
        );
        assert_eq!(reserve.name, Some("Risk Reserve".to_string()));
        assert_eq!(
            reserve.description,
            Some("Rolling reserve for merchant".to_string())
        );
        assert_eq!(reserve.percent, Some(2530));
        assert_eq!(reserve.release, Some(ReleaseSchedule::Months));
        assert_eq!(reserve.release_factor, Some(6));
        assert_eq!(reserve.start, Some(20240101));
        assert_eq!(reserve.finish, Some(20241231));
        assert_eq!(reserve.max, Some(1000000));
        assert_eq!(reserve.status, Some(ReserveStatus::Active));
        assert!(!reserve.inactive);
        assert!(reserve.frozen);
    }

    #[test]
    fn reserve_deserialize_minimal() {
        let json = r#"{"id": "t1_rsv_12345678901234567890123"}"#;

        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert_eq!(reserve.id.as_str(), "t1_rsv_12345678901234567890123");
        assert!(reserve.created.is_none());
        assert!(reserve.modified.is_none());
        assert!(reserve.creator.is_none());
        assert!(reserve.modifier.is_none());
        assert!(reserve.login.is_none());
        assert!(reserve.org.is_none());
        assert!(reserve.division.is_none());
        assert!(reserve.partition.is_none());
        assert!(reserve.level.is_none());
        assert!(reserve.entity.is_none());
        assert!(reserve.hold.is_none());
        assert!(reserve.name.is_none());
        assert!(reserve.description.is_none());
        assert!(reserve.percent.is_none());
        assert!(reserve.release.is_none());
        assert!(reserve.release_factor.is_none());
        assert!(reserve.start.is_none());
        assert!(reserve.finish.is_none());
        assert!(reserve.max.is_none());
        assert!(reserve.status.is_none());
        assert!(!reserve.inactive);
        assert!(!reserve.frozen);
    }

    #[test]
    fn reserve_bool_from_int() {
        let json = r#"{"id": "t1_rsv_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert!(reserve.inactive);
        assert!(reserve.frozen);

        let json = r#"{"id": "t1_rsv_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert!(!reserve.inactive);
        assert!(!reserve.frozen);
    }

    #[test]
    fn reserve_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_rsv_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "percent": 2530,
            "release": "months"
        }"#;

        let reserve: Reserve = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&reserve).unwrap();
        let deserialized: Reserve = serde_json::from_str(&serialized).unwrap();
        assert_eq!(reserve.id, deserialized.id);
        assert_eq!(reserve.entity, deserialized.entity);
        assert_eq!(reserve.percent, deserialized.percent);
        assert_eq!(reserve.release, deserialized.release);
    }
}
