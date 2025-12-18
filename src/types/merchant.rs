//! Merchant types for the Payrix API.
//!
//! Merchants represent payment processing accounts under entities.
//!
//! **OpenAPI schema:** `merchantsResponse`

use payrix_macros::PayrixEntity;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, DateYmd, PayrixId};

// =============================================================================
// Enums
// =============================================================================

/// Merchant type (business structure) per OpenAPI spec.
///
/// **OpenAPI schema:** `merchantType`
///
/// Valid values:
/// - `0` - Sole Proprietor
/// - `1` - Corporation
/// - `2` - Limited Liability Corporation (LLC)
/// - `3` - Partnership
/// - `5` - Non-Profit Organization
/// - `6` - Government Organization
/// - `7` - C-Corporation
/// - `8` - S-Corporation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MerchantType {
    /// Sole proprietor (individual owner).
    #[default]
    SoleProprietor = 0,
    /// Corporation.
    Corporation = 1,
    /// Limited Liability Corporation (LLC).
    LimitedLiabilityCorporation = 2,
    /// Partnership (general or limited).
    Partnership = 3,
    /// Non-profit organization.
    NonProfitOrganization = 5,
    /// Government organization.
    GovernmentOrganization = 6,
    /// C-Corporation.
    CCorporation = 7,
    /// S-Corporation.
    SCorporation = 8,
}

/// Merchant status per OpenAPI spec.
///
/// **OpenAPI schema:** `merchantStatus`
///
/// Valid values:
/// - `0` - Not ready. Occurs when a new Merchant is created. Unable to process payments.
/// - `1` - Ready. New Merchant submitted for underwriting approval.
/// - `2` - Boarded. Merchant has been approved. Payment processing now available.
/// - `3` - Manual. New Merchant is pending manual verification.
/// - `4` - Closed. New Merchant was declined and cannot access the platform.
/// - `5` - Incomplete. Can be manually set to save an incomplete boarding request.
/// - `6` - Pending. New Merchant was submitted for boarding, pending review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MerchantStatus {
    /// Not ready for processing. Occurs when a new Merchant is created.
    #[default]
    NotReady = 0,
    /// Ready. Submitted for underwriting approval.
    Ready = 1,
    /// Boarded. Approved and ready to process payments.
    Boarded = 2,
    /// Manual. Pending manual verification.
    Manual = 3,
    /// Closed. Declined and cannot access the platform.
    Closed = 4,
    /// Incomplete. Draft saved for later completion.
    Incomplete = 5,
    /// Pending. Submitted for boarding, pending review.
    Pending = 6,
}

/// Merchant environment per OpenAPI spec.
///
/// **OpenAPI schema:** `merchantEnvironment`
///
/// Valid values:
/// - `supermarket` - Supermarkets / Grocery
/// - `moto` - Mail Order / Telephone Order
/// - `cardPresent` - Card Present Environment
/// - `fuel` - Fuel / Gas Stations
/// - `serviceStation` - Service Stations
/// - `restaurant` - Restaurants
/// - `ecommerce` - eCommerce / Online Sales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MerchantEnvironment {
    /// Supermarkets / Grocery.
    Supermarket,
    /// Mail Order / Telephone Order.
    #[serde(rename = "moto")]
    MailOrTelephoneOrder,
    /// Card Present Environment.
    CardPresent,
    /// Fuel / Gas Stations.
    Fuel,
    /// Service Stations.
    ServiceStation,
    /// Restaurants.
    Restaurant,
    /// eCommerce / Online Sales.
    #[default]
    #[serde(rename = "ecommerce", alias = "eCommerce")]
    Ecommerce,
}

/// Risk level per OpenAPI spec.
///
/// **OpenAPI schema:** `RiskLevel`
///
/// Valid values:
/// - `restricted` - Restricted from processing transactions.
/// - `prohibited` - Prohibited business use category.
/// - `high` - High merchant risk score.
/// - `medium` - Medium merchant risk score.
/// - `low` - Low merchant risk score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    /// Restricted from processing transactions.
    Restricted,
    /// Prohibited business use category.
    Prohibited,
    /// High merchant risk score.
    High,
    /// Medium merchant risk score.
    Medium,
    /// Low merchant risk score.
    #[default]
    Low,
}

