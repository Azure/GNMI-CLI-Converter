//! Shared test utilities for integration tests.
//!
//! Provides helper functions for loading test data, normalizing output,
//! and comparing results across integration test files.

use serde_json::Value;
use std::path::Path;

/// Load test JSON data from a string.
pub fn load_test_data(json_str: &str) -> Value {
    serde_json::from_str(json_str).expect("Failed to parse test JSON data")
}

/// Load test JSON data from a file path relative to the project root.
#[allow(dead_code)]
pub fn load_test_data_file(path: &Path) -> Value {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read test data file {:?}: {}", path, e));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse test data file {:?}: {}", path, e))
}

/// Normalize output for comparison by trimming trailing whitespace from each line
/// and removing trailing blank lines.
pub fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

/// Compare two outputs after normalization.
#[allow(dead_code)]
pub fn compare_outputs(actual: &str, expected: &str) -> bool {
    normalize_output(actual) == normalize_output(expected)
}

/// Assert that the output contains all expected substrings.
pub fn assert_output_contains_all(output: &str, expected_substrings: &[&str]) {
    for s in expected_substrings {
        assert!(
            output.contains(s),
            "Output missing expected substring: '{}'\nFull output:\n{}",
            s,
            output
        );
    }
}

/// Assert that the output does NOT contain any of the given substrings.
#[allow(dead_code)]
pub fn assert_output_excludes_all(output: &str, excluded_substrings: &[&str]) {
    for s in excluded_substrings {
        assert!(
            !output.contains(s),
            "Output unexpectedly contains: '{}'\nFull output:\n{}",
            s,
            output
        );
    }
}
