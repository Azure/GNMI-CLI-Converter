//! Unit tests for network-related show commands.
//!
//! Covers: lldp neighbors, lldp table, mac, mac aging-time, ndp, srv6 stats, vlan brief.
//!
//! Formatter modules: network (LldpTableFormatter, LldpNeighborsFormatter, MacFormatter,
//! MacAgingTimeFormatter, NdpFormatter, Srv6Formatter, VlanBriefFormatter)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data, normalize_output,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 44: show lldp neighbors
// =============================================================================

#[test]
fn test_show_lldp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(44, "lldp_neighbors");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("lldp_neighbors", data).unwrap();
            assert!(!result.trim().is_empty(), "LLDP neighbors output should not be empty");
            assert_output_contains_all(&result, &["LLDP neighbors"]);
        }
    }
}

// =============================================================================
// 45: show lldp table
// =============================================================================

#[test]
fn test_show_lldp_table() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "capability_codes_helper": "B - Bridge, R - Router, W - Wlan, S - Station",
        "neighbors": [
            {"localPort": "Ethernet0", "remoteDevice": "switch1", "remotePortId": "Ethernet1", "capability": "BR", "remotePortDescr": "Server1"},
            {"localPort": "Ethernet4", "remoteDevice": "switch2", "remotePortId": "Ethernet2", "capability": "R", "remotePortDescr": "Server2"}
        ],
        "total": 2
    }"#,
    );
    let result = converter.convert("lldp_table", data).unwrap();
    assert_output_contains_all(
        &result,
        &["Ethernet0", "switch1", "Ethernet4", "switch2"],
    );
}

#[test]
fn test_show_lldp_table_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(45, "lldp_table");
    assert!(!test_data.is_empty(), "No test data found for command 45");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("lldp_table", data).unwrap();
        assert_output_contains_all(&result, &["LocalPort", "RemoteDevice"]);
    }
}

// =============================================================================
// 46: show mac
// =============================================================================

/// Test show mac with data from sonic-utilities fdbshow_test.py show_mac_output.
/// Uses 5 MAC entries matching the exact mock data from sonic-utilities.
#[test]
fn test_show_mac() {
    let converter = GnmiToCliConverter::new();
    // Mock data matching sonic-utilities fdbshow_test.py mock variant "1"
    let data = load_test_data(
        r#"{
        "entries": [
            {"No.": "1", "vlan": "2", "mac": "11:22:33:44:55:66", "port": "Ethernet0", "type": "Dynamic"},
            {"No.": "2", "vlan": "3", "mac": "11:22:33:66:55:44", "port": "Ethernet4", "type": "Static"},
            {"No.": "3", "vlan": "4", "mac": "66:55:44:33:22:11", "port": "Ethernet0", "type": "Dynamic"},
            {"No.": "4", "vlan": "4", "mac": "77:66:44:33:22:11", "port": "1000000000fff", "type": "Dynamic"},
            {"No.": "5", "vlan": "5", "mac": "77:66:55:44:22:11", "port": "Ethernet4", "type": "Dynamic"}
        ],
        "total": 5
    }"#,
    );
    let result = converter.convert("mac", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac_output
    let expected = concat!(
        "  No.    Vlan  MacAddress         Port           Type\n",
        "-----  ------  -----------------  -------------  -------\n",
        "    1       2  11:22:33:44:55:66  Ethernet0      Dynamic\n",
        "    2       3  11:22:33:66:55:44  Ethernet4      Static\n",
        "    3       4  66:55:44:33:22:11  Ethernet0      Dynamic\n",
        "    4       4  77:66:44:33:22:11  1000000000fff  Dynamic\n",
        "    5       5  77:66:55:44:22:11  Ethernet4      Dynamic\n",
        "Total number of entries 5",
    );
    assert_cli_output_matches(&result, expected);
}

/// Test show mac with 2 entries and default VLAN, from sonic-utilities show_mac_output_with_def_vlan.
#[test]
fn test_show_mac_with_def_vlan() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "entries": [
            {"No.": "1", "vlan": "2", "mac": "11:22:33:44:55:66", "port": "Ethernet0", "type": "Dynamic"},
            {"No.": "2", "vlan": "3", "mac": "11:22:33:66:55:44", "port": "Ethernet4", "type": "Static"}
        ],
        "total": 2
    }"#,
    );
    let result = converter.convert("mac", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac_output_with_def_vlan
    let expected = concat!(
        "  No.    Vlan  MacAddress         Port       Type\n",
        "-----  ------  -----------------  ---------  -------\n",
        "    1       2  11:22:33:44:55:66  Ethernet0  Dynamic\n",
        "    2       3  11:22:33:66:55:44  Ethernet4  Static\n",
        "Total number of entries 2",
    );
    assert_cli_output_matches(&result, expected);
}

/// Test show mac with no results, from sonic-utilities fdbshow_test.py show_mac_no_results_output.
#[test]
fn test_show_mac_empty() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"entries": [], "total": 0}"#);
    let result = converter.convert("mac", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac_no_results_output
    let expected = "\
No.    Vlan    MacAddress    Port    Type
-----  ------  ------------  ------  ------
Total number of entries 0";
    assert_cli_output_matches(&result, expected);
}

