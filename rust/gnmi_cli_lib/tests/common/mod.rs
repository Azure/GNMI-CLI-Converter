//! Shared test utilities for integration tests.
//!
//! Provides helper functions for loading test data, normalizing output,
//! and comparing results across integration test files.

pub mod ipv6_expected;

use serde_json::Value;
use std::path::PathBuf;

/// Load test JSON data from a string.
#[allow(dead_code)]
pub fn load_test_data(json_str: &str) -> Value {
    serde_json::from_str(json_str).expect("Failed to parse test JSON data")
}

/// Normalize output for comparison by trimming trailing whitespace from each line
/// and removing trailing blank lines.
#[allow(dead_code)]
pub fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

/// Assert that the output contains all expected substrings.
#[allow(dead_code)]
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

/// Load test data from the verifying test_data directory by index.
/// Returns (json_string, cli_string) pairs from devices with valid data.
#[allow(dead_code)]
pub fn load_verifying_test_data(index: u32, command_suffix: &str) -> Vec<(String, String)> {
    let filename = format!("data_{:02}_show_{}.json", index, command_suffix);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("gnmi_cli_lib_verifying")
        .join("test_data")
        .join(&filename);

    if !path.exists() {
        return vec![];
    }

    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", filename, e));
    let data: Value = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", filename, e));

    let mut results = vec![];
    if let Some(test_cases) = data["test_cases"].as_array() {
        for tc in test_cases {
            if let Some(devices) = tc["devices"].as_object() {
                for (_device_name, device_data) in devices {
                    let json_str = device_data["json_string"].as_str();
                    let cli_str = device_data["cli_string"].as_str();
                    if let (Some(js), Some(cs)) = (json_str, cli_str) {
                        if !js.is_empty() && !cs.is_empty() {
                            results.push((js.to_string(), cs.to_string()));
                        }
                    }
                }
            }
        }
    }
    results
}

/// Assert that the converter output matches the expected CLI output from
/// the verifying test data (using normalized comparison).
#[allow(dead_code)]
pub fn assert_cli_output_matches(actual: &str, expected: &str) {
    let actual_norm = normalize_output(actual);
    let expected_norm = normalize_output(expected);
    assert_eq!(
        actual_norm, expected_norm,
        "\n\nActual output:\n{}\n\nExpected output:\n{}\n",
        actual_norm, expected_norm
    );
}
