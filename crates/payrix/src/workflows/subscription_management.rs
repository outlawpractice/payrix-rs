//! Subscription management workflow for Payrix.
//!
//! This module provides a high-level interface for managing subscriptions in Payrix.
//! It encapsulates the multi-step process of subscribing customers to plans and
//! provides utilities for querying payment history and managing subscription state.
//!
//! # Overview
//!
//! Payrix subscriptions involve three core components:
//!
//! 1. **Plans** - Define billing amount and frequency (not customer-specific)
//! 2. **Tokens** - Stored payment methods (cards/bank accounts)
//! 3. **Subscriptions** - Link a customer's token to a plan
//!
//! # Key Behaviors
//!
//! - First payment is NOT processed immediately when a subscription is created
//! - To charge immediately, use `charge_immediately: true` which creates a separate Sale transaction
//! - `frozen = true` pauses billing; `inactive = true` cancels the subscription
//! - Max consecutive failures triggers automatic subscription cancellation
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::subscription_management::*;
//!
//! # async fn example() -> Result<(), SubscriptionError> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Subscribe a customer to a new monthly plan
//! let config = SubscribeCustomerConfig {
//!     merchant_id: "t1_mer_xxx".to_string(),
//!     plan: PlanReference::NewPlan(PlanConfig {
//!         name: "Premium Monthly".to_string(),
//!         description: Some("Premium subscription plan".to_string()),
//!         schedule: BillingSchedule::Monthly,
//!         schedule_factor: 1,
//!         amount: 2999, // $29.99
//!         max_failures: Some(3),
//!     }),
//!     token: TokenReference::ExistingId("t1_tok_xxx".to_string()),
//!     start_date: None, // Start today
//!     end_date: None,   // No end date
//!     charge_immediately: true,
//!     tax: None,
//!     descriptor: None,
//!     origin: None,
//!     txn_description: None,
//! };
//!
//! let result = add_plan_to_customer(&client, config).await?;
//! println!("Created subscription: {}", result.subscription.id.as_str());
//!
//! // Check payment history
//! let history = payments_to_date(&client, result.subscription.id.as_str()).await?;
//! println!("Total paid: ${:.2}", history.total_paid as f64 / 100.0);
//!
//! // Get next payment info
//! let next = next_payment(&client, result.subscription.id.as_str()).await?;
//! println!("Next payment: {} - ${:.2}", next.date, next.amount as f64 / 100.0);
//! # Ok(())
//! # }
//! ```

use chrono::{Datelike, Duration, Months, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

use crate::client::PayrixClient;
use crate::entity::EntityType;
use crate::error::Error;
use crate::search::{parse_payrix_date, SearchBuilder, SearchOperator};
use crate::types::{
    CreateToken, PaymentInfo, PaymentMethod, Plan, PlanSchedule, Subscription, SubscriptionOrigin,
    Token, Transaction, TransactionStatus, TransactionType,
};

// ============================================================================
// Error Types
// ============================================================================

/// Errors specific to subscription operations.
#[derive(Debug)]
pub enum SubscriptionError {
    /// Plan not found with the given ID.
    PlanNotFound(String),

    /// Token not found with the given ID.
    TokenNotFound(String),

    /// Subscription not found with the given ID.
    SubscriptionNotFound(String),

    /// Customer not found with the given ID.
    CustomerNotFound(String),

    /// Invalid subscription state for the requested operation.
    InvalidState(String),

    /// Error calculating payment dates or amounts.
    CalculationError(String),

    /// Invalid date format provided.
    InvalidDate(String),

    /// Underlying Payrix API error.
    Api(Error),
}

impl fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionError::PlanNotFound(id) => write!(f, "Plan not found: {}", id),
            SubscriptionError::TokenNotFound(id) => write!(f, "Token not found: {}", id),
            SubscriptionError::SubscriptionNotFound(id) => {
                write!(f, "Subscription not found: {}", id)
            }
            SubscriptionError::CustomerNotFound(id) => write!(f, "Customer not found: {}", id),
            SubscriptionError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            SubscriptionError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
            SubscriptionError::InvalidDate(msg) => write!(f, "Invalid date: {}", msg),
            SubscriptionError::Api(err) => write!(f, "Payrix API error: {}", err),
        }
    }
}

impl std::error::Error for SubscriptionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SubscriptionError::Api(err) => Some(err),
            _ => None,
        }
    }
}

impl From<Error> for SubscriptionError {
    fn from(err: Error) -> Self {
        SubscriptionError::Api(err)
    }
}

/// Result type for subscription operations.
pub type SubscriptionResult<T> = std::result::Result<T, SubscriptionError>;

// ============================================================================
// User-Friendly Enums
// ============================================================================

/// User-friendly billing schedule.
///
/// This is a more ergonomic wrapper around [`PlanSchedule`] for configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BillingSchedule {
    /// Bill daily.
    Daily,

    /// Bill weekly.
    Weekly,

    /// Bill monthly (default).
    #[default]
    Monthly,

    /// Bill annually.
    Annually,
}

impl BillingSchedule {
    /// Convert to Payrix's `PlanSchedule` enum.
    pub fn to_plan_schedule(self) -> PlanSchedule {
        match self {
            BillingSchedule::Daily => PlanSchedule::Daily,
            BillingSchedule::Weekly => PlanSchedule::Weekly,
            BillingSchedule::Monthly => PlanSchedule::Monthly,
            BillingSchedule::Annually => PlanSchedule::Annually,
        }
    }

    /// Create from Payrix's `PlanSchedule` enum.
    pub fn from_plan_schedule(schedule: PlanSchedule) -> Self {
        match schedule {
            PlanSchedule::Daily => BillingSchedule::Daily,
            PlanSchedule::Weekly => BillingSchedule::Weekly,
            PlanSchedule::Monthly => BillingSchedule::Monthly,
            PlanSchedule::Annually => BillingSchedule::Annually,
        }
    }
}

