//! Synchronization logic for the entity cache.
//!
//! This module handles initial sync and incremental sync of entities
//! from the Payrix API to the local cache.

use std::time::Instant;

use sqlx::Row;
use tracing::{debug, error, info, warn};

use crate::entity::EntityType;
use crate::error::Result;
use crate::types::{Chargeback, Customer, Merchant, Token, Transaction};

use super::entity_cache::{EntityCache, SyncStats};

// =============================================================================
// Initial Sync
// =============================================================================

/// Perform an initial sync of all entities from Payrix.
///
/// This fetches all chargebacks, transactions, merchants, customers, and tokens
/// from the Payrix API and stores them in the local cache.
pub async fn initial_sync(cache: &EntityCache) -> Result<SyncStats> {
    let start = Instant::now();
    let mut stats = SyncStats::default();

    info!("Starting initial sync from Payrix API");

    // Sync each entity type, collecting stats
    match sync_chargebacks(cache).await {
        Ok(count) => {
            stats.chargebacks = count;
            info!(count, "Synced chargebacks");
        }
        Err(e) => {
            error!(error = %e, "Failed to sync chargebacks");
            stats.errors.push(format!("chargebacks: {}", e));
        }
    }

    match sync_transactions(cache).await {
        Ok(count) => {
            stats.transactions = count;
            info!(count, "Synced transactions");
        }
        Err(e) => {
            error!(error = %e, "Failed to sync transactions");
            stats.errors.push(format!("transactions: {}", e));
        }
    }

    match sync_merchants(cache).await {
        Ok(count) => {
            stats.merchants = count;
            info!(count, "Synced merchants");
        }
        Err(e) => {
            error!(error = %e, "Failed to sync merchants");
            stats.errors.push(format!("merchants: {}", e));
        }
    }

    match sync_customers(cache).await {
        Ok(count) => {
            stats.customers = count;
            info!(count, "Synced customers");
        }
        Err(e) => {
            error!(error = %e, "Failed to sync customers");
            stats.errors.push(format!("customers: {}", e));
        }
    }

    match sync_tokens(cache).await {
        Ok(count) => {
            stats.tokens = count;
            info!(count, "Synced tokens");
        }
        Err(e) => {
            error!(error = %e, "Failed to sync tokens");
            stats.errors.push(format!("tokens: {}", e));
        }
    }

    stats.duration = start.elapsed();
    info!(
        total = stats.total(),
        duration_secs = stats.duration.as_secs_f64(),
        "Initial sync complete"
    );

    Ok(stats)
}

// =============================================================================
// Entity Type Sync
// =============================================================================

/// Sync a specific entity type from Payrix.
///
/// Returns the number of entities synced.
pub async fn sync_entity_type(cache: &EntityCache, entity_type: EntityType) -> Result<usize> {
    match entity_type {
        EntityType::Chargebacks => sync_chargebacks(cache).await,
        EntityType::Txns => sync_transactions(cache).await,
        EntityType::Merchants => sync_merchants(cache).await,
        EntityType::Customers => sync_customers(cache).await,
        EntityType::Tokens => sync_tokens(cache).await,
        _ => {
            warn!(entity_type = ?entity_type, "Entity type not supported for caching");
            Ok(0)
        }
    }
}

// =============================================================================
// Individual Entity Syncs
// =============================================================================

async fn sync_chargebacks(cache: &EntityCache) -> Result<usize> {
    let entity_type = "chargebacks";
    let log_id = start_sync_log(cache, entity_type, "full").await?;

    let result = async {
        debug!("Fetching all chargebacks from Payrix API");
        let chargebacks: Vec<Chargeback> = cache.client().get_all(EntityType::Chargebacks).await?;
        let count = chargebacks.len();

        debug!(count, "Upserting chargebacks to cache");
        for chargeback in &chargebacks {
            cache.upsert_chargeback(chargeback).await?;
        }

        Ok::<_, crate::error::Error>(count)
    }
    .await;

    match &result {
        Ok(count) => complete_sync_log(cache, log_id, *count, None).await?,
        Err(e) => complete_sync_log(cache, log_id, 0, Some(&e.to_string())).await?,
    }

    result
}

