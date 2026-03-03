//! Integration tests for interface formatters.
//!
//! Tests interface alias, neighbor, portchannel, switchport, status,
//! description, counters, FEC, and naming mode commands using mock data
//! derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/interfaces_test.py
//!
//! ## Formatter coverage
//! - `interfaces_alias` (InterfacesAliasFormatter)
//! - `interfaces_neighbor_expected` (InterfacesNeighborFormatter)
//! - `interfaces_portchannel` (InterfacesPortchannelFormatter)
//! - `interfaces_switchport_*` (SwitchportStatusFormatter / SwitchportConfigFormatter)
//! - `interfaces_status` (InterfacesStatusFormatter)
//! - `interfaces_description` (InterfacesDescriptionFormatter)
//! - `interfaces_counters*` (InterfacesCountersFormatter variants)
//! - `interfaces_fec_*` (FecStatusFormatter / FecStatsFormatter)
//! - `interfaces_naming_mode` (NamingModeFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_interfaces::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Interface alias tests
// Source: interfaces_test.py show_interfaces_alias_output
// =============================================================================

// #09 - show interfaces alias
#[test]
fn test_show_interfaces_alias() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_ALIAS).unwrap();
    let result = converter.convert("interfaces_alias", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_ALIAS_OUTPUT, "interfaces_alias");
}

// #09 - show interfaces alias (single interface)
#[test]
fn test_show_interfaces_alias_single_interface() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_ALIAS_ETH0).unwrap();
    let result = converter.convert("interfaces_alias", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_ALIAS_ETHERNET0_OUTPUT, "interfaces_alias single");
}

// =============================================================================
// Interface neighbor expected tests
// Source: interfaces_test.py show_interfaces_neighbor_expected_output
// =============================================================================

// #23 - show interfaces neighbor expected
#[test]
fn test_show_interfaces_neighbor_expected() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_NEIGHBOR).unwrap();
    let result = converter.convert("interfaces_neighbor_expected", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_NEIGHBOR_EXPECTED_OUTPUT, "interfaces_neighbor_expected");
}

// =============================================================================
// Interface portchannel tests
// Source: interfaces_test.py show_interfaces_portchannel_output
// =============================================================================

// #24 - show interfaces portchannel
#[test]
fn test_show_interfaces_portchannel() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_PORTCHANNEL).unwrap();
    let result = converter.convert("interfaces_portchannel", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_PORTCHANNEL_OUTPUT, "interfaces_portchannel");
}

// =============================================================================
// Switchport tests
// Source: interfaces_test.py show_interfaces_switchport_*
// =============================================================================

// #27 - show interfaces switchport status
#[test]
fn test_show_interfaces_switchport_status() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_SWITCHPORT_STATUS).unwrap();
    let result = converter.convert("interfaces_switchport_status", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_SWITCHPORT_STATUS_OUTPUT, "interfaces_switchport_status");
}

// #26 - show interfaces switchport config
#[test]
fn test_show_interfaces_switchport_config() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_SWITCHPORT_CONFIG).unwrap();
    let result = converter.convert("interfaces_switchport_config", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_SWITCHPORT_CONFIG_OUTPUT, "interfaces_switchport_config");
}

// =============================================================================
// Interface flap tests
// Source: interfaces_test.py intf_flap_expected_output_with_data
// =============================================================================

// #21 - show interfaces flap
#[test]
fn test_show_interfaces_flap() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_FLAP).unwrap();
    let result = converter.convert("interfaces_flap", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_FLAP_OUTPUT, "interfaces_flap");
}

// =============================================================================
// Interface errors tests
// Source: interfaces_test.py intf_errors_*
// =============================================================================

// #19 - show interfaces errors
#[test]
fn test_show_interfaces_errors() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_ERRORS).unwrap();
    let result = converter.convert("interfaces_errors", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_ERRORS_OUTPUT, "interfaces_errors");
}

// =============================================================================
// Interface status tests
// =============================================================================

// #25 - show interfaces status
#[test]
fn test_show_interfaces_status() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_STATUS).unwrap();
    let result = converter.convert("interfaces_status", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_STATUS_OUTPUT, "interfaces_status");
}

