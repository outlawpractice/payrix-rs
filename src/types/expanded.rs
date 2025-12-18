//! Expanded entity types with embedded relationships.
//!
//! When using the Payrix API's `expand[]` query parameter, related entities
//! are embedded directly in the response instead of just returning their IDs.
//! These types handle the expanded response format.
//!
//! # Nested Expansions
//!
//! Some expansions can be nested. For example, `token|customer` expands the
//! token AND the customer nested inside the token:
//!
//! ```text
//! expand[token][][customer][]
//! ```
//!
//! Results in:
//! ```json
//! {
//!   "token": {
//!     "id": "t1_tok_xxx",
//!     "customer": {
//!       "id": "t1_cus_xxx",
//!       "first": "John",
//!       ...
//!     }
//!   }
//! }
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! // Get a transaction with payment, token, and customer expanded
//! let txn: TransactionExpanded = client
//!     .get_transaction_full(txn_id)
//!     .await?;
//!
//! // Access expanded data
//! if let Some(ref payment) = txn.payment {
//!     println!("Card: {} ending in {}", payment.method, payment.last4);
//! }
//!
//! if let Some(ref token) = txn.token {
//!     if let Some(ref customer) = token.customer {
//!         println!("Customer: {} {}", customer.first, customer.last);
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};

use super::{
    batch::Platform, bool_from_int_default_false, deserialize_optional_i32, deserialize_string_or_int,
    BatchStatus, ChargebackCycle, ChargebackPaymentMethod, ChargebackStatusValue, Member,
    Payment, PaymentMethod, PayrixId, Plan, PlanSchedule, PlanType, PlanUm, Subscription,
    SubscriptionOrigin, TokenStatus, Transaction, TransactionStatus, TransactionType,
};

// =============================================================================
// TokenExpanded
// =============================================================================

/// A token with expanded relationships.
///
/// Used when expanding `token` with nested expansions like `token|customer`.
/// The `customer` field becomes the full `Customer` object instead of just
/// a `PayrixId`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this token.
    pub id: PayrixId,

    /// The date and time this token was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this token was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this token.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this token.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Token Data
    // -------------------------------------------------------------------------

    /// The token string value used for transactions.
    #[serde(default)]
    pub token: Option<String>,

    /// Token status (pending/ready).
    #[serde(default)]
    pub status: Option<TokenStatus>,

    /// Card/account expiration in MMYY format.
    #[serde(default)]
    pub expiration: Option<String>,

    /// Token name.
    #[serde(default)]
    pub name: Option<String>,

    /// Token description.
    #[serde(default)]
    pub description: Option<String>,

    /// Custom data.
    #[serde(default)]
    pub custom: Option<String>,

    /// Whether this token is inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this token is frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// Entry mode.
    #[serde(default)]
    pub entry_mode: Option<i32>,

    /// Origin of the token.
    #[serde(default)]
    pub origin: Option<String>,

    /// Omnitoken value.
    #[serde(default)]
    pub omnitoken: Option<String>,

    /// Auth token customer reference.
    #[serde(default)]
    pub auth_token_customer: Option<String>,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Expanded payment details.
    ///
    /// Contains card/account information like BIN, last4, routing number.
    /// Only populated when expanding `payment`.
    #[serde(default)]
    pub payment: Option<Payment>,

    /// Expanded customer.
    /// Customer ID.
    ///
    /// Note: The API returns customer as an ID string even when using nested expansion.
    /// Use `client.get_customer_expanded()` to fetch full customer details.
    #[serde(default)]
    pub customer: Option<PayrixId>,
}

impl TokenExpanded {
    /// Returns the payment method if available.
    pub fn payment_method(&self) -> Option<PaymentMethod> {
        self.payment.as_ref().and_then(|p| p.method)
    }

    /// Returns the card display string (e.g., "Visa ending in 1111").
    pub fn card_display(&self) -> Option<String> {
        self.payment.as_ref().map(|p| p.display())
    }

    /// Returns the customer ID if available.
    ///
    /// Note: Customer data is not expanded in token responses.
    /// Use `client.get_customer_expanded()` to fetch full customer details.
    pub fn customer_id(&self) -> Option<&str> {
        self.customer.as_ref().map(|c| c.as_str())
    }
}

// =============================================================================
// TransactionExpanded
// =============================================================================

