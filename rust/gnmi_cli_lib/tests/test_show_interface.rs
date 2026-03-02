//! Unit tests for interface-related show commands.
//!
//! Covers: interfaces alias, counters (standard/detailed/errors/fec-histogram/fec-stats/rates/rif/trim),
//! description, errors, fec status, flap, naming_mode, neighbor expected, portchannel,
//! status, switchport config/status.
//!
//! Formatter modules: interface (InterfaceCountersFormatter, InterfaceDescriptionFormatter,
//! InterfaceStatusFormatter, InterfaceAliasFormatter, InterfaceFlapFormatter, etc.)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 09: show interfaces alias
// =============================================================================

#[test]
fn test_show_interfaces_alias() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"alias": "etp1"},
        "Ethernet4": {"alias": "etp2"},
        "Ethernet8": {"alias": "etp3"}
    }"#,
    );
    let result = converter.convert("interfaces_alias", data).unwrap();
    let expected = concat!(
        "Name       Alias\n",
        "---------  -------\n",
        "Ethernet0  etp1\n",
        "Ethernet4  etp2\n",
        "Ethernet8  etp3",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_alias_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(9, "interfaces_alias");
    assert!(!test_data.is_empty(), "No test data found for command 09");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_alias", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 10: show interfaces counters
// =============================================================================

#[test]
fn test_show_interfaces_counters() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities portstat_test.py pattern
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "RxOk": "13489", "RxErr": "0", "RxDrp": "323", "RxOvr": "0",
            "TxOk": "10355", "TxErr": "0", "TxDrp": "0", "TxOvr": "0",
            "RxBps": "18.59", "TxBps": "27.46",
            "RxUtil": "0.00%", "TxUtil": "0.00%", "State": "U"
        },
        "Ethernet4": {
            "RxOk": "5000", "RxErr": "1", "RxDrp": "10", "RxOvr": "0",
            "TxOk": "3000", "TxErr": "0", "TxDrp": "0", "TxOvr": "0",
            "RxBps": "10.00", "TxBps": "15.00",
            "RxUtil": "0.00%", "TxUtil": "0.00%", "State": "U"
        }
    }"#,
    );
    let result = converter.convert("interfaces_counters", data).unwrap();
    let expected = concat!(
        "    IFACE    STATE    RX_OK    RX_BPS    RX_UTIL    RX_ERR    RX_DRP    RX_OVR    TX_OK    TX_BPS    TX_UTIL    TX_ERR    TX_DRP    TX_OVR\n",
        "---------  -------  -------  --------  ---------  --------  --------  --------  -------  --------  ---------  --------  --------  --------\n",
        "Ethernet0        U   13,489     18.59      0.00%         0       323         0   10,355     27.46      0.00%         0         0         0\n",
        "Ethernet4        U    5,000        10      0.00%         1        10         0    3,000        15      0.00%         0         0         0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_counters_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(10, "interfaces_counters");
    assert!(!test_data.is_empty(), "No test data found for command 10");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_counters", data).unwrap();
        assert_output_contains_all(&result, &["IFACE", "STATE", "RX_OK", "TX_OK"]);
    }
}

// =============================================================================
// 11: show interfaces counters detailed
// =============================================================================

