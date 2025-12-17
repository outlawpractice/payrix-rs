//! Webhook server implementation using axum.
//!
//! This module provides an HTTP server for receiving Payrix webhook callbacks.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use ipnet::IpNet;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use super::events::WebhookEvent;
use super::logging::WebhookLogger;

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for the webhook server.
#[derive(Clone)]
pub struct WebhookServerConfig {
    /// Allowed IP addresses/CIDRs.
    ///
    /// If empty, all IPs are allowed. If set, only requests from these
    /// IP ranges will be accepted.
    pub allowed_ips: Vec<IpNet>,

    /// Required authentication header name.
    ///
    /// If set along with `auth_header_value`, incoming requests must
    /// include this header with the specified value.
    pub auth_header_name: Option<String>,

    /// Required authentication header value.
    pub auth_header_value: Option<String>,

    /// Enable request logging to stdout.
    pub enable_logging: bool,

    /// Optional database/custom logger.
    pub db_logger: Option<Arc<dyn WebhookLogger>>,

    /// Channel buffer size for outgoing events.
    pub channel_buffer_size: usize,
}

impl std::fmt::Debug for WebhookServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookServerConfig")
            .field("allowed_ips", &self.allowed_ips)
            .field("auth_header_name", &self.auth_header_name)
            .field("auth_header_value", &"[REDACTED]")
            .field("enable_logging", &self.enable_logging)
            .field("db_logger", &self.db_logger.as_ref().map(|_| "[logger]"))
            .field("channel_buffer_size", &self.channel_buffer_size)
            .finish()
    }
}

impl Default for WebhookServerConfig {
    fn default() -> Self {
        Self {
            allowed_ips: Vec::new(),
            auth_header_name: None,
            auth_header_value: None,
            enable_logging: true,
            db_logger: None,
            channel_buffer_size: 1000,
        }
    }
}

impl WebhookServerConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set allowed IP ranges.
    ///
    /// # Example
    ///
    /// ```
    /// use payrix::webhooks::WebhookServerConfig;
    ///
    /// let config = WebhookServerConfig::new()
    ///     .with_allowed_ips(vec!["10.0.0.0/8".parse().unwrap()]);
    /// ```
    pub fn with_allowed_ips(mut self, ips: Vec<IpNet>) -> Self {
        self.allowed_ips = ips;
        self
    }

    /// Set authentication header requirement.
    ///
    /// # Example
    ///
    /// ```
    /// use payrix::webhooks::WebhookServerConfig;
    ///
    /// let config = WebhookServerConfig::new()
    ///     .with_auth_header("X-Webhook-Secret", "my-secret-value");
    /// ```
    pub fn with_auth_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.auth_header_name = Some(name.into());
        self.auth_header_value = Some(value.into());
        self
    }

    /// Set a custom logger.
    pub fn with_logger(mut self, logger: Arc<dyn WebhookLogger>) -> Self {
        self.db_logger = Some(logger);
        self
    }

    /// Enable or disable stdout logging.
    pub fn with_stdout_logging(mut self, enabled: bool) -> Self {
        self.enable_logging = enabled;
        self
    }

    /// Set the channel buffer size.
    pub fn with_channel_buffer(mut self, size: usize) -> Self {
        self.channel_buffer_size = size;
        self
    }
}

// =============================================================================
// Server State
// =============================================================================

/// Shared state for the webhook server.
#[derive(Clone)]
struct ServerState {
    config: WebhookServerConfig,
    event_sender: mpsc::Sender<WebhookEvent>,
}

// =============================================================================
// Webhook Server
// =============================================================================

/// An HTTP server for receiving Payrix webhooks.
///
/// The server provides:
/// - IP allowlist filtering
/// - Optional header-based authentication
/// - Event parsing and distribution via tokio channels
/// - Optional database logging
///
/// # Example
///
/// ```no_run
/// use payrix::webhooks::{WebhookServer, WebhookServerConfig};
/// use std::net::SocketAddr;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = WebhookServerConfig::new()
///     .with_auth_header("X-Webhook-Secret", "my-secret");
///
/// let (server, mut events) = WebhookServer::with_config(config);
///
/// // Spawn the event handler
/// tokio::spawn(async move {
///     while let Some(event) = events.recv().await {
///         println!("Received event: {:?}", event.event_type);
///     }
/// });
///
/// // Run the server
/// let addr: SocketAddr = "0.0.0.0:13847".parse()?;
/// server.run(addr).await?;
/// # Ok(())
/// # }
/// ```
pub struct WebhookServer {
    state: ServerState,
}

impl WebhookServer {
    /// Create a new webhook server with default configuration.
    ///
    /// Returns the server and a receiver for webhook events.
    pub fn new() -> (Self, mpsc::Receiver<WebhookEvent>) {
        Self::with_config(WebhookServerConfig::default())
    }

