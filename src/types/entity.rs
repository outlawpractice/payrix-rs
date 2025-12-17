//! Entity types for the Payrix API.
//!
//! Entities represent business organizations in the Payrix hierarchy.
//! Each entity can have multiple merchants, accounts, and members (beneficial owners).
//!
//! **OpenAPI schema:** `entitiesResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, MerchantType, PayrixId, TaxIdStatus};

// =============================================================================
// ENUMS
// =============================================================================

/// EIN/TIN type values per OpenAPI spec.
///
/// **OpenAPI schema:** `EinType`
///
/// Valid values:
/// - `ssn` - Social Security Number
/// - `tin` - Employer Identification Number
/// - `other` - Other/Unknown TIN
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EinType {
    /// Social Security Number.
    Ssn,
    /// Employer Identification Number.
    Tin,
    /// Other/Unknown TIN.
    Other,
}

/// Global business type values per OpenAPI spec.
///
/// **OpenAPI schema:** `GlobalBusinessType`
///
/// # Dual-Purpose Enum
///
/// This enum serves different purposes depending on the entity's country:
///
/// - **For USA entities**: Indicates the type of Tax Identification Number (TIN) used
/// - **For Canadian entities**: Indicates the province/territory where the business is registered
///
/// The appropriate variant to use depends on the `country` field of the parent [`Entity`].
///
/// # Valid Values for USA
///
/// When `Entity.country == "USA"`:
/// - [`Ssn`](GlobalBusinessType::Ssn) - Social Security Number
/// - [`Tin`](GlobalBusinessType::Tin) - Employer Identification Number
/// - [`Other`](GlobalBusinessType::Other) - Other/Unknown TIN type
///
/// # Valid Values for Canada
///
/// When `Entity.country == "CAN"`:
/// - [`FederalCanada`](GlobalBusinessType::FederalCanada) - Federally registered business
/// - Province/territory codes (`Alberta`, `BritishColumbia`, `Ontario`, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlobalBusinessType {
    /// Social Security Number (USA).
    #[serde(rename = "ssn")]
    Ssn,
    /// Employer Identification Number (USA).
    #[serde(rename = "tin")]
    Tin,
    /// Other/Unknown TIN (USA).
    #[serde(rename = "other")]
    Other,
    /// Federal (Canada).
    #[serde(rename = "CD")]
    FederalCanada,
    /// Alberta (Canada).
    #[serde(rename = "AB")]
    Alberta,
    /// British Columbia (Canada).
    #[serde(rename = "BC")]
    BritishColumbia,
    /// Manitoba (Canada).
    #[serde(rename = "MB")]
    Manitoba,
    /// New Brunswick (Canada).
    #[serde(rename = "NB")]
    NewBrunswick,
    /// Newfoundland and Labrador (Canada).
    #[serde(rename = "NL")]
    NewfoundlandLabrador,
    /// Northwest Territories (Canada).
    #[serde(rename = "NT")]
    NorthwestTerritories,
    /// Nova Scotia (Canada).
    #[serde(rename = "NS")]
    NovaScotia,
    /// Nunavut (Canada).
    #[serde(rename = "NU")]
    Nunavut,
    /// Ontario (Canada).
    #[serde(rename = "ON")]
    Ontario,
    /// Prince Edward Island (Canada).
    #[serde(rename = "PE")]
    PrinceEdwardIsland,
    /// Quebec (Canada).
    #[serde(rename = "QC")]
    Quebec,
    /// Saskatchewan (Canada).
    #[serde(rename = "SK")]
    Saskatchewan,
    /// Yukon (Canada).
    #[serde(rename = "YT")]
    Yukon,
}

/// Pending risk check status per OpenAPI spec.
///
/// **OpenAPI schema:** `PendingRiskCheck`
///
/// Valid values:
/// - `pending` - Pending
/// - `successful` - Successful
/// - `failed` - Failed
/// - `manual` - Manual
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PendingRiskCheck {
    /// Pending.
    #[default]
    Pending,
    /// Successful.
    Successful,
    /// Failed.
    Failed,
    /// Manual.
    Manual,
}

