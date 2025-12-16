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

pub mod merchant_onboarding;

// Re-export key types for convenience
pub use merchant_onboarding::{
    check_boarding_status, onboard_merchant, Address, BankAccountInfo, BoardingStatus,
    BoardingStatusResult, BusinessInfo, MemberInfo, MerchantConfig, OnboardMerchantRequest,
    OnboardMerchantResult, TermsAcceptance,
};
