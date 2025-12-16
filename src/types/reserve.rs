//! Reserve types for the Payrix API.
//!
//! Reserves hold funds aside for risk management purposes, such as
//! potential chargebacks or merchant obligations.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, option_bool_from_int, DateYmd, PayrixId};

/// Reserve status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ReserveStatus {
    /// Reserve is active
    #[default]
    Active = 1,
    /// Reserve is released
    Released = 2,
    /// Reserve is partially released
    PartiallyReleased = 3,
    /// Reserve is expired
    Expired = 4,
}

/// Reserve type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ReserveType {
    /// Rolling reserve (percentage of transactions)
    #[default]
    Rolling = 1,
    /// Fixed/capped reserve
    Fixed = 2,
    /// Upfront reserve
    Upfront = 3,
}

/// A Payrix reserve.
///
/// Reserves hold funds for risk management. All monetary values are in **cents**.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Reserve {
    /// Unique identifier (30 characters, e.g., "t1_rsv_...")
    pub id: PayrixId,

    /// Entity ID that owns this reserve
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Fund ID for this reserve
    #[serde(default)]
    pub fund: Option<PayrixId>,

    /// Login ID that created this reserve
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Reserve status
    #[serde(default)]
    pub status: Option<ReserveStatus>,

    /// Reserve type
    #[serde(default, rename = "type")]
    pub reserve_type: Option<ReserveType>,

    /// Reserve amount in cents (current held amount)
    #[serde(default)]
    pub amount: Option<i64>,

    /// Maximum/cap amount in cents
    #[serde(default)]
    pub cap: Option<i64>,

    /// Percentage of transactions to reserve
    #[serde(default)]
    pub percent: Option<i32>,

    /// Currency code (e.g., "USD")
    #[serde(default)]
    pub currency: Option<String>,

    /// Number of days to hold reserves
    #[serde(default)]
    pub hold_days: Option<i32>,

    /// Release date (YYYYMMDD format)
    #[serde(default)]
    pub release_date: Option<DateYmd>,

    /// Reserve name/label
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

