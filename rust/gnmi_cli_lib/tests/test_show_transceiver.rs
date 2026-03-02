//! Unit tests for transceiver-related show commands.
//!
//! Covers: interfaces transceiver eeprom, error-status, info, lpmode, pm, presence, status.
//!
//! Formatter modules: transceiver (TransceiverEepromFormatter, TransceiverErrorStatusFormatter,
//! TransceiverInfoFormatter, TransceiverLpmodeFormatter, TransceiverPmFormatter,
//! TransceiverPresenceFormatter, TransceiverStatusFormatter)

mod common;

use common::{assert_cli_output_matches, assert_output_contains_all, load_verifying_test_data};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 28: show interfaces transceiver eeprom
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_eeprom() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(28, "interfaces_transceiver_eeprom");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_eeprom", data)
                .unwrap();
            if result.trim().is_empty() { continue; }
            assert!(result.contains("Ethernet") || result.contains("EEPROM"),
                "Transceiver eeprom output should contain Ethernet or EEPROM");
        }
    }
}

// =============================================================================
// 29: show interfaces transceiver error-status
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_error_status() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(29, "interfaces_transceiver_error_status");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_error_status", data)
                .unwrap();
            if result.trim().is_empty() { continue; }
            assert_output_contains_all(&result, &["Port", "Error Status"]);
        }
    }
}

// =============================================================================
// 30: show interfaces transceiver info
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_info() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(30, "interfaces_transceiver_info");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_info", data)
                .unwrap();
            if result.trim().is_empty() { continue; }
            assert!(result.contains("Ethernet"),
                "Transceiver info output should contain Ethernet");
        }
    }
}

// =============================================================================
// 31: show interfaces transceiver lpmode
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_lpmode() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(31, "interfaces_transceiver_lpmode");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_lpmode", data)
                .unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

// =============================================================================
// 32: show interfaces transceiver pm
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_pm() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(32, "interfaces_transceiver_pm");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_pm", data)
                .unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

// =============================================================================
// 33: show interfaces transceiver presence
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_presence() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(33, "interfaces_transceiver_presence");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_presence", data)
                .unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

// =============================================================================
// 34: show interfaces transceiver status
// =============================================================================

#[test]
fn test_show_interfaces_transceiver_status() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(34, "interfaces_transceiver_status");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_transceiver_status", data)
                .unwrap();
            if result.trim().is_empty() { continue; }
            assert!(result.contains("Ethernet"),
                "Transceiver status output should contain Ethernet");
        }
    }
}
