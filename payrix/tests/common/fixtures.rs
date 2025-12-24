//! Fixture loading utilities for offline testing.
//!
//! This module provides utilities to load mock data from JSON files for testing
//! without requiring API access.

use serde::de::DeserializeOwned;
use std::path::Path;

/// Load fixture data from a JSON file.
///
/// Expects the file to have the standard Payrix API response structure:
/// ```json
/// {
///   "response": {
///     "data": [...],
///     "details": { ... }
///   }
/// }
/// ```
///
/// # Example
/// ```ignore
/// let chargebacks: Vec<Chargeback> = load_fixture("chargebacks");
/// ```
pub fn load_fixture<T: DeserializeOwned>(name: &str) -> Vec<T> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("mock_data")
        .join(format!("{}.json", name));

    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture '{}' from {:?}: {}", name, path, e));

    let wrapper: serde_json::Value = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse fixture '{}': {}", name, e));

    let data = wrapper
        .get("response")
        .and_then(|r| r.get("data"))
        .unwrap_or_else(|| panic!("Fixture '{}' missing response.data field", name))
        .clone();

    serde_json::from_value(data)
        .unwrap_or_else(|e| panic!("Failed to deserialize fixture '{}': {}", name, e))
}

/// Load a single item from a fixture file.
///
/// Returns the first item from the fixture data array.
///
/// # Panics
/// Panics if the fixture is empty.
pub fn load_single_fixture<T: DeserializeOwned>(name: &str) -> T {
    load_fixture::<T>(name)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("Fixture '{}' should have at least one item", name))
}

/// Load fixture data as raw JSON Values.
///
/// Useful when you need to inspect the raw structure without deserializing.
pub fn load_fixture_raw(name: &str) -> Vec<serde_json::Value> {
    load_fixture(name)
}

/// Check if a fixture file exists.
pub fn fixture_exists(name: &str) -> bool {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("mock_data")
        .join(format!("{}.json", name));
    path.exists()
}

/// List all available fixtures.
pub fn list_fixtures() -> Vec<String> {
    let mock_data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("mock_data");

    if !mock_data_dir.exists() {
        return Vec::new();
    }

    std::fs::read_dir(&mock_data_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let path = e.path();
                    if path.extension().map(|ext| ext == "json").unwrap_or(false) {
                        path.file_stem()
                            .and_then(|s| s.to_str())
                            .map(String::from)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_fixtures() {
        let fixtures = list_fixtures();
        println!("Available fixtures: {:?}", fixtures);
        // Should have at least the chargeback fixtures
        assert!(fixtures.iter().any(|f| f == "chargebacks"));
    }

    #[test]
    fn test_fixture_exists() {
        assert!(fixture_exists("chargebacks"));
        assert!(!fixture_exists("nonexistent_fixture"));
    }

    #[test]
    fn test_load_fixture_raw() {
        let data = load_fixture_raw("chargebacks");
        assert!(!data.is_empty(), "Should have chargeback data");

        // Check first item has expected fields
        let first = &data[0];
        assert!(first.get("id").is_some(), "Should have id field");
        assert!(first.get("status").is_some(), "Should have status field");
    }

    #[test]
    fn test_load_chargeback_fixture() {
        use payrix::Chargeback;

        let chargebacks: Vec<Chargeback> = load_fixture("chargebacks");
        assert!(!chargebacks.is_empty(), "Should have chargebacks");

        let cb = &chargebacks[0];
        assert!(cb.id.as_str().starts_with("t1_chb_"), "Should have valid ID prefix");
        assert!(cb.status.is_some(), "Should have status");
    }

    #[test]
    fn test_load_single_fixture() {
        use payrix::Chargeback;

        let cb: Chargeback = load_single_fixture("chargebacks");
        assert!(cb.id.as_str().starts_with("t1_chb_"), "Should have valid ID prefix");
    }
}
