//! Integration tests for IPv6 formatters.
//!
//! Tests IPv6 route, BGP summary, BGP neighbors, BGP network, interfaces,
//! link-local mode, prefix-list, protocol, and FIB commands using mock data
//! derived from sonic-utilities test vectors.
//!
//! Source: sonic-utilities/tests/show_ip_route_common.py
//!
//! ## Formatter coverage
//! - `ipv6_route` (Ipv6RouteFormatter)
//! - `ipv6_bgp_summary` (Ipv6BgpSummaryFormatter)
//! - `ipv6_bgp_neighbors` (Ipv6BgpNeighborsFormatter)
//! - `ipv6_bgp_network` (Ipv6BgpNetworkFormatter)
//! - `ipv6_interfaces` (Ipv6InterfacesFormatter)
//! - `ipv6_link_local_mode` (Ipv6LinkLocalModeFormatter)
//! - `ipv6_prefix_list` (Ipv6PrefixListFormatter)
//! - `ipv6_protocol` (Ipv6ProtocolFormatter)
//! - `ipv6_fib` (Ipv6FibFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_ipv6::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// IPv6 route tests
// Source: show_ip_route_common.py show_ipv6_route_expected_output
// =============================================================================

// #37 - show ipv6 route
#[test]
fn test_show_ipv6_route() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_ROUTE).unwrap();
    let result = converter.convert("ipv6_route", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_ROUTE_EXPECTED_OUTPUT, "ipv6_route");
}

// #37 - show ipv6 route (empty)
#[test]
fn test_show_ipv6_route_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_ROUTE_EMPTY).unwrap();
    let result = converter.convert("ipv6_route", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_ROUTE_EMPTY_OUTPUT, "ipv6_route empty");
}

// =============================================================================
// IPv6 BGP summary tests
// =============================================================================

// #41 - show ipv6 bgp summary
#[test]
fn test_show_ipv6_bgp_summary() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_BGP_SUMMARY).unwrap();
    let result = converter.convert("ipv6_bgp_summary", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_BGP_SUMMARY_OUTPUT, "ipv6_bgp_summary");
}

// #41 - show ipv6 bgp summary (empty)
#[test]
fn test_show_ipv6_bgp_summary_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_BGP_SUMMARY_EMPTY).unwrap();
    let result = converter.convert("ipv6_bgp_summary", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_BGP_SUMMARY_EMPTY_OUTPUT, "ipv6_bgp_summary empty");
}

// =============================================================================
// IPv6 BGP neighbors tests
// =============================================================================

// #39 - show ipv6 bgp neighbors
#[test]
fn test_show_ipv6_bgp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_BGP_NEIGHBORS).unwrap();
    let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_BGP_NEIGHBORS_OUTPUT, "ipv6_bgp_neighbors");
}

// =============================================================================
// IPv6 BGP network tests
// =============================================================================

// #40 - show ipv6 bgp network
#[test]
fn test_show_ipv6_bgp_network() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_BGP_NETWORK).unwrap();
    let result = converter.convert("ipv6_bgp_network", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_BGP_NETWORK_OUTPUT, "ipv6_bgp_network");
}

// =============================================================================
// IPv6 interfaces tests
// =============================================================================

// #36 - show ipv6 interfaces
#[test]
fn test_show_ipv6_interfaces() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_INTERFACES).unwrap();
    let result = converter.convert("ipv6_interfaces", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_INTERFACES_OUTPUT, "ipv6_interfaces");
}

// =============================================================================
// IPv6 link-local mode tests
// =============================================================================

// #42 - show ipv6 link-local-mode
#[test]
fn test_show_ipv6_link_local_mode() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_LINK_LOCAL_MODE).unwrap();
    let result = converter.convert("ipv6_link_local_mode", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_LINK_LOCAL_MODE_OUTPUT, "ipv6_link_local_mode");
}

// =============================================================================
// IPv6 prefix-list tests
// =============================================================================

// #35 - show ipv6 prefix-list
#[test]
fn test_show_ipv6_prefix_list() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_PREFIX_LIST).unwrap();
    let result = converter.convert("ipv6_prefix_list", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_PREFIX_LIST_OUTPUT, "ipv6_prefix_list");
}

// =============================================================================
// IPv6 protocol tests
// =============================================================================

// #38 - show ipv6 protocol
#[test]
fn test_show_ipv6_protocol() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_PROTOCOL).unwrap();
    let result = converter.convert("ipv6_protocol", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_PROTOCOL_OUTPUT, "ipv6_protocol");
}

// =============================================================================
// IPv6 FIB tests
// =============================================================================

// #43 - show ipv6 fib
#[test]
fn test_show_ipv6_fib() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_IPV6_FIB).unwrap();
    let result = converter.convert("ipv6_fib", data).unwrap();
    assert_exact_match(&result, SHOW_IPV6_FIB_OUTPUT, "ipv6_fib");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_ipv6_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "ipv6_route",
        "ipv6_bgp_summary",
        "ipv6_bgp_neighbors",
        "ipv6_bgp_network",
        "ipv6_interfaces",
        "ipv6_link_local_mode",
        "ipv6_prefix_list",
        "ipv6_protocol",
        "ipv6_fib",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing ipv6 command: {}", cmd);
    }
}