    /// Create a webhook server with custom configuration.
    ///
    /// Returns the server and a receiver for webhook events.
    pub fn with_config(config: WebhookServerConfig) -> (Self, mpsc::Receiver<WebhookEvent>) {
        let (sender, receiver) = mpsc::channel(config.channel_buffer_size);

        let state = ServerState {
            config,
            event_sender: sender,
        };

        (Self { state }, receiver)
    }

    /// Build the axum router for this server.
    ///
    /// Use this method to embed the webhook routes in an existing axum application.
    pub fn router(self) -> Router {
        let state = Arc::new(self.state);

        Router::new()
            .route("/webhooks/payrix", post(handle_webhook))
            .route("/health", get(health_check))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                security_middleware,
            ))
            .with_state(state)
    }

    /// Run the server on the specified address.
    ///
    /// This method blocks until the server is shut down.
    pub async fn run(self, addr: SocketAddr) -> Result<(), std::io::Error> {
        let router = self.router().into_make_service_with_connect_info::<SocketAddr>();

        info!("Starting webhook server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await
    }

    /// Run the server with graceful shutdown support.
    ///
    /// The server will shut down when the provided future completes.
    pub async fn run_with_shutdown<F>(
        self,
        addr: SocketAddr,
        shutdown_signal: F,
    ) -> Result<(), std::io::Error>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let router = self.router().into_make_service_with_connect_info::<SocketAddr>();

        info!("Starting webhook server on {} (with graceful shutdown)", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal)
            .await
    }
}

impl Default for WebhookServer {
    fn default() -> Self {
        Self::new().0
    }
}

// =============================================================================
// Request Handlers
// =============================================================================

/// Handle incoming webhook POST requests.
async fn handle_webhook(
    State(state): State<Arc<ServerState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let source_ip = addr.ip();

    // Extract event details from payload
    let event_type = payload
        .get("event")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let resource_type = payload
        .get("resourceType")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let resource_id = payload
        .get("resourceId")
        .or_else(|| payload.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    // Extract the actual resource data
    let data = payload
        .get("resource")
        .cloned()
        .unwrap_or_else(|| payload.clone());

    let event = WebhookEvent::new(event_type.clone(), resource_type, resource_id, data, source_ip);

    if state.config.enable_logging {
        info!(
            event_type = %event.event_type,
            resource_id = %event.resource_id,
            source_ip = %source_ip,
            "Received webhook event"
        );
    }

    // Log to database if configured
    if let Some(logger) = &state.config.db_logger {
        if let Err(e) = logger.log_received(&event).await {
            warn!("Failed to log webhook event: {}", e);
        }
    }

    // Send to event channel
    if let Err(e) = state.event_sender.send(event).await {
        warn!("Failed to send webhook event to channel: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Event processing failed");
    }

    (StatusCode::OK, "OK")
}

/// Health check endpoint.
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

// =============================================================================
// Security Middleware
// =============================================================================

/// Security middleware for IP allowlist and header authentication.
async fn security_middleware(
    State(state): State<Arc<ServerState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let source_ip = addr.ip();

    // Check IP allowlist
    if !state.config.allowed_ips.is_empty() {
        let allowed = state
            .config
            .allowed_ips
            .iter()
            .any(|net| net.contains(&source_ip));

        if !allowed {
            warn!(
                source_ip = %source_ip,
                "Webhook request from unauthorized IP"
            );
            return (StatusCode::FORBIDDEN, "IP not allowed").into_response();
        }
    }

    // Check authentication header
    if let (Some(header_name), Some(expected_value)) = (
        &state.config.auth_header_name,
        &state.config.auth_header_value,
    ) {
        let actual_value = request
            .headers()
            .get(header_name)
            .and_then(|v| v.to_str().ok());

        if actual_value != Some(expected_value.as_str()) {
            warn!(
                source_ip = %source_ip,
                header = %header_name,
                "Webhook request with invalid authentication"
            );
            return (StatusCode::UNAUTHORIZED, "Invalid authentication").into_response();
        }
    }

    debug!(source_ip = %source_ip, "Webhook request passed security checks");
    next.run(request).await
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = WebhookServerConfig::new()
            .with_auth_header("X-Secret", "value")
            .with_stdout_logging(false)
            .with_channel_buffer(500);

        assert_eq!(config.auth_header_name, Some("X-Secret".to_string()));
        assert_eq!(config.auth_header_value, Some("value".to_string()));
        assert!(!config.enable_logging);
        assert_eq!(config.channel_buffer_size, 500);
    }

    #[test]
    fn test_config_with_ip_allowlist() {
        let config = WebhookServerConfig::new().with_allowed_ips(vec![
            "10.0.0.0/8".parse().unwrap(),
            "192.168.0.0/16".parse().unwrap(),
        ]);

        assert_eq!(config.allowed_ips.len(), 2);
    }

    #[tokio::test]
    async fn test_server_creation() {
        let (_server, mut receiver) = WebhookServer::new();

        // Server should be created and receiver should be ready
        assert!(receiver.try_recv().is_err()); // No events yet
    }
}
