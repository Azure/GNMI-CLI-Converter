//! Integration tests for watermark formatters.
//!
//! Tests buffer pool, headroom pool, queue watermark, and priority-group
//! watermark commands using mock data derived from sonic-utilities test vectors.
//! Each test asserts the converter output is identical to the raw expected
//! string from sonic-utilities.
//!
//! Source: sonic-utilities/tests/wm_input/wm_test_vectors.py
//!
//! ## Formatter coverage
//! - `buffer_pool` (BufferPoolFormatter)
//! - `headroom_pool` (HeadroomPoolFormatter)
//! - `queue_watermark` (QueueWatermarkFormatter)
//! - `watermark_telemetry` (WatermarkTelemetryFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_watermark::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Buffer pool watermark tests
// Source: wm_test_vectors.py show_buffer_pool_*
// =============================================================================

// #01 - show buffer_pool watermark
#[test]
fn test_show_buffer_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_BUFFER_POOL_WM).unwrap();
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_BUFFER_POOL_WM_OUTPUT, "buffer_pool_watermark");
}

// #02 - show buffer_pool persistent-watermark
#[test]
fn test_show_buffer_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_BUFFER_POOL_PWM).unwrap();
    let result = converter.convert("buffer_pool_persistent_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_BUFFER_POOL_PERSISTENT_WM_OUTPUT, "buffer_pool_persistent_watermark");
}

// #01 - show buffer_pool watermark (single pool)
#[test]
fn test_show_buffer_pool_watermark_single_pool() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_BUFFER_POOL_SINGLE).unwrap();
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_BUFFER_POOL_SINGLE_OUTPUT, "buffer_pool_watermark single pool");
}

// #01 - show buffer_pool watermark (empty)
#[test]
fn test_show_buffer_pool_watermark_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_BUFFER_POOL_EMPTY).unwrap();
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_BUFFER_POOL_EMPTY_OUTPUT, "buffer_pool_watermark empty");
}

// =============================================================================
// Headroom pool watermark tests
// Source: wm_test_vectors.py show_hdrm_pool_*
// =============================================================================

// #07 - show headroom-pool watermark
#[test]
fn test_show_headroom_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_HDRM_POOL_WM).unwrap();
    let result = converter.convert("headroom_pool_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_HDRM_POOL_WM_OUTPUT, "headroom_pool_watermark");
}

// #08 - show headroom-pool persistent-watermark
#[test]
fn test_show_headroom_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_HDRM_POOL_PWM).unwrap();
    let result = converter.convert("headroom_pool_persistent_watermark", data).unwrap();
    assert_exact_match(&result, SHOW_HDRM_POOL_PERSISTENT_WM_OUTPUT, "headroom_pool_persistent_watermark");
}

// =============================================================================
// Queue watermark tests
// Source: wm_test_vectors.py show_queue_wm_*
// =============================================================================

// #61 - show queue watermark unicast
#[test]
fn test_show_queue_wm_unicast() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_WM_UNICAST).unwrap();
    let result = converter.convert("queue_watermark_unicast", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_WM_UNICAST_OUTPUT, "queue_watermark_unicast");
}

// #57 - show queue persistent-watermark unicast
#[test]
fn test_show_queue_pwm_unicast() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_PWM_UNICAST).unwrap();
    let result = converter.convert("queue_persistent_watermark_unicast", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_PWM_UNICAST_OUTPUT, "queue_persistent_watermark_unicast");
}

// #60 - show queue watermark multicast
#[test]
fn test_show_queue_wm_multicast() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_WM_MULTICAST).unwrap();
    let result = converter.convert("queue_watermark_multicast", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_WM_MULTICAST_OUTPUT, "queue_watermark_multicast");
}

// #59 - show queue watermark all
#[test]
fn test_show_queue_wm_all() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_WM_ALL).unwrap();
    let result = converter.convert("queue_watermark_all", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_WM_ALL_OUTPUT, "queue_watermark_all");
}

// #55 - show queue persistent-watermark all
#[test]
fn test_show_queue_pwm_all() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_QUEUE_PWM_ALL).unwrap();
    let result = converter.convert("queue_persistent_watermark_all", data).unwrap();
    assert_exact_match(&result, SHOW_QUEUE_PWM_ALL_OUTPUT, "queue_persistent_watermark_all");
}