#[test]
fn test_show_interfaces_counters_detailed() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities portstat_test.py detailed format
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "Rx64": "500", "Rx65_127": "300", "Rx128_255": "200",
            "Rx256_511": "100", "Rx512_1023": "50", "Rx1024_1518": "25",
            "Rx1519_2047": "10", "Rx2048_4095": "5", "Rx4096_9216": "2",
            "Rx9217_16383": "1",
            "RxAll": "1193", "RxUnicast": "800", "RxMulticast": "300", "RxBroadcast": "93",
            "RxJabbers": "0", "RxFragments": "0", "RxUndersize": "0", "RxOverruns": "0",
            "Tx64": "400", "Tx65_127": "250", "Tx128_255": "150",
            "Tx256_511": "80", "Tx512_1023": "40", "Tx1024_1518": "20",
            "Tx1519_2047": "8", "Tx2048_4095": "4", "Tx4096_9216": "2",
            "Tx9217_16383": "0",
            "TxAll": "954", "TxUnicast": "700", "TxMulticast": "200", "TxBroadcast": "54",
            "TrimPkts": "0", "TrimSent": "0", "TrimDrp": "0",
            "TimestampClearedCounters": "Never"
        }
    }"#,
    );
    let result = converter
        .convert("interfaces_counters_detailed", data)
        .unwrap();
    let expected = concat!(
        "Packets Received 64 Octets..................... 500\n",
        "Packets Received 65-127 Octets................. 300\n",
        "Packets Received 128-255 Octets................ 200\n",
        "Packets Received 256-511 Octets................ 100\n",
        "Packets Received 512-1023 Octets............... 50\n",
        "Packets Received 1024-1518 Octets.............. 25\n",
        "Packets Received 1519-2047 Octets.............. 10\n",
        "Packets Received 2048-4095 Octets.............. 5\n",
        "Packets Received 4096-9216 Octets.............. 2\n",
        "Packets Received 9217-16383 Octets............. 1\n",
        "\n",
        "Total Packets Received Without Errors.......... 1,193\n",
        "Unicast Packets Received....................... 800\n",
        "Multicast Packets Received..................... 300\n",
        "Broadcast Packets Received..................... 93\n",
        "\n",
        "Jabbers Received............................... 0\n",
        "Fragments Received............................. 0\n",
        "Undersize Received............................. 0\n",
        "Overruns Received.............................. 0\n",
        "\n",
        "Packets Transmitted 64 Octets.................. 400\n",
        "Packets Transmitted 65-127 Octets.............. 250\n",
        "Packets Transmitted 128-255 Octets............. 150\n",
        "Packets Transmitted 256-511 Octets............. 80\n",
        "Packets Transmitted 512-1023 Octets............ 40\n",
        "Packets Transmitted 1024-1518 Octets........... 20\n",
        "Packets Transmitted 1519-2047 Octets........... 8\n",
        "Packets Transmitted 2048-4095 Octets........... 4\n",
        "Packets Transmitted 4096-9216 Octets........... 2\n",
        "Packets Transmitted 9217-16383 Octets.......... 0\n",
        "\n",
        "Total Packets Transmitted Successfully......... 954\n",
        "Unicast Packets Transmitted.................... 700\n",
        "Multicast Packets Transmitted.................. 200\n",
        "Broadcast Packets Transmitted.................. 54\n",
        "Trimmed Packets................................ 0\n",
        "Trimmed Sent Packets........................... 0\n",
        "Trimmed Dropped Packets........................ 0\n",
        "\n",
        "Time Since Counters Last Cleared............... Never",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_counters_detailed_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(11, "interfaces_counters_detailed");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_counters_detailed", data)
                .unwrap();
            assert!(!result.trim().is_empty(), "Detailed counters output should not be empty");
            assert!(result.lines().count() >= 2, "Expected at least 2 lines");
        }
    }
}

// =============================================================================
// 12: show interfaces counters errors
// =============================================================================

#[test]
fn test_show_interfaces_counters_errors() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities portstat_test.py errors format
    let data = load_test_data(
        r#"{
        "Ethernet0": {"RxDrp": "323", "RxErr": "0", "RxOvr": "0", "TxDrp": "0", "TxErr": "0", "TxOvr": "0", "State": "U"},
        "Ethernet4": {"RxDrp": "10", "RxErr": "1", "RxOvr": "0", "TxDrp": "5", "TxErr": "2", "TxOvr": "0", "State": "U"}
    }"#,
    );
    let result = converter
        .convert("interfaces_counters_errors", data)
        .unwrap();
    let expected = concat!(
        "    IFACE    STATE    RX_ERR    RX_DRP    RX_OVR    TX_ERR    TX_DRP    TX_OVR\n",
        "---------  -------  --------  --------  --------  --------  --------  --------\n",
        "Ethernet0        U         0       323         0         0         0         0\n",
        "Ethernet4        U         1        10         0         2         5         0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_counters_errors_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(12, "interfaces_counters_errors");
    assert!(!test_data.is_empty(), "No test data found for command 12");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("interfaces_counters_errors", data)
            .unwrap();
        assert_output_contains_all(&result, &["IFACE", "STATE"]);
    }
}

// =============================================================================
// 13: show interfaces counters fec-histogram
// =============================================================================

#[test]
fn test_show_interfaces_counters_fec_histogram() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(13, "interfaces_counters_fec_histogram");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_counters_fec_histogram", data)
                .unwrap();
            assert!(!result.trim().is_empty(), "FEC histogram output should not be empty");
        }
    }
}

// =============================================================================
// 14: show interfaces counters fec-stats
// =============================================================================

