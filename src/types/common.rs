//! Common types used across all Payrix API responses.

use crate::error::PayrixApiError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// A validated Payrix ID (exactly 30 characters).
///
/// All Payrix resource IDs follow the format `t1_xxx_...` and are exactly 30 characters.
/// This type enforces that constraint at deserialization time.
///
/// # Example
/// ```
/// use payrix::types::PayrixId;
///
/// // Valid ID (exactly 30 characters)
/// let id: PayrixId = "t1_txn_12345678901234567890123".parse().unwrap();
///
/// // Invalid ID (wrong length) - will fail
/// let result: Result<PayrixId, _> = "too_short".parse();
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayrixId(String);

impl PayrixId {
    /// The required length for all Payrix IDs.
    pub const LENGTH: usize = 30;

    /// Create a new PayrixId, validating the length.
    ///
    /// Returns `Err` if the string is not exactly 30 characters.
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.len() != Self::LENGTH {
            return Err(format!(
                "PayrixId must be exactly {} characters, got {}",
                Self::LENGTH,
                s.len()
            ));
        }
        Ok(Self(s))
    }

    /// Get the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume self and return the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for PayrixId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PayrixId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PayrixId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Serialize for PayrixId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PayrixId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PayrixId {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::new(s).map_err(|e| e.into())
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for PayrixId {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

/// A validated date in YYYYMMDD format (exactly 8 characters).
///
/// This type enforces the format used by Payrix for date fields like
/// `established`, `boarded`, `authDate`, etc.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateYmd(String);

impl DateYmd {
    /// The required length for YYYYMMDD dates.
    pub const LENGTH: usize = 8;

    /// Minimum valid year for Payrix dates.
    pub const MIN_YEAR: u16 = 2000;

    /// Create a new DateYmd, validating the format and date validity.
    ///
    /// Returns `Err` if:
    /// - String is not exactly 8 numeric characters
    /// - Year is before 2000
    /// - Month is not 1-12
    /// - Day is not valid for the given month/year
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.len() != Self::LENGTH {
            return Err(format!(
                "DateYmd must be exactly {} characters (YYYYMMDD), got {}",
                Self::LENGTH,
                s.len()
            ));
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("DateYmd must contain only digits, got '{}'", s));
        }

        // Parse and validate components (safe: we verified all chars are digits above)
        let year: u16 = s[0..4].parse().expect("year digits verified");
        let month: u8 = s[4..6].parse().expect("month digits verified");
        let day: u8 = s[6..8].parse().expect("day digits verified");

        if year < Self::MIN_YEAR {
            return Err(format!(
                "DateYmd year must be >= {}, got {}",
                Self::MIN_YEAR,
                year
            ));
        }

        if !(1..=12).contains(&month) {
            return Err(format!("DateYmd month must be 1-12, got {}", month));
        }

        let max_day = Self::days_in_month(year, month);
        if !(1..=max_day).contains(&day) {
            return Err(format!(
                "DateYmd day must be 1-{} for {}/{}, got {}",
                max_day, year, month, day
            ));
        }

        Ok(Self(s))
    }

    /// Returns the number of days in the given month/year.
    fn days_in_month(year: u16, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 0, // Invalid month, but we check this earlier
        }
    }

    /// Returns true if the given year is a leap year.
    fn is_leap_year(year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Get the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the year component.
    pub fn year(&self) -> u16 {
        self.0[0..4]
            .parse()
            .expect("DateYmd year validated at construction")
    }

    /// Get the month component (1-12).
    pub fn month(&self) -> u8 {
        self.0[4..6]
            .parse()
            .expect("DateYmd month validated at construction")
    }

    /// Get the day component (1-31).
    pub fn day(&self) -> u8 {
        self.0[6..8]
            .parse()
            .expect("DateYmd day validated at construction")
    }

    /// Consume self and return the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for DateYmd {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DateYmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DateYmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Serialize for DateYmd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateYmd {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for DateYmd {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::new(s).map_err(|e| e.into())
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for DateYmd {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

/// A validated card expiration date in MMYY format (exactly 4 characters).
///
/// This type enforces the format used by Payrix for card expiration fields.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateMmyy(String);

impl DateMmyy {
    /// The required length for MMYY dates.
    pub const LENGTH: usize = 4;

    /// Create a new DateMmyy, validating the format.
    ///
    /// Returns `Err` if:
    /// - String is not exactly 4 numeric characters
    /// - Month is not 1-12
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.len() != Self::LENGTH {
            return Err(format!(
                "DateMmyy must be exactly {} characters (MMYY), got {}",
                Self::LENGTH,
                s.len()
            ));
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("DateMmyy must contain only digits, got '{}'", s));
        }

        // Validate month (safe: we verified all chars are digits above)
        let month: u8 = s[0..2].parse().expect("month digits verified");
        if !(1..=12).contains(&month) {
            return Err(format!("DateMmyy month must be 1-12, got {}", month));
        }

        Ok(Self(s))
    }

    /// Get the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the month component (1-12).
    pub fn month(&self) -> u8 {
        self.0[0..2]
            .parse()
            .expect("DateMmyy month validated at construction")
    }

    /// Get the year component (2-digit).
    pub fn year(&self) -> u8 {
        self.0[2..4]
            .parse()
            .expect("DateMmyy year validated at construction")
    }

    /// Get the full 4-digit year (assumes 2000s).
    pub fn full_year(&self) -> u16 {
        2000 + self.year() as u16
    }

    /// Consume self and return the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for DateMmyy {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DateMmyy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DateMmyy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Serialize for DateMmyy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateMmyy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for DateMmyy {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::new(s).map_err(|e| e.into())
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for DateMmyy {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

/// Serde helper module for converting between Rust bools and Payrix's 0/1 integers.
///
/// Use with `#[serde(with = "crate::types::bool_from_int")]` on fields.
pub mod bool_from_int {
    use super::*;

    /// Serialize a bool as 0 or 1.
    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *value { 1 } else { 0 })
    }

    /// Deserialize 0/1 as bool.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(value != 0)
    }
}

