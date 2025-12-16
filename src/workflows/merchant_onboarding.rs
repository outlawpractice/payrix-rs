//! Merchant onboarding workflow for Payrix.
//!
//! This module provides a high-level interface for onboarding merchants to Payrix.
//! It encapsulates the complex nested API structure into user-friendly types and
//! handles the conversion to the Payrix API format internally.
//!
//! # Documentation Sources
//!
//! This implementation is based on the following Payrix/Worldpay documentation:
//!
//! - [Worldpay Developer Hub - Payrix Pro](https://docs.worldpay.com/apis/payrix) -
//!   Primary API documentation
//! - [Merchant Boarding via API](https://resource.payrix.com/resources/merchant-boarding-via-api) -
//!   API boarding process and JSON structure
//! - [New Merchant Boarding Flow](https://resource.payrix.com/docs/new-merchant-boarding-flow) -
//!   Boarding status flow and underwriting process
//! - [Required Merchant Onboarding API Fields](https://payrix.atlassian.net/wiki/spaces/PRC/pages/826081337) -
//!   Field requirements for /entities, /merchants, /members, /accounts endpoints
//! - [Payrix Pro API Documentation](https://resource.payrix.com/resources/api-how-to-add-a-new-merchant) -
//!   How to add a new merchant via API
//! - [Merchant Boarding: API - Canada](https://resource.payrix.com/resources/merchant-boarding-api-payrix-canada) -
//!   Example JSON payloads for merchant boarding
//!
//! # Overview
//!
//! Merchant onboarding in Payrix requires submitting business information, bank accounts,
//! and beneficial owner details in a specific nested JSON format. This module simplifies
//! that process by providing:
//!
//! - **User-friendly request types** that clearly define required and optional fields
//! - **Automatic conversion** to Payrix's nested JSON structure
//! - **Status tracking** to monitor the boarding process
//!
//! # Workflow Steps
//!
//! 1. Create an [`OnboardMerchantRequest`] with all required information
//! 2. Call [`onboard_merchant()`] to submit the application
//! 3. Check the result's `boarding_status` for immediate approval or pending review
//! 4. Use [`check_boarding_status()`] to poll for status updates if needed
//!
//! # Boarding Status Flow
//!
//! After submission, a merchant can be in one of these states:
//!
//! | Status | Description |
//! |--------|-------------|
//! | `Boarded` | Approved and ready to process payments |
//! | `Pending` | Under automated review (typically resolves within 30 seconds) |
//! | `ManualReview` | Requires manual underwriting review |
//! | `Incomplete` | Missing required information |
//!
//! # Example
//!
//! ```no_run
//! use payrix::{PayrixClient, Environment};
//! use payrix::workflows::merchant_onboarding::*;
//! use payrix::types::{MerchantType, MerchantEnvironment, MemberType, AccountHolderType, AccountType, DateYmd};
//!
//! # async fn example() -> payrix::Result<()> {
//! let client = PayrixClient::new("api-key", Environment::Test)?;
//!
//! let request = OnboardMerchantRequest {
//!     business: BusinessInfo {
//!         business_type: MerchantType::LimitedLiabilityCorporation,
//!         legal_name: "Payrix Rust LLC".to_string(),
//!         address: Address {
//!             line1: "123 Main Street".to_string(),
//!             line2: Some("Suite 100".to_string()),
//!             city: "Springfield".to_string(),
//!             state: "IL".to_string(),
//!             zip: "62701".to_string(),
//!             country: "USA".to_string(),
//!         },
//!         phone: "5551234567".to_string(),
//!         email: "payrixrust@gmail.com".to_string(),
//!         website: Some("https://github.com/outlawpractice/payrix-rs".to_string()),
//!         ein: "123456789".to_string(),
//!     },
//!     merchant: MerchantConfig {
//!         dba: "Payrix Rust LLC".to_string(),
//!         mcc: "8111".to_string(),
//!         environment: MerchantEnvironment::Ecommerce,
//!         annual_cc_sales: 50000000,  // $500,000.00 in cents
//!         avg_ticket: 5000,           // $50.00 in cents
//!         established: DateYmd::new("20200101").unwrap(),
//!         is_new_business: false,
//!     },
//!     accounts: vec![
//!         // Primary operating account - accepts deposits and fee withdrawals
//!         BankAccountInfo {
//!             name: Some("Operating Account".to_string()),
//!             routing_number: Some("123456789".to_string()),
//!             account_number: Some("987654321".to_string()),
//!             holder_type: AccountHolderType::Business,
//!             transaction_type: AccountType::All, // Merchant account
//!             currency: Some("USD".to_string()),
//!             is_primary: true,
//!             plaid_public_token: None,
//!         },
//!         // Trust account - deposits only (no fee withdrawals)
//!         BankAccountInfo {
//!             name: Some("Trust Account".to_string()),
//!             routing_number: Some("123456789".to_string()),
//!             account_number: Some("987654322".to_string()),
//!             holder_type: AccountHolderType::Business,
//!             transaction_type: AccountType::Credit,  // Trust account - deposits only
//!             currency: Some("USD".to_string()),
//!             is_primary: false,
//!             plaid_public_token: None,
//!         },
//!     ],
//!     members: vec![MemberInfo {
//!         member_type: MemberType::Owner,
//!         first_name: "John".to_string(),
//!         last_name: "Doe".to_string(),
//!         title: Some("CEO".to_string()),
//!         ownership_percentage: 100,
//!         date_of_birth: "19800115".to_string(),  // YYYYMMDD format
//!         ssn: "123456789".to_string(),
//!         email: "john@acme.com".to_string(),
//!         phone: "5559876543".to_string(),
//!         address: Address {
//!             line1: "456 Oak Avenue".to_string(),
//!             line2: None,
//!             city: "Springfield".to_string(),
//!             state: "IL".to_string(),
//!             zip: "62702".to_string(),
//!             country: "USA".to_string(),
//!         },
//!     }],
//!     terms_acceptance: TermsAcceptance {
//!         version: "4.21".to_string(),
//!         accepted_at: "2024-01-15 10:30:00".to_string(),
//!     },
//! };
//!
//! let result = onboard_merchant(&client, request).await?;
//!
//! match result.boarding_status {
//!     BoardingStatus::Boarded => {
//!         println!("Merchant approved! ID: {}", result.merchant_id);
//!     }
//!     BoardingStatus::Pending => {
//!         println!("Application pending review...");
//!         // Poll for updates
//!         let status = check_boarding_status(&client, &result.merchant_id).await?;
//!         println!("Current status: {:?}", status.status);
//!     }
//!     BoardingStatus::ManualReview => {
//!         println!("Requires manual review");
//!     }
//!     _ => {
//!         println!("Status: {:?}", result.boarding_status);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Required Fields
//!
//! The following information is required for merchant onboarding:
//!
//! **Business Information:**
//! - Legal business name
//! - Business type (LLC, Corporation, etc.)
//! - Complete address
//! - Phone and email
//! - EIN (Tax ID)
//!
//! **Merchant Configuration:**
//! - DBA (Doing Business As) name
//! - MCC (Merchant Category Code)
//! - Processing environment
//! - Expected annual sales and average ticket
//! - Date established
//!
//! **Bank Account:**
//! - At least one bank account with routing and account numbers
//!
//! **Beneficial Owners:**
//! - At least one owner/control person with personal details
//! - SSN and date of birth for identity verification
//! - Ownership percentage (must total 100% for all owners)
//!
//! **Terms Acceptance:**
//! - Version of terms accepted
//! - Timestamp of acceptance

use serde::{Deserialize, Serialize};

use crate::client::PayrixClient;
use crate::entity::EntityType;
use crate::error::Result;
use crate::types::{
    Account, AccountHolderType, AccountType, DateYmd, Entity, Member, MemberType, Merchant,
    MerchantEnvironment, MerchantStatus, MerchantType,
};

// ============================================================================
// Public Request Types
// ============================================================================

/// Complete request for onboarding a new merchant.
///
/// This structure contains all the information required to onboard a merchant
/// to Payrix. The workflow will convert these user-friendly types to the
/// Payrix API's nested JSON format internally.
///
/// # Required Components
///
/// - `business` - Legal business entity information
/// - `merchant` - Merchant processing configuration
/// - `accounts` - At least one bank account for funding
/// - `members` - Beneficial owners/control persons
/// - `terms_acceptance` - Acknowledgment of terms and conditions
#[derive(Debug, Clone)]
pub struct OnboardMerchantRequest {
    /// Business entity information (legal name, address, tax ID)
    pub business: BusinessInfo,

    /// Merchant processing configuration (DBA, MCC, volumes)
    pub merchant: MerchantConfig,

    /// Bank accounts for funding (at least one required)
    pub accounts: Vec<BankAccountInfo>,

    /// Beneficial owners and control persons (at least one required)
    pub members: Vec<MemberInfo>,

    /// Terms and conditions acceptance record
    pub terms_acceptance: TermsAcceptance,
}

/// Business entity information.
///
/// Contains the legal details about the business being onboarded,
/// including business structure, address, and tax identification.
#[derive(Debug, Clone)]
pub struct BusinessInfo {
    /// Business structure type (LLC, Corporation, Sole Proprietor, etc.)
    ///
    /// This determines the legal structure of the business and affects
    /// compliance requirements.
    pub business_type: MerchantType,

    /// Legal business name as registered with the state.
    ///
    /// This should match the name on file with the IRS and state registration.
    pub legal_name: String,

    /// Business address.
    ///
    /// This should be the primary business location, not a P.O. Box.
    pub address: Address,

    /// Business phone number (digits only, no formatting).
    ///
    /// Example: "5551234567"
    pub phone: String,

    /// Business email address.
    ///
    /// This will be used for business communications from Payrix.
    pub email: String,

    /// Business website URL (optional).
    ///
    /// Include the full URL with protocol (e.g., "https://www.example.com").
    pub website: Option<String>,

    /// Employer Identification Number (EIN) / Tax ID.
    ///
    /// 9-digit federal tax identification number, no dashes or spaces.
    /// Example: "123456789"
    pub ein: String,
}

/// Merchant processing configuration.
///
/// Contains information about how the merchant will process payments,
/// including expected volumes and business category.
#[derive(Debug, Clone)]
pub struct MerchantConfig {
    /// "Doing Business As" name.
    ///
    /// The public-facing name customers see on their statements.
    /// May be different from the legal business name.
    pub dba: String,

    /// Merchant Category Code (MCC).
    ///
    /// A 4-digit code that classifies the type of business.
    /// Examples:
    /// - "5812" - Restaurants
    /// - "5999" - Miscellaneous retail
    /// - "8111" - Legal services
    pub mcc: String,

    /// Processing environment.
    ///
    /// Indicates how transactions will primarily be processed.
    pub environment: MerchantEnvironment,

    /// Expected annual credit card sales volume in cents.
    ///
    /// Example: 50000000 = $500,000.00 per year
    pub annual_cc_sales: i64,

    /// Average transaction amount in cents.
    ///
    /// Example: 5000 = $50.00 average ticket
    pub avg_ticket: i64,

