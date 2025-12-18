//! Token types for the Payrix API.
//!
//! Tokens represent stored payment methods (credit cards or bank accounts).
//! They allow recurring charges without storing sensitive payment data.
//!
//! **OpenAPI schema:** `tokensResponse`

use payrix_macros::PayrixEntity;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use super::{bool_from_int_default_false, DateMmyy, PaymentMethod, PayrixId, Transaction};

// =============================================================================
// Custom Deserializers
// =============================================================================

/// Deserialize payment method from either an integer or an object.
///
/// The Payrix API may return the payment field as:
/// - An integer (e.g., `2` for Visa)
/// - An object with a `method` field (e.g., `{"method": 2, ...}`)
///
/// **OpenAPI schema:** `tokensModelPayment` is `anyOf` string OR `paymentResponse` object.
fn deserialize_payment_method<'de, D>(deserializer: D) -> Result<Option<PaymentMethod>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PaymentMethodVisitor;

    impl<'de> serde::de::Visitor<'de> for PaymentMethodVisitor {
        type Value = Option<PaymentMethod>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer, object with method field, or null")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            // Direct integer value - match PaymentMethod enum values
            match v {
                1 => Ok(Some(PaymentMethod::AmericanExpress)),
                2 => Ok(Some(PaymentMethod::Visa)),
                3 => Ok(Some(PaymentMethod::Mastercard)),
                4 => Ok(Some(PaymentMethod::DinersClub)),
                5 => Ok(Some(PaymentMethod::Discover)),
                8 => Ok(Some(PaymentMethod::IndividualChecking)),
                9 => Ok(Some(PaymentMethod::IndividualSavings)),
                10 => Ok(Some(PaymentMethod::BusinessChecking)),
                11 => Ok(Some(PaymentMethod::BusinessSavings)),
                _ => Err(serde::de::Error::custom(format!(
                    "unknown payment method value: {}",
                    v
                ))),
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_i64(v as i64)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            // Object with "method" field
            let mut method: Option<PaymentMethod> = None;
            while let Some(key) = map.next_key::<String>()? {
                if key == "method" {
                    method = Some(map.next_value()?);
                } else {
                    // Skip other fields
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
            Ok(method)
        }
    }

    deserializer.deserialize_any(PaymentMethodVisitor)
}

// =============================================================================
// Enums
// =============================================================================

/// Token status values per OpenAPI spec.
///
/// **OpenAPI schema:** `tokenStatus`
///
/// Valid values:
/// - `pending` - The payment data is not yet available, Token is not ready for use.
/// - `ready` - The payment data is available, Token is ready for use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenStatus {
    /// The payment data is not yet available, Token is not ready for use.
    #[default]
    Pending,
    /// The payment data is available, Token is ready for use.
    Ready,
}

// =============================================================================
// Token (Response)
// =============================================================================