// #18 - show interfaces description
#[test]
fn test_show_interfaces_description() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_DESCRIPTION).unwrap();
    let result = converter.convert("interfaces_description", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_DESCRIPTION_OUTPUT, "interfaces_description");
}

// =============================================================================
// Interface counters tests
// =============================================================================

// #10 - show interfaces counters
#[test]
fn test_show_interfaces_counters() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_COUNTERS).unwrap();
    let result = converter.convert("interfaces_counters", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_COUNTERS_OUTPUT, "interfaces_counters");
}

// #12 - show interfaces counters errors
#[test]
fn test_show_interfaces_counters_errors() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_COUNTERS_ERRORS).unwrap();
    let result = converter.convert("interfaces_counters_errors", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_COUNTERS_ERRORS_OUTPUT, "interfaces_counters_errors");
}

// #15 - show interfaces counters rates
#[test]
fn test_show_interfaces_counters_rates() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_COUNTERS_RATES).unwrap();
    let result = converter.convert("interfaces_counters_rates", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_COUNTERS_RATES_OUTPUT, "interfaces_counters_rates");
}

// #16 - show interfaces counters rif
#[test]
fn test_show_interfaces_counters_rif() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_INTERFACES_COUNTERS_RIF).unwrap();
    let result = converter.convert("interfaces_counters_rif", data).unwrap();
    assert_exact_match(&result, SHOW_INTERFACES_COUNTERS_RIF_OUTPUT, "interfaces_counters_rif");
}

// =============================================================================
// FEC tests
// =============================================================================

// #20 - show interfaces fec status
#[test]
fn test_show_interfaces_fec_status() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_FEC_STATUS).unwrap();
    let result = converter.convert("interfaces_fec_status", data).unwrap();
    assert_exact_match(&result, SHOW_FEC_STATUS_OUTPUT, "interfaces_fec_status");
}

// #14 - show interfaces counters fec-stats
#[test]
fn test_show_interfaces_counters_fec_stats() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_FEC_STATS).unwrap();
    let result = converter.convert("interfaces_counters_fec_stats", data).unwrap();
    assert_exact_match(&result, SHOW_FEC_STATS_OUTPUT, "interfaces_counters_fec_stats");
}

// #13 - show interfaces counters fec-histogram
#[test]
fn test_show_interfaces_counters_fec_histogram() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_FEC_HISTOGRAM).unwrap();
    let result = converter.convert("interfaces_counters_fec_histogram", data).unwrap();
    assert_exact_match(&result, SHOW_FEC_HISTOGRAM_OUTPUT, "interfaces_counters_fec_histogram");
}

// #17 - show interfaces counters trim
#[test]
fn test_show_interfaces_counters_trim() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_COUNTERS_TRIM).unwrap();
    let result = converter.convert("interfaces_counters_trim", data).unwrap();
    assert_exact_match(&result, SHOW_COUNTERS_TRIM_OUTPUT, "interfaces_counters_trim");
}

// =============================================================================
// Naming mode test
// =============================================================================

// #22 - show interfaces naming_mode
#[test]
fn test_show_interfaces_naming_mode() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_NAMING_MODE).unwrap();
    let result = converter.convert("interfaces_naming_mode", data).unwrap();
    assert_exact_match(&result, SHOW_NAMING_MODE_OUTPUT, "interfaces_naming_mode");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_interface_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "interfaces_alias",
        "interfaces_neighbor_expected",
        "interfaces_portchannel",
        "interfaces_switchport_status",
        "interfaces_switchport_config",
        "interfaces_status",
        "interfaces_description",
        "interfaces_counters",
        "interfaces_counters_errors",
        "interfaces_counters_rates",
        "interfaces_counters_rif",
        "interfaces_fec_status",
        "interfaces_counters_fec_stats",
        "interfaces_counters_fec_histogram",
        "interfaces_counters_trim",
        "interfaces_naming_mode",
        "interfaces_flap",
        "interfaces_errors",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing interface command: {}", cmd);
    }
}

// registration test
#[test]
fn test_transceiver_subcommands_registered() {
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