    /// Date the business was established (YYYYMMDD format).
    ///
    /// Use `DateYmd::new("20200101")` for January 1, 2020.
    pub established: DateYmd,

    /// Whether this is a new business (less than 2 years old).
    pub is_new_business: bool,
}

/// Bank account information for funding.
///
/// At least one bank account is required for merchant onboarding.
/// Accounts can be configured for different purposes:
///
/// - **Deposits only** (`transaction_type: Credit`) - e.g., a trust account that can only receive funds
/// - **Withdrawals only** (`transaction_type: Debit`) - for fee debits only
/// - **Both** (`transaction_type: All`) - standard merchant account for deposits and fee withdrawals
///
/// # Multiple Account Scenarios
///
/// A common configuration is two accounts:
/// 1. A trust account (Credit only) for receiving customer payments
/// 2. A merchant operating account (All) for deposits and fee withdrawals
///
/// # Plaid Integration
///
/// For instant account verification via Plaid, provide the `plaid_public_token`
/// instead of routing/account numbers. The Payrix API will retrieve the bank
/// details from Plaid.
#[derive(Clone)]
pub struct BankAccountInfo {
    /// Account name/label (optional).
    ///
    /// A descriptive name for the account, e.g., "Operating Account" or "Trust Account".
    pub name: Option<String>,

    /// Bank routing number (ABA number).
    ///
    /// 9-digit routing number, no dashes or spaces.
    /// Example: "123456789"
    ///
    /// Not required if using Plaid (`plaid_public_token` is provided).
    pub routing_number: Option<String>,

    /// Bank account number.
    ///
    /// The full account number, no dashes or spaces.
    ///
    /// Not required if using Plaid (`plaid_public_token` is provided).
    pub account_number: Option<String>,

    /// Account holder type.
    ///
    /// Whether this is a personal or business account.
    pub holder_type: AccountHolderType,

    /// Transaction type this account supports.
    ///
    /// - `Credit` - Deposits/credits only (e.g., trust account)
    /// - `Debit` - Withdrawals/debits only (e.g., fee account)
    /// - `All` - Both deposits and withdrawals (standard merchant account)
    ///
    /// Defaults to `All` if not specified.
    pub transaction_type: AccountType,

    /// Currency code.
    ///
    /// ISO 4217 currency code, e.g., "USD", "CAD".
    /// Defaults to "USD" if not specified.
    pub currency: Option<String>,

    /// Whether this is the primary account.
    ///
    /// At least one account must be marked as primary.
    /// The primary account is used for standard deposits and fee withdrawals.
    pub is_primary: bool,

    /// Plaid public token for instant account verification.
    ///
    /// If provided, the Payrix API will retrieve bank details from Plaid,
    /// and `routing_number`/`account_number` are not required.
    ///
    /// Obtain this token from Plaid Link in your frontend application.
    pub plaid_public_token: Option<String>,
}

/// Beneficial owner or control person information.
///
/// KYC/AML regulations require collecting information about individuals
/// who own or control the business. At least one member is required.
///
/// # Member Types
///
/// - `Owner` - Individual with 25% or more ownership
/// - `ControlPerson` - Individual with significant control (e.g., CEO, CFO)
/// - `Principal` - Other key individual
#[derive(Clone)]
pub struct MemberInfo {
    /// Type of member relationship.
    pub member_type: MemberType,

    /// First name.
    pub first_name: String,

    /// Last name.
    pub last_name: String,

    /// Title or position (optional).
    ///
    /// Example: "CEO", "Owner", "CFO"
    pub title: Option<String>,

    /// Ownership percentage (0-100).
    ///
    /// Sum of all owners should equal 100.
    pub ownership_percentage: i32,

    /// Date of birth in YYYYMMDD format.
    ///
    /// Required for identity verification.
    /// Example: "19800115" for January 15, 1980.
    ///
    /// Note: This is a String rather than DateYmd because dates of birth
    /// typically predate the year 2000, which is outside DateYmd's valid range.
    pub date_of_birth: String,

    /// Social Security Number.
    ///
    /// Full 9-digit SSN, no dashes or spaces. Required for identity verification.
    /// Example: "123456789"
    ///
    /// **Security Note:** This field is transmitted securely and stored encrypted.
    pub ssn: String,

    /// Email address.
    pub email: String,

    /// Phone number (digits only).
    pub phone: String,

    /// Home address.
    ///
    /// Must be a residential address, not a business address.
    pub address: Address,
}

/// Physical address.
///
/// Used for both business and residential addresses.
#[derive(Debug, Clone)]
pub struct Address {
    /// Street address line 1.
    pub line1: String,

    /// Street address line 2 (optional).
    ///
    /// Apartment, suite, unit, etc.
    pub line2: Option<String>,

    /// City.
    pub city: String,

    /// State or province code.
    ///
    /// Use 2-letter state codes for US (e.g., "CA", "NY", "TX").
    pub state: String,

    /// ZIP or postal code.
    pub zip: String,

    /// Country code.
    ///
    /// Use "USA" for United States, "CAN" for Canada.
    pub country: String,
}

/// Terms and conditions acceptance record.
///
/// Documents when and what version of the terms were accepted.
/// This is required for compliance and legal purposes.
#[derive(Debug, Clone)]
pub struct TermsAcceptance {
    /// Version of terms and conditions accepted.
    ///
    /// Example: "4.21"
    pub version: String,

    /// Timestamp when terms were accepted.
    ///
    /// Format: "YYYY-MM-DD HH:mm:ss"
    /// Example: "2024-01-15 10:30:00"
    pub accepted_at: String,
}

// ============================================================================
// Public Response Types
// ============================================================================

/// Result of merchant onboarding operation.
///
/// Contains the created resources and current boarding status.
#[derive(Debug, Clone)]
pub struct OnboardMerchantResult {
    /// The created entity ID.
    ///
    /// This is the parent business entity in Payrix.
    pub entity_id: String,

    /// The created merchant ID.
    ///
    /// Use this ID for subsequent operations and status checks.
    pub merchant_id: String,

    /// Current boarding status.
    ///
    /// Check this to determine if the merchant was approved immediately
    /// or requires further review.
    pub boarding_status: BoardingStatus,

    /// The full entity response from Payrix.
    pub entity: Entity,

    /// The full merchant response from Payrix.
    pub merchant: Merchant,

    /// Created bank accounts.
    pub accounts: Vec<Account>,

    /// Created beneficial owners/members.
    pub members: Vec<Member>,
}

/// Boarding status for merchant applications.
///
/// This is a user-friendly enum that maps to the underlying Payrix
/// merchant status values relevant to onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardingStatus {
    /// Not ready for boarding (internal state).
    ///
    /// The merchant has not yet been submitted for underwriting.
    NotReady,

    /// Submitted for boarding.
    ///
    /// The application has been submitted and underwriting has been triggered.
    /// This typically transitions quickly to another state.
    Submitted,

    /// Successfully boarded and approved.
    ///
    /// The merchant is approved to process payments.
    Boarded,

    /// Requires manual underwriting review.
    ///
    /// Additional documentation or review is needed before approval.
    ManualReview,

    /// Account has been closed.
    Closed,

    /// Application is incomplete.
    ///
    /// Missing required information that must be provided.
    Incomplete,

    /// Pending automated review.
    ///
    /// The application is being reviewed by automated systems.
    /// Typically resolves within 30 seconds.
    Pending,
}

impl From<MerchantStatus> for BoardingStatus {
    fn from(status: MerchantStatus) -> Self {
        match status {
            MerchantStatus::NotReady => BoardingStatus::NotReady,
            MerchantStatus::Ready => BoardingStatus::Submitted,
            MerchantStatus::Boarded => BoardingStatus::Boarded,
            MerchantStatus::Manual => BoardingStatus::ManualReview,
            MerchantStatus::Closed => BoardingStatus::Closed,
            MerchantStatus::Incomplete => BoardingStatus::Incomplete,
            MerchantStatus::Pending => BoardingStatus::Pending,
        }
    }
}

impl std::fmt::Display for BoardingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardingStatus::NotReady => write!(f, "Not Ready"),
            BoardingStatus::Submitted => write!(f, "Submitted"),
            BoardingStatus::Boarded => write!(f, "Boarded"),
            BoardingStatus::ManualReview => write!(f, "Manual Review"),
            BoardingStatus::Closed => write!(f, "Closed"),
            BoardingStatus::Incomplete => write!(f, "Incomplete"),
            BoardingStatus::Pending => write!(f, "Pending"),
        }
    }
}

/// Result of checking boarding status.
///
/// Contains the current status and related information.
#[derive(Debug, Clone)]
pub struct BoardingStatusResult {
    /// Current boarding status.
    pub status: BoardingStatus,

    /// Merchant ID.
    pub merchant_id: String,

    /// Entity ID (parent business).
    pub entity_id: String,

    /// Date boarded (if approved).
    ///
    /// Format: YYYYMMDD
    pub boarded_date: Option<String>,
}

// ============================================================================
// Internal Types for Payrix API Serialization
// ============================================================================

/// Internal payload structure matching Payrix's nested JSON format.
///
/// This is not exposed publicly - users work with the friendly request types
/// which are converted to this format internally.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PayrixOnboardingPayload {
    /// Business type (0=SoleProprietor, 1=Corp, 2=LLC, etc.)
    #[serde(rename = "type")]
    entity_type: MerchantType,

    /// Legal business name
    name: String,

    /// Street address line 1
    address1: String,

    /// Street address line 2 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    address2: Option<String>,

    /// City
    city: String,

    /// State/province code
    state: String,

    /// ZIP/postal code
    zip: String,

    /// Country code
    country: String,

    /// Phone number
    phone: String,

    /// Email address
    email: String,

    /// Website URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,

    /// EIN (Tax ID)
    ein: String,

    /// Terms & Conditions version
    tc_version: String,

    /// Terms & Conditions acceptance timestamp
    tc_date: String,

    /// Terms & Conditions attestation (1 = accepted)
    tc_attestation: i32,

    /// Nested bank accounts
    accounts: Vec<PayrixAccountPayload>,

    /// Nested merchant configuration
    merchant: PayrixMerchantPayload,
}

/// Internal bank account payload for Payrix API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PayrixAccountPayload {
    /// Whether this is the primary account (1 = yes, 0 = no)
    primary: i32,

    /// Account name/label
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Transaction type (credit, debit, or all)
    #[serde(rename = "type")]
    transaction_type: AccountType,

    /// Currency code (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,

    /// Nested account details (for manual entry)
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<PayrixAccountDetails>,

    /// Plaid public token for instant verification
    #[serde(skip_serializing_if = "Option::is_none")]
    public_token: Option<String>,
}

/// Internal account details for Payrix API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PayrixAccountDetails {
    /// Account method/type (8 = checking)
    method: i32,

    /// Bank account number
    number: String,

    /// Bank routing number
    routing: String,

    /// Account holder type (1 = individual, 2 = business)
    holder_type: AccountHolderType,
}