#[test]
fn test_show_interfaces_counters_fec_stats() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(14, "interfaces_counters_fec_stats");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_counters_fec_stats", data)
                .unwrap();
            assert_output_contains_all(&result, &["IFACE", "STATE", "FEC_CORR"]);
        }
    }
}

// =============================================================================
// 15: show interfaces counters rates
// =============================================================================

#[test]
fn test_show_interfaces_counters_rates() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities portstat_test.py rates format
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "RxBps": "12.50", "RxOk": "13489", "RxPps": "0.06", "RxUtil": "0.00%",
            "TxBps": "18.46", "TxOk": "10355", "TxPps": "0.08", "TxUtil": "0.00%",
            "State": "U"
        }
    }"#,
    );
    let result = converter
        .convert("interfaces_counters_rates", data)
        .unwrap();
    let expected = concat!(
        "    IFACE    STATE    RX_OK    RX_BPS    RX_PPS    RX_UTIL    TX_OK    TX_BPS    TX_PPS    TX_UTIL\n",
        "---------  -------  -------  --------  --------  ---------  -------  --------  --------  ---------\n",
        "Ethernet0        U   13,489      12.5      0.06      0.00%   10,355     18.46      0.08      0.00%",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_counters_rates_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(15, "interfaces_counters_rates");
    assert!(!test_data.is_empty(), "No test data found for command 15");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("interfaces_counters_rates", data)
            .unwrap();
        assert_output_contains_all(&result, &["IFACE", "STATE", "RX_OK", "TX_OK"]);
    }
}

// =============================================================================
// 16: show interfaces counters rif
// =============================================================================

#[test]
fn test_show_interfaces_counters_rif() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {
            "RxOkPackets": "100", "RxOkBits": "8000",
            "RxErrPackets": "0", "RxErrBits": "0",
            "TxOkPackets": "200", "TxOkBits": "16000",
            "TxErrPackets": "0", "TxErrBits": "0",
            "RxBps": "N/A", "RxPps": "N/A",
            "TxBps": "N/A", "TxPps": "N/A"
        }
    }"#,
    );
    let result = converter.convert("interfaces_counters_rif", data).unwrap();
    let expected = concat!(
        "    IFACE    RX_OK    RX_BPS    RX_PPS    RX_ERR    TX_OK    TX_BPS    TX_PPS    TX_ERR    RX_OK_BITS    TX_OK_BITS    RX_ERR_BITS    TX_ERR_BITS\n",
        "---------  -------  --------  --------  --------  -------  --------  --------  --------  ------------  ------------  -------------  -------------\n",
        "Ethernet0      100       N/A       N/A         0      200       N/A       N/A         0          8000         16000              0              0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_counters_rif_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(16, "interfaces_counters_rif");
    assert!(!test_data.is_empty(), "No test data found for command 16");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_counters_rif", data).unwrap();
        assert_output_contains_all(&result, &["IFACE", "RX_OK", "TX_OK"]);
    }
}

// =============================================================================
// 17: show interfaces counters trim
// =============================================================================

#[test]
fn test_show_interfaces_counters_trim() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(17, "interfaces_counters_trim");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_counters_trim", data)
                .unwrap();
            assert_output_contains_all(&result, &["IFACE", "STATE", "TRIM"]);
        }
    }
}

// =============================================================================
// 18: show interfaces description
// =============================================================================

#[test]
fn test_show_interfaces_description() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities intfutil_test.py
    let data = load_test_data(
        r#"{
        "Ethernet0": {"Description": "ARISTA01T2:Ethernet1", "Admin": "up", "Oper": "down", "Alias": "Ethernet0"},
        "Ethernet16": {"Description": "ARISTA04T1:Ethernet16", "Admin": "up", "Oper": "up", "Alias": "etp5"},
        "Ethernet32": {"Description": "Servers7:eth0", "Admin": "up", "Oper": "up", "Alias": "etp9"}
    }"#,
    );
    let result = converter.convert("interfaces_description", data).unwrap();
    let expected = concat!(
        "  Interface    Oper    Admin      Alias            Description\n",
        "-----------  ------  -------  ---------  ---------------------\n",
        "  Ethernet0    down       up  Ethernet0   ARISTA01T2:Ethernet1\n",
        " Ethernet16      up       up       etp5  ARISTA04T1:Ethernet16\n",
        " Ethernet32      up       up       etp9          Servers7:eth0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_description_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(18, "interfaces_description");
    assert!(!test_data.is_empty(), "No test data found for command 18");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_description", data).unwrap();
        assert_output_contains_all(&result, &["Interface", "Oper", "Admin", "Alias", "Description"]);
    }
}

