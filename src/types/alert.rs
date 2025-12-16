//! Alert types for the Payrix API.
//!
//! Alerts provide notifications for various events in the Payrix system.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, PayrixId};

/// A Payrix alert configuration.
///
/// Alerts define notification rules for various events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    /// Unique identifier (30 characters, e.g., "t1_alt_...")
    pub id: PayrixId,

    /// Entity ID that owns this alert (required)
    pub entity: PayrixId,

    /// Login ID that created this alert
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Alert name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Alert description
    #[serde(default)]
    pub description: Option<String>,

    /// Alert type
    #[serde(default, rename = "type")]
    pub alert_type: Option<i32>,

    /// Whether the alert is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub enabled: bool,

    /// Alert priority level
    #[serde(default)]
    pub priority: Option<i32>,

    /// Alert category
    #[serde(default)]
    pub category: Option<String>,

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

/// Request to create a new alert.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAlert {
    /// Entity ID (required)
    pub entity: String,

    /// Alert name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Alert description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Alert type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub alert_type: Option<i32>,

    /// Whether the alert is enabled
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub enabled: Option<bool>,

    /// Alert priority level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Alert category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Custom data field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

/// A Payrix alert action.
///
/// Alert actions define what happens when an alert is triggered.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AlertAction {
    /// Unique identifier (30 characters, e.g., "t1_ala_...")
    pub id: PayrixId,

    /// Alert ID this action belongs to (required)
    pub alert: PayrixId,

    /// Login ID that created this action
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Action type (e.g., email, webhook, SMS)
    #[serde(default, rename = "type")]
    pub action_type: Option<i32>,

    /// Action name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Action configuration/payload
    #[serde(default)]
    pub config: Option<String>,

    /// Target for the action (email address, URL, phone number)
    #[serde(default)]
    pub target: Option<String>,

    /// Subject line (for email actions)
    #[serde(default)]
    pub subject: Option<String>,

    /// Message body template
    #[serde(default)]
    pub message: Option<String>,

    /// Whether the action is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub enabled: bool,

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

/// Request to create a new alert action.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAlertAction {
    /// Alert ID (required)
    pub alert: String,

    /// Action type (e.g., email, webhook, SMS)
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub action_type: Option<i32>,

    /// Action name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Action configuration/payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,

    /// Target for the action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Subject line (for email actions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Message body template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Whether the action is enabled
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub enabled: Option<bool>,
}

/// A Payrix alert trigger.
///
/// Alert triggers define conditions that cause an alert to fire.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AlertTrigger {
    /// Unique identifier (30 characters, e.g., "t1_alt_...")
    pub id: PayrixId,

    /// Alert ID this trigger belongs to (required)
    pub alert: PayrixId,

    /// Login ID that created this trigger
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Trigger type
    #[serde(default, rename = "type")]
    pub trigger_type: Option<i32>,

    /// Trigger name/label
    #[serde(default)]
    pub name: Option<String>,

    /// Field to evaluate
    #[serde(default)]
    pub field: Option<String>,

    /// Comparison operator
    #[serde(default)]
    pub operator: Option<String>,

    /// Value to compare against
    #[serde(default)]
    pub value: Option<String>,

    /// Entity type to monitor
    #[serde(default)]
    pub entity_type: Option<String>,

    /// Trigger condition configuration
    #[serde(default)]
    pub config: Option<String>,

    /// Whether the trigger is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub enabled: bool,

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

/// Request to create a new alert trigger.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAlertTrigger {
    /// Alert ID (required)
    pub alert: String,

    /// Trigger type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub trigger_type: Option<i32>,

    /// Trigger name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to evaluate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// Comparison operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    /// Value to compare against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Entity type to monitor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,

    /// Trigger condition configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,

    /// Whether the trigger is enabled
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub enabled: Option<bool>,
}
