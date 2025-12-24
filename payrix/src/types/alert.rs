//! Alert types for the Payrix API.
//!
//! Alerts provide notifications for various events in the Payrix system.
//!
//! **OpenAPI schema:** `alertsResponse`, `alertActionsResponse`, `alertTriggersResponse`

use payrix_macros::PayrixEntity;
use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// ENUMS
// =============================================================================

/// Alert action type values per OpenAPI spec.
///
/// **OpenAPI schema:** `alertActionType`
///
/// Valid values:
/// - `email` - Deliver the Alert to an email address
/// - `web` - Deliver the Alert through a web site notification
/// - `app` - Deliver the Alert through a mobile application notification
/// - `sms` - Deliver the Alert through an SMS message to a mobile device
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertActionType {
    /// Deliver the Alert to an email address.
    #[default]
    Email,
    /// Deliver the Alert through a web site notification.
    Web,
    /// Deliver the Alert through a mobile application notification.
    App,
    /// Deliver the Alert through an SMS message to a mobile device.
    Sms,
}

/// Options for the web alert action format.
///
/// Determines how the alert data is sent to the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AlertActionOptions {
    /// JSON format.
    #[default]
    Json,
    /// XML format.
    Xml,
    /// SOAP format.
    Soap,
    /// Form-encoded format.
    Form,
}

// =============================================================================
// ALERT STRUCT
// =============================================================================

/// A Payrix alert configuration.
///
/// Alerts define notification rules for various events.
///
/// **OpenAPI schema:** `alertsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateAlert, update = UpdateAlert)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[payrix(readonly)]
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: alertsModelLogin)
    #[payrix(readonly)]
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The identifier of the Login that this Alert applies to.
    ///
    /// **OpenAPI type:** string (ref: alertsModelForlogin)
    #[payrix(create_only)]
    #[serde(default)]
    pub forlogin: Option<PayrixId>,

    /// The identifier (ID) of the team that this Alert relates to.
    ///
    /// The Alert is triggered based on the activity of this Team.
    ///
    /// **OpenAPI type:** string (ref: alertsModelTeam)
    #[payrix(create_only)]
    #[serde(default)]
    pub team: Option<PayrixId>,

    /// The identifier of the Division that this Alert applies to.
    ///
    /// **OpenAPI type:** string (ref: alertsModelDivision)
    #[payrix(create_only)]
    #[serde(default)]
    pub division: Option<PayrixId>,

    /// The partition for which this Alert applies.
    ///
    /// **OpenAPI type:** string (ref: alertsModelPartition)
    #[payrix(create_only)]
    #[serde(default)]
    pub partition: Option<PayrixId>,

    /// The name of this Alert.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Alert.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub description: Option<String>,

    /// Alert actions associated with this alert.
    ///
    /// **OpenAPI type:** array of alertActionsResponse
    #[serde(default)]
    pub alert_actions: Option<Vec<AlertAction>>,

    /// Alert triggers associated with this alert.
    ///
    /// **OpenAPI type:** array of alertTriggersResponse
    #[serde(default)]
    pub alert_triggers: Option<Vec<AlertTrigger>>,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// Helper methods for CreateAlert