/// A transaction with commonly expanded relationships.
///
/// This type is returned by convenience methods like `get_transaction_full()`
/// that expand payment, token, customer, and other related entities in a
/// single API call.
///
/// # Fields
///
/// The transaction includes all standard transaction fields plus expanded
/// versions of related entities. When a relationship is expanded, you get
/// the full object instead of just an ID.
///
/// # Example
///
/// ```rust,ignore
/// let txn = client.get_transaction_full(txn_id).await?;
///
/// // Transaction data
/// println!("Amount: ${:.2}", txn.total.unwrap_or(0) as f64 / 100.0);
/// println!("Status: {:?}", txn.status);
///
/// // Expanded payment
/// if let Some(ref payment) = txn.payment {
///     println!("Card: {}", payment.display());
/// }
///
/// // Expanded token with nested customer
/// if let Some(ref token) = txn.token {
///     println!("Token: {}", token.token.as_deref().unwrap_or("N/A"));
///     if let Some(ref customer) = token.customer {
///         println!("Customer: {} {}", customer.first, customer.last);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this transaction.
    pub id: PayrixId,

    /// The date and time this transaction was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this transaction was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this transaction.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this transaction.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Transaction Type and Status
    // -------------------------------------------------------------------------

    /// The transaction type (Sale, Auth, Capture, Refund, etc.).
    #[serde(default, rename = "type")]
    pub txn_type: Option<TransactionType>,

    /// The transaction status.
    #[serde(default)]
    pub status: Option<TransactionStatus>,

    /// Transaction origin (ecommerce, terminal, etc.).
    #[serde(default)]
    pub origin: Option<i32>,

    // -------------------------------------------------------------------------
    // Amounts (in cents)
    // -------------------------------------------------------------------------

    /// Total transaction amount in cents.
    #[serde(default)]
    pub total: Option<i64>,

    /// Approved amount in cents.
    #[serde(default)]
    pub approved: Option<i64>,

    /// Original approved amount in cents.
    #[serde(default)]
    pub original_approved: Option<i64>,

    /// Refunded amount in cents.
    #[serde(default)]
    pub refunded: Option<i64>,

    /// Reserved amount in cents.
    #[serde(default)]
    pub reserved: Option<i64>,

    // -------------------------------------------------------------------------
    // Transaction Details
    // -------------------------------------------------------------------------

    /// Authorization code.
    #[serde(default)]
    pub authorization: Option<String>,

    /// Auth code from processor.
    #[serde(default)]
    pub auth_code: Option<String>,

    /// Currency code (e.g., "USD").
    #[serde(default)]
    pub currency: Option<String>,

    /// Transaction descriptor (appears on statements).
    #[serde(default)]
    pub descriptor: Option<String>,

    /// Transaction description.
    #[serde(default)]
    pub description: Option<String>,

    /// Card-on-file type.
    #[serde(default)]
    pub cof_type: Option<String>,

    /// Card expiration in MMYY format.
    #[serde(default)]
    pub expiration: Option<String>,

    /// CVV response code.
    #[serde(default)]
    pub cvv: Option<i32>,

    /// Processing platform.
    #[serde(default)]
    pub platform: Option<String>,

    // -------------------------------------------------------------------------
    // Date Fields (API sometimes returns these as integers in YYYYMMDD format)
    // -------------------------------------------------------------------------

    /// Date/time captured.
    #[serde(default, deserialize_with = "deserialize_string_or_int")]
    pub captured: Option<String>,

    /// Date/time settled.
    #[serde(default, deserialize_with = "deserialize_string_or_int")]
    pub settled: Option<String>,

    /// Date/time returned.
    #[serde(default, deserialize_with = "deserialize_string_or_int")]
    pub returned: Option<String>,

    /// Date funded (integer in YYYYMMDD format).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub funded: Option<i32>,

    // -------------------------------------------------------------------------
    // Customer Info (from transaction, not expanded customer)
    // -------------------------------------------------------------------------

    /// First name.
    #[serde(default)]
    pub first: Option<String>,

    /// Middle name.
    #[serde(default)]
    pub middle: Option<String>,

    /// Last name.
    #[serde(default)]
    pub last: Option<String>,

    /// Email address.
    #[serde(default)]
    pub email: Option<String>,

    /// Phone number.
    #[serde(default)]
    pub phone: Option<String>,

    // -------------------------------------------------------------------------
    // Address
    // -------------------------------------------------------------------------

    /// Address line 1.
    #[serde(default)]
    pub address1: Option<String>,

    /// Address line 2.
    #[serde(default)]
    pub address2: Option<String>,

    /// City.
    #[serde(default)]
    pub city: Option<String>,

    /// State.
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code.
    #[serde(default)]
    pub zip: Option<String>,

    /// Country.
    #[serde(default)]
    pub country: Option<String>,

    // -------------------------------------------------------------------------
    // Status Flags
    // -------------------------------------------------------------------------

    /// Whether this transaction is inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this transaction is frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// Whether funding is enabled.
    #[serde(default)]
    pub funding_enabled: Option<i32>,

    // -------------------------------------------------------------------------
    // Relationship IDs (non-expanded)
    // -------------------------------------------------------------------------

    /// Batch ID (not expanded).
    #[serde(default)]
    pub batch: Option<PayrixId>,

    /// Related transaction ID (for refunds, etc.).
    #[serde(default)]
    pub fortxn: Option<PayrixId>,

    /// Source transaction ID (for reauthorizations).
    #[serde(default)]
    pub fromtxn: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Expanded payment details.
    ///
    /// Contains card/account information like BIN, last4, routing number.
    #[serde(default)]
    pub payment: Option<Payment>,

    /// Expanded token with optional nested customer.
    ///
    /// When using `token|customer` expansion, the customer is nested here.
    #[serde(default)]
    pub token: Option<TokenExpanded>,

    /// Merchant ID (not expanded - use separate query if full merchant data needed).
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Expanded subscription.
    #[serde(default)]
    pub subscription: Option<Subscription>,
}

