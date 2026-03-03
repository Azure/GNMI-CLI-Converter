//! Integration tests for queue counter formatters.
//!
//! Tests queue counters, trim counters, and WRED counter commands
//! using mock data derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/queuestat_input/assert_show_output.py
//!
//! ## Formatter coverage
//! - `queue_counters` (QueueCountersFormatter)
//! - `wred_counters` (WredCountersFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_queue::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Queue counters tests
// Source: queuestat_input/assert_show_output.py counters_all
// =============================================================================

// #53 - show queue counters
#[test]
fn test_show_queue_counters_ethernet0() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_COUNTERS_ETH0).unwrap();
    let result = converter.convert("queue_counters", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_COUNTERS_ETH0_OUTPUT, "queue_counters ethernet0");
}

// #53 - show queue counters (simple)
#[test]
fn test_show_queue_counters_simple() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_COUNTERS_SIMPLE).unwrap();
    let result = converter.convert("queue_counters", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_COUNTERS_SIMPLE_OUTPUT, "queue_counters simple");
}

// #53 - show queue counters (empty)
#[test]
fn test_show_queue_counters_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_COUNTERS_EMPTY).unwrap();
    let result = converter.convert("queue_counters", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_COUNTERS_EMPTY_OUTPUT, "queue_counters empty");
}

// =============================================================================
// Trim counters tests
// =============================================================================

// #53 - show queue counters (trim)
#[test]
fn test_show_queue_counters_trim() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRIM_COUNTERS_ETH0).unwrap();
    let result = converter.convert("queue_counters", data).unwrap();
    assert_exact_match(&result, SHOW_TRIM_COUNTERS_ETH0_OUTPUT, "queue_counters trim");
}

// =============================================================================
// WRED counters tests
// =============================================================================

// #62 - show queue wredcounters
#[test]
fn test_show_queue_wredcounters() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_WRED_COUNTERS).unwrap();
    let result = converter.convert("queue_wredcounters", data).unwrap();
    assert_exact_match(&result, SHOW_WRED_COUNTERS_OUTPUT, "queue_wredcounters");
}

// =============================================================================
// Priority group counters tests
// =============================================================================

// #53 - show queue counters (priority-group)
#[test]
fn test_show_priority_group_counters() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PG_COUNTERS).unwrap();
    let result = converter.convert("queue_counters", data).unwrap();
    assert_exact_match(&result, SHOW_PG_COUNTERS_OUTPUT, "priority_group_counters");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_queue_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "queue_counters",
        "queue_wredcounters",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing queue command: {}", cmd);
    }
}

// registration test
#[test]
fn test_queue_watermark_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "queue_watermark_unicast",
        "queue_watermark_multicast",
        "queue_watermark_all",
        "queue_persistent_watermark_unicast",
        "queue_persistent_watermark_multicast",
        "queue_persistent_watermark_all",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing queue watermark command: {}", cmd);
    }
}