/// A stored payment token.
///
/// Tokens store payment method information securely.
/// Use the `token` field (not `id`) when creating transactions.
///
/// **OpenAPI schema:** `tokensResponse`
///
/// See `API_INCONSISTENCIES.md` for known deviations from this spec.
#[derive(Debug, Clone, Serialize, Deserialize, PayrixEntity)]
#[payrix(create = CreateToken, update = UpdateToken)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Token {
    // -------------------------------------------------------------------------
    // Core Identifiers (readonly)
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

    /// The Customer that this Token is associated with.
    ///
    /// Required when creating a token.
    ///
    /// **OpenAPI type:** string (ref: `tokensModelCustomer`)
    #[payrix(create_only, create_required)]
    #[serde(default)]
    pub customer: Option<PayrixId>,

    /// The Login that owns this token.
    ///
    /// Optional when creating a token.
    ///
    /// **OpenAPI type:** string
    #[payrix(create_only)]
    #[serde(default)]
    pub login: Option<PayrixId>,

    // -------------------------------------------------------------------------
    // Payment Information
    // -------------------------------------------------------------------------

    /// The payment method that is associated with this Token.
    ///
    /// When creating a token, provide full payment details via `PaymentInfo`.
    /// The response returns only the payment method type.
    ///
    /// **OpenAPI type:** anyOf (string | `paymentResponse` object)
    #[payrix(create_only, create_required, create_type = "PaymentInfo")]
    #[serde(default, deserialize_with = "deserialize_payment_method")]
    pub payment: Option<PaymentMethod>,

    /// Indicates if this token is ready for use in transactions and subscriptions or not.
    ///
    /// A token without complete payment details will be marked as 'pending'.
    ///
    /// **OpenAPI type:** string (ref: `tokenStatus`)
    ///
    /// Valid values:
    /// - `pending` - The payment data is not yet available, Token is not ready for use.
    /// - `ready` - The payment data is available, Token is ready for use.
    #[payrix(readonly)]
    #[serde(default)]
    pub status: Option<TokenStatus>,

    /// The auto-generated token identifier.
    ///
    /// Use this value (not `id`) when processing payments.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub token: Option<String>,

    /// The magnetic stripe track data for the payment record for use in a transaction.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub track: Option<String>,

    /// The CVV (Card Verification Value) for the payment record for use in a transaction.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub cvv: Option<String>,

    /// The expiry month for the payment method associated with this Token.
    ///
    /// This field is stored as a text string in 'MMYY' format, where 'MM' is the
    /// number of a month and 'YY' is the last two digits of a year.
    /// For example, '0623' for June 2023.
    ///
    /// The value must reflect a future date.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub expiration: Option<DateMmyy>,

    /// The name of this Token.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this Token.
    ///
    /// This field is stored as a text string and must be between 0 and 100 characters long.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub description: Option<String>,

    /// Custom, free-form field for client-supplied text.
    ///
    /// Must be between 0 and 1000 characters long.
    ///
    /// **OpenAPI type:** string
    #[payrix(mutable)]
    #[serde(default)]
    pub custom: Option<String>,

    /// The customer reference from the authToken used for user authentication.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub auth_token_customer: Option<String>,

    /// The origin of the token.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub origin: Option<String>,

    /// Entry mode set to the token.
    ///
    /// **OpenAPI type:** integer (int32)
    #[payrix(readonly)]
    #[serde(default)]
    pub entry_mode: Option<i32>,

    /// The omnitoken value.
    ///
    /// If this field has a value, the whole record is treated as an omnitoken.
    ///
    /// **OpenAPI type:** string
    #[payrix(readonly)]
    #[serde(default)]
    pub omnitoken: Option<String>,

    // -------------------------------------------------------------------------
    // Status Flags (mutable)
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

    // -------------------------------------------------------------------------
    // Nested Relations (expanded via `expand` query parameter)
    // -------------------------------------------------------------------------

    /// Array of payment updates associated with this Token.
    ///
    /// Only populated when using the `expand` query parameter.
    ///
    /// **OpenAPI type:** array of `paymentUpdatesResponse`
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub payment_updates: Option<Vec<serde_json::Value>>,

    /// Array of subscription tokens associated with this Token.
    ///
    /// Only populated when using the `expand` query parameter.
    ///
    /// **OpenAPI type:** array of `subscriptionTokensResponse`
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub subscription_tokens: Option<Vec<serde_json::Value>>,

    /// Array of transactions associated with this Token.
    ///
    /// Only populated when using the `expand` query parameter.
    ///
    /// **OpenAPI type:** array of `txnsResponse`
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub txns: Option<Vec<Transaction>>,
}

// =============================================================================
// Helper Structs
// =============================================================================

/// Payment information for token creation.
///
/// Used when creating new tokens via POST /tokens.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    /// Payment method type.
    ///
    /// **OpenAPI type:** integer (ref: `paymentMethod`)
    pub method: PaymentMethod,

    /// Card/account number.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Routing number (for bank accounts).
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,

    /// Card expiration in MMYY format.
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,

    /// CVV code (for credit cards).
    ///
    /// **OpenAPI type:** string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv: Option<String>,
}

/// Custom data stored with a token.
///
/// This is typically serialized as JSON in the `custom` field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCustom {
    /// Your application's firm/tenant ID.
    #[serde(default)]
    pub firm_id: Option<String>,

    /// Your application's contact ID.
    #[serde(default)]
    pub contact_id: Option<String>,

    /// Your application's case/matter ID.
    #[serde(default)]
    pub case_id: Option<String>,
}

// =============================================================================
// NewToken (Request)
// =============================================================================

