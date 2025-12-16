//! Member types for the Payrix API.
//!
//! Members represent beneficial owners and key individuals associated with merchants.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, DateYmd, PayrixId};

/// Member type values.
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

/// A Payrix member (beneficial owner or control person).
///
/// Members represent the beneficial owners and key individuals associated with
/// a merchant entity. They are required for KYC (Know Your Customer) compliance
/// and underwriting verification.
///
/// # Creating a Member
///
/// When creating a new member, the following fields are required:
/// - `entity` - Parent entity ID
/// - `first`, `last` - Legal name (must match government ID)
/// - `dob` - Date of birth (YYYYMMDD format)
/// - `ssn` - Social Security Number (9 digits, no dashes)
/// - `address1`, `city`, `state`, `zip`, `country` - Home address
/// - `ownership` - Ownership percentage (for Owner type)
///
/// Read-only fields (returned by API, not sent on create):
/// - `id` - Assigned by Payrix
/// - `merchant` - Set by API
/// - `login` - Set by API
/// - `created`, `modified` - Timestamps set by API
///
/// # Beneficial Ownership Requirements
///
/// For compliance with FinCEN regulations, you must report:
/// 1. **All individuals who own 25% or more** of the business
/// 2. **At least one control person** (someone with significant management responsibility)
///
/// The total ownership percentages for all `Owner` type members should sum to 100%.
///
/// # Member Types
///
/// - `Owner` - Beneficial owner with equity stake (≥25% ownership requires reporting)
/// - `ControlPerson` - Individual with significant management control (CEO, CFO, etc.)
/// - `Principal` - Key principal of the business
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Member {
    /// Unique identifier (30 characters, e.g., "t1_mem_...").
    ///
    /// **Read-only**: Assigned by Payrix when member is created.
    pub id: PayrixId,

    /// Parent entity ID.
    ///
    /// **Required for creation**. The entity this member is associated with.
    /// Format: "t1_ent_..." (30 characters).
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant ID.
    ///
    /// **Read-only**: Set by Payrix based on entity relationship.
    #[serde(default)]
    pub merchant: Option<PayrixId>,

    /// Login ID that created this member.
    ///
    /// **Read-only**: Set by Payrix based on the authenticated user.
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Member type indicating their role.
    ///
    /// **Optional** (defaults to `Owner`).
    /// - `Owner` - Beneficial owner (required if ≥25% ownership)
    /// - `ControlPerson` - Has significant management control
    /// - `Principal` - Key principal of the business
    #[serde(default, rename = "type")]
    pub member_type: Option<MemberType>,

    /// Legal first name.
    ///
    /// **Required for creation**. Must match government-issued ID exactly.
    #[serde(default)]
    pub first: Option<String>,

    /// Middle name or initial.
    ///
    /// **Optional**.
    #[serde(default)]
    pub middle: Option<String>,

    /// Legal last name.
    ///
    /// **Required for creation**. Must match government-issued ID exactly.
    #[serde(default)]
    pub last: Option<String>,

    /// Title or position within the company.
    ///
    /// **Optional**. Examples: "CEO", "Managing Member", "Partner", "Owner"
    #[serde(default)]
    pub title: Option<String>,

    /// Ownership percentage (0-100).
    ///
    /// **Required for Owner type**. The percentage of the business this person owns.
    /// All owner percentages should sum to 100%.
    #[serde(default)]
    pub ownership: Option<i32>,

    /// Date of birth in YYYYMMDD format.
    ///
    /// **Required for creation**. Example: "19800115" for January 15, 1980.
    /// Required for KYC verification.
    #[serde(default)]
    pub dob: Option<DateYmd>,

    /// Social Security Number.
    ///
    /// **Required for creation**. 9 digits without dashes
    /// (e.g., "123456789" not "123-45-6789").
    /// Note: API returns only last 4 digits in responses for security.
    #[serde(default)]
    pub ssn: Option<String>,

    /// Personal email address.
    ///
    /// **Optional but recommended**. Used for identity verification.
    #[serde(default)]
    pub email: Option<String>,

    /// Personal phone number.
    ///
    /// **Optional but recommended**. Digits only, no formatting.
    #[serde(default)]
    pub phone: Option<String>,

    /// Home address line 1.
    ///
    /// **Required for creation**. Personal residential address (not business).
    #[serde(default)]
    pub address1: Option<String>,

    /// Home address line 2.
    ///
    /// **Optional**. Apartment, unit, suite number, etc.
    #[serde(default)]
    pub address2: Option<String>,

    /// City of residence.
    ///
    /// **Required for creation**.
    #[serde(default)]
    pub city: Option<String>,

    /// State or province code.
    ///
    /// **Required for creation**. Use 2-letter codes for US (e.g., "IL", "CA").
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP or postal code.
    ///
    /// **Required for creation**.
    #[serde(default)]
    pub zip: Option<String>,

    /// Country code.
    ///
    /// **Required for creation**. Use "USA" for United States.
    #[serde(default)]
    pub country: Option<String>,

    /// Custom data field for your application's use.
    ///
    /// **Optional**. Can store up to 1000 characters of arbitrary data.
    #[serde(default)]
    pub custom: Option<String>,

    /// Created timestamp in "YYYY-MM-DD HH:mm:ss.sss" format.
    ///
    /// **Read-only**: Set by Payrix when member is created.
    #[serde(default)]
    pub created: Option<String>,

    /// Last modified timestamp in "YYYY-MM-DD HH:mm:ss.sss" format.
    ///
    /// **Read-only**: Updated by Payrix on changes.
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive.
    ///
    /// **Optional on create**. Set to `true` to create in inactive state.
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen.
    ///
    /// **Read-only**: Set by Payrix for compliance/risk reasons.
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // MemberType enum tests
    #[test]
    fn test_member_type_default() {
        assert_eq!(MemberType::default(), MemberType::Owner);
    }

    #[test]
    fn test_member_type_serialize() {
        assert_eq!(serde_json::to_string(&MemberType::Owner).unwrap(), "1");
        assert_eq!(serde_json::to_string(&MemberType::ControlPerson).unwrap(), "2");
        assert_eq!(serde_json::to_string(&MemberType::Principal).unwrap(), "3");
    }

    #[test]
    fn test_member_type_deserialize() {
        assert_eq!(serde_json::from_str::<MemberType>("1").unwrap(), MemberType::Owner);
        assert_eq!(serde_json::from_str::<MemberType>("2").unwrap(), MemberType::ControlPerson);
        assert_eq!(serde_json::from_str::<MemberType>("3").unwrap(), MemberType::Principal);
    }

    // Member struct tests
    #[test]
    fn test_member_deserialize_full() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123",
            "entity": "t1_ent_23456789012345678901234",
            "merchant": "t1_mer_34567890123456789012345",
            "login": "t1_tlg_45678901234567890123456",
            "type": 1,
            "first": "John",
            "middle": "A",
            "last": "Doe",
            "title": "CEO",
            "ownership": 50,
            "dob": "20000115",
            "ssn": "1234",
            "email": "john@example.com",
            "phone": "555-1234",
            "address1": "123 Main St",
            "address2": "Apt 4B",
            "city": "New York",
            "state": "NY",
            "zip": "10001",
            "country": "US",
            "custom": "{\"key\":\"value\"}",
            "created": "2024-01-01 12:00:00.000",
            "modified": "2024-01-02 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert_eq!(member.entity, Some(PayrixId::new("t1_ent_23456789012345678901234").unwrap()));
        assert_eq!(member.merchant, Some(PayrixId::new("t1_mer_34567890123456789012345").unwrap()));
        assert_eq!(member.login, Some(PayrixId::new("t1_tlg_45678901234567890123456").unwrap()));
        assert_eq!(member.member_type, Some(MemberType::Owner));
        assert_eq!(member.first, Some("John".to_string()));
        assert_eq!(member.middle, Some("A".to_string()));
        assert_eq!(member.last, Some("Doe".to_string()));
        assert_eq!(member.title, Some("CEO".to_string()));
        assert_eq!(member.ownership, Some(50));
        assert_eq!(member.dob, Some(DateYmd::new("20000115").unwrap()));
        assert_eq!(member.ssn, Some("1234".to_string()));
        assert_eq!(member.email, Some("john@example.com".to_string()));
        assert_eq!(member.phone, Some("555-1234".to_string()));
        assert_eq!(member.address1, Some("123 Main St".to_string()));
        assert_eq!(member.address2, Some("Apt 4B".to_string()));
        assert_eq!(member.city, Some("New York".to_string()));
        assert_eq!(member.state, Some("NY".to_string()));
        assert_eq!(member.zip, Some("10001".to_string()));
        assert_eq!(member.country, Some("US".to_string()));
        assert_eq!(member.custom, Some("{\"key\":\"value\"}".to_string()));
        assert_eq!(member.created, Some("2024-01-01 12:00:00.000".to_string()));
        assert_eq!(member.modified, Some("2024-01-02 12:00:00.000".to_string()));
        assert_eq!(member.inactive, false);
        assert_eq!(member.frozen, true);
    }

    #[test]
    fn test_member_deserialize_minimal() {
        let json = r#"{
            "id": "t1_mem_12345678901234567890123"
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id.as_str(), "t1_mem_12345678901234567890123");
        assert_eq!(member.entity, None);
        assert_eq!(member.merchant, None);
        assert_eq!(member.login, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.first, None);
        assert_eq!(member.middle, None);
        assert_eq!(member.last, None);
        assert_eq!(member.title, None);
        assert_eq!(member.ownership, None);
        assert_eq!(member.dob, None);
        assert_eq!(member.ssn, None);
        assert_eq!(member.email, None);
        assert_eq!(member.phone, None);
        assert_eq!(member.address1, None);
        assert_eq!(member.address2, None);
        assert_eq!(member.city, None);
        assert_eq!(member.state, None);
        assert_eq!(member.zip, None);
        assert_eq!(member.country, None);
        assert_eq!(member.custom, None);
        assert_eq!(member.created, None);
        assert_eq!(member.modified, None);
        assert_eq!(member.inactive, false);
        assert_eq!(member.frozen, false);
    }

    #[test]
    fn test_member_bool_from_int() {
        // Test inactive field with int values
        let json = r#"{"id": "t1_mem_12345678901234567890123", "inactive": 1}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.inactive, true);

        let json = r#"{"id": "t1_mem_12345678901234567890123", "inactive": 0}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.inactive, false);

        // Test frozen field with int values
        let json = r#"{"id": "t1_mem_12345678901234567890123", "frozen": 1}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.frozen, true);

        let json = r#"{"id": "t1_mem_12345678901234567890123", "frozen": 0}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.frozen, false);
    }
}