/// Serde helper module for converting between `Option<bool>` and Payrix's 0/1 integers.
///
/// Use with `#[serde(with = "crate::types::option_bool_from_int")]` on fields.
/// Deserializes missing values as `None`, 0 as `Some(false)`, non-zero as `Some(true)`.
pub mod option_bool_from_int {
    use super::*;

    /// Serialize `Option<bool>` as 0, 1, or null.
    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(b) => serializer.serialize_i32(if *b { 1 } else { 0 }),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize 0/1/null as `Option<bool>`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<i32> = Option::deserialize(deserializer)?;
        Ok(value.map(|v| v != 0))
    }
}

/// Serde helper for bool that defaults to false when missing.
///
/// Use with `#[serde(default, with = "crate::types::bool_from_int_default_false")]`.
pub mod bool_from_int_default_false {
    use super::*;

    /// Serialize a bool as 0 or 1.
    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *value { 1 } else { 0 })
    }

    /// Deserialize 0/1 as bool, defaulting to false if missing.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Handle both missing values (default) and present values
        let value: Option<i32> = Option::deserialize(deserializer)?;
        Ok(value.map(|v| v != 0).unwrap_or(false))
    }
}

/// Top-level query response wrapper from Payrix.
///
/// Payrix wraps all responses in this structure, which may contain
/// errors even with a 200 status code.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct PayrixQuery<T> {
    /// The response data (if successful)
    pub response: Option<PayrixResponse<T>>,
    /// Errors that occurred (may be present even with HTTP 200)
    #[serde(default)]
    pub errors: Vec<PayrixApiError>,
}

/// Response wrapper containing data and pagination details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct PayrixResponse<T> {
    /// The actual data items
    #[serde(default)]
    pub data: Vec<T>,
    /// Errors within the response
    #[serde(default)]
    pub errors: Vec<PayrixApiError>,
    /// Pagination and totals information
    #[serde(default)]
    pub details: ResponseDetails,
}

/// Response metadata including pagination.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDetails {
    /// Pagination information
    #[serde(default)]
    pub page: PageInfo,
    /// Total count of matching records
    #[serde(default)]
    pub totals: Totals,
}

/// Pagination information.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Current page number (1-indexed)
    #[serde(default)]
    pub current: i32,
    /// Number of items per page
    #[serde(default)]
    pub limit: i32,
    /// Whether there are more pages
    #[serde(default, rename = "hasMore")]
    pub has_more: bool,
}