/// Entity check stage values per OpenAPI spec.
///
/// **OpenAPI schema:** `entityCheckStage`
///
/// Valid values:
/// - `createEntity` - Merchant created, no Signup Form submitted
/// - `underwriting` - Risk/Underwriting Review, Merchant Signup Form submitted
/// - `preboard` - Preboard, check the Merchant before they are boarded
/// - `postboard` - Check the Merchant after they are boarded
/// - `txn` - Check the Merchant when they process a Transaction
/// - `txnVolume` - Check the Merchant when their transaction volume hits a certain amount
/// - `payout` - Check the Merchant when a Payout occurs
/// - `payoutVolume` - Check the Merchant when the volume of Payouts hits a certain amount
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntityCheckStage {
    /// Merchant created, no Signup Form submitted.
    #[default]
    CreateEntity,
    /// Risk/Underwriting Review, Merchant Signup Form submitted.
    Underwriting,
    /// Preboard, check the Merchant before they are boarded.
    Preboard,
    /// Check the Merchant after they are boarded.
    Postboard,
    /// Check the Merchant when they process a Transaction.
    Txn,
    /// Check the Merchant when their transaction volume hits a certain amount.
    TxnVolume,
    /// Check the Merchant when a Payout occurs.
    Payout,
    /// Check the Merchant when the volume of Payouts hits a certain amount.
    PayoutVolume,
}

/// Entity reserved status values per OpenAPI spec.
///
/// **OpenAPI schema:** `entitiesReserved`
///
/// Valid values:
/// - `0` - No reserve
/// - `1` - Block transaction, will never be processed
/// - `2` - Reserved (not documented in OpenAPI, included for defensive deserialization)
/// - `3` - Hold transaction, will not be captured
/// - `4` - Reserve transaction, funds should be reserved
/// - `5` - Block current activity, no change for merchant
/// - `6` - Passed decision(s)
/// - `7` - We did not have policies to process
/// - `8` - We onboard the merchant and wait for manual check later
/// - `9` - Schedule the automatic release of the reserve
/// - `10` - Hold transaction, automatic release when associated sale is done
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EntityReserved {
    /// No reserve.
    #[default]
    NoReserve = 0,
    /// Block transaction, will never be processed.
    Block = 1,
    /// Reserved value (not documented in OpenAPI spec, included for defensive deserialization).
    Reserved = 2,
    /// Hold transaction, will not be captured.
    Hold = 3,
    /// Reserve transaction, funds should be reserved.
    Reserve = 4,
    /// Block current activity, no change for merchant.
    BlockActivity = 5,
    /// Passed decision(s).
    Passed = 6,
    /// We did not have policies to process.
    NoPolicies = 7,
    /// We onboard the merchant and wait for manual check later.
    PostReviewOnly = 8,
    /// Schedule the automatic release of the reserve.
    ScheduledRelease = 9,
    /// Hold transaction, automatic release when associated sale is done.
    HoldAutoRelease = 10,
}

/// Entity public status per OpenAPI spec.
///
/// **OpenAPI schema:** `entitiesPublic`
///
/// Valid values:
/// - `0` - Private entity
/// - `1` - Public entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EntityPublic {
    /// Private entity.
    #[default]
    Private = 0,
    /// Public entity.
    Public = 1,
}

// =============================================================================
// ENTITY STRUCT
// =============================================================================

