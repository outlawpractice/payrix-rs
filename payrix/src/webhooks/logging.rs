//! Webhook logging traits and implementations.
//!
//! This module provides a trait for logging webhook events and several
//! implementations for different backends.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::IpAddr;
use std::sync::Mutex;
use uuid::Uuid;

use super::events::WebhookEvent;
use crate::error::Result;

// =============================================================================
// Log Entry Types
// =============================================================================

/// Processing status for a webhook event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessingStatus {
    /// Event received but not yet processed.
    Received,
    /// Event is being processed.
    Processing,
    /// Event was successfully processed.
    Processed,
    /// Event processing failed.
    Failed,
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Received => write!(f, "received"),
            Self::Processing => write!(f, "processing"),
            Self::Processed => write!(f, "processed"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

/// A log entry for a webhook event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookLogEntry {
    /// Unique ID for this log entry.
    pub id: Uuid,

    /// When the event was received.
    pub received_at: DateTime<Utc>,

    /// Source IP address of the webhook request.
    pub source_ip: IpAddr,

    /// Event type (e.g., "chargeback.created").
    pub event_type: String,

    /// Resource type (e.g., "chargebacks").
    pub resource_type: String,

    /// Resource ID.
    pub resource_id: String,

    /// Full event payload.
    pub payload: serde_json::Value,

    /// Current processing status.
    pub processing_status: ProcessingStatus,

    /// Error message if processing failed.
    pub error_message: Option<String>,

    /// When processing completed.
    pub processed_at: Option<DateTime<Utc>>,
}

impl WebhookLogEntry {
    /// Create a new log entry from a webhook event.
    pub fn from_event(event: &WebhookEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            received_at: event.received_at,
            source_ip: event.source_ip,
            event_type: event.event_type.clone(),
            resource_type: event.resource_type.clone(),
            resource_id: event.resource_id.clone(),
            payload: event.data.clone(),
            processing_status: ProcessingStatus::Received,
            error_message: None,
            processed_at: None,
        }
    }

    /// Mark this entry as processing.
    pub fn mark_processing(&mut self) {
        self.processing_status = ProcessingStatus::Processing;
    }

    /// Mark this entry as successfully processed.
    pub fn mark_processed(&mut self) {
        self.processing_status = ProcessingStatus::Processed;
        self.processed_at = Some(Utc::now());
    }

    /// Mark this entry as failed with an error message.
    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.processing_status = ProcessingStatus::Failed;
        self.error_message = Some(error.into());
        self.processed_at = Some(Utc::now());
    }
}

// =============================================================================
// Filter Types
// =============================================================================

/// Filter criteria for querying webhook logs.
#[derive(Debug, Default, Clone)]
pub struct WebhookLogFilter {
    /// Filter by event type.
    pub event_type: Option<String>,

    /// Filter by resource ID.
    pub resource_id: Option<String>,

    /// Filter by processing status.
    pub status: Option<ProcessingStatus>,

    /// Filter by received time (after).
    pub received_after: Option<DateTime<Utc>>,

    /// Filter by received time (before).
    pub received_before: Option<DateTime<Utc>>,

    /// Maximum number of entries to return.
    pub limit: Option<usize>,
}