async fn sync_transactions(cache: &EntityCache) -> Result<usize> {
    let entity_type = "transactions";
    let log_id = start_sync_log(cache, entity_type, "full").await?;

    let result = async {
        debug!("Fetching all transactions from Payrix API");
        let transactions: Vec<Transaction> = cache.client().get_all(EntityType::Txns).await?;
        let count = transactions.len();

        debug!(count, "Upserting transactions to cache");
        for txn in &transactions {
            cache.upsert_transaction(txn).await?;
        }

        Ok::<_, crate::error::Error>(count)
    }
    .await;

    match &result {
        Ok(count) => complete_sync_log(cache, log_id, *count, None).await?,
        Err(e) => complete_sync_log(cache, log_id, 0, Some(&e.to_string())).await?,
    }

    result
}

async fn sync_merchants(cache: &EntityCache) -> Result<usize> {
    let entity_type = "merchants";
    let log_id = start_sync_log(cache, entity_type, "full").await?;

    let result = async {
        debug!("Fetching all merchants from Payrix API");
        let merchants: Vec<Merchant> = cache.client().get_all(EntityType::Merchants).await?;
        let count = merchants.len();

        debug!(count, "Upserting merchants to cache");
        for merchant in &merchants {
            cache.upsert_merchant(merchant).await?;
        }

        Ok::<_, crate::error::Error>(count)
    }
    .await;

    match &result {
        Ok(count) => complete_sync_log(cache, log_id, *count, None).await?,
        Err(e) => complete_sync_log(cache, log_id, 0, Some(&e.to_string())).await?,
    }

    result
}

async fn sync_customers(cache: &EntityCache) -> Result<usize> {
    let entity_type = "customers";
    let log_id = start_sync_log(cache, entity_type, "full").await?;

    let result = async {
        debug!("Fetching all customers from Payrix API");
        let customers: Vec<Customer> = cache.client().get_all(EntityType::Customers).await?;
        let count = customers.len();

        debug!(count, "Upserting customers to cache");
        for customer in &customers {
            cache.upsert_customer(customer).await?;
        }

        Ok::<_, crate::error::Error>(count)
    }
    .await;

    match &result {
        Ok(count) => complete_sync_log(cache, log_id, *count, None).await?,
        Err(e) => complete_sync_log(cache, log_id, 0, Some(&e.to_string())).await?,
    }

    result
}

async fn sync_tokens(cache: &EntityCache) -> Result<usize> {
    let entity_type = "tokens";
    let log_id = start_sync_log(cache, entity_type, "full").await?;

    let result = async {
        debug!("Fetching all tokens from Payrix API");
        let tokens: Vec<Token> = cache.client().get_all(EntityType::Tokens).await?;
        let count = tokens.len();

        debug!(count, "Upserting tokens to cache");
        for token in &tokens {
            cache.upsert_token(token).await?;
        }

        Ok::<_, crate::error::Error>(count)
    }
    .await;

    match &result {
        Ok(count) => complete_sync_log(cache, log_id, *count, None).await?,
        Err(e) => complete_sync_log(cache, log_id, 0, Some(&e.to_string())).await?,
    }

    result
}

// =============================================================================
// Sync Logging
// =============================================================================

async fn start_sync_log(cache: &EntityCache, entity_type: &str, operation: &str) -> Result<i32> {
    let row = sqlx::query(
        r#"
        INSERT INTO payrix_sync_log (entity_type, operation)
        VALUES ($1, $2)
        RETURNING id
        "#,
    )
    .bind(entity_type)
    .bind(operation)
    .fetch_one(cache.pool())
    .await?;

    Ok(row.get("id"))
}

async fn complete_sync_log(
    cache: &EntityCache,
    log_id: i32,
    records_processed: usize,
    error_message: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE payrix_sync_log
        SET completed_at = NOW(),
            records_processed = $2,
            error_message = $3
        WHERE id = $1
        "#,
    )
    .bind(log_id)
    .bind(records_processed as i32)
    .bind(error_message)
    .execute(cache.pool())
    .await?;

    Ok(())
}
