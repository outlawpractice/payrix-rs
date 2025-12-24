//! Webhook setup workflow for configuring Payrix alert notifications.
//!
//! This module provides high-level APIs for setting up webhook alerts in Payrix,
//! allowing you to receive real-time notifications for events like chargebacks,
//! transactions, and merchant status changes.
//!
//! # Overview
//!
//! Payrix webhooks are implemented using three resources:
//!
//! 1. **Alert** - The parent container for webhook configuration
//! 2. **AlertAction** - Defines where to send notifications (web endpoint)
//! 3. **AlertTrigger** - Defines which events trigger notifications
//!
//! This workflow handles all three resources automatically.
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::webhook_setup::{setup_webhooks, WebhookConfig, WebhookEventType};
//!
//! # async fn example() -> payrix::Result<()> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Configure webhooks for chargeback events
//! let config = WebhookConfig::new("https://api.example.com")
//!     .with_events(vec![
//!         WebhookEventType::ChargebackCreated,
//!         WebhookEventType::ChargebackOpened,
//!         WebhookEventType::ChargebackClosed,
//!         WebhookEventType::ChargebackWon,
//!         WebhookEventType::ChargebackLost,
//!     ])
//!     .with_auth("X-Webhook-Secret", "my-secret-value");
//!
//! let result = setup_webhooks(&client, config).await?;
//! println!("Created alert: {}", result.alert_id);
//! println!("Triggers created: {:?}", result.triggers_created);
//! # Ok(())
//! # }
//! ```
//!
//! # Security
//!
//! Payrix does not provide webhook signature verification (HMAC). To secure
//! your webhook endpoint:
//!
//! 1. **Header Authentication** - Use `with_auth()` to require a secret header
//! 2. **IP Allowlist** - Configure your firewall to only accept from Payrix IPs
//! 3. **HTTPS** - Always use HTTPS endpoints
//!
//! # Checking Existing Configuration
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::webhook_setup::get_webhook_status;
//!
//! # async fn example() -> payrix::Result<()> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! let status = get_webhook_status(&client).await?;
//! for alert in &status.alerts {
//!     println!("Alert: {} - {}", alert.name, alert.endpoint);
//!     for event in &alert.events {
//!         println!("  - {}", event);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use serde::Serialize;

use crate::entity::EntityType;
use crate::error::{Error, Result};
use crate::types::{Alert, AlertAction, AlertActionType, AlertTrigger};
use crate::PayrixClient;

// =============================================================================
// Event Types
// =============================================================================

/// Webhook event types that can be subscribed to.
///
/// These correspond to the `event` field on AlertTrigger resources.
///
/// # Example
///
/// ```
/// use payrix::workflows::webhook_setup::WebhookEventType;
///
/// let event = WebhookEventType::ChargebackCreated;
/// assert_eq!(event.as_event_str(), "chargeback.created");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebhookEventType {
    // ===== Generic Events =====
    /// Any resource created.
    Create,
    /// Any resource updated.
    Update,
    /// Any resource deleted.
    Delete,
    /// Ownership changed.
    Ownership,
    /// Batch operation.
    Batch,

    // ===== Account Events =====
    /// Account event (generic).
    Account,
    /// Account created.
    AccountCreated,
    /// Account updated.
    AccountUpdated,

    // ===== Chargeback/Dispute Events =====
    /// Chargeback event (generic).
    Chargeback,
    /// New chargeback created.
    ChargebackCreated,
    /// Chargeback reopened.
    ChargebackOpened,
    /// Chargeback closed.
    ChargebackClosed,
    /// Merchant won the dispute.
    ChargebackWon,
    /// Merchant lost the dispute.
    ChargebackLost,

    // ===== Transaction Events =====
    /// Transaction created.
    TransactionCreated,
    /// Transaction approved.
    TransactionApproved,
    /// Transaction failed.
    TransactionFailed,
    /// Transaction captured.
    TransactionCaptured,
    /// Transaction settled.
    TransactionSettled,
    /// Transaction returned.
    TransactionReturned,

    // ===== Merchant Events =====
    /// Merchant created.
    MerchantCreated,
    /// Merchant boarding in progress.
    MerchantBoarding,
    /// Merchant boarded successfully.
    MerchantBoarded,
    /// Merchant closed.
    MerchantClosed,
    /// Merchant boarding failed.
    MerchantFailed,
    /// Merchant held.
    MerchantHeld,

    // ===== Disbursement Events =====
    /// Disbursement requested.
    DisbursementRequested,
    /// Disbursement processing.
    DisbursementProcessing,
    /// Disbursement processed.
    DisbursementProcessed,
    /// Disbursement failed.
    DisbursementFailed,
    /// Disbursement denied.
    DisbursementDenied,
    /// Disbursement returned.
    DisbursementReturned,

    // ===== Other Events =====
    /// Payout event.
    Payout,
    /// Fee event.
    Fee,
}