/// Request to create a new reserve.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewReserve {
    /// Entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    /// Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,

    /// Fund ID for this reserve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund: Option<String>,

    /// Reserve type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub reserve_type: Option<ReserveType>,

    /// Reserve amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Maximum/cap amount in cents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<i64>,

    /// Percentage of transactions to reserve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,

    /// Currency code (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Number of days to hold reserves
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_days: Option<i32>,

    /// Reserve name/label
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

    // ==================== ReserveStatus Tests ====================

    #[test]
    fn reserve_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&ReserveStatus::Active).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ReserveStatus::Released).unwrap(), "2");
        assert_eq!(serde_json::to_string(&ReserveStatus::PartiallyReleased).unwrap(), "3");
        assert_eq!(serde_json::to_string(&ReserveStatus::Expired).unwrap(), "4");
    }

    #[test]
    fn reserve_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<ReserveStatus>("1").unwrap(), ReserveStatus::Active);
        assert_eq!(serde_json::from_str::<ReserveStatus>("2").unwrap(), ReserveStatus::Released);
        assert_eq!(serde_json::from_str::<ReserveStatus>("3").unwrap(), ReserveStatus::PartiallyReleased);
        assert_eq!(serde_json::from_str::<ReserveStatus>("4").unwrap(), ReserveStatus::Expired);
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

    // ==================== ReserveType Tests ====================

    #[test]
    fn reserve_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&ReserveType::Rolling).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ReserveType::Fixed).unwrap(), "2");
        assert_eq!(serde_json::to_string(&ReserveType::Upfront).unwrap(), "3");
    }

    #[test]
    fn reserve_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<ReserveType>("1").unwrap(), ReserveType::Rolling);
        assert_eq!(serde_json::from_str::<ReserveType>("2").unwrap(), ReserveType::Fixed);
        assert_eq!(serde_json::from_str::<ReserveType>("3").unwrap(), ReserveType::Upfront);
    }

    #[test]
    fn reserve_type_default() {
        assert_eq!(ReserveType::default(), ReserveType::Rolling);
    }

    #[test]
    fn reserve_type_invalid_value() {
        assert!(serde_json::from_str::<ReserveType>("0").is_err());
        assert!(serde_json::from_str::<ReserveType>("99").is_err());
    }

    // ==================== Reserve Struct Tests ====================

    #[test]
    fn reserve_deserialize_full() {
        let json = r#"{
            "id": "t1_rsv_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "fund": "t1_fnd_12345678901234567890123",
            "login": "t1_log_12345678901234567890123",
            "status": 1,
            "type": 2,
            "amount": 50000,
            "cap": 100000,
            "percent": 10,
            "currency": "USD",
            "holdDays": 30,
            "releaseDate": "20241231",
            "name": "Risk Reserve",
            "description": "Rolling reserve for merchant",
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert_eq!(reserve.id.as_str(), "t1_rsv_12345678901234567890123");
        assert_eq!(reserve.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(reserve.merchant.unwrap().as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(reserve.fund.unwrap().as_str(), "t1_fnd_12345678901234567890123");
        assert_eq!(reserve.status, Some(ReserveStatus::Active));
        assert_eq!(reserve.reserve_type, Some(ReserveType::Fixed));
        assert_eq!(reserve.amount, Some(50000));
        assert_eq!(reserve.cap, Some(100000));
        assert_eq!(reserve.percent, Some(10));
        assert_eq!(reserve.currency, Some("USD".to_string()));
        assert_eq!(reserve.hold_days, Some(30));
        assert_eq!(reserve.release_date.as_ref().unwrap().as_str(), "20241231");
        assert_eq!(reserve.name, Some("Risk Reserve".to_string()));
        assert_eq!(reserve.description, Some("Rolling reserve for merchant".to_string()));
        assert!(!reserve.inactive);
        assert!(reserve.frozen);
    }

    #[test]
    fn reserve_deserialize_minimal() {
        let json = r#"{
            "id": "t1_rsv_12345678901234567890123"
        }"#;

        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert_eq!(reserve.id.as_str(), "t1_rsv_12345678901234567890123");
        assert!(reserve.entity.is_none());
        assert!(reserve.merchant.is_none());
        assert!(reserve.status.is_none());
        assert!(reserve.reserve_type.is_none());
        assert!(!reserve.inactive);
        assert!(!reserve.frozen);
    }

    #[test]
    fn reserve_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_rsv_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert!(!reserve.inactive);
        assert!(!reserve.frozen);
    }

    #[test]
    fn reserve_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_rsv_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert!(reserve.inactive);
        assert!(reserve.frozen);
    }

    #[test]
    fn reserve_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_rsv_12345678901234567890123"}"#;
        let reserve: Reserve = serde_json::from_str(json).unwrap();
        assert!(!reserve.inactive);
        assert!(!reserve.frozen);
    }

    // ==================== NewReserve Tests ====================

    #[test]
    fn new_reserve_serialize_full() {
        let new_reserve = NewReserve {
            entity: Some("t1_ent_12345678901234567890123".to_string()),
            merchant: Some("t1_mer_12345678901234567890123".to_string()),
            fund: Some("t1_fnd_12345678901234567890123".to_string()),
            reserve_type: Some(ReserveType::Fixed),
            amount: Some(50000),
            cap: Some(100000),
            percent: Some(10),
            currency: Some("USD".to_string()),
            hold_days: Some(30),
            name: Some("Risk Reserve".to_string()),
            description: Some("Rolling reserve for merchant".to_string()),
            custom: Some("custom data".to_string()),
            inactive: Some(false),
        };

        let json = serde_json::to_string(&new_reserve).unwrap();
        assert!(json.contains("\"entity\":\"t1_ent_12345678901234567890123\""));
        assert!(json.contains("\"merchant\":\"t1_mer_12345678901234567890123\""));
        assert!(json.contains("\"type\":2"));
        assert!(json.contains("\"amount\":50000"));
        assert!(json.contains("\"cap\":100000"));
        assert!(json.contains("\"percent\":10"));
        assert!(json.contains("\"holdDays\":30"));
        assert!(json.contains("\"inactive\":0"));
    }

    #[test]
    fn new_reserve_serialize_minimal() {
        let new_reserve = NewReserve {
            ..Default::default()
        };

        let json = serde_json::to_string(&new_reserve).unwrap();
        // Optional fields should be omitted
        assert!(!json.contains("\"entity\""));
        assert!(!json.contains("\"merchant\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"amount\""));
        assert!(!json.contains("\"inactive\""));
    }

    #[test]
    fn new_reserve_option_bool_to_int_true() {
        let new_reserve = NewReserve {
            inactive: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_reserve).unwrap();
        assert!(json.contains("\"inactive\":1"));
    }

    #[test]
    fn new_reserve_option_bool_to_int_false() {
        let new_reserve = NewReserve {
            inactive: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_reserve).unwrap();
        assert!(json.contains("\"inactive\":0"));
    }
}
