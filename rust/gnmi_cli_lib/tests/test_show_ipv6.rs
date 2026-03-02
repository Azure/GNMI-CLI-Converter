//! Unit tests for IPv6-related show commands.
//!
//! Covers: ipv6 prefix-list, interfaces, route, protocol, bgp neighbors,
//! bgp network, bgp summary, link-local-mode, fib.
//!
//! Formatter modules: ipv6 (Ipv6RouteFormatter, Ipv6PrefixListFormatter,
//! Ipv6BgpNeighborsFormatter, etc.)
//!
//! Mock data and expected outputs are defined in common/ipv6_expected.rs,
//! extracted from sonic-utilities test vectors.

mod common;

use common::ipv6_expected::*;
use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data, load_verifying_test_data,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use std::collections::HashMap;

// =============================================================================
// 35: show ipv6 prefix-list
// =============================================================================

#[test]
fn test_show_ipv6_prefix_list() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(35, "ipv6_prefix_list");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_prefix_list", data).unwrap();
            assert!(!result.trim().is_empty(), "ipv6 prefix-list output should not be empty");
        }
    }
}

#[test]
fn test_show_ipv6_prefix_list_mock_3_entries() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_PREFIX_LIST_JSON);
    let result = converter.convert("ipv6_prefix_list", data).unwrap();
    assert_cli_output_matches(&result, IPV6_PREFIX_LIST_EXPECTED);
}

#[test]
fn test_show_ipv6_prefix_list_mock_multi_protocol() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_PREFIX_LIST_MULTI_JSON);
    let result = converter.convert("ipv6_prefix_list", data).unwrap();
    assert_cli_output_matches(&result, IPV6_PREFIX_LIST_MULTI_EXPECTED);
}

// =============================================================================
// 36: show ipv6 interfaces
// =============================================================================

#[test]
fn test_show_ipv6_interfaces() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(36, "ipv6_interfaces");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_interfaces", data).unwrap();
            assert_output_contains_all(&result, &["Interface", "IPv6 address"]);
        }
    }
}

#[test]
fn test_show_ipv6_interfaces_mock_two_interfaces() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_INTERFACES_JSON);
    let result = converter.convert("ipv6_interfaces", data).unwrap();
    assert_cli_output_matches(&result, IPV6_INTERFACES_EXPECTED);
}

#[test]
fn test_show_ipv6_interfaces_mock_single_interface() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_INTERFACES_SINGLE_JSON);
    let result = converter.convert("ipv6_interfaces", data).unwrap();
    assert_cli_output_matches(&result, IPV6_INTERFACES_SINGLE_EXPECTED);
}

// =============================================================================
// 37: show ipv6 route
// =============================================================================

#[test]
fn test_show_ipv6_route() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(37, "ipv6_route");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_route", data).unwrap();
            assert!(!result.trim().is_empty(), "ipv6 route output should not be empty");
            assert!(result.lines().count() >= 3, "Expected at least 3 lines in route output");
        }
    }
}

#[test]
fn test_show_ipv6_route_mock_single_line_frr() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_ROUTE_SINGLE_LINE_JSON);
    let result = converter.convert("ipv6_route", data).unwrap();
    assert_cli_output_matches(&result, IPV6_ROUTE_SINGLE_LINE_EXPECTED);
}

#[test]
fn test_show_ipv6_route_mock_specific_prefix() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_ROUTE_SPECIFIC_PREFIX_JSON);
    let mut opts = HashMap::new();
    opts.insert("argument".to_string(), "2603:10e2:400::".to_string());
    let result = converter
        .convert_with_options("ipv6_route", data, Some(opts))
        .unwrap();
    assert_cli_output_matches(&result, IPV6_ROUTE_SPECIFIC_PREFIX_EXPECTED);
}

#[test]
fn test_show_ipv6_route_mock_bgp_via() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_ROUTE_BGP_VIA_JSON);
    let result = converter.convert("ipv6_route", data).unwrap();
    assert_cli_output_matches(&result, IPV6_ROUTE_BGP_VIA_EXPECTED);
}

#[test]
fn test_show_ipv6_route_mock_empty() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_ROUTE_EMPTY_JSON);
    let result = converter.convert("ipv6_route", data).unwrap();
    assert_cli_output_matches(&result, IPV6_ROUTE_EMPTY_EXPECTED);
}

#[test]
fn test_show_ipv6_route_mock_specific_prefix_bgp() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_ROUTE_SPECIFIC_PREFIX_BGP_JSON);
    let mut opts = HashMap::new();
    opts.insert("argument".to_string(), "20c0:a820:0:80::/64".to_string());
    let result = converter
        .convert_with_options("ipv6_route", data, Some(opts))
        .unwrap();
    assert_cli_output_matches(&result, IPV6_ROUTE_SPECIFIC_PREFIX_BGP_EXPECTED);
}

// =============================================================================
// 38: show ipv6 protocol
// =============================================================================

#[test]
fn test_show_ipv6_protocol() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(38, "ipv6_protocol");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_protocol", data).unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

#[test]
fn test_show_ipv6_protocol_mock_4_protocols() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_PROTOCOL_JSON);
    let result = converter.convert("ipv6_protocol", data).unwrap();
    assert_cli_output_matches(&result, IPV6_PROTOCOL_EXPECTED);
}

