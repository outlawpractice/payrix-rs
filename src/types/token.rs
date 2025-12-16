//! Token types for the Payrix API.
//!
//! Tokens represent stored payment methods (credit cards or bank accounts).
//! They allow recurring charges without storing sensitive payment data.

use serde::{Deserialize, Serialize};

use super::{bool_from_int_default_false, option_bool_from_int, DateMmyy, PaymentMethod, PayrixId};

/// Token status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenStatus {
    /// Token is pending activation
    #[default]
    Pending,
    /// Token is ready for use
    Ready,
}

/// Token type values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    /// OmniToken
    #[default]
    #[serde(rename = "omnitoken")]
    OmniToken,
    /// Network token
    #[serde(rename = "networktoken")]
    NetworkToken,
    /// Internal token
    Internal,
}

/// A stored payment token.
///
/// Tokens store payment method information securely.
/// Use the `token` field (not `id`) when creating transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Unique identifier (30 characters, e.g., "t1_tok_...")
    pub id: PayrixId,

    /// Associated customer ID
    #[serde(default)]
    pub customer: Option<PayrixId>,

    /// Associated merchant ID
    pub merchant: PayrixId,

    /// Auto-generated token string for use in transactions.
    /// Use this value (not `id`) when processing payments.
    #[serde(default)]
    pub token: Option<String>,

    /// Payment method type
    #[serde(default)]
    pub payment: Option<PaymentMethod>,

    /// First 6 digits of card (BIN)
    #[serde(default)]
    pub first6: Option<String>,

    /// Last 4 digits of card/account number
    #[serde(default)]
    pub last4: Option<String>,

    /// Card expiration in MMYY format (null for bank accounts)
    #[serde(default)]
    pub expiration: Option<DateMmyy>,

    /// Name for this token (e.g., "Primary Card")
    #[serde(default)]
    pub name: Option<String>,

    /// Description field. Often stores your application's contact ID.
    #[serde(default)]
    pub description: Option<String>,

    /// Token status
    #[serde(default)]
    pub status: Option<TokenStatus>,

    /// Custom field for client data (0-1000 chars)
    #[serde(default)]
    pub custom: Option<String>,

    /// Token type
    #[serde(default, rename = "type")]
    pub token_type: Option<TokenType>,

    /// Whether token is inactive (false=active, true=inactive)
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether token is frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub created: Option<String>,

    /// Timestamp in "YYYY-MM-DD HH:mm:ss.sss" format
    #[serde(default)]
    pub modified: Option<String>,
}

/// Payment information for token creation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    /// Payment method type
    pub method: PaymentMethod,

    /// Card/account number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Routing number (for bank accounts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,

    /// Card expiration in MMYY format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,

    /// CVV code (for credit cards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv: Option<String>,
}

/// Custom data stored with a token.
///
/// This is typically serialized as JSON in the `custom` field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCustom {
    /// Your application's firm/tenant ID
    #[serde(default)]
    pub firm_id: Option<String>,
    /// Your application's contact ID
    #[serde(default)]
    pub contact_id: Option<String>,
    /// Your application's case/matter ID
    #[serde(default)]
    pub case_id: Option<String>,
}

