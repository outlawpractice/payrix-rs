//! Database schema for the entity cache.
//!
//! This module provides functions to create the necessary database tables
//! for caching Payrix entities.

use sqlx::PgPool;

use crate::error::Result;

/// SQL to create the chargebacks cache table.
const CREATE_CHARGEBACKS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_chargebacks (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    merchant_id VARCHAR(50),
    txn_id VARCHAR(50),
    cycle VARCHAR(50),
    status INTEGER,
    total BIGINT,
    reason_code VARCHAR(50),
    created_at TIMESTAMPTZ,
    modified_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
"#;

/// SQL to create indexes for the chargebacks table.
const CREATE_CHARGEBACKS_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_chargebacks_merchant ON payrix_chargebacks(merchant_id);
CREATE INDEX IF NOT EXISTS idx_chargebacks_txn ON payrix_chargebacks(txn_id);
CREATE INDEX IF NOT EXISTS idx_chargebacks_status ON payrix_chargebacks(status);
CREATE INDEX IF NOT EXISTS idx_chargebacks_cycle ON payrix_chargebacks(cycle);
CREATE INDEX IF NOT EXISTS idx_chargebacks_synced ON payrix_chargebacks(synced_at)
"#;

/// SQL to create the transactions cache table.
const CREATE_TRANSACTIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_transactions (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    merchant_id VARCHAR(50),
    token_id VARCHAR(50),
    status INTEGER,
    type INTEGER,
    total BIGINT,
    created_at TIMESTAMPTZ,
    modified_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
"#;

/// SQL to create indexes for the transactions table.
const CREATE_TRANSACTIONS_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_transactions_merchant ON payrix_transactions(merchant_id);
CREATE INDEX IF NOT EXISTS idx_transactions_token ON payrix_transactions(token_id);
CREATE INDEX IF NOT EXISTS idx_transactions_status ON payrix_transactions(status);
CREATE INDEX IF NOT EXISTS idx_transactions_type ON payrix_transactions(type);
CREATE INDEX IF NOT EXISTS idx_transactions_synced ON payrix_transactions(synced_at)
"#;

/// SQL to create the merchants cache table.
const CREATE_MERCHANTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_merchants (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    entity_id VARCHAR(50),
    status INTEGER,
    dba VARCHAR(255),
    created_at TIMESTAMPTZ,
    modified_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
"#;

/// SQL to create indexes for the merchants table.
const CREATE_MERCHANTS_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_merchants_entity ON payrix_merchants(entity_id);
CREATE INDEX IF NOT EXISTS idx_merchants_status ON payrix_merchants(status);
CREATE INDEX IF NOT EXISTS idx_merchants_synced ON payrix_merchants(synced_at)
"#;

/// SQL to create the customers cache table.
const CREATE_CUSTOMERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_customers (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    merchant_id VARCHAR(50),
    email VARCHAR(255),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    created_at TIMESTAMPTZ,
    modified_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
"#;

/// SQL to create indexes for the customers table.
const CREATE_CUSTOMERS_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_customers_merchant ON payrix_customers(merchant_id);
CREATE INDEX IF NOT EXISTS idx_customers_email ON payrix_customers(email);
CREATE INDEX IF NOT EXISTS idx_customers_synced ON payrix_customers(synced_at)
"#;

/// SQL to create the tokens cache table.
const CREATE_TOKENS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_tokens (
    id VARCHAR(50) PRIMARY KEY,
    data JSONB NOT NULL,
    customer_id VARCHAR(50),
    payment_type INTEGER,
    status INTEGER,
    created_at TIMESTAMPTZ,
    modified_at TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
"#;

/// SQL to create indexes for the tokens table.
const CREATE_TOKENS_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_tokens_customer ON payrix_tokens(customer_id);
CREATE INDEX IF NOT EXISTS idx_tokens_status ON payrix_tokens(status);
CREATE INDEX IF NOT EXISTS idx_tokens_synced ON payrix_tokens(synced_at)
"#;

/// SQL to create the sync_log table for tracking sync operations.
const CREATE_SYNC_LOG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS payrix_sync_log (
    id SERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL,
    operation VARCHAR(20) NOT NULL,
    entity_id VARCHAR(50),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    records_processed INTEGER DEFAULT 0,
    error_message TEXT
)
"#;

/// SQL to create indexes for the sync_log table.
const CREATE_SYNC_LOG_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_sync_log_entity_type ON payrix_sync_log(entity_type);
CREATE INDEX IF NOT EXISTS idx_sync_log_started ON payrix_sync_log(started_at DESC)
"#;

/// Ensure all cache tables and indexes exist.
///
/// This function creates the necessary database schema for the entity cache.
/// It is safe to call multiple times (uses CREATE IF NOT EXISTS).
///
/// # Arguments
///
/// * `pool` - The database connection pool
///
/// # Example
///
/// ```no_run
/// use sqlx::PgPool;
/// use payrix::cache::ensure_schema;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let pool = PgPool::connect("postgres://localhost/payrix_cache").await?;
/// ensure_schema(&pool).await?;
/// # Ok(())
/// # }
/// ```
pub async fn ensure_schema(pool: &PgPool) -> Result<()> {
    // Create tables
    sqlx::query(CREATE_CHARGEBACKS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_TRANSACTIONS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_MERCHANTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_CUSTOMERS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_TOKENS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_SYNC_LOG_TABLE).execute(pool).await?;

    // Create indexes (split by semicolon and execute individually)
    for index_sql in CREATE_CHARGEBACKS_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    for index_sql in CREATE_TRANSACTIONS_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    for index_sql in CREATE_MERCHANTS_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    for index_sql in CREATE_CUSTOMERS_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    for index_sql in CREATE_TOKENS_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    for index_sql in CREATE_SYNC_LOG_INDEXES.split(';') {
        let sql = index_sql.trim();
        if !sql.is_empty() {
            sqlx::query(sql).execute(pool).await?;
        }
    }

    Ok(())
}