/// Internal merchant payload for Payrix API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PayrixMerchantPayload {
    /// DBA name
    dba: String,

    /// Is new business (0 = no, 1 = yes)
    new: i32,

    /// Merchant Category Code
    mcc: String,

    /// Boarding status (1 = Board Immediately)
    status: i32,

    /// Processing environment
    environment: MerchantEnvironment,

    /// Annual credit card sales in cents
    annual_cc_sales: i64,

    /// Average ticket in cents
    avg_ticket: i64,

    /// Date established (YYYYMMDD)
    established: String,

    /// Nested members (owners/principals)
    members: Vec<PayrixMemberPayload>,
}

/// Internal member payload for Payrix API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PayrixMemberPayload {
    /// Member type (1=Owner, 2=ControlPerson, 3=Principal)
    #[serde(rename = "type")]
    member_type: MemberType,

    /// First name
    first: String,

    /// Last name
    last: String,

    /// Title (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// Ownership percentage
    ownership: i32,

    /// Date of birth (YYYYMMDD)
    dob: String,

    /// SSN
    ssn: String,

    /// Email
    email: String,

    /// Phone
    phone: String,

    /// Address line 1
    address1: String,

    /// Address line 2 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    address2: Option<String>,

    /// City
    city: String,

    /// State
    state: String,

    /// ZIP
    zip: String,

    /// Country
    country: String,
}

// ============================================================================
// Custom Debug Implementations (mask sensitive data)
// ============================================================================

/// Helper to mask sensitive strings, showing only last 4 characters.
fn mask_sensitive(value: &str) -> String {
    if value.len() <= 4 {
        "*".repeat(value.len())
    } else {
        format!("{}{}", "*".repeat(value.len() - 4), &value[value.len() - 4..])
    }
}

impl std::fmt::Debug for BankAccountInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BankAccountInfo")
            .field("name", &self.name)
            .field("routing_number", &self.routing_number.as_ref().map(|s| mask_sensitive(s)))
            .field("account_number", &self.account_number.as_ref().map(|s| mask_sensitive(s)))
            .field("holder_type", &self.holder_type)
            .field("transaction_type", &self.transaction_type)
            .field("currency", &self.currency)
            .field("is_primary", &self.is_primary)
            .field("plaid_public_token", &self.plaid_public_token.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

impl std::fmt::Debug for MemberInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemberInfo")
            .field("member_type", &self.member_type)
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("title", &self.title)
            .field("ownership_percentage", &self.ownership_percentage)
            .field("date_of_birth", &self.date_of_birth)
            .field("ssn", &mask_sensitive(&self.ssn))
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("address", &self.address)
            .finish()
    }
}

impl std::fmt::Debug for PayrixAccountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PayrixAccountDetails")
            .field("method", &self.method)
            .field("number", &mask_sensitive(&self.number))
            .field("routing", &mask_sensitive(&self.routing))
            .field("holder_type", &self.holder_type)
            .finish()
    }
}

impl std::fmt::Debug for PayrixMemberPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PayrixMemberPayload")
            .field("member_type", &self.member_type)
            .field("first", &self.first)
            .field("last", &self.last)
            .field("title", &self.title)
            .field("ownership", &self.ownership)
            .field("dob", &self.dob)
            .field("ssn", &mask_sensitive(&self.ssn))
            .field("email", &self.email)
            .field("phone", &self.phone)
            .field("address1", &self.address1)
            .field("address2", &self.address2)
            .field("city", &self.city)
            .field("state", &self.state)
            .field("zip", &self.zip)
            .field("country", &self.country)
            .finish()
    }
}

// ============================================================================
// Type Conversions
// ============================================================================

impl From<OnboardMerchantRequest> for PayrixOnboardingPayload {
    fn from(request: OnboardMerchantRequest) -> Self {
        PayrixOnboardingPayload {
            entity_type: request.business.business_type,
            name: request.business.legal_name,
            address1: request.business.address.line1,
            address2: request.business.address.line2,
            city: request.business.address.city,
            state: request.business.address.state,
            zip: request.business.address.zip,
            country: request.business.address.country,
            phone: request.business.phone,
            email: request.business.email,
            website: request.business.website,
            ein: request.business.ein,
            tc_version: request.terms_acceptance.version,
            tc_date: request.terms_acceptance.accepted_at,
            tc_attestation: 1,
            accounts: request.accounts.into_iter().map(|a| a.into()).collect(),
            merchant: PayrixMerchantPayload {
                dba: request.merchant.dba,
                new: if request.merchant.is_new_business { 1 } else { 0 },
                mcc: request.merchant.mcc,
                status: 1, // Board Immediately
                environment: request.merchant.environment,
                annual_cc_sales: request.merchant.annual_cc_sales,
                avg_ticket: request.merchant.avg_ticket,
                established: request.merchant.established.as_str().to_string(),
                members: request.members.into_iter().map(|m| m.into()).collect(),
            },
        }
    }
}

impl From<BankAccountInfo> for PayrixAccountPayload {
    fn from(account: BankAccountInfo) -> Self {
        // Build account details if routing/account numbers are provided (manual entry)
        let account_details = match (&account.routing_number, &account.account_number) {
            (Some(routing), Some(number)) => Some(PayrixAccountDetails {
                method: 8, // Checking account
                number: number.clone(),
                routing: routing.clone(),
                holder_type: account.holder_type,
            }),
            _ => None,
        };

        PayrixAccountPayload {
            primary: if account.is_primary { 1 } else { 0 },
            name: account.name,
            transaction_type: account.transaction_type,
            currency: account.currency,
            account: account_details,
            public_token: account.plaid_public_token,
        }
    }
}

impl From<MemberInfo> for PayrixMemberPayload {
    fn from(member: MemberInfo) -> Self {
        PayrixMemberPayload {
            member_type: member.member_type,
            first: member.first_name,
            last: member.last_name,
            title: member.title,
            ownership: member.ownership_percentage,
            dob: member.date_of_birth,
            ssn: member.ssn,
            email: member.email,
            phone: member.phone,
            address1: member.address.line1,
            address2: member.address.line2,
            city: member.address.city,
            state: member.address.state,
            zip: member.address.zip,
            country: member.address.country,
        }
    }
}

// ============================================================================
// Response Parsing Types
// ============================================================================

/// Internal response structure for parsing Payrix's nested response.
///
/// Some fields are included for completeness when parsing the full API response,
/// even if not all are used in the current implementation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct PayrixOnboardingResponse {
    /// Entity ID
    id: String,

    /// Nested merchant in response
    #[serde(default)]
    merchant: Option<MerchantInResponse>,

    /// Nested accounts in response
    #[serde(default)]
    accounts: Option<Vec<Account>>,
}

/// Internal merchant response structure.
///
/// Some fields are included for completeness when parsing the full API response,
/// even if not all are used in the current implementation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct MerchantInResponse {
    /// Merchant ID
    id: String,

    /// Merchant status
    #[serde(default)]
    status: Option<MerchantStatus>,

    /// Entity ID
    #[serde(default)]
    entity: Option<String>,

    /// Boarded date
    #[serde(default)]
    boarded: Option<String>,

    /// Members in response
    #[serde(default)]
    members: Option<Vec<Member>>,
}

// ============================================================================
// Validation
// ============================================================================

/// Validates an onboarding request before sending to the API.
///
/// This catches common errors early with clear error messages, rather than
/// waiting for cryptic API responses.
fn validate_request(request: &OnboardMerchantRequest) -> Result<()> {
    // Validate accounts
    if request.accounts.is_empty() {
        return Err(crate::error::Error::Config(
            "At least one bank account is required".into(),
        ));
    }

    if !request.accounts.iter().any(|a| a.is_primary) {
        return Err(crate::error::Error::Config(
            "One account must be marked as primary".into(),
        ));
    }

    // Validate each account has either routing/account numbers OR Plaid token
    for (i, account) in request.accounts.iter().enumerate() {
        let has_manual = account.routing_number.is_some() && account.account_number.is_some();
        let has_plaid = account.plaid_public_token.is_some();

        if !has_manual && !has_plaid {
            return Err(crate::error::Error::Config(format!(
                "Account {} requires either routing/account numbers or a Plaid token",
                i + 1
            )));
        }

        // Validate routing number format (9 digits)
        if let Some(ref routing) = account.routing_number {
            if routing.len() != 9 || !routing.chars().all(|c| c.is_ascii_digit()) {
                return Err(crate::error::Error::Config(format!(
                    "Account {} routing number must be exactly 9 digits",
                    i + 1
                )));
            }
        }
    }

    // Validate members
    if request.members.is_empty() {
        return Err(crate::error::Error::Config(
            "At least one member (owner or control person) is required".into(),
        ));
    }

    // Validate SSN format for each member (9 digits)
    for (i, member) in request.members.iter().enumerate() {
        if member.ssn.len() != 9 || !member.ssn.chars().all(|c| c.is_ascii_digit()) {
            return Err(crate::error::Error::Config(format!(
                "Member {} SSN must be exactly 9 digits (no dashes)",
                i + 1
            )));
        }

        // Validate date of birth format (YYYYMMDD, 8 digits)
        if member.date_of_birth.len() != 8
            || !member.date_of_birth.chars().all(|c| c.is_ascii_digit())
        {
            return Err(crate::error::Error::Config(format!(
                "Member {} date of birth must be in YYYYMMDD format (8 digits)",
                i + 1
            )));
        }

        // Validate ownership percentage is reasonable
        if member.ownership_percentage < 0 || member.ownership_percentage > 100 {
            return Err(crate::error::Error::Config(format!(
                "Member {} ownership percentage must be between 0 and 100",
                i + 1
            )));
        }
    }

    // Validate EIN format (9 digits)
    if request.business.ein.len() != 9
        || !request.business.ein.chars().all(|c| c.is_ascii_digit())
    {
        return Err(crate::error::Error::Config(
            "EIN must be exactly 9 digits (no dashes)".into(),
        ));
    }

    Ok(())
}

// ============================================================================
// Workflow Functions
// ============================================================================

