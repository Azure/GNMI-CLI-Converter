//! Integration tests for drop counter formatters.
//!
//! Tests dropcounters capabilities, configuration, and counts commands
//! using mock data derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/single_asic_dropstat_test.py
//!
//! ## Formatter coverage
//! - `dropcounters_capabilities` (DropCountersCapabilitiesFormatter)
//! - `dropcounters_configuration` (DropCountersConfigFormatter)
//! - `dropcounters_counts` (DropCountersCountsFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_dropcounters::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Drop counters capabilities tests
// =============================================================================

// #05 - show dropcounters capabilities
#[test]
fn test_show_dropcounters_capabilities() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_DROPCOUNTERS_CAPABILITIES).unwrap();
    let result = converter.convert("dropcounters_capabilities", data).unwrap();
    assert_exact_match(&result, SHOW_DROPCOUNTERS_CAPABILITIES_OUTPUT, "dropcounters_capabilities");
}

// #05 - show dropcounters capabilities (configuration)
#[test]
fn test_show_dropcounters_configuration() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_DROPCOUNTERS_CONFIG).unwrap();
    let result = converter.convert("dropcounters_configuration", data).unwrap();
    assert_exact_match(&result, SHOW_DROPCOUNTERS_CONFIG_OUTPUT, "dropcounters_configuration");
}

// #06 - show dropcounters counts
#[test]
fn test_show_dropcounters_counts() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_DROPCOUNTERS_COUNTS).unwrap();
    let result = converter.convert("dropcounters_counts", data).unwrap();
    assert_exact_match(&result, SHOW_DROPCOUNTERS_COUNTS_OUTPUT, "dropcounters_counts");
}

// #06 - show dropcounters counts (empty)
#[test]
fn test_show_dropcounters_counts_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_DROPCOUNTERS_COUNTS_EMPTY).unwrap();
    let result = converter.convert("dropcounters_counts", data).unwrap();
    assert_exact_match(&result, SHOW_DROPCOUNTERS_COUNTS_EMPTY_OUTPUT, "dropcounters_counts empty");
}

// #05 - show dropcounters capabilities (drop reasons)
#[test]
fn test_show_drop_reasons() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_DROP_REASONS).unwrap();
    let result = converter.convert("dropcounters_capabilities", data).unwrap();
    assert_exact_match(&result, SHOW_DROP_REASONS_OUTPUT, "drop_reasons");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_dropcounters_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "dropcounters_capabilities",
        "dropcounters_configuration",
        "dropcounters_counts",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing dropcounters command: {}", cmd);
    }
}