impl TransactionExpanded {
    /// Returns the transaction amount as a decimal (dollars, not cents).
    pub fn amount_dollars(&self) -> f64 {
        self.total.unwrap_or(0) as f64 / 100.0
    }

    /// Returns the approved amount as a decimal (dollars, not cents).
    pub fn approved_dollars(&self) -> f64 {
        self.approved.unwrap_or(0) as f64 / 100.0
    }

    /// Returns the payment display string if payment is expanded.
    pub fn payment_display(&self) -> Option<String> {
        self.payment.as_ref().map(|p| p.display())
    }

    /// Returns the customer name from the transaction's first/last fields.
    ///
    /// Note: Customer object is not expanded in transaction responses.
    pub fn customer_name(&self) -> Option<String> {
        let first = self.first.as_deref().unwrap_or("");
        let last = self.last.as_deref().unwrap_or("");
        let name = format!("{} {}", first, last).trim().to_string();
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    /// Returns the customer ID from the expanded token.
    ///
    /// Note: Customer data is not expanded. Use `client.get_customer_expanded()`
    /// to fetch full customer details.
    pub fn customer_id(&self) -> Option<&str> {
        self.token.as_ref().and_then(|t| t.customer_id())
    }

    /// Returns true if this transaction was approved.
    pub fn is_approved(&self) -> bool {
        matches!(self.status, Some(TransactionStatus::Captured))
    }
}

// =============================================================================
// CustomerExpanded
// =============================================================================

/// A customer with expanded relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerExpanded {
    // Core fields - flatten the base customer
    /// The ID of this customer.
    pub id: PayrixId,

    /// The date and time this customer was created.
    #[serde(default)]
    pub created: Option<String>,

    /// First name.
    #[serde(default)]
    pub first: Option<String>,

    /// Last name.
    #[serde(default)]
    pub last: Option<String>,

    /// Email address.
    #[serde(default)]
    pub email: Option<String>,

    /// Merchant ID.
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Whether this customer is inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    // Expanded relationships

    /// Expanded tokens.
    #[serde(default)]
    pub tokens: Option<Vec<TokenExpanded>>,

    /// Expanded invoices (as JSON for now).
    #[serde(default)]
    pub invoices: Option<Vec<serde_json::Value>>,
}

// =============================================================================
// SubscriptionExpanded
// =============================================================================

