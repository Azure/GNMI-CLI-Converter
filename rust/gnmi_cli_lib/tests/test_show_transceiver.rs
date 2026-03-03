//! Integration tests for transceiver formatters.
//!
//! Tests transceiver eeprom, presence, status, lpmode, error-status, pm,
//! and info commands using mock data derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/sfp_test.py
//!
//! ## Formatter coverage
//! - `interfaces_transceiver_eeprom` (TransceiverEepromFormatter)
//! - `interfaces_transceiver_presence` (TransceiverPresenceFormatter)
//! - `interfaces_transceiver_status` (TransceiverStatusFormatter)
//! - `interfaces_transceiver_lpmode` (TransceiverLpmodeFormatter)
//! - `interfaces_transceiver_error_status` (TransceiverErrorStatusFormatter)
//! - `interfaces_transceiver_pm` (TransceiverPmFormatter)
//! - `interfaces_transceiver_info` (TransceiverInfoFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_transceiver::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Transceiver EEPROM tests
// Source: sfp_test.py test_sfp_eeprom_with_dom_output
// =============================================================================

// #28 - show interfaces transceiver eeprom (SFP w/DOM)
#[test]
fn test_show_interfaces_transceiver_eeprom_sfp_dom() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_EEPROM_SFP_DOM).unwrap();
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_EEPROM_SFP_DOM_OUTPUT, "transceiver_eeprom sfp_dom");
}

// #28 - show interfaces transceiver eeprom (QSFP-DD)
#[test]
fn test_show_interfaces_transceiver_eeprom_qsfpdd() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_EEPROM_QSFPDD).unwrap();
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_EEPROM_QSFPDD_OUTPUT, "transceiver_eeprom qsfpdd");
}

// #28 - show interfaces transceiver eeprom (all)
#[test]
fn test_show_interfaces_transceiver_eeprom_all() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_EEPROM_ALL).unwrap();
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_EEPROM_ALL_OUTPUT, "transceiver_eeprom all");
}

// =============================================================================
// Transceiver presence tests
// Source: sfp_test.py test_sfp_presence_all_output
// =============================================================================

// #33 - show interfaces transceiver presence
#[test]
fn test_show_interfaces_transceiver_presence() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_PRESENCE).unwrap();
    let result = converter.convert("interfaces_transceiver_presence", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_PRESENCE_OUTPUT, "transceiver_presence");
}

// =============================================================================
// Transceiver status tests
// Source: sfp_test.py test_qsfp_status_output
// =============================================================================

// #34 - show interfaces transceiver status (QSFP)
#[test]
fn test_show_interfaces_transceiver_status_qsfp() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_STATUS_QSFP).unwrap();
    let result = converter.convert("interfaces_transceiver_status", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_STATUS_QSFP_OUTPUT, "transceiver_status qsfp");
}

// #34 - show interfaces transceiver status (removed)
#[test]
fn test_show_interfaces_transceiver_status_removed() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_STATUS_REMOVED).unwrap();
    let result = converter.convert("interfaces_transceiver_status", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_STATUS_REMOVED_OUTPUT, "transceiver_status removed");
}

// =============================================================================
// Transceiver lpmode tests
// =============================================================================

// #31 - show interfaces transceiver lpmode
#[test]
fn test_show_interfaces_transceiver_lpmode() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_LPMODE).unwrap();
    let result = converter.convert("interfaces_transceiver_lpmode", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_LPMODE_OUTPUT, "transceiver_lpmode");
}

// =============================================================================
// Transceiver error-status tests
// =============================================================================

// #29 - show interfaces transceiver error-status
#[test]
fn test_show_interfaces_transceiver_error_status() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_ERROR_STATUS).unwrap();
    let result = converter.convert("interfaces_transceiver_error_status", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_ERROR_STATUS_OUTPUT, "transceiver_error_status");
}

// =============================================================================
// Transceiver PM tests
// Source: sfp_test.py test_qsfp_dd_pm_output
// =============================================================================

// #32 - show interfaces transceiver pm
#[test]
fn test_show_interfaces_transceiver_pm() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_PM).unwrap();
    let result = converter.convert("interfaces_transceiver_pm", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_PM_OUTPUT, "transceiver_pm");
}

// #32 - show interfaces transceiver pm (N/A)
#[test]
fn test_show_interfaces_transceiver_pm_na() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_PM_NA).unwrap();
    let result = converter.convert("interfaces_transceiver_pm", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_PM_NA_OUTPUT, "transceiver_pm N/A");
}

// =============================================================================
// Transceiver firmware tests
// =============================================================================

// show interfaces transceiver firmware (not in command table)
#[test]
fn test_show_interfaces_transceiver_firmware() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_FIRMWARE).unwrap();
    let cmds = converter.available_commands();
    if cmds.iter().any(|c| c.contains("firmware")) {
        let result = converter.convert("interfaces_transceiver_firmware", data).unwrap();
        assert_exact_match(&result, SHOW_TRANSCEIVER_FIRMWARE_OUTPUT, "transceiver_firmware");
    } else {
        // interfaces_transceiver_firmware is not a registered command
        assert!(converter.convert("interfaces_transceiver_firmware", data).is_err());
    }
}

// =============================================================================
// Transceiver info tests
// =============================================================================

// #30 - show interfaces transceiver info (empty)
#[test]
fn test_show_interfaces_transceiver_info_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_TRANSCEIVER_INFO_EMPTY).unwrap();
    let result = converter.convert("interfaces_transceiver_info", data).unwrap();
    assert_exact_match(&result, SHOW_TRANSCEIVER_INFO_EMPTY_OUTPUT, "transceiver_info empty");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_transceiver_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "interfaces_transceiver_eeprom",
        "interfaces_transceiver_presence",
        "interfaces_transceiver_status",
        "interfaces_transceiver_lpmode",
        "interfaces_transceiver_error_status",
        "interfaces_transceiver_pm",
        "interfaces_transceiver_info",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing transceiver command: {}", cmd);
    }
}