// =============================================================================
// 19: show interfaces errors
// =============================================================================

#[test]
fn test_show_interfaces_errors() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(19, "interfaces_errors");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("interfaces_errors", data).unwrap();
            assert!(!result.trim().is_empty(), "Interfaces errors output should not be empty");
            assert!(result.lines().count() >= 2, "Expected at least header + separator");
        }
    }
}

// =============================================================================
// 20: show interfaces fec status
// =============================================================================

#[test]
fn test_show_interfaces_fec_status() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities intfutil_test.py fec status format
    let data = load_test_data(
        r#"[
        {"Interface": "Ethernet0", "FEC Oper": "N/A", "FEC Admin": "rs"},
        {"Interface": "Ethernet16", "FEC Oper": "N/A", "FEC Admin": "N/A"},
        {"Interface": "Ethernet124", "FEC Oper": "rs", "FEC Admin": "auto"}
    ]"#,
    );
    let result = converter.convert("interfaces_fec_status", data).unwrap();
    let expected = concat!(
        "  Interface    FEC Oper    FEC Admin\n",
        "-----------  ----------  -----------\n",
        "  Ethernet0         N/A           rs\n",
        " Ethernet16         N/A          N/A\n",
        "Ethernet124          rs         auto",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_fec_status_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(20, "interfaces_fec_status");
    assert!(!test_data.is_empty(), "No test data found for command 20");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_fec_status", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 21: show interfaces flap
// =============================================================================

#[test]
fn test_show_interfaces_flap() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities interfaces_test.py flap format
    let data = load_test_data(
        r#"[
        {"Interface": "Ethernet0", "Oper": "Up", "Admin": "Up", "Flap Count": "1", "Link Up TimeStamp(UTC)": "Sun Jan 25 07:44:50 2026", "Link Down TimeStamp(UTC)": "Never"},
        {"Interface": "Ethernet4", "Oper": "Down", "Admin": "Up", "Flap Count": "3", "Link Up TimeStamp(UTC)": "Never", "Link Down TimeStamp(UTC)": "Sun Jan 25 06:00:00 2026"}
    ]"#,
    );
    let result = converter.convert("interfaces_flap", data).unwrap();
    let expected = concat!(
        "Interface      Flap Count  Admin    Oper    Link Down TimeStamp(UTC)    Link Up TimeStamp(UTC)\n",
        "-----------  ------------  -------  ------  --------------------------  ------------------------\n",
        "Ethernet0               1  Up       Up      Never                       Sun Jan 25 07:44:50 2026\n",
        "Ethernet4               3  Up       Down    Sun Jan 25 06:00:00 2026    Never",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_flap_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(21, "interfaces_flap");
    assert!(!test_data.is_empty(), "No test data found for command 21");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_flap", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 22: show interfaces naming_mode
// =============================================================================

#[test]
fn test_show_interfaces_naming_mode() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"naming_mode": "default"}"#);
    let result = converter.convert("interfaces_naming_mode", data).unwrap();
    let expected = "default";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_naming_mode_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(22, "interfaces_naming_mode");
    assert!(!test_data.is_empty(), "No test data found for command 22");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_naming_mode", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 23: show interfaces neighbor expected
// =============================================================================

#[test]
fn test_show_interfaces_neighbor_expected() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(23, "interfaces_neighbor_expected");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_neighbor_expected", data)
                .unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

// =============================================================================
// 24: show interfaces portchannel
// =============================================================================

#[test]
fn test_show_interfaces_portchannel() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "102": {
            "Team Dev": "PortChannel102",
            "Protocol": {"name": "LACP", "active": true, "operational_status": "up"},
            "Ports": [
                {"name": "Ethernet256", "selected": true, "in_sync": true, "status": "enabled"},
                {"name": "Ethernet260", "selected": true, "in_sync": true, "status": "enabled"}
            ]
        }
    }"#,
    );
    let result = converter.convert("interfaces_portchannel", data).unwrap();
    let expected = concat!(
        "Flags: A - active, I - inactive, Up - up, Dw - Down, N/A - not available,\n",
        "       S - selected, D - deselected, * - not synced\n",
        "  No.  Team Dev        Protocol     Ports\n",
        "-----  --------------  -----------  -----------------------------\n",
        "  102  PortChannel102  LACP(A)(Up)  Ethernet256(S) Ethernet260(S)",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_portchannel_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(24, "interfaces_portchannel");
    assert!(!test_data.is_empty(), "No test data found for command 24");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_portchannel", data).unwrap();
        assert_output_contains_all(&result, &["Flags:", "Team Dev", "Protocol"]);
    }
}