/// Tax ID (TIN) status per OpenAPI spec.
///
/// **OpenAPI schema:** `TinStatus`
///
/// Valid values:
/// - `0` - Pending verification
/// - `1` - Valid (verified successfully)
/// - `2` - Invalid (verification failed)
/// - `3` - Not required
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TaxIdStatus {
    /// Pending verification.
    #[default]
    Pending = 0,
    /// Valid (verified successfully).
    Valid = 1,
    /// Invalid (verification failed).
    Invalid = 2,
    /// Not required.
    NotRequired = 3,
}

/// SAQ (Self-Assessment Questionnaire) type per OpenAPI spec.
///
/// **OpenAPI schema:** `SaqType`
///
/// PCI DSS Self-assessment questionnaire type. Details about a merchant's
/// business and payment acceptance methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaqType {
    /// Card-not-present merchants with PCI DSS-validated third-party providers.
    #[serde(rename = "SAQ-A")]
    SaqA,
    /// E-commerce merchants with website that may impact cardholder data.
    #[serde(rename = "SAQ-A-EP")]
    SaqAEp,
    /// Merchants with imprint machines or standalone dial-out terminals.
    #[serde(rename = "SAQ-B")]
    SaqB,
    /// Merchants with standalone PTS-approved terminals with IP connection.
    #[serde(rename = "SAQ-B-IP")]
    SaqBIp,
    /// Merchants that enter single transactions manually via internet terminal.
    #[serde(rename = "SAQ-C-VT")]
    SaqCVt,
    /// Merchants with payment systems connected to the internet.
    #[serde(rename = "SAQ-C")]
    SaqC,
    /// Merchants with hardware-only terminals managed by P2PE solution.
    #[serde(rename = "SAQ-P2PE-HW")]
    SaqP2PeHw,
    /// Generic field for merchants not fitting other SAQ types.
    #[serde(rename = "SAQ-D")]
    SaqD,
}

/// Merchant location type per OpenAPI spec.
///
/// **OpenAPI schema:** `merchantLocationType`
///
/// Valid values:
/// - `77` - Retail Storefront
/// - `78` - Warehouse
/// - `79` - Private Residence
/// - `80` - Others
/// - `81` - P. RES-PROF/Construction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LocationType {
    /// Retail Storefront.
    RetailStorefront = 77,
    /// Warehouse.
    Warehouse = 78,
    /// Private Residence.
    PrivateResidence = 79,
    /// Others.
    Others = 80,
    /// P. RES-PROF/Construction.
    PrivateResidenceProfConstruction = 81,
}

/// Express batch close method per OpenAPI spec.
///
/// **OpenAPI schema:** `ExpressBatchCloseMethod`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressBatchCloseMethod {
    /// Batch close occurs at specific time intervals (Automated).
    TimeInitiated,
    /// Batch close occurs when initiated by the Merchant (Manual).
    MerchantInitiated,
}

// =============================================================================
// Merchant (Response)
// =============================================================================

