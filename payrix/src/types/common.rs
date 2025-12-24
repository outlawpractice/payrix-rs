//! Common types used across all Payrix API responses.

use crate::error::PayrixApiError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// A validated Payrix ID.
///
/// All Payrix resource IDs follow the format `t1_xxx_...` and are typically
/// 29-32 characters long. Most are 30 characters, but some endpoints return
/// IDs with slightly different lengths.
///
/// NOTE: The API is inconsistent with ID lengths. This type accepts any
/// non-empty string up to 50 characters to handle various internal ID formats.
///
/// # Example
/// ```
/// use payrix::types::PayrixId;
///
/// // Valid ID (30 characters - most common)
/// let id: PayrixId = "t1_txn_12345678901234567890123".parse().unwrap();
///
/// // Also valid (32 characters - some endpoints)
/// let id2: PayrixId = "t1_txn_1234567890123456789012345".parse().unwrap();
///
/// // Invalid ID (too short) - will fail
/// let result: Result<PayrixId, _> = "too_short".parse();
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayrixId(String);

impl PayrixId {
    /// The typical length for Payrix IDs (most common).
    pub const TYPICAL_LENGTH: usize = 30;
    /// Minimum accepted length for Payrix IDs.
    ///
    /// NOTE: The API sometimes returns shorter IDs (e.g., 15 chars) for
    /// certain internal/system references. Standard IDs are 29-32 chars.
    pub const MIN_LENGTH: usize = 10;
    /// Maximum accepted length for Payrix IDs.
    pub const MAX_LENGTH: usize = 32;

    /// Create a new PayrixId, validating the length.
    ///
    /// Returns `Err` if the string is not between 10 and 32 characters.
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.len() < Self::MIN_LENGTH || s.len() > Self::MAX_LENGTH {
            return Err(format!(
                "PayrixId must be between {} and {} characters, got {}",
                Self::MIN_LENGTH,
                Self::MAX_LENGTH,
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

/// A date value from the Payrix API.
///
/// This type stores date values used by Payrix for date fields like
/// `established`, `boarded`, `authDate`, etc. The API typically returns
/// YYYYMMDD format (8 characters), but may also return shorter values
/// like just a year (4 characters) in some cases.
///
/// NOTE: The API is inconsistent with date formats. This type accepts
/// any numeric string to avoid deserialization failures.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateYmd(String);

impl DateYmd {
    /// The expected length for YYYYMMDD dates.
    pub const LENGTH: usize = 8;

    /// Minimum valid year for Payrix dates.
    pub const MIN_YEAR: u16 = 2000;

    /// Create a new DateYmd from any numeric string.
    ///
    /// Returns `Err` if the string contains non-digit characters or is empty.
    ///
    /// NOTE: This accepts any length to handle API inconsistencies where
    /// some date fields return just a year (4 chars) instead of YYYYMMDD.
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.is_empty() {
            return Err("DateYmd cannot be empty".to_string());
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("DateYmd must contain only digits, got '{}'", s));
        }

        // Only validate if we have a full 8-char date
        if s.len() == Self::LENGTH {
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

    /// Get the year component if the date has at least 4 characters.
    pub fn year(&self) -> Option<u16> {
        if self.0.len() >= 4 {
            self.0[0..4].parse().ok()
        } else {
            None
        }
    }

    /// Get the month component (1-12) if the date has at least 6 characters.
    pub fn month(&self) -> Option<u8> {
        if self.0.len() >= 6 {
            self.0[4..6].parse().ok()
        } else {
            None
        }
    }

    /// Get the day component (1-31) if the date has at least 8 characters.
    pub fn day(&self) -> Option<u8> {
        if self.0.len() >= 8 {
            self.0[6..8].parse().ok()
        } else {
            None
        }
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
        // Payrix API can return dates as either strings ("20231215") or integers (20231215)
        struct DateYmdVisitor;

        impl<'de> serde::de::Visitor<'de> for DateYmdVisitor {
            type Value = DateYmd;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string or integer in YYYYMMDD format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                DateYmd::new(v).map_err(serde::de::Error::custom)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                DateYmd::new(v.to_string()).map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                DateYmd::new(v.to_string()).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(DateYmdVisitor)
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

/// Deserialize an `Option<i64>` from either a string or integer.
///
/// The Payrix API may return amounts as either strings (`"1000"`) or integers (`1000`).
pub fn deserialize_optional_amount<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionalAmountVisitor;

    impl<'de> serde::de::Visitor<'de> for OptionalAmountVisitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer, string containing an integer, or null")
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
            Ok(Some(v))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v as i64))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<i64>()
                .map(Some)
                .map_err(|_| serde::de::Error::custom(format!("invalid amount string: {}", v)))
        }
    }

    deserializer.deserialize_any(OptionalAmountVisitor)
}

/// Deserialize an `Option<i32>` from either a string or integer.
///
/// The Payrix API may return small integers as either strings (`"1"`) or integers (`1`).
pub fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionalI32Visitor;

    impl<'de> serde::de::Visitor<'de> for OptionalI32Visitor {
        type Value = Option<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer, string containing an integer, or null")
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
            Ok(Some(v as i32))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v as i32))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse::<i32>()
                .map(Some)
                .map_err(|_| serde::de::Error::custom(format!("invalid i32 string: {}", v)))
        }
    }

    deserializer.deserialize_any(OptionalI32Visitor)
}

