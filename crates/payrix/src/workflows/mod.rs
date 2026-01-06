//! High-level workflow modules for common Payrix operations.
//!
//! This module provides workflow abstractions that encapsulate multi-step
//! Payrix API operations into simple, user-friendly interfaces. Each workflow
//! handles the complexity of the underlying API, including:
//!
//! - Converting user-friendly types to Payrix's nested JSON format
//! - Managing the sequence of API calls required for complex operations
//! - Providing clear status tracking and error handling
//!
//! # Available Workflows
//!
//! - [`merchant_onboarding`] - Onboard new merchants with business info, bank accounts, and owners
//! - [`dispute_handling`] - Handle chargeback disputes with compile-time state enforcement
//! - [`webhook_setup`] - Set up webhook alerts for real-time event notifications
//! - [`subscription_management`] - Manage customer subscriptions to recurring payment plans
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::merchant_onboarding::{
//!     onboard_merchant, OnboardMerchantRequest, BusinessInfo, MerchantConfig,
//!     BankAccountInfo, MemberInfo, Address, TermsAcceptance,
//! };
//!
//! # async fn example() -> payrix::Result<()> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! // Use the merchant onboarding workflow
//! // See merchant_onboarding module for complete examples
//! # Ok(())
//! # }
//! ```

pub mod dispute_handling;
pub mod merchant_onboarding;
pub mod subscription_management;
pub mod webhook_setup;

// Re-export key types for convenience
pub use merchant_onboarding::{
    check_boarding_status, onboard_merchant, Address, BankAccountInfo, BoardingStatus,
    BoardingStatusResult, BusinessInfo, MemberInfo, MerchantConfig, OnboardMerchantRequest,
    OnboardMerchantResult, TermsAcceptance,
};

// Re-export dispute handling types
pub use dispute_handling::{
    ActiveDispute, Arbitration, ChargebackDispute, ChargebackState, Evidence, EvidenceDocument,
    First, PreArbitration, Representment, Retrieval, SecondChargeback, Terminal, TypedChargeback,
    evidence_from_base64_url, evidence_from_bytes, evidence_from_path,
    get_actionable_disputes, get_disputes_by_cycle, get_disputes_for_transaction,
    MAX_DOCUMENTS, MAX_DOCUMENT_SIZE, MAX_TOTAL_SIZE,
};

// Re-export webhook setup types
pub use webhook_setup::{
    get_webhook_status, remove_webhook_by_id, remove_webhooks, setup_webhooks,
    WebhookAlertInfo, WebhookConfig, WebhookEventType, WebhookSetupResult, WebhookStatus,
};

// Re-export subscription management types
pub use subscription_management::{
    add_plan_to_customer, calculate_subscription_revenue, cancel_subscription,
    get_active_subscriptions_for_customer, get_subscribers_for_plan, get_subscription_status,
    get_upcoming_payments, next_payment, pause_subscription, payments_to_date,
    resume_subscription, retry_failed_payment, update_payment_method, BillingSchedule,
    NextPayment, PaymentHistory, PlanConfig, PlanReference, SubscribeCustomerConfig,
    SubscribeCustomerResult, SubscriptionError, SubscriptionResult, SubscriptionRevenue,
    SubscriptionState, SubscriptionStatus, TokenConfig, TokenReference, UpcomingPayment,
};