/// Totals information for paginated queries.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Totals {
    /// Total count of matching records
    #[serde(default)]
    pub count: i64,
}

/// A validated timestamp in "YYYY-MM-DD HH:mm:ss.sss" format.
///
/// This type enforces the timestamp format used by Payrix for `created` and `modified` fields.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(String);

impl Timestamp {
    /// The expected format pattern for validation.
    const FORMAT_DESCRIPTION: &'static str = "YYYY-MM-DD HH:mm:ss.sss";

    /// Create a new Timestamp, validating the format.
    ///
    /// Expected format: "YYYY-MM-DD HH:mm:ss.sss" (23 characters)
    /// Example: "2024-01-15 10:30:45.123"
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();

        // Validate basic length (23 chars for full format, 19 for without ms)
        if s.len() != 23 && s.len() != 19 {
            return Err(format!(
                "Timestamp must be {} format, got '{}' ({} chars)",
                Self::FORMAT_DESCRIPTION,
                s,
                s.len()
            ));
        }

        // Validate structure: YYYY-MM-DD HH:mm:ss[.sss]
        let bytes = s.as_bytes();
        if bytes.len() >= 19 {
            // Check separators
            if bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b' '
                || bytes[13] != b':' || bytes[16] != b':'
            {
                return Err(format!(
                    "Timestamp must be {} format, got '{}'",
                    Self::FORMAT_DESCRIPTION,
                    s
                ));
            }
            if s.len() == 23 && bytes[19] != b'.' {
                return Err(format!(
                    "Timestamp must be {} format, got '{}'",
                    Self::FORMAT_DESCRIPTION,
                    s
                ));
            }
        }

        // Validate numeric parts
        let year: u16 = s[0..4].parse().map_err(|_| {
            format!("Invalid year in timestamp '{}'", s)
        })?;
        let month: u8 = s[5..7].parse().map_err(|_| {
            format!("Invalid month in timestamp '{}'", s)
        })?;
        let day: u8 = s[8..10].parse().map_err(|_| {
            format!("Invalid day in timestamp '{}'", s)
        })?;
        let hour: u8 = s[11..13].parse().map_err(|_| {
            format!("Invalid hour in timestamp '{}'", s)
        })?;
        let minute: u8 = s[14..16].parse().map_err(|_| {
            format!("Invalid minute in timestamp '{}'", s)
        })?;
        let second: u8 = s[17..19].parse().map_err(|_| {
            format!("Invalid second in timestamp '{}'", s)
        })?;

        // Validate ranges
        if year < 2000 {
            return Err(format!("Timestamp year must be >= 2000, got {}", year));
        }
        if !(1..=12).contains(&month) {
            return Err(format!("Timestamp month must be 1-12, got {}", month));
        }
        if !(1..=31).contains(&day) {
            return Err(format!("Timestamp day must be 1-31, got {}", day));
        }
        if hour > 23 {
            return Err(format!("Timestamp hour must be 0-23, got {}", hour));
        }
        if minute > 59 {
            return Err(format!("Timestamp minute must be 0-59, got {}", minute));
        }
        if second > 59 {
            return Err(format!("Timestamp second must be 0-59, got {}", second));
        }

        if s.len() == 23 {
            let _ms: u16 = s[20..23].parse().map_err(|_| {
                format!("Invalid milliseconds in timestamp '{}'", s)
            })?;
        }