/// Deserialize an `Option<String>` from either a string or integer.
///
/// The Payrix API sometimes returns date fields as integers (e.g., `20251216`)
/// when the schema documents them as strings.
pub fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrIntVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrIntVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string, integer, or null")
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
            Ok(Some(v.to_string()))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.to_string()))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v))
        }
    }

    deserializer.deserialize_any(StringOrIntVisitor)
}

/// Macro to implement flexible deserialization for i32 repr enums.
///
/// The Payrix API sometimes returns integer enum values as strings (e.g., `"1"` instead of `1`).
/// This macro creates a custom Deserialize implementation that accepts both formats.
///
/// # Usage
/// ```ignore
/// impl_flexible_i32_enum_deserialize!(TransactionType, [
///     (1, CreditCardSale),
///     (2, CreditCardAuth),
///     // ...
/// ]);
/// ```
#[macro_export]
macro_rules! impl_flexible_i32_enum_deserialize {
    ($enum_name:ident, [$(($value:expr, $variant:ident)),* $(,)?]) => {
        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $enum_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(concat!("an integer or string integer for ", stringify!($enum_name)))
                    }

                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v as i32 {
                            $($value => Ok($enum_name::$variant),)*
                            other => Err(serde::de::Error::custom(format!(
                                "unknown {} value: {}",
                                stringify!($enum_name),
                                other
                            ))),
                        }
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_i64(v as i64)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v.parse::<i32>() {
                            Ok(i) => self.visit_i64(i as i64),
                            Err(_) => Err(serde::de::Error::custom(format!(
                                "invalid {} string: {}",
                                stringify!($enum_name),
                                v
                            ))),
                        }
                    }
                }

                deserializer.deserialize_any(EnumVisitor)
            }
        }
    };
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

// =============================================================================
// COMPOSABLE ENTITY COMPONENTS
// =============================================================================