/// Request to create a new payment token.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewToken {
    /// Customer ID (required)
    pub customer: String,

    /// Login ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,

    /// Payment information
    pub payment: PaymentInfo,

    /// Name for this token (e.g., "Primary Card")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description field. Store your contact ID here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Custom data (JSON string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// Whether token is inactive (false=active, true=inactive)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,

    /// Whether token is frozen
    #[serde(skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub frozen: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== TokenStatus Tests ====================

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

    // ==================== TokenType Tests ====================

    #[test]
    fn token_type_serialize_all_variants() {
        assert_eq!(serde_json::to_string(&TokenType::OmniToken).unwrap(), "\"omnitoken\"");
        assert_eq!(serde_json::to_string(&TokenType::NetworkToken).unwrap(), "\"networktoken\"");
        assert_eq!(serde_json::to_string(&TokenType::Internal).unwrap(), "\"internal\"");
    }

    #[test]
    fn token_type_deserialize_all_variants() {
        assert_eq!(serde_json::from_str::<TokenType>("\"omnitoken\"").unwrap(), TokenType::OmniToken);
        assert_eq!(serde_json::from_str::<TokenType>("\"networktoken\"").unwrap(), TokenType::NetworkToken);
        assert_eq!(serde_json::from_str::<TokenType>("\"internal\"").unwrap(), TokenType::Internal);
    }

    #[test]
    fn token_type_default() {
        assert_eq!(TokenType::default(), TokenType::OmniToken);
    }

    #[test]
    fn token_type_invalid_value() {
        assert!(serde_json::from_str::<TokenType>("\"invalid\"").is_err());
    }

    // ==================== Token Struct Tests ====================

    #[test]
    fn token_deserialize_full() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "customer": "t1_cus_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123",
            "token": "tok_abc123xyz789",
            "payment": 2,
            "first6": "424242",
            "last4": "4242",
            "expiration": "1225",
            "name": "Primary Card",
            "description": "Contact ID: 12345",
            "status": "ready",
            "custom": "{\"firmId\":\"firm123\"}",
            "type": "omnitoken",
            "inactive": 0,
            "frozen": 1,
            "created": "2024-01-01 00:00:00.000",
            "modified": "2024-01-01 12:00:00.000"
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.id.as_str(), "t1_tok_12345678901234567890123");
        assert_eq!(token.customer.unwrap().as_str(), "t1_cus_12345678901234567890123");
        assert_eq!(token.merchant.as_str(), "t1_mer_12345678901234567890123");
        assert_eq!(token.token.unwrap(), "tok_abc123xyz789");
        assert_eq!(token.payment, Some(PaymentMethod::Visa));
        assert_eq!(token.first6.unwrap(), "424242");
        assert_eq!(token.last4.unwrap(), "4242");
        assert_eq!(token.expiration.unwrap().as_str(), "1225");
        assert_eq!(token.name.unwrap(), "Primary Card");
        assert_eq!(token.status, Some(TokenStatus::Ready));
        assert_eq!(token.token_type, Some(TokenType::OmniToken));
        assert!(!token.inactive);
        assert!(token.frozen);
    }

    #[test]
    fn token_deserialize_minimal() {
        let json = r#"{
            "id": "t1_tok_12345678901234567890123",
            "merchant": "t1_mer_12345678901234567890123"
        }"#;

        let token: Token = serde_json::from_str(json).unwrap();
        assert_eq!(token.id.as_str(), "t1_tok_12345678901234567890123");
        assert_eq!(token.merchant.as_str(), "t1_mer_12345678901234567890123");
        assert!(token.customer.is_none());
        assert!(token.token.is_none());
        assert!(token.status.is_none());
        assert!(token.token_type.is_none());
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    #[test]
    fn token_bool_from_int_zero_is_false() {
        let json = r#"{"id": "t1_tok_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123", "inactive": 0, "frozen": 0}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    #[test]
    fn token_bool_from_int_one_is_true() {
        let json = r#"{"id": "t1_tok_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123", "inactive": 1, "frozen": 1}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(token.inactive);
        assert!(token.frozen);
    }

    #[test]
    fn token_bool_from_int_missing_defaults_false() {
        let json = r#"{"id": "t1_tok_12345678901234567890123", "merchant": "t1_mer_12345678901234567890123"}"#;
        let token: Token = serde_json::from_str(json).unwrap();
        assert!(!token.inactive);
        assert!(!token.frozen);
    }

    // ==================== NewToken Tests ====================

    #[test]
    fn new_token_serialize_full() {
        let new_token = NewToken {
            customer: "t1_cus_12345678901234567890123".to_string(),
            login: Some("t1_log_12345678901234567890123".to_string()),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                number: Some("4242424242424242".to_string()),
                routing: None,
                expiration: Some("1225".to_string()),
                cvv: Some("123".to_string()),
            },
            name: Some("Primary Card".to_string()),
            description: Some("Contact ID: 12345".to_string()),
            custom: Some("{\"firmId\":\"firm123\"}".to_string()),
            inactive: Some(false),
            frozen: Some(true),
        };

        let json = serde_json::to_string(&new_token).unwrap();
        assert!(json.contains("\"customer\":\"t1_cus_12345678901234567890123\""));
        assert!(json.contains("\"method\":2"));
        assert!(json.contains("\"name\":\"Primary Card\""));
        assert!(json.contains("\"inactive\":0"));
        assert!(json.contains("\"frozen\":1"));
    }

    #[test]
    fn new_token_serialize_minimal() {
        let new_token = NewToken {
            customer: "t1_cus_12345678901234567890123".to_string(),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                ..Default::default()
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&new_token).unwrap();
        assert!(json.contains("\"customer\":\"t1_cus_12345678901234567890123\""));
        assert!(json.contains("\"method\":2"));
        // Optional fields should be omitted
        assert!(!json.contains("\"login\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"inactive\""));
        assert!(!json.contains("\"frozen\""));
    }

    #[test]
    fn new_token_option_bool_to_int_true() {
        let new_token = NewToken {
            customer: "t1_cus_12345678901234567890123".to_string(),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                ..Default::default()
            },
            inactive: Some(true),
            frozen: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_token).unwrap();
        assert!(json.contains("\"inactive\":1"));
        assert!(json.contains("\"frozen\":1"));
    }

    #[test]
    fn new_token_option_bool_to_int_false() {
        let new_token = NewToken {
            customer: "t1_cus_12345678901234567890123".to_string(),
            payment: PaymentInfo {
                method: PaymentMethod::Visa,
                ..Default::default()
            },
            inactive: Some(false),
            frozen: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&new_token).unwrap();
        assert!(json.contains("\"inactive\":0"));
        assert!(json.contains("\"frozen\":0"));
    }
}