        Ok(Self(s))
    }

    /// Get the inner string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume self and return the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Timestamp {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::new(s).map_err(|e| e.into())
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Timestamp {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

/// A validated percentage value (0-100).
///
/// This type enforces valid percentage values for fields like
/// reserve percentages, ownership percentages, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Percent(u8);

impl Percent {
    /// Create a new Percent, validating the value is 0-100.
    pub fn new(value: u8) -> Result<Self, String> {
        if value > 100 {
            return Err(format!("Percent must be 0-100, got {}", value));
        }
        Ok(Self(value))
    }

    /// Create a Percent from a value, clamping to 0-100.
    pub fn clamped(value: u8) -> Self {
        Self(value.min(100))
    }

    /// Get the inner value.
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get the value as a decimal (0.0 to 1.0).
    pub fn as_decimal(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl TryFrom<u8> for Percent {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Percent> for u8 {
    fn from(p: Percent) -> Self {
        p.0
    }
}

impl Serialize for Percent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Payment method types supported by Payrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PaymentMethod {
    /// American Express card
    AmericanExpress = 1,
    /// Visa card
    #[default]
    Visa = 2,
    /// Mastercard
    Mastercard = 3,
    /// Diners Club card
    DinersClub = 4,
    /// Discover card
    Discover = 5,
    /// Individual checking account
    IndividualChecking = 8,
    /// Individual savings account
    IndividualSavings = 9,
    /// Business checking account
    BusinessChecking = 10,
    /// Business savings account
    BusinessSavings = 11,
}

impl PaymentMethod {
    /// Returns true if this is a credit card payment method.
    pub fn is_card(&self) -> bool {
        matches!(
            self,
            PaymentMethod::AmericanExpress
                | PaymentMethod::Visa
                | PaymentMethod::Mastercard
                | PaymentMethod::DinersClub
                | PaymentMethod::Discover
        )
    }

    /// Returns true if this is a bank account payment method.
    pub fn is_bank(&self) -> bool {
        matches!(
            self,
            PaymentMethod::IndividualChecking
                | PaymentMethod::IndividualSavings
                | PaymentMethod::BusinessChecking
                | PaymentMethod::BusinessSavings
        )
    }

    /// Get the display name for this payment method.
    pub fn display_name(&self) -> &'static str {
        match self {
            PaymentMethod::AmericanExpress => "American Express",
            PaymentMethod::Visa => "Visa",
            PaymentMethod::Mastercard => "Mastercard",
            PaymentMethod::DinersClub => "Diners Club",
            PaymentMethod::Discover => "Discover",
            PaymentMethod::IndividualChecking => "Individual Checking",
            PaymentMethod::IndividualSavings => "Individual Savings",
            PaymentMethod::BusinessChecking => "Business Checking",
            PaymentMethod::BusinessSavings => "Business Savings",
        }
    }
}

/// Bank account type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BankAccountType {
    /// Checking account
    Checking,
    /// Savings account
    Savings,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PayrixId Tests ====================

    #[test]
    fn payrix_id_valid() {
        let id = PayrixId::new("t1_txn_12345678901234567890123").unwrap();
        assert_eq!(id.as_str(), "t1_txn_12345678901234567890123");
        assert_eq!(id.to_string(), "t1_txn_12345678901234567890123");
    }

    #[test]
    fn payrix_id_exactly_30_chars() {
        // Exactly 30 characters should work
        let id = PayrixId::new("123456789012345678901234567890");
        assert!(id.is_ok());
        assert_eq!(id.unwrap().as_str().len(), 30);
    }

    #[test]
    fn payrix_id_too_short() {
        let result = PayrixId::new("t1_txn_short");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 30 characters"));
    }

    #[test]
    fn payrix_id_too_long() {
        let result = PayrixId::new("t1_txn_1234567890123456789012345678901234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 30 characters"));
    }

    #[test]
    fn payrix_id_empty() {
        let result = PayrixId::new("");
        assert!(result.is_err());
    }

    #[test]
    fn payrix_id_from_str() {
        let id: PayrixId = "t1_mer_12345678901234567890123".parse().unwrap();
        assert_eq!(id.as_str(), "t1_mer_12345678901234567890123");
    }

    #[test]
    fn payrix_id_serialize() {
        let id = PayrixId::new("t1_txn_12345678901234567890123").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"t1_txn_12345678901234567890123\"");
    }

    #[test]
    fn payrix_id_deserialize_valid() {
        let json = "\"t1_txn_12345678901234567890123\"";
        let id: PayrixId = serde_json::from_str(json).unwrap();
        assert_eq!(id.as_str(), "t1_txn_12345678901234567890123");
    }

    #[test]
    fn payrix_id_deserialize_invalid_length() {
        let json = "\"too_short\"";
        let result: Result<PayrixId, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn payrix_id_into_inner() {
        let id = PayrixId::new("t1_txn_12345678901234567890123").unwrap();
        let s: String = id.into_inner();
        assert_eq!(s, "t1_txn_12345678901234567890123");
    }

    // ==================== DateYmd Tests ====================

    #[test]
    fn date_ymd_valid() {
        let date = DateYmd::new("20231215").unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 15);
        assert_eq!(date.as_str(), "20231215");
    }

    #[test]
    fn date_ymd_first_day_of_year() {
        let date = DateYmd::new("20240101").unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn date_ymd_last_day_of_year() {
        let date = DateYmd::new("20231231").unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 31);
    }

    #[test]
    fn date_ymd_year_2000() {
        // Year 2000 should be valid (minimum)
        let date = DateYmd::new("20000101");
        assert!(date.is_ok());
    }

    #[test]
    fn date_ymd_year_before_2000() {
        let result = DateYmd::new("19991231");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("year must be >= 2000"));
    }

    #[test]
    fn date_ymd_invalid_month_zero() {
        let result = DateYmd::new("20230015");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("month must be 1-12"));
    }

    #[test]
    fn date_ymd_invalid_month_13() {
        let result = DateYmd::new("20231315");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("month must be 1-12"));
    }

    #[test]
    fn date_ymd_invalid_day_zero() {
        let result = DateYmd::new("20231200");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be"));
    }

    #[test]
    fn date_ymd_invalid_day_32() {
        let result = DateYmd::new("20231232");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be 1-31"));
    }

    #[test]
    fn date_ymd_feb_28_non_leap_year() {
        // Feb 28 in non-leap year should be valid
        let date = DateYmd::new("20230228");
        assert!(date.is_ok());
    }

    #[test]
    fn date_ymd_feb_29_non_leap_year() {
        // Feb 29 in non-leap year (2023) should fail
        let result = DateYmd::new("20230229");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be 1-28"));
    }

    #[test]
    fn date_ymd_feb_29_leap_year() {
        // Feb 29 in leap year (2024) should be valid
        let date = DateYmd::new("20240229");
        assert!(date.is_ok());
        assert_eq!(date.unwrap().day(), 29);
    }

    #[test]
    fn date_ymd_feb_30_leap_year() {
        // Feb 30 should never be valid, even in leap year
        let result = DateYmd::new("20240230");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be 1-29"));
    }

    #[test]
    fn date_ymd_leap_year_divisible_by_100_not_400() {
        // 2100 is divisible by 100 but not 400, so NOT a leap year
        let result = DateYmd::new("21000229");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be 1-28"));
    }

    #[test]
    fn date_ymd_leap_year_divisible_by_400() {
        // 2000 is divisible by 400, so IS a leap year
        let date = DateYmd::new("20000229");
        assert!(date.is_ok());
    }

    #[test]
    fn date_ymd_april_30() {
        // April has 30 days
        let date = DateYmd::new("20230430");
        assert!(date.is_ok());
    }

    #[test]
    fn date_ymd_april_31() {
        // April 31 doesn't exist
        let result = DateYmd::new("20230431");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("day must be 1-30"));
    }

    #[test]
    fn date_ymd_june_30() {
        let date = DateYmd::new("20230630");
        assert!(date.is_ok());
    }

    #[test]
    fn date_ymd_june_31() {
        let result = DateYmd::new("20230631");
        assert!(result.is_err());
    }

    #[test]
    fn date_ymd_too_short() {
        let result = DateYmd::new("2023121");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 8 characters"));
    }

    #[test]
    fn date_ymd_too_long() {
        let result = DateYmd::new("202312150");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 8 characters"));
    }

    #[test]
    fn date_ymd_non_numeric() {
        let result = DateYmd::new("2023Dec5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("only digits"));
    }

    #[test]
    fn date_ymd_serialize() {
        let date = DateYmd::new("20231215").unwrap();
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, "\"20231215\"");
    }

    #[test]
    fn date_ymd_deserialize_valid() {
        let json = "\"20231215\"";
        let date: DateYmd = serde_json::from_str(json).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn date_ymd_deserialize_invalid() {
        let json = "\"20231315\""; // Invalid month
        let result: Result<DateYmd, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ==================== DateMmyy Tests ====================

    #[test]
    fn date_mmyy_valid() {
        let date = DateMmyy::new("1225").unwrap();
        assert_eq!(date.month(), 12);
        assert_eq!(date.year(), 25);
        assert_eq!(date.full_year(), 2025);
        assert_eq!(date.as_str(), "1225");
    }

    #[test]
    fn date_mmyy_january() {
        let date = DateMmyy::new("0130").unwrap();
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 30);
        assert_eq!(date.full_year(), 2030);
    }

    #[test]
    fn date_mmyy_december() {
        let date = DateMmyy::new("1299").unwrap();
        assert_eq!(date.month(), 12);
        assert_eq!(date.year(), 99);
        assert_eq!(date.full_year(), 2099);
    }

    #[test]
    fn date_mmyy_year_00() {
        // Year 00 should be valid (2000)
        let date = DateMmyy::new("0600").unwrap();
        assert_eq!(date.year(), 0);
        assert_eq!(date.full_year(), 2000);
    }

    #[test]
    fn date_mmyy_invalid_month_zero() {
        let result = DateMmyy::new("0025");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("month must be 1-12"));
    }

    #[test]
    fn date_mmyy_invalid_month_13() {
        let result = DateMmyy::new("1325");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("month must be 1-12"));
    }

    #[test]
    fn date_mmyy_too_short() {
        let result = DateMmyy::new("123");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 4 characters"));
    }

    #[test]
    fn date_mmyy_too_long() {
        let result = DateMmyy::new("12345");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 4 characters"));
    }

    #[test]
    fn date_mmyy_non_numeric() {
        let result = DateMmyy::new("12YY");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("only digits"));
    }

    #[test]
    fn date_mmyy_serialize() {
        let date = DateMmyy::new("1225").unwrap();
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, "\"1225\"");
    }

    #[test]
    fn date_mmyy_deserialize_valid() {
        let json = "\"0628\"";
        let date: DateMmyy = serde_json::from_str(json).unwrap();
        assert_eq!(date.month(), 6);
        assert_eq!(date.year(), 28);
    }

    #[test]
    fn date_mmyy_deserialize_invalid() {
        let json = "\"1328\""; // Invalid month
        let result: Result<DateMmyy, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn date_mmyy_into_inner() {
        let date = DateMmyy::new("1225").unwrap();
        let s: String = date.into_inner();
        assert_eq!(s, "1225");
    }

    // ==================== Option<T> Deserialization Tests ====================

    #[test]
    fn option_payrix_id_deserialize_null() {
        let json = "null";
        let result: Option<PayrixId> = serde_json::from_str(json).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn option_payrix_id_deserialize_valid() {
        let json = "\"t1_txn_12345678901234567890123\"";
        let result: Option<PayrixId> = serde_json::from_str(json).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str(), "t1_txn_12345678901234567890123");
    }

    #[test]
    fn option_date_ymd_deserialize_null() {
        let json = "null";
        let result: Option<DateYmd> = serde_json::from_str(json).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn option_date_mmyy_deserialize_null() {
        let json = "null";
        let result: Option<DateMmyy> = serde_json::from_str(json).unwrap();
        assert!(result.is_none());
    }

    // ==================== Edge Cases ====================

    #[test]
    fn date_ymd_all_months_max_days() {
        // Test last valid day of each month in non-leap year
        let test_cases = [
            ("20230131", 31), // Jan
            ("20230228", 28), // Feb (non-leap)
            ("20230331", 31), // Mar
            ("20230430", 30), // Apr
            ("20230531", 31), // May
            ("20230630", 30), // Jun
            ("20230731", 31), // Jul
            ("20230831", 31), // Aug
            ("20230930", 30), // Sep
            ("20231031", 31), // Oct
            ("20231130", 30), // Nov
            ("20231231", 31), // Dec
        ];

        for (date_str, expected_day) in test_cases {
            let date = DateYmd::new(date_str).unwrap();
            assert_eq!(date.day(), expected_day, "Failed for {}", date_str);
        }
    }

    #[test]
    fn date_ymd_all_months_invalid_day_after_max() {
        // Test invalid day (max + 1) for each month in non-leap year
        let invalid_dates = [
            "20230132", // Jan 32
            "20230229", // Feb 29 (non-leap)
            "20230332", // Mar 32
            "20230431", // Apr 31
            "20230532", // May 32
            "20230631", // Jun 31
            "20230732", // Jul 32
            "20230832", // Aug 32
            "20230931", // Sep 31
            "20231032", // Oct 32
            "20231131", // Nov 31
            "20231232", // Dec 32
        ];

        for date_str in invalid_dates {
            let result = DateYmd::new(date_str);
            assert!(result.is_err(), "Expected {} to be invalid", date_str);
        }
    }
}
