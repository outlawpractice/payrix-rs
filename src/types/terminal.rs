//! Terminal types for the Payrix API.
//!
//! Terminals represent physical payment terminal devices.
//!
//! **OpenAPI schema:** `terminalsResponse`
//!
//! This type is only available when the `terminal` feature is enabled.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId, TerminalCapability};

// =============================================================================
// ENUMS
// =============================================================================

/// Terminal environment values per OpenAPI spec.
///
/// **OpenAPI schema:** `terminalEnvironment`
///
/// Valid values:
/// - `1` - Retail
/// - `2` - Retail with tips
/// - `3` - Restaurant
/// - `4` - Lodging
/// - `5` - Bar
/// - `6` - Cash Advance
/// - `7` - Mail/Telephone Order
/// - `8` - Pay At The Pump
/// - `9` - Service Rest
/// - `10` - E-commerce
/// - `11` - Direct Marketing
/// - `12` - Fine Dining
/// - `13` - Gift Card Only
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TerminalEnvironment {
    /// Retail.
    Retail = 1,
    /// Retail with tips.
    RetailWithTips = 2,
    /// Restaurant.
    Restaurant = 3,
    /// Lodging.
    Lodging = 4,
    /// Bar.
    Bar = 5,
    /// Cash Advance.
    CashAdvance = 6,
    /// Mail/Telephone Order.
    MailTelephoneOrder = 7,
    /// Pay At The Pump.
    PayAtThePump = 8,
    /// Service Rest.
    ServiceRest = 9,
    /// E-commerce.
    Ecommerce = 10,
    /// Direct Marketing.
    DirectMarketing = 11,
    /// Fine Dining.
    FineDining = 12,
    /// Gift Card Only.
    GiftCardOnly = 13,
}

/// Terminal auto close values per OpenAPI spec.
///
/// **OpenAPI schema:** `AutoClose`
///
/// Valid values:
/// - `0` - Automatic
/// - `1` - Manual
/// - `2` - None
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AutoClose {
    /// Automatic.
    Automatic = 0,
    /// Manual.
    Manual = 1,
    /// None.
    None = 2,
}

// =============================================================================
// TERMINAL STRUCT
// =============================================================================