/// Common metadata fields present on all Payrix entity responses.
///
/// These fields are read-only and set by the API. They should never be
/// included in create or update requests.
///
/// Use with `#[serde(flatten)]` to include in entity response structs.
///
/// # Example
///
/// ```ignore
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct MyEntity {
///     pub id: PayrixId,
///     #[serde(flatten)]
///     pub meta: EntityMeta,
///     // ... entity-specific fields
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityMeta {
    /// The date and time at which this resource was created.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    #[serde(default)]
    pub created: Option<String>,

    /// The date and time at which this resource was modified.
    ///
    /// Format: `YYYY-MM-DD HH:MM:SS.SSSS`
    #[serde(default)]
    pub modified: Option<String>,

    /// The identifier of the Login that created this resource.
    #[serde(default)]
    pub creator: Option<PayrixId>,

    /// The identifier of the Login that last modified this resource.
    #[serde(default)]
    pub modifier: Option<PayrixId>,
}

/// Common status flags present on all Payrix entity responses.
///
/// These fields are mutable and can be set on both create and update requests.
/// For responses, use this type with `bool` fields that default to `false`.
///
/// Use with `#[serde(flatten)]` to include in entity response structs.
///
/// # Example
///
/// ```ignore
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct MyEntity {
///     pub id: PayrixId,
///     #[serde(flatten)]
///     pub status: StatusFlags,
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusFlags {
    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    #[serde(default, with = "bool_from_int_default_false")]
    pub inactive: bool,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    #[serde(default, with = "bool_from_int_default_false")]
    pub frozen: bool,
}

/// Common status flags for create/update requests.
///
/// Unlike `StatusFlags`, these use `Option<bool>` so that omitting a field
/// doesn't change its value (useful for partial updates).
///
/// Use with `#[serde(flatten)]` to include in request structs.
///
/// # Example
///
/// ```ignore
/// #[derive(Debug, Clone, Serialize)]
/// pub struct UpdateMyEntity {
///     pub name: Option<String>,
///     #[serde(flatten)]
///     pub status: StatusFlagsRequest,
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusFlagsRequest {
    /// Whether this resource is marked as inactive.
    ///
    /// - `0` - Active
    /// - `1` - Inactive
    #[serde(default, skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub inactive: Option<bool>,

    /// Whether this resource is marked as frozen.
    ///
    /// - `0` - Not Frozen
    /// - `1` - Frozen
    #[serde(default, skip_serializing_if = "Option::is_none", with = "option_bool_from_int")]
    pub frozen: Option<bool>,
}

impl StatusFlagsRequest {
    /// Create a request to set inactive status.
    pub fn inactive(value: bool) -> Self {
        Self {
            inactive: Some(value),
            frozen: None,
        }
    }

    /// Create a request to set frozen status.
    pub fn frozen(value: bool) -> Self {
        Self {
            inactive: None,
            frozen: Some(value),
        }
    }

    /// Create a request to deactivate the entity.
    pub fn deactivate() -> Self {
        Self::inactive(true)
    }

    /// Create a request to reactivate the entity.
    pub fn activate() -> Self {
        Self::inactive(false)
    }
}

/// Common name and description fields present on many Payrix entities.
///
/// Used by Alert, AlertTrigger, Plan, Token, and other entity types.
///
/// # Example
///
/// ```ignore
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct MyEntity {
///     pub id: PayrixId,
///     #[serde(flatten)]
///     pub naming: NameDescription,
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameDescription {
    /// The name of this resource.
    ///
    /// Typically 0-100 characters.
    #[serde(default)]
    pub name: Option<String>,

    /// A description of this resource.
    ///
    /// Typically 0-100 characters.
    #[serde(default)]
    pub description: Option<String>,
}

impl NameDescription {
    /// Create a new NameDescription with just a name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            description: None,
        }
    }

    /// Create with both name and description.
    pub fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            description: Some(description.into()),
        }
    }
}

