//! Payment types for expanded payment information.
//!
//! When using the `expand[payment][]` query parameter, the Payrix API returns
//! a full payment object instead of just the payment method integer.
//!
//! This module provides the `Payment` struct to hold this expanded data.

use serde::{Deserialize, Serialize};

use super::PaymentMethod;

/// Expanded payment information from `expand[payment][]`.
///
/// This is returned when you expand the `payment` field on entities like
/// Token or Transaction. It contains card/account details beyond just
/// the payment method type.
///
/// **Note:** This is NOT the same as `PaymentMethod` (which is just an enum).
/// This struct holds the full expanded payment object.
///
/// # Example Response
///
/// ```json
/// {
///   "bin": "411111",
///   "id": "g157b215cd94669",
///   "last4": null,
///   "lastChecked": null,
///   "mask": null,
///   "method": 2,
///   "number": "1111",
///   "payment": null,
///   "plaidConsumerAccount": null,
///   "routing": "0"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    /// Payment record ID.
    ///
    /// Note: This uses a non-standard ID format (e.g., "g157b215cd94669"),
    /// not the typical `t1_xxx` PayrixId format.
    #[serde(default)]
    pub id: Option<String>,

    /// The payment method type (Visa, Mastercard, etc.).
    #[serde(default)]
    pub method: Option<PaymentMethod>,

    /// Card BIN (Bank Identification Number) - first 6 digits.
    #[serde(default)]
    pub bin: Option<String>,

    /// Last 4 digits of card/account number.
    #[serde(default)]
    pub last4: Option<String>,

    /// Masked card/account number.
    #[serde(default)]
    pub mask: Option<String>,

    /// Card/account number (typically last 4 digits for display).
    #[serde(default)]
    pub number: Option<String>,

    /// Bank routing number (for ACH/bank accounts).
    #[serde(default)]
    pub routing: Option<String>,

    /// Last time the payment method was checked/validated.
    #[serde(default)]
    pub last_checked: Option<String>,

    /// Plaid consumer account reference.
    #[serde(default)]
    pub plaid_consumer_account: Option<String>,

    /// Nested payment reference (usually null).
    #[serde(default)]
    pub payment: Option<Box<Payment>>,
}

impl Payment {
    /// Returns the card display string (e.g., "Visa ending in 1111").
    pub fn display(&self) -> String {
        let method_name = self.method.as_ref().map(|m| format!("{:?}", m)).unwrap_or_default();
        let last4 = self.last4.as_deref()
            .or(self.number.as_deref())
            .unwrap_or("****");

        if method_name.is_empty() {
            format!("**** {}", last4)
        } else {
            format!("{} ending in {}", method_name, last4)
        }
    }

    /// Returns true if this is a card payment method.
    pub fn is_card(&self) -> bool {
        self.method.as_ref().map(|m| m.is_card()).unwrap_or(false)
    }

    /// Returns true if this is a bank account payment method.
    pub fn is_bank_account(&self) -> bool {
        self.method.as_ref().map(|m| m.is_bank()).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_deserialize_from_api_response() {
        let json = r#"{
            "bin": "411111",
            "id": "g157b215cd94669",
            "last4": null,
            "lastChecked": null,
            "mask": null,
            "method": 2,
            "number": "1111",
            "payment": null,
            "plaidConsumerAccount": null,
            "routing": "0"
        }"#;

        let payment: Payment = serde_json::from_str(json).unwrap();

        assert_eq!(payment.id.as_deref(), Some("g157b215cd94669"));
        assert_eq!(payment.method, Some(PaymentMethod::Visa));
        assert_eq!(payment.bin.as_deref(), Some("411111"));
        assert!(payment.last4.is_none());
        assert_eq!(payment.number.as_deref(), Some("1111"));
        assert_eq!(payment.routing.as_deref(), Some("0"));
        assert!(payment.payment.is_none());
    }

    #[test]
    fn payment_display() {
        let payment = Payment {
            id: Some("g157b215cd94669".to_string()),
            method: Some(PaymentMethod::Visa),
            bin: Some("411111".to_string()),
            last4: None,
            mask: None,
            number: Some("1111".to_string()),
            routing: None,
            last_checked: None,
            plaid_consumer_account: None,
            payment: None,
        };

        assert_eq!(payment.display(), "Visa ending in 1111");
        assert!(payment.is_card());
        assert!(!payment.is_bank_account());
    }

    #[test]
    fn payment_display_with_last4() {
        let payment = Payment {
            id: None,
            method: Some(PaymentMethod::Mastercard),
            bin: None,
            last4: Some("4242".to_string()),
            mask: None,
            number: None,
            routing: None,
            last_checked: None,
            plaid_consumer_account: None,
            payment: None,
        };

        assert_eq!(payment.display(), "Mastercard ending in 4242");
    }

    #[test]
    fn payment_bank_account() {
        let payment = Payment {
            id: None,
            method: Some(PaymentMethod::IndividualChecking),
            bin: None,
            last4: Some("9876".to_string()),
            mask: None,
            number: None,
            routing: Some("123456789".to_string()),
            last_checked: None,
            plaid_consumer_account: None,
            payment: None,
        };

        assert!(!payment.is_card());
        assert!(payment.is_bank_account());
        assert_eq!(payment.display(), "IndividualChecking ending in 9876");
    }
}
