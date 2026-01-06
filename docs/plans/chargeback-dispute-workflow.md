# Chargeback Dispute Handling Workflow API

## Overview

Create a high-level, workflow-based API for handling chargeback disputes using **Rust's typestate pattern** to enforce valid state transitions at compile time.

**User Requirements:**
- Typestate pattern for compile-time safety
- Evidence required for `represent()`
- Full documentation with examples and diagrams

## Chargeback Lifecycle (Extended)

The chargeback lifecycle can involve **multiple rounds** of representment:

```
                                    ┌──────────────────────────────────────┐
                                    │          (cycle can repeat)          │
                                    ▼                                      │
Retrieval → First → Representment → Pre-Arbitration → Second Chargeback ──┘
    │         │          │               │                    │
    │         │          │               │               ┌────┴────┐
    │    ┌────┴────┐     │          ┌────┴────┐          │         │
    ▼    ▼         ▼     ▼          ▼         ▼          ▼         ▼
 (wait) Represent Accept (await)  Accept   Request    Represent Accept
        + Evidence Liability      Liability Arbitration + Evidence Liability
                                                              │
                                                              ▼
                                                         Arbitration
                                                              │
                                                    ┌─────────┼─────────┐
                                                    ▼         ▼         ▼
                                                   Won       Lost     Split
```

### Key Insight: Chargebacks are Long-Running and Stateless

Chargebacks can take **weeks to months** to resolve. Our API must be:
- **Stateless** - Load fresh state from Payrix API on each operation
- **Event-driven** - Webhooks notify us when state changes
- **Refreshable** - Easy to reload a `TypedChargeback` from the latest API data

### Available Actions by Stage (Compile-Time Enforced)

| Stage | Available Methods | Transitions To |
|-------|-------------------|----------------|
| Retrieval | (none - await first chargeback) | First |
| First | `represent(evidence)`, `accept_liability()` | Representment, Terminal |
| Representment | (none - await decision) | PreArbitration, Won, Lost |
| Pre-Arbitration | `request_arbitration()`, `accept_liability()`, `represent(evidence)` | Arbitration, Terminal, Representment |
| SecondChargeback | `represent(evidence)`, `accept_liability()` | Representment, Terminal |
| Arbitration | (none - await decision) | Terminal |

### Handling Multiple Cycles

The `ChargebackCycle` enum already supports this:
- `first` → `representment` → `preArbitration` → can cycle back or escalate
- The typestate pattern handles this by:
  1. Loading fresh state from API
  2. Converting to appropriate `TypedChargeback<State>`
  3. Only exposing valid methods for that state

## Design: Typestate Pattern

### Type Hierarchy

```rust
// Zero-sized marker types for states
pub struct Retrieval;
pub struct First;
pub struct Representment;
pub struct PreArbitration;
pub struct Arbitration;
pub struct Terminal;

// Sealed trait for state markers
pub trait ChargebackState: private::Sealed {
    fn state_name() -> &'static str;
}

// The typed wrapper - methods only available in appropriate states
pub struct TypedChargeback<S: ChargebackState> {
    inner: Chargeback,
    _state: PhantomData<S>,
}

// Only First has represent() - COMPILE-TIME ENFORCED
impl TypedChargeback<First> {
    pub async fn represent(self, client: &PayrixClient, evidence: Evidence)
        -> Result<TypedChargeback<Representment>>;
    pub async fn accept_liability(self, client: &PayrixClient)
        -> Result<TypedChargeback<Terminal>>;
}

// Only PreArbitration has request_arbitration() - COMPILE-TIME ENFORCED
impl TypedChargeback<PreArbitration> {
    pub async fn request_arbitration(self, client: &PayrixClient)
        -> Result<TypedChargeback<Arbitration>>;
    pub async fn accept_liability(self, client: &PayrixClient)
        -> Result<TypedChargeback<Terminal>>;
}
```

### Runtime-to-Typestate Bridge

```rust
// Dispatch enum for handling runtime state from API
pub enum ActiveDispute {
    Retrieval(TypedChargeback<Retrieval>),
    First(TypedChargeback<First>),
    Representment(TypedChargeback<Representment>),
    PreArbitration(TypedChargeback<PreArbitration>),
    Arbitration(TypedChargeback<Arbitration>),
}

pub enum ChargebackDispute {
    Active(ActiveDispute),
    Terminal(TypedChargeback<Terminal>),
}

impl ChargebackDispute {
    // Entry point - converts runtime state to typestate
    pub async fn load(client: &PayrixClient, id: &str) -> Result<Self>;
    pub fn from_chargeback(chargeback: Chargeback) -> Self;
}
```