// =============================================================================
// Priority group watermark tests
// Source: wm_test_vectors.py show_pg_wm_*
// =============================================================================

// #58 - show queue watermark (priority-group shared)
#[test]
fn test_show_priority_group_watermark_shared() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PG_WM_SHARED).unwrap();
    let result = converter.convert("priority_group_watermark_shared", data).unwrap();
    assert_exact_match(&result, SHOW_PG_WM_SHARED_OUTPUT, "priority_group_watermark_shared");
}

// #58 - show queue watermark (priority-group headroom)
#[test]
fn test_show_priority_group_watermark_headroom() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PG_WM_HDRM).unwrap();
    let result = converter.convert("priority_group_watermark_headroom", data).unwrap();
    assert_exact_match(&result, SHOW_PG_WM_HDRM_OUTPUT, "priority_group_watermark_headroom");
}

// #54 - show queue persistent-watermark (priority-group shared)
#[test]
fn test_show_priority_group_persistent_watermark_shared() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PG_PWM_SHARED).unwrap();
    let result = converter.convert("priority_group_persistent_watermark_shared", data).unwrap();
    assert_exact_match(&result, SHOW_PG_PERSISTENT_WM_SHARED_OUTPUT, "priority_group_persistent_watermark_shared");
}

// #54 - show queue persistent-watermark (priority-group headroom)
#[test]
fn test_show_priority_group_persistent_watermark_headroom() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PG_PWM_HDRM).unwrap();
    let result = converter.convert("priority_group_persistent_watermark_headroom", data).unwrap();
    assert_exact_match(&result, SHOW_PG_PERSISTENT_WM_HDRM_OUTPUT, "priority_group_persistent_watermark_headroom");
}

// =============================================================================
// Watermark telemetry interval tests
// =============================================================================

// #70 - show watermark telemetry interval
#[test]
fn test_show_watermark_telemetry() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_WATERMARK_TELEMETRY).unwrap();
    let result = converter.convert("watermark_telemetry", data).unwrap();
    assert_exact_match(&result, SHOW_WATERMARK_TELEMETRY_OUTPUT, "watermark_telemetry");
}

// #70 - show watermark telemetry interval (with suffix)
#[test]
fn test_show_watermark_telemetry_with_suffix() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_WATERMARK_TELEMETRY_SUFFIX).unwrap();
    let result = converter.convert("watermark_telemetry", data).unwrap();
    assert_exact_match(&result, SHOW_WATERMARK_TELEMETRY_SUFFIX_OUTPUT, "watermark_telemetry with suffix");
}

// =============================================================================
// Cross-formatter validation tests
// =============================================================================

// #01/#07 - cross: buffer_pool vs headroom-pool titles
#[test]
fn test_buffer_pool_and_headroom_pool_different_titles() {
    let converter = GnmiToCliConverter::new();
    let bp_data: serde_json::Value = serde_json::from_str(MOCK_BUFFER_POOL_WM).unwrap();
    let hp_data: serde_json::Value = serde_json::from_str(MOCK_HDRM_POOL_WM).unwrap();
    let bp_result = converter.convert("buffer_pool_watermark", bp_data).unwrap();
    let hp_result = converter.convert("headroom_pool_watermark", hp_data).unwrap();
    assert!(bp_result.contains("Shared pool"), "Buffer pool should have 'Shared pool' title");
    assert!(hp_result.contains("Headroom pool"), "Headroom pool should have 'Headroom pool' title");
}

// registration test
#[test]
fn test_all_watermark_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "buffer_pool_watermark",
        "buffer_pool_persistent_watermark",
        "headroom_pool_watermark",
        "headroom_pool_persistent_watermark",
        "queue_watermark_unicast",
        "queue_watermark_multicast",
        "queue_watermark_all",
        "queue_persistent_watermark_unicast",
        "queue_persistent_watermark_multicast",
        "queue_persistent_watermark_all",
        "watermark_telemetry",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing watermark command: {}", cmd);
    }
}