#[test]
fn test_show_ipv6_protocol_mock_multi_vrf() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_PROTOCOL_MULTI_VRF_JSON);
    let result = converter.convert("ipv6_protocol", data).unwrap();
    assert_cli_output_matches(&result, IPV6_PROTOCOL_MULTI_VRF_EXPECTED);
}

// =============================================================================
// 39: show ipv6 bgp neighbors
// =============================================================================

#[test]
fn test_show_ipv6_bgp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(39, "ipv6_bgp_neighbors");
    if !test_data.is_empty() {
        let mut verified = 0;
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
            if result.trim().is_empty() {
                continue;
            }
            assert_output_contains_all(&result, &["BGP"]);
            verified += 1;
        }
        assert!(verified > 0, "No non-empty ipv6 bgp neighbors outputs");
    }
}

#[test]
fn test_show_ipv6_bgp_neighbors_mock_detail() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NEIGHBORS_DETAIL_JSON);
    let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
    assert_output_contains_all(&result, IPV6_BGP_NEIGHBORS_DETAIL_EXPECTED_SUBSTRINGS);
}

#[test]
fn test_show_ipv6_bgp_neighbors_mock_adv_routes() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NEIGHBORS_ADV_ROUTES_JSON);
    let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_NEIGHBORS_ADV_ROUTES_EXPECTED);
}

#[test]
fn test_show_ipv6_bgp_neighbors_mock_recv_routes() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NEIGHBORS_RECV_ROUTES_JSON);
    let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_NEIGHBORS_RECV_ROUTES_EXPECTED);
}

// =============================================================================
// 40: show ipv6 bgp network
// =============================================================================

#[test]
fn test_show_ipv6_bgp_network() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(40, "ipv6_bgp_network");
    if !test_data.is_empty() {
        let mut verified = 0;
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_bgp_network", data).unwrap();
            if result.trim().is_empty() {
                continue;
            }
            assert!(!result.trim().is_empty(), "ipv6 bgp network output should not be empty");
            verified += 1;
        }
        assert!(verified > 0, "No non-empty ipv6 bgp network outputs");
    }
}

#[test]
fn test_show_ipv6_bgp_network_mock_passthrough() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NETWORK_JSON);
    let result = converter.convert("ipv6_bgp_network", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_NETWORK_EXPECTED);
}

#[test]
fn test_show_ipv6_bgp_network_mock_full_table() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NETWORK_FULL_TABLE_JSON);
    let result = converter.convert("ipv6_bgp_network", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_NETWORK_FULL_TABLE_EXPECTED);
}

#[test]
fn test_show_ipv6_bgp_network_mock_ip_address() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_NETWORK_IP_ADDR_JSON);
    let result = converter.convert("ipv6_bgp_network", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_NETWORK_IP_ADDR_EXPECTED);
}

// =============================================================================
// 41: show ipv6 bgp summary
// =============================================================================

#[test]
fn test_show_ipv6_bgp_summary() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(41, "ipv6_bgp_summary");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_bgp_summary", data).unwrap();
            assert_output_contains_all(&result, &["IPv6 Unicast Summary", "BGP router identifier"]);
        }
    }
}

#[test]
fn test_show_ipv6_bgp_summary_mock_2_peers() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_BGP_SUMMARY_JSON);
    let result = converter.convert("ipv6_bgp_summary", data).unwrap();
    assert_cli_output_matches(&result, IPV6_BGP_SUMMARY_EXPECTED);
}

// =============================================================================
// 42: show ipv6 link-local-mode
// =============================================================================

#[test]
fn test_show_ipv6_link_local_mode() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(42, "ipv6_link_local_mode");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_link_local_mode", data).unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

#[test]
fn test_show_ipv6_link_local_mode_mock_3_ports() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_LINK_LOCAL_MODE_JSON);
    let result = converter.convert("ipv6_link_local_mode", data).unwrap();
    assert_cli_output_matches(&result, IPV6_LINK_LOCAL_MODE_EXPECTED);
}

#[test]
fn test_show_ipv6_link_local_mode_mock_enabled() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_LINK_LOCAL_MODE_ENABLED_JSON);
    let result = converter.convert("ipv6_link_local_mode", data).unwrap();
    assert_cli_output_matches(&result, IPV6_LINK_LOCAL_MODE_ENABLED_EXPECTED);
}

// =============================================================================
// 43: show ipv6 fib
// =============================================================================

#[test]
fn test_show_ipv6_fib() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(43, "ipv6_fib");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("ipv6_fib", data).unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

#[test]
fn test_show_ipv6_fib_mock_5_entries() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_FIB_JSON);
    let result = converter.convert("ipv6_fib", data).unwrap();
    assert_cli_output_matches(&result, IPV6_FIB_EXPECTED);
}

#[test]
fn test_show_ipv6_fib_mock_single_entry() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(IPV6_FIB_SINGLE_JSON);
    let result = converter.convert("ipv6_fib", data).unwrap();
    assert_cli_output_matches(&result, IPV6_FIB_SINGLE_EXPECTED);
}