// CreateToken is generated by the PayrixEntity derive macro.
// See the Token struct for field documentation.

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // TokenStatus Tests
    // =========================================================================

    #[test]
    fn token_status_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TokenStatus::Pending).unwrap(), "\"pending\"");
        assert_eq!(serde_json::to_string(&TokenStatus::Ready).unwrap(), "\"ready\"");
    }

    #[test]
    fn token_status_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TokenStatus>("\"pending\"").unwrap(), TokenStatus::Pending);
        assert_eq!(serde_json::from_str::<TokenStatus>("\"ready\"").unwrap(), TokenStatus::Ready);
    }

    #[test]
    fn token_status_default() {
        assert_eq!(TokenStatus::default(), TokenStatus::Pending);
    }

    #[test]
    fn token_status_invalid_value() {
        assert!(serde_json::from_str::<TokenStatus>("\"invalid\"").is_err());
    }

    // =========================================================================
    // Token Struct Tests
    // =========================================================================

    #[test]
    fn token_deserialize_full() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-01 12:00:00.0000",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345",
            "customer": "t1_cus_12345678901234567890123",
            "payment": 2,
            "status": "ready",
            "token": "tok_abc123xyz789",
            "track": "track_data_here",
            "cvv": "123",
            "expiration": "1225",
            "name": "Primary Card",
            "description": "Contact ID: 12345",
            "custom": "{\"firmId\":\"firm123\"}",
            "authTokenCustomer": "auth-customer-ref",
            "origin": "web",
            "entryMode": 1,
            "omnitoken": "omni_token_value",
            "inactive": 0,
            "frozen": 1
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();

        // Core identifiers
        assert_eq!(token.id.as_str(), "t1_tok_12345678901234567890123");
        assert_eq!(token.created.as_deref(), Some("2024-01-01 00:00:00.0000"));
        assert_eq!(token.modified.as_deref(), Some("2024-01-01 12:00:00.0000"));
        assert_eq!(token.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(token.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
        assert_eq!(token.customer.as_ref().unwrap().as_str(), "t1_cus_12345678901234567890123");

        // Payment information
        assert_eq!(token.payment, Some(PaymentMethod::Visa));
        assert_eq!(token.status, Some(TokenStatus::Ready));
        assert_eq!(token.token.as_deref(), Some("tok_abc123xyz789"));
        assert_eq!(token.track.as_deref(), Some("track_data_here"));
        assert_eq!(token.cvv.as_deref(), Some("123"));
        assert_eq!(token.expiration.as_ref().unwrap().as_str(), "1225");
        assert_eq!(token.name.as_deref(), Some("Primary Card"));
        assert_eq!(token.description.as_deref(), Some("Contact ID: 12345"));
        assert_eq!(token.custom.as_deref(), Some("{\"firmId\":\"firm123\"}"));
        assert_eq!(token.auth_token_customer.as_deref(), Some("auth-customer-ref"));
        assert_eq!(token.origin.as_deref(), Some("web"));
        assert_eq!(token.entry_mode, Some(1));
        assert_eq!(token.omnitoken.as_deref(), Some("omni_token_value"));

        // Status flags
        assert!(!token.inactive);
        assert!(token.frozen);

        // Nested relations (not expanded)
        assert!(token.payment_updates.is_none());
        assert!(token.subscription_tokens.is_none());
        assert!(token.txns.is_none());
    }

    #[test]
    fn token_deserialize_minimal() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123"
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();

        assert_eq!(token.id.as_str(), "t1_tok_12345678901234567890123");
        assert!(token.created.is_none());
        assert!(token.modified.is_none());
        assert!(token.creator.is_none());
        assert!(token.modifier.is_none());
        assert!(token.customer.is_none());
        assert!(token.payment.is_none());
        assert!(token.status.is_none());
        assert!(token.token.is_none());
        assert!(token.track.is_none());
        assert!(token.cvv.is_none());
        assert!(token.expiration.is_none());
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    #[test]
    fn token_deserialize_payment_as_object() {
        // Payrix API may return payment as an object with method field
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "payment": {"method": 2, "number": "4111xxxxxxxx1111"}
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.payment, Some(PaymentMethod::Visa));
    }

    #[test]
    fn token_deserialize_payment_as_integer() {
        // Payrix API may also return payment as just the method integer
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "payment": 3
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.payment, Some(PaymentMethod::Mastercard));
    }

    #[test]
    fn token_creator_modifier_fields() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "creator": "t1_lgn_creator1234567890123456",
            "modifier": "t1_lgn_modifier123456789012345"
        }"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.creator.as_ref().unwrap().as_str(), "t1_lgn_creator1234567890123456");
        assert_eq!(token.modifier.as_ref().unwrap().as_str(), "t1_lgn_modifier123456789012345");
    }

    #[test]
    fn token_new_fields() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "track": "track_data",
            "authTokenCustomer": "auth-ref-123",
            "origin": "api",
            "entryMode": 5,
            "omnitoken": "omni_value"
        }"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.track.as_deref(), Some("track_data"));
        assert_eq!(token.auth_token_customer.as_deref(), Some("auth-ref-123"));
        assert_eq!(token.origin.as_deref(), Some("api"));
        assert_eq!(token.entry_mode, Some(5));
        assert_eq!(token.omnitoken.as_deref(), Some("omni_value"));
    }

    #[test]
    fn token_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_tok_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    #[test]
    fn token_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_tok_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(token.inactive);
        assert!(token.frozen);
    }

    #[test]
    fn token_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_tok_12345678901234567890123"}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    // =========================================================================
    // CreateToken Tests (generated by PayrixEntity macro)
    // =========================================================================

    #[test]
    fn create_token_serialize_full() {
        let create_token = CreateToken {
            customer: "t1_cus_12345678901234567890123".parse().unwrap(),
            login: Some("t1_log_12345678901234567890123".parse().unwrap()),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                number: Some("4242424242424242".to_string()),
                routing: None,
                expiration: Some("1225".to_string()),
                cvv: Some("123".to_string()),
            },
            expiration: Some("1225".parse().unwrap()),
            name: Some("Primary Card".to_string()),
            description: Some("Contact ID: 12345".to_string()),
            custom: Some("{\"firmId\":\"firm123\"}".to_string()),
            inactive: Some(false),
            frozen: Some(true),
        };

        let json = serde_json::to_string(&create_token).unwrap();
        assert!(json.contains("\"customer\":\"t1_cus_12345678901234567890123\""));
        assert!(json.contains("\"method\":2"));
        assert!(json.contains("\"name\":\"Primary Card\""));
        // Option<bool> serializes as true/false
        assert!(json.contains("\"inactive\":false"));
        assert!(json.contains("\"frozen\":true"));
    }

    #[test]
    fn create_token_serialize_minimal() {
        let create_token = CreateToken {
            customer: "t1_cus_12345678901234567890123".parse().unwrap(),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                ..Default::default()
            },
            login: None,
            expiration: None,
            name: None,
            description: None,
            custom: None,
            inactive: None,
            frozen: None,
        };

        let json = serde_json::to_string(&create_token).unwrap();
        assert!(json.contains("\"customer\":\"t1_cus_12345678901234567890123\""));
        assert!(json.contains("\"method\":2"));
        // Optional fields should be omitted
        assert!(!json.contains("\"login\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"inactive\""));
        assert!(!json.contains("\"frozen\""));
    }

    #[test]
    fn create_token_option_bool_serialization() {
        let create_token = CreateToken {
            customer: "t1_cus_12345678901234567890123".parse().unwrap(),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                ..Default::default()
            },
            login: None,
            expiration: None,
            name: None,
            description: None,
            custom: None,
            inactive: Some(true),
            frozen: Some(false),
        };

        let json = serde_json::to_string(&create_token).unwrap();
        // Option<bool> serializes as true/false, not 0/1
        assert!(json.contains("\"inactive\":true"));
        assert!(json.contains("\"frozen\":false"));
    }

    #[test]
    fn token_custom_serialize_deserialize() {
        let custom = TokenCustom {
            firm_id: Some("firm123".to_string()),
            contact_id: Some("contact456".to_string()),
            case_id: Some("case789".to_string()),
        };

        let json = serde_json::to_string(&custom).unwrap();
        assert!(json.contains("\"firmId\":\"firm123\""));
        assert!(json.contains("\"contactId\":\"contact456\""));
        assert!(json.contains("\"caseId\":\"case789\""));

        let parsed: TokenCustom = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.firm_id.as_deref(), Some("firm123"));
        assert_eq!(parsed.contact_id.as_deref(), Some("contact456"));
        assert_eq!(parsed.case_id.as_deref(), Some("case789"));
    }
}