// =============================================================================
// 25: show interfaces status
// =============================================================================

#[test]
fn test_show_interfaces_status() {
    let converter = GnmiToCliConverter::new();
    // Data from sonic-utilities intfutil_test.py status format
    let data = load_test_data(
        r#"[
        {"Interface": "Ethernet0", "Lanes": "0", "Speed": "25G", "MTU": "9100", "FEC": "rs", "Alias": "Ethernet0", "Vlan": "routed", "Oper": "down", "Admin": "up", "Type": "QSFP28 or later", "Asym PFC": "off"},
        {"Interface": "Ethernet16", "Lanes": "16", "Speed": "100M", "MTU": "9100", "FEC": "N/A", "Alias": "etp5", "Vlan": "trunk", "Oper": "up", "Admin": "up", "Type": "RJ45", "Asym PFC": "off"},
        {"Interface": "Ethernet32", "Lanes": "13,14,15,16", "Speed": "40G", "MTU": "9100", "FEC": "rs", "Alias": "etp9", "Vlan": "PortChannel1001", "Oper": "up", "Admin": "up", "Type": "N/A", "Asym PFC": "off"}
    ]"#,
    );
    let result = converter.convert("interfaces_status", data).unwrap();
    let expected = concat!(
        "  Interface        Lanes    Speed    MTU    FEC      Alias             Vlan    Oper    Admin  Type               Asym PFC\n",
        "-----------  -----------  -------  -----  -----  ---------  ---------------  ------  -------  ---------------  ----------\n",
        "  Ethernet0            0      25G   9100     rs  Ethernet0           routed    down       up  QSFP28 or later         off\n",
        " Ethernet16           16     100M   9100    N/A       etp5            trunk      up       up  RJ45                    off\n",
        " Ethernet32  13,14,15,16      40G   9100     rs       etp9  PortChannel1001      up       up  N/A                     off",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_status_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(25, "interfaces_status");
    assert!(!test_data.is_empty(), "No test data found for command 25");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("interfaces_status", data).unwrap();
        assert_output_contains_all(&result, &["Interface", "Speed", "MTU", "Oper", "Admin"]);
    }
}

// =============================================================================
// 26: show interfaces switchport config
// =============================================================================

#[test]
fn test_show_interfaces_switchport_config() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"[
        {"Interface": "Ethernet0", "Mode": "routed", "Untagged": "", "Tagged": ""},
        {"Interface": "Ethernet4", "Mode": "trunk", "Untagged": "1000", "Tagged": "2000"}
    ]"#,
    );
    let result = converter
        .convert("interfaces_switchport_config", data)
        .unwrap();
    let expected = concat!(
        "Interface    Mode    Untagged    Tagged\n",
        "-----------  ------  ----------  --------\n",
        "Ethernet0    routed              \n",
        "Ethernet4    trunk   1000        2000",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_interfaces_switchport_config_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(26, "interfaces_switchport_config");
    assert!(!test_data.is_empty(), "No test data found for command 26");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("interfaces_switchport_config", data)
            .unwrap();
        assert_output_contains_all(&result, &["Interface", "Mode"]);
    }
}

// =============================================================================
// 27: show interfaces switchport status
// =============================================================================

#[test]
fn test_show_interfaces_switchport_status() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(27, "interfaces_switchport_status");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter
                .convert("interfaces_switchport_status", data)
                .unwrap();
            assert_output_contains_all(&result, &["Interface", "Mode"]);
        }
    }
}

// =============================================================================
// Edge cases: empty data
// =============================================================================

#[test]
fn test_show_interfaces_commands_empty_data() {
    let converter = GnmiToCliConverter::new();
    let empty_obj = load_test_data(r#"{}"#);

    let commands = [
        "interfaces_alias",
        "interfaces_counters",
        "interfaces_counters_errors",
        "interfaces_counters_rates",
        "interfaces_counters_rif",
        "interfaces_description",
        "interfaces_status",
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
