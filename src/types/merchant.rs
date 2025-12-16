//! Merchant types for the Payrix API.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{bool_from_int_default_false, DateYmd, PayrixId};

/// Merchant type (business structure).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MerchantType {
    /// Sole proprietor
    #[default]
    SoleProprietor = 0,
    /// Corporation
    Corporation = 1,
    /// Limited Liability Company (LLC)
    LimitedLiabilityCorporation = 2,
    /// Partnership
    Partnership = 3,
    /// Non-profit organization
    NonProfitOrganization = 5,
    /// Government organization
    GovernmentOrganization = 6,
    /// C-Corporation
    CCorporation = 7,
    /// S-Corporation
    SCorporation = 8,
}

/// Merchant status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MerchantStatus {
    /// Not ready for processing
    #[default]
    NotReady = 0,
    /// Ready for processing
    Ready = 1,
    /// Boarded and active
    Boarded = 2,
    /// Manual review required
    Manual = 3,
    /// Account closed
    Closed = 4,
    /// Application incomplete
    Incomplete = 5,
    /// Pending review
    Pending = 6,
}

/// Merchant environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MerchantEnvironment {
    /// Supermarket
    Supermarket,
    /// Mail or telephone order
    #[serde(rename = "moto")]
    MailOrTelephoneOrder,
    /// Card present retail
    CardPresent,
    /// Fuel station
    Fuel,
    /// Service station
    ServiceStation,
    /// Restaurant
    Restaurant,
    /// Ecommerce
    #[default]
    #[serde(rename = "eCommerce")]
    Ecommerce,
}

/// Risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    /// Restricted
    Restricted,
    /// Prohibited
    Prohibited,
    /// High risk
    High,
    /// Medium risk
    Medium,
    /// Low risk
    #[default]
    Low,
}

/// Tax ID (TIN) status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TaxIdStatus {
    /// Pending verification
    #[default]
    Pending = 0,
    /// Valid
    Valid = 1,
    /// Invalid
    Invalid = 2,
    /// Not required
    NotRequired = 3,
}

/// A Payrix merchant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    /// Unique identifier (30 characters, e.g., "t1_mer_...")
    pub id: PayrixId,

    /// Parent entity ID
    #[serde(default)]
    pub entity: Option<PayrixId>,

    /// Merchant's login/username
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Merchant's DBA (doing business as) name
    #[serde(default)]
    pub dba: Option<String>,

    /// Merchant status
    #[serde(default)]
    pub status: Option<MerchantStatus>,

    /// Merchant environment
    #[serde(default)]
    pub environment: Option<MerchantEnvironment>,

    /// Risk level
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,

    /// Whether merchant is new
    #[serde(default, with = "bool_from_int_default_false")]
    pub new: bool,

    /// Date established (YYYYMMDD format)
    #[serde(default)]
    pub established: Option<DateYmd>,

    /// Annual credit card sales in cents
    #[serde(default)]
    pub annual_cc_sales: Option<i64>,

    /// Average ticket amount in cents
    #[serde(default)]
    pub avg_ticket: Option<i64>,

    /// Merchant Category Code (e.g., "8111" for legal services)
    #[serde(default)]
    pub mcc: Option<String>,

    /// Date boarded (YYYYMMDD format)
    #[serde(default)]
    pub boarded: Option<DateYmd>,

    /// Chargeback notification email
    #[serde(default)]
    pub chargeback_notification_email: Option<String>,

    /// Created timestamp
    #[serde(default)]
    pub created: Option<String>,

    /// Last modified timestamp
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