### Evidence Type (Required for represent)

```rust
pub struct Evidence {
    message: String,           // Required explanation
    documents: Vec<EvidenceDocument>,
}

pub struct EvidenceDocument {
    name: String,
    content: Vec<u8>,
    mime_type: String,
}

impl Evidence {
    pub fn new(message: impl Into<String>) -> Self;
    pub fn with_document(self, name: &str, content: Vec<u8>, mime_type: &str) -> Self;
}
```

### File Upload Helpers (HTML Form → Payrix)

**File:** `src/workflows/dispute_handling.rs` (additional helpers)

```rust
/// Convert an axum/multipart file upload to Evidence
#[cfg(feature = "webhooks")]
pub async fn evidence_from_multipart(
    field: axum::extract::multipart::Field,
) -> Result<EvidenceDocument>;

/// Convert bytes with filename to EvidenceDocument
pub fn evidence_from_bytes(
    filename: &str,
    content: Vec<u8>,
) -> Result<EvidenceDocument>;

/// Read file from disk into EvidenceDocument
pub fn evidence_from_path(path: impl AsRef<Path>) -> Result<EvidenceDocument>;

/// Parse a base64 data URL (e.g., "data:application/pdf;base64,JVBERi0...")
/// Commonly used in browser file uploads via JavaScript FileReader
pub fn evidence_from_base64_url(
    filename: &str,
    data_url: &str,
) -> Result<EvidenceDocument>;

/// Validate evidence meets Payrix requirements
impl Evidence {
    pub fn validate(&self) -> Result<()>;  // Size limits, format checks
}
```

**Base64 URL format:** `data:[<mediatype>][;base64],<data>`
- Example: `data:application/pdf;base64,JVBERi0xLjQKJ...`
- The helper extracts MIME type and decodes base64 content