/// A subscription with expanded relationships.
///
/// Subscriptions can expand the `plan` relationship to get full plan details
/// in a single API call.
///
/// # Example
///
/// ```rust,ignore
/// let sub = client.get_subscription_expanded(sub_id).await?;
///
/// if let Some(ref plan) = sub.plan {
///     println!("Plan: {} - ${:.2}/month",
///         plan.name.as_deref().unwrap_or("Unknown"),
///         plan.amount.unwrap_or(0) as f64 / 100.0);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this subscription.
    pub id: PayrixId,

    /// The date and time this subscription was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this subscription was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this subscription.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this subscription.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Subscription Data
    // -------------------------------------------------------------------------

    /// Statement entity for billing.
    #[serde(default)]
    pub statement_entity: Option<PayrixId>,

    /// First transaction processed through this subscription.
    #[serde(default)]
    pub first_txn: Option<PayrixId>,

    /// Start date (YYYYMMDD format).
    #[serde(default)]
    pub start: Option<i32>,

    /// End date (YYYYMMDD format).
    #[serde(default)]
    pub finish: Option<i32>,

    /// Tax amount in cents.
    #[serde(default)]
    pub tax: Option<i64>,

    /// Statement descriptor.
    #[serde(default)]
    pub descriptor: Option<String>,

    /// Transaction description.
    #[serde(default)]
    pub txn_description: Option<String>,

    /// Order reference.
    #[serde(default)]
    pub order: Option<String>,

    /// Transaction origin.
    #[serde(default)]
    pub origin: Option<SubscriptionOrigin>,

    /// 3D Secure authentication token.
    #[serde(default)]
    pub authentication: Option<String>,

    /// 3D Secure authentication ID.
    #[serde(default)]
    pub authentication_id: Option<String>,

    /// Current consecutive payment failures.
    #[serde(default)]
    pub failures: Option<i32>,

    /// Maximum allowed consecutive failures.
    #[serde(default)]
    pub max_failures: Option<i32>,

    /// Whether this subscription is inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this subscription is frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Expanded plan.
    ///
    /// Contains full plan details including schedule, amount, and billing terms.
    #[serde(default)]
    pub plan: Option<Plan>,
}

impl SubscriptionExpanded {
    /// Returns the plan amount in dollars.
    pub fn plan_amount_dollars(&self) -> Option<f64> {
        self.plan
            .as_ref()
            .and_then(|p| p.amount)
            .map(|a| a as f64 / 100.0)
    }

    /// Returns the plan name if available.
    pub fn plan_name(&self) -> Option<&str> {
        self.plan.as_ref().and_then(|p| p.name.as_deref())
    }
}

// =============================================================================
// PlanExpanded
// =============================================================================

/// A plan with expanded relationships.
///
/// Plans can expand `merchant` and `subscriptions` relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this plan.
    pub id: PayrixId,

    /// The date and time this plan was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this plan was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this plan.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this plan.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Plan Data
    // -------------------------------------------------------------------------

    /// Billing ID.
    #[serde(default)]
    pub billing: Option<PayrixId>,

    /// Plan type (recurring or installment).
    #[serde(default, rename = "type")]
    pub plan_type: Option<PlanType>,

    /// Plan name.
    #[serde(default)]
    pub name: Option<String>,

    /// Plan description.
    #[serde(default)]
    pub description: Option<String>,

    /// Transaction description.
    #[serde(default)]
    pub txn_description: Option<String>,

    /// Order reference.
    #[serde(default)]
    pub order: Option<String>,

    /// Billing schedule (daily, weekly, monthly, annually).
    #[serde(default)]
    pub schedule: Option<PlanSchedule>,

    /// Schedule multiplier.
    #[serde(default)]
    pub schedule_factor: Option<i32>,

    /// Unit of measure (actual cents or percentage).
    #[serde(default)]
    pub um: Option<PlanUm>,

    /// Amount in cents.
    #[serde(default)]
    pub amount: Option<i64>,

    /// Maximum consecutive failures before inactivating.
    #[serde(default)]
    pub max_failures: Option<i32>,

    /// Whether this plan is inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this plan is frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Merchant ID (not expanded - use separate query if full merchant data needed).
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Expanded subscriptions.
    #[serde(default)]
    pub subscriptions: Option<Vec<Subscription>>,
}

impl PlanExpanded {
    /// Returns the plan amount in dollars.
    pub fn amount_dollars(&self) -> f64 {
        self.amount.unwrap_or(0) as f64 / 100.0
    }

    /// Returns the number of active subscriptions.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions
            .as_ref()
            .map(|s| s.iter().filter(|sub| !sub.inactive).count())
            .unwrap_or(0)
    }
}

// =============================================================================
// ChargebackExpanded
// =============================================================================