/// A Payrix merchant.
///
/// A merchant represents a payment processing account under an entity.
/// Merchants handle the actual transaction processing configuration.
///
/// **OpenAPI schema:** `merchantsResponse`
///
/// See `API_INCONSISTENCIES.md` for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateMerchant, update = UpdateMerchant)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[payrix(readonly)]
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: `creator`)
    #[payrix(readonly)]
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The date and time on which this Merchant last processed a Transaction.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}`)
    #[serde(default)]
    pub last_activity: Option<String>,

    /// The total approved amount for this merchant.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total_approved_sales: Option<i64>,

    /// The Entity associated with this Merchant.
    ///
    /// **OpenAPI type:** string (ref: `merchantsModelEntity`)
    #[payrix(create_only)]
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Login ID associated with this merchant.
    ///
    /// **OpenAPI type:** string
    #[payrix(create_only)]
    #[serde(default)]
    pub login: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Business Information
    // -------------------------------------------------------------------------

    /// The name under which the Merchant is doing business, if applicable.
    ///
    /// This field is stored as a text string and must be between 0 and 50 characters long.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub dba: Option<String>,

    /// Whether the Merchant is new to credit card processing.
    ///
    /// Merchants are considered to be new by default.
    ///
    /// **OpenAPI type:** integer (ref: `New`)
    ///
    /// Valid values:
    /// - `0` - Not new
    /// - `1` - New
    #[serde(default, with = "bool_from_int_default_false")]
    pub new: bool,

    /// Whether incremental authorization is supported.
    ///
    /// Allows submission of merchant registration files with a MasterCard Auth Integrity value.
    ///
    /// **OpenAPI type:** integer (ref: `IncrementalAuthSupported`)
    ///
    /// Valid values:
    /// - `0` - Incremental Auth Not Supported
    /// - `1` - Incremental Auth Supported
    #[serde(default, with = "bool_from_int_default_false")]
    pub incremental_auth_supported: bool,

    /// Whether the merchant is a seasonal merchant or operates year-round.
    ///
    /// **OpenAPI type:** integer (ref: `Seasonal`)
    ///
    /// Valid values:
    /// - `0` - Year-Round Merchant
    /// - `1` - Seasonal Merchant
    #[serde(default, with = "bool_from_int_default_false")]
    pub seasonal: bool,

    /// Whether the merchant accepts pre-purchase for products shipped later.
    ///
    /// **OpenAPI type:** integer (ref: `AdvancedBilling`)
    ///
    /// Valid values:
    /// - `0` - AdvancedBilling is disabled
    /// - `1` - AdvancedBilling is enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub advanced_billing: bool,

    /// The date on which the Merchant was established.
    ///
    /// Format: YYYYMMDD (e.g., '20160120' for January 20, 2016).
    ///
    /// **Note:** Include the `established` date value to ensure successful underwriting.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub established: Option<DateYmd>,

    // -------------------------------------------------------------------------
    // Sales Volume Information
    // -------------------------------------------------------------------------

    /// The value of the annual credit card sales of this Merchant.
    ///
    /// This field is specified as an integer in cents. For example, $25.30 is '2530'.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, rename = "annualCCSales")]
    pub annual_cc_sales: Option<i64>,

    /// The value of the annual credit card sale volume of this Merchant.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, rename = "annualCCSaleVolume")]
    pub annual_cc_sale_volume: Option<i64>,

    /// The value of the annual ACH/direct deposit sale volume of this Merchant.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default, rename = "annualACHSaleVolume")]
    pub annual_ach_sale_volume: Option<i64>,

    /// The Annual AMEX Sales Volume for the outlet.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub amex_volume: Option<i64>,

    /// The value of the average credit card sales of this Merchant.
    ///
    /// This field is specified as an integer in cents.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub avg_ticket: Option<i64>,

    /// The expected total volume for all credit card and ACH payments.
    ///
    /// **OpenAPI type:** integer (int64)
    #[serde(default)]
    pub total_volume: Option<i64>,

    // -------------------------------------------------------------------------
    // Card Network Identifiers
    // -------------------------------------------------------------------------

    /// The American Express merchant identifier for this Merchant, if applicable.
    ///
    /// This field is stored as a text string and must be between 1 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub amex: Option<String>,

    /// The Discover merchant identifier for this Merchant, if applicable.
    ///
    /// This field is stored as a text string and must be between 1 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub discover: Option<String>,

    /// The Merchant Category Code (MCC) for this Merchant.
    ///
    /// This code is not required to create a Merchant, but it is required
    /// to successfully board a Merchant.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mcc: Option<String>,

    /// Merchant Verification Value.
    ///
    /// A value assigned by Visa to identify participation in select merchant programs.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub visa_mvv: Option<String>,

    /// Whether the merchant has acknowledged and accepted the Visa Disclosure.
    ///
    /// **OpenAPI type:** integer (ref: `VisaDisclosure`)
    ///
    /// Valid values:
    /// - `0` - Not Accepted
    /// - `1` - Accepted
    #[serde(default, with = "bool_from_int_default_false")]
    pub visa_disclosure: bool,

    // -------------------------------------------------------------------------
    // Status & Configuration
    // -------------------------------------------------------------------------

    /// The environment which the Merchant is in, if applicable.
    ///
    /// **OpenAPI type:** string (ref: `merchantEnvironment`)
    #[payrix(mutable)]
    #[serde(default)]
    pub environment: Option<MerchantEnvironment>,

    /// The status of the Merchant.
    ///
    /// **OpenAPI type:** integer (ref: `merchantStatus`)
    #[serde(default)]
    pub status: Option<MerchantStatus>,

    /// Whether the merchant auto-boarded.
    ///
    /// **OpenAPI type:** integer (ref: `AutoBoarded`)
    ///
    /// Valid values:
    /// - `0` - Not Auto-Boarded
    /// - `1` - Auto-Boarded
    #[serde(default, with = "bool_from_int_default_false")]
    pub auto_boarded: bool,

    /// The reason for manual or closed status.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub status_reason: Option<String>,

    /// The denial or account closure reason code.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub account_closure_reason_code: Option<String>,

    /// The date the account closure reason code was provided.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub account_closure_reason_date: Option<i32>,

    /// The risk level which the Merchant is in, if applicable.
    ///
    /// **OpenAPI type:** string (ref: `RiskLevel`)
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,

    /// The date and time on which this Merchant was successfully boarded.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub boarded: Option<DateYmd>,

    // -------------------------------------------------------------------------
    // Risk & Compliance
    // -------------------------------------------------------------------------

    /// The credit ratio to use while calculating risk factors for credit.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub credit_ratio: Option<i32>,

    /// The credit timeliness for the Merchant.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub credit_timeliness: Option<i32>,

    /// Chargeback Ratio.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub chargeback_ratio: Option<i32>,

    /// The NDX (Non Delivery Exposure) days.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub ndx_days: Option<i32>,

    /// The NDX (Non Delivery Exposure) percentage.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub ndx_percentage: Option<i32>,

    // -------------------------------------------------------------------------
    // PCI Compliance
    // -------------------------------------------------------------------------

    /// PCI DSS Self-assessment questionnaire (SAQ) type.
    ///
    /// **OpenAPI type:** string (ref: `SaqType`)
    #[serde(default)]
    pub saq_type: Option<SaqType>,

    /// The date of the PCI SAQ assessment.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub saq_date: Option<i32>,

    /// Qualified Security Assessor (QSA) business name.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub qsa: Option<String>,

    /// Whether the letter status is ON or OFF.
    ///
    /// **OpenAPI type:** integer (ref: `LetterStatus`)
    ///
    /// Valid values:
    /// - `0` - OFF
    /// - `1` - ON
    #[serde(default, with = "bool_from_int_default_false")]
    pub letter_status: bool,

    /// The date associated with the letter status.
    ///
    /// Format: YYYYMMDD
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub letter_date: Option<i32>,

    // -------------------------------------------------------------------------
    // Terms & Disclosure
    // -------------------------------------------------------------------------

    /// Whether the merchant has acknowledged and accepted the Terms and Conditions.
    ///
    /// **OpenAPI type:** integer (ref: `TcAttestation`)
    ///
    /// Valid values:
    /// - `0` - Not Accepted
    /// - `1` - Accepted
    #[serde(default, with = "bool_from_int_default_false")]
    pub tc_attestation: bool,

    /// Transaction Session Id.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub tmx_session_id: Option<String>,

    /// The IP address where the Merchant platform disclosure is hosted.
    ///
    /// **OpenAPI type:** string
    #[serde(default, rename = "disclosureIP")]
    pub disclosure_ip: Option<String>,

    /// The date the Merchant platform disclosure was reviewed.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub disclosure_date: Option<i32>,

    // -------------------------------------------------------------------------
    // Contact & Notifications
    // -------------------------------------------------------------------------

    /// Notification Email for chargebacks.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub chargeback_notification_email: Option<String>,

    // -------------------------------------------------------------------------
    // Location & Business Type
    // -------------------------------------------------------------------------

    /// Description of the type of address that the business operates at.
    ///
    /// **OpenAPI type:** integer (ref: `merchantLocationType`)
    #[serde(default)]
    pub location_type: Option<LocationType>,

    /// The merchant percentage of transactions that are Card Not Present.
    ///
    /// Includes MOTO and eCommerce.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub percent_keyed: Option<i32>,

    /// Percentage of internet originated transactions.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub percent_ecomm: Option<i32>,

    /// Percentage of transactions that are Business to Business.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub percent_business: Option<i32>,

    /// The NAICS sector code that describes the industry.
    ///
    /// **OpenAPI type:** integer (ref: `Naics`)
    #[serde(default)]
    pub naics: Option<i32>,

    /// The NAICS sector description matching the NAICS sector code.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub naics_description: Option<String>,

    // -------------------------------------------------------------------------
    // Digital Wallets
    // -------------------------------------------------------------------------

    /// Whether Apple Pay is active for this merchant.
    ///
    /// **OpenAPI type:** integer (ref: `ApplePayActive`)
    ///
    /// Valid values:
    /// - `0` - Inactive
    /// - `1` - Active
    #[serde(default, with = "bool_from_int_default_false")]
    pub apple_pay_active: bool,

    /// The status of Apple Pay for this merchant.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub apple_pay_status: Option<String>,

    /// Whether Google Pay is active for this merchant.
    ///
    /// **OpenAPI type:** integer (ref: `GooglePayActive`)
    ///
    /// Valid values:
    /// - `0` - Inactive
    /// - `1` - Active
    #[serde(default, with = "bool_from_int_default_false")]
    pub google_pay_active: bool,

    // -------------------------------------------------------------------------
    // Express Platform Configuration
    // -------------------------------------------------------------------------

    /// The batch close method for this merchant (Express platform).
    ///
    /// **OpenAPI type:** string (ref: `ExpressBatchCloseMethod`)
    #[serde(default)]
    pub express_batch_close_method: Option<ExpressBatchCloseMethod>,

    /// The batch close time for this merchant (Express platform).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub express_batch_close_time: Option<String>,

    /// Whether Pass Token is enabled for this merchant.
    ///
    /// **OpenAPI type:** integer (ref: `PassTokenEnabled`)
    ///
    /// Valid values:
    /// - `0` - Disabled
    /// - `1` - Enabled
    #[serde(default, with = "bool_from_int_default_false")]
    pub pass_token_enabled: bool,

    // -------------------------------------------------------------------------
    // Status Flags
    // -------------------------------------------------------------------------

    /// Whether this resource is marked as inactive.
    ///
    /// **OpenAPI type:** integer (ref: `Inactive`)
    ///
    /// Valid values:
    /// - `0` - Active
    /// - `1` - Inactive
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// **OpenAPI type:** integer (ref: `Frozen`)
    ///
    /// Valid values:
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    #[payrix(mutable)]
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // MerchantType Tests
    // =========================================================================

    #[test]
    fn merchant_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&MerchantType::SoleProprietor).unwrap(), "0");
        assert_eq!(serde_json::to_string(&MerchantType::Corporation).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MerchantType::LimitedLiabilityCorporation).unwrap(), "2");
        assert_eq!(serde_json::to_string(&MerchantType::Partnership).unwrap(), "3");
        assert_eq!(serde_json::to_string(&MerchantType::NonProfitOrganization).unwrap(), "5");
        assert_eq!(serde_json::to_string(&MerchantType::GovernmentOrganization).unwrap(), "6");
        assert_eq!(serde_json::to_string(&MerchantType::CCorporation).unwrap(), "7");
        assert_eq!(serde_json::to_string(&MerchantType::SCorporation).unwrap(), "8");
    }

    #[test]
    fn merchant_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<MerchantType>("0").unwrap(), MerchantType::SoleProprietor);
        assert_eq!(serde_json::from_str::<MerchantType>("1").unwrap(), MerchantType::Corporation);
        assert_eq!(serde_json::from_str::<MerchantType>("2").unwrap(), MerchantType::LimitedLiabilityCorporation);
        assert_eq!(serde_json::from_str::<MerchantType>("3").unwrap(), MerchantType::Partnership);
        assert_eq!(serde_json::from_str::<MerchantType>("5").unwrap(), MerchantType::NonProfitOrganization);
        assert_eq!(serde_json::from_str::<MerchantType>("6").unwrap(), MerchantType::GovernmentOrganization);
        assert_eq!(serde_json::from_str::<MerchantType>("7").unwrap(), MerchantType::CCorporation);
        assert_eq!(serde_json::from_str::<MerchantType>("8").unwrap(), MerchantType::SCorporation);
    }

    #[test]
    fn merchant_type_default() {
        assert_eq!(MerchantType::default(), MerchantType::SoleProprietor);
    }

    // =========================================================================
    // MerchantStatus Tests
    // =========================================================================

    #[test]
    fn merchant_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&MerchantStatus::NotReady).unwrap(), "0");
        assert_eq!(serde_json::to_string(&MerchantStatus::Ready).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MerchantStatus::Boarded).unwrap(), "2");
        assert_eq!(serde_json::to_string(&MerchantStatus::Manual).unwrap(), "3");
        assert_eq!(serde_json::to_string(&MerchantStatus::Closed).unwrap(), "4");
        assert_eq!(serde_json::to_string(&MerchantStatus::Incomplete).unwrap(), "5");
        assert_eq!(serde_json::to_string(&MerchantStatus::Pending).unwrap(), "6");
    }

    #[test]
    fn merchant_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<MerchantStatus>("0").unwrap(), MerchantStatus::NotReady);
        assert_eq!(serde_json::from_str::<MerchantStatus>("1").unwrap(), MerchantStatus::Ready);
        assert_eq!(serde_json::from_str::<MerchantStatus>("2").unwrap(), MerchantStatus::Boarded);
        assert_eq!(serde_json::from_str::<MerchantStatus>("3").unwrap(), MerchantStatus::Manual);
        assert_eq!(serde_json::from_str::<MerchantStatus>("4").unwrap(), MerchantStatus::Closed);
        assert_eq!(serde_json::from_str::<MerchantStatus>("5").unwrap(), MerchantStatus::Incomplete);
        assert_eq!(serde_json::from_str::<MerchantStatus>("6").unwrap(), MerchantStatus::Pending);
    }

    #[test]
    fn merchant_status_default() {
        assert_eq!(MerchantStatus::default(), MerchantStatus::NotReady);
    }

    // =========================================================================
    // MerchantEnvironment Tests
    // =========================================================================

    #[test]
    fn merchant_environment_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Supermarket).unwrap(), "\"supermarket\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::MailOrTelephoneOrder).unwrap(), "\"moto\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::CardPresent).unwrap(), "\"cardPresent\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Fuel).unwrap(), "\"fuel\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::ServiceStation).unwrap(), "\"serviceStation\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Restaurant).unwrap(), "\"restaurant\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Ecommerce).unwrap(), "\"ecommerce\"");
    }

    #[test]
    fn merchant_environment_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"supermarket\"").unwrap(), MerchantEnvironment::Supermarket);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"moto\"").unwrap(), MerchantEnvironment::MailOrTelephoneOrder);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"cardPresent\"").unwrap(), MerchantEnvironment::CardPresent);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"fuel\"").unwrap(), MerchantEnvironment::Fuel);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"serviceStation\"").unwrap(), MerchantEnvironment::ServiceStation);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"restaurant\"").unwrap(), MerchantEnvironment::Restaurant);
        // API returns "eCommerce" but also accepts "ecommerce"
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"eCommerce\"").unwrap(), MerchantEnvironment::Ecommerce);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"ecommerce\"").unwrap(), MerchantEnvironment::Ecommerce);
    }

    #[test]
    fn merchant_environment_default() {
        assert_eq!(MerchantEnvironment::default(), MerchantEnvironment::Ecommerce);
    }

    // =========================================================================
    // RiskLevel Tests
    // =========================================================================

    #[test]
    fn risk_level_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&RiskLevel::Restricted).unwrap(), "\"restricted\"");
        assert_eq!(serde_json::to_string(&RiskLevel::Prohibited).unwrap(), "\"prohibited\"");
        assert_eq!(serde_json::to_string(&RiskLevel::High).unwrap(), "\"high\"");
        assert_eq!(serde_json::to_string(&RiskLevel::Medium).unwrap(), "\"medium\"");
        assert_eq!(serde_json::to_string(&RiskLevel::Low).unwrap(), "\"low\"");
    }

    #[test]
    fn risk_level_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<RiskLevel>("\"restricted\"").unwrap(), RiskLevel::Restricted);
        assert_eq!(serde_json::from_str::<RiskLevel>("\"prohibited\"").unwrap(), RiskLevel::Prohibited);
        assert_eq!(serde_json::from_str::<RiskLevel>("\"high\"").unwrap(), RiskLevel::High);
        assert_eq!(serde_json::from_str::<RiskLevel>("\"medium\"").unwrap(), RiskLevel::Medium);
        assert_eq!(serde_json::from_str::<RiskLevel>("\"low\"").unwrap(), RiskLevel::Low);
    }

    #[test]
    fn risk_level_default() {
        assert_eq!(RiskLevel::default(), RiskLevel::Low);
    }

    // =========================================================================
    // TaxIdStatus Tests
    // =========================================================================

    #[test]
    fn tax_id_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TaxIdStatus::Pending).unwrap(), "0");
        assert_eq!(serde_json::to_string(&TaxIdStatus::Valid).unwrap(), "1");
        assert_eq!(serde_json::to_string(&TaxIdStatus::Invalid).unwrap(), "2");
        assert_eq!(serde_json::to_string(&TaxIdStatus::NotRequired).unwrap(), "3");
    }

    #[test]
    fn tax_id_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TaxIdStatus>("0").unwrap(), TaxIdStatus::Pending);
        assert_eq!(serde_json::from_str::<TaxIdStatus>("1").unwrap(), TaxIdStatus::Valid);
        assert_eq!(serde_json::from_str::<TaxIdStatus>("2").unwrap(), TaxIdStatus::Invalid);
        assert_eq!(serde_json::from_str::<TaxIdStatus>("3").unwrap(), TaxIdStatus::NotRequired);
    }

    #[test]
    fn tax_id_status_default() {
        assert_eq!(TaxIdStatus::default(), TaxIdStatus::Pending);
    }

    // =========================================================================
    // Merchant Struct Tests
    // =========================================================================

    #[test]
    fn merchant_deserialize_full() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-04-01 12:00:00.0000",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345",
            "entity": "t1_ent_12345678901234567890123",
            "login": "t1_lgn_12345678901234567890123",
            "dba": "Acme Widgets",
            "status": 2,
            "environment": "eCommerce",
            "riskLevel": "low",
            "new": 1,
            "established": "20150101",
            "annualCCSales": 50000000,
            "avgTicket": 2500,
            "mcc": "5734",
            "boarded": "20240101",
            "chargebackNotificationEmail": "chargeback@example.com",
            "inactive": 0,
            "frozen": 1
        }"#;

        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id.as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(merchant.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(merchant.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
        assert_eq!(merchant.entity.unwrap().as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(merchant.login.unwrap().as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(merchant.dba.as_deref(), Some("Acme Widgets"));
        assert_eq!(merchant.status, Some(MerchantStatus::Boarded));
        assert_eq!(merchant.environment, Some(MerchantEnvironment::Ecommerce));
        assert_eq!(merchant.risk_level, Some(RiskLevel::Low));
        assert!(merchant.new);
        assert_eq!(merchant.established.as_ref().unwrap().as_str(), "20150101");
        assert_eq!(merchant.annual_cc_sales, Some(50000000));
        assert_eq!(merchant.avg_ticket, Some(2500));
        assert_eq!(merchant.mcc.as_deref(), Some("5734"));
        assert_eq!(merchant.boarded.as_ref().unwrap().as_str(), "20240101");
        assert_eq!(merchant.chargeback_notification_email.as_deref(), Some("chargeback@example.com"));
        assert!(!merchant.inactive);
        assert!(merchant.frozen);
    }

    #[test]
    fn merchant_deserialize_minimal() {
        let json = r#"{"id": "t1_mer_12345678901234567890123"}"#;

        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id.as_str(), "t1_mer_12345678901234567890123");
        assert!(merchant.entity.is_none());
        assert!(merchant.status.is_none());
        assert!(!merchant.new);
        assert!(!merchant.inactive);
        assert!(!merchant.frozen);
    }

    #[test]
    fn merchant_new_fields() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "lastActivity": "2024-06-15 10:30:00",
            "totalApprovedSales": 1500000,
            "incrementalAuthSupported": 1,
            "seasonal": 0,
            "advancedBilling": 1,
            "autoBoarded": 1,
            "applePayActive": 1,
            "googlePayActive": 0
        }"#;

        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.last_activity.as_deref(), Some("2024-06-15 10:30:00"));
        assert_eq!(merchant.total_approved_sales, Some(1500000));
        assert!(merchant.incremental_auth_supported);
        assert!(!merchant.seasonal);
        assert!(merchant.advanced_billing);
        assert!(merchant.auto_boarded);
        assert!(merchant.apple_pay_active);
        assert!(!merchant.google_pay_active);
    }

    #[test]
    fn merchant_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_mer_12345678901234567890123", "new": 0, "inactive": 0, "frozen": 0}"#;
        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert!(!merchant.new);
        assert!(!merchant.inactive);
        assert!(!merchant.frozen);
    }

    #[test]
    fn merchant_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_mer_12345678901234567890123", "new": 1, "inactive": 1, "frozen": 1}"#;
        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert!(merchant.new);
        assert!(merchant.inactive);
        assert!(merchant.frozen);
    }
}