/// A Payrix entity.
///
/// Entities represent business organizations in the Payrix hierarchy.
/// Each entity can have multiple merchants, accounts, and members (beneficial owners).
///
/// **OpenAPI schema:** `entitiesResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    // -------------------------------------------------------------------------
    // Core Identifiers
    // -------------------------------------------------------------------------

    /// The ID of this resource.
    ///
    /// **OpenAPI type:** string
    pub id: PayrixId,

    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{4}$`)
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    ///
    /// **OpenAPI type:** string (ref: creator)
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub modifier: Option<PayrixId>,

    /// The incoming IP address from which this Entity was created.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ip_created: Option<String>,

    /// The incoming IP address from which this Entity was last modified.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ip_modified: Option<String>,

    /// The client IP address from which the Entity was created.
    ///
    /// Valid values are any IPv4 or IPv6 address.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub client_ip: Option<String>,

    /// The ID of the Login that owns this resource.
    ///
    /// **OpenAPI type:** string (ref: entitiesModelLogin)
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// The parameter associated with this Entity.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub parameter: Option<String>,

    /// The sum of all negative disbursements, in cents, associated to this Entity.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub total_credit_disbursements: Option<i32>,

    // -------------------------------------------------------------------------
    // Business Information
    // -------------------------------------------------------------------------

    /// The type of Entity (business structure).
    ///
    /// **OpenAPI type:** integer (ref: entityType)
    #[serde(default, rename = "type")]
    pub entity_type: Option<MerchantType>,

    /// The name of this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub name: Option<String>,

    /// The display name of this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 1,000 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub display_name: Option<String>,

    // -------------------------------------------------------------------------
    // Address
    // -------------------------------------------------------------------------

    /// The first line of the address associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// The second line of the address associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The name of the city in the address associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The U.S. state or Canadian province relevant to the address provided.
    ///
    /// If the location is within the U.S. and Canada, specify the 2-character postal
    /// abbreviation for the state. If the location is outside of the U.S. and Canada,
    /// provide the full state name.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The ZIP code in the address associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The country in the address associated with the Entity.
    ///
    /// Currently accepts values including `USA` and `CAN`.
    ///
    /// **OpenAPI type:** string (ref: entityCountry)
    #[serde(default)]
    pub country: Option<String>,

    /// The time zone for the address associated with this Entity.
    ///
    /// Valid values: est, pst, cst, mst, akst, hst, sst, chst, ast, pwt, mht, chut, nst
    ///
    /// **OpenAPI type:** string (ref: Timezone)
    #[serde(default)]
    pub timezone: Option<String>,

    /// The website URL associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 0 and 500 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub website: Option<String>,

    // -------------------------------------------------------------------------
    // Contact Information
    // -------------------------------------------------------------------------

    /// The phone number associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 5 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The customer service phone number associated with this Entity.
    ///
    /// For Merchants, this number will be displayed on the customer's credit card statement.
    /// This field is stored as a text string and must be between 5 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub customer_phone: Option<String>,

    /// The fax number associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 5 and 15 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fax: Option<String>,

    /// The email address associated with this Entity.
    ///
    /// This field is stored as a text string and must be between 1 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    // -------------------------------------------------------------------------
    // Tax Information
    // -------------------------------------------------------------------------

    /// The IRS Employer Identification Number (EIN) for the Entity.
    ///
    /// This field is stored as an integer and must be 9 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ein: Option<String>,

    /// Indicates if the TIN being used is an EIN, SSN, or other/unknown number.
    ///
    /// **OpenAPI type:** string (ref: EinType)
    #[serde(default)]
    pub ein_type: Option<EinType>,

    /// The business registration number for the entity.
    ///
    /// This field is stored as an alphanumeric and must be between 9 to 10 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub global_business_id: Option<String>,

    /// The business registration type for the entity.
    ///
    /// **OpenAPI type:** string (ref: GlobalBusinessType)
    #[serde(default)]
    pub global_business_type: Option<GlobalBusinessType>,

    /// The IRS Legal Filing Name.
    ///
    /// This must match what has been provided to the IRS when filing taxes.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub irs_filing_name: Option<String>,

    /// The Tax Identification Number (TIN) status of the entity.
    ///
    /// **OpenAPI type:** integer (ref: TinStatus)
    #[serde(default)]
    pub tin_status: Option<TaxIdStatus>,

    // -------------------------------------------------------------------------
    // Business Details
    // -------------------------------------------------------------------------

    /// The number of locations at which this Entity does business.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub locations: Option<i32>,

    /// This field is stored as a text string and must be between 0 and 1,000 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub industry: Option<String>,

    /// The secondary billing descriptor to appear on bank statements for funds transfer.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub payout_secondary_descriptor: Option<String>,

    /// The currency of this Entity.
    ///
    /// See ISO 4217 currency codes for all valid values.
    ///
    /// **OpenAPI type:** string (ref: Currency)
    #[serde(default)]
    pub currency: Option<String>,

    // -------------------------------------------------------------------------
    // Terms and Conditions
    // -------------------------------------------------------------------------

    /// An indicator showing the version of the terms and conditions accepted.
    ///
    /// This field is stored as a text string and must be between 0 and 20 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub tc_version: Option<String>,

    /// Date the tcVersion was last updated.
    ///
    /// Format: YYYY-MM-DD HH:MM:SS
    ///
    /// **OpenAPI type:** string (pattern: `^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}`)
    #[serde(default)]
    pub tc_date: Option<String>,

    /// IP address of client from last tcVersion update.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub tc_ip: Option<String>,

    /// Date and time on which this Entity accepted the Terms and Conditions.
    ///
    /// The date is specified as a 12-digit string in YYYYMMDDHHII format,
    /// for example, `201601201528` for January 20, 2016, at 15:28 (3:28 PM).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub tc_accept_date: Option<String>,

    /// IP address from which this Entity accepted the Terms and Conditions.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub tc_accept_ip: Option<String>,

    // -------------------------------------------------------------------------
    // Risk & Status
    // -------------------------------------------------------------------------

    /// Indicates whether this is a publicly held entity.
    ///
    /// **OpenAPI type:** integer (ref: entitiesPublic)
    #[serde(default)]
    pub public: Option<EntityPublic>,

    /// Indicates the reserve status of the entity.
    ///
    /// **OpenAPI type:** integer (ref: entitiesReserved)
    #[serde(default)]
    pub reserved: Option<EntityReserved>,

    /// Whether it is pending a risk check.
    ///
    /// **OpenAPI type:** string (ref: PendingRiskCheck)
    #[serde(default)]
    pub pending_risk_check: Option<PendingRiskCheck>,

    /// The last stage completed for risk underwriting review.
    ///
    /// **OpenAPI type:** string (ref: entityCheckStage)
    #[serde(default)]
    pub check_stage: Option<EntityCheckStage>,

    // -------------------------------------------------------------------------
    // Custom Data
    // -------------------------------------------------------------------------

    /// Custom, free-form field for client-supplied text.
    ///
    /// Must be between 0 and 1,000 characters long.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub custom: Option<String>,

    // -------------------------------------------------------------------------
    // Status Flags
    // -------------------------------------------------------------------------

    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    ///
    /// **OpenAPI type:** integer (ref: Inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    ///
    /// **OpenAPI type:** integer (ref: Frozen)
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_deserialize_full() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-04-01 12:00:00.0000",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345",
            "ipCreated": "192.168.1.1",
            "ipModified": "192.168.1.2",
            "clientIp": "10.0.0.1",
            "login": "t1_lgn_12345678901234567890123",
            "parameter": "param123",
            "totalCreditDisbursements": -50000,
            "type": 1,
            "name": "Acme Corporation",
            "displayName": "Acme Corp",
            "address1": "123 Main St",
            "address2": "Suite 100",
            "city": "Springfield",
            "state": "IL",
            "zip": "62701",
            "country": "USA",
            "timezone": "cst",
            "website": "https://acme.com",
            "phone": "5551234567",
            "customerPhone": "5559876543",
            "fax": "5551112222",
            "email": "info@acme.com",
            "ein": "123456789",
            "einType": "tin",
            "globalBusinessId": "123456789",
            "globalBusinessType": "tin",
            "irsFilingName": "Acme Corporation Inc",
            "tinStatus": 1,
            "locations": 5,
            "industry": "Software",
            "payoutSecondaryDescriptor": "ACME PAYOUT",
            "currency": "USD",
            "tcVersion": "2.0",
            "tcDate": "2024-01-15 10:30:00",
            "tcIp": "192.168.1.100",
            "tcAcceptDate": "202401151030",
            "tcAcceptIp": "192.168.1.100",
            "public": 0,
            "reserved": 0,
            "pendingRiskCheck": "successful",
            "checkStage": "postboard",
            "custom": "custom data here",
            "inactive": 0,
            "frozen": 0
        }"#;

        let entity: Entity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id.as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entity.ip_created.as_deref(), Some("192.168.1.1"));
        assert_eq!(entity.client_ip.as_deref(), Some("10.0.0.1"));
        assert_eq!(entity.entity_type, Some(MerchantType::Corporation));
        assert_eq!(entity.name.as_deref(), Some("Acme Corporation"));
        assert_eq!(entity.display_name.as_deref(), Some("Acme Corp"));
        assert_eq!(entity.ein_type, Some(EinType::Tin));
        assert_eq!(entity.global_business_type, Some(GlobalBusinessType::Tin));
        assert_eq!(entity.tin_status, Some(TaxIdStatus::Valid));
        assert_eq!(entity.locations, Some(5));
        assert_eq!(entity.public, Some(EntityPublic::Private));
        assert_eq!(entity.reserved, Some(EntityReserved::NoReserve));
        assert_eq!(entity.pending_risk_check, Some(PendingRiskCheck::Successful));
        assert_eq!(entity.check_stage, Some(EntityCheckStage::Postboard));
        assert!(!entity.inactive);
        assert!(!entity.frozen);
    }

    #[test]
    fn entity_deserialize_minimal() {
        let json = r#"{"id": "t1_ent_12345678901234567890123"}"#;

        let entity: Entity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id.as_str(), "t1_ent_12345678901234567890123");
        assert!(entity.name.is_none());
        assert!(entity.entity_type.is_none());
        assert!(!entity.inactive);
        assert!(!entity.frozen);
    }

    #[test]
    fn entity_ein_type_values() {
        let test_cases = vec![
            ("ssn", EinType::Ssn),
            ("tin", EinType::Tin),
            ("other", EinType::Other),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ent_12345678901234567890123", "einType": "{}"}}"#,
                val
            );
            let entity: Entity = serde_json::from_str(&json).unwrap();
            assert_eq!(entity.ein_type, Some(expected));
        }
    }

    #[test]
    fn entity_pending_risk_check_values() {
        let test_cases = vec![
            ("pending", PendingRiskCheck::Pending),
            ("successful", PendingRiskCheck::Successful),
            ("failed", PendingRiskCheck::Failed),
            ("manual", PendingRiskCheck::Manual),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ent_12345678901234567890123", "pendingRiskCheck": "{}"}}"#,
                val
            );
            let entity: Entity = serde_json::from_str(&json).unwrap();
            assert_eq!(entity.pending_risk_check, Some(expected));
        }
    }

    #[test]
    fn entity_check_stage_values() {
        let test_cases = vec![
            ("createEntity", EntityCheckStage::CreateEntity),
            ("underwriting", EntityCheckStage::Underwriting),
            ("preboard", EntityCheckStage::Preboard),
            ("postboard", EntityCheckStage::Postboard),
            ("txn", EntityCheckStage::Txn),
            ("txnVolume", EntityCheckStage::TxnVolume),
            ("payout", EntityCheckStage::Payout),
            ("payoutVolume", EntityCheckStage::PayoutVolume),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ent_12345678901234567890123", "checkStage": "{}"}}"#,
                val
            );
            let entity: Entity = serde_json::from_str(&json).unwrap();
            assert_eq!(entity.check_stage, Some(expected));
        }
    }

    #[test]
    fn entity_reserved_values() {
        let test_cases = vec![
            (0, EntityReserved::NoReserve),
            (1, EntityReserved::Block),
            (3, EntityReserved::Hold),
            (4, EntityReserved::Reserve),
            (5, EntityReserved::BlockActivity),
            (6, EntityReserved::Passed),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ent_12345678901234567890123", "reserved": {}}}"#,
                val
            );
            let entity: Entity = serde_json::from_str(&json).unwrap();
            assert_eq!(entity.reserved, Some(expected));
        }
    }

    #[test]
    fn entity_global_business_type_canada() {
        let test_cases = vec![
            ("CD", GlobalBusinessType::FederalCanada),
            ("ON", GlobalBusinessType::Ontario),
            ("QC", GlobalBusinessType::Quebec),
            ("BC", GlobalBusinessType::BritishColumbia),
        ];

        for (val, expected) in test_cases {
            let json = format!(
                r#"{{"id": "t1_ent_12345678901234567890123", "globalBusinessType": "{}"}}"#,
                val
            );
            let entity: Entity = serde_json::from_str(&json).unwrap();
            assert_eq!(entity.global_business_type, Some(expected));
        }
    }

    #[test]
    fn entity_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "name": "Test Entity",
            "type": 2,
            "einType": "tin",
            "pendingRiskCheck": "successful"
        }"#;

        let entity: Entity = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&entity).unwrap();
        let deserialized: Entity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entity.id, deserialized.id);
        assert_eq!(entity.name, deserialized.name);
        assert_eq!(entity.entity_type, deserialized.entity_type);
        assert_eq!(entity.ein_type, deserialized.ein_type);
    }

    #[test]
    fn entity_bool_from_int() {
        let json = r#"{"id": "t1_ent_12345678901234567890123", "inactive": 1, "frozen": 0}"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        assert!(entity.inactive);
        assert!(!entity.frozen);
    }
}