/// A chargeback with expanded relationships.
///
/// Chargebacks can expand `txn` (transaction) and `merchant` relationships
/// to get full details in a single API call.
///
/// # Example
///
/// ```rust,ignore
/// let cb = client.get_chargeback_expanded(chargeback_id).await?;
///
/// if let Some(ref txn) = cb.txn {
///     println!("Original transaction: {} for ${:.2}",
///         txn.id.as_str(),
///         txn.total.unwrap_or(0) as f64 / 100.0);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargebackExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this chargeback.
    pub id: PayrixId,

    /// The date and time this chargeback was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this chargeback was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this chargeback.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this chargeback.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Chargeback Data
    // -------------------------------------------------------------------------

    /// Merchant's processing MID.
    #[serde(default)]
    pub mid: Option<String>,

    /// Chargeback description.
    #[serde(default)]
    pub description: Option<String>,

    /// Total amount in cents.
    #[serde(default)]
    pub total: Option<i64>,

    /// Represented total in cents.
    #[serde(default)]
    pub represented_total: Option<i64>,

    /// Current cycle/stage.
    #[serde(default)]
    pub cycle: Option<ChargebackCycle>,

    /// Currency code.
    #[serde(default)]
    pub currency: Option<String>,

    /// Processing platform.
    #[serde(default)]
    pub platform: Option<String>,

    /// Payment method.
    #[serde(default)]
    pub payment_method: Option<ChargebackPaymentMethod>,

    /// Processing reference number.
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,

    /// Reason description.
    #[serde(default)]
    pub reason: Option<String>,

    /// Reason code.
    #[serde(default)]
    pub reason_code: Option<String>,

    /// Date issued (YYYYMMDD).
    #[serde(default)]
    pub issued: Option<i32>,

    /// Date received (YYYYMMDD).
    #[serde(default)]
    pub received: Option<i32>,

    /// Reply deadline (YYYYMMDD).
    #[serde(default)]
    pub reply: Option<i32>,

    /// Bank reference number.
    #[serde(default)]
    pub bank_ref: Option<String>,

    /// Chargeback reference number.
    #[serde(default)]
    pub chargeback_ref: Option<String>,

    /// Current status.
    #[serde(default)]
    pub status: Option<ChargebackStatusValue>,

    /// Last status change ID.
    #[serde(default)]
    pub last_status_change: Option<PayrixId>,

    /// Whether actionable.
    #[serde(default, with = "bool_from_int_default_false")]
    pub actionable: bool,

    /// Whether shadowed.
    #[serde(default, with = "bool_from_int_default_false")]
    pub shadow: bool,

    /// Whether inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Expanded transaction.
    ///
    /// The original transaction that was disputed.
    #[serde(default)]
    pub txn: Option<Transaction>,

    /// Merchant ID (not expanded - use separate query if full merchant data needed).
    #[serde(default)]
    pub merchant: Option<PayrixId>,
}

impl ChargebackExpanded {
    /// Returns the chargeback amount in dollars.
    pub fn amount_dollars(&self) -> f64 {
        self.total.unwrap_or(0) as f64 / 100.0
    }

    /// Returns the original transaction amount in dollars.
    pub fn original_transaction_amount(&self) -> Option<f64> {
        self.txn.as_ref().and_then(|t| t.total).map(|a| a as f64 / 100.0)
    }

    /// Returns true if this chargeback can still be responded to.
    pub fn is_actionable(&self) -> bool {
        self.actionable && matches!(self.status, Some(ChargebackStatusValue::Open))
    }

    /// Returns the merchant ID if available.
    ///
    /// Note: Merchant data is not expanded in chargeback responses.
    /// Use `client.get_merchant_expanded()` to fetch full merchant details.
    pub fn merchant_id(&self) -> Option<&str> {
        self.merchant.as_ref().map(|m| m.as_str())
    }
}

// =============================================================================
// BatchExpanded
// =============================================================================

/// A batch with expanded relationships.
///
/// Batches can expand `merchant` and `txns` (transactions) relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this batch.
    pub id: PayrixId,

    /// The date and time this batch was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this batch was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this batch.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this batch.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Batch Data
    // -------------------------------------------------------------------------

    /// Batch date.
    #[serde(default)]
    pub date: Option<String>,

    /// Processing date.
    #[serde(default)]
    pub processing_date: Option<String>,

    /// Processing ID.
    #[serde(default)]
    pub processing_id: Option<String>,

    /// Processing platform.
    #[serde(default)]
    pub platform: Option<Platform>,

    /// Batch status (open/closed).
    #[serde(default)]
    pub status: Option<BatchStatus>,

    /// Reference code.
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,

    /// Client reference code.
    #[serde(default)]
    pub client_ref: Option<String>,

    /// Close time.
    #[serde(default)]
    pub close_time: Option<String>,

    /// Whether inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Merchant ID (not expanded - use separate query if full merchant data needed).
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Expanded transactions in this batch.
    #[serde(default)]
    pub txns: Option<Vec<Transaction>>,
}

impl BatchExpanded {
    /// Returns the number of transactions in this batch.
    pub fn transaction_count(&self) -> usize {
        self.txns.as_ref().map(|t| t.len()).unwrap_or(0)
    }

