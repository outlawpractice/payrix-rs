//! Entity cache implementation.
//!
//! This module provides the main `EntityCache` struct for caching Payrix entities.

use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
#[cfg(feature = "webhooks")]
use tracing::{debug, warn};

use crate::entity::EntityType;
use crate::error::{Error, Result};
use crate::types::{Chargeback, Customer, Merchant, Token, Transaction};
use crate::PayrixClient;

#[cfg(feature = "webhooks")]
use crate::webhooks::WebhookEvent;

use super::schema::ensure_schema;

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for the entity cache.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Database connection URL.
    pub database_url: String,

    /// Maximum number of connections in the pool.
    pub max_connections: u32,

    /// Whether to create schema on startup.
    pub auto_create_schema: bool,

    /// Whether to sync on startup.
    pub sync_on_startup: bool,
}

impl CacheConfig {
    /// Create a new cache configuration.
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            max_connections: 10,
            auto_create_schema: true,
            sync_on_startup: false,
        }
    }

    /// Set the maximum number of connections.
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Enable or disable auto schema creation.
    pub fn with_auto_schema(mut self, enabled: bool) -> Self {
        self.auto_create_schema = enabled;
        self
    }

    /// Enable or disable sync on startup.
    pub fn with_sync_on_startup(mut self, enabled: bool) -> Self {
        self.sync_on_startup = enabled;
        self
    }
}

// =============================================================================
// Sync Statistics
// =============================================================================

/// Statistics from a sync operation.
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    /// Number of chargebacks synced.
    pub chargebacks: usize,

    /// Number of transactions synced.
    pub transactions: usize,

    /// Number of merchants synced.
    pub merchants: usize,

    /// Number of customers synced.
    pub customers: usize,

    /// Number of tokens synced.
    pub tokens: usize,

    /// Duration of the sync operation.
    pub duration: std::time::Duration,

    /// Any errors encountered (non-fatal).
    pub errors: Vec<String>,
}

impl SyncStats {
    /// Get the total number of entities synced.
    pub fn total(&self) -> usize {
        self.chargebacks + self.transactions + self.merchants + self.customers + self.tokens
    }
}

// =============================================================================
// Entity Cache
// =============================================================================

/// A local database cache for Payrix entities.
///
/// The cache provides fast local queries and can be kept in sync with Payrix
/// via webhooks or periodic syncs.
pub struct EntityCache {
    pool: PgPool,
    client: PayrixClient,
}