/// Subscription state for user-friendly status display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionState {
    /// Active and will charge on schedule.
    Active,

    /// Temporarily paused (frozen = true).
    Paused,

    /// Cancelled (inactive = true).
    Cancelled,

    /// Not started yet (start date in future).
    Pending,

    /// Past end date.
    Expired,
}

impl SubscriptionState {
    /// Determine state from subscription flags and dates.
    pub fn from_subscription(sub: &Subscription, today: NaiveDate) -> Self {
        if sub.inactive {
            return SubscriptionState::Cancelled;
        }
        if sub.frozen {
            return SubscriptionState::Paused;
        }

        // Check finish date
        if let Some(finish) = sub.finish {
            if let Some(finish_date) = parse_payrix_date(&finish.to_string()) {
                if finish_date < today {
                    return SubscriptionState::Expired;
                }
            }
        }

        // Check start date
        if let Some(start) = sub.start {
            if let Some(start_date) = parse_payrix_date(&start.to_string()) {
                if start_date > today {
                    return SubscriptionState::Pending;
                }
            }
        }

        SubscriptionState::Active
    }
}

// ============================================================================
// Configuration Types
// ============================================================================

/// Configuration for creating a new plan inline.
#[derive(Debug, Clone)]
pub struct PlanConfig {
    /// Human-readable name for the plan.
    pub name: String,

    /// Optional description of the plan.
    pub description: Option<String>,

    /// Billing schedule (Daily, Weekly, Monthly, Annually).
    pub schedule: BillingSchedule,

    /// Multiplier for the schedule.
    ///
    /// For example, with `schedule: Monthly` and `schedule_factor: 2`,
    /// the plan bills every 2 months.
    ///
    /// Must be greater than 0.
    pub schedule_factor: i32,

    /// Amount to charge in cents.
    ///
    /// Example: 2999 = $29.99
    ///
    /// Must be greater than 0.
    pub amount: i64,

    /// Maximum consecutive payment failures before subscription cancellation.
    ///
    /// Defaults to 3 if not specified.
    pub max_failures: Option<i32>,
}