/// A Payrix terminal.
///
/// Terminals represent physical payment terminal devices.
///
/// **OpenAPI schema:** `terminalsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
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

    /// The identifier of the Merchant that owns this terminals resource.
    ///
    /// **OpenAPI type:** string (ref: terminalsModelMerchant)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The type of terminal.
    ///
    /// **OpenAPI type:** integer (ref: terminalType)
    #[serde(default, rename = "type")]
    pub terminal_type: Option<i32>,

    /// The credit card terminal's capabilities.
    ///
    /// **OpenAPI type:** integer (ref: Capability)
    #[serde(default)]
    pub capability: Option<TerminalCapability>,

    /// How is the terminal employed in the type of business using the terminal.
    ///
    /// **OpenAPI type:** integer (ref: terminalEnvironment)
    #[serde(default)]
    pub environment: Option<TerminalEnvironment>,

    /// If the terminal should be manually or automatically closed for the day.
    ///
    /// **OpenAPI type:** integer (ref: AutoClose)
    #[serde(default)]
    pub auto_close: Option<AutoClose>,

    /// The time when the terminal should be automatically closed for the day.
    ///
    /// This field is only required when AutoClose is set to automatic.
    /// Format: HHMM (e.g., 1145, 2200)
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub auto_close_time: Option<i32>,

    /// Whether or not the terminal has cloud services enabled.
    ///
    /// - `0` - Cloud Services Disabled
    /// - `1` - Cloud Services Enabled
    ///
    /// **OpenAPI type:** integer (ref: CloudEnabled)
    #[serde(default, with = "bool_from_int_default_false")]
    pub cloud_enabled: bool,

    /// The serial number of the terminal device.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub serial: Option<String>,

    /// The ID of the token record to associate with this terminal transaction request.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub token: Option<PayrixId>,

    /// The name of this Terminal.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// A description of the Terminal.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub description: Option<String>,

    /// The first line of the address associated with this Terminal's location.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// The second line of the address associated with this Terminal's location.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The name of the city in the address associated with this Terminal's location.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The state in the address associated with this Terminal's location.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The ZIP code in the address associated with this Terminal's location.
    ///
    /// This field is stored as a text string and must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The country in the address associated with the Terminal's location.
    ///
    /// Currently, this field only accepts the value 'USA'.
    ///
    /// **OpenAPI type:** string (ref: Country)
    #[serde(default)]
    pub country: Option<String>,

    /// The telephone number associated with the Terminal's location.
    ///
    /// Stored as a text string and must be between 5 and 15 characters long and numeric only.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The timezone for the address associated with the terminal's location.
    ///
    /// Valid values: est, cst, pst, mst, akst, hst, sst, chst, ast, pwt, mht
    ///
    /// **OpenAPI type:** string (ref: Timezone)
    #[serde(default)]
    pub timezone: Option<String>,

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
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_deserialize_full() {
        let json = r#"{
            "id": "t1_trm_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "merchant": "t1_mer_12345678901234567890123",
            "type": 1,
            "capability": 3,
            "environment": 1,
            "autoClose": 0,
            "autoCloseTime": 2200,
            "cloudEnabled": 1,
            "serial": "TERM123456",
            "name": "Main Terminal",
            "description": "Front desk terminal",
            "address1": "123 Main St",
            "city": "Austin",
            "state": "TX",
            "zip": "78701",
            "country": "USA",
            "timezone": "cst",
            "inactive": 0,
            "frozen": 0
        }"#;

        let terminal: Terminal = serde_json::from_str(json).unwrap();
        assert_eq!(terminal.id.as_str(), "t1_trm_12345678901234567890123");
        assert_eq!(terminal.capability, Some(TerminalCapability::IntegratedCircuitReader));
        assert_eq!(terminal.environment, Some(TerminalEnvironment::Retail));
        assert_eq!(terminal.auto_close, Some(AutoClose::Automatic));
        assert_eq!(terminal.auto_close_time, Some(2200));
        assert!(terminal.cloud_enabled);
        assert_eq!(terminal.serial, Some("TERM123456".to_string()));
        assert!(!terminal.inactive);
    }

    #[test]
    fn terminal_deserialize_minimal() {
        let json = r#"{"id": "t1_trm_12345678901234567890123"}"#;

        let terminal: Terminal = serde_json::from_str(json).unwrap();
        assert_eq!(terminal.id.as_str(), "t1_trm_12345678901234567890123");
        assert!(terminal.capability.is_none());
        assert!(!terminal.cloud_enabled);
        assert!(!terminal.inactive);
    }

    #[test]
    fn terminal_capability_values() {
        let test_cases = vec![
            (1, TerminalCapability::KeyEntryOnly),
            (2, TerminalCapability::MagneticStripe),
            (3, TerminalCapability::IntegratedCircuitReader),
            (4, TerminalCapability::ContactlessPayment),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_trm_12345678901234567890123", "capability": {}}}"#,
                val
            );
            let terminal: Terminal = serde_json::from_str(&json).unwrap();
            assert_eq!(terminal.capability, Some(expected));
        }
    }

    #[test]
    fn terminal_environment_values() {
        let test_cases = vec![
            (1, TerminalEnvironment::Retail),
            (3, TerminalEnvironment::Restaurant),
            (10, TerminalEnvironment::Ecommerce),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_trm_12345678901234567890123", "environment": {}}}"#,
                val
            );
            let terminal: Terminal = serde_json::from_str(&json).unwrap();
            assert_eq!(terminal.environment, Some(expected));
        }
    }

    #[test]
    fn terminal_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_trm_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "capability": 3,
            "environment": 1
        }"#;

        let terminal: Terminal = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&terminal).unwrap();
        let deserialized: Terminal = serde_json::from_str(&serialized).unwrap();
        assert_eq!(terminal.id, deserialized.id);
        assert_eq!(terminal.capability, deserialized.capability);
        assert_eq!(terminal.environment, deserialized.environment);
    }
}
