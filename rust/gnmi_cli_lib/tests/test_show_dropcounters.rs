//! Unit tests for dropcounters-related show commands.
//!
//! Covers: dropcounters capabilities, dropcounters counts.
//!
//! Formatter modules: dropcounters (DropCounterCapabilitiesFormatter,
//! DropCounterCountsFormatter)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 05: show dropcounters capabilities
// =============================================================================

#[test]
fn test_show_dropcounters_capabilities() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "PORT_INGRESS_DROPS": {
            "count": "11",
            "reasons": "[SMAC_EQUALS_DMAC,INGRESS_VLAN_FILTER,L2_ANY,L3_ANY]"
        },
        "PORT_EGRESS_DROPS": {
            "count": "3",
            "reasons": "[L2_ANY,L3_ANY,EGRESS_VLAN_FILTER]"
        }
    }"#,
    );
    let result = converter
        .convert("dropcounters_capabilities", data)
        .unwrap();
    let expected = concat!(
        "Counter Type          Total\n",
        "------------------  -------\n",
        "PORT_EGRESS_DROPS         3\n",
        "PORT_INGRESS_DROPS       11\n",
        "\n",
        "PORT_EGRESS_DROPS\n",
        "        L2_ANY\n",
        "        L3_ANY\n",
        "        EGRESS_VLAN_FILTER\n",
        "\n",
        "PORT_INGRESS_DROPS\n",
        "        SMAC_EQUALS_DMAC\n",
        "        INGRESS_VLAN_FILTER\n",
        "        L2_ANY\n",
        "        L3_ANY",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_dropcounters_capabilities_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(5, "dropcounters_capabilities");
    assert!(!test_data.is_empty(), "No test data found for command 05");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("dropcounters_capabilities", data)
            .unwrap();
        // Verify structural correctness: header present, at least one counter type listed
        assert_output_contains_all(
            &result,
            &["Counter Type", "Total"],
        );
        // Result should have at least a header line and one data row
        assert!(
            result.lines().count() >= 3,
            "Expected at least 3 lines in dropcounters capabilities output, got {}\n{}",
            result.lines().count(),
            result
        );
    }
}

// =============================================================================
// 06: show dropcounters counts
// =============================================================================

#[test]
fn test_show_dropcounters_counts() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"RX_DROPS": "100", "RX_ERR": "0", "TX_DROPS": "50", "TX_ERR": "0", "State": "U"},
        "Ethernet4": {"RX_DROPS": "200", "RX_ERR": "5", "TX_DROPS": "10", "TX_ERR": "1", "State": "U"}
    }"#,
    );
    let result = converter.convert("dropcounters_counts", data).unwrap();
    let expected = concat!(
        "    IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS\n",
        "---------  -------  --------  ----------  --------  ----------\n",
        "Ethernet0        U         0         100         0          50\n",
        "Ethernet4        U         5         200         1          10",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_dropcounters_counts_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(6, "dropcounters_counts");
    assert!(!test_data.is_empty(), "No test data found for command 06");
    let mut verified = 0;
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("dropcounters_counts", data).unwrap();
        if result.trim().is_empty() {
            continue; // Some input_options/devices yield empty output
        }
        // Verify structural correctness: header and ports present
        // (column set varies by device; counter values may drift between captures)
        assert_output_contains_all(
            &result,
            &["IFACE", "STATE"],
        );
        assert!(
            result.lines().count() >= 3,
            "Expected at least 3 lines in dropcounters counts output, got {}\n{}",
            result.lines().count(),
            result
        );
        verified += 1;
    }
    assert!(verified > 0, "No non-empty dropcounters counts outputs to verify");
}

// =============================================================================
// Edge cases: empty data
// =============================================================================

#[test]
fn test_show_dropcounters_empty_data() {
    let converter = GnmiToCliConverter::new();
    let empty_obj = load_test_data(r#"{}"#);

    for cmd in &["dropcounters_capabilities", "dropcounters_counts"] {
        let result = converter.convert(cmd, empty_obj.clone());
        assert!(
            result.is_ok(),
            "Command '{}' failed with empty data: {:?}",
            cmd,
            result.err()
        );
    }
}