/// Onboard a new merchant to Payrix.
///
/// This high-level workflow handles the complete merchant onboarding process:
///
/// 1. **Creates the business entity** with address and tax information
/// 2. **Creates the merchant account** with processing configuration
/// 3. **Adds bank accounts** for funding
/// 4. **Registers beneficial owners** for compliance
/// 5. **Initiates the underwriting process** by setting status to "Board Immediately"
///
/// The function internally converts the user-friendly request types to the
/// Payrix API's nested JSON structure and submits a single POST to `/entities`.
///
/// # Arguments
///
/// * `client` - The Payrix API client
/// * `request` - The complete onboarding request
///
/// # Returns
///
/// Returns an [`OnboardMerchantResult`] containing:
/// - The created entity and merchant IDs
/// - Current boarding status
/// - Full entity and merchant objects
/// - Created accounts and members
///
/// # Errors
///
/// Returns an error if:
/// - The API request fails
/// - Required fields are missing or invalid
/// - Rate limits are exceeded
///
/// # Example
///
/// See module-level documentation for a complete example.
pub async fn onboard_merchant(
    client: &PayrixClient,
    request: OnboardMerchantRequest,
) -> Result<OnboardMerchantResult> {
    // Validate the request before sending to API
    validate_request(&request)?;

    // Convert the user-friendly request to Payrix's nested format
    let payload: PayrixOnboardingPayload = request.into();

    // Submit to the entities endpoint with nested data
    // The Payrix API creates entity, merchant, accounts, and members in one call
    let response: PayrixOnboardingResponse = client.create(EntityType::Entities, &payload).await?;

    // Extract the merchant and member data from the nested response
    let merchant_response = response.merchant.unwrap_or_else(|| MerchantInResponse {
        id: String::new(),
        status: None,
        entity: None,
        boarded: None,
        members: None,
    });

    let boarding_status = merchant_response
        .status
        .map(BoardingStatus::from)
        .unwrap_or(BoardingStatus::NotReady);

    // Fetch the full entity and merchant objects for the result
    let entity: Entity = client
        .get_one(EntityType::Entities, &response.id)
        .await?
        .ok_or_else(|| crate::error::Error::Internal("Entity not found after creation".into()))?;

    let merchant: Merchant = client
        .get_one(EntityType::Merchants, &merchant_response.id)
        .await?
        .ok_or_else(|| crate::error::Error::Internal("Merchant not found after creation".into()))?;

    // Get accounts and members
    let accounts: Vec<Account> = client
        .search(
            EntityType::Accounts,
            &format!("entity[equals]={}", response.id),
        )
        .await?;

    let members: Vec<Member> = client
        .search(
            EntityType::Members,
            &format!("merchant[equals]={}", merchant_response.id),
        )
        .await?;

    Ok(OnboardMerchantResult {
        entity_id: response.id,
        merchant_id: merchant_response.id,
        boarding_status,
        entity,
        merchant,
        accounts,
        members,
    })
}