/// Common name and description fields for create/update requests.
///
/// Uses `Option` for both fields so partial updates only change specified fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameDescriptionRequest {
    /// The name of this resource (0-100 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A description of this resource (0-100 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl NameDescriptionRequest {
    /// Create with just a name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            description: None,
        }
    }

    /// Add a description to this request.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
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
    fn payrix_id_empty_is_error() {
        let result = PayrixId::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 10 and 32 characters"));
    }

    #[test]
    fn payrix_id_too_long() {
        let result = PayrixId::new("t1_txn_1234567890123456789012345678901234567890");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 10 and 32 characters"));
    }

    #[test]
    fn payrix_id_15_chars_ok() {
        // Some API responses return 15 character IDs for internal references
        let result = PayrixId::new("t1_txn_12345678");
        assert!(result.is_ok());
    }

    #[test]
    fn payrix_id_10_chars_ok() {
        // Minimum valid length
        let result = PayrixId::new("t1_txn_123");
        assert!(result.is_ok());
    }

    #[test]
    fn payrix_id_32_chars_ok() {
        // Some API responses return 32 character IDs
        let result = PayrixId::new("t1_txn_1234567890123456789012345");
        assert!(result.is_ok());
    }

    #[test]
    fn payrix_id_29_chars_ok() {
        // Some API responses return 29 character IDs
        let result = PayrixId::new("t1_txn_1234567890123456789012");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str().len(), 29);
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
    fn payrix_id_deserialize_empty_is_error() {
        let json = "\"\"";
        let result: Result<PayrixId, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn payrix_id_deserialize_short_is_error() {
        // Single character IDs are not valid PayrixIds
        let json = "\"x\"";
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
        assert_eq!(date.year().unwrap(), 2023);
        assert_eq!(date.month().unwrap(), 12);
        assert_eq!(date.day().unwrap(), 15);
        assert_eq!(date.as_str(), "20231215");
    }

    #[test]
    fn date_ymd_first_day_of_year() {
        let date = DateYmd::new("20240101").unwrap();
        assert_eq!(date.year().unwrap(), 2024);
        assert_eq!(date.month().unwrap(), 1);
        assert_eq!(date.day().unwrap(), 1);
    }

    #[test]
    fn date_ymd_last_day_of_year() {
        let date = DateYmd::new("20231231").unwrap();
        assert_eq!(date.year().unwrap(), 2023);
        assert_eq!(date.month().unwrap(), 12);
        assert_eq!(date.day().unwrap(), 31);
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
        assert_eq!(date.unwrap().day().unwrap(), 29);
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
    fn date_ymd_short_is_valid() {
        // Short dates (e.g., just a year) are now accepted to handle API inconsistencies
        let result = DateYmd::new("2023");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "2023");
    }

    #[test]
    fn date_ymd_long_is_valid() {
        // Longer numeric strings are accepted as-is
        let result = DateYmd::new("202312150");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "202312150");
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
    fn date_ymd_deserialize_valid_string() {
        let json = "\"20231215\"";
        let date: DateYmd = serde_json::from_str(json).unwrap();
        assert_eq!(date.year().unwrap(), 2023);
        assert_eq!(date.month().unwrap(), 12);
        assert_eq!(date.day().unwrap(), 15);
    }

    #[test]
    fn date_ymd_deserialize_valid_integer() {
        // Payrix API can return dates as integers
        let json = "20231215";
        let date: DateYmd = serde_json::from_str(json).unwrap();
        assert_eq!(date.year().unwrap(), 2023);
        assert_eq!(date.month().unwrap(), 12);
        assert_eq!(date.day().unwrap(), 15);
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
            assert_eq!(date.day().unwrap(), expected_day, "Failed for {}", date_str);
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

    // ==================== EntityMeta Tests ====================

    #[test]
    fn entity_meta_deserialize() {
        let json = r#"{
            "created": "2024-01-01 00:00:00.0000",
            "modified": "2024-01-02 12:00:00.0000",
            "creator": "t1_lgn_12345678901234567890123",
            "modifier": "t1_lgn_12345678901234567890124"
        }"#;

        let meta: EntityMeta = serde_json::from_str(json).unwrap();
        assert_eq!(meta.created.as_deref(), Some("2024-01-01 00:00:00.0000"));
        assert_eq!(meta.modified.as_deref(), Some("2024-01-02 12:00:00.0000"));
        assert_eq!(meta.creator.as_ref().map(|c| c.as_str()), Some("t1_lgn_12345678901234567890123"));
        assert_eq!(meta.modifier.as_ref().map(|m| m.as_str()), Some("t1_lgn_12345678901234567890124"));
    }

    #[test]
    fn entity_meta_deserialize_empty() {
        let json = "{}";
        let meta: EntityMeta = serde_json::from_str(json).unwrap();
        assert!(meta.created.is_none());
        assert!(meta.modified.is_none());
        assert!(meta.creator.is_none());
        assert!(meta.modifier.is_none());
    }

    #[test]
    fn entity_meta_default() {
        let meta = EntityMeta::default();
        assert!(meta.created.is_none());
        assert!(meta.modified.is_none());
        assert!(meta.creator.is_none());
        assert!(meta.modifier.is_none());
    }

    // ==================== StatusFlags Tests ====================

    #[test]
    fn status_flags_deserialize() {
        let json = r#"{"inactive": 1, "frozen": 0}"#;
        let status: StatusFlags = serde_json::from_str(json).unwrap();
        assert!(status.inactive);
        assert!(!status.frozen);
    }

    #[test]
    fn status_flags_deserialize_empty() {
        let json = "{}";
        let status: StatusFlags = serde_json::from_str(json).unwrap();
        assert!(!status.inactive);
        assert!(!status.frozen);
    }

    #[test]
    fn status_flags_default() {
        let status = StatusFlags::default();
        assert!(!status.inactive);
        assert!(!status.frozen);
    }

    #[test]
    fn status_flags_serialize() {
        let status = StatusFlags { inactive: true, frozen: false };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"inactive\":1"));
        assert!(json.contains("\"frozen\":0"));
    }

    // ==================== StatusFlagsRequest Tests ====================

    #[test]
    fn status_flags_request_serialize() {
        let status = StatusFlagsRequest { inactive: Some(true), frozen: Some(false) };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"inactive\":1"));
        assert!(json.contains("\"frozen\":0"));
    }

    #[test]
    fn status_flags_request_serialize_empty() {
        let status = StatusFlagsRequest::default();
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn status_flags_request_serialize_partial() {
        let status = StatusFlagsRequest::inactive(true);
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"inactive\":1"));
        assert!(!json.contains("frozen"));
    }

    #[test]
    fn status_flags_request_deactivate() {
        let status = StatusFlagsRequest::deactivate();
        assert_eq!(status.inactive, Some(true));
        assert_eq!(status.frozen, None);
    }

    #[test]
    fn status_flags_request_activate() {
        let status = StatusFlagsRequest::activate();
        assert_eq!(status.inactive, Some(false));
        assert_eq!(status.frozen, None);
    }

    // ==================== NameDescription Tests ====================

    #[test]
    fn name_description_deserialize() {
        let json = r#"{"name": "Test Name", "description": "Test Description"}"#;
        let nd: NameDescription = serde_json::from_str(json).unwrap();
        assert_eq!(nd.name.as_deref(), Some("Test Name"));
        assert_eq!(nd.description.as_deref(), Some("Test Description"));
    }

    #[test]
    fn name_description_deserialize_empty() {
        let json = "{}";
        let nd: NameDescription = serde_json::from_str(json).unwrap();
        assert!(nd.name.is_none());
        assert!(nd.description.is_none());
    }

    #[test]
    fn name_description_new() {
        let nd = NameDescription::new("Test");
        assert_eq!(nd.name.as_deref(), Some("Test"));
        assert!(nd.description.is_none());
    }

    #[test]
    fn name_description_with_description() {
        let nd = NameDescription::with_description("Test", "Description");
        assert_eq!(nd.name.as_deref(), Some("Test"));
        assert_eq!(nd.description.as_deref(), Some("Description"));
    }

    // ==================== NameDescriptionRequest Tests ====================

    #[test]
    fn name_description_request_serialize() {
        let nd = NameDescriptionRequest {
            name: Some("Test".to_string()),
            description: Some("Desc".to_string()),
        };
        let json = serde_json::to_string(&nd).unwrap();
        assert!(json.contains("\"name\":\"Test\""));
        assert!(json.contains("\"description\":\"Desc\""));
    }

    #[test]
    fn name_description_request_serialize_empty() {
        let nd = NameDescriptionRequest::default();
        let json = serde_json::to_string(&nd).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn name_description_request_builder() {
        let nd = NameDescriptionRequest::name("Test")
            .with_description("Desc");
        assert_eq!(nd.name.as_deref(), Some("Test"));
        assert_eq!(nd.description.as_deref(), Some("Desc"));
    }

    // ==================== Flatten Integration Tests ====================

    #[test]
    fn flatten_entity_meta_with_other_fields() {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TestEntity {
            id: String,
            #[serde(flatten)]
            meta: EntityMeta,
            custom_field: Option<String>,
        }

        let json = r#"{
            "id": "test123",
            "created": "2024-01-01 00:00:00.0000",
            "modifier": "t1_lgn_12345678901234567890123",
            "customField": "custom value"
        }"#;

        let entity: TestEntity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id, "test123");
        assert_eq!(entity.meta.created.as_deref(), Some("2024-01-01 00:00:00.0000"));
        assert!(entity.meta.modified.is_none());
        assert!(entity.meta.creator.is_none());
        assert!(entity.meta.modifier.is_some());
        assert_eq!(entity.custom_field.as_deref(), Some("custom value"));
    }

    #[test]
    fn flatten_status_flags_with_other_fields() {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TestEntity {
            id: String,
            #[serde(flatten)]
            status: StatusFlags,
        }

        let json = r#"{"id": "test123", "inactive": 1, "frozen": 0}"#;
        let entity: TestEntity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.id, "test123");
        assert!(entity.status.inactive);
        assert!(!entity.status.frozen);
    }

    #[test]
    fn flatten_status_flags_request_serialize() {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct UpdateRequest {
            name: Option<String>,
            #[serde(flatten)]
            status: StatusFlagsRequest,
        }

        let request = UpdateRequest {
            name: Some("Updated".to_string()),
            status: StatusFlagsRequest::inactive(true),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"name\":\"Updated\""));
        assert!(json.contains("\"inactive\":1"));
        assert!(!json.contains("frozen"));
    }
}