impl EntityCache {
    /// Create a new entity cache and connect to the database.
    ///
    /// This will optionally create the schema if `auto_create_schema` is enabled
    /// in the config (default: true).
    ///
    /// # Arguments
    ///
    /// * `database_url` - PostgreSQL connection URL
    /// * `client` - Payrix API client for syncing data
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payrix::{PayrixClient, Environment};
    /// use payrix::cache::EntityCache;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PayrixClient::new("api-key", Environment::Test)?;
    /// let cache = EntityCache::new(
    ///     "postgres://user:pass@localhost/payrix_cache",
    ///     client,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(database_url: &str, client: PayrixClient) -> Result<Self> {
        Self::with_config(CacheConfig::new(database_url), client).await
    }

    /// Create a new entity cache with custom configuration.
    pub async fn with_config(config: CacheConfig, client: PayrixClient) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await
            .map_err(|e| Error::Internal(format!("Failed to connect to database: {}", e)))?;

        if config.auto_create_schema {
            ensure_schema(&pool).await?;
        }

        let cache = Self { pool, client };

        if config.sync_on_startup {
            cache.initial_sync().await?;
        }

        Ok(cache)
    }

    /// Get the database connection pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the Payrix client.
    pub fn client(&self) -> &PayrixClient {
        &self.client
    }

    // =========================================================================
    // Chargeback Methods
    // =========================================================================

    /// Get a chargeback from the cache by ID.
    ///
    /// Returns `None` if the chargeback is not in the cache.
    pub async fn get_chargeback(&self, id: &str) -> Result<Option<Chargeback>> {
        let row = sqlx::query("SELECT data FROM payrix_chargebacks WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let chargeback: Chargeback = serde_json::from_value(data)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize chargeback: {}", e)))?;
                Ok(Some(chargeback))
            }
            None => Ok(None),
        }
    }

    /// Get a chargeback, fetching from API if not in cache.
    pub async fn get_or_fetch_chargeback(&self, id: &str) -> Result<Option<Chargeback>> {
        // Try cache first
        if let Some(chargeback) = self.get_chargeback(id).await? {
            return Ok(Some(chargeback));
        }

        // Fetch from API
        let chargeback: Option<Chargeback> = self.client.get_one(EntityType::Chargebacks, id).await?;

        // Store in cache if found
        if let Some(ref cb) = chargeback {
            self.upsert_chargeback(cb).await?;
        }

        Ok(chargeback)
    }

    /// Find chargebacks by merchant ID.
    pub async fn find_chargebacks_by_merchant(&self, merchant_id: &str) -> Result<Vec<Chargeback>> {
        let rows = sqlx::query("SELECT data FROM payrix_chargebacks WHERE merchant_id = $1 ORDER BY created_at DESC")
            .bind(merchant_id)
            .fetch_all(&self.pool)
            .await?;

        let mut chargebacks = Vec::with_capacity(rows.len());
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let chargeback: Chargeback = serde_json::from_value(data)
                .map_err(|e| Error::Internal(format!("Failed to deserialize chargeback: {}", e)))?;
            chargebacks.push(chargeback);
        }

        Ok(chargebacks)
    }

    /// Find chargebacks by transaction ID.
    pub async fn find_chargebacks_by_transaction(&self, txn_id: &str) -> Result<Vec<Chargeback>> {
        let rows = sqlx::query("SELECT data FROM payrix_chargebacks WHERE txn_id = $1 ORDER BY created_at DESC")
            .bind(txn_id)
            .fetch_all(&self.pool)
            .await?;

        let mut chargebacks = Vec::with_capacity(rows.len());
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let chargeback: Chargeback = serde_json::from_value(data)
                .map_err(|e| Error::Internal(format!("Failed to deserialize chargeback: {}", e)))?;
            chargebacks.push(chargeback);
        }

        Ok(chargebacks)
    }

    /// Upsert a chargeback into the cache.
    pub async fn upsert_chargeback(&self, chargeback: &Chargeback) -> Result<()> {
        let data = serde_json::to_value(chargeback)
            .map_err(|e| Error::Internal(format!("Failed to serialize chargeback: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO payrix_chargebacks (id, data, merchant_id, txn_id, cycle, status, total, reason_code, created_at, modified_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                merchant_id = EXCLUDED.merchant_id,
                txn_id = EXCLUDED.txn_id,
                cycle = EXCLUDED.cycle,
                status = EXCLUDED.status,
                total = EXCLUDED.total,
                reason_code = EXCLUDED.reason_code,
                modified_at = EXCLUDED.modified_at,
                synced_at = NOW()
            "#,
        )
        .bind(chargeback.id.as_str())
        .bind(&data)
        .bind(chargeback.merchant.as_ref().map(|m| m.as_str()))
        .bind(chargeback.txn.as_ref().map(|t| t.as_str()))
        .bind(chargeback.cycle.as_ref().map(|c| format!("{:?}", c)))
        .bind(chargeback.status.map(|s| s as i32))
        .bind(chargeback.total)
        .bind(chargeback.reason_code.as_deref())
        .bind(parse_payrix_datetime(chargeback.created.as_deref()))
        .bind(parse_payrix_datetime(chargeback.modified.as_deref()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Transaction Methods
    // =========================================================================

    /// Get a transaction from the cache by ID.
    pub async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>> {
        let row = sqlx::query("SELECT data FROM payrix_transactions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let txn: Transaction = serde_json::from_value(data)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize transaction: {}", e)))?;
                Ok(Some(txn))
            }
            None => Ok(None),
        }
    }

    /// Get a transaction, fetching from API if not in cache.
    pub async fn get_or_fetch_transaction(&self, id: &str) -> Result<Option<Transaction>> {
        if let Some(txn) = self.get_transaction(id).await? {
            return Ok(Some(txn));
        }

        let txn: Option<Transaction> = self.client.get_one(EntityType::Txns, id).await?;

        if let Some(ref t) = txn {
            self.upsert_transaction(t).await?;
        }

        Ok(txn)
    }

    /// Upsert a transaction into the cache.
    pub async fn upsert_transaction(&self, txn: &Transaction) -> Result<()> {
        let data = serde_json::to_value(txn)
            .map_err(|e| Error::Internal(format!("Failed to serialize transaction: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO payrix_transactions (id, data, merchant_id, token_id, status, type, total, created_at, modified_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                merchant_id = EXCLUDED.merchant_id,
                token_id = EXCLUDED.token_id,
                status = EXCLUDED.status,
                type = EXCLUDED.type,
                total = EXCLUDED.total,
                modified_at = EXCLUDED.modified_at,
                synced_at = NOW()
            "#,
        )
        .bind(txn.id.as_str())
        .bind(&data)
        .bind(txn.merchant.as_ref().map(|m| m.as_str()))
        .bind(txn.token.as_deref())
        .bind(txn.status.map(|s| s as i32))
        .bind(txn.txn_type as i32)
        .bind(txn.total)
        .bind(parse_payrix_datetime(txn.created.as_deref()))
        .bind(parse_payrix_datetime(txn.modified.as_deref()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Merchant Methods
    // =========================================================================

    /// Get a merchant from the cache by ID.
    pub async fn get_merchant(&self, id: &str) -> Result<Option<Merchant>> {
        let row = sqlx::query("SELECT data FROM payrix_merchants WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let merchant: Merchant = serde_json::from_value(data)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize merchant: {}", e)))?;
                Ok(Some(merchant))
            }
            None => Ok(None),
        }
    }

    /// Get a merchant, fetching from API if not in cache.
    pub async fn get_or_fetch_merchant(&self, id: &str) -> Result<Option<Merchant>> {
        if let Some(merchant) = self.get_merchant(id).await? {
            return Ok(Some(merchant));
        }

        let merchant: Option<Merchant> = self.client.get_one(EntityType::Merchants, id).await?;

        if let Some(ref m) = merchant {
            self.upsert_merchant(m).await?;
        }

        Ok(merchant)
    }

    /// Upsert a merchant into the cache.
    pub async fn upsert_merchant(&self, merchant: &Merchant) -> Result<()> {
        let data = serde_json::to_value(merchant)
            .map_err(|e| Error::Internal(format!("Failed to serialize merchant: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO payrix_merchants (id, data, entity_id, status, dba, created_at, modified_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                entity_id = EXCLUDED.entity_id,
                status = EXCLUDED.status,
                dba = EXCLUDED.dba,
                modified_at = EXCLUDED.modified_at,
                synced_at = NOW()
            "#,
        )
        .bind(merchant.id.as_str())
        .bind(&data)
        .bind(merchant.entity.as_ref().map(|e| e.as_str()))
        .bind(merchant.status.map(|s| s as i32))
        .bind(merchant.dba.as_deref())
        .bind(parse_payrix_datetime(merchant.created.as_deref()))
        .bind(parse_payrix_datetime(merchant.modified.as_deref()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Customer Methods
    // =========================================================================

    /// Get a customer from the cache by ID.
    pub async fn get_customer(&self, id: &str) -> Result<Option<Customer>> {
        let row = sqlx::query("SELECT data FROM payrix_customers WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let customer: Customer = serde_json::from_value(data)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize customer: {}", e)))?;
                Ok(Some(customer))
            }
            None => Ok(None),
        }
    }

    /// Get a customer, fetching from API if not in cache.
    pub async fn get_or_fetch_customer(&self, id: &str) -> Result<Option<Customer>> {
        if let Some(customer) = self.get_customer(id).await? {
            return Ok(Some(customer));
        }

        let customer: Option<Customer> = self.client.get_one(EntityType::Customers, id).await?;

        if let Some(ref c) = customer {
            self.upsert_customer(c).await?;
        }

        Ok(customer)
    }

    /// Upsert a customer into the cache.
    pub async fn upsert_customer(&self, customer: &Customer) -> Result<()> {
        let data = serde_json::to_value(customer)
            .map_err(|e| Error::Internal(format!("Failed to serialize customer: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO payrix_customers (id, data, merchant_id, email, first_name, last_name, created_at, modified_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                merchant_id = EXCLUDED.merchant_id,
                email = EXCLUDED.email,
                first_name = EXCLUDED.first_name,
                last_name = EXCLUDED.last_name,
                modified_at = EXCLUDED.modified_at,
                synced_at = NOW()
            "#,
        )
        .bind(customer.id.as_str())
        .bind(&data)
        .bind(customer.merchant.as_ref().map(|m| m.as_str()))
        .bind(customer.email.as_deref())
        .bind(customer.first.as_deref())
        .bind(customer.last.as_deref())
        .bind(parse_payrix_datetime(customer.created.as_deref()))
        .bind(parse_payrix_datetime(customer.modified.as_deref()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Token Methods
    // =========================================================================

    /// Get a token from the cache by ID.
    pub async fn get_token(&self, id: &str) -> Result<Option<Token>> {
        let row = sqlx::query("SELECT data FROM payrix_tokens WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let token: Token = serde_json::from_value(data)
                    .map_err(|e| Error::Internal(format!("Failed to deserialize token: {}", e)))?;
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    /// Get a token, fetching from API if not in cache.
    pub async fn get_or_fetch_token(&self, id: &str) -> Result<Option<Token>> {
        if let Some(token) = self.get_token(id).await? {
            return Ok(Some(token));
        }

        let token: Option<Token> = self.client.get_one(EntityType::Tokens, id).await?;

        if let Some(ref t) = token {
            self.upsert_token(t).await?;
        }

        Ok(token)
    }

    /// Upsert a token into the cache.
    pub async fn upsert_token(&self, token: &Token) -> Result<()> {
        let data = serde_json::to_value(token)
            .map_err(|e| Error::Internal(format!("Failed to serialize token: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO payrix_tokens (id, data, customer_id, payment_type, status, created_at, modified_at, synced_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                customer_id = EXCLUDED.customer_id,
                payment_type = EXCLUDED.payment_type,
                status = EXCLUDED.status,
                modified_at = EXCLUDED.modified_at,
                synced_at = NOW()
            "#,
        )
        .bind(token.id.as_str())
        .bind(&data)
        .bind(token.customer.as_ref().map(|c| c.as_str()))
        .bind(token.payment.map(|p| p as i32))
        .bind(token.status.map(|s| s as i32))
        .bind(parse_payrix_datetime(token.created.as_deref()))
        .bind(parse_payrix_datetime(token.modified.as_deref()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Webhook Processing
    // =========================================================================

    /// Process a webhook event and update the cache accordingly.
    ///
    /// This method extracts entity data from webhook events and updates
    /// the appropriate cache table.
    #[cfg(feature = "webhooks")]
    pub async fn process_webhook(&self, event: &WebhookEvent) -> Result<()> {
        debug!(
            event_type = %event.event_type,
            resource_type = %event.resource_type,
            resource_id = %event.resource_id,
            "Processing webhook for cache"
        );

        match event.resource_type.as_str() {
            "chargebacks" => {
                if let Ok(chargeback) = serde_json::from_value::<Chargeback>(event.data.clone()) {
                    self.upsert_chargeback(&chargeback).await?;
                } else {
                    warn!("Failed to parse chargeback from webhook data");
                }
            }
            "txns" => {
                if let Ok(txn) = serde_json::from_value::<Transaction>(event.data.clone()) {
                    self.upsert_transaction(&txn).await?;
                } else {
                    warn!("Failed to parse transaction from webhook data");
                }
            }
            "merchants" => {
                if let Ok(merchant) = serde_json::from_value::<Merchant>(event.data.clone()) {
                    self.upsert_merchant(&merchant).await?;
                } else {
                    warn!("Failed to parse merchant from webhook data");
                }
            }
            "customers" => {
                if let Ok(customer) = serde_json::from_value::<Customer>(event.data.clone()) {
                    self.upsert_customer(&customer).await?;
                } else {
                    warn!("Failed to parse customer from webhook data");
                }
            }
            "tokens" => {
                if let Ok(token) = serde_json::from_value::<Token>(event.data.clone()) {
                    self.upsert_token(&token).await?;
                } else {
                    warn!("Failed to parse token from webhook data");
                }
            }
            _ => {
                debug!(resource_type = %event.resource_type, "Ignoring webhook for uncached entity type");
            }
        }

        Ok(())
    }

    // =========================================================================
    // Sync Methods (in sync.rs)
    // =========================================================================

    /// Perform an initial sync of all entities from Payrix.
    ///
    /// This fetches all chargebacks, transactions, merchants, customers, and tokens
    /// from the Payrix API and stores them in the local cache.
    ///
    /// **Note:** This can take a long time for accounts with many entities.
    /// Consider using `sync_entity_type` for incremental syncs.
    pub async fn initial_sync(&self) -> Result<SyncStats> {
        super::sync::initial_sync(self).await
    }

    /// Sync a specific entity type from Payrix.
    pub async fn sync_entity_type(&self, entity_type: EntityType) -> Result<usize> {
        super::sync::sync_entity_type(self, entity_type).await
    }

    /// Get the last sync time for an entity type.
    pub async fn last_sync_time(&self, entity_type: &str) -> Result<Option<DateTime<Utc>>> {
        let row = sqlx::query(
            r#"
            SELECT completed_at FROM payrix_sync_log
            WHERE entity_type = $1 AND completed_at IS NOT NULL
            ORDER BY completed_at DESC
            LIMIT 1
            "#,
        )
        .bind(entity_type)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.get("completed_at")))
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse a Payrix datetime string into a chrono DateTime.
fn parse_payrix_datetime(s: Option<&str>) -> Option<DateTime<Utc>> {
    s.and_then(|s| {
        // Payrix format: "YYYY-MM-DD HH:MM:SS.SSSS"
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f")
            .ok()
            .map(|dt| dt.and_utc())
    })
}