    /// Returns the total amount of all transactions in dollars.
    pub fn total_amount_dollars(&self) -> f64 {
        self.txns
            .as_ref()
            .map(|txns| txns.iter().filter_map(|t| t.total).sum::<i64>())
            .unwrap_or(0) as f64
            / 100.0
    }

    /// Returns true if this batch is still open for transactions.
    pub fn is_open(&self) -> bool {
        matches!(self.status, Some(BatchStatus::Open))
    }

    /// Returns the merchant ID if available.
    ///
    /// Note: Merchant data is not expanded in batch responses.
    /// Use `client.get_merchant_expanded()` to fetch full merchant details.
    pub fn merchant_id(&self) -> Option<&str> {
        self.merchant.as_ref().map(|m| m.as_str())
    }
}

// =============================================================================
// MerchantExpanded
// =============================================================================

/// A merchant with expanded relationships.
///
/// Merchants can expand the `members` relationship to get beneficial owners
/// and control persons in a single API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantExpanded {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this merchant.
    pub id: PayrixId,

    /// The date and time this merchant was created.
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time this merchant was last modified.
    #[serde(default)]
    pub modified: Option<String>,

    /// The login that created this merchant.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The login that last modified this merchant.
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Merchant Data
    // -------------------------------------------------------------------------

    /// DBA (Doing Business As) name.
    #[serde(default)]
    pub dba: Option<String>,

    /// Legal business name.
    #[serde(default)]
    pub name: Option<String>,

    /// Entity ID.
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant email.
    #[serde(default)]
    pub email: Option<String>,

    /// Merchant phone.
    #[serde(default)]
    pub phone: Option<String>,

    /// Website URL.
    #[serde(default)]
    pub website: Option<String>,

    /// Address line 1.
    #[serde(default)]
    pub address1: Option<String>,

    /// Address line 2.
    #[serde(default)]
    pub address2: Option<String>,

    /// City.
    #[serde(default)]
    pub city: Option<String>,

    /// State.
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code.
    #[serde(default)]
    pub zip: Option<String>,

    /// Country.
    #[serde(default)]
    pub country: Option<String>,

    /// Timezone.
    #[serde(default)]
    pub timezone: Option<String>,

    /// MCC (Merchant Category Code).
    #[serde(default)]
    pub mcc: Option<String>,

    /// Merchant status.
    #[serde(default)]
    pub status: Option<i32>,

    /// Whether inactive.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether frozen.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    // -------------------------------------------------------------------------
    // Expanded Relationships
    // -------------------------------------------------------------------------

    /// Expanded members (beneficial owners, control persons).
    #[serde(default)]
    pub members: Option<Vec<Member>>,
}

impl MerchantExpanded {
    /// Returns the number of members.
    pub fn member_count(&self) -> usize {
        self.members.as_ref().map(|m| m.len()).unwrap_or(0)
    }

    /// Returns the primary member if one exists.
    pub fn primary_member(&self) -> Option<&Member> {
        self.members
            .as_ref()
            .and_then(|members| members.iter().find(|m| m.primary))
    }

    /// Returns total ownership percentage of all members.
    pub fn total_ownership_percent(&self) -> f64 {
        self.members
            .as_ref()
            .map(|members| members.iter().filter_map(|m| m.ownership).sum::<i32>())
            .unwrap_or(0) as f64
            / 100.0
    }