/// Test show mac filtered by port and vlan, from sonic-utilities show_mac__port_vlan_output.
#[test]
fn test_show_mac_port_vlan_filter() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "entries": [
            {"No.": "1", "vlan": "2", "mac": "11:22:33:44:55:66", "port": "Ethernet0", "type": "Dynamic"}
        ],
        "total": 1
    }"#,
    );
    let result = converter.convert("mac", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac__port_vlan_output
    let expected = concat!(
        "  No.    Vlan  MacAddress         Port       Type\n",
        "-----  ------  -----------------  ---------  -------\n",
        "    1       2  11:22:33:44:55:66  Ethernet0  Dynamic\n",
        "Total number of entries 1",
    );
    assert_cli_output_matches(&result, expected);
}

/// Test show mac filtered by type (Static), from sonic-utilities show_mac__type_output.
#[test]
fn test_show_mac_type_filter() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "entries": [
            {"No.": "1", "vlan": "3", "mac": "11:22:33:66:55:44", "port": "Ethernet4", "type": "Static"}
        ],
        "total": 1
    }"#,
    );
    let result = converter.convert("mac", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac__type_output
    let expected = concat!(
        "  No.    Vlan  MacAddress         Port       Type\n",
        "-----  ------  -----------------  ---------  ------\n",
        "    1       3  11:22:33:66:55:44  Ethernet4  Static\n",
        "Total number of entries 1",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_mac_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(46, "mac");
    assert!(!test_data.is_empty(), "No test data found for command 46");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("mac", data).unwrap();
        assert_output_contains_all(&result, &["No.", "Vlan", "MacAddress", "Total number of entries"]);
    }
}

// =============================================================================
// 47: show mac aging-time
// =============================================================================

/// Test show mac aging-time with data from sonic-utilities fdbshow_test.py show_mac_aging_time_output.
#[test]
fn test_show_mac_aging_time() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"fdb_aging_time": "600s"}"#);
    let result = converter.convert("mac_aging_time", data).unwrap();
    // Expected output from sonic-utilities fdbshow_test.py show_mac_aging_time_output
    let expected = "Aging time for switch is 600 seconds";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_mac_aging_time_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(47, "mac_aging_time");
    assert!(!test_data.is_empty(), "No test data found for command 47");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("mac_aging_time", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 49: show ndp
// =============================================================================

#[test]
fn test_show_ndp() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities show_test.py ndp format
    let data = load_test_data(
        r#"{
        "total_entries": 2,
        "entries": [
            {"address": "fe80::6838:7bff:fe14:3020", "mac_address": "6A:38:7B:14:30:20", "iface": "PortChannel102", "vlan": "-", "status": "STALE"},
            {"address": "fe80::1", "mac_address": "00:11:22:33:44:55", "iface": "Ethernet0", "vlan": "-", "status": "REACHABLE"}
        ]
    }"#,
    );
    let result = converter.convert("ndp", data).unwrap();
    assert_output_contains_all(
        &result,
        &[
            "fe80::6838:7bff:fe14:3020",
            "6a:38:7b:14:30:20",
            "PortChannel102",
            "fe80::1",
            "Ethernet0",
            "Total number of entries",
        ],
    );
}

#[test]
fn test_show_ndp_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(49, "ndp");
    assert!(!test_data.is_empty(), "No test data found for command 49");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("ndp", data).unwrap();
        assert_output_contains_all(&result, &["Address", "MacAddress", "Iface", "Total number of entries"]);
    }
}

// =============================================================================
// 65: show srv6 stats
// =============================================================================

#[test]
fn test_show_srv6_stats() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(65, "srv6_stats");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("srv6_stats", data).unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

#[test]
fn test_show_srv6_stats_empty() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"[]"#);
    let result = converter.convert("srv6_stats", data).unwrap();
    // Empty SRv6 stats should show header only
    let expected = "\
MySID    Packets    Bytes
-------  ---------  -------";
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 69: show vlan brief
// =============================================================================

#[test]
fn test_show_vlan_brief() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(69, "vlan_brief");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("vlan_brief", data).unwrap();
            assert_output_contains_all(&result, &["VLAN ID", "IP Address", "Ports"]);
        }
    }
}

#[test]
fn test_show_vlan_brief_empty() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("vlan_brief", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Edge cases: empty data
// =============================================================================

#[test]
fn test_show_network_commands_empty_data() {
    let converter = GnmiToCliConverter::new();
    let empty_obj = load_test_data(r#"{}"#);

    let commands = [
        "ndp",
        "mac",
        "lldp_table",
        "srv6_stats",
        "vlan_brief",
    ];

    for cmd in &commands {
        let result = converter.convert(cmd, empty_obj.clone());
        assert!(
            result.is_ok(),
            "Command '{}' failed with empty data: {:?}",
            cmd,
            result.err()
        );
    }
}
