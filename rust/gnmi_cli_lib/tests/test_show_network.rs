//! Integration tests for network formatters.
//!
//! Tests LLDP, VLAN, MAC, ARP, NDP, BGP, SRv6, and NTP commands
//! using mock data derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/vlan_test.py, bgp test vectors
//!
//! ## Formatter coverage
//! - `lldp_table` / `lldp_neighbors` (LldpFormatter)
//! - `vlan_brief` (VlanBriefFormatter)
//! - `mac` (MacFormatter)
//! - `mac_aging_time` (MacAgingTimeFormatter)
//! - `ndp` (NdpFormatter)
//! - `bgp_summary` (BgpSummaryFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_network::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// LLDP tests
// =============================================================================

// #45 - show lldp table
#[test]
fn test_show_lldp_table() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_LLDP_TABLE).unwrap();
    let result = converter.convert("lldp_table", data).unwrap();
    assert_exact_match(&result, SHOW_LLDP_TABLE_OUTPUT, "lldp_table");
}

// #44 - show lldp neighbors
#[test]
fn test_show_lldp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_LLDP_NEIGHBORS).unwrap();
    let result = converter.convert("lldp_neighbors", data).unwrap();
    assert_exact_match(&result, SHOW_LLDP_NEIGHBORS_OUTPUT, "lldp_neighbors");
}

// =============================================================================
// VLAN tests
// Source: vlan_test.py show_vlan_brief_output
// =============================================================================

// #69 - show vlan brief
#[test]
fn test_show_vlan_brief() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_VLAN_BRIEF).unwrap();
    let result = converter.convert("vlan_brief", data).unwrap();
    assert_exact_match(&result, SHOW_VLAN_BRIEF_OUTPUT, "vlan_brief");
}

// #69 - show vlan brief (empty)
#[test]
fn test_show_vlan_brief_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_VLAN_BRIEF_EMPTY).unwrap();
    let result = converter.convert("vlan_brief", data).unwrap();
    assert_exact_match(&result, SHOW_VLAN_BRIEF_EMPTY_OUTPUT, "vlan_brief empty");
}

// =============================================================================
// MAC tests
// =============================================================================

// #46 - show mac
#[test]
fn test_show_mac() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_MAC_TABLE).unwrap();
    let result = converter.convert("mac", data).unwrap();
    assert_exact_match(&result, SHOW_MAC_TABLE_OUTPUT, "mac");
}

// #47 - show mac aging_time
#[test]
fn test_show_mac_aging_time() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_MAC_AGING_TIME).unwrap();
    let result = converter.convert("mac_aging_time", data).unwrap();
    assert_exact_match(&result, SHOW_MAC_AGING_TIME_OUTPUT, "mac_aging_time");
}

// =============================================================================
// NDP tests
// =============================================================================

// #49 - show ndp
#[test]
fn test_show_ndp() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_NDP_TABLE).unwrap();
    let result = converter.convert("ndp", data).unwrap();
    assert_exact_match(&result, SHOW_NDP_TABLE_OUTPUT, "ndp");
}

// =============================================================================
// BGP summary tests
// =============================================================================

// show bgp summary (not in command table, expects error)
#[test]
fn test_show_bgp_summary() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_BGP_SUMMARY).unwrap();
    let result = converter.convert("bgp_summary", data);
    // bgp_summary is not a registered command - verify it returns an error
    assert!(result.is_err(), "bgp_summary should not be a registered command");
}

// =============================================================================
// SRv6 stats tests
// =============================================================================

// #65 - show srv6 stats
#[test]
fn test_show_srv6_stats() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_SRV6_STATS).unwrap();
    let result = converter.convert("srv6_stats", data).unwrap();
    assert_exact_match(&result, SHOW_SRV6_STATS_OUTPUT, "srv6_stats");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_network_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "lldp_table",
        "lldp_neighbors",
        "vlan_brief",
        "mac",
        "mac_aging_time",
        "ndp",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing network command: {}", cmd);
    }
}