    /// Returns a display name for the merchant (DBA or name or "Unknown").
    pub fn display_name(&self) -> String {
        self.dba
            .as_ref()
            .or(self.name.as_ref())
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // =========================================================================
    // TokenExpanded Tests
    // =========================================================================

    #[test]
    fn test_token_expanded_with_minimal_fields() {
        // Test that TokenExpanded works with only required fields
        let json = json!({
            "id": "t1_tok_test123456789012345678"
        });

        let token: TokenExpanded = serde_json::from_value(json).unwrap();

        assert!(token.id.as_str().starts_with("t1_tok_"));
        assert!(token.payment.is_none());
        assert!(token.customer.is_none());
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    #[test]
    fn test_token_expanded_with_expansions() {
        let json = json!({
            "id": "t1_tok_694380e35c8eca506eb3856",
            "token": "36720364621c45c3227c95022e527839",
            "status": "ready",
            "expiration": "1229",
            "inactive": 1,
            "frozen": 0,
            "payment": {
                "bin": "411111",
                "method": 2,
                "number": "1111"
            },
            "customer": "t1_cus_694380e3086a60cfae6d1eb"
        });

        let token: TokenExpanded = serde_json::from_value(json).unwrap();

        // Core fields
        assert!(token.id.as_str().starts_with("t1_tok_"));
        assert!(token.token.is_some());
        assert_eq!(token.status, Some(TokenStatus::Ready));
        assert!(token.inactive);

        // Payment expansion should be present
        assert!(token.payment.is_some());
        let payment = token.payment.as_ref().unwrap();
        assert_eq!(payment.method, Some(PaymentMethod::Visa));
        assert_eq!(payment.bin.as_deref(), Some("411111"));

        // Customer should be an ID (not expanded by API)
        assert!(token.customer.is_some());
        assert!(token.customer_id().unwrap().starts_with("t1_cus_"));

        // Convenience methods
        assert_eq!(token.payment_method(), Some(PaymentMethod::Visa));
        assert!(token.customer_id().is_some());
    }

    #[test]
    fn test_token_expanded_handles_unknown_fields() {
        // API may return fields not in our schema - should not fail
        let json = json!({
            "id": "t1_tok_test123456789012345678",
            "status": "ready",
            "future_field": "should be ignored",
            "another_unknown": 12345
        });

        let result: Result<TokenExpanded, _> = serde_json::from_value(json);
        assert!(result.is_ok(), "Should handle unknown fields gracefully");

        let token = result.unwrap();
        assert_eq!(token.status, Some(TokenStatus::Ready));
    }

    // =========================================================================
    // TransactionExpanded Tests
    // =========================================================================

    #[test]
    fn test_transaction_expanded_with_minimal_fields() {
        let json = json!({
            "id": "t1_txn_test123456789012345678"
        });

        let txn: TransactionExpanded = serde_json::from_value(json).unwrap();

        assert!(txn.id.as_str().starts_with("t1_txn_"));
        assert!(txn.payment.is_none());
        assert!(txn.token.is_none());
        assert!(txn.merchant.is_none());
        assert_eq!(txn.amount_dollars(), 0.0);
    }

    #[test]
    fn test_transaction_expanded_with_all_expansions() {
        let json = json!({
            "id": "t1_txn_694380e3e1bd4ad74cdf956",
            "type": 1,
            "status": 3,
            "total": 1000,
            "approved": 1000,
            "currency": "USD",
            "first": "John",
            "last": "Doe",
            "payment": {
                "bin": "411111",
                "method": 2,
                "number": "1111"
            },
            "token": {
                "id": "t1_tok_694380e35c8eca506eb3856",
                "token": "abc123",
                "customer": "t1_cus_694380e3086a60cfae6d1eb"
            },
            "merchant": "t1_mer_test123456789012345678"
        });

        let txn: TransactionExpanded = serde_json::from_value(json).unwrap();

        // Core fields
        assert!(txn.id.as_str().starts_with("t1_txn_"));
        assert_eq!(txn.total, Some(1000));
        assert_eq!(txn.amount_dollars(), 10.0);

        // Payment expansion (payment IS expanded as an object)
        assert!(txn.payment.is_some());
        assert_eq!(txn.payment.as_ref().unwrap().method, Some(PaymentMethod::Visa));

        // Token expansion with nested customer ID
        assert!(txn.token.is_some());
        let token = txn.token.as_ref().unwrap();
        assert!(token.customer.is_some());
        assert!(token.customer_id().unwrap().starts_with("t1_cus_"));

        // Merchant is an ID (not expanded by API)
        assert!(txn.merchant.is_some());
        assert!(txn.merchant.as_ref().unwrap().as_str().starts_with("t1_mer_"));

        // Convenience methods (customer_name from first/last fields)
        assert_eq!(txn.customer_name(), Some("John Doe".to_string()));
        assert!(txn.payment_display().is_some());
    }

    #[test]
    fn test_transaction_expanded_amount_calculation() {
        let json = json!({
            "id": "t1_txn_test123456789012345678",
            "total": 12345
        });

        let txn: TransactionExpanded = serde_json::from_value(json).unwrap();
        assert_eq!(txn.amount_dollars(), 123.45);
    }

    // =========================================================================
    // CustomerExpanded Tests
    // =========================================================================

    #[test]
    fn test_customer_expanded_with_tokens_array() {
        let json = json!({
            "id": "t1_cus_test123456789012345678",
            "first": "Jane",
            "last": "Smith",
            "tokens": [
                {"id": "t1_tok_111111111111111111111", "status": "ready"},
                {"id": "t1_tok_222222222222222222222", "status": "pending"}
            ]
        });

        let customer: CustomerExpanded = serde_json::from_value(json).unwrap();

        assert!(customer.id.as_str().starts_with("t1_cus_"));
        assert_eq!(customer.first.as_deref(), Some("Jane"));

        // Tokens expansion
        assert!(customer.tokens.is_some());
        let tokens = customer.tokens.as_ref().unwrap();
        assert_eq!(tokens.len(), 2);

        // Each token should have an ID
        for token in tokens {
            assert!(token.id.as_str().starts_with("t1_tok_"));
        }
    }

    // =========================================================================
    // Subscription & Plan Expanded Tests
    // =========================================================================

    #[test]
    fn test_subscription_expanded_with_plan() {
        let json = json!({
            "id": "t1_sbn_test123456789012345678",
            "start": 20250101,
            "plan": {
                "id": "t1_pln_test123456789012345678",
                "name": "Monthly Plan",
                "amount": 1999
            }
        });

        let sub: SubscriptionExpanded = serde_json::from_value(json).unwrap();

        assert!(sub.id.as_str().starts_with("t1_sbn_"));
        assert!(sub.plan.is_some());

        let plan = sub.plan.as_ref().unwrap();
        assert_eq!(plan.name.as_deref(), Some("Monthly Plan"));
        assert_eq!(plan.amount, Some(1999));

        // Convenience methods
        assert_eq!(sub.plan_amount_dollars(), Some(19.99));
        assert_eq!(sub.plan_name(), Some("Monthly Plan"));
    }

    #[test]
    fn test_plan_expanded_with_subscriptions() {
        let json = json!({
            "id": "t1_pln_test123456789012345678",
            "name": "Premium Plan",
            "amount": 4999,
            "subscriptions": [
                {"id": "t1_sbn_111111111111111111111", "inactive": 0},
                {"id": "t1_sbn_222222222222222222222", "inactive": 1}
            ]
        });

        let plan: PlanExpanded = serde_json::from_value(json).unwrap();

        assert!(plan.id.as_str().starts_with("t1_pln_"));
        assert_eq!(plan.amount_dollars(), 49.99);
        assert_eq!(plan.subscription_count(), 1); // Only active subscriptions
    }

    // =========================================================================
    // Chargeback & Batch Expanded Tests
    // =========================================================================

    #[test]
    fn test_chargeback_expanded_with_transaction() {
        let json = json!({
            "id": "t1_chb_test123456789012345678",
            "status": "open",
            "total": 5000,
            "cycle": "first",
            "actionable": 1,
            "txn": {
                "id": "t1_txn_test123456789012345678",
                "type": 1,
                "total": 5000
            }
        });

        let cb: ChargebackExpanded = serde_json::from_value(json).unwrap();

        assert!(cb.id.as_str().starts_with("t1_chb_"));
        assert_eq!(cb.amount_dollars(), 50.0);
        assert!(cb.is_actionable());
        assert!(cb.txn.is_some());

        // Convenience method
        assert_eq!(cb.original_transaction_amount(), Some(50.0));
    }

    #[test]
    fn test_batch_expanded_with_transactions() {
        let json = json!({
            "id": "t1_bat_test123456789012345678",
            "status": "open",
            "txns": [
                {"id": "t1_txn_111111111111111111111", "type": 1, "total": 1000},
                {"id": "t1_txn_222222222222222222222", "type": 1, "total": 2000}
            ]
        });

        let batch: BatchExpanded = serde_json::from_value(json).unwrap();

        assert!(batch.id.as_str().starts_with("t1_bat_"));
        assert!(batch.is_open());
        assert_eq!(batch.transaction_count(), 2);
        assert_eq!(batch.total_amount_dollars(), 30.0);
    }

    // =========================================================================
    // Merchant Expanded Tests
    // =========================================================================

    #[test]
    fn test_merchant_expanded_with_members() {
        let json = json!({
            "id": "t1_mer_test123456789012345678",
            "dba": "Test Business",
            "name": "Test Business Inc",
            "members": [
                {"id": "t1_mem_111111111111111111111", "first": "John", "ownership": 6000},
                {"id": "t1_mem_222222222222222222222", "first": "Jane", "ownership": 4000}
            ]
        });

        let merchant: MerchantExpanded = serde_json::from_value(json).unwrap();

        assert!(merchant.id.as_str().starts_with("t1_mer_"));
        assert_eq!(merchant.display_name(), "Test Business");
        assert_eq!(merchant.member_count(), 2);
        assert_eq!(merchant.total_ownership_percent(), 100.0);
    }
}
