//! Search helpers for building Payrix query parameters.
//!
//! Payrix uses a specific search syntax documented at:
//! <https://resource.payrix.com/resources/api-call-syntax>

use chrono::{Datelike, NaiveDate};

/// Operators for search field comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOperator {
    /// Exact match (default)
    Equals,
    /// Exact string match
    Exact,
    /// Greater than
    Greater,
    /// Less than
    Less,
    /// Pattern match (use % as wildcard)
    Like,
    /// Value in list
    In,
    /// Sort by field
    Sort,
    /// Not equal
    Diff,
    /// Not like pattern
    NotLike,
    /// Value not in list
    NotIn,
}

impl SearchOperator {
    /// Get the Payrix query parameter name.
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchOperator::Equals => "equals",
            SearchOperator::Exact => "exact",
            SearchOperator::Greater => "greater",
            SearchOperator::Less => "less",
            SearchOperator::Like => "like",
            SearchOperator::In => "in",
            SearchOperator::Sort => "sort",
            SearchOperator::Diff => "diff",
            SearchOperator::NotLike => "notlike",
            SearchOperator::NotIn => "notin",
        }
    }
}

/// Create a search field for Payrix queries.
///
/// # Examples
///
/// ```
/// use payrix::search::{make_search_field, SearchOperator};
///
/// // Simple equality
/// let search = make_search_field("merchant", "mer_123", None);
/// assert_eq!(search, "merchant[equals]=mer_123");
///
/// // Greater than
/// let search = make_search_field("created", "20240101", Some(SearchOperator::Greater));
/// assert_eq!(search, "created[greater]=20240101");
///
/// // In list
/// let search = make_search_field("id", "id1,id2,id3", Some(SearchOperator::In));
/// assert_eq!(search, "id[in]=id1,id2,id3");
/// ```
pub fn make_search_field(field: &str, value: &str, operator: Option<SearchOperator>) -> String {
    let op = operator.unwrap_or(SearchOperator::Equals);
    format!("{}[{}]={}", field, op.as_str(), value)
}

/// Create a search field with multiple values (for IN/NOTIN operators).
///
/// # Examples
///
/// ```
/// use payrix::search::{make_search_field_multi, SearchOperator};
///
/// let ids = vec!["id1", "id2", "id3"];
/// let search = make_search_field_multi("id", &ids, SearchOperator::In);
/// assert_eq!(search, "id[in]=id1,id2,id3");
/// ```
pub fn make_search_field_multi(field: &str, values: &[&str], operator: SearchOperator) -> String {
    let joined = values.join(",");
    format!("{}[{}]={}", field, operator.as_str(), joined)
}

/// Format a date as Payrix expects: YYYYMMDD.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use payrix::search::make_payrix_date;
///
/// let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
/// assert_eq!(make_payrix_date(&date), "20240315");
/// ```
pub fn make_payrix_date(date: &NaiveDate) -> String {
    format!("{:04}{:02}{:02}", date.year(), date.month(), date.day())
}

/// Parse a Payrix date (YYYYMMDD) into a NaiveDate.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use payrix::search::parse_payrix_date;
///
/// let date = parse_payrix_date("20240315").unwrap();
/// assert_eq!(date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
/// ```
pub fn parse_payrix_date(date_str: &str) -> Option<NaiveDate> {
    if date_str.len() != 8 {
        return None;
    }
    let year = date_str[0..4].parse().ok()?;
    let month = date_str[4..6].parse().ok()?;
    let day = date_str[6..8].parse().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

/// Build expand query parameters for related resources.
///
/// # Examples
///
/// ```
/// use payrix::search::build_expand_query;
///
/// // Simple expansion
/// let query = build_expand_query(&["token", "customer"]);
/// assert_eq!(query, "expand[token][]&expand[customer][]");
///
/// // Nested expansion (token.customer)
/// let query = build_expand_query(&["token|customer"]);
/// assert_eq!(query, "expand[token][][customer][]");
/// ```
pub fn build_expand_query(expand: &[&str]) -> String {
    expand
        .iter()
        .map(|e| {
            if e.contains('|') {
                // Handle nested expansion like "token|customer"
                let parts: Vec<&str> = e.split('|').collect();
                format!(
                    "expand{}",
                    parts.iter().map(|p| format!("[{}][]", p)).collect::<String>()
                )
            } else {
                format!("expand[{}][]", e)
            }
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Search builder for constructing complex queries.
///
/// # Examples
///
/// ```
/// use payrix::search::{SearchBuilder, SearchOperator};
///
/// let search = SearchBuilder::new()
///     .field("merchant", "mer_123")
///     .field_with_op("created", "20240101", SearchOperator::Greater)
///     .field_with_op("status", "1", SearchOperator::Equals)
///     .build();
///
/// assert_eq!(search, "merchant[equals]=mer_123&created[greater]=20240101&status[equals]=1");
/// ```
#[derive(Debug, Clone, Default)]
pub struct SearchBuilder {
    parts: Vec<String>,
}

impl SearchBuilder {
    /// Create a new search builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a field with equals operator.
    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.parts.push(make_search_field(name, value, None));
        self
    }

    /// Add a field with a specific operator.
    pub fn field_with_op(mut self, name: &str, value: &str, operator: SearchOperator) -> Self {
        self.parts
            .push(make_search_field(name, value, Some(operator)));
        self
    }

    /// Add a field with multiple values.
    pub fn field_multi(mut self, name: &str, values: &[&str], operator: SearchOperator) -> Self {
        self.parts
            .push(make_search_field_multi(name, values, operator));
        self
    }

    /// Add a raw search string.
    pub fn raw(mut self, search: &str) -> Self {
        self.parts.push(search.to_string());
        self
    }

    /// Build the final search string.
    pub fn build(self) -> String {
        self.parts.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_search_field() {
        assert_eq!(
            make_search_field("merchant", "mer_123", None),
            "merchant[equals]=mer_123"
        );
        assert_eq!(
            make_search_field("created", "20240101", Some(SearchOperator::Greater)),
            "created[greater]=20240101"
        );
        assert_eq!(
            make_search_field("name", "%test%", Some(SearchOperator::Like)),
            "name[like]=%test%"
        );
    }

    #[test]
    fn test_make_search_field_multi() {
        assert_eq!(
            make_search_field_multi("id", &["a", "b", "c"], SearchOperator::In),
            "id[in]=a,b,c"
        );
    }

    #[test]
    fn test_payrix_date() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        assert_eq!(make_payrix_date(&date), "20240315");
        assert_eq!(parse_payrix_date("20240315"), Some(date));
        assert_eq!(parse_payrix_date("invalid"), None);
    }

    #[test]
    fn test_build_expand_query() {
        assert_eq!(
            build_expand_query(&["token", "customer"]),
            "expand[token][]&expand[customer][]"
        );
        assert_eq!(
            build_expand_query(&["token|customer"]),
            "expand[token][][customer][]"
        );
    }

    #[test]
    fn test_search_builder() {
        let search = SearchBuilder::new()
            .field("merchant", "mer_123")
            .field_with_op("created", "20240101", SearchOperator::Greater)
            .build();
        assert_eq!(
            search,
            "merchant[equals]=mer_123&created[greater]=20240101"
        );
    }
}