**Payrix Evidence Requirements** (from [Dispute Response Options](https://resource.payrix.com/docs/dispute-response-options)):
- Max 8 documents per representment
- Max 1 MB per document
- Max 8 MB total combined
- Formats: TIFF/TIF, PDF (primary); PNG, JPG, GIF (also accepted)
- Must submit 5 business days before due date
- Must include Chargeback ID and/or Issuer Dispute Case Number

### Stateless API + Webhook Refresh Pattern

Since chargebacks are long-running:

```rust
// When webhook notifies of chargeback change:
async fn handle_chargeback_event(
    client: &PayrixClient,
    event: ChargebackEvent,
) -> Result<()> {
    // Always load fresh state from API
    let dispute = ChargebackDispute::load(&client, &event.chargeback_id).await?;

    // The loaded dispute is in the correct typestate
    // based on the current API state, not cached state
    match dispute {
        ChargebackDispute::Active(ActiveDispute::First(first)) => {
            // Can now call first.represent() or first.accept_liability()
        }
        ChargebackDispute::Active(ActiveDispute::PreArbitration(pre_arb)) => {
            // Can now call pre_arb.request_arbitration() etc.
        }
        ChargebackDispute::Terminal(_) => {
            // Dispute is closed, no actions available
        }
        _ => {}
    }
    Ok(())
}
```

**Key Methods for Refresh:**
```rust
impl ChargebackDispute {
    /// Load fresh state from API (primary entry point)
    pub async fn load(client: &PayrixClient, id: &str) -> Result<Self>;

    /// Convert existing Chargeback data (e.g., from webhook payload)
    pub fn from_chargeback(chargeback: Chargeback) -> Self;

    /// Refresh this dispute with latest API data
    pub async fn refresh(&self, client: &PayrixClient) -> Result<ChargebackDispute>;
}
```

### Example Usage

```rust
// Load chargeback - runtime state becomes compile-time type
let dispute = ChargebackDispute::load(&client, "t1_chb_...").await?;

match dispute {
    ChargebackDispute::Active(active) => match active {
        ActiveDispute::First(first) => {
            // represent() ONLY available here - won't compile elsewhere
            let evidence = Evidence::new("Customer received goods")
                .with_document("receipt.pdf", pdf_bytes, "application/pdf");
            let represented = first.represent(&client, evidence).await?;
        }
        ActiveDispute::PreArbitration(pre_arb) => {
            // request_arbitration() ONLY available here
            let arbitrating = pre_arb.request_arbitration(&client).await?;
        }
        _ => { /* other states */ }
    },
    ChargebackDispute::Terminal(terminal) => {
        println!("Dispute closed: {:?}", terminal.inner().status);
    }
}

// COMPILE ERROR: represent() not available in PreArbitration
// let pre_arb: TypedChargeback<PreArbitration> = ...;
// pre_arb.represent(...);  // ERROR!
```

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `src/workflows/dispute_handling.rs` | Create | Main workflow module (~400 lines) |
| `src/workflows/mod.rs` | Modify | Add `pub mod dispute_handling;` + re-exports |
| `src/lib.rs` | Modify | Re-export public types |

## Implementation Steps

### 1. Create `dispute_handling.rs`

**Section 1: State Marker Types** (~50 lines)
- Define zero-sized marker types: `Retrieval`, `First`, `Representment`, `PreArbitration`, `Arbitration`, `Terminal`
- Sealed trait `ChargebackState` with `state_name()` method
- Implement trait for all markers

**Section 2: TypedChargeback Wrapper** (~80 lines)
- Generic struct `TypedChargeback<S: ChargebackState>`
- Common methods: `inner()`, `id()`, `state_name()`, `into_inner()`
- `reply_deadline()`, `amount()`, `reason_code()` getters

**Section 3: Evidence Types** (~60 lines)
- `Evidence` struct with message and documents
- `EvidenceDocument` struct
- Builder methods: `new()`, `with_document()`
- Validation: file size limits (1MB per file, 8MB total, 8 files max)

**Section 4: State-Specific Methods** (~100 lines)
- `impl TypedChargeback<First>`: `represent()`, `accept_liability()`
- `impl TypedChargeback<PreArbitration>`: `request_arbitration()`, `accept_liability()`
- Each method: validates, calls API, returns new state type

**Section 5: Runtime Bridge** (~80 lines)
- `ActiveDispute` enum for actionable states
- `ChargebackDispute` enum (Active/Terminal)
- `load()` async function - entry point
- `from_chargeback()` - sync conversion from existing Chargeback

**Section 6: Convenience Functions** (~30 lines)
- `get_actionable_disputes(client, merchant_id)` - search helper
- `get_disputes_by_cycle(client, merchant_id, cycle)` - filtered search

**Section 7: Tests** (~100 lines)
- Unit tests for state marker traits
- Evidence validation tests
- State transition logic tests

### 2. Update `workflows/mod.rs`

```rust
pub mod dispute_handling;

pub use dispute_handling::{
    ChargebackDispute, ActiveDispute, TypedChargeback,
    Evidence, EvidenceDocument,
    Retrieval, First, Representment, PreArbitration, Arbitration, Terminal,
    ChargebackState,
};
```

### 3. Update `lib.rs`

Add re-exports for the most commonly used types.

## Critical Files Reference

- `src/types/chargeback.rs` - Existing types: `Chargeback`, `ChargebackCycle`, `ChargebackMessage`, `NewChargebackMessage`, etc.
- `src/workflows/merchant_onboarding.rs` - Pattern reference for workflow structure
- `src/client.rs` - Client methods: `get_one()`, `create()`, `search()`
- `src/entity.rs` - `EntityType::Chargebacks`, `EntityType::ChargebackMessages`

## Documentation Requirements

1. **Module-level docs** with ASCII workflow diagram
2. **Type docs** explaining the typestate pattern
3. **Method docs** with examples using `no_run`
4. **Doc tests** for evidence builder

---

# Part 2: Webhook Server & Alert Setup

## Overview

To receive real-time notifications when chargebacks change state, we need:

1. **Webhook Server** - HTTP server to receive Payrix webhook callbacks
2. **Event System** - Tokio channels to distribute events to consumers
3. **Alert Setup Workflow** - High-level API to configure Payrix alerts

## Architecture

```
┌─────────────────┐        ┌─────────────────┐        ┌──────────────────┐
│  Payrix API     │───────>│  WebhookServer  │───────>│  Event Channels  │
│  (sends POST)   │        │  (axum)         │        │  (tokio mpsc)    │
└─────────────────┘        └─────────────────┘        └────────┬─────────┘
                                                               │
                                    ┌──────────────────────────┼──────────────────────────┐
                                    │                          │                          │
                              ┌─────▼─────┐              ┌─────▼─────┐              ┌─────▼─────┐
                              │ Chargeback │              │Transaction│              │  Other    │
                              │ Handler    │              │ Handler   │              │ Handlers  │
                              └───────────┘              └───────────┘              └───────────┘
```

## Chargeback Events to Handle

| Event | Description | Use in Workflow |
|-------|-------------|-----------------|
| `chargeback.created` | New chargeback opened | Create `TypedChargeback<First>` |
| `chargeback.opened` | Chargeback re-opened | Refresh state |
| `chargeback.closed` | Chargeback closed | Transition to `Terminal` |
| `chargeback.won` | Merchant won | Transition to `Terminal` |
| `chargeback.lost` | Merchant lost | Transition to `Terminal` |

## Component 1: Webhook Server Module

**File:** `src/webhooks/server.rs`

```rust
use axum::{Router, Json, extract::State};
use tokio::sync::mpsc;

pub struct WebhookServer {
    router: Router,
    event_sender: mpsc::Sender<WebhookEvent>,
    config: WebhookServerConfig,
}

pub struct WebhookServerConfig {
    /// Allowed IP addresses/CIDRs (if empty, all IPs allowed)
    pub allowed_ips: Vec<IpNet>,
    /// Required header name for authentication (optional)
    pub auth_header_name: Option<String>,
    /// Required header value for authentication (optional)
    pub auth_header_value: Option<String>,
    /// Enable request logging to stdout
    pub enable_logging: bool,
    /// Optional database logger
    pub db_logger: Option<Arc<dyn WebhookLogger>>,
}

impl WebhookServer {
    /// Create with default config
    pub fn new() -> (Self, mpsc::Receiver<WebhookEvent>);

    /// Create with custom config
    pub fn with_config(config: WebhookServerConfig) -> (Self, mpsc::Receiver<WebhookEvent>);

    /// Run the server on the given address
    pub async fn run(self, addr: SocketAddr) -> Result<()>;

    /// Get the router for embedding in an existing axum app
    pub fn router(self) -> Router;
}

/// Webhook event payload from Payrix
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    pub event_type: String,           // e.g., "chargeback.created"
    pub resource_type: String,        // e.g., "chargebacks"
    pub resource_id: String,          // e.g., "t1_chb_..."
    pub data: serde_json::Value,      // Full payload
    pub received_at: chrono::DateTime<Utc>,
    pub source_ip: IpAddr,            // For audit logging
}
```

**Endpoints:**

| Path | Method | Purpose |
|------|--------|---------|
| `/webhooks/payrix` | POST | Receive all Payrix webhooks |
| `/health` | GET | Health check for load balancers |

### Webhook Security

**Note:** Payrix does not appear to provide webhook signature verification (HMAC). Security options:

1. **IP Allowlist** (Recommended)
   - Configure firewall or middleware to only accept from Payrix IPs
   - Use `WebhookServerConfig::allowed_ips`

2. **Custom Header Authentication**
   - Set a secret header value when configuring the AlertAction
   - Server validates the header on each request
   - Use `header_name` and `header_value` in AlertAction

3. **Network-Level Security**
   - Deploy behind a firewall/WAF
   - Use private networking if Payrix supports VPC peering

```rust
// Example: IP allowlist + header auth
let config = WebhookServerConfig {
    allowed_ips: vec![
        "203.0.113.0/24".parse()?,  // Payrix IP range (example)
    ],
    auth_header_name: Some("X-Webhook-Secret".to_string()),
    auth_header_value: Some("my-secret-value".to_string()),
    enable_logging: true,
    db_logger: None,
};
```

### IP Address Management

**Challenge:** Payrix webhook source IPs may change over time.

**Solution:**
1. **Contact Payrix Support** - Request official IP ranges for webhook sources
2. **Provide config file option** - Load allowed IPs from external config (JSON/YAML)
3. **DNS-based lookup** (if Payrix provides a hostname) - Resolve IPs periodically

```rust
pub struct WebhookServerConfig {
    /// Allowed IPs - loaded from config file or hardcoded
    pub allowed_ips: Vec<IpNet>,
    /// Optional: Path to IP allowlist file (reloaded periodically)
    pub ip_allowlist_file: Option<PathBuf>,
    /// How often to reload the IP allowlist file (default: 1 hour)
    pub ip_reload_interval: Duration,
}

/// Load IPs from a JSON file like:
/// { "allowed_ips": ["203.0.113.0/24", "198.51.100.0/24"] }
pub fn load_ip_allowlist(path: &Path) -> Result<Vec<IpNet>>;
```

**Recommendation:** Contact Payrix support to get their official webhook IP ranges and ask if they have a process for notifying customers of IP changes

### Webhook Logging

**File:** `src/webhooks/logging.rs`

```rust
/// Trait for webhook logging backends
#[async_trait]
pub trait WebhookLogger: Send + Sync {
    /// Log an incoming webhook event
    async fn log_event(&self, event: &WebhookLogEntry) -> Result<()>;

    /// Query logged events (for debugging/audit)
    async fn query_events(&self, filter: WebhookLogFilter) -> Result<Vec<WebhookLogEntry>>;
}

#[derive(Debug, Clone)]
pub struct WebhookLogEntry {
    pub id: Uuid,
    pub received_at: DateTime<Utc>,
    pub source_ip: IpAddr,
    pub event_type: String,
    pub resource_id: String,
    pub payload: serde_json::Value,
    pub processing_status: ProcessingStatus,
    pub error_message: Option<String>,
}

pub enum ProcessingStatus {
    Received,
    Processing,
    Processed,
    Failed,
}

/// SQLx-based logger (optional feature)
#[cfg(feature = "sqlx")]
pub struct SqlxWebhookLogger { ... }

/// In-memory logger for testing
pub struct InMemoryWebhookLogger { ... }

/// Stdout logger (always available)
pub struct StdoutWebhookLogger { ... }
```

### Database Schema for Webhook Logging

**File:** `migrations/001_webhook_logs.sql` (for SQLx)

```sql
-- Webhook event log table
CREATE TABLE IF NOT EXISTS payrix_webhook_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_ip INET NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    resource_id VARCHAR(50),
    payload JSONB NOT NULL,
    processing_status VARCHAR(20) NOT NULL DEFAULT 'received',
    error_message TEXT,
    processed_at TIMESTAMPTZ,

    -- Indexes for common queries
    CONSTRAINT valid_status CHECK (
        processing_status IN ('received', 'processing', 'processed', 'failed')
    )
);

-- Index for querying by event type and time
CREATE INDEX idx_webhook_logs_event_time
    ON payrix_webhook_logs(event_type, received_at DESC);

-- Index for querying by resource
CREATE INDEX idx_webhook_logs_resource
    ON payrix_webhook_logs(resource_type, resource_id);

-- Index for finding failed/pending events
CREATE INDEX idx_webhook_logs_status
    ON payrix_webhook_logs(processing_status)
    WHERE processing_status != 'processed';

-- Partitioning by month (optional, for high-volume)
-- CREATE TABLE payrix_webhook_logs_y2024m01 PARTITION OF payrix_webhook_logs
--     FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
```

**SQLite version:**
```sql
CREATE TABLE IF NOT EXISTS payrix_webhook_logs (
    id TEXT PRIMARY KEY,
    received_at TEXT NOT NULL,
    source_ip TEXT NOT NULL,
    event_type TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    payload TEXT NOT NULL,  -- JSON string
    processing_status TEXT NOT NULL DEFAULT 'received',
    error_message TEXT,
    processed_at TEXT
);

CREATE INDEX idx_webhook_logs_event_time ON payrix_webhook_logs(event_type, received_at);
CREATE INDEX idx_webhook_logs_resource ON payrix_webhook_logs(resource_type, resource_id);
```

## Component 2: Event Channel System

**File:** `src/webhooks/events.rs`

```rust
/// Typed chargeback event
pub enum ChargebackEvent {
    Created { chargeback_id: String, data: Chargeback },
    Opened { chargeback_id: String, data: Chargeback },
    Closed { chargeback_id: String, data: Chargeback },
    Won { chargeback_id: String, data: Chargeback },
    Lost { chargeback_id: String, data: Chargeback },
}

/// Subscribe to chargeback events
pub struct ChargebackEventSubscriber {
    receiver: mpsc::Receiver<ChargebackEvent>,
}

impl ChargebackEventSubscriber {
    pub async fn next(&mut self) -> Option<ChargebackEvent>;

    /// Convert to Stream for async iteration
    pub fn into_stream(self) -> impl Stream<Item = ChargebackEvent>;
}
```

## Component 3: Alert Setup Workflow

**File:** `src/workflows/webhook_setup.rs`

### High-Level API

```rust
/// Configuration for webhook setup
pub struct WebhookConfig {
    pub base_url: String,              // e.g., "https://payments.outlawpractice.com"
    pub webhook_path: String,          // e.g., "/webhooks/payrix" (default)
    pub header_name: Option<String>,   // Auth header name
    pub header_value: Option<String>,  // Auth header value (secret)
    pub events: Vec<WebhookEventType>, // Which events to subscribe to
}

/// Setup webhooks with a single call
pub async fn setup_webhooks(
    client: &PayrixClient,
    config: WebhookConfig,
) -> Result<WebhookSetupResult>;

/// Check existing webhook configuration
pub async fn get_webhook_status(
    client: &PayrixClient,
) -> Result<WebhookStatus>;

/// Remove all webhook alerts
pub async fn remove_webhooks(
    client: &PayrixClient,
) -> Result<()>;
```

### Supported Event Types (Complete List)

From [AlertTrigger documentation](https://resource.payrix.com/docs/setting-up-web-alerts-webhooks):

```rust
pub enum WebhookEventType {
    // ===== Generic Events =====
    Create,           // Any resource created
    Update,           // Any resource updated
    Delete,           // Any resource deleted
    Ownership,        // Ownership changed
    Batch,            // Batch operation

    // ===== Account Events =====
    Account,
    AccountCreated,
    AccountUpdated,

    // ===== Chargeback/Dispute Events =====
    Chargeback,
    ChargebackCreated,
    ChargebackOpened,
    ChargebackClosed,
    ChargebackWon,
    ChargebackLost,

    // ===== Transaction Events =====
    TransactionCreated,   // txn.created
    TransactionApproved,  // txn.approved
    TransactionFailed,    // txn.failed
    TransactionCaptured,  // txn.captured
    TransactionSettled,   // txn.settled
    TransactionReturned,  // txn.returned

    // ===== Merchant Events =====
    MerchantCreated,   // merchant.created
    MerchantBoarding,  // merchant.boarding
    MerchantBoarded,   // merchant.boarded
    MerchantClosed,    // merchant.closed
    MerchantFailed,    // merchant.failed
    MerchantHeld,      // merchant.held

    // ===== Disbursement Events =====
    DisbursementRequested,   // disbursement.requested
    DisbursementProcessing,  // disbursement.processing
    DisbursementProcessed,   // disbursement.processed
    DisbursementFailed,      // disbursement.failed
    DisbursementDenied,      // disbursement.denied
    DisbursementReturned,    // disbursement.returned

    // ===== Payout Events =====
    Payout,

    // ===== Fee Events =====
    Fee,

    // ===== Subscription Events =====
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionCancelled,

    // ===== Convenience Groups =====
    AllChargebacks,     // All chargeback.* events
    AllTransactions,    // All txn.* events
    AllMerchants,       // All merchant.* events
    AllDisbursements,   // All disbursement.* events
    All,                // ALL events
}

impl WebhookEventType {
    /// Convert to Payrix API event string
    pub fn as_event_str(&self) -> &'static str {
        match self {
            Self::ChargebackCreated => "chargeback.created",
            Self::ChargebackOpened => "chargeback.opened",
            Self::TransactionCreated => "txn.created",
            // ... etc
        }
    }

    /// Expand group types to individual events
    pub fn expand(&self) -> Vec<WebhookEventType> {
        match self {
            Self::AllChargebacks => vec![
                Self::ChargebackCreated,
                Self::ChargebackOpened,
                Self::ChargebackClosed,
                Self::ChargebackWon,
                Self::ChargebackLost,
            ],
            // ... etc
            _ => vec![self.clone()],
        }
    }
}
```

### Setup Logic

`setup_webhooks()` will:

1. **Check existing alerts** - `GET /alerts` with web type
2. **Compare with requested events** - Identify missing triggers
3. **Create missing configuration:**
   - Create `Alert` if none exists
   - Create `AlertAction` with `type: web`, `value: {base_url}{webhook_path}`
   - Create `AlertTrigger` for each event type
4. **Return status** - What was created/updated

### Example Usage

```rust
use payrix::workflows::webhook_setup::{setup_webhooks, WebhookConfig, WebhookEventType};

// Simple setup - all chargeback events
let config = WebhookConfig::new("https://payments.outlawpractice.com")
    .with_events(vec![
        WebhookEventType::ChargebackCreated,
        WebhookEventType::ChargebackClosed,
        WebhookEventType::ChargebackWon,
        WebhookEventType::ChargebackLost,
    ])
    .with_auth("X-Webhook-Secret", "my-secret-value");

let result = setup_webhooks(&client, config).await?;
println!("Created alert: {}", result.alert_id);
println!("Triggers: {:?}", result.triggers_created);
```

## Component 4: CLI Binary (Optional)

**File:** `src/bin/payrix-webhooks.rs`

```bash
# Run webhook server (default port: 13847 - avoids common ports)
payrix-webhooks serve --port 13847

# Specify custom port (required in production)
payrix-webhooks serve --port 443 --tls-cert cert.pem --tls-key key.pem

# Setup webhooks in Payrix
payrix-webhooks setup --base-url https://payments.example.com --events chargeback

# Check webhook status
payrix-webhooks status

# Remove webhooks
payrix-webhooks remove

# Enable logging to file
payrix-webhooks serve --port 13847 --log-file /var/log/payrix-webhooks.log
```

**CLI Design:**
```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the webhook server
    Serve {
        /// Port to listen on (default: 13847)
        #[arg(short, long, default_value = "13847")]
        port: u16,

        /// Bind address (default: 0.0.0.0)
        #[arg(short, long, default_value = "0.0.0.0")]
        bind: String,

        /// Log file path (optional, logs to stdout if not set)
        #[arg(long)]
        log_file: Option<PathBuf>,

        /// Allowed IP CIDRs (can specify multiple)
        #[arg(long)]
        allow_ip: Vec<String>,

        /// Auth header name
        #[arg(long)]
        auth_header: Option<String>,

        /// Auth header value
        #[arg(long)]
        auth_secret: Option<String>,
    },
    /// Setup webhook alerts in Payrix
    Setup { ... },
    /// Check webhook configuration status
    Status,
    /// Remove all webhook alerts
    Remove,
}
```

## Files to Create/Modify

### Part 1: Dispute Workflow (Core)

| File | Action | Purpose |
|------|--------|---------|
| `src/workflows/dispute_handling.rs` | Create | Typestate pattern dispute workflow |
| `src/workflows/mod.rs` | Modify | Add dispute_handling module |
| `src/lib.rs` | Modify | Re-export dispute types |

### Part 2: Webhooks (Feature-Gated)

| File | Action | Purpose |
|------|--------|---------|
| `src/webhooks/mod.rs` | Create | Module declarations |
| `src/webhooks/server.rs` | Create | axum webhook server |
| `src/webhooks/events.rs` | Create | Event types and channels |
| `src/webhooks/logging.rs` | Create | Logging trait + implementations |
| `src/workflows/webhook_setup.rs` | Create | Alert setup workflow |
| `src/bin/payrix-webhooks.rs` | Create | CLI binary |
| `Cargo.toml` | Modify | Add axum, clap, optional features |
| `src/lib.rs` | Modify | Export webhook module (feature-gated) |

## Feature Flags

```toml
[features]
default = []
webhooks = ["dep:axum", "dep:tower", "dep:tower-http", "dep:ipnet", "dep:uuid"]
webhook-cli = ["webhooks", "dep:clap", "tokio/full"]
webhook-sqlx-logging = ["webhooks", "sqlx"]
```

## Dependencies to Add

```toml
[dependencies]
# Webhook server (optional)
axum = { version = "0.7", optional = true }
tower = { version = "0.4", optional = true }
tower-http = { version = "0.5", features = ["trace"], optional = true }
ipnet = { version = "2.9", optional = true }  # IP CIDR parsing
uuid = { version = "1.0", features = ["v4"], optional = true }  # Log entry IDs

# CLI (optional)
clap = { version = "4.4", features = ["derive"], optional = true }

# Async trait for logging interface
async-trait = "0.1"

[dev-dependencies]
# For testing webhooks
axum-test = "15.0"
```

## Integration with Dispute Workflow

The webhook events can automatically update dispute state:

```rust
use payrix::webhooks::{WebhookServer, ChargebackEventSubscriber};
use payrix::workflows::dispute_handling::ChargebackDispute;

// Start webhook server
let (server, events) = WebhookServer::new();
tokio::spawn(server.run("0.0.0.0:8080".parse()?));

// Subscribe to chargeback events
let mut subscriber = ChargebackEventSubscriber::from(events);

while let Some(event) = subscriber.next().await {
    match event {
        ChargebackEvent::Created { chargeback_id, data } => {
            // New chargeback - create typed dispute
            let dispute = ChargebackDispute::from_chargeback(data);
            // Handle based on state...
        }
        ChargebackEvent::Won { chargeback_id, .. } => {
            println!("Won dispute {}", chargeback_id);
        }
        ChargebackEvent::Lost { chargeback_id, .. } => {
            println!("Lost dispute {}", chargeback_id);
        }
        _ => {}
    }
}
```

## Implementation Order

1. **Phase 1: Core Dispute Workflow** (existing plan)
   - Typestate pattern
   - Evidence handling
   - API actions

2. **Phase 2: Webhook Server**
   - axum server
   - Event parsing
   - Channel distribution

3. **Phase 3: Alert Setup Workflow**
   - `setup_webhooks()`
   - Existing alert detection
   - CRUD operations

4. **Phase 4: CLI Binary**
   - serve command
   - setup/status/remove commands

5. **Phase 5: Integration**
   - Connect webhooks to dispute state
   - Documentation and examples

---

# Part 3: Deployment & Operations

## NGINX Reverse Proxy Configuration

For production deployments, proxy the webhook server through NGINX with HTTPS:

```nginx
# /etc/nginx/sites-available/payrix-webhooks

upstream payrix_webhooks {
    server 127.0.0.1:13847;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name webhooks.yourcompany.com;

    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/webhooks.yourcompany.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/webhooks.yourcompany.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;

    # Webhook endpoint
    location /webhooks/payrix {
        proxy_pass http://payrix_webhooks;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;

        # Pass through the real client IP for allowlist checking
        # The webhook server should use X-Real-IP header
    }

    # Health check (optional - for load balancer)
    location /health {
        proxy_pass http://payrix_webhooks;
    }

    # Block all other paths
    location / {
        return 404;
    }
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name webhooks.yourcompany.com;
    return 301 https://$server_name$request_uri;
}
```

**Usage:**
```bash
# Enable the site
sudo ln -s /etc/nginx/sites-available/payrix-webhooks /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

---

# Part 4: Future Extension - Local Entity Cache

## Concept: Local Database Cache for Payrix Entities

**Problem:** Each API call to Payrix costs latency and may hit rate limits.

**Solution:** Maintain a local database that mirrors Payrix entities, kept in sync via webhooks.

### Benefits
- **Reduce API calls** - Query local DB instead of Payrix API
- **Faster queries** - Local database is much faster than API calls
- **Complex queries** - SQL JOINs, aggregations not possible via API
- **Offline resilience** - App continues working if Payrix is temporarily unavailable

### PCI DSS Considerations

**Safe to cache locally:**
- Transactions (tokenized, no card data)
- Chargebacks
- Merchants
- Customers
- Tokens (the token itself, not underlying card data)
- Alerts, Members, Accounts, etc.

**Payrix handles PCI compliance:**
- Card numbers are tokenized before being stored
- No raw PAN data comes through the API
- Webhooks contain the same tokenized data

### Architecture

```
┌─────────────────┐     webhook     ┌──────────────────┐
│   Payrix API    │ ───────────────> │  WebhookServer   │
└────────┬────────┘                  └────────┬─────────┘
         │                                    │
         │                                    ▼
         │                           ┌────────────────────┐
         │                           │   EntityCache      │
         │                           │   (SQLite/PG)      │
         │                           └────────┬───────────┘
         │                                    │
         │                                    ▼
         │                           ┌────────────────────┐
         │   initial load            │   Your App         │
         └──────────────────────────>│   (queries cache)  │
                                     └────────────────────┘
```

### Proposed API

```rust
pub struct PayrixEntityCache {
    db: Pool<Postgres>,  // or Sqlite
    client: PayrixClient,
}

impl PayrixEntityCache {
    /// Create and connect to cache database
    pub async fn new(db_url: &str, client: PayrixClient) -> Result<Self>;

    /// Initial load of all entities from Payrix
    pub async fn initial_sync(&self) -> Result<SyncStats>;

    /// Process a webhook event and update cache
    pub async fn process_webhook(&self, event: &WebhookEvent) -> Result<()>;

    /// Query chargebacks from local cache
    pub async fn get_chargeback(&self, id: &str) -> Result<Option<Chargeback>>;

    /// Query with filters (SQL-backed)
    pub async fn find_chargebacks(&self, filter: ChargebackFilter) -> Result<Vec<Chargeback>>;

    /// Get entity and fall back to API if not in cache
    pub async fn get_or_fetch<T>(&self, id: &str) -> Result<Option<T>>;
}
```

### Database Schema (for entity cache)

```sql
-- Generic pattern for all Payrix entities
CREATE TABLE payrix_chargebacks (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,            -- Full Payrix response
    created_at TIMESTAMPTZ,         -- Payrix created timestamp
    updated_at TIMESTAMPTZ,         -- Payrix modified timestamp
    synced_at TIMESTAMPTZ NOT NULL, -- When we last synced
    UNIQUE(id)
);

CREATE TABLE payrix_transactions (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    merchant_id VARCHAR(50),        -- Extracted for indexing
    customer_id VARCHAR(50),
    status INTEGER,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL
);

-- Indexes for common queries
CREATE INDEX idx_transactions_merchant ON payrix_transactions(merchant_id);
CREATE INDEX idx_transactions_status ON payrix_transactions(status);
```

### Implementation Scope

**This is a significant feature.** Recommend implementing as a separate phase after the core dispute workflow and webhook server are complete.

**Files to create (future):**
- `src/cache/mod.rs` - Module declarations
- `src/cache/entity_cache.rs` - Main cache implementation
- `src/cache/sync.rs` - Initial sync and incremental sync
- `migrations/002_entity_cache.sql` - Cache tables