impl CreateAlert {
    /// Create a new alert for a specific login.
    pub fn for_login(login_id: impl Into<PayrixId>, name: impl Into<String>) -> Self {
        Self {
            forlogin: Some(login_id.into()),
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Create a new alert for a specific team.
    pub fn for_team(team_id: impl Into<PayrixId>, name: impl Into<String>) -> Self {
        Self {
            team: Some(team_id.into()),
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

// =============================================================================
// ALERT ACTION STRUCT
// =============================================================================

/// A Payrix alert action.
///
/// Alert actions define what happens when an alert is triggered.
///
/// **OpenAPI schema:** `alertActionsResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateAlertAction, update = UpdateAlertAction)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AlertAction {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[payrix(readonly)]
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Alert resource that defines this alertAction.
    ///
    /// **OpenAPI type:** string (ref: alertActionsModelAlert)
    #[payrix(create_only)]
    #[serde(default)]
    pub alert: Option<PayrixId>,

    /// The medium to use to deliver this Alert.
    ///
    /// - `email` - Deliver the Alert to an email address
    /// - `web` - Deliver the Alert through a web site notification
    /// - `app` - Deliver the Alert through a mobile application notification
    /// - `sms` - Deliver the Alert through an SMS message to a mobile device
    ///
    /// **OpenAPI type:** string (ref: alertActionType)
    #[payrix(mutable)]
    #[serde(default, rename = "type")]
    pub action_type: Option<AlertActionType>,

    /// When the 'type' field of this resource is set to 'web', this field
    /// determines the format that the Alert data should be sent in.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub options: Option<String>,

    /// A value used to deliver the alert.
    ///
    /// The field should be set to an email address if the type is email,
    /// an endpoint if the type is web, etc.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub value: Option<String>,

    /// The request header name for authentication to the endpoint.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub header_name: Option<String>,

    /// The request header value for authentication to the endpoint.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub header_value: Option<String>,

    /// The number of times an alert should be resent in case of a failure.
    ///
    /// This field can only be set for web type alertActions.
    ///
    /// **OpenAPI type:** integer (int32)
    #[payrix(mutable)]
    #[serde(default)]
    pub retries: Option<i32>,

    /// Whether it was temporarily disabled for reaching the maximum number
    /// of failed attempts.
    ///
    /// - `0` - Not Temporarily Disabled
    /// - `1` - Temporarily Disabled
    ///
    /// **OpenAPI type:** integer (ref: MaxAttemptsTempDisabled)
    #[payrix(readonly)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub max_attempts_temp_disabled: bool,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// Helper methods for CreateAlertAction
impl CreateAlertAction {
    /// Create a new email alert action.
    pub fn email(alert_id: impl Into<PayrixId>, email: impl Into<String>) -> Self {
        Self {
            alert: Some(alert_id.into()),
            action_type: Some(AlertActionType::Email),
            value: Some(email.into()),
            ..Default::default()
        }
    }

    /// Create a new webhook alert action.
    pub fn webhook(alert_id: impl Into<PayrixId>, url: impl Into<String>) -> Self {
        Self {
            alert: Some(alert_id.into()),
            action_type: Some(AlertActionType::Web),
            value: Some(url.into()),
            options: Some("JSON".to_string()),
            retries: Some(3),
            ..Default::default()
        }
    }

    /// Create a new SMS alert action.
    pub fn sms(alert_id: impl Into<PayrixId>, phone: impl Into<String>) -> Self {
        Self {
            alert: Some(alert_id.into()),
            action_type: Some(AlertActionType::Sms),
            value: Some(phone.into()),
            ..Default::default()
        }
    }

    /// Set authentication headers for webhook.
    pub fn with_auth(mut self, header_name: impl Into<String>, header_value: impl Into<String>) -> Self {
        self.header_name = Some(header_name.into());
        self.header_value = Some(header_value.into());
        self
    }

    /// Set the response format for webhook.
    pub fn with_options(mut self, options: impl Into<String>) -> Self {
        self.options = Some(options.into());
        self
    }

    /// Set the number of retries for webhook.
    pub fn with_retries(mut self, retries: i32) -> Self {
        self.retries = Some(retries);
        self
    }
}

// =============================================================================
// ALERT TRIGGER STRUCT
// =============================================================================

/// A Payrix alert trigger.
///
/// Alert triggers define conditions that cause an alert to fire.
///
/// **OpenAPI schema:** `alertTriggersResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateAlertTrigger, update = UpdateAlertTrigger)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct AlertTrigger {
    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[payrix(readonly)]
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The identifier of the Alert resource that you want to invoke with this trigger.
    ///
    /// **OpenAPI type:** string (ref: alertTriggersModelAlert)
    #[payrix(create_only)]
    #[serde(default)]
    pub alert: Option<PayrixId>,

    /// The event type that triggers the associated Alert.
    ///
    /// **OpenAPI type:** string (ref: alertTriggerEvent)
    ///
    /// Valid values include: `create`, `update`, `delete`, `ownership`, `batch`,
    /// `account`, `account.created`, `account.updated`, `payout`, `fee`,
    /// `chargeback`, `chargeback.opened`, `chargeback.closed`, `chargeback.created`,
    /// `chargeback.lost`, `chargeback.won`, `txn.created`, `txn.approved`,
    /// `txn.failed`, `txn.captured`, `txn.settled`, `txn.returned`,
    /// `merchant.created`, `merchant.boarding`, `merchant.boarded`,
    /// `merchant.closed`, `merchant.failed`, `merchant.held`,
    /// `disbursement.requested`, `disbursement.processing`, `disbursement.processed`,
    /// `disbursement.failed`, `disbursement.denied`, `disbursement.returned`,
    /// and many more.
    #[payrix(mutable)]
    #[serde(default)]
    pub event: Option<String>,

    /// The resource type that this trigger applies to.
    ///
    /// **OpenAPI type:** integer (ref: Resource)
    ///
    /// Valid values include: 1 (apiKeys), 2 (contacts), 3 (customers),
    /// 4 (alertTriggers), 7 (alerts), 8 (logins), 9 (merchants), 10 (orgs),
    /// 13 (plans), 14 (subscriptions), 15 (tokens), 16 (txns), and many more.
    #[payrix(mutable)]
    #[serde(default)]
    pub resource: Option<i32>,

    /// The name of this alertTrigger.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub name: Option<String>,

    /// Description of Alert Triggers.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub description: Option<String>,

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// Helper methods for CreateAlertTrigger
impl CreateAlertTrigger {
    /// Create a new alert trigger.
    pub fn new(alert_id: impl Into<PayrixId>, event: impl Into<String>, resource: i32) -> Self {
        Self {
            alert: Some(alert_id.into()),
            event: Some(event.into()),
            resource: Some(resource),
            ..Default::default()
        }
    }

    /// Create a transaction event trigger.
    ///
    /// Common events: `txn.created`, `txn.approved`, `txn.failed`, `txn.settled`
    pub fn transaction(alert_id: impl Into<PayrixId>, event: impl Into<String>) -> Self {
        Self::new(alert_id, event, 16) // 16 = txns resource
    }

    /// Create a chargeback event trigger.
    ///
    /// Common events: `chargeback.created`, `chargeback.opened`, `chargeback.closed`,
    /// `chargeback.won`, `chargeback.lost`
    pub fn chargeback(alert_id: impl Into<PayrixId>, event: impl Into<String>) -> Self {
        Self::new(alert_id, event, 73) // 73 = chargebacks resource
    }

    /// Create a merchant event trigger.
    ///
    /// Common events: `merchant.created`, `merchant.boarded`, `merchant.held`
    pub fn merchant(alert_id: impl Into<PayrixId>, event: impl Into<String>) -> Self {
        Self::new(alert_id, event, 9) // 9 = merchants resource
    }

    /// Set the name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== AlertActionType Tests ====================

    #[test]
    fn alert_action_type_serialize_all_variants() {
        assert_eq!(
            serde_json::to_string(&AlertActionType::Email).unwrap(),
            "\"email\""
        );
        assert_eq!(
            serde_json::to_string(&AlertActionType::Web).unwrap(),
            "\"web\""
        );
        assert_eq!(
            serde_json::to_string(&AlertActionType::App).unwrap(),
            "\"app\""
        );
        assert_eq!(
            serde_json::to_string(&AlertActionType::Sms).unwrap(),
            "\"sms\""
        );
    }

    #[test]
    fn alert_action_type_deserialize_all_variants() {
        assert_eq!(
            serde_json::from_str::<AlertActionType>("\"email\"").unwrap(),
            AlertActionType::Email
        );
        assert_eq!(
            serde_json::from_str::<AlertActionType>("\"web\"").unwrap(),
            AlertActionType::Web
        );
        assert_eq!(
            serde_json::from_str::<AlertActionType>("\"app\"").unwrap(),
            AlertActionType::App
        );
        assert_eq!(
            serde_json::from_str::<AlertActionType>("\"sms\"").unwrap(),
            AlertActionType::Sms
        );
    }

    #[test]
    fn alert_action_type_default() {
        assert_eq!(AlertActionType::default(), AlertActionType::Email);
    }

    #[test]
    fn alert_action_type_invalid_value() {
        assert!(serde_json::from_str::<AlertActionType>("\"invalid\"").is_err());
        assert!(serde_json::from_str::<AlertActionType>("\"EMAIL\"").is_err());
    }

    // ==================== Alert Struct Tests ====================

    #[test]
    fn alert_deserialize_full() {
        let json = r#"{
            "id": "t1_alt_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "login": "t1_lgn_12345678901234567890125",
            "forlogin": "t1_lgn_12345678901234567890126",
            "team": "t1_tea_12345678901234567890123",
            "division": "t1_div_12345678901234567890123",
            "partition": "t1_par_12345678901234567890123",
            "name": "Transaction Alert",
            "description": "Alert for new transactions",
            "inactive": 0,
            "frozen": 1
        }"#;

        let alert: Alert = serde_json::from_str(json).unwrap();
        assert_eq!(alert.id.as_str(), "t1_alt_12345678901234567890123");
        assert_eq!(alert.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(alert.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(
            alert.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            alert.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            alert.login.as_ref().map(|l| l.as_str()),
            Some("t1_lgn_12345678901234567890125")
        );
        assert_eq!(
            alert.forlogin.as_ref().map(|f| f.as_str()),
            Some("t1_lgn_12345678901234567890126")
        );
        assert_eq!(
            alert.team.as_ref().map(|t| t.as_str()),
            Some("t1_tea_12345678901234567890123")
        );
        assert_eq!(
            alert.division.as_ref().map(|d| d.as_str()),
            Some("t1_div_12345678901234567890123")
        );
        assert_eq!(
            alert.partition.as_ref().map(|p| p.as_str()),
            Some("t1_par_12345678901234567890123")
        );
        assert_eq!(alert.name, Some("Transaction Alert".to_string()));
        assert_eq!(
            alert.description,
            Some("Alert for new transactions".to_string())
        );
        assert!(!alert.inactive);
        assert!(alert.frozen);
    }

    #[test]
    fn alert_deserialize_minimal() {
        let json = r#"{"id": "t1_alt_12345678901234567890123"}"#;

        let alert: Alert = serde_json::from_str(json).unwrap();
        assert_eq!(alert.id.as_str(), "t1_alt_12345678901234567890123");
        assert!(alert.created.is_none());
        assert!(alert.modified.is_none());
        assert!(alert.creator.is_none());
        assert!(alert.modifier.is_none());
        assert!(alert.login.is_none());
        assert!(alert.forlogin.is_none());
        assert!(alert.team.is_none());
        assert!(alert.division.is_none());
        assert!(alert.partition.is_none());
        assert!(alert.name.is_none());
        assert!(alert.description.is_none());
        assert!(!alert.inactive);
        assert!(!alert.frozen);
    }

    #[test]
    fn alert_bool_from_int() {
        let json =
            r#"{"id": "t1_alt_12345678901234567890123", "inactive": 1, "frozen": 0}"#;
        let alert: Alert = serde_json::from_str(json).unwrap();
        assert!(alert.inactive);
        assert!(!alert.frozen);
    }

    // ==================== AlertAction Struct Tests ====================

    #[test]
    fn alert_action_deserialize_full() {
        let json = r#"{
            "id": "t1_ala_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "type": "web",
            "options": "json",
            "value": "https://webhook.example.com/alerts",
            "headerName": "Authorization",
            "headerValue": "Bearer token123",
            "retries": 3,
            "maxAttemptsTempDisabled": 0,
            "alert": "t1_alt_12345678901234567890123",
            "inactive": 0,
            "frozen": 0
        }"#;

        let action: AlertAction = serde_json::from_str(json).unwrap();
        assert_eq!(action.id.as_str(), "t1_ala_12345678901234567890123");
        assert_eq!(action.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(
            action.modified,
            Some("2024-01-02 23:59:59.9999".to_string())
        );
        assert_eq!(
            action.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            action.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(action.action_type, Some(AlertActionType::Web));
        assert_eq!(action.options, Some("json".to_string()));
        assert_eq!(
            action.value,
            Some("https://webhook.example.com/alerts".to_string())
        );
        assert_eq!(action.header_name, Some("Authorization".to_string()));
        assert_eq!(action.header_value, Some("Bearer token123".to_string()));
        assert_eq!(action.retries, Some(3));
        assert!(!action.max_attempts_temp_disabled);
        assert_eq!(
            action.alert.as_ref().map(|a| a.as_str()),
            Some("t1_alt_12345678901234567890123")
        );
        assert!(!action.inactive);
        assert!(!action.frozen);
    }

    #[test]
    fn alert_action_deserialize_minimal() {
        let json = r#"{"id": "t1_ala_12345678901234567890123"}"#;

        let action: AlertAction = serde_json::from_str(json).unwrap();
        assert_eq!(action.id.as_str(), "t1_ala_12345678901234567890123");
        assert!(action.created.is_none());
        assert!(action.modified.is_none());
        assert!(action.creator.is_none());
        assert!(action.modifier.is_none());
        assert!(action.action_type.is_none());
        assert!(action.options.is_none());
        assert!(action.value.is_none());
        assert!(action.header_name.is_none());
        assert!(action.header_value.is_none());
        assert!(action.retries.is_none());
        assert!(!action.max_attempts_temp_disabled);
        assert!(action.alert.is_none());
        assert!(!action.inactive);
        assert!(!action.frozen);
    }

    #[test]
    fn alert_action_all_types() {
        let test_cases = vec![
            ("email", AlertActionType::Email),
            ("web", AlertActionType::Web),
            ("app", AlertActionType::App),
            ("sms", AlertActionType::Sms),
        ];

        for (type_str, expected_type) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ala_12345678901234567890123", "type": "{}"}}"#,
                type_str
            );
            let action: AlertAction = serde_json::from_str(&json).unwrap();
            assert_eq!(action.action_type, Some(expected_type));
        }
    }

    // ==================== AlertTrigger Struct Tests ====================

    #[test]
    fn alert_trigger_deserialize_full() {
        let json = r#"{
            "id": "t1_att_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "alert": "t1_alt_12345678901234567890123",
            "event": "txn.created",
            "resource": 16,
            "name": "Transaction Created Trigger",
            "description": "Triggers when a transaction is created",
            "inactive": 0,
            "frozen": 0
        }"#;

        let trigger: AlertTrigger = serde_json::from_str(json).unwrap();
        assert_eq!(trigger.id.as_str(), "t1_att_12345678901234567890123");
        assert_eq!(
            trigger.created,
            Some("2024-01-01 00:00:00.0000".to_string())
        );
        assert_eq!(
            trigger.modified,
            Some("2024-01-02 23:59:59.9999".to_string())
        );
        assert_eq!(
            trigger.creator.as_ref().map(|c| c.as_str()),
            Some("t1_lgn_12345678901234567890123")
        );
        assert_eq!(
            trigger.modifier.as_ref().map(|m| m.as_str()),
            Some("t1_lgn_12345678901234567890124")
        );
        assert_eq!(
            trigger.alert.as_ref().map(|a| a.as_str()),
            Some("t1_alt_12345678901234567890123")
        );
        assert_eq!(trigger.event, Some("txn.created".to_string()));
        assert_eq!(trigger.resource, Some(16));
        assert_eq!(
            trigger.name,
            Some("Transaction Created Trigger".to_string())
        );
        assert_eq!(
            trigger.description,
            Some("Triggers when a transaction is created".to_string())
        );
        assert!(!trigger.inactive);
        assert!(!trigger.frozen);
    }

    #[test]
    fn alert_trigger_deserialize_minimal() {
        let json = r#"{"id": "t1_att_12345678901234567890123"}"#;

        let trigger: AlertTrigger = serde_json::from_str(json).unwrap();
        assert_eq!(trigger.id.as_str(), "t1_att_12345678901234567890123");
        assert!(trigger.created.is_none());
        assert!(trigger.modified.is_none());
        assert!(trigger.creator.is_none());
        assert!(trigger.modifier.is_none());
        assert!(trigger.alert.is_none());
        assert!(trigger.event.is_none());
        assert!(trigger.resource.is_none());
        assert!(trigger.name.is_none());
        assert!(trigger.description.is_none());
        assert!(!trigger.inactive);
        assert!(!trigger.frozen);
    }

    #[test]
    fn alert_trigger_various_events() {
        let events = vec![
            "create",
            "update",
            "delete",
            "txn.created",
            "txn.approved",
            "txn.failed",
            "merchant.created",
            "merchant.boarded",
            "chargeback.opened",
            "disbursement.processed",
        ];

        for event in events {
            let json = format!(
                r#"{{"id": "t1_att_12345678901234567890123", "event": "{}"}}"#,
                event
            );
            let trigger: AlertTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(trigger.event, Some(event.to_string()));
        }
    }

    #[test]
    fn alert_trigger_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_att_12345678901234567890123",
            "alert": "t1_alt_12345678901234567890123",
            "event": "txn.created",
            "resource": 16
        }"#;

        let trigger: AlertTrigger = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&trigger).unwrap();
        let deserialized: AlertTrigger = serde_json::from_str(&serialized).unwrap();
        assert_eq!(trigger.id, deserialized.id);
        assert_eq!(trigger.alert, deserialized.alert);
        assert_eq!(trigger.event, deserialized.event);
        assert_eq!(trigger.resource, deserialized.resource);
    }

    // ==================== Create/Update Type Tests ====================

    #[test]
    fn create_alert_serializes_correctly() {
        let login_id: PayrixId = "t1_lgn_12345678901234567890123".parse().unwrap();
        let create = CreateAlert::for_login(login_id, "Test Alert")
            .with_description("A test alert");

        let json = serde_json::to_string(&create).unwrap();
        assert!(json.contains("\"forlogin\""));
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"description\""));
    }

    #[test]
    fn create_alert_action_serializes_correctly() {
        let alert_id: PayrixId = "t1_alt_12345678901234567890123".parse().unwrap();
        let create = CreateAlertAction::webhook(alert_id, "https://example.com")
            .with_auth("Authorization", "Bearer token");

        let json = serde_json::to_string(&create).unwrap();
        assert!(json.contains("\"alert\""));
        assert!(json.contains("\"type\""));
        assert!(json.contains("\"value\""));
        assert!(json.contains("\"headerName\""));
        assert!(json.contains("\"headerValue\""));
    }

    #[test]
    fn create_alert_trigger_serializes_correctly() {
        let alert_id: PayrixId = "t1_alt_12345678901234567890123".parse().unwrap();
        let create = CreateAlertTrigger::transaction(alert_id, "txn.created")
            .with_name("Transaction Trigger");

        let json = serde_json::to_string(&create).unwrap();
        assert!(json.contains("\"alert\""));
        assert!(json.contains("\"event\""));
        assert!(json.contains("\"resource\""));
        assert!(json.contains("\"name\""));
    }
}
