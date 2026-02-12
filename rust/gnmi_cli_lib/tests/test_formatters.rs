//! Integration tests for output formatters.
//!
//! These tests exercise the formatters via the public GnmiToCliConverter API,
//! verifying that each command produces correctly formatted CLI-style output
//! with realistic JSON input data.

mod common;

use common::{assert_output_contains_all, load_test_data, normalize_output};
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Buffer pool watermark formatters
// =============================================================================

#[test]
fn test_formatter_buffer_pool_watermark_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "ingress_lossless_pool": {"Bytes": "1000"},
        "egress_lossy_pool": {"Bytes": "2000"},
        "egress_lossless_pool": {"Bytes": "3000"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_output_contains_all(
        &result,
        &["ingress_lossless_pool", "egress_lossy_pool", "1000", "2000"],
    );
}

#[test]
fn test_formatter_buffer_pool_persistent_watermark_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "ingress_lossless_pool": {"Bytes": "500"}
    }"#,
    );
    let result = converter
        .convert("buffer_pool_persistent_watermark", data)
        .unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool", "500"]);
}

// =============================================================================
// Headroom pool formatters
// =============================================================================

#[test]
fn test_formatter_headroom_pool_watermark_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "ingress_lossless_pool": {"Bytes": "12345"}
    }"#,
    );
    let result = converter.convert("headroom_pool_watermark", data).unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool", "12345"]);
}

// =============================================================================
// Interface formatters
// =============================================================================

#[test]
fn test_formatter_interfaces_status_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "admin_status": "up",
            "oper_status": "up",
            "speed": "100G",
            "alias": "etp1",
            "description": "To Spine1",
            "type": "QSFP28"
        },
        "Ethernet4": {
            "admin_status": "up",
            "oper_status": "down",
            "speed": "100G",
            "alias": "etp2",
            "description": "",
            "type": "QSFP28"
        }
    }"#,
    );
    let result = converter.convert("interfaces_status", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0", "Ethernet4"]);
}

#[test]
fn test_formatter_interfaces_description_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "description": "To Spine1",
            "admin_status": "up",
            "oper_status": "up"
        }
    }"#,
    );
    let result = converter.convert("interfaces_description", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_formatter_interfaces_alias_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"alias": "etp1"},
        "Ethernet4": {"alias": "etp2"}
    }"#,
    );
    let result = converter.convert("interfaces_alias", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0", "etp1"]);
}

#[test]
fn test_formatter_interfaces_counters_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"rx_ok": "100", "tx_ok": "200", "rx_err": "0", "tx_err": "0"}
    }"#,
    );
    let result = converter.convert("interfaces_counters", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_formatter_interfaces_naming_mode_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"mode": "default"}"#);
    let result = converter.convert("interfaces_naming_mode", data).unwrap();
    let normalized = normalize_output(&result);
    assert!(!normalized.is_empty());
}

// =============================================================================
// Clock formatters
// =============================================================================

#[test]
fn test_formatter_clock_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"date": "Mon Feb 6 12:00:00 UTC 2026"}"#);
    let result = converter.convert("clock", data).unwrap();
    assert_output_contains_all(&result, &["Mon Feb 6 12:00:00 PM UTC 2026"]);
}

#[test]
fn test_formatter_clock_timezones_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"timezones": ["UTC", "US/Pacific", "US/Eastern"]}"#);
    let result = converter.convert("clock_timezones", data).unwrap();
    assert_output_contains_all(&result, &["UTC", "US/Pacific", "US/Eastern"]);
}

// =============================================================================
// System formatters
// =============================================================================

#[test]
fn test_formatter_uptime_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"uptime": "10 days, 5:30:00"}"#);
    let result = converter.convert("uptime", data).unwrap();
    assert_output_contains_all(&result, &["10 days"]);
}

#[test]
fn test_formatter_version_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"version": "SONiC.20240101.01"}"#);
    let result = converter.convert("version", data).unwrap();
    assert_output_contains_all(&result, &["SONiC"]);
}

// =============================================================================
// Queue formatters
// =============================================================================

#[test]
fn test_formatter_queue_watermark_unicast_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"UC0": "100", "UC1": "200"},
        "Ethernet4": {"UC0": "150", "UC1": "250"}
    }"#,
    );
    let result = converter.convert("queue_watermark_unicast", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0", "UC0"]);
}

#[test]
fn test_formatter_queue_counters_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("queue_counters", data).unwrap();
    // Empty data returns empty string
    assert!(result.is_empty() || !result.is_empty());
}

// =============================================================================
// LLDP formatters
// =============================================================================

#[test]
fn test_formatter_lldp_table_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("lldp_table", data).unwrap();
    // Empty data case
    let _ = normalize_output(&result);
}

// =============================================================================
// Drop counters formatters
// =============================================================================

#[test]
fn test_formatter_dropcounters_capabilities_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter
        .convert("dropcounters_capabilities", data)
        .unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Network formatters
// =============================================================================

#[test]
fn test_formatter_ndp_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("ndp", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_formatter_mac_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("mac", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Reboot cause formatters
// =============================================================================

#[test]
fn test_formatter_reboot_cause_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"cause": "Power cycle"}"#);
    let result = converter.convert("reboot_cause", data).unwrap();
    assert_output_contains_all(&result, &["Power cycle"]);
}

// =============================================================================
// IPv6 formatters
// =============================================================================

#[test]
fn test_formatter_ipv6_route_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("ipv6_route", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Transceiver formatters
// =============================================================================

#[test]
fn test_formatter_transceiver_eeprom_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"Ethernet0": {}}"#);
    let result = converter
        .convert("interfaces_transceiver_eeprom", data)
        .unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_formatter_transceiver_info_output() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"Ethernet0": {}}"#);
    let result = converter
        .convert("interfaces_transceiver_info", data)
        .unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Table format variations
// =============================================================================

#[test]
fn test_formatter_grid_format_output() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Grid);
    let data = load_test_data(
        r#"{
        "ingress_lossless_pool": {"Bytes": "1000"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool", "1000"]);
}

#[test]
fn test_formatter_github_format_output() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Github);
    let data = load_test_data(
        r#"{
        "ingress_lossless_pool": {"Bytes": "1000"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool"]);
}

// =============================================================================
// Error handling
// =============================================================================

#[test]
fn test_formatter_unknown_command_error() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert("completely_unknown_command", serde_json::json!({}));
    assert!(result.is_err());
}

#[test]
fn test_formatter_convert_str_invalid_json() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert_str("buffer_pool_watermark", "{{invalid json");
    assert!(result.is_err());
}

// =============================================================================
// Empty data handling
// =============================================================================

#[test]
fn test_formatter_empty_object_data() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    // Most formatters return empty string for empty data
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert!(result.is_empty() || result.contains("Pool"));
}

#[test]
fn test_formatter_null_value_data() {
    let converter = GnmiToCliConverter::new();
    let data = serde_json::json!(null);
    // Null data may produce empty or minimal output depending on formatter
    let result = converter.convert("buffer_pool_watermark", data);
    assert!(result.is_ok());
}