impl WebhookLogFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by event type.
    pub fn with_event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    /// Filter by resource ID.
    pub fn with_resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// Filter by processing status.
    pub fn with_status(mut self, status: ProcessingStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Filter by received time range.
    pub fn with_received_range(
        mut self,
        after: Option<DateTime<Utc>>,
        before: Option<DateTime<Utc>>,
    ) -> Self {
        self.received_after = after;
        self.received_before = before;
        self
    }

    /// Limit the number of results.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

// =============================================================================
// Logger Trait
// =============================================================================

/// Trait for webhook event logging backends.
///
/// Implement this trait to provide custom logging storage for webhook events.
#[async_trait]
pub trait WebhookLogger: Send + Sync {
    /// Log a received webhook event.
    async fn log_received(&self, event: &WebhookEvent) -> Result<Uuid>;

    /// Update the processing status of an event.
    async fn update_status(&self, id: Uuid, status: ProcessingStatus, error: Option<String>)
        -> Result<()>;

    /// Query logged events.
    async fn query(&self, filter: WebhookLogFilter) -> Result<Vec<WebhookLogEntry>>;

    /// Get a single log entry by ID.
    async fn get(&self, id: Uuid) -> Result<Option<WebhookLogEntry>>;
}

// =============================================================================
// In-Memory Logger (for testing)
// =============================================================================

/// An in-memory webhook logger for testing.
///
/// This logger stores events in memory and is useful for unit tests
/// and development. It is not suitable for production use.
pub struct InMemoryWebhookLogger {
    entries: Mutex<VecDeque<WebhookLogEntry>>,
    max_entries: usize,
}

impl InMemoryWebhookLogger {
    /// Create a new in-memory logger.
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    /// Create a new in-memory logger with a maximum capacity.
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: Mutex::new(VecDeque::with_capacity(max_entries.min(1000))),
            max_entries,
        }
    }

    /// Get the number of stored entries.
    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    /// Check if the logger is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all entries.
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl Default for InMemoryWebhookLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebhookLogger for InMemoryWebhookLogger {
    async fn log_received(&self, event: &WebhookEvent) -> Result<Uuid> {
        let entry = WebhookLogEntry::from_event(event);
        let id = entry.id;

        let mut entries = self.entries.lock().unwrap();

        // Remove oldest entries if at capacity
        while entries.len() >= self.max_entries {
            entries.pop_front();
        }

        entries.push_back(entry);
        Ok(id)
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: ProcessingStatus,
        error: Option<String>,
    ) -> Result<()> {
        let mut entries = self.entries.lock().unwrap();

        if let Some(entry) = entries.iter_mut().find(|e| e.id == id) {
            entry.processing_status = status;
            entry.error_message = error;
            if matches!(status, ProcessingStatus::Processed | ProcessingStatus::Failed) {
                entry.processed_at = Some(Utc::now());
            }
        }

        Ok(())
    }

    async fn query(&self, filter: WebhookLogFilter) -> Result<Vec<WebhookLogEntry>> {
        let entries = self.entries.lock().unwrap();

        let mut results: Vec<_> = entries
            .iter()
            .filter(|e| {
                if let Some(ref event_type) = filter.event_type {
                    if &e.event_type != event_type {
                        return false;
                    }
                }
                if let Some(ref resource_id) = filter.resource_id {
                    if &e.resource_id != resource_id {
                        return false;
                    }
                }
                if let Some(status) = filter.status {
                    if e.processing_status != status {
                        return false;
                    }
                }
                if let Some(after) = filter.received_after {
                    if e.received_at < after {
                        return false;
                    }
                }
                if let Some(before) = filter.received_before {
                    if e.received_at > before {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by received_at descending (newest first)
        results.sort_by(|a, b| b.received_at.cmp(&a.received_at));

        if let Some(limit) = filter.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn get(&self, id: Uuid) -> Result<Option<WebhookLogEntry>> {
        let entries = self.entries.lock().unwrap();
        Ok(entries.iter().find(|e| e.id == id).cloned())
    }
}

// =============================================================================
// Stdout Logger
// =============================================================================

/// A simple stdout logger for webhook events.
///
/// This logger prints events to stdout and is useful for development
/// and debugging. It does not store events.
pub struct StdoutWebhookLogger {
    /// Whether to include the full payload in output.
    pub include_payload: bool,
}

impl StdoutWebhookLogger {
    /// Create a new stdout logger.
    pub fn new() -> Self {
        Self {
            include_payload: false,
        }
    }

    /// Create a stdout logger that includes the full payload.
    pub fn with_payload() -> Self {
        Self {
            include_payload: true,
        }
    }
}

impl Default for StdoutWebhookLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebhookLogger for StdoutWebhookLogger {
    async fn log_received(&self, event: &WebhookEvent) -> Result<Uuid> {
        let id = Uuid::new_v4();

        if self.include_payload {
            println!(
                "[{}] Webhook received: {} {} {} from {} - {:?}",
                event.received_at,
                event.event_type,
                event.resource_type,
                event.resource_id,
                event.source_ip,
                event.data
            );
        } else {
            println!(
                "[{}] Webhook received: {} {} {} from {}",
                event.received_at,
                event.event_type,
                event.resource_type,
                event.resource_id,
                event.source_ip
            );
        }

        Ok(id)
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: ProcessingStatus,
        error: Option<String>,
    ) -> Result<()> {
        if let Some(err) = error {
            println!("[{}] Status update: {} -> {} (error: {})", Utc::now(), id, status, err);
        } else {
            println!("[{}] Status update: {} -> {}", Utc::now(), id, status);
        }
        Ok(())
    }

    async fn query(&self, _filter: WebhookLogFilter) -> Result<Vec<WebhookLogEntry>> {
        // Stdout logger doesn't store entries
        Ok(Vec::new())
    }

    async fn get(&self, _id: Uuid) -> Result<Option<WebhookLogEntry>> {
        // Stdout logger doesn't store entries
        Ok(None)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn create_test_event() -> WebhookEvent {
        WebhookEvent::new(
            "chargeback.created",
            "chargebacks",
            "t1_chb_123",
            serde_json::json!({"id": "t1_chb_123"}),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        )
    }

    #[tokio::test]
    async fn test_in_memory_logger_basic() {
        let logger = InMemoryWebhookLogger::new();
        let event = create_test_event();

        let id = logger.log_received(&event).await.unwrap();
        assert_eq!(logger.len(), 1);

        let entry = logger.get(id).await.unwrap().unwrap();
        assert_eq!(entry.event_type, "chargeback.created");
        assert_eq!(entry.processing_status, ProcessingStatus::Received);
    }

    #[tokio::test]
    async fn test_in_memory_logger_update_status() {
        let logger = InMemoryWebhookLogger::new();
        let event = create_test_event();

        let id = logger.log_received(&event).await.unwrap();

        logger
            .update_status(id, ProcessingStatus::Processed, None)
            .await
            .unwrap();

        let entry = logger.get(id).await.unwrap().unwrap();
        assert_eq!(entry.processing_status, ProcessingStatus::Processed);
        assert!(entry.processed_at.is_some());
    }

    #[tokio::test]
    async fn test_in_memory_logger_query() {
        let logger = InMemoryWebhookLogger::new();

        // Add multiple events
        for i in 0..5 {
            let event = WebhookEvent::new(
                format!("event.{}", i),
                "test",
                format!("id_{}", i),
                serde_json::json!({}),
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            );
            logger.log_received(&event).await.unwrap();
        }

        // Query all
        let all = logger.query(WebhookLogFilter::new()).await.unwrap();
        assert_eq!(all.len(), 5);

        // Query with filter
        let filtered = logger
            .query(WebhookLogFilter::new().with_event_type("event.2"))
            .await
            .unwrap();
        assert_eq!(filtered.len(), 1);

        // Query with limit
        let limited = logger
            .query(WebhookLogFilter::new().with_limit(2))
            .await
            .unwrap();
        assert_eq!(limited.len(), 2);
    }

    #[tokio::test]
    async fn test_in_memory_logger_capacity() {
        let logger = InMemoryWebhookLogger::with_capacity(3);

        // Add more events than capacity
        for i in 0..5 {
            let event = WebhookEvent::new(
                format!("event.{}", i),
                "test",
                format!("id_{}", i),
                serde_json::json!({}),
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            );
            logger.log_received(&event).await.unwrap();
        }

        // Should only have max_entries
        assert_eq!(logger.len(), 3);

        // Should have the newest entries
        let entries = logger.query(WebhookLogFilter::new()).await.unwrap();
        assert!(entries.iter().any(|e| e.event_type == "event.4"));
        assert!(entries.iter().any(|e| e.event_type == "event.3"));
        assert!(entries.iter().any(|e| e.event_type == "event.2"));
    }

    #[test]
    fn test_log_entry_from_event() {
        let event = create_test_event();
        let entry = WebhookLogEntry::from_event(&event);

        assert_eq!(entry.event_type, "chargeback.created");
        assert_eq!(entry.resource_id, "t1_chb_123");
        assert_eq!(entry.processing_status, ProcessingStatus::Received);
        assert!(entry.error_message.is_none());
    }

    #[test]
    fn test_log_entry_status_updates() {
        let event = create_test_event();
        let mut entry = WebhookLogEntry::from_event(&event);

        entry.mark_processing();
        assert_eq!(entry.processing_status, ProcessingStatus::Processing);

        entry.mark_processed();
        assert_eq!(entry.processing_status, ProcessingStatus::Processed);
        assert!(entry.processed_at.is_some());
    }

    #[test]
    fn test_log_entry_mark_failed() {
        let event = create_test_event();
        let mut entry = WebhookLogEntry::from_event(&event);

        entry.mark_failed("Test error");
        assert_eq!(entry.processing_status, ProcessingStatus::Failed);
        assert_eq!(entry.error_message, Some("Test error".to_string()));
        assert!(entry.processed_at.is_some());
    }

    #[test]
    fn test_filter_builder() {
        let filter = WebhookLogFilter::new()
            .with_event_type("chargeback.created")
            .with_status(ProcessingStatus::Failed)
            .with_limit(10);

        assert_eq!(filter.event_type, Some("chargeback.created".to_string()));
        assert_eq!(filter.status, Some(ProcessingStatus::Failed));
        assert_eq!(filter.limit, Some(10));
    }
}