/// A Payrix entity (business entity above merchants).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// Unique identifier (30 characters, e.g., "t1_ent_...")
    pub id: PayrixId,

    /// Login ID that owns this entity
    #[serde(default)]
    pub login: Option<PayrixId>,

    /// Entity name (1-100 characters)
    #[serde(default)]
    pub name: Option<String>,

    /// Entity type (business structure)
    #[serde(default, rename = "type")]
    pub entity_type: Option<MerchantType>,

    /// Address line 1
    #[serde(default)]
    pub address1: Option<String>,

    /// Address line 2
    #[serde(default)]
    pub address2: Option<String>,

    /// City
    #[serde(default)]
    pub city: Option<String>,

    /// State/province code
    #[serde(default)]
    pub state: Option<String>,

    /// ZIP/postal code
    #[serde(default)]
    pub zip: Option<String>,

    /// Country code
    #[serde(default)]
    pub country: Option<String>,

    /// Phone number
    #[serde(default)]
    pub phone: Option<String>,

    /// Fax number
    #[serde(default)]
    pub fax: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Website URL
    #[serde(default)]
    pub website: Option<String>,

    /// EIN (Tax ID)
    #[serde(default)]
    pub ein: Option<String>,

    /// Tax ID status
    #[serde(default)]
    pub tin_status: Option<TaxIdStatus>,

    /// Custom field (0-1000 characters)
    #[serde(default)]
    pub custom: Option<String>,

    /// Created timestamp
    #[serde(default)]
    pub created: Option<String>,

    /// Last modified timestamp
    #[serde(default)]
    pub modified: Option<String>,

    /// Whether resource is inactive
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether resource is frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== MerchantType Tests ====================

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

    #[test]
    fn merchant_type_invalid_value() {
        assert!(serde_json::from_str::<MerchantType>("99").is_err());
    }

    // ==================== MerchantStatus Tests ====================

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

    #[test]
    fn merchant_status_invalid_value() {
        assert!(serde_json::from_str::<MerchantStatus>("99").is_err());
    }

    // ==================== MerchantEnvironment Tests ====================

    #[test]
    fn merchant_environment_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Supermarket).unwrap(), "\"supermarket\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::MailOrTelephoneOrder).unwrap(), "\"moto\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::CardPresent).unwrap(), "\"cardPresent\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Fuel).unwrap(), "\"fuel\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::ServiceStation).unwrap(), "\"serviceStation\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Restaurant).unwrap(), "\"restaurant\"");
        assert_eq!(serde_json::to_string(&MerchantEnvironment::Ecommerce).unwrap(), "\"eCommerce\"");
    }

    #[test]
    fn merchant_environment_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"supermarket\"").unwrap(), MerchantEnvironment::Supermarket);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"moto\"").unwrap(), MerchantEnvironment::MailOrTelephoneOrder);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"cardPresent\"").unwrap(), MerchantEnvironment::CardPresent);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"fuel\"").unwrap(), MerchantEnvironment::Fuel);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"serviceStation\"").unwrap(), MerchantEnvironment::ServiceStation);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"restaurant\"").unwrap(), MerchantEnvironment::Restaurant);
        assert_eq!(serde_json::from_str::<MerchantEnvironment>("\"eCommerce\"").unwrap(), MerchantEnvironment::Ecommerce);
    }

    #[test]
    fn merchant_environment_default() {
        assert_eq!(MerchantEnvironment::default(), MerchantEnvironment::Ecommerce);
    }

    #[test]
    fn merchant_environment_invalid_value() {
        assert!(serde_json::from_str::<MerchantEnvironment>("\"invalid\"").is_err());
    }

    // ==================== RiskLevel Tests ====================

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

    #[test]
    fn risk_level_invalid_value() {
        assert!(serde_json::from_str::<RiskLevel>("\"invalid\"").is_err());
    }

    // ==================== TaxIdStatus Tests ====================

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

    #[test]
    fn tax_id_status_invalid_value() {
        assert!(serde_json::from_str::<TaxIdStatus>("99").is_err());
    }

    // ==================== Merchant Struct Tests ====================

    #[test]
    fn merchant_deserialize_full() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "entity": "t1_ent_12345678901234567890123",
            "login": "t1_lgn_12345678901234567890123",
            "dba": "Acme Widgets",
            "status": 2,
            "environment": "eCommerce",
            "riskLevel": "low",
            "new": 1,
            "established": "20150101",
            "annualCcSales": 50000000,
            "avgTicket": 2500,
            "mcc": "5734",
            "boarded": "20240101",
            "chargebackNotificationEmail": "chargeback@example.com",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id.as_str(), "t1_mer_12345678901234567890123");
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
        assert_eq!(merchant.created.as_deref(), Some("2024-01-01 00:00:00.000"));
        assert_eq!(merchant.modified.as_deref(), Some("2024-04-01 12:00:00.000"));
        assert!(!merchant.inactive);
        assert!(merchant.frozen);
    }

    #[test]
    fn merchant_deserialize_minimal() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123"
        }"#;

        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert_eq!(merchant.id.as_str(), "t1_mer_12345678901234567890123");
        assert!(merchant.entity.is_none());
        assert!(merchant.status.is_none());
        assert!(merchant.environment.is_none());
        assert!(merchant.risk_level.is_none());
        assert!(!merchant.new);
        assert!(!merchant.inactive);
        assert!(!merchant.frozen);
    }

    #[test]
    fn merchant_bool_from_int_zero_is_false() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "new": 0,
            "inactive": 0,
            "frozen": 0
        }"#;
        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert!(!merchant.new);
        assert!(!merchant.inactive);
        assert!(!merchant.frozen);
    }

    #[test]
    fn merchant_bool_from_int_one_is_true() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123",
            "new": 1,
            "inactive": 1,
            "frozen": 1
        }"#;
        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert!(merchant.new);
        assert!(merchant.inactive);
        assert!(merchant.frozen);
    }

    #[test]
    fn merchant_bool_from_int_missing_defaults_false() {
        let json = r#"{
            "id": "t1_mer_12345678901234567890123"
        }"#;
        let merchant: Merchant = serde_json::from_str(json).unwrap();
        assert!(!merchant.new);
        assert!(!merchant.inactive);
        assert!(!merchant.frozen);
    }

    // ==================== Entity Struct Tests ====================

    #[test]
    fn entity_deserialize_full() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "login": "t1_lgn_12345678901234567890123",
            "name": "Acme Corporation",
            "type": 1,
            "address1": "123 Main St",
            "address2": "Suite 100",
            "city": "Springfield",
            "state": "IL",
            "zip": "62701",
            "country": "USA",
            "phone": "555-1234",
            "fax": "555-5678",
            "email": "info@acme.com",
            "website": "https://acme.com",
            "ein": "12-3456789",
            "tinStatus": 1,
            "custom": "custom data",
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-04-01 12:00:00.000",
            "inactive": 0,
            "frozen": 1
        }"#;

        let entity: Entity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id.as_str(), "t1_ent_12345678901234567890123");
        assert_eq!(entity.login.unwrap().as_str(), "t1_lgn_12345678901234567890123");
        assert_eq!(entity.name.as_deref(), Some("Acme Corporation"));
        assert_eq!(entity.entity_type, Some(MerchantType::Corporation));
        assert_eq!(entity.address1.as_deref(), Some("123 Main St"));
        assert_eq!(entity.address2.as_deref(), Some("Suite 100"));
        assert_eq!(entity.city.as_deref(), Some("Springfield"));
        assert_eq!(entity.state.as_deref(), Some("IL"));
        assert_eq!(entity.zip.as_deref(), Some("62701"));
        assert_eq!(entity.country.as_deref(), Some("USA"));
        assert_eq!(entity.phone.as_deref(), Some("555-1234"));
        assert_eq!(entity.fax.as_deref(), Some("555-5678"));
        assert_eq!(entity.email.as_deref(), Some("info@acme.com"));
        assert_eq!(entity.website.as_deref(), Some("https://acme.com"));
        assert_eq!(entity.ein.as_deref(), Some("12-3456789"));
        assert_eq!(entity.tin_status, Some(TaxIdStatus::Valid));
        assert_eq!(entity.custom.as_deref(), Some("custom data"));
        assert_eq!(entity.created.as_deref(), Some("2024-01-01 00:00:00.000"));
        assert_eq!(entity.modified.as_deref(), Some("2024-04-01 12:00:00.000"));
        assert!(!entity.inactive);
        assert!(entity.frozen);
    }

    #[test]
    fn entity_deserialize_minimal() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123"
        }"#;

        let entity: Entity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id.as_str(), "t1_ent_12345678901234567890123");
        assert!(entity.login.is_none());
        assert!(entity.name.is_none());
        assert!(entity.entity_type.is_none());
        assert!(entity.tin_status.is_none());
        assert!(!entity.inactive);
        assert!(!entity.frozen);
    }

    #[test]
    fn entity_bool_from_int_zero_is_false() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "inactive": 0,
            "frozen": 0
        }"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        assert!(!entity.inactive);
        assert!(!entity.frozen);
    }

    #[test]
    fn entity_bool_from_int_one_is_true() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123",
            "inactive": 1,
            "frozen": 1
        }"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        assert!(entity.inactive);
        assert!(entity.frozen);
    }

    #[test]
    fn entity_bool_from_int_missing_defaults_false() {
        let json = r#"{
            "id": "t1_ent_12345678901234567890123"
        }"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        assert!(!entity.inactive);
        assert!(!entity.frozen);
    }
}