impl WebhookEventType {
    /// Get the Payrix API event string for this event type.
    ///
    /// # Example
    ///
    /// ```
    /// use payrix::workflows::webhook_setup::WebhookEventType;
    ///
    /// assert_eq!(WebhookEventType::ChargebackCreated.as_event_str(), "chargeback.created");
    /// assert_eq!(WebhookEventType::TransactionApproved.as_event_str(), "txn.approved");
    /// ```
    pub fn as_event_str(&self) -> &'static str {
        match self {
            // Generic events
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Ownership => "ownership",
            Self::Batch => "batch",

            // Account events
            Self::Account => "account",
            Self::AccountCreated => "account.created",
            Self::AccountUpdated => "account.updated",

            // Chargeback events
            Self::Chargeback => "chargeback",
            Self::ChargebackCreated => "chargeback.created",
            Self::ChargebackOpened => "chargeback.opened",
            Self::ChargebackClosed => "chargeback.closed",
            Self::ChargebackWon => "chargeback.won",
            Self::ChargebackLost => "chargeback.lost",

            // Transaction events
            Self::TransactionCreated => "txn.created",
            Self::TransactionApproved => "txn.approved",
            Self::TransactionFailed => "txn.failed",
            Self::TransactionCaptured => "txn.captured",
            Self::TransactionSettled => "txn.settled",
            Self::TransactionReturned => "txn.returned",

            // Merchant events
            Self::MerchantCreated => "merchant.created",
            Self::MerchantBoarding => "merchant.boarding",
            Self::MerchantBoarded => "merchant.boarded",
            Self::MerchantClosed => "merchant.closed",
            Self::MerchantFailed => "merchant.failed",
            Self::MerchantHeld => "merchant.held",

            // Disbursement events
            Self::DisbursementRequested => "disbursement.requested",
            Self::DisbursementProcessing => "disbursement.processing",
            Self::DisbursementProcessed => "disbursement.processed",
            Self::DisbursementFailed => "disbursement.failed",
            Self::DisbursementDenied => "disbursement.denied",
            Self::DisbursementReturned => "disbursement.returned",

            // Other events
            Self::Payout => "payout",
            Self::Fee => "fee",
        }
    }

    /// Get all chargeback-related events.
    ///
    /// Returns the events for monitoring the full chargeback lifecycle.
    pub fn all_chargeback_events() -> Vec<Self> {
        vec![
            Self::ChargebackCreated,
            Self::ChargebackOpened,
            Self::ChargebackClosed,
            Self::ChargebackWon,
            Self::ChargebackLost,
        ]
    }

    /// Get all transaction-related events.
    pub fn all_transaction_events() -> Vec<Self> {
        vec![
            Self::TransactionCreated,
            Self::TransactionApproved,
            Self::TransactionFailed,
            Self::TransactionCaptured,
            Self::TransactionSettled,
            Self::TransactionReturned,
        ]
    }

    /// Get all merchant-related events.
    pub fn all_merchant_events() -> Vec<Self> {
        vec![
            Self::MerchantCreated,
            Self::MerchantBoarding,
            Self::MerchantBoarded,
            Self::MerchantClosed,
            Self::MerchantFailed,
            Self::MerchantHeld,
        ]
    }

    /// Get all disbursement-related events.
    pub fn all_disbursement_events() -> Vec<Self> {
        vec![
            Self::DisbursementRequested,
            Self::DisbursementProcessing,
            Self::DisbursementProcessed,
            Self::DisbursementFailed,
            Self::DisbursementDenied,
            Self::DisbursementReturned,
        ]
    }
}