impl PlanConfig {
    /// Validate the plan configuration.
    pub fn validate(&self) -> Result<(), SubscriptionError> {
        if self.name.is_empty() {
            return Err(SubscriptionError::InvalidState(
                "Plan name cannot be empty".to_string(),
            ));
        }
        if self.schedule_factor <= 0 {
            return Err(SubscriptionError::InvalidState(
                "Schedule factor must be greater than 0".to_string(),
            ));
        }
        if self.amount <= 0 {
            return Err(SubscriptionError::InvalidState(
                "Amount must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Reference to either an existing plan or a new plan to create.
#[derive(Debug, Clone)]
pub enum PlanReference {
    /// Use an existing plan by its ID.
    ExistingId(String),

    /// Create a new plan with this configuration.
    NewPlan(PlanConfig),
}

/// Configuration for creating a new payment token inline.
///
/// **Security Note:** This struct contains sensitive payment data.
/// The `Debug` implementation redacts card numbers and CVV values.
#[derive(Clone)]
pub struct TokenConfig {
    /// Customer ID to associate the token with.
    pub customer_id: String,

    /// Payment method type.
    pub method: PaymentMethod,

    /// Card or account number.
    pub number: String,

    /// Routing number (for bank accounts).
    pub routing: Option<String>,

    /// Card expiration in MMYY format.
    pub expiration: Option<String>,

    /// Card CVV/security code.
    pub cvv: Option<String>,
}

impl fmt::Debug for TokenConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Redact sensitive fields
        let redacted_number = if self.number.len() > 4 {
            format!("****{}", &self.number[self.number.len() - 4..])
        } else {
            "****".to_string()
        };

        f.debug_struct("TokenConfig")
            .field("customer_id", &self.customer_id)
            .field("method", &self.method)
            .field("number", &redacted_number)
            .field("routing", &self.routing.as_ref().map(|_| "[REDACTED]"))
            .field("expiration", &self.expiration)
            .field("cvv", &self.cvv.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

/// Reference to either an existing token or a new token to create.
#[derive(Debug, Clone)]
pub enum TokenReference {
    /// Use an existing token by its ID.
    ExistingId(String),

    /// Create a new token with this configuration.
    NewToken(TokenConfig),
}

/// Complete configuration for subscribing a customer to a plan.
#[derive(Debug, Clone)]
pub struct SubscribeCustomerConfig {
    /// Merchant ID for the subscription.
    pub merchant_id: String,

    /// Plan to subscribe to (existing or create new).
    pub plan: PlanReference,

    /// Payment token to use (existing or create new).
    pub token: TokenReference,

    /// Start date in YYYYMMDD format.
    ///
    /// Defaults to today if not specified.
    pub start_date: Option<i32>,

    /// End date in YYYYMMDD format (optional).
    ///
    /// For fixed-term subscriptions.
    pub end_date: Option<i32>,

    /// Tax amount in cents (optional).
    pub tax: Option<i64>,

    /// Custom descriptor for transactions.
    pub descriptor: Option<String>,

    /// Transaction origin.
    ///
    /// Defaults to `ECommerce` if not specified.
    pub origin: Option<SubscriptionOrigin>,

    /// Whether to charge immediately.
    ///
    /// If true, a separate Sale transaction is created since Payrix
    /// doesn't process payment immediately when subscription is created.
    pub charge_immediately: bool,

    /// Custom description for generated transactions.
    pub txn_description: Option<String>,
}

// ============================================================================
// Result Types
// ============================================================================

/// Result of subscribing a customer to a plan.
#[derive(Debug, Clone)]
pub struct SubscribeCustomerResult {
    /// The created subscription.
    pub subscription: Subscription,

    /// The plan (fetched or created).
    pub plan: Plan,

    /// The token (fetched or created).
    pub token: Token,

    /// If `charge_immediately` was true, the initial transaction.
    pub initial_transaction: Option<Transaction>,

    /// Whether a new plan was created.
    pub plan_created: bool,

    /// Whether a new token was created.
    pub token_created: bool,
}

/// Payment history for a subscription.
#[derive(Debug, Clone)]
pub struct PaymentHistory {
    /// Total amount paid in cents.
    pub total_paid: i64,

    /// Number of successful payments.
    pub payment_count: i32,

    /// Number of failed payments.
    pub failed_count: i32,

    /// Last successful payment date.
    pub last_payment_date: Option<String>,

    /// Last successful payment amount in cents.
    pub last_payment_amount: Option<i64>,

    /// All transactions for this subscription (most recent first).
    pub transactions: Vec<Transaction>,
}

impl PaymentHistory {
    /// Get total paid as dollars.
    pub fn total_paid_dollars(&self) -> f64 {
        self.total_paid as f64 / 100.0
    }
}

/// Next payment prediction for a subscription.
#[derive(Debug, Clone)]
pub struct NextPayment {
    /// Expected payment date.
    pub date: NaiveDate,

    /// Expected amount in cents.
    pub amount: i64,

    /// Days until next payment.
    pub days_until: i64,

    /// Whether subscription is active (will actually charge).
    pub is_active: bool,
}

impl NextPayment {
    /// Get amount as dollars.
    pub fn amount_dollars(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

/// Comprehensive subscription status.
#[derive(Debug, Clone)]
pub struct SubscriptionStatus {
    /// The subscription.
    pub subscription: Subscription,

    /// The associated plan (if available).
    pub plan: Option<Plan>,

    /// Current state.
    pub state: SubscriptionState,

    /// Payment history summary.
    pub payment_summary: PaymentHistory,

    /// Next scheduled payment (if active).
    pub next_payment: Option<NextPayment>,
}

/// Revenue calculation result for a plan.
#[derive(Debug, Clone)]
pub struct SubscriptionRevenue {
    /// Total revenue collected in cents.
    pub total_collected: i64,

    /// Projected monthly revenue in cents (based on active subscriptions).
    pub projected_monthly: i64,

    /// Projected annual revenue in cents.
    pub projected_annual: i64,

    /// Active subscriber count.
    pub active_subscribers: i32,

    /// Churned subscriber count (cancelled in period).
    pub churned_subscribers: i32,
}

impl SubscriptionRevenue {
    /// Get total collected as dollars.
    pub fn total_collected_dollars(&self) -> f64 {
        self.total_collected as f64 / 100.0
    }

    /// Get projected monthly as dollars.
    pub fn projected_monthly_dollars(&self) -> f64 {
        self.projected_monthly as f64 / 100.0
    }

    /// Get projected annual as dollars.
    pub fn projected_annual_dollars(&self) -> f64 {
        self.projected_annual as f64 / 100.0
    }
}

/// Upcoming payment for a customer.
#[derive(Debug, Clone)]
pub struct UpcomingPayment {
    /// Subscription ID.
    pub subscription_id: String,

    /// Plan name (if available).
    pub plan_name: Option<String>,

    /// Payment date.
    pub date: NaiveDate,

    /// Amount in cents.
    pub amount: i64,

    /// Days until payment.
    pub days_until: i64,
}

impl UpcomingPayment {
    /// Get amount as dollars.
    pub fn amount_dollars(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

// ============================================================================
// Core Functions
// ============================================================================

/// Subscribe a customer to a plan.
///
/// This is the main entry point for creating subscriptions. It supports:
/// - Using an existing plan or creating a new one inline
/// - Using an existing token or creating a new one inline
/// - Optionally charging immediately (creates a separate Sale transaction)
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `config` - Subscription configuration
///
/// # Returns
///
/// A [`SubscribeCustomerResult`] containing the created subscription and related entities.
///
/// # Example
///
/// ```no_run
/// # use payrix::{PayrixClient, Environment};
/// # use payrix::workflows::subscription_management::*;
/// # async fn example() -> Result<(), SubscriptionError> {
/// # let client = PayrixClient::new("api-key", Environment::Test)?;
/// let config = SubscribeCustomerConfig {
///     merchant_id: "t1_mer_xxx".to_string(),
///     plan: PlanReference::ExistingId("t1_pln_xxx".to_string()),
///     token: TokenReference::ExistingId("t1_tok_xxx".to_string()),
///     start_date: None,
///     end_date: None,
///     charge_immediately: false,
///     tax: None,
///     descriptor: None,
///     origin: None,
///     txn_description: None,
/// };
///
/// let result = add_plan_to_customer(&client, config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn add_plan_to_customer(
    client: &PayrixClient,
    config: SubscribeCustomerConfig,
) -> SubscriptionResult<SubscribeCustomerResult> {
    // 1. Resolve plan
    let (plan, plan_created) = match config.plan {
        PlanReference::ExistingId(id) => {
            let plan: Option<Plan> = client.get_one(EntityType::Plans, &id).await?;
            match plan {
                Some(p) => (p, false),
                None => return Err(SubscriptionError::PlanNotFound(id)),
            }
        }
        PlanReference::NewPlan(plan_config) => {
            // Validate plan configuration
            plan_config.validate()?;

            let new_plan = json!({
                "merchant": config.merchant_id,
                "type": "recurring",
                "name": plan_config.name,
                "description": plan_config.description,
                "schedule": plan_config.schedule.to_plan_schedule() as i32,
                "scheduleFactor": plan_config.schedule_factor,
                "um": "actual",
                "amount": plan_config.amount,
                "maxFailures": plan_config.max_failures.unwrap_or(3)
            });
            let plan: Plan = client.create(EntityType::Plans, &new_plan).await?;
            (plan, true)
        }
    };

    // 2. Resolve token
    let (token, token_created) = match config.token {
        TokenReference::ExistingId(id) => {
            let token: Option<Token> = client.get_one(EntityType::Tokens, &id).await?;
            match token {
                Some(t) => (t, false),
                None => return Err(SubscriptionError::TokenNotFound(id)),
            }
        }
        TokenReference::NewToken(token_config) => {
            let create_token = CreateToken {
                customer: token_config.customer_id.parse().map_err(|_| {
                    SubscriptionError::CustomerNotFound(token_config.customer_id.clone())
                })?,
                payment: PaymentInfo {
                    method: token_config.method,
                    number: Some(token_config.number),
                    routing: token_config.routing,
                    expiration: token_config.expiration,
                    cvv: token_config.cvv,
                },
                login: None,
                expiration: None,
                name: None,
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            };
            let token: Token = client.create(EntityType::Tokens, &create_token).await?;
            (token, true)
        }
    };

    // 3. Calculate start date
    let start_date = config.start_date.unwrap_or_else(|| {
        let today = Utc::now().naive_utc().date();
        today.year() * 10000 + today.month() as i32 * 100 + today.day() as i32
    });

    // 4. Create subscription
    let origin = config
        .origin
        .unwrap_or(SubscriptionOrigin::ECommerce) as i32;

    let mut sub_json = json!({
        "plan": plan.id.as_str(),
        "start": start_date,
        "origin": origin
    });

    if let Some(end_date) = config.end_date {
        sub_json["finish"] = json!(end_date);
    }
    if let Some(tax) = config.tax {
        sub_json["tax"] = json!(tax);
    }
    if let Some(ref descriptor) = config.descriptor {
        sub_json["descriptor"] = json!(descriptor);
    }
    if let Some(ref txn_desc) = config.txn_description {
        sub_json["txnDescription"] = json!(txn_desc);
    }

    let subscription: Subscription = client.create(EntityType::Subscriptions, &sub_json).await?;

    // 5. Create subscription token link (links the token to the subscription)
    let sub_token_json = json!({
        "subscription": subscription.id.as_str(),
        "token": token.id.as_str()
    });
    let sub_token_result: Result<serde_json::Value, _> = client
        .create(EntityType::SubscriptionTokens, &sub_token_json)
        .await;

    if let Err(ref e) = sub_token_result {
        tracing::warn!(
            subscription_id = %subscription.id.as_str(),
            token_id = %token.id.as_str(),
            error = %e,
            "Failed to create subscription token link - subscription may not charge correctly"
        );
    }

    // 6. Charge immediately if requested
    let initial_transaction = if config.charge_immediately {
        let txn_json = json!({
            "merchant": config.merchant_id,
            "type": TransactionType::CreditCardSale as i32,
            "token": token.id.as_str(),
            "total": plan.amount.unwrap_or(0),
            "origin": config.origin.map(|o| o as i32).unwrap_or(2),
            "order": format!("SUB-{}", subscription.id.as_str()),
            "description": config.txn_description.as_deref().unwrap_or("Subscription payment")
        });

        let txn: Transaction = client.create(EntityType::Txns, &txn_json).await?;
        Some(txn)
    } else {
        None
    };

    Ok(SubscribeCustomerResult {
        subscription,
        plan,
        token,
        initial_transaction,
        plan_created,
        token_created,
    })
}

/// Get all payments for a subscription.
///
/// Returns payment history including total paid, payment count, and all transactions.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID
///
/// # Returns
///
/// A [`PaymentHistory`] with transaction details and summary statistics.
pub async fn payments_to_date(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<PaymentHistory> {
    // Query transactions for this subscription
    let search = SearchBuilder::new()
        .field("subscription", subscription_id)
        .build();

    let transactions: Vec<Transaction> = client
        .search(EntityType::Txns, &search)
        .await?;

    // Calculate statistics
    let mut total_paid: i64 = 0;
    let mut payment_count: i32 = 0;
    let mut failed_count: i32 = 0;
    let mut last_payment_date: Option<String> = None;
    let mut last_payment_amount: Option<i64> = None;

    for txn in &transactions {
        let is_successful = matches!(
            txn.status,
            Some(TransactionStatus::Approved)
                | Some(TransactionStatus::Captured)
                | Some(TransactionStatus::Settled)
        );

        if is_successful {
            payment_count += 1;
            if let Some(total) = txn.total {
                total_paid += total;
            }

            // Track most recent successful payment
            if last_payment_date.is_none()
                || txn.created.as_ref() > last_payment_date.as_ref()
            {
                last_payment_date = txn.created.clone();
                last_payment_amount = txn.total;
            }
        } else {
            failed_count += 1;
        }
    }

    Ok(PaymentHistory {
        total_paid,
        payment_count,
        failed_count,
        last_payment_date,
        last_payment_amount,
        transactions,
    })
}

/// Calculate the next payment date and amount for a subscription.
///
/// Uses the plan schedule and last payment date to determine when the next
/// payment will occur.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID
///
/// # Returns
///
/// A [`NextPayment`] with the predicted next payment details.
pub async fn next_payment(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<NextPayment> {
    // Get subscription with expanded plan
    let subscription: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let subscription = subscription
        .ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    // Get the plan
    let plan_id = subscription
        .plan
        .as_ref()
        .ok_or_else(|| SubscriptionError::CalculationError("Subscription has no plan".to_string()))?;

    let plan: Option<Plan> = client.get_one(EntityType::Plans, plan_id.as_str()).await?;
    let plan = plan.ok_or_else(|| SubscriptionError::PlanNotFound(plan_id.to_string()))?;

    // Get payment history to find last payment
    let history = payments_to_date(client, subscription_id).await?;

    let today = Utc::now().naive_utc().date();

    // Determine base date (last payment or subscription start)
    let base_date = if let Some(ref last_date) = history.last_payment_date {
        // Parse the datetime string (format: "YYYY-MM-DD HH:MM:SS.SSSS")
        if last_date.len() >= 10 {
            NaiveDate::parse_from_str(&last_date[0..10], "%Y-%m-%d")
                .ok()
                .unwrap_or(today)
        } else {
            today
        }
    } else if let Some(start) = subscription.start {
        parse_payrix_date(&start.to_string()).unwrap_or(today)
    } else {
        today
    };

    // Calculate next payment date based on schedule
    let schedule = plan.schedule.unwrap_or(PlanSchedule::Monthly);
    let factor = plan.schedule_factor.unwrap_or(1) as i64;

    let next_date = match schedule {
        PlanSchedule::Daily => base_date + Duration::days(factor),
        PlanSchedule::Weekly => base_date + Duration::weeks(factor),
        PlanSchedule::Monthly => {
            // Use Months for proper month arithmetic
            base_date + Months::new(factor as u32)
        }
        PlanSchedule::Annually => base_date + Months::new((12 * factor) as u32),
    };

    // If next date is in the past, calculate from today
    let next_date = if next_date <= today {
        calculate_next_from_today(today, schedule, factor as i32)
    } else {
        next_date
    };

    let days_until = (next_date - today).num_days();
    let state = SubscriptionState::from_subscription(&subscription, today);

    Ok(NextPayment {
        date: next_date,
        amount: plan.amount.unwrap_or(0),
        days_until,
        is_active: state == SubscriptionState::Active,
    })
}

/// Helper to calculate next payment date from today.
fn calculate_next_from_today(today: NaiveDate, schedule: PlanSchedule, factor: i32) -> NaiveDate {
    match schedule {
        PlanSchedule::Daily => today + Duration::days(factor as i64),
        PlanSchedule::Weekly => today + Duration::weeks(factor as i64),
        PlanSchedule::Monthly => today + Months::new(factor as u32),
        PlanSchedule::Annually => today + Months::new((12 * factor) as u32),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Cancel a subscription.
///
/// Sets `inactive = true` to permanently cancel the subscription.
/// Use [`pause_subscription`] for temporary suspension.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID to cancel
pub async fn cancel_subscription(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<Subscription> {
    // Verify subscription exists
    let existing: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let existing =
        existing.ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    if existing.inactive {
        return Err(SubscriptionError::InvalidState(
            "Subscription is already cancelled".to_string(),
        ));
    }

    let updated: Subscription = client
        .update(
            EntityType::Subscriptions,
            subscription_id,
            &json!({"inactive": 1}),
        )
        .await?;

    Ok(updated)
}

/// Pause a subscription.
///
/// Sets `frozen = true` to temporarily suspend billing.
/// Use [`resume_subscription`] to reactivate.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID to pause
pub async fn pause_subscription(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<Subscription> {
    // Verify subscription exists and check state
    let existing: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let existing =
        existing.ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    if existing.inactive {
        return Err(SubscriptionError::InvalidState(
            "Cannot pause a cancelled subscription".to_string(),
        ));
    }

    if existing.frozen {
        return Err(SubscriptionError::InvalidState(
            "Subscription is already paused".to_string(),
        ));
    }

    let updated: Subscription = client
        .update(
            EntityType::Subscriptions,
            subscription_id,
            &json!({"frozen": 1}),
        )
        .await?;

    Ok(updated)
}

/// Resume a paused subscription.
///
/// Sets `frozen = false` to reactivate billing.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID to resume
pub async fn resume_subscription(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<Subscription> {
    // Verify subscription exists and check state
    let existing: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let existing =
        existing.ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    if existing.inactive {
        return Err(SubscriptionError::InvalidState(
            "Cannot resume a cancelled subscription".to_string(),
        ));
    }

    if !existing.frozen {
        return Err(SubscriptionError::InvalidState(
            "Subscription is not paused".to_string(),
        ));
    }

    let updated: Subscription = client
        .update(
            EntityType::Subscriptions,
            subscription_id,
            &json!({"frozen": 0}),
        )
        .await?;

    Ok(updated)
}

/// Update the payment method for a subscription.
///
/// This updates the token associated with the subscription for future payments.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID
/// * `new_token` - The new token reference (existing ID or create new)
pub async fn update_payment_method(
    client: &PayrixClient,
    subscription_id: &str,
    new_token: TokenReference,
) -> SubscriptionResult<Subscription> {
    // Verify subscription exists
    let existing: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let existing =
        existing.ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    if existing.inactive {
        return Err(SubscriptionError::InvalidState(
            "Cannot update payment method for cancelled subscription".to_string(),
        ));
    }

    // Resolve the new token
    let token = match new_token {
        TokenReference::ExistingId(id) => {
            let token: Option<Token> = client.get_one(EntityType::Tokens, &id).await?;
            token.ok_or_else(|| SubscriptionError::TokenNotFound(id))?
        }
        TokenReference::NewToken(token_config) => {
            let create_token = CreateToken {
                customer: token_config.customer_id.parse().map_err(|_| {
                    SubscriptionError::CustomerNotFound(token_config.customer_id.clone())
                })?,
                payment: PaymentInfo {
                    method: token_config.method,
                    number: Some(token_config.number),
                    routing: token_config.routing,
                    expiration: token_config.expiration,
                    cvv: token_config.cvv,
                },
                login: None,
                expiration: None,
                name: None,
                description: None,
                custom: None,
                inactive: None,
                frozen: None,
            };
            client.create(EntityType::Tokens, &create_token).await?
        }
    };

    // Create a new subscription_token link
    let sub_token_json = json!({
        "subscription": subscription_id,
        "token": token.id.as_str()
    });

    let sub_token_result: Result<serde_json::Value, _> = client
        .create(EntityType::SubscriptionTokens, &sub_token_json)
        .await;

    if let Err(ref e) = sub_token_result {
        tracing::warn!(
            subscription_id = %subscription_id,
            token_id = %token.id.as_str(),
            error = %e,
            "Failed to create subscription token link for payment method update"
        );
    }

    // Re-fetch the subscription to get current state
    let updated: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    updated.ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))
}

/// Get detailed subscription status with payment history.
///
/// Returns comprehensive information about the subscription including
/// current state, payment history, and next payment prediction.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID
pub async fn get_subscription_status(
    client: &PayrixClient,
    subscription_id: &str,
) -> SubscriptionResult<SubscriptionStatus> {
    // Get subscription
    let subscription: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let subscription = subscription
        .ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    // Get plan if available
    let plan = if let Some(ref plan_id) = subscription.plan {
        client
            .get_one::<Plan>(EntityType::Plans, plan_id.as_str())
            .await?
    } else {
        None
    };

    // Get payment history
    let payment_summary = payments_to_date(client, subscription_id).await?;

    // Calculate state
    let today = Utc::now().naive_utc().date();
    let state = SubscriptionState::from_subscription(&subscription, today);

    // Calculate next payment if active
    let next_payment = if state == SubscriptionState::Active {
        next_payment(client, subscription_id).await.ok()
    } else {
        None
    };

    Ok(SubscriptionStatus {
        subscription,
        plan,
        state,
        payment_summary,
        next_payment,
    })
}

// ============================================================================
// Payer/Payee Functions
// ============================================================================

/// Get all active subscriptions for a customer.
///
/// This queries the subscription_tokens endpoint to find subscriptions linked
/// to the customer's payment tokens.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `customer_id` - The customer ID
///
/// # Note
///
/// This function queries subscription tokens for each of the customer's tokens
/// to find associated subscriptions. For customers with many tokens, this may
/// result in multiple API calls.
pub async fn get_active_subscriptions_for_customer(
    client: &PayrixClient,
    customer_id: &str,
) -> SubscriptionResult<Vec<Subscription>> {
    // First get all tokens for this customer
    let token_search = SearchBuilder::new()
        .field("customer", customer_id)
        .field_with_op("inactive", "0", SearchOperator::Equals)
        .build();

    let tokens: Vec<Token> = client
        .search(EntityType::Tokens, &token_search)
        .await?;

    if tokens.is_empty() {
        return Ok(vec![]);
    }

    // Build a list of token IDs
    let token_ids: Vec<String> = tokens.iter().map(|t| t.id.to_string()).collect();

    // Query subscription_tokens for these tokens
    let mut subscription_ids: Vec<String> = Vec::new();

    for token_id in &token_ids {
        let sub_token_search = SearchBuilder::new()
            .field("token", token_id)
            .build();

        let sub_tokens: Vec<serde_json::Value> = client
            .search(EntityType::SubscriptionTokens, &sub_token_search)
            .await
            .unwrap_or_default();

        for st in sub_tokens {
            if let Some(sub_id) = st.get("subscription").and_then(|v| v.as_str()) {
                if !subscription_ids.contains(&sub_id.to_string()) {
                    subscription_ids.push(sub_id.to_string());
                }
            }
        }
    }

    if subscription_ids.is_empty() {
        return Ok(vec![]);
    }

    // Fetch the actual subscriptions
    let mut subscriptions: Vec<Subscription> = Vec::new();
    for sub_id in &subscription_ids {
        if let Ok(Some(sub)) = client
            .get_one::<Subscription>(EntityType::Subscriptions, sub_id)
            .await
        {
            // Only include active subscriptions
            if !sub.inactive && !sub.frozen {
                subscriptions.push(sub);
            }
        }
    }

    Ok(subscriptions)
}

/// Get all subscribers for a plan (merchant view).
///
/// Returns all subscriptions associated with a given plan.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `plan_id` - The plan ID
pub async fn get_subscribers_for_plan(
    client: &PayrixClient,
    plan_id: &str,
) -> SubscriptionResult<Vec<Subscription>> {
    let search = SearchBuilder::new().field("plan", plan_id).build();

    let subscriptions: Vec<Subscription> = client
        .search(EntityType::Subscriptions, &search)
        .await
        .unwrap_or_default();

    Ok(subscriptions)
}

/// Calculate subscription revenue for a plan.
///
/// Returns revenue metrics including total collected, projected monthly/annual,
/// and subscriber counts.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `plan_id` - The plan ID
/// * `start_date` - Optional start date for the calculation period
/// * `end_date` - Optional end date for the calculation period
pub async fn calculate_subscription_revenue(
    client: &PayrixClient,
    plan_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> SubscriptionResult<SubscriptionRevenue> {
    // Get the plan
    let plan: Option<Plan> = client.get_one(EntityType::Plans, plan_id).await?;
    let plan = plan.ok_or_else(|| SubscriptionError::PlanNotFound(plan_id.to_string()))?;

    // Get all subscriptions for this plan
    let subscriptions = get_subscribers_for_plan(client, plan_id).await?;

    let today = Utc::now().naive_utc().date();
    let mut total_collected: i64 = 0;
    let mut active_subscribers: i32 = 0;
    let mut churned_subscribers: i32 = 0;

    for sub in &subscriptions {
        let state = SubscriptionState::from_subscription(sub, today);

        match state {
            SubscriptionState::Active | SubscriptionState::Pending => {
                active_subscribers += 1;
            }
            SubscriptionState::Cancelled => {
                churned_subscribers += 1;
            }
            _ => {}
        }

        // Get payment history for this subscription
        let history = payments_to_date(client, sub.id.as_str()).await?;

        // Filter transactions by date range if specified
        for txn in &history.transactions {
            let include = match (&start_date, &end_date, &txn.created) {
                (Some(start), Some(end), Some(created)) => {
                    if let Ok(txn_date) = NaiveDate::parse_from_str(&created[0..10], "%Y-%m-%d") {
                        txn_date >= *start && txn_date <= *end
                    } else {
                        true
                    }
                }
                (Some(start), None, Some(created)) => {
                    if let Ok(txn_date) = NaiveDate::parse_from_str(&created[0..10], "%Y-%m-%d") {
                        txn_date >= *start
                    } else {
                        true
                    }
                }
                (None, Some(end), Some(created)) => {
                    if let Ok(txn_date) = NaiveDate::parse_from_str(&created[0..10], "%Y-%m-%d") {
                        txn_date <= *end
                    } else {
                        true
                    }
                }
                _ => true,
            };

            if include {
                if let Some(total) = txn.total {
                    let is_successful = matches!(
                        txn.status,
                        Some(TransactionStatus::Approved)
                            | Some(TransactionStatus::Captured)
                            | Some(TransactionStatus::Settled)
                    );
                    if is_successful {
                        total_collected += total;
                    }
                }
            }
        }
    }

    // Calculate projections based on plan schedule and active subscribers
    let plan_amount = plan.amount.unwrap_or(0);
    let schedule = plan.schedule.unwrap_or(PlanSchedule::Monthly);
    let factor = plan.schedule_factor.unwrap_or(1) as i64;

    let payments_per_year: f64 = match schedule {
        PlanSchedule::Daily => 365.0 / factor as f64,
        PlanSchedule::Weekly => 52.0 / factor as f64,
        PlanSchedule::Monthly => 12.0 / factor as f64,
        PlanSchedule::Annually => 1.0 / factor as f64,
    };

    let projected_annual = (plan_amount as f64 * payments_per_year * active_subscribers as f64) as i64;
    let projected_monthly = projected_annual / 12;

    Ok(SubscriptionRevenue {
        total_collected,
        projected_monthly,
        projected_annual,
        active_subscribers,
        churned_subscribers,
    })
}

/// Get all upcoming payments for a customer across subscriptions.
///
/// Returns a list of upcoming payments sorted by date.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `customer_id` - The customer ID
/// * `days_ahead` - Number of days to look ahead (defaults to 30)
pub async fn get_upcoming_payments(
    client: &PayrixClient,
    customer_id: &str,
    days_ahead: Option<i32>,
) -> SubscriptionResult<Vec<UpcomingPayment>> {
    let days = days_ahead.unwrap_or(30);
    let today = Utc::now().naive_utc().date();
    let cutoff = today + Duration::days(days as i64);

    // Get active subscriptions for this customer
    let subscriptions = get_active_subscriptions_for_customer(client, customer_id).await?;

    let mut upcoming: Vec<UpcomingPayment> = Vec::new();

    for sub in subscriptions {
        // Get next payment for this subscription
        if let Ok(next) = next_payment(client, sub.id.as_str()).await {
            if next.is_active && next.date <= cutoff {
                // Get plan name
                let plan_name = if let Some(ref plan_id) = sub.plan {
                    client
                        .get_one::<Plan>(EntityType::Plans, plan_id.as_str())
                        .await
                        .ok()
                        .flatten()
                        .and_then(|p| p.name)
                } else {
                    None
                };

                upcoming.push(UpcomingPayment {
                    subscription_id: sub.id.to_string(),
                    plan_name,
                    date: next.date,
                    amount: next.amount,
                    days_until: next.days_until,
                });
            }
        }
    }

    // Sort by date
    upcoming.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(upcoming)
}

/// Retry a failed subscription payment.
///
/// Creates a new Sale transaction for the subscription with an optional amount override.
///
/// # Arguments
///
/// * `client` - The Payrix client
/// * `subscription_id` - The subscription ID
/// * `amount_override` - Optional amount override (uses plan amount if not specified)
pub async fn retry_failed_payment(
    client: &PayrixClient,
    subscription_id: &str,
    amount_override: Option<i64>,
) -> SubscriptionResult<Transaction> {
    // Get subscription
    let subscription: Option<Subscription> = client
        .get_one(EntityType::Subscriptions, subscription_id)
        .await?;

    let subscription = subscription
        .ok_or_else(|| SubscriptionError::SubscriptionNotFound(subscription_id.to_string()))?;

    let today = Utc::now().naive_utc().date();
    let state = SubscriptionState::from_subscription(&subscription, today);

    if state == SubscriptionState::Cancelled {
        return Err(SubscriptionError::InvalidState(
            "Cannot retry payment for cancelled subscription".to_string(),
        ));
    }

    // Get the plan for the amount
    let plan_id = subscription
        .plan
        .as_ref()
        .ok_or_else(|| SubscriptionError::CalculationError("Subscription has no plan".to_string()))?;

    let plan: Option<Plan> = client.get_one(EntityType::Plans, plan_id.as_str()).await?;
    let plan = plan.ok_or_else(|| SubscriptionError::PlanNotFound(plan_id.to_string()))?;

    let amount = amount_override.unwrap_or_else(|| plan.amount.unwrap_or(0));

    // Get a token for this subscription
    // Note: This is simplified - we'd ideally query subscription_tokens
    let payment_history = payments_to_date(client, subscription_id).await?;

    // Find a token from previous transactions
    let token_id = payment_history
        .transactions
        .iter()
        .find_map(|txn| txn.token.as_ref())
        .ok_or_else(|| {
            SubscriptionError::CalculationError(
                "No payment token found for subscription".to_string(),
            )
        })?;

    // Get merchant from plan
    let merchant_id = plan
        .merchant
        .as_ref()
        .ok_or_else(|| SubscriptionError::CalculationError("Plan has no merchant".to_string()))?;

    // Create retry transaction
    let txn_json = json!({
        "merchant": merchant_id.as_str(),
        "type": TransactionType::CreditCardSale as i32,
        "token": token_id.as_str(),
        "total": amount,
        "subscription": subscription_id,
        "origin": subscription.origin.map(|o| o as i32).unwrap_or(2),
        "order": format!("RETRY-{}", subscription_id),
        "description": "Subscription payment retry"
    });

    let transaction: Transaction = client.create(EntityType::Txns, &txn_json).await?;

    Ok(transaction)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn billing_schedule_conversion() {
        assert_eq!(
            BillingSchedule::Daily.to_plan_schedule(),
            PlanSchedule::Daily
        );
        assert_eq!(
            BillingSchedule::Weekly.to_plan_schedule(),
            PlanSchedule::Weekly
        );
        assert_eq!(
            BillingSchedule::Monthly.to_plan_schedule(),
            PlanSchedule::Monthly
        );
        assert_eq!(
            BillingSchedule::Annually.to_plan_schedule(),
            PlanSchedule::Annually
        );
    }

    #[test]
    fn billing_schedule_from_plan_schedule() {
        assert_eq!(
            BillingSchedule::from_plan_schedule(PlanSchedule::Daily),
            BillingSchedule::Daily
        );
        assert_eq!(
            BillingSchedule::from_plan_schedule(PlanSchedule::Weekly),
            BillingSchedule::Weekly
        );
        assert_eq!(
            BillingSchedule::from_plan_schedule(PlanSchedule::Monthly),
            BillingSchedule::Monthly
        );
        assert_eq!(
            BillingSchedule::from_plan_schedule(PlanSchedule::Annually),
            BillingSchedule::Annually
        );
    }

    #[test]
    fn subscription_state_active() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let sub = Subscription {
            id: "t1_sbn_test".parse().unwrap(),
            inactive: false,
            frozen: false,
            start: Some(20240101),
            finish: Some(20241231),
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            plan: None,
            statement_entity: None,
            first_txn: None,
            tax: None,
            descriptor: None,
            txn_description: None,
            order: None,
            origin: None,
            authentication: None,
            authentication_id: None,
            failures: None,
            max_failures: None,
            #[cfg(not(feature = "sqlx"))]
            invoices: None,
            #[cfg(not(feature = "sqlx"))]
            subscription_tokens: None,
        };

        assert_eq!(
            SubscriptionState::from_subscription(&sub, today),
            SubscriptionState::Active
        );
    }

    #[test]
    fn subscription_state_cancelled() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let sub = Subscription {
            id: "t1_sbn_test".parse().unwrap(),
            inactive: true,
            frozen: false,
            start: Some(20240101),
            finish: None,
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            plan: None,
            statement_entity: None,
            first_txn: None,
            tax: None,
            descriptor: None,
            txn_description: None,
            order: None,
            origin: None,
            authentication: None,
            authentication_id: None,
            failures: None,
            max_failures: None,
            #[cfg(not(feature = "sqlx"))]
            invoices: None,
            #[cfg(not(feature = "sqlx"))]
            subscription_tokens: None,
        };

        assert_eq!(
            SubscriptionState::from_subscription(&sub, today),
            SubscriptionState::Cancelled
        );
    }

    #[test]
    fn subscription_state_paused() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let sub = Subscription {
            id: "t1_sbn_test".parse().unwrap(),
            inactive: false,
            frozen: true,
            start: Some(20240101),
            finish: None,
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            plan: None,
            statement_entity: None,
            first_txn: None,
            tax: None,
            descriptor: None,
            txn_description: None,
            order: None,
            origin: None,
            authentication: None,
            authentication_id: None,
            failures: None,
            max_failures: None,
            #[cfg(not(feature = "sqlx"))]
            invoices: None,
            #[cfg(not(feature = "sqlx"))]
            subscription_tokens: None,
        };

        assert_eq!(
            SubscriptionState::from_subscription(&sub, today),
            SubscriptionState::Paused
        );
    }

    #[test]
    fn subscription_state_pending() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let sub = Subscription {
            id: "t1_sbn_test".parse().unwrap(),
            inactive: false,
            frozen: false,
            start: Some(20240701), // Future date
            finish: None,
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            plan: None,
            statement_entity: None,
            first_txn: None,
            tax: None,
            descriptor: None,
            txn_description: None,
            order: None,
            origin: None,
            authentication: None,
            authentication_id: None,
            failures: None,
            max_failures: None,
            #[cfg(not(feature = "sqlx"))]
            invoices: None,
            #[cfg(not(feature = "sqlx"))]
            subscription_tokens: None,
        };

        assert_eq!(
            SubscriptionState::from_subscription(&sub, today),
            SubscriptionState::Pending
        );
    }

    #[test]
    fn subscription_state_expired() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let sub = Subscription {
            id: "t1_sbn_test".parse().unwrap(),
            inactive: false,
            frozen: false,
            start: Some(20240101),
            finish: Some(20240601), // Past date
            created: None,
            modified: None,
            creator: None,
            modifier: None,
            plan: None,
            statement_entity: None,
            first_txn: None,
            tax: None,
            descriptor: None,
            txn_description: None,
            order: None,
            origin: None,
            authentication: None,
            authentication_id: None,
            failures: None,
            max_failures: None,
            #[cfg(not(feature = "sqlx"))]
            invoices: None,
            #[cfg(not(feature = "sqlx"))]
            subscription_tokens: None,
        };

        assert_eq!(
            SubscriptionState::from_subscription(&sub, today),
            SubscriptionState::Expired
        );
    }

    #[test]
    fn payment_history_total_dollars() {
        let history = PaymentHistory {
            total_paid: 2999,
            payment_count: 1,
            failed_count: 0,
            last_payment_date: None,
            last_payment_amount: None,
            transactions: vec![],
        };

        assert!((history.total_paid_dollars() - 29.99).abs() < 0.001);
    }

    #[test]
    fn next_payment_amount_dollars() {
        let next = NextPayment {
            date: NaiveDate::from_ymd_opt(2024, 7, 15).unwrap(),
            amount: 4999,
            days_until: 30,
            is_active: true,
        };

        assert!((next.amount_dollars() - 49.99).abs() < 0.001);
    }

    #[test]
    fn calculate_next_from_today_daily() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let next = calculate_next_from_today(today, PlanSchedule::Daily, 1);
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 6, 16).unwrap());
    }

    #[test]
    fn calculate_next_from_today_weekly() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let next = calculate_next_from_today(today, PlanSchedule::Weekly, 1);
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 6, 22).unwrap());
    }

    #[test]
    fn calculate_next_from_today_monthly() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let next = calculate_next_from_today(today, PlanSchedule::Monthly, 1);
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 7, 15).unwrap());
    }

    #[test]
    fn calculate_next_from_today_annually() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let next = calculate_next_from_today(today, PlanSchedule::Annually, 1);
        assert_eq!(next, NaiveDate::from_ymd_opt(2025, 6, 15).unwrap());
    }

    #[test]
    fn calculate_next_with_factor() {
        let today = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();

        // Every 2 months
        let next = calculate_next_from_today(today, PlanSchedule::Monthly, 2);
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 8, 15).unwrap());

        // Every 2 weeks
        let next = calculate_next_from_today(today, PlanSchedule::Weekly, 2);
        assert_eq!(next, NaiveDate::from_ymd_opt(2024, 6, 29).unwrap());
    }
}