/// Check the current boarding status of a merchant.
///
/// Use this function to poll the status of a merchant application that
/// returned `Pending` or `ManualReview` status after onboarding.
///
/// # Arguments
///
/// * `client` - The Payrix API client
/// * `merchant_id` - The merchant ID to check
///
/// # Returns
///
/// Returns a [`BoardingStatusResult`] containing:
/// - Current boarding status
/// - Merchant and entity IDs
/// - Boarded date (if approved)
///
/// # Example
///
/// ```no_run
/// use payrix::{PayrixClient, Environment};
/// use payrix::workflows::merchant_onboarding::{check_boarding_status, BoardingStatus};
///
/// # async fn example() -> payrix::Result<()> {
/// let client = PayrixClient::new("api-key", Environment::Test)?;
///
/// let status = check_boarding_status(&client, "t1_mer_12345678901234567890123").await?;
///
/// match status.status {
///     BoardingStatus::Boarded => {
///         println!("Approved on: {:?}", status.boarded_date);
///     }
///     BoardingStatus::Pending => {
///         println!("Still pending review...");
///     }
///     _ => {
///         println!("Status: {}", status.status);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub async fn check_boarding_status(
    client: &PayrixClient,
    merchant_id: &str,
) -> Result<BoardingStatusResult> {
    let merchant: Merchant = client
        .get_one(EntityType::Merchants, merchant_id)
        .await?
        .ok_or_else(|| crate::error::Error::NotFound(format!("Merchant not found: {}", merchant_id)))?;

    let status = merchant
        .status
        .map(BoardingStatus::from)
        .unwrap_or(BoardingStatus::NotReady);

    Ok(BoardingStatusResult {
        status,
        merchant_id: merchant.id.as_str().to_string(),
        entity_id: merchant
            .entity
            .map(|e| e.as_str().to_string())
            .unwrap_or_default(),
        boarded_date: merchant.boarded.map(|d| d.as_str().to_string()),
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Sensitive Data Masking Tests
    // ============================================================================

    #[test]
    fn test_mask_sensitive_full_ssn() {
        let result = mask_sensitive("123456789");
        assert_eq!(result, "*****6789");
    }

    #[test]
    fn test_mask_sensitive_short_value() {
        let result = mask_sensitive("1234");
        assert_eq!(result, "****");
    }

    #[test]
    fn test_mask_sensitive_empty() {
        let result = mask_sensitive("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_bank_account_debug_masks_sensitive() {
        let account = BankAccountInfo {
            name: Some("Test Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("9876543210".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: None,
        };
        let debug_str = format!("{:?}", account);
        // Should NOT contain full account/routing numbers
        assert!(!debug_str.contains("123456789"));
        assert!(!debug_str.contains("9876543210"));
        // Should contain masked versions
        assert!(debug_str.contains("*****6789"));
        assert!(debug_str.contains("******3210"));
    }

    #[test]
    fn test_member_info_debug_masks_ssn() {
        let member = MemberInfo {
            member_type: MemberType::Owner,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            title: Some("CEO".to_string()),
            ownership_percentage: 100,
            date_of_birth: "19800115".to_string(),
            ssn: "123456789".to_string(),
            email: "john@example.com".to_string(),
            phone: "5551234567".to_string(),
            address: Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60601".to_string(),
                country: "USA".to_string(),
            },
        };
        let debug_str = format!("{:?}", member);
        // Should NOT contain full SSN
        assert!(!debug_str.contains("123456789"));
        // Should contain masked version
        assert!(debug_str.contains("*****6789"));
        // Name should still be visible
        assert!(debug_str.contains("John"));
    }

    // ============================================================================
    // Validation Tests
    // ============================================================================

    /// Helper to create a valid request for validation tests
    fn valid_request() -> OnboardMerchantRequest {
        OnboardMerchantRequest {
            business: BusinessInfo {
                business_type: MerchantType::LimitedLiabilityCorporation,
                legal_name: "Test LLC".to_string(),
                address: Address {
                    line1: "123 Main St".to_string(),
                    line2: None,
                    city: "Chicago".to_string(),
                    state: "IL".to_string(),
                    zip: "60601".to_string(),
                    country: "USA".to_string(),
                },
                phone: "5551234567".to_string(),
                email: "test@example.com".to_string(),
                website: None,
                ein: "123456789".to_string(),
            },
            merchant: MerchantConfig {
                dba: "Test DBA".to_string(),
                mcc: "5999".to_string(),
                environment: MerchantEnvironment::Ecommerce,
                annual_cc_sales: 100000,
                avg_ticket: 5000,
                established: DateYmd::new("20200101").unwrap(),
                is_new_business: false,
            },
            accounts: vec![BankAccountInfo {
                name: Some("Operating".to_string()),
                routing_number: Some("123456789".to_string()),
                account_number: Some("987654321".to_string()),
                holder_type: AccountHolderType::Business,
                transaction_type: AccountType::All,
                currency: Some("USD".to_string()),
                is_primary: true,
                plaid_public_token: None,
            }],
            members: vec![MemberInfo {
                member_type: MemberType::Owner,
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                title: Some("CEO".to_string()),
                ownership_percentage: 100,
                date_of_birth: "19800115".to_string(),
                ssn: "123456789".to_string(),
                email: "john@example.com".to_string(),
                phone: "5551234567".to_string(),
                address: Address {
                    line1: "456 Oak Ave".to_string(),
                    line2: None,
                    city: "Chicago".to_string(),
                    state: "IL".to_string(),
                    zip: "60602".to_string(),
                    country: "USA".to_string(),
                },
            }],
            terms_acceptance: TermsAcceptance {
                version: "4.21".to_string(),
                accepted_at: "2024-01-15 10:30:00".to_string(),
            },
        }
    }

    #[test]
    fn test_validate_valid_request() {
        let request = valid_request();
        assert!(validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_empty_accounts() {
        let mut request = valid_request();
        request.accounts = vec![];
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("bank account"));
    }

    #[test]
    fn test_validate_no_primary_account() {
        let mut request = valid_request();
        request.accounts[0].is_primary = false;
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("primary"));
    }

    #[test]
    fn test_validate_account_missing_routing_and_plaid() {
        let mut request = valid_request();
        request.accounts[0].routing_number = None;
        request.accounts[0].account_number = None;
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("routing/account numbers or a Plaid token"));
    }

    #[test]
    fn test_validate_invalid_routing_number() {
        let mut request = valid_request();
        request.accounts[0].routing_number = Some("12345".to_string()); // Too short
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("routing number"));
    }

    #[test]
    fn test_validate_routing_number_with_dashes() {
        let mut request = valid_request();
        request.accounts[0].routing_number = Some("123-456-789".to_string());
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("routing number"));
    }

    #[test]
    fn test_validate_empty_members() {
        let mut request = valid_request();
        request.members = vec![];
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("member"));
    }

    #[test]
    fn test_validate_invalid_ssn() {
        let mut request = valid_request();
        request.members[0].ssn = "123-45-6789".to_string(); // With dashes
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("SSN"));
    }

    #[test]
    fn test_validate_ssn_too_short() {
        let mut request = valid_request();
        request.members[0].ssn = "12345".to_string();
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("SSN"));
    }

    #[test]
    fn test_validate_invalid_dob() {
        let mut request = valid_request();
        request.members[0].date_of_birth = "1980-01-15".to_string(); // With dashes
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("date of birth"));
    }

    #[test]
    fn test_validate_ownership_over_100() {
        let mut request = valid_request();
        request.members[0].ownership_percentage = 150;
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("ownership"));
    }

    #[test]
    fn test_validate_negative_ownership() {
        let mut request = valid_request();
        request.members[0].ownership_percentage = -10;
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("ownership"));
    }

    #[test]
    fn test_validate_invalid_ein() {
        let mut request = valid_request();
        request.business.ein = "12-3456789".to_string(); // With dash
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("EIN"));
    }

    #[test]
    fn test_validate_ein_too_short() {
        let mut request = valid_request();
        request.business.ein = "12345".to_string();
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("EIN"));
    }

    #[test]
    fn test_validate_plaid_token_valid() {
        let mut request = valid_request();
        // Remove manual entry, add Plaid token
        request.accounts[0].routing_number = None;
        request.accounts[0].account_number = None;
        request.accounts[0].plaid_public_token = Some("public-token-xxx".to_string());
        assert!(validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_routing_number_with_letters() {
        let mut request = valid_request();
        request.accounts[0].routing_number = Some("12345678A".to_string());
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("routing number"));
    }

    #[test]
    fn test_validate_ssn_with_letters() {
        let mut request = valid_request();
        request.members[0].ssn = "12345678A".to_string();
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("SSN"));
    }

    #[test]
    fn test_validate_dob_too_short() {
        let mut request = valid_request();
        request.members[0].date_of_birth = "1980".to_string();
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("date of birth"));
    }

    #[test]
    fn test_validate_ein_with_letters() {
        let mut request = valid_request();
        request.business.ein = "12345678A".to_string();
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("EIN"));
    }

    #[test]
    fn test_validate_ownership_zero_valid() {
        let mut request = valid_request();
        request.members[0].ownership_percentage = 0;
        assert!(validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_ownership_100_valid() {
        let mut request = valid_request();
        request.members[0].ownership_percentage = 100;
        assert!(validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_second_account_fails() {
        let mut request = valid_request();
        request.accounts.push(BankAccountInfo {
            name: Some("Second".to_string()),
            routing_number: Some("invalid".to_string()), // Invalid
            account_number: Some("123456".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::Credit,
            currency: None,
            is_primary: false,
            plaid_public_token: None,
        });
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("Account 2"));
    }

    #[test]
    fn test_validate_second_member_fails() {
        let mut request = valid_request();
        request.members.push(MemberInfo {
            member_type: MemberType::Owner,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            title: None,
            ownership_percentage: 50,
            date_of_birth: "19850520".to_string(),
            ssn: "invalid".to_string(), // Invalid
            email: "jane@example.com".to_string(),
            phone: "5559876543".to_string(),
            address: Address {
                line1: "789 Pine St".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60603".to_string(),
                country: "USA".to_string(),
            },
        });
        let err = validate_request(&request).unwrap_err();
        assert!(err.to_string().contains("Member 2"));
    }

    // ============================================================================
    // Boarding Status Tests
    // ============================================================================

    #[test]
    fn test_boarding_status_from_merchant_status() {
        assert_eq!(
            BoardingStatus::from(MerchantStatus::NotReady),
            BoardingStatus::NotReady
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Ready),
            BoardingStatus::Submitted
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Boarded),
            BoardingStatus::Boarded
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Manual),
            BoardingStatus::ManualReview
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Closed),
            BoardingStatus::Closed
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Incomplete),
            BoardingStatus::Incomplete
        );
        assert_eq!(
            BoardingStatus::from(MerchantStatus::Pending),
            BoardingStatus::Pending
        );
    }

    #[test]
    fn test_boarding_status_display() {
        assert_eq!(format!("{}", BoardingStatus::NotReady), "Not Ready");
        assert_eq!(format!("{}", BoardingStatus::Submitted), "Submitted");
        assert_eq!(format!("{}", BoardingStatus::Boarded), "Boarded");
        assert_eq!(format!("{}", BoardingStatus::ManualReview), "Manual Review");
        assert_eq!(format!("{}", BoardingStatus::Closed), "Closed");
        assert_eq!(format!("{}", BoardingStatus::Incomplete), "Incomplete");
        assert_eq!(format!("{}", BoardingStatus::Pending), "Pending");
    }

    #[test]
    fn test_onboarding_payload_serialization() {
        let request = OnboardMerchantRequest {
            business: BusinessInfo {
                business_type: MerchantType::LimitedLiabilityCorporation,
                legal_name: "Test Business LLC".to_string(),
                address: Address {
                    line1: "123 Main St".to_string(),
                    line2: Some("Suite 100".to_string()),
                    city: "Springfield".to_string(),
                    state: "IL".to_string(),
                    zip: "62701".to_string(),
                    country: "USA".to_string(),
                },
                phone: "5551234567".to_string(),
                email: "test@example.com".to_string(),
                website: Some("https://example.com".to_string()),
                ein: "123456789".to_string(),
            },
            merchant: MerchantConfig {
                dba: "Test DBA".to_string(),
                mcc: "5999".to_string(),
                environment: MerchantEnvironment::Ecommerce,
                annual_cc_sales: 50000000,
                avg_ticket: 5000,
                established: DateYmd::new("20200101").unwrap(),
                is_new_business: false,
            },
            accounts: vec![BankAccountInfo {
                name: Some("Test Account".to_string()),
                routing_number: Some("123456789".to_string()),
                account_number: Some("987654321".to_string()),
                holder_type: AccountHolderType::Business,
                transaction_type: AccountType::All,
                currency: Some("USD".to_string()),
                is_primary: true,
                plaid_public_token: None,
            }],
            members: vec![MemberInfo {
                member_type: MemberType::Owner,
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                title: Some("CEO".to_string()),
                ownership_percentage: 100,
                date_of_birth: "19800115".to_string(),
                ssn: "123456789".to_string(),
                email: "john@example.com".to_string(),
                phone: "5559876543".to_string(),
                address: Address {
                    line1: "456 Oak Ave".to_string(),
                    line2: None,
                    city: "Springfield".to_string(),
                    state: "IL".to_string(),
                    zip: "62702".to_string(),
                    country: "USA".to_string(),
                },
            }],
            terms_acceptance: TermsAcceptance {
                version: "4.21".to_string(),
                accepted_at: "2024-01-15 10:30:00".to_string(),
            },
        };

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string_pretty(&payload).unwrap();

        // Verify key fields are present in serialized output
        // Note: to_string_pretty adds spaces, so we check for ": " format
        assert!(json.contains("\"type\": 2"), "Expected LLC type (2) in JSON: {}", json);
        assert!(json.contains("\"name\": \"Test Business LLC\""));
        assert!(json.contains("\"address1\": \"123 Main St\""));
        assert!(json.contains("\"tcVersion\": \"4.21\""));
        assert!(json.contains("\"tcAttestation\": 1"));
        assert!(json.contains("\"dba\": \"Test DBA\""));
        assert!(json.contains("\"mcc\": \"5999\""));
        assert!(json.contains("\"status\": 1")); // Board Immediately
        assert!(json.contains("\"primary\": 1"));
        assert!(json.contains("\"routing\": \"123456789\""));
        assert!(json.contains("\"first\": \"John\""));
        assert!(json.contains("\"ownership\": 100"));
    }

    #[test]
    fn test_account_payload_conversion() {
        // Test with manual entry (routing/account numbers)
        let account = BankAccountInfo {
            name: Some("Operating Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("987654321".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: None,
        };

        let payload: PayrixAccountPayload = account.into();
        assert_eq!(payload.primary, 1);
        assert_eq!(payload.name, Some("Operating Account".to_string()));
        assert_eq!(payload.transaction_type, AccountType::All);
        assert_eq!(payload.currency, Some("USD".to_string()));
        assert!(payload.account.is_some());
        let account_details = payload.account.unwrap();
        assert_eq!(account_details.routing, "123456789");
        assert_eq!(account_details.number, "987654321");
        assert_eq!(account_details.method, 8);
        assert_eq!(account_details.holder_type, AccountHolderType::Business);
    }

    #[test]
    fn test_account_payload_with_plaid() {
        // Test with Plaid token (no routing/account numbers)
        let account = BankAccountInfo {
            name: Some("Plaid Account".to_string()),
            routing_number: None,
            account_number: None,
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::Credit,  // Deposits only
            currency: Some("USD".to_string()),
            is_primary: false,
            plaid_public_token: Some("public-sandbox-xxx".to_string()),
        };

        let payload: PayrixAccountPayload = account.into();
        assert_eq!(payload.primary, 0);
        assert_eq!(payload.transaction_type, AccountType::Credit);
        assert!(payload.account.is_none());  // No manual account details
        assert_eq!(payload.public_token, Some("public-sandbox-xxx".to_string()));
    }

    #[test]
    fn test_member_payload_conversion() {
        let member = MemberInfo {
            member_type: MemberType::Owner,
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            title: Some("President".to_string()),
            ownership_percentage: 50,
            date_of_birth: "19850620".to_string(),
            ssn: "987654321".to_string(),
            email: "jane@example.com".to_string(),
            phone: "5551112222".to_string(),
            address: Address {
                line1: "789 Pine Rd".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60601".to_string(),
                country: "USA".to_string(),
            },
        };

        let payload: PayrixMemberPayload = member.into();
        assert_eq!(payload.first, "Jane");
        assert_eq!(payload.last, "Smith");
        assert_eq!(payload.title, Some("President".to_string()));
        assert_eq!(payload.ownership, 50);
        assert_eq!(payload.dob, "19850620");
        assert_eq!(payload.ssn, "987654321");
    }

    #[test]
    fn test_trust_and_operating_account_scenario() {
        // Test the common scenario of having two accounts:
        // 1. Operating account (All) - for deposits AND fee withdrawals
        // 2. Trust account (Credit only) - for deposits only, no fee withdrawals
        //
        // This is common for businesses that handle client funds (law firms,
        // escrow companies, property managers, etc.) where trust funds must
        // be kept separate from operating funds.

        let operating_account = BankAccountInfo {
            name: Some("Operating Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("111111111".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,  // Deposits AND fee withdrawals
            currency: Some("USD".to_string()),
            is_primary: true,  // Primary account for fees
            plaid_public_token: None,
        };

        let trust_account = BankAccountInfo {
            name: Some("Client Trust Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("222222222".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::Credit,  // Deposits ONLY - no fee withdrawals
            currency: Some("USD".to_string()),
            is_primary: false,  // Not primary - fees come from operating
            plaid_public_token: None,
        };

        // Convert to Payrix payloads
        let operating_payload: PayrixAccountPayload = operating_account.into();
        let trust_payload: PayrixAccountPayload = trust_account.into();

        // Verify operating account setup
        assert_eq!(operating_payload.primary, 1);
        assert_eq!(operating_payload.transaction_type, AccountType::All);
        assert_eq!(operating_payload.name, Some("Operating Account".to_string()));
        let operating_details = operating_payload.account.unwrap();
        assert_eq!(operating_details.number, "111111111");

        // Verify trust account setup
        assert_eq!(trust_payload.primary, 0);
        assert_eq!(trust_payload.transaction_type, AccountType::Credit);
        assert_eq!(trust_payload.name, Some("Client Trust Account".to_string()));
        let trust_details = trust_payload.account.unwrap();
        assert_eq!(trust_details.number, "222222222");

        // Verify the accounts serialize correctly for Payrix API
        let accounts = vec![
            BankAccountInfo {
                name: Some("Operating Account".to_string()),
                routing_number: Some("123456789".to_string()),
                account_number: Some("111111111".to_string()),
                holder_type: AccountHolderType::Business,
                transaction_type: AccountType::All,
                currency: Some("USD".to_string()),
                is_primary: true,
                plaid_public_token: None,
            },
            BankAccountInfo {
                name: Some("Client Trust Account".to_string()),
                routing_number: Some("123456789".to_string()),
                account_number: Some("222222222".to_string()),
                holder_type: AccountHolderType::Business,
                transaction_type: AccountType::Credit,
                currency: Some("USD".to_string()),
                is_primary: false,
                plaid_public_token: None,
            },
        ];

        let payloads: Vec<PayrixAccountPayload> = accounts.into_iter().map(Into::into).collect();
        let json = serde_json::to_string_pretty(&payloads).unwrap();

        // Verify both accounts appear in JSON
        assert!(json.contains("\"number\": \"111111111\""), "Operating account number should be in JSON");
        assert!(json.contains("\"number\": \"222222222\""), "Trust account number should be in JSON");
        // AccountType serializes as lowercase strings
        assert!(json.contains("\"type\": \"all\""), "Operating account should have type 'all'");
        assert!(json.contains("\"type\": \"credit\""), "Trust account should have type 'credit'");
    }

    // ============================================================================
    // Required Fields Tests
    // ============================================================================

    /// Helper to create a minimal valid OnboardMerchantRequest for testing
    fn create_test_request() -> OnboardMerchantRequest {
        OnboardMerchantRequest {
            business: BusinessInfo {
                business_type: MerchantType::LimitedLiabilityCorporation,
                legal_name: "Test Business LLC".to_string(),
                address: Address {
                    line1: "123 Main St".to_string(),
                    line2: None,
                    city: "Springfield".to_string(),
                    state: "IL".to_string(),
                    zip: "62701".to_string(),
                    country: "USA".to_string(),
                },
                phone: "5551234567".to_string(),
                email: "test@example.com".to_string(),
                website: None,
                ein: "123456789".to_string(),
            },
            merchant: MerchantConfig {
                dba: "Test DBA".to_string(),
                mcc: "5999".to_string(),
                environment: MerchantEnvironment::Ecommerce,
                annual_cc_sales: 50000000,
                avg_ticket: 5000,
                established: DateYmd::new("20200101").unwrap(),
                is_new_business: false,
            },
            accounts: vec![BankAccountInfo {
                name: None,
                routing_number: Some("123456789".to_string()),
                account_number: Some("987654321".to_string()),
                holder_type: AccountHolderType::Business,
                transaction_type: AccountType::All,
                currency: None,
                is_primary: true,
                plaid_public_token: None,
            }],
            members: vec![MemberInfo {
                member_type: MemberType::Owner,
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                title: None,
                ownership_percentage: 100,
                date_of_birth: "19800115".to_string(),
                ssn: "123456789".to_string(),
                email: "john@example.com".to_string(),
                phone: "5559876543".to_string(),
                address: Address {
                    line1: "456 Oak Ave".to_string(),
                    line2: None,
                    city: "Springfield".to_string(),
                    state: "IL".to_string(),
                    zip: "62702".to_string(),
                    country: "USA".to_string(),
                },
            }],
            terms_acceptance: TermsAcceptance {
                version: "4.21".to_string(),
                accepted_at: "2024-01-15 10:30:00".to_string(),
            },
        }
    }

    #[test]
    fn test_payload_contains_all_required_entity_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Required Entity fields
        assert!(json.contains("\"type\":"), "Missing required field: type (entity_type)");
        assert!(json.contains("\"name\":"), "Missing required field: name");
        assert!(json.contains("\"address1\":"), "Missing required field: address1");
        assert!(json.contains("\"city\":"), "Missing required field: city");
        assert!(json.contains("\"state\":"), "Missing required field: state");
        assert!(json.contains("\"zip\":"), "Missing required field: zip");
        assert!(json.contains("\"country\":"), "Missing required field: country");
        assert!(json.contains("\"phone\":"), "Missing required field: phone");
        assert!(json.contains("\"email\":"), "Missing required field: email");
        assert!(json.contains("\"ein\":"), "Missing required field: ein");

        // Terms & Conditions fields
        assert!(json.contains("\"tcVersion\":"), "Missing required field: tcVersion");
        assert!(json.contains("\"tcDate\":"), "Missing required field: tcDate");
        assert!(json.contains("\"tcAttestation\":"), "Missing required field: tcAttestation");

        // Nested required fields
        assert!(json.contains("\"accounts\":"), "Missing required field: accounts");
        assert!(json.contains("\"merchant\":"), "Missing required field: merchant");
    }

    #[test]
    fn test_payload_contains_all_required_merchant_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Deserialize to inspect merchant section
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let merchant = value.get("merchant").expect("merchant field missing");

        assert!(merchant.get("dba").is_some(), "Missing required merchant field: dba");
        assert!(merchant.get("mcc").is_some(), "Missing required merchant field: mcc");
        assert!(merchant.get("status").is_some(), "Missing required merchant field: status");
        assert!(merchant.get("environment").is_some(), "Missing required merchant field: environment");
        assert!(merchant.get("annualCcSales").is_some(), "Missing required merchant field: annualCcSales");
        assert!(merchant.get("avgTicket").is_some(), "Missing required merchant field: avgTicket");
        assert!(merchant.get("established").is_some(), "Missing required merchant field: established");
        assert!(merchant.get("new").is_some(), "Missing required merchant field: new (is_new_business)");
        assert!(merchant.get("members").is_some(), "Missing required merchant field: members");
    }

    #[test]
    fn test_payload_contains_all_required_account_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let accounts = value.get("accounts").expect("accounts field missing").as_array().unwrap();
        assert!(!accounts.is_empty(), "accounts array should not be empty");

        let account = &accounts[0];
        assert!(account.get("primary").is_some(), "Missing required account field: primary");
        assert!(account.get("type").is_some(), "Missing required account field: type");

        // For manual entry accounts, nested account details are required
        let account_details = account.get("account").expect("Missing account.account for manual entry");
        assert!(account_details.get("method").is_some(), "Missing required field: account.method");
        assert!(account_details.get("number").is_some(), "Missing required field: account.number");
        assert!(account_details.get("routing").is_some(), "Missing required field: account.routing");
        assert!(account_details.get("holderType").is_some(), "Missing required field: account.holderType");
    }

    #[test]
    fn test_payload_contains_all_required_member_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let members = value
            .get("merchant").expect("merchant missing")
            .get("members").expect("members missing")
            .as_array().unwrap();
        assert!(!members.is_empty(), "members array should not be empty");

        let member = &members[0];
        assert!(member.get("type").is_some(), "Missing required member field: type");
        assert!(member.get("first").is_some(), "Missing required member field: first");
        assert!(member.get("last").is_some(), "Missing required member field: last");
        assert!(member.get("ownership").is_some(), "Missing required member field: ownership");
        assert!(member.get("dob").is_some(), "Missing required member field: dob");
        assert!(member.get("ssn").is_some(), "Missing required member field: ssn");
        assert!(member.get("email").is_some(), "Missing required member field: email");
        assert!(member.get("phone").is_some(), "Missing required member field: phone");
        assert!(member.get("address1").is_some(), "Missing required member field: address1");
        assert!(member.get("city").is_some(), "Missing required member field: city");
        assert!(member.get("state").is_some(), "Missing required member field: state");
        assert!(member.get("zip").is_some(), "Missing required member field: zip");
        assert!(member.get("country").is_some(), "Missing required member field: country");
    }

    // ============================================================================
    // Read-Only Fields Tests
    // ============================================================================

    #[test]
    fn test_payload_excludes_entity_readonly_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // These fields are read-only and should NOT be in the serialized payload
        // They are returned by the Payrix API but should never be sent
        assert!(!json.contains("\"id\":"), "Read-only field 'id' should not be serialized");
        assert!(!json.contains("\"created\":"), "Read-only field 'created' should not be serialized");
        assert!(!json.contains("\"modified\":"), "Read-only field 'modified' should not be serialized");
        assert!(!json.contains("\"login\":"), "Read-only field 'login' should not be serialized");
        assert!(!json.contains("\"frozen\":"), "Read-only field 'frozen' should not be serialized");
        assert!(!json.contains("\"inactive\":"), "Read-only field 'inactive' should not be serialized");
    }

    #[test]
    fn test_payload_excludes_account_readonly_fields() {
        let account = BankAccountInfo {
            name: Some("Test Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("987654321".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: None,
        };

        let payload: PayrixAccountPayload = account.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Account read-only fields that should NOT be serialized
        assert!(!json.contains("\"id\":"), "Read-only field 'id' should not be in account payload");
        assert!(!json.contains("\"entity\":"), "Read-only field 'entity' should not be in account payload");
        assert!(!json.contains("\"merchant\":"), "Read-only field 'merchant' should not be in account payload");
        assert!(!json.contains("\"login\":"), "Read-only field 'login' should not be in account payload");
        assert!(!json.contains("\"last4\":"), "Read-only field 'last4' should not be in account payload");
        assert!(!json.contains("\"status\":"), "Read-only field 'status' should not be in account payload");
        assert!(!json.contains("\"verified\":"), "Read-only field 'verified' should not be in account payload");
        assert!(!json.contains("\"created\":"), "Read-only field 'created' should not be in account payload");
        assert!(!json.contains("\"modified\":"), "Read-only field 'modified' should not be in account payload");
        assert!(!json.contains("\"frozen\":"), "Read-only field 'frozen' should not be in account payload");
        assert!(!json.contains("\"inactive\":"), "Read-only field 'inactive' should not be in account payload");
    }

    #[test]
    fn test_payload_excludes_member_readonly_fields() {
        let member = MemberInfo {
            member_type: MemberType::Owner,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            title: None,
            ownership_percentage: 100,
            date_of_birth: "19800115".to_string(),
            ssn: "123456789".to_string(),
            email: "john@example.com".to_string(),
            phone: "5559876543".to_string(),
            address: Address {
                line1: "456 Oak Ave".to_string(),
                line2: None,
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62702".to_string(),
                country: "USA".to_string(),
            },
        };

        let payload: PayrixMemberPayload = member.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Member read-only fields that should NOT be serialized
        assert!(!json.contains("\"id\":"), "Read-only field 'id' should not be in member payload");
        assert!(!json.contains("\"entity\":"), "Read-only field 'entity' should not be in member payload");
        assert!(!json.contains("\"merchant\":"), "Read-only field 'merchant' should not be in member payload");
        assert!(!json.contains("\"login\":"), "Read-only field 'login' should not be in member payload");
        assert!(!json.contains("\"created\":"), "Read-only field 'created' should not be in member payload");
        assert!(!json.contains("\"modified\":"), "Read-only field 'modified' should not be in member payload");
        assert!(!json.contains("\"frozen\":"), "Read-only field 'frozen' should not be in member payload");
        assert!(!json.contains("\"inactive\":"), "Read-only field 'inactive' should not be in member payload");
    }

    #[test]
    fn test_payload_excludes_merchant_readonly_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let merchant = value.get("merchant").expect("merchant field missing");
        let merchant_json = serde_json::to_string(merchant).unwrap();

        // Merchant read-only fields that should NOT be serialized
        // Note: The "status" field IS included because we set it to 1 (Board Immediately)
        // This is a create-time value, not a read-only response value
        assert!(!merchant_json.contains("\"id\":"), "Read-only field 'id' should not be in merchant payload");
        assert!(!merchant_json.contains("\"entity\":"), "Read-only field 'entity' should not be in merchant payload");
        assert!(!merchant_json.contains("\"login\":"), "Read-only field 'login' should not be in merchant payload");
        assert!(!merchant_json.contains("\"created\":"), "Read-only field 'created' should not be in merchant payload");
        assert!(!merchant_json.contains("\"modified\":"), "Read-only field 'modified' should not be in merchant payload");
        assert!(!merchant_json.contains("\"frozen\":"), "Read-only field 'frozen' should not be in merchant payload");
        assert!(!merchant_json.contains("\"inactive\":"), "Read-only field 'inactive' should not be in merchant payload");
        assert!(!merchant_json.contains("\"boarded\":"), "Read-only field 'boarded' should not be in merchant payload");
    }

    // ============================================================================
    // Serialization Format Tests
    // ============================================================================

    #[test]
    fn test_payload_uses_camel_case() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Verify camelCase is used (not snake_case)
        assert!(json.contains("\"tcVersion\":"), "Should use camelCase: tcVersion");
        assert!(json.contains("\"tcDate\":"), "Should use camelCase: tcDate");
        assert!(json.contains("\"tcAttestation\":"), "Should use camelCase: tcAttestation");
        assert!(json.contains("\"annualCcSales\":"), "Should use camelCase: annualCcSales");
        assert!(json.contains("\"avgTicket\":"), "Should use camelCase: avgTicket");
        assert!(json.contains("\"holderType\":"), "Should use camelCase: holderType");

        // Verify snake_case is NOT used
        assert!(!json.contains("\"tc_version\":"), "Should not use snake_case");
        assert!(!json.contains("\"tc_date\":"), "Should not use snake_case");
        assert!(!json.contains("\"annual_cc_sales\":"), "Should not use snake_case");
        assert!(!json.contains("\"avg_ticket\":"), "Should not use snake_case");
        assert!(!json.contains("\"holder_type\":"), "Should not use snake_case");
    }

    #[test]
    fn test_payload_skips_none_optional_fields() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Optional fields that are None should not appear in JSON
        // In our test request, website is None
        assert!(!json.contains("\"website\":"), "Optional None field 'website' should not be serialized");

        // address2 is also None in our test request
        assert!(!json.contains("\"address2\":"), "Optional None field 'address2' should not be serialized");
    }

    #[test]
    fn test_payload_includes_some_optional_fields() {
        let mut request = create_test_request();
        request.business.website = Some("https://example.com".to_string());
        request.business.address.line2 = Some("Suite 100".to_string());

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Optional fields that are Some should appear in JSON
        assert!(json.contains("\"website\":\"https://example.com\""), "Optional Some field 'website' should be serialized");
        assert!(json.contains("\"address2\":\"Suite 100\""), "Optional Some field 'address2' should be serialized");
    }

    // ============================================================================
    // Value Correctness Tests
    // ============================================================================

    #[test]
    fn test_boarding_status_is_board_immediately() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let merchant = value.get("merchant").expect("merchant missing");
        let status = merchant.get("status").expect("status missing").as_i64().unwrap();

        // Status should be 1 (Board Immediately) for onboarding requests
        assert_eq!(status, 1, "Merchant status should be 1 (Board Immediately)");
    }

    #[test]
    fn test_tc_attestation_is_one() {
        let request = create_test_request();
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let tc_attestation = value.get("tcAttestation").expect("tcAttestation missing").as_i64().unwrap();

        // Terms attestation should always be 1 (accepted)
        assert_eq!(tc_attestation, 1, "tcAttestation should be 1");
    }

    #[test]
    fn test_primary_account_flag_serialization() {
        // Primary account
        let primary_account = BankAccountInfo {
            name: None,
            routing_number: Some("123456789".to_string()),
            account_number: Some("111111111".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: None,
            is_primary: true,
            plaid_public_token: None,
        };
        let primary_payload: PayrixAccountPayload = primary_account.into();
        assert_eq!(primary_payload.primary, 1, "Primary account should have primary=1");

        // Non-primary account
        let secondary_account = BankAccountInfo {
            name: None,
            routing_number: Some("123456789".to_string()),
            account_number: Some("222222222".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::Credit,
            currency: None,
            is_primary: false,
            plaid_public_token: None,
        };
        let secondary_payload: PayrixAccountPayload = secondary_account.into();
        assert_eq!(secondary_payload.primary, 0, "Non-primary account should have primary=0");
    }

    #[test]
    fn test_new_business_flag_serialization() {
        // Established business
        let mut request = create_test_request();
        request.merchant.is_new_business = false;
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let new_flag = value.get("merchant").unwrap().get("new").unwrap().as_i64().unwrap();
        assert_eq!(new_flag, 0, "Established business should have new=0");

        // New business
        let mut request = create_test_request();
        request.merchant.is_new_business = true;
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let new_flag = value.get("merchant").unwrap().get("new").unwrap().as_i64().unwrap();
        assert_eq!(new_flag, 1, "New business should have new=1");
    }

    #[test]
    fn test_account_method_is_checking() {
        let account = BankAccountInfo {
            name: None,
            routing_number: Some("123456789".to_string()),
            account_number: Some("987654321".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: None,
            is_primary: true,
            plaid_public_token: None,
        };

        let payload: PayrixAccountPayload = account.into();
        let account_details = payload.account.expect("account details missing");

        // Method should be 8 (checking account)
        assert_eq!(account_details.method, 8, "Account method should be 8 (checking)");
    }

    // ============================================================================
    // Type Enum Serialization Tests
    // ============================================================================

    #[test]
    fn test_entity_type_serialization() {
        // Test that MerchantType (entity type) serializes to correct integer
        let mut request = create_test_request();
        request.business.business_type = MerchantType::LimitedLiabilityCorporation;
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // LLC should serialize to 2
        assert!(json.contains("\"type\":2"), "LLC should serialize to type=2, got: {}", json);
    }

    #[test]
    fn test_member_type_serialization() {
        // Owner type
        let mut member = MemberInfo {
            member_type: MemberType::Owner,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            title: None,
            ownership_percentage: 100,
            date_of_birth: "19800115".to_string(),
            ssn: "123456789".to_string(),
            email: "john@example.com".to_string(),
            phone: "5559876543".to_string(),
            address: Address {
                line1: "456 Oak Ave".to_string(),
                line2: None,
                city: "Springfield".to_string(),
                state: "IL".to_string(),
                zip: "62702".to_string(),
                country: "USA".to_string(),
            },
        };
        let payload: PayrixMemberPayload = member.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":1"), "Owner should serialize to type=1");

        // ControlPerson type
        member.member_type = MemberType::ControlPerson;
        let payload: PayrixMemberPayload = member.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":2"), "ControlPerson should serialize to type=2");

        // Principal type
        member.member_type = MemberType::Principal;
        let payload: PayrixMemberPayload = member.into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":3"), "Principal should serialize to type=3");
    }

    #[test]
    fn test_account_type_serialization() {
        // All type
        let mut account = BankAccountInfo {
            name: None,
            routing_number: Some("123456789".to_string()),
            account_number: Some("987654321".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: None,
            is_primary: true,
            plaid_public_token: None,
        };
        let payload: PayrixAccountPayload = account.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"all\""), "AccountType::All should serialize to 'all'");

        // Credit type
        account.transaction_type = AccountType::Credit;
        let payload: PayrixAccountPayload = account.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"credit\""), "AccountType::Credit should serialize to 'credit'");

        // Debit type
        account.transaction_type = AccountType::Debit;
        let payload: PayrixAccountPayload = account.into();
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"debit\""), "AccountType::Debit should serialize to 'debit'");
    }

    #[test]
    fn test_account_holder_type_serialization() {
        // Business holder type
        let mut account = BankAccountInfo {
            name: None,
            routing_number: Some("123456789".to_string()),
            account_number: Some("987654321".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: None,
            is_primary: true,
            plaid_public_token: None,
        };
        let payload: PayrixAccountPayload = account.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        // Business should serialize to 2
        assert!(json.contains("\"holderType\":2"), "AccountHolderType::Business should serialize to 2, got: {}", json);

        // Individual holder type
        account.holder_type = AccountHolderType::Individual;
        let payload: PayrixAccountPayload = account.into();
        let json = serde_json::to_string(&payload).unwrap();
        // Individual should serialize to 1
        assert!(json.contains("\"holderType\":1"), "AccountHolderType::Individual should serialize to 1, got: {}", json);
    }

    #[test]
    fn test_environment_serialization() {
        let mut request = create_test_request();

        // Ecommerce environment
        request.merchant.environment = MerchantEnvironment::Ecommerce;
        let payload: PayrixOnboardingPayload = request.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let env = value.get("merchant").unwrap().get("environment").unwrap().as_str().unwrap();
        assert_eq!(env, "ecommerce", "Ecommerce should serialize to 'ecommerce'");

        // CardPresent (retail) environment
        request.merchant.environment = MerchantEnvironment::CardPresent;
        let payload: PayrixOnboardingPayload = request.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let env = value.get("merchant").unwrap().get("environment").unwrap().as_str().unwrap();
        assert_eq!(env, "cardPresent", "CardPresent should serialize to 'cardPresent'");

        // Restaurant environment
        request.merchant.environment = MerchantEnvironment::Restaurant;
        let payload: PayrixOnboardingPayload = request.clone().into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let env = value.get("merchant").unwrap().get("environment").unwrap().as_str().unwrap();
        assert_eq!(env, "restaurant", "Restaurant should serialize to 'restaurant'");

        // MailOrTelephoneOrder (MOTO) environment
        request.merchant.environment = MerchantEnvironment::MailOrTelephoneOrder;
        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let env = value.get("merchant").unwrap().get("environment").unwrap().as_str().unwrap();
        assert_eq!(env, "moto", "MailOrTelephoneOrder should serialize to 'moto'");
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_plaid_account_omits_manual_details() {
        let account = BankAccountInfo {
            name: Some("Plaid Verified Account".to_string()),
            routing_number: None,  // No manual entry
            account_number: None,  // No manual entry
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::All,
            currency: Some("USD".to_string()),
            is_primary: true,
            plaid_public_token: Some("public-sandbox-token".to_string()),
        };

        let payload: PayrixAccountPayload = account.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Should have public_token but not nested account details
        assert!(json.contains("\"publicToken\":\"public-sandbox-token\""), "Should include publicToken");
        assert!(!json.contains("\"account\":"), "Should not include nested account when using Plaid, got: {}", json);
        assert!(!json.contains("\"routing\":"), "Should not include routing when using Plaid");
        assert!(!json.contains("\"number\":"), "Should not include number when using Plaid");
    }

    #[test]
    fn test_multiple_members_serialization() {
        let mut request = create_test_request();

        // Add a second member (control person)
        request.members.push(MemberInfo {
            member_type: MemberType::ControlPerson,
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            title: Some("CFO".to_string()),
            ownership_percentage: 0,  // Control persons may not have ownership
            date_of_birth: "19850620".to_string(),
            ssn: "987654321".to_string(),
            email: "jane@example.com".to_string(),
            phone: "5551112222".to_string(),
            address: Address {
                line1: "789 Pine Rd".to_string(),
                line2: None,
                city: "Chicago".to_string(),
                state: "IL".to_string(),
                zip: "60601".to_string(),
                country: "USA".to_string(),
            },
        });

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let members = value.get("merchant").unwrap().get("members").unwrap().as_array().unwrap();

        assert_eq!(members.len(), 2, "Should have 2 members");

        // Verify first member (Owner)
        assert_eq!(members[0].get("first").unwrap().as_str().unwrap(), "John");
        assert_eq!(members[0].get("type").unwrap().as_i64().unwrap(), 1); // Owner

        // Verify second member (ControlPerson)
        assert_eq!(members[1].get("first").unwrap().as_str().unwrap(), "Jane");
        assert_eq!(members[1].get("type").unwrap().as_i64().unwrap(), 2); // ControlPerson
        assert_eq!(members[1].get("title").unwrap().as_str().unwrap(), "CFO");
    }

    #[test]
    fn test_multiple_accounts_serialization() {
        let mut request = create_test_request();

        // Add trust account
        request.accounts.push(BankAccountInfo {
            name: Some("Trust Account".to_string()),
            routing_number: Some("123456789".to_string()),
            account_number: Some("222222222".to_string()),
            holder_type: AccountHolderType::Business,
            transaction_type: AccountType::Credit,  // Deposits only
            currency: Some("USD".to_string()),
            is_primary: false,
            plaid_public_token: None,
        });

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let accounts = value.get("accounts").unwrap().as_array().unwrap();

        assert_eq!(accounts.len(), 2, "Should have 2 accounts");

        // First account is primary (All type)
        assert_eq!(accounts[0].get("primary").unwrap().as_i64().unwrap(), 1);
        assert_eq!(accounts[0].get("type").unwrap().as_str().unwrap(), "all");

        // Second account is trust (Credit only)
        assert_eq!(accounts[1].get("primary").unwrap().as_i64().unwrap(), 0);
        assert_eq!(accounts[1].get("type").unwrap().as_str().unwrap(), "credit");
        assert_eq!(accounts[1].get("name").unwrap().as_str().unwrap(), "Trust Account");
    }

    // ============================================================================
    // Response Parsing Tests
    // ============================================================================

    #[test]
    fn test_payrix_onboarding_response_deserialize() {
        // Test parsing of the nested response structure from Payrix
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "merchant": {
                "id": "t1_mer_23456789012345678901234",
                "status": 2,
                "entity": "t1_ent_12345678901234567890123",
                "boarded": "20240115"
            },
            "accounts": [
                {
                    "id": "t1_acc_34567890123456789012345",
                    "entity": "t1_ent_12345678901234567890123",
                    "primary": 1,
                    "type": "all",
                    "status": 1,
                    "inactive": 0,
                    "frozen": 0
                }
            ]
        }"#;

        let response: PayrixOnboardingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "t1_ent_12345678901234567890123");

        let merchant = response.merchant.unwrap();
        assert_eq!(merchant.id, "t1_mer_23456789012345678901234");
        assert_eq!(merchant.status, Some(MerchantStatus::Boarded));
        assert_eq!(merchant.boarded, Some("20240115".to_string()));

        let accounts = response.accounts.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].id.as_str(), "t1_acc_34567890123456789012345");
    }

    #[test]
    fn test_payrix_onboarding_response_minimal() {
        // Test parsing with minimal response (only id)
        let json = r#"{
            "id": "t1_ent_12345678901234567890123"
        }"#;

        let response: PayrixOnboardingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "t1_ent_12345678901234567890123");
        assert!(response.merchant.is_none());
        assert!(response.accounts.is_none());
    }

    #[test]
    fn test_merchant_in_response_deserialize() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "status": 6,
            "entity": "t1_ent_23456789012345678901234"
        }"#;

        let merchant: MerchantInResponse = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id, "t1_mer_12345678901234567890123");
        assert_eq!(merchant.status, Some(MerchantStatus::Pending));
        assert_eq!(merchant.entity, Some("t1_ent_23456789012345678901234".to_string()));
        assert!(merchant.boarded.is_none());
        assert!(merchant.members.is_none());
    }

    #[test]
    fn test_merchant_in_response_with_members() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "status": 2,
            "members": [
                {
                    "id": "t1_mem_34567890123456789012345",
                    "first": "John",
                    "last": "Doe"
                }
            ]
        }"#;

        let merchant: MerchantInResponse = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id, "t1_mer_12345678901234567890123");
        let members = merchant.members.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].first, Some("John".to_string()));
        assert_eq!(members[0].last, Some("Doe".to_string()));
    }

    // ============================================================================
    // BoardingStatus Tests
    // ============================================================================

    #[test]
    fn test_boarding_status_all_variants() {
        // Verify all MerchantStatus values map correctly to BoardingStatus
        let test_cases = vec![
            (MerchantStatus::NotReady, BoardingStatus::NotReady),
            (MerchantStatus::Ready, BoardingStatus::Submitted),
            (MerchantStatus::Boarded, BoardingStatus::Boarded),
            (MerchantStatus::Manual, BoardingStatus::ManualReview),
            (MerchantStatus::Closed, BoardingStatus::Closed),
            (MerchantStatus::Incomplete, BoardingStatus::Incomplete),
            (MerchantStatus::Pending, BoardingStatus::Pending),
        ];

        for (merchant_status, expected) in test_cases {
            let actual = BoardingStatus::from(merchant_status);
            assert_eq!(actual, expected, "MerchantStatus::{:?} should map to BoardingStatus::{:?}", merchant_status, expected);
        }
    }

    #[test]
    fn test_boarding_status_equality() {
        assert_eq!(BoardingStatus::Boarded, BoardingStatus::Boarded);
        assert_ne!(BoardingStatus::Boarded, BoardingStatus::Pending);
    }

    #[test]
    fn test_boarding_status_clone() {
        let status = BoardingStatus::ManualReview;
        let cloned = status; // BoardingStatus implements Copy
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_boarding_status_copy() {
        let status = BoardingStatus::Boarded;
        let copied = status; // Copy trait
        assert_eq!(status, copied);
    }

    // ============================================================================
    // OnboardMerchantResult Tests
    // ============================================================================

    #[test]
    fn test_onboard_merchant_result_fields() {
        // Test that OnboardMerchantResult has the expected fields
        // In real usage, this would be populated by the API call

        // Simulate parsing a response JSON to create Entity and Merchant
        let entity_json = r#"{"id": "t1_ent_12345678901234567890123"}"#;
        let merchant_json = r#"{"id": "t1_mer_23456789012345678901234"}"#;

        let entity: Entity = serde_json::from_str(entity_json).unwrap();
        let merchant: Merchant = serde_json::from_str(merchant_json).unwrap();

        let result = OnboardMerchantResult {
            entity_id: entity.id.as_str().to_string(),
            merchant_id: merchant.id.as_str().to_string(),
            boarding_status: BoardingStatus::Boarded,
            entity,
            merchant,
            accounts: vec![],
            members: vec![],
        };

        assert_eq!(result.entity_id, "t1_ent_12345678901234567890123");
        assert_eq!(result.merchant_id, "t1_mer_23456789012345678901234");
        assert_eq!(result.boarding_status, BoardingStatus::Boarded);
        assert!(result.accounts.is_empty());
        assert!(result.members.is_empty());
    }

    // ============================================================================
    // BoardingStatusResult Tests
    // ============================================================================

    #[test]
    fn test_boarding_status_result_structure() {
        let result = BoardingStatusResult {
            status: BoardingStatus::Pending,
            merchant_id: "t1_mer_12345678901234567890123".to_string(),
            entity_id: "t1_ent_23456789012345678901234".to_string(),
            boarded_date: None,
        };

        assert_eq!(result.status, BoardingStatus::Pending);
        assert_eq!(result.merchant_id, "t1_mer_12345678901234567890123");
        assert!(result.boarded_date.is_none());
    }

    #[test]
    fn test_boarding_status_result_with_boarded_date() {
        let result = BoardingStatusResult {
            status: BoardingStatus::Boarded,
            merchant_id: "t1_mer_12345678901234567890123".to_string(),
            entity_id: "t1_ent_23456789012345678901234".to_string(),
            boarded_date: Some("20240115".to_string()),
        };

        assert_eq!(result.status, BoardingStatus::Boarded);
        assert_eq!(result.boarded_date, Some("20240115".to_string()));
    }

    // ============================================================================
    // Validation Edge Case Tests
    // ============================================================================

    #[test]
    fn test_empty_accounts_serialization() {
        let mut request = create_test_request();
        request.accounts = vec![];  // Empty accounts

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Should still serialize with empty array
        assert!(json.contains("\"accounts\":[]"), "Empty accounts should serialize to empty array");
    }

    #[test]
    fn test_empty_members_serialization() {
        let mut request = create_test_request();
        request.members = vec![];  // Empty members

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let merchant = value.get("merchant").unwrap();
        let members = merchant.get("members").unwrap().as_array().unwrap();

        // Should still serialize with empty array
        assert!(members.is_empty(), "Empty members should serialize to empty array");
    }

    #[test]
    fn test_special_characters_in_strings() {
        let mut request = create_test_request();
        request.business.legal_name = "O'Reilly & Sons, LLC \"Test\"".to_string();
        request.business.address.line1 = "123 Main St. #456".to_string();

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Should properly escape special characters
        assert!(json.contains("O'Reilly"), "Single quote should be preserved");
        assert!(json.contains("& Sons"), "Ampersand should be preserved");
        assert!(json.contains("#456"), "Hash should be preserved");

        // Verify JSON is valid by parsing it back
        let _: serde_json::Value = serde_json::from_str(&json)
            .expect("JSON with special characters should be valid");
    }

    #[test]
    fn test_unicode_in_strings() {
        let mut request = create_test_request();
        request.business.legal_name = "Café München LLC".to_string();
        request.members[0].first_name = "José".to_string();

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        // Should preserve Unicode characters
        assert!(json.contains("Café"), "Unicode é should be preserved");
        assert!(json.contains("München"), "Unicode ü should be preserved");
        assert!(json.contains("José"), "Unicode é in name should be preserved");
    }

    #[test]
    fn test_max_ownership_percentage() {
        let mut request = create_test_request();
        request.members[0].ownership_percentage = 100;

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let members = value.get("merchant").unwrap().get("members").unwrap().as_array().unwrap();
        let ownership = members[0].get("ownership").unwrap().as_i64().unwrap();

        assert_eq!(ownership, 100);
    }

    #[test]
    fn test_zero_ownership_percentage() {
        let mut request = create_test_request();
        request.members[0].member_type = MemberType::ControlPerson;
        request.members[0].ownership_percentage = 0;  // Control persons may have 0% ownership

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let members = value.get("merchant").unwrap().get("members").unwrap().as_array().unwrap();
        let ownership = members[0].get("ownership").unwrap().as_i64().unwrap();

        assert_eq!(ownership, 0);
    }

    #[test]
    fn test_large_annual_sales() {
        let mut request = create_test_request();
        request.merchant.annual_cc_sales = 10_000_000_000; // $100 million in cents

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let annual_cc_sales = value.get("merchant").unwrap().get("annualCcSales").unwrap().as_i64().unwrap();

        assert_eq!(annual_cc_sales, 10_000_000_000);
    }

    #[test]
    fn test_address_line2_with_apartment() {
        let mut request = create_test_request();
        request.business.address.line2 = Some("Apt 4B, Floor 12".to_string());
        request.members[0].address.line2 = Some("Unit #789".to_string());

        let payload: PayrixOnboardingPayload = request.into();
        let json = serde_json::to_string(&payload).unwrap();

        assert!(json.contains("\"address2\":\"Apt 4B, Floor 12\""), "Business address2 should be present");

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let members = value.get("merchant").unwrap().get("members").unwrap().as_array().unwrap();
        let member_address2 = members[0].get("address2").unwrap().as_str().unwrap();

        assert_eq!(member_address2, "Unit #789");
    }
}