impl std::fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_event_str())
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for setting up webhooks.
///
/// Use the builder pattern to configure your webhook endpoint and events.
///
/// # Example
///
/// ```
/// use payrix::workflows::webhook_setup::{WebhookConfig, WebhookEventType};
///
/// let config = WebhookConfig::new("https://api.example.com")
///     .with_path("/webhooks/payrix")
///     .with_events(vec![
///         WebhookEventType::ChargebackCreated,
///         WebhookEventType::TransactionApproved,
///     ])
///     .with_auth("X-Webhook-Secret", "my-secret");
/// ```
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    /// Base URL for the webhook endpoint (e.g., "https://api.example.com").
    pub base_url: String,

    /// Path for the webhook endpoint (default: "/webhooks/payrix").
    pub webhook_path: String,

    /// Authentication header name (optional).
    pub header_name: Option<String>,

    /// Authentication header value (optional).
    pub header_value: Option<String>,

    /// Events to subscribe to.
    pub events: Vec<WebhookEventType>,

    /// Alert name (optional, will be generated if not provided).
    pub alert_name: Option<String>,

    /// Alert description (optional).
    pub alert_description: Option<String>,

    /// Number of retries for failed webhook deliveries.
    pub retries: Option<i32>,
}

impl WebhookConfig {
    /// Create a new webhook configuration with the base URL.
    ///
    /// # Example
    ///
    /// ```
    /// use payrix::workflows::webhook_setup::WebhookConfig;
    ///
    /// let config = WebhookConfig::new("https://api.example.com");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            webhook_path: "/webhooks/payrix".to_string(),
            header_name: None,
            header_value: None,
            events: Vec::new(),
            alert_name: None,
            alert_description: None,
            retries: None,
        }
    }

    /// Set the webhook path (default: "/webhooks/payrix").
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.webhook_path = path.into();
        self
    }

    /// Set the events to subscribe to.
    pub fn with_events(mut self, events: Vec<WebhookEventType>) -> Self {
        self.events = events;
        self
    }

    /// Add all chargeback events.
    pub fn with_all_chargeback_events(mut self) -> Self {
        self.events.extend(WebhookEventType::all_chargeback_events());
        self
    }

    /// Add all transaction events.
    pub fn with_all_transaction_events(mut self) -> Self {
        self.events.extend(WebhookEventType::all_transaction_events());
        self
    }

    /// Add all merchant events.
    pub fn with_all_merchant_events(mut self) -> Self {
        self.events.extend(WebhookEventType::all_merchant_events());
        self
    }

    /// Set authentication header (recommended for security).
    ///
    /// When set, all webhook requests from Payrix will include this header,
    /// allowing you to verify the request is authentic.
    pub fn with_auth(mut self, header_name: impl Into<String>, header_value: impl Into<String>) -> Self {
        self.header_name = Some(header_name.into());
        self.header_value = Some(header_value.into());
        self
    }

    /// Set the alert name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.alert_name = Some(name.into());
        self
    }

    /// Set the alert description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.alert_description = Some(description.into());
        self
    }

    /// Set the number of retries for failed deliveries.
    pub fn with_retries(mut self, retries: i32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Get the full webhook URL.
    pub fn webhook_url(&self) -> String {
        let base = self.base_url.trim_end_matches('/');
        let path = if self.webhook_path.starts_with('/') {
            self.webhook_path.clone()
        } else {
            format!("/{}", self.webhook_path)
        };
        format!("{}{}", base, path)
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<()> {
        if self.base_url.is_empty() {
            return Err(Error::Validation("base_url is required".to_string()));
        }

        if !self.base_url.starts_with("https://") && !self.base_url.starts_with("http://") {
            return Err(Error::Validation(
                "base_url must start with http:// or https://".to_string(),
            ));
        }

        if self.events.is_empty() {
            return Err(Error::Validation(
                "at least one event type is required".to_string(),
            ));
        }

        // Check for header auth consistency
        if self.header_name.is_some() != self.header_value.is_some() {
            return Err(Error::Validation(
                "both header_name and header_value must be set together".to_string(),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Setup Result
// =============================================================================

/// Result of setting up webhooks.
#[derive(Debug, Clone)]
pub struct WebhookSetupResult {
    /// The ID of the created/updated Alert.
    pub alert_id: String,

    /// The ID of the created AlertAction.
    pub action_id: String,

    /// The events that were configured.
    pub triggers_created: Vec<String>,

    /// Whether an existing configuration was updated.
    pub was_updated: bool,
}

// =============================================================================
// Status Types
// =============================================================================

/// Information about a configured webhook alert.
#[derive(Debug, Clone)]
pub struct WebhookAlertInfo {
    /// Alert ID.
    pub id: String,

    /// Alert name.
    pub name: String,

    /// Webhook endpoint URL.
    pub endpoint: String,

    /// Configured events.
    pub events: Vec<String>,

    /// Whether the alert is active.
    pub is_active: bool,

    /// Header name for authentication (if configured).
    pub auth_header: Option<String>,
}

/// Current webhook configuration status.
#[derive(Debug, Clone)]
pub struct WebhookStatus {
    /// Configured webhook alerts.
    pub alerts: Vec<WebhookAlertInfo>,
}

impl WebhookStatus {
    /// Check if any webhooks are configured.
    pub fn is_configured(&self) -> bool {
        !self.alerts.is_empty()
    }

    /// Get all configured events across all alerts.
    pub fn all_events(&self) -> Vec<&str> {
        self.alerts
            .iter()
            .flat_map(|a| a.events.iter().map(String::as_str))
            .collect()
    }
}

// =============================================================================
// API Request Types
// =============================================================================

/// Request body for creating an Alert.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewAlert {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// Request body for creating an AlertAction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewAlertAction {
    alert: String,
    #[serde(rename = "type")]
    action_type: AlertActionType,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<i32>,
}

/// Request body for creating an AlertTrigger.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewAlertTrigger {
    alert: String,
    event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

// =============================================================================
// Main Functions
// =============================================================================

/// Set up webhook alerts in Payrix.
///
/// This function creates the necessary Alert, AlertAction, and AlertTrigger
/// resources to receive webhook notifications.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `config` - Webhook configuration
///
/// # Returns
///
/// A `WebhookSetupResult` with the IDs of created resources.
///
/// # Example
///
/// ```no_run
/// use payrix::{PayrixClient, Environment};
/// use payrix::workflows::webhook_setup::{setup_webhooks, WebhookConfig, WebhookEventType};
///
/// # async fn example() -> payrix::Result<()> {
/// let client = PayrixClient::new("api-key", Environment::Test)?;
///
/// let config = WebhookConfig::new("https://api.example.com")
///     .with_events(WebhookEventType::all_chargeback_events())
///     .with_auth("X-Webhook-Secret", "my-secret");
///
/// let result = setup_webhooks(&client, config).await?;
/// println!("Alert ID: {}", result.alert_id);
/// # Ok(())
/// # }
/// ```
pub async fn setup_webhooks(client: &PayrixClient, config: WebhookConfig) -> Result<WebhookSetupResult> {
    // Validate configuration
    config.validate()?;

    // Deduplicate events
    let mut events: Vec<WebhookEventType> = config.events.clone();
    events.sort_by_key(|e| e.as_event_str());
    events.dedup_by_key(|e| e.as_event_str());

    // Generate alert name if not provided
    let alert_name = config.alert_name.clone().unwrap_or_else(|| {
        format!("Webhook Alert - {}", chrono::Utc::now().format("%Y-%m-%d"))
    });

    // Step 1: Create the Alert
    let new_alert = NewAlert {
        name: alert_name,
        description: config.alert_description.clone(),
    };

    let alert: Alert = client.create(EntityType::Alerts, &new_alert).await?;
    let alert_id = alert.id.to_string();

    // Step 2: Create the AlertAction (web type)
    let new_action = NewAlertAction {
        alert: alert_id.clone(),
        action_type: AlertActionType::Web,
        value: config.webhook_url(),
        header_name: config.header_name.clone(),
        header_value: config.header_value.clone(),
        retries: config.retries,
    };

    let action: AlertAction = client.create(EntityType::AlertActions, &new_action).await?;
    let action_id = action.id.to_string();

    // Step 3: Create AlertTriggers for each event
    let mut triggers_created = Vec::new();
    for event in &events {
        let event_str = event.as_event_str();
        let new_trigger = NewAlertTrigger {
            alert: alert_id.clone(),
            event: event_str.to_string(),
            name: Some(format!("{} trigger", event_str)),
        };

        let _trigger: AlertTrigger = client.create(EntityType::AlertTriggers, &new_trigger).await?;
        triggers_created.push(event_str.to_string());
    }

    Ok(WebhookSetupResult {
        alert_id,
        action_id,
        triggers_created,
        was_updated: false,
    })
}

/// Get the current webhook configuration status.
///
/// Queries Payrix for all configured webhook alerts and returns
/// information about each one.
///
/// # Arguments
///
/// * `client` - The Payrix client
///
/// # Returns
///
/// A `WebhookStatus` with information about all configured webhooks.
///
/// # Example
///
/// ```no_run
/// use payrix::{PayrixClient, Environment};
/// use payrix::workflows::webhook_setup::get_webhook_status;
///
/// # async fn example() -> payrix::Result<()> {
/// let client = PayrixClient::new("api-key", Environment::Test)?;
///
/// let status = get_webhook_status(&client).await?;
/// if status.is_configured() {
///     for alert in &status.alerts {
///         println!("Alert: {} -> {}", alert.name, alert.endpoint);
///     }
/// } else {
///     println!("No webhooks configured");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn get_webhook_status(client: &PayrixClient) -> Result<WebhookStatus> {
    use crate::SearchBuilder;

    // Search for all alerts
    let alerts: Vec<Alert> = client.search(EntityType::Alerts, &SearchBuilder::new().build()).await?;

    let mut alert_infos = Vec::new();

    for alert in alerts {
        let alert_id = alert.id.to_string();

        // Get actions for this alert (filter for web type)
        let action_search = SearchBuilder::new()
            .field("alert", &alert_id)
            .build();
        let actions: Vec<AlertAction> = client.search(EntityType::AlertActions, &action_search).await?;

        // Find web-type action
        let web_action = actions.iter().find(|a| {
            a.action_type == Some(AlertActionType::Web)
        });

        if let Some(action) = web_action {
            // Get triggers for this alert
            let trigger_search = SearchBuilder::new()
                .field("alert", &alert_id)
                .build();
            let triggers: Vec<AlertTrigger> = client.search(EntityType::AlertTriggers, &trigger_search).await?;

            let events: Vec<String> = triggers
                .iter()
                .filter_map(|t| t.event.clone())
                .collect();

            alert_infos.push(WebhookAlertInfo {
                id: alert_id,
                name: alert.name.unwrap_or_else(|| "Unnamed".to_string()),
                endpoint: action.value.clone().unwrap_or_default(),
                events,
                is_active: !alert.inactive,
                auth_header: action.header_name.clone(),
            });
        }
    }

    Ok(WebhookStatus { alerts: alert_infos })
}

/// Remove all webhook alerts.
///
/// This function removes all configured webhook alerts, actions, and triggers.
/// Use with caution in production.
///
/// # Arguments
///
/// * `client` - The Payrix client
///
/// # Returns
///
/// The number of alerts removed.
///
/// # Example
///
/// ```no_run
/// use payrix::{PayrixClient, Environment};
/// use payrix::workflows::webhook_setup::remove_webhooks;
///
/// # async fn example() -> payrix::Result<()> {
/// let client = PayrixClient::new("api-key", Environment::Test)?;
///
/// let count = remove_webhooks(&client).await?;
/// println!("Removed {} webhook alerts", count);
/// # Ok(())
/// # }
/// ```
pub async fn remove_webhooks(client: &PayrixClient) -> Result<usize> {
    use crate::SearchBuilder;

    // Get current status
    let status = get_webhook_status(client).await?;
    let count = status.alerts.len();

    for alert in &status.alerts {
        // Delete triggers first
        let trigger_search = SearchBuilder::new()
            .field("alert", &alert.id)
            .build();
        let triggers: Vec<AlertTrigger> = client.search(EntityType::AlertTriggers, &trigger_search).await?;

        for trigger in triggers {
            let _: AlertTrigger = client.remove(EntityType::AlertTriggers, trigger.id.as_str()).await?;
        }

        // Delete actions
        let action_search = SearchBuilder::new()
            .field("alert", &alert.id)
            .build();
        let actions: Vec<AlertAction> = client.search(EntityType::AlertActions, &action_search).await?;

        for action in actions {
            let _: AlertAction = client.remove(EntityType::AlertActions, action.id.as_str()).await?;
        }

        // Delete the alert
        let _: Alert = client.remove(EntityType::Alerts, &alert.id).await?;
    }

    Ok(count)
}

/// Remove a specific webhook alert by ID.
///
/// This function removes a single webhook alert and its associated
/// actions and triggers.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `alert_id` - The ID of the alert to remove
///
/// # Example
///
/// ```no_run
/// use payrix::{PayrixClient, Environment};
/// use payrix::workflows::webhook_setup::remove_webhook_by_id;
///
/// # async fn example() -> payrix::Result<()> {
/// let client = PayrixClient::new("api-key", Environment::Test)?;
///
/// remove_webhook_by_id(&client, "t1_alt_12345678901234567890123").await?;
/// # Ok(())
/// # }
/// ```
pub async fn remove_webhook_by_id(client: &PayrixClient, alert_id: &str) -> Result<()> {
    use crate::SearchBuilder;

    // Delete triggers first
    let trigger_search = SearchBuilder::new()
        .field("alert", alert_id)
        .build();
    let triggers: Vec<AlertTrigger> = client.search(EntityType::AlertTriggers, &trigger_search).await?;

    for trigger in triggers {
        let _: AlertTrigger = client.remove(EntityType::AlertTriggers, trigger.id.as_str()).await?;
    }

    // Delete actions
    let action_search = SearchBuilder::new()
        .field("alert", alert_id)
        .build();
    let actions: Vec<AlertAction> = client.search(EntityType::AlertActions, &action_search).await?;

    for action in actions {
        let _: AlertAction = client.remove(EntityType::AlertActions, action.id.as_str()).await?;
    }

    // Delete the alert
    let _: Alert = client.remove(EntityType::Alerts, alert_id).await?;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_event_type_as_str() {
        assert_eq!(WebhookEventType::ChargebackCreated.as_event_str(), "chargeback.created");
        assert_eq!(WebhookEventType::ChargebackOpened.as_event_str(), "chargeback.opened");
        assert_eq!(WebhookEventType::ChargebackClosed.as_event_str(), "chargeback.closed");
        assert_eq!(WebhookEventType::ChargebackWon.as_event_str(), "chargeback.won");
        assert_eq!(WebhookEventType::ChargebackLost.as_event_str(), "chargeback.lost");

        assert_eq!(WebhookEventType::TransactionCreated.as_event_str(), "txn.created");
        assert_eq!(WebhookEventType::TransactionApproved.as_event_str(), "txn.approved");

        assert_eq!(WebhookEventType::MerchantBoarded.as_event_str(), "merchant.boarded");
    }

    #[test]
    fn test_webhook_event_type_display() {
        assert_eq!(format!("{}", WebhookEventType::ChargebackCreated), "chargeback.created");
    }

    #[test]
    fn test_webhook_event_type_all_chargeback() {
        let events = WebhookEventType::all_chargeback_events();
        assert_eq!(events.len(), 5);
        assert!(events.contains(&WebhookEventType::ChargebackCreated));
        assert!(events.contains(&WebhookEventType::ChargebackWon));
        assert!(events.contains(&WebhookEventType::ChargebackLost));
    }

    #[test]
    fn test_webhook_config_builder() {
        let config = WebhookConfig::new("https://api.example.com")
            .with_path("/hooks")
            .with_events(vec![WebhookEventType::ChargebackCreated])
            .with_auth("X-Secret", "my-secret")
            .with_name("Test Alert")
            .with_retries(3);

        assert_eq!(config.base_url, "https://api.example.com");
        assert_eq!(config.webhook_path, "/hooks");
        assert_eq!(config.events.len(), 1);
        assert_eq!(config.header_name, Some("X-Secret".to_string()));
        assert_eq!(config.header_value, Some("my-secret".to_string()));
        assert_eq!(config.alert_name, Some("Test Alert".to_string()));
        assert_eq!(config.retries, Some(3));
    }

    #[test]
    fn test_webhook_config_url() {
        let config = WebhookConfig::new("https://api.example.com/");
        assert_eq!(config.webhook_url(), "https://api.example.com/webhooks/payrix");

        let config2 = WebhookConfig::new("https://api.example.com")
            .with_path("custom/path");
        assert_eq!(config2.webhook_url(), "https://api.example.com/custom/path");
    }

    #[test]
    fn test_webhook_config_validation() {
        // Empty base_url
        let config = WebhookConfig::new("")
            .with_events(vec![WebhookEventType::ChargebackCreated]);
        assert!(config.validate().is_err());

        // Invalid protocol
        let config = WebhookConfig::new("ftp://example.com")
            .with_events(vec![WebhookEventType::ChargebackCreated]);
        assert!(config.validate().is_err());

        // No events
        let config = WebhookConfig::new("https://example.com");
        assert!(config.validate().is_err());

        // Missing header value
        let config = WebhookConfig::new("https://example.com")
            .with_events(vec![WebhookEventType::ChargebackCreated]);
        let mut config = config;
        config.header_name = Some("X-Secret".to_string());
        assert!(config.validate().is_err());

        // Valid config
        let config = WebhookConfig::new("https://example.com")
            .with_events(vec![WebhookEventType::ChargebackCreated])
            .with_auth("X-Secret", "value");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_webhook_status_helpers() {
        let status = WebhookStatus {
            alerts: vec![
                WebhookAlertInfo {
                    id: "alert1".to_string(),
                    name: "Alert 1".to_string(),
                    endpoint: "https://example.com".to_string(),
                    events: vec!["chargeback.created".to_string()],
                    is_active: true,
                    auth_header: None,
                },
            ],
        };

        assert!(status.is_configured());
        assert_eq!(status.all_events(), vec!["chargeback.created"]);

        let empty_status = WebhookStatus { alerts: vec![] };
        assert!(!empty_status.is_configured());
    }

    #[test]
    fn test_webhook_config_with_all_events() {
        let config = WebhookConfig::new("https://example.com")
            .with_all_chargeback_events()
            .with_all_transaction_events();

        assert_eq!(config.events.len(), 11); // 5 chargeback + 6 transaction
    }
}
