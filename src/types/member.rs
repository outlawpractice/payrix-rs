//! Member types for the Payrix API.
//!
//! Members represent beneficial owners and key individuals associated with merchants.
//!
//! **OpenAPI schema:** `membersResponse`

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, PayrixId};

// =============================================================================
// MEMBER ENUMS
// =============================================================================

/// Member type values.
///
/// Used for member creation requests. The `type` field is not part of the
/// OpenAPI response schema but is used when creating members via the API.
///
/// Valid values:
/// - `1` - Owner (beneficial owner)
/// - `2` - ControlPerson (authorized signer)
/// - `3` - Principal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MemberType {
    /// Beneficial owner
    #[default]
    Owner = 1,
    /// Control person / authorized signer
    ControlPerson = 2,
    /// Principal
    Principal = 3,
}

/// Gender values for a Member.
///
/// **OpenAPI schema:** `Gender`
///
/// Valid values:
/// - `male` - Male
/// - `female` - Female
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    /// Male
    #[default]
    Male,

    /// Female
    Female,
}

// =============================================================================
// MEMBER STRUCT
// =============================================================================

/// A Payrix member (beneficial owner or control person).
///
/// Members represent the beneficial owners and key individuals associated with
/// a merchant entity. They are required for KYC (Know Your Customer) compliance
/// and underwriting verification.
///
/// **OpenAPI schema:** `membersResponse`
///
/// See API_INCONSISTENCIES.md for known deviations from this spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Member {
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

    /// The identifier of the Merchant associated with this Member.
    ///
    /// **OpenAPI type:** string (ref: membersModelMerchant)
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// The title that this Member holds in relation to the associated Merchant.
    ///
    /// This field is stored as a text string (0-100 characters).
    /// For example, 'CEO', 'Owner' or 'Director of Finance'.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub title: Option<String>,

    /// The first name associated with this Member.
    ///
    /// This field is stored as a text string (1-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub first: Option<String>,

    /// The middle name associated with this Member.
    ///
    /// This field is stored as a text string (0-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub middle: Option<String>,

    /// The last name associated with this Member.
    ///
    /// This field is stored as a text string (1-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub last: Option<String>,

    /// The social security number of this Member.
    ///
    /// This field is required if the Merchant is a sole trader.
    /// Stored as a text string (9 characters, numeric only).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub ssn: Option<String>,

    /// The country of which the member is a citizen.
    ///
    /// Valid values: 3-letter ISO country code.
    ///
    /// **OpenAPI type:** string (ref: Citizenship)
    #[serde(default)]
    pub citizenship: Option<String>,

    /// The date of birth of this Member.
    ///
    /// Format: YYYYMMDD (e.g., `20160120` for January 20, 2016).
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub dob: Option<i32>,

    /// The gender of this Member.
    ///
    /// - `male` - Male
    /// - `female` - Female
    ///
    /// **OpenAPI type:** string (ref: Gender)
    #[serde(default)]
    pub gender: Option<Gender>,

    /// The driver's license number of this Member.
    ///
    /// This field is stored as a text string (0-15 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub dl: Option<String>,

    /// The U.S. state or Canadian province for the driver's license.
    ///
    /// Use 2-character postal abbreviation for US/Canada.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub dlstate: Option<String>,

    /// The share of the Member's ownership of the associated Merchant.
    ///
    /// Expressed in basis points. For example, 25.3% is expressed as '2530'.
    /// This field is stored as an integer (1-10000).
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub ownership: Option<i32>,

    /// The email address of this Member.
    ///
    /// This field is stored as a text string (1-100 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub email: Option<String>,

    /// The fax number associated with this Member.
    ///
    /// This field is stored as a text string (5-15 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub fax: Option<String>,

    /// The phone number associated with this Member.
    ///
    /// This field is stored as a text string (5-15 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub phone: Option<String>,

    /// The country code for this Member.
    ///
    /// Valid values: 3-letter ISO country code.
    ///
    /// **OpenAPI type:** string (ref: Country)
    #[serde(default)]
    pub country: Option<String>,

    /// The timezone for the address associated with the Member's location.
    ///
    /// Valid values: est, cst, pst, mst, akst, hst, sst, chst, ast, pwt, mht, chut, nst
    ///
    /// **OpenAPI type:** string (ref: Timezone)
    #[serde(default)]
    pub timezone: Option<String>,

    /// The ZIP code in the address associated with this Member.
    ///
    /// This field is stored as a text string (1-20 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub zip: Option<String>,

    /// The U.S. state or Canadian province for the address.
    ///
    /// Use 2-character postal abbreviation for US/Canada.
    /// For locations outside US/Canada, provide the full state name.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub state: Option<String>,

    /// The name of the city in the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub city: Option<String>,

    /// The second line of the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address2: Option<String>,

    /// The first line of the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub address1: Option<String>,

    /// Whether the Member is the 'primary' contact for the associated Merchant.
    ///
    /// Only one Member per Merchant can be the 'primary' Member.
    /// - `0` - Not Primary contact
    /// - `1` - Primary contact
    ///
    /// **OpenAPI type:** integer (ref: membersPrimary)
    #[serde(default, with = "bool_from_int_default_false")]
    pub primary: bool,

    /// A credit score (three-digit number, 334-818).
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub credit_score: Option<i32>,

    /// Date for Credit Score.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS`
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub credit_score_date: Option<String>,

    /// Whether the member has significant management responsibility.
    ///
    /// A 'controlling authority' or 'Control Prong' includes:
    /// CEO, CFO, COO, Managing Member, General Partner, President, Vice Presidents,
    /// or anyone with significant legal authority to enter the Legal Entity into
    /// a commercial relationship.
    ///
    /// - `0` - No significant responsibility
    /// - `1` - Significant responsibility
    ///
    /// **OpenAPI type:** integer (ref: SignificantResponsibility)
    #[serde(default, with = "bool_from_int_default_false")]
    pub significant_responsibility: bool,

    /// Whether this person is politically exposed.
    ///
    /// Defined as: "persons whom through their prominent position or influence,
    /// is more susceptible to being involved in bribery or corruption."
    ///
    /// - `0` - Not politically exposed
    /// - `1` - Politically exposed
    ///
    /// **OpenAPI type:** integer (ref: PoliticallyExposed)
    #[serde(default, with = "bool_from_int_default_false")]
    pub politically_exposed: bool,

    /// The mailing first line of the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mailing_address1: Option<String>,

    /// Treasury Prime roles.
    ///
    /// Example: `["signer", "control_person"]`
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub treasury_prime_roles: Option<String>,

    /// The mailing second line of the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mailing_address2: Option<String>,

    /// The mailing city in the address associated with this Member.
    ///
    /// This field is stored as a text string (1-500 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mailing_city: Option<String>,

    /// The mailing state for the address.
    ///
    /// Use 2-character postal abbreviation for US/Canada.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mailing_state: Option<String>,

    /// The mailing postal code in the address associated with this Member.
    ///
    /// This field is stored as a text string (1-20 characters).
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub mailing_postal_code: Option<String>,

    /// The mailing country for the address.
    ///
    /// Valid values: 3-letter ISO country code.
    ///
    /// **OpenAPI type:** string (ref: MailingCountry)
    #[serde(default)]
    pub mailing_country: Option<String>,

    /// The ID of the related facilitator record.
    ///
    /// **OpenAPI type:** string (ref: membersModelFacilitator)
    #[serde(default)]
    pub facilitator: Option<PayrixId>,

    /// The ID of the related vendor record.
    ///
    /// **OpenAPI type:** string (ref: membersModelVendor)
    #[serde(default)]
    pub vendor: Option<PayrixId>,

    /// A URL used in Identity Verification for the member.
    ///
    /// Obtained from identity verification through the integration partner.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub shareable_url: Option<String>,

    /// The status of the verification process for the member.
    ///
    /// Part of the verification result from the third party.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub status: Option<String>,

    /// The timestamp when the record was created.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub created_at: Option<i32>,

    /// The timestamp when verification was completed.
    ///
    /// **OpenAPI type:** integer (int32)
    #[serde(default)]
    pub completed_at: Option<i32>,

    /// The integration ID for the verification response.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub plaid_idv_id: Option<String>,

    /// The ID of the template used in the verification process.
    ///
    /// **OpenAPI type:** string
    #[serde(default)]
    pub template_id: Option<String>,

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

    // ==================== MemberType Tests ====================

    #[test]
    fn member_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&MemberType::Owner).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MemberType::ControlPerson).unwrap(), "2");
        assert_eq!(serde_json::to_string(&MemberType::Principal).unwrap(), "3");
    }

    #[test]
    fn member_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<MemberType>("1").unwrap(), MemberType::Owner);
        assert_eq!(serde_json::from_str::<MemberType>("2").unwrap(), MemberType::ControlPerson);
        assert_eq!(serde_json::from_str::<MemberType>("3").unwrap(), MemberType::Principal);
    }

    #[test]
    fn member_type_default() {
        assert_eq!(MemberType::default(), MemberType::Owner);
    }

    // ==================== Gender Tests ====================

    #[test]
    fn gender_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&Gender::Male).unwrap(), "\"male\"");
        assert_eq!(serde_json::to_string(&Gender::Female).unwrap(), "\"female\"");
    }

    #[test]
    fn gender_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<Gender>("\"male\"").unwrap(), Gender::Male);
        assert_eq!(serde_json::from_str::<Gender>("\"female\"").unwrap(), Gender::Female);
    }

    #[test]
    fn gender_default() {
        assert_eq!(Gender::default(), Gender::Male);
    }

    // ==================== Member Struct Tests ====================

    #[test]
    fn member_deserialize_full() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 23:59:59.9999",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124",
            "merchant": "t1_mer_12345678901234567890123",
            "title": "CEO",
            "first": "John",
            "middle": "Q",
            "last": "Smith",
            "ssn": "123456789",
            "citizenship": "USA",
            "dob": 19800115,
            "gender": "male",
            "dl": "D1234567",
            "dlstate": "IL",
            "ownership": 5000,
            "email": "john@example.com",
            "fax": "5551234567",
            "phone": "5559876543",
            "country": "USA",
            "timezone": "cst",
            "zip": "60601",
            "state": "IL",
            "city": "Chicago",
            "address2": "Suite 100",
            "address1": "123 Main St",
            "primary": 1,
            "creditScore": 750,
            "creditScoreDate": "2024-01-01 00:00:00",
            "significantResponsibility": 1,
            "politicallyExposed": 0,
            "mailingAddress1": "456 Mail St",
            "mailingAddress2": "PO Box 789",
            "mailingCity": "Springfield",
            "mailingState": "IL",
            "mailingPostalCode": "62701",
            "mailingCountry": "USA",
            "status": "approved",
            "inactive": 0,
            "frozen": 1
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert_eq!(member.created, Some("2024-01-01 00:00:00.0000".to_string()));
        assert_eq!(member.modified, Some("2024-01-02 23:59:59.9999".to_string()));
        assert_eq!(member.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(member.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
        assert_eq!(member.merchant.as_ref().map(|m| m.as_str()), Some("t1_mer_12345678901234567890123"));
        assert_eq!(member.title, Some("CEO".to_string()));
        assert_eq!(member.first, Some("John".to_string()));
        assert_eq!(member.middle, Some("Q".to_string()));
        assert_eq!(member.last, Some("Smith".to_string()));
        assert_eq!(member.ssn, Some("123456789".to_string()));
        assert_eq!(member.citizenship, Some("USA".to_string()));
        assert_eq!(member.dob, Some(19800115));
        assert_eq!(member.gender, Some(Gender::Male));
        assert_eq!(member.dl, Some("D1234567".to_string()));
        assert_eq!(member.dlstate, Some("IL".to_string()));
        assert_eq!(member.ownership, Some(5000));
        assert_eq!(member.email, Some("john@example.com".to_string()));
        assert_eq!(member.fax, Some("5551234567".to_string()));
        assert_eq!(member.phone, Some("5559876543".to_string()));
        assert_eq!(member.country, Some("USA".to_string()));
        assert_eq!(member.timezone, Some("cst".to_string()));
        assert_eq!(member.zip, Some("60601".to_string()));
        assert_eq!(member.state, Some("IL".to_string()));
        assert_eq!(member.city, Some("Chicago".to_string()));
        assert_eq!(member.address2, Some("Suite 100".to_string()));
        assert_eq!(member.address1, Some("123 Main St".to_string()));
        assert!(member.primary);
        assert_eq!(member.credit_score, Some(750));
        assert_eq!(member.credit_score_date, Some("2024-01-01 00:00:00".to_string()));
        assert!(member.significant_responsibility);
        assert!(!member.politically_exposed);
        assert_eq!(member.mailing_address1, Some("456 Mail St".to_string()));
        assert_eq!(member.mailing_address2, Some("PO Box 789".to_string()));
        assert_eq!(member.mailing_city, Some("Springfield".to_string()));
        assert_eq!(member.mailing_state, Some("IL".to_string()));
        assert_eq!(member.mailing_postal_code, Some("62701".to_string()));
        assert_eq!(member.mailing_country, Some("USA".to_string()));
        assert_eq!(member.status, Some("approved".to_string()));
        assert!(!member.inactive);
        assert!(member.frozen);
    }

    #[test]
    fn member_deserialize_minimal() {
        let json = r#"{"id": "t1_mem_12345678901234567890123"}"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert!(member.created.is_none());
        assert!(member.modified.is_none());
        assert!(member.creator.is_none());
        assert!(member.modifier.is_none());
        assert!(member.merchant.is_none());
        assert!(member.title.is_none());
        assert!(member.first.is_none());
        assert!(member.middle.is_none());
        assert!(member.last.is_none());
        assert!(member.ssn.is_none());
        assert!(member.citizenship.is_none());
        assert!(member.dob.is_none());
        assert!(member.gender.is_none());
        assert!(member.dl.is_none());
        assert!(member.dlstate.is_none());
        assert!(member.ownership.is_none());
        assert!(member.email.is_none());
        assert!(member.fax.is_none());
        assert!(member.phone.is_none());
        assert!(member.country.is_none());
        assert!(member.timezone.is_none());
        assert!(member.zip.is_none());
        assert!(member.state.is_none());
        assert!(member.city.is_none());
        assert!(member.address2.is_none());
        assert!(member.address1.is_none());
        assert!(!member.primary);
        assert!(member.credit_score.is_none());
        assert!(member.credit_score_date.is_none());
        assert!(!member.significant_responsibility);
        assert!(!member.politically_exposed);
        assert!(member.mailing_address1.is_none());
        assert!(member.mailing_address2.is_none());
        assert!(member.mailing_city.is_none());
        assert!(member.mailing_state.is_none());
        assert!(member.mailing_postal_code.is_none());
        assert!(member.mailing_country.is_none());
        assert!(member.facilitator.is_none());
        assert!(member.vendor.is_none());
        assert!(member.shareable_url.is_none());
        assert!(member.status.is_none());
        assert!(member.created_at.is_none());
        assert!(member.completed_at.is_none());
        assert!(member.plaid_idv_id.is_none());
        assert!(member.template_id.is_none());
        assert!(!member.inactive);
        assert!(!member.frozen);
    }

    #[test]
    fn member_bool_from_int() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "primary": 1,
            "significantResponsibility": 1,
            "politicallyExposed": 1,
            "inactive": 1,
            "frozen": 1
        }"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert!(member.primary);
        assert!(member.significant_responsibility);
        assert!(member.politically_exposed);
        assert!(member.inactive);
        assert!(member.frozen);

        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "primary": 0,
            "significantResponsibility": 0,
            "politicallyExposed": 0,
            "inactive": 0,
            "frozen": 0
        }"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert!(!member.primary);
        assert!(!member.significant_responsibility);
        assert!(!member.politically_exposed);
        assert!(!member.inactive);
        assert!(!member.frozen);
    }

    #[test]
    fn member_ownership_basis_points() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "ownership": 2530
        }"#;
        let member: Member = serde_json::from_str(json).unwrap();
        // 2530 basis points = 25.3% ownership
        assert_eq!(member.ownership, Some(2530));
    }

    #[test]
    fn member_serialize_roundtrip() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "first": "Jane",
            "last": "Doe",
            "ownership": 5000
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&member).unwrap();
        let deserialized: Member = serde_json::from_str(&serialized).unwrap();
        assert_eq!(member.id, deserialized.id);
        assert_eq!(member.merchant, deserialized.merchant);
        assert_eq!(member.first, deserialized.first);
        assert_eq!(member.last, deserialized.last);
        assert_eq!(member.ownership, deserialized.ownership);
    }
}
