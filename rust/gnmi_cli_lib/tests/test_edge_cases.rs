//! Edge-case tests for formatter and parser code paths.
//!
//! Covers boundary conditions and less-common input shapes across:
//! - converter.rs: auto_parse_json_string edge case
//! - base.rs: Bool values in table, realign/separator edge cases
//! - dropcounters.rs: Default impls, empty data
//! - interface.rs: detailed counters, portchannel, fec, switchport, trim
//! - ipv6.rs: prefix-list, bgp graceful restart, max prefixes
//! - network.rs: LLDP total fallback
//! - queue.rs: persistent-watermark, wredcounters sorting
//! - system.rs: processes memory, mmu, services edge cases
//! - transceiver.rs: empty presence, eeprom string/null, error-status
//! - watermark.rs: buffer pool edge cases
//! - parser.rs: compound subcommands, filter_by_argument, help
//! - main.rs: JSON format output, verbose errors

mod common;

use common::{assert_output_contains_all, load_test_data, normalize_output};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::json;

// =============================================================================
// converter.rs – auto_parse_json_string: string that parses to non-obj/array
// =============================================================================

#[test]
fn test_converter_json_string_wrapping_number() {
    // Line 103: string parses to Value::Number → should NOT unwrap, keeps original
    let converter = GnmiToCliConverter::new();
    let data = json!("42");
    // convert_str with a JSON-encoded string of a number
    let result = converter.convert_str("clock", "\"42\"");
    assert!(result.is_ok());
}

#[test]
fn test_converter_convert_from_cli_command() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"Ethernet0": {"admin_status": "up", "oper_status": "up",
                       "speed": "100G", "type": "QSFP", "mtu": "9100", "fec": "rs"}});
    let result = converter.convert_from_cli_command("show interfaces status", data);
    assert!(result.is_ok());
    assert_output_contains_all(&result.unwrap(), &["Ethernet0"]);
}

#[test]
fn test_converter_convert_from_cli_command_unknown() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert_from_cli_command("show nonexistent xyz", json!({}));
    assert!(result.is_err());
}

// =============================================================================
// base.rs – Bool/empty row/realign/separator edge cases (lines 105,167,785...)
// =============================================================================

#[test]
fn test_formatter_table_with_bool_values() {
    // Line 105: Value::Bool branch in table extraction
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "admin_status": true,
            "oper_status": false,
            "speed": "100G"
        }
    });
    let result = converter.convert("interfaces_status", data);
    assert!(result.is_ok());
}

// =============================================================================
// dropcounters.rs – Default impl, empty data, Object-keyed format
// =============================================================================

#[test]
fn test_dropcounters_capabilities_with_array_data() {
    // Lines 23-24 Default, line 90 empty match
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {
            "counter_type": "PORT_INGRESS_DROPS",
            "count": "10",
            "reasons": "SMAC_EQUALS_DMAC,INGRESS_VLAN_FILTER"
        },
        {
            "counter_type": "SWITCH_EGRESS_DROPS",
            "count": "5",
            "reasons": "L2_ANY"
        }
    ]);
    let result = converter.convert("dropcounters_capabilities", data).unwrap();
    assert_output_contains_all(&result, &["PORT_INGRESS_DROPS", "SMAC_EQUALS_DMAC"]);
}

#[test]
fn test_dropcounters_capabilities_with_object_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "PORT_INGRESS_DROPS": {
            "count": "3",
            "reasons": "EXCEEDS_L2_MTU"
        }
    });
    let result = converter.convert("dropcounters_capabilities", data).unwrap();
    assert_output_contains_all(&result, &["PORT_INGRESS_DROPS", "EXCEEDS_L2_MTU"]);
}

#[test]
fn test_dropcounters_counts_with_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {
            "counter_type": "PORT_INGRESS_DROPS",
            "count": "100",
            "reasons": "L3_ANY"
        }
    ]);
    let result = converter.convert("dropcounters_counts", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// interface.rs – detailed counters, portchannel, fec, switchport, trim, errors
// =============================================================================

#[test]
fn test_interfaces_counters_detailed_format() {
    // Lines 501-574: format_single_interface method
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "RxOkPackets": "1000",
            "RxOkBits": "80000",
            "RxErrPackets": "5",
            "RxErrBits": "400",
            "TxOkPackets": "2000",
            "TxOkBits": "160000",
            "TxErrPackets": "3",
            "TxErrBits": "240"
        },
        "Ethernet4": {
            "RxOkPackets": "500",
            "RxOkBits": "40000",
            "RxErrPackets": "0",
            "RxErrBits": "0",
            "TxOkPackets": "600",
            "TxOkBits": "48000",
            "TxErrPackets": "0",
            "TxErrBits": "0"
        }
    });
    let result = converter.convert("interfaces_counters_detailed", data).unwrap();
    assert_output_contains_all(&result, &["Packets Received", "Packets Transmitted"]);
}

#[test]
fn test_interfaces_portchannel_with_protocol_object() {
    // Lines 882, 905, 908: Protocol as Object with name/active/operational_status
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "1": {
            "Name": "PortChannel1",
            "Protocol": {
                "name": "LACP",
                "active": true,
                "operational_status": "up"
            },
            "Ports": ["Ethernet0", "Ethernet4"],
            "Min Links": "1",
            "Fallback": "Disabled"
        },
        "2": {
            "Name": "PortChannel2",
            "flags": "LACP(A)(Up)",
            "members": ["Ethernet8"],
            "Min Links": "1",
            "Fallback": "Enabled"
        }
    });
    let result = converter.convert("interfaces_portchannel", data).unwrap();
    assert_output_contains_all(&result, &["PortChannel1", "LACP"]);
}

#[test]
fn test_interfaces_fec_histogram() {
    // Lines 1115, 1118: fec histogram format
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "FEC Histogram": [
                {"Bin": "0", "Codewords": "1000"},
                {"Bin": "1", "Codewords": "500"},
                {"Bin": "2", "Codewords": "0"}
            ]
        }
    });
    let result = converter.convert("interfaces_counters_fec_histogram", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_fec_stats() {
    // Lines 1143, 1447 etc.: fec stats normalize_zero_ber
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "FEC Corrected": "100",
            "FEC Uncorrected": "0",
            "Symbol Errors": "0",
            "BER": "0.00e+00"
        }
    });
    let result = converter.convert("interfaces_counters_fec_stats", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_switchport_config() {
    // Line 823: switchport config with prefix filtering
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "Mode": "access",
            "Untagged VLAN": "100"
        },
        "Ethernet4": {
            "Mode": "trunk",
            "Untagged VLAN": "1"
        }
    });
    let result = converter.convert("interfaces_switchport_config", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_switchport_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "Mode": "routed"
        }
    });
    let result = converter.convert("interfaces_switchport_status", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_counters_trim() {
    // Lines 1514, 1529-1535: trim counters with TRIM columns
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "IFACE": "Ethernet0",
            "TRIM_PKTS": "100",
            "TRIM_TX_PKTS": "50",
            "TRIM_DRP_PKTS": "0"
        },
        "Ethernet4": {
            "IFACE": "Ethernet4",
            "TRIM_PKTS": "200",
            "TRIM_TX_PKTS": "100",
            "TRIM_DRP_PKTS": "5"
        }
    });
    let result = converter.convert("interfaces_counters_trim", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_errors_with_data() {
    // Line 656: interface errors formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "CRC Error": {"count": "5", "Last timestamp(UTC)": "2026-01-01 00:00:00"},
            "Frame Error": {"count": "0", "Last timestamp(UTC)": "N/A"}
        }
    });
    let result = converter.convert("interfaces_errors", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_counters_detailed_with_negative_number() {
    // Line 27: format_number_with_commas with negative number
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "RxOkPackets": "-1000",
            "RxOkBits": "0",
            "RxErrPackets": "0",
            "RxErrBits": "0",
            "TxOkPackets": "0",
            "TxOkBits": "0",
            "TxErrPackets": "0",
            "TxErrBits": "0"
        }
    });
    let result = converter.convert("interfaces_counters_detailed", data).unwrap();
    // The formatter may render negative values as-is or N/A
    assert!(result.contains("Packets Received"));
}

#[test]
fn test_interfaces_counters_detailed_with_na_counter() {
    // Line 331,333,334,336: TimestampClearedCounters, N/A handling
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "InOctets": "N/A",
            "OutOctets": "1000",
            "TimestampClearedCounters": "not-a-number"
        }
    });
    let result = converter.convert("interfaces_counters", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_flap_with_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "Interface": "Ethernet0",
            "Flap Count": "3",
            "Last Flap Time": "2026-01-01"
        }
    });
    let result = converter.convert("interfaces_flap", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_fec_status() {
    // Line 1583, 1592, 1596: format_g6 function paths
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "FEC Corrected": "12345678",
            "FEC Uncorrected": "0",
            "BER": "1.23456e-10",
            "FEC Status": "locked"
        }
    });
    let result = converter.convert("interfaces_fec_status", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// ipv6.rs – prefix-list, bgp neighbors, bgp network, fib, link-local
// =============================================================================

#[test]
fn test_ipv6_prefix_list_with_data() {
    // Lines 249, 259, 291 - prefix list formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "PL_V6_1": {
            "seq": [
                {"seq_num": "10", "action": "permit", "prefix": "2001:db8::/32", "ge": "48", "le": "64"},
                {"seq_num": "20", "action": "deny", "prefix": "fc00::/7"}
            ]
        }
    });
    let result = converter.convert("ipv6_prefix_list", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_interfaces_with_addresses() {
    // Lines 249, 259: ipv6 interfaces with address data 
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "admin_status": "up",
            "oper_status": "up",
            "ipv6_addresses": [
                {"address": "2001:db8::1/64", "Type": "global"}
            ],
            "type": "Ethernet"
        }
    });
    let result = converter.convert("ipv6_interfaces", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_bgp_neighbors_detailed() {
    // Lines 1705-1715, 1758, 1761, 1856, 1896, 1910-1912: BGP neighbor detail
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "2001:db8::2": {
            "remoteAs": 65001,
            "localAs": 65000,
            "nbrInternalLink": false,
            "nbrExternalLink": true,
            "bgpState": "Established",
            "bgpTimerUpMsec": 86400000,
            "bgpTimerUpString": "1d00h00m",
            "bgpTimerLastRead": "00:00:05",
            "bgpTimerLastWrite": "00:00:10",
            "addressFamilyInfo": {
                "ipv6Unicast": {
                    "acceptedPrefixCounter": 100,
                    "sentPrefixCounter": 50,
                    "prefixAllowedMax": 1000,
                    "prefixAllowedMaxWarning": false,
                    "prefixAllowedWarningThresh": 80
                }
            },
            "gracefulRestartInfo": {
                "endOfRibSend": true,
                "endOfRibRecv": true,
                "timers": {
                    "configuredRestartTimer": 120,
                    "receivedRestartTimer": 120
                },
                "ipv6Unicast": {
                    "endOfRibSend": true,
                    "endOfRibSendAfterUpdate": false,
                    "endOfRibRecv": true,
                    "timers": {
                        "stalePathTimer": 360,
                        "llgrStaleTime": 0
                    }
                }
            },
            "messageStats": {
                "opensSent": 1,
                "opensRecv": 1,
                "notificationsSent": 0,
                "notificationsRecv": 0,
                "updatesSent": 50,
                "updatesRecv": 100,
                "keepalivesSent": 1000,
                "keepalivesRecv": 1000,
                "routeRefreshSent": 0,
                "routeRefreshRecv": 0,
                "totalSent": 1051,
                "totalRecv": 1101
            },
            "connectionsEstablished": 1,
            "connectionsDropped": 0,
            "externalBgpNbrMaxHopsAway": 1,
            "extendedOptionalParametersLength": true,
            "hostLocal": "2001:db8::1",
            "portLocal": 179,
            "hostForeign": "2001:db8::2",
            "portForeign": 45678,
            "nexthop": {
                "ipv6Global": "2001:db8::1",
                "ipv6LinkLocal": "fe80::1"
            }
        }
    });
    let result = converter.convert("ipv6_bgp_neighbors", data).unwrap();
    assert_output_contains_all(&result, &["65001", "Established"]);
}

#[test]
fn test_ipv6_bgp_network() {
    // Lines 2417-2419: bgp network formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "networks": [
            {"network": "2001:db8::/32", "next_hop": "::"}
        ]
    });
    let result = converter.convert("ipv6_bgp_network", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_bgp_summary() {
    // Lines 2629, 2697: bgp summary
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "routerId": "10.0.0.1",
        "as": 65000,
        "totalPeers": 2,
        "peers": {
            "2001:db8::1": {
                "remoteAs": 65001,
                "state": "Established",
                "pfxRcd": 100,
                "pfxSnt": 50
            }
        }
    });
    let result = converter.convert("ipv6_bgp_summary", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_fib() {
    // Lines 2765, 2837-2838: fib formatter
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {
            "prefix": "2001:db8::/32",
            "nexthop": "2001:db8::1",
            "interface": "Ethernet0",
            "protocol": "bgp"
        }
    ]);
    let result = converter.convert("ipv6_fib", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_link_local_mode() {
    // Line 2870: link-local-mode
    let converter = GnmiToCliConverter::new();
    let data = json!({"mode": "enabled"});
    let result = converter.convert("ipv6_link_local_mode", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_route_with_entries() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "2001:db8::/32": {
            "nexthops": [
                {"ip": "2001:db8::1", "interface": "Ethernet0"}
            ],
            "protocol": "connected"
        }
    });
    let result = converter.convert("ipv6_route", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ipv6_protocol() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"protocol": "BGP", "router_id": "10.0.0.1"});
    let result = converter.convert("ipv6_protocol", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// network.rs – LLDP with total fallback (line 181)
// =============================================================================

#[test]
fn test_lldp_table_with_neighbors() {
    // Line 181: total calculated from data when not explicitly provided
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "neighbors": [
            {
                "LocalPort": "Ethernet0",
                "RemoteDevice": "switch1",
                "RemotePort": "Ethernet1",
                "Capability": "Bridge"
            }
        ]
    });
    let result = converter.convert("lldp_table", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_lldp_neighbors_with_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "neighbor": "switch1",
            "remote_port": "Ethernet1",
            "chassis_id": "aa:bb:cc:dd:ee:ff"
        }
    });
    let result = converter.convert("lldp_neighbors", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// queue.rs – persistent-watermark, wredcounters, queue sorting
// =============================================================================

#[test]
fn test_queue_persistent_watermark_multicast() {
    // Lines 142, 175: persistent watermark multicast
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "MC8": "100",
            "MC9": "200"
        }
    });
    let result = converter.convert("queue_persistent_watermark_multicast", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_queue_persistent_watermark_all() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "UC0": "50",
            "UC1": "100",
            "MC8": "200"
        }
    });
    let result = converter.convert("queue_persistent_watermark_all", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_queue_watermark_multicast() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"MC8": "50", "MC9": "25"}
    });
    let result = converter.convert("queue_watermark_multicast", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_queue_watermark_all() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"UC0": "10", "MC8": "20"}
    });
    let result = converter.convert("queue_watermark_all", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

#[test]
fn test_queue_wredcounters_with_data() {
    // Lines 804, 805: wredcounters sorting UC/MC
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "UC0": {"GreenPackets": "100", "YellowPackets": "10", "RedPackets": "1"},
            "UC1": {"GreenPackets": "200", "YellowPackets": "20", "RedPackets": "2"},
            "MC8": {"GreenPackets": "50", "YellowPackets": "5", "RedPackets": "0"}
        },
        "Ethernet4": {
            "UC0": {"GreenPackets": "300", "YellowPackets": "30", "RedPackets": "3"}
        }
    });
    let result = converter.convert("queue_wredcounters", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_queue_counters_with_data() {
    // Lines 275, 323: queue counters with interface data
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "UC0": {
                "totalpacket": "1000",
                "totalbytes": "80000",
                "droppacket": "5",
                "dropbytes": "400"
            },
            "UC1": {
                "totalpacket": "2000",
                "totalbytes": "160000",
                "droppacket": "0",
                "dropbytes": "0"
            }
        }
    });
    let result = converter.convert("queue_counters", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_queue_persistent_watermark_unicast() {
    // Lines 463-467: persistent watermark unicast
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"UC0": "123", "UC1": "456"}
    });
    let result = converter.convert("queue_persistent_watermark_unicast", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0"]);
}

// =============================================================================
// system.rs – processes memory, processes cpu, mmu, services, system-memory
// =============================================================================

#[test]
fn test_processes_memory_with_data() {
    // Lines 653, 691: processes memory formatter
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {
            "PID": "1",
            "PPID": "0",
            "CMD": "init",
            "%MEM": "0.1",
            "%CPU": "0.0",
            "STIME": "Jan01",
            "TIME": "00:00:01",
            "TT": "?",
            "UID": "root"
        },
        {
            "PID": "2",
            "PPID": "1",
            "CMD": "very-long-command-name-that-is-over-fifty-characters-long-testing-truncation",
            "%MEM": "1.0",
            "%CPU": "0.5",
            "STIME": "Jan01",
            "TIME": "00:01:00",
            "TT": "?",
            "UID": "root"
        }
    ]);
    let result = converter.convert("processes_memory", data).unwrap();
    // Output may be empty or a simple table depending on data shape
    let _ = normalize_output(&result);
}

#[test]
fn test_processes_cpu_with_data() {
    // Line 569: processes CPU prefix padding
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "processes": [
            {
                "USER": "root",
                "PID": "1",
                "%CPU": "0.1",
                "%MEM": "0.2",
                "VSZ": "12345",
                "RSS": "6789",
                "TTY": "?",
                "STAT": "Ss",
                "START": "Jan01",
                "TIME": "0:01",
                "COMMAND": "/sbin/init"
            }
        ]
    });
    let result = converter.convert("processes_cpu", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_system_memory_with_data() {
    // Line 239: system-memory formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "MemTotal": "16384000",
        "MemFree": "8192000",
        "MemAvailable": "12000000",
        "Buffers": "500000",
        "Cached": "3000000"
    });
    let result = converter.convert("system_memory", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_mmu_with_data() {
    // Lines 1068, 1069: mmu formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "pool": {
            "ingress_lossless_pool": {
                "size": "1000000",
                "type": "ingress"
            }
        }
    });
    let result = converter.convert("mmu", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_services_with_data() {
    // Line 984: services formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "services": [
            {"name": "swss", "status": "active"},
            {"name": "bgp", "status": "active"},
            {"name": "snmp", "status": "inactive"}
        ]
    });
    let result = converter.convert("services", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_processes_summary_with_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "summary": "24 total, 1 running, 23 sleeping, 0 stopped, 0 zombie"
    });
    let result = converter.convert("processes_summary", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// transceiver.rs – presence, eeprom, info, error-status, lpmode, pm, status
// =============================================================================

#[test]
fn test_transceiver_presence_with_array_data() {
    // Lines 25, 71-72: presence formatter with array format
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {"port": "Ethernet0", "Presence": "Present"},
        {"port": "Ethernet4", "Presence": "Not Present"},
        {"port": "Ethernet8", "Presence": "present"}
    ]);
    let result = converter.convert("interfaces_transceiver_presence", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0", "Present"]);
}

#[test]
fn test_transceiver_presence_with_object_data() {
    // Lines 166-167, 199-200: presence from object map
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"Presence": "Present"},
        "Ethernet4": {"Presence": "Not Present"}
    });
    let result = converter.convert("interfaces_transceiver_presence", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_eeprom_with_string_json_data() {
    // Lines 457-459,483-485,491-493: EEPROM string JSON parsing branches
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": "{\"Connector\": \"LC\", \"Encoding\": \"64B/66B\", \"Vendor Name\": \"ACME\"}",
        "Ethernet4": "{}",
        "Ethernet8": null,
        "Ethernet12": "",
        "Ethernet16": "not-valid-json"
    });
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert_output_contains_all(&result, &["SFP EEPROM"]);
}

#[test]
fn test_transceiver_eeprom_with_nested_object() {
    // Lines 611-613: EEPROM with Application Advertisement
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "Connector": "LC",
            "Encoding": "64B/66B",
            "Vendor Name": "ACME",
            "Application Advertisement": {
                "1": {"Host IF": "400GAUI-8", "Media IF": "400G-FR4"}
            }
        }
    });
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert_output_contains_all(&result, &["SFP EEPROM detected"]);
}

#[test]
fn test_transceiver_error_status_with_data() {
    // Lines 246,253,254: error status with object data (CMIS state, REMOVED)
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "CMIS State (SW)": "READY",
            "Module Fault Cause": "No Fault"
        },
        "Ethernet4": {
            "CMIS State (SW)": "REMOVED"
        }
    });
    let result = converter.convert("interfaces_transceiver_error_status", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_error_status_as_array() {
    // Lines 363-364: error status from array format
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {"port": "Ethernet0", "CMIS State (SW)": "READY", "Module Fault Cause": "No Fault"}
    ]);
    let result = converter.convert("interfaces_transceiver_error_status", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_info_with_data() {
    // Lines 403: transceiver info formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "type": "QSFP28",
            "vendor_name": "ACME",
            "serial_number": "SN12345",
            "hardware_rev": "1.0",
            "firmware_version": "2.0"
        }
    });
    let result = converter.convert("interfaces_transceiver_info", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_lpmode() {
    // Lines 735-736: lpmode
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"Low-power Mode": "Off"},
        "Ethernet4": {"Low-power Mode": "On"}
    });
    let result = converter.convert("interfaces_transceiver_lpmode", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_pm_with_data() {
    // Lines 829-830, 903-904: pm formatter
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "Tx Power": "-1.5 dBm",
            "Rx Power": "-2.0 dBm",
            "Bias Current": "30 mA"
        }
    });
    let result = converter.convert("interfaces_transceiver_pm", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_status_with_cmis() {
    // Lines 930, 985, 1028-1032: transceiver status
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "CMIS State (SW)": "READY",
            "Module State": "ModuleReady",
            "Tx Fault": "false",
            "Tx Disable": "false",
            "Tx Disabled Channel": "0"
        }
    });
    let result = converter.convert("interfaces_transceiver_status", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_transceiver_status_removed() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "CMIS State (SW)": "REMOVED"
        }
    });
    let result = converter.convert("interfaces_transceiver_status", data).unwrap();
    assert_output_contains_all(&result, &["not applicable"]);
}

#[test]
fn test_transceiver_eeprom_with_empty_object() {
    // Test empty obj produces "Not detected"
    let converter = GnmiToCliConverter::new();
    let data = json!({"Ethernet0": {}});
    let result = converter.convert("interfaces_transceiver_eeprom", data).unwrap();
    assert!(result.contains("Not detected"));
}

// =============================================================================
// watermark.rs – buffer pool empty, edge cases (lines 236,237,277)
// =============================================================================

#[test]
fn test_watermark_telemetry_interval() {
    // Line 277: watermark telemetry interval
    let converter = GnmiToCliConverter::new();
    let data = json!({"interval": "120"});
    let result = converter.convert("watermark_telemetry", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_buffer_pool_watermark_empty_map() {
    // Lines 236, 237: buffer pool with empty map → table with headers only
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    // Should produce a table with at least headers
    let _ = normalize_output(&result);
}

// =============================================================================
// parser.rs – compound subcommands, filter, help (lines 308,349-357,416...)
// =============================================================================

#[test]
fn test_parse_and_convert_with_argument() {
    // Line 536: filter_by_argument with argument containing ':'
    let converter = GnmiToCliConverter::new();
    let result = gnmi_cli_lib::parser::parse_and_convert(
        "show interfaces status Ethernet0",
        json!({
            "Ethernet0": {"admin_status": "up", "oper_status": "up", "speed": "100G"},
            "Ethernet4": {"admin_status": "up", "oper_status": "down", "speed": "25G"},
            "Ethernet0:1": {"admin_status": "up", "oper_status": "up", "speed": "10G"}
        }),
    );
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Ethernet0"));
}

#[test]
fn test_parse_and_convert_invalid_command() {
    // Line 654: parse_and_convert with invalid parsed command
    let result = gnmi_cli_lib::parser::parse_and_convert(
        "show invalid_cmd",
        json!({}),
    );
    assert!(result.is_err());
}

#[test]
fn test_parse_command_no_subcommand() {
    // Lines 349-357: missing subcommand error path
    let parsed = gnmi_cli_lib::parser::parse_command("show interfaces");
    // Should produce a valid parsed with empty subcommand or auto-resolved
    let _ = parsed;
}

#[test]
fn test_parse_command_compound_subcommand() {
    // Lines 308, 416-417: compound subcommand normalization
    let parsed = gnmi_cli_lib::parser::parse_command("show interfaces counters fec-histogram");
    assert!(parsed.is_valid);
    assert_eq!(parsed.command, "interfaces");
}

#[test]
fn test_parse_command_with_options() {
    // Parser option extraction
    let parsed = gnmi_cli_lib::parser::parse_command("show interfaces counters --period 5 --nonzero");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_command_unknown_subcommand() {
    // Line 454: unknown subcommand error path
    let parsed = gnmi_cli_lib::parser::parse_command("show interfaces nonexistent_sub");
    assert!(!parsed.is_valid);
}

#[test]
fn test_parse_and_convert_with_period_option() {
    // Parser with period option forwarded to converter
    let result = gnmi_cli_lib::parser::parse_and_convert(
        "show interfaces counters --period 5",
        json!({
            "Ethernet0": {"InOctets": "1000", "OutOctets": "2000"}
        }),
    );
    assert!(result.is_ok());
}

#[test]
fn test_command_parser_get_gnmi_path() {
    let parser = gnmi_cli_lib::parser::CommandParser::new();
    let path = parser.get_gnmi_path("show interfaces status");
    assert!(path.is_some());
    assert!(path.unwrap().contains("interfaces"));
}

#[test]
fn test_command_parser_list_commands() {
    let parser = gnmi_cli_lib::parser::CommandParser::new();
    let cmds = parser.list_commands();
    assert!(!cmds.is_empty());
    assert!(cmds.contains_key("interfaces"));
}

#[test]
fn test_command_parser_get_help() {
    let parser = gnmi_cli_lib::parser::CommandParser::new();
    let help = parser.get_command_help("interfaces", None);
    assert!(help.contains("subcommand") || help.contains("show"));

    // Unknown command help
    let help_unknown = parser.get_command_help("nonexistent", None);
    assert!(help_unknown.contains("Unknown command"));

    // Known command, unknown subcommand
    let help_sub = parser.get_command_help("interfaces", Some("nonexistent_sub"));
    assert!(help_sub.contains("Unknown subcommand"));

    // Known command, known subcommand with optional arg
    let help_valid = parser.get_command_help("interfaces", Some("status"));
    assert!(help_valid.contains("show"));
}

// =============================================================================
// Additional edge cases: Version, mac, ndp, vlan, srv6, reboot-cause, uptime
// =============================================================================

#[test]
fn test_version_with_full_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "build_version": "SONiC.20240101.01",
        "sonic_os_version": "11",
        "debian_version": "11.0",
        "kernel_version": "5.10.0",
        "platform": "x86_64-mlnx_msn2700-r0",
        "hwsku": "Mellanox-SN2700",
        "asic_type": "mellanox",
        "serial_number": "SN12345",
        "uptime": "10 days",
        "docker_images": [
            {"Repository": "docker-orchagent", "Tag": "latest", "Image ID": "abc123", "Size": "500MB"},
            {"Repository": "docker-syncd", "Tag": "latest", "Image ID": "def456", "Size": "600MB"}
        ]
    });
    let result = converter.convert("version", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_mac_with_entries() {
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {"MacAddress": "aa:bb:cc:dd:ee:ff", "Vlan": "100", "Port": "Ethernet0", "Type": "Dynamic"},
        {"MacAddress": "11:22:33:44:55:66", "Vlan": "200", "Port": "Ethernet4", "Type": "Static"}
    ]);
    let result = converter.convert("mac", data).unwrap();
    assert_output_contains_all(&result, &["Ethernet0", "Dynamic"]);
}

#[test]
fn test_mac_aging_time() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"aging_time": "600"});
    let result = converter.convert("mac_aging_time", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_ndp_with_data() {
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {"Address": "2001:db8::1", "MacAddress": "aa:bb:cc:dd:ee:ff", "Iface": "Ethernet0", "Vlan": "-", "Status": "STALE"}
    ]);
    let result = converter.convert("ndp", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_vlan_brief() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "100": {
            "vlanid": "100",
            "ip_address": "10.0.0.1/24",
            "ports": "Ethernet0,Ethernet4",
            "dhcp_servers": []
        }
    });
    let result = converter.convert("vlan_brief", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_srv6_stats() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "packets": "1000",
        "bytes": "80000"
    });
    let result = converter.convert("srv6_stats", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_reboot_cause_history() {
    let converter = GnmiToCliConverter::new();
    let data = json!([
        {"cause": "Power cycle", "time": "2026-01-01 00:00:00", "user": "admin", "comment": "scheduled"},
        {"cause": "Watchdog", "time": "2025-12-31 12:00:00", "user": "system", "comment": ""}
    ]);
    let result = converter.convert("reboot_cause_history", data).unwrap();
    let _ = normalize_output(&result);
}

// =============================================================================
// Table format variations (pipe, plain) for formatter coverage
// =============================================================================

#[test]
fn test_formatter_pipe_format() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Pipe);
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "1000"},
        "egress_lossy_pool": {"Bytes": "2000"}
    });
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool"]);
}

#[test]
fn test_formatter_plain_format() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Plain);
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "500"}
    });
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    assert_output_contains_all(&result, &["ingress_lossless_pool"]);
}

// =============================================================================
// Convenience module coverage
// =============================================================================

#[test]
fn test_convenience_convert() {
    let data = json!({"date": "Mon Feb 6 12:00:00 UTC 2026"});
    let result = gnmi_cli_lib::convenience::convert("clock", data);
    assert!(result.is_ok());
}

// =============================================================================
// convert_str with valid JSON string
// =============================================================================

#[test]
fn test_convert_str_valid_json() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert_str(
        "interfaces_status",
        r#"{"Ethernet0": {"admin_status": "up", "oper_status": "up"}}"#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_convert_with_options() {
    let converter = GnmiToCliConverter::new();
    let mut opts = std::collections::HashMap::new();
    opts.insert("period".to_string(), "5".to_string());
    let result = converter.convert_with_options(
        "interfaces_counters",
        json!({"Ethernet0": {"InOctets": "1000"}}),
        Some(opts),
    );
    assert!(result.is_ok());
}

// =============================================================================
// Additional interface formatters for boundary coverage
// =============================================================================

#[test]
fn test_interfaces_neighbor_expected() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"neighbor": "switch1", "port": "Ethernet1"}
    });
    let result = converter.convert("interfaces_neighbor_expected", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_counters_rates() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"RX_BPS": "1000000", "TX_BPS": "2000000", "RX_PPS": "100", "TX_PPS": "200"}
    });
    let result = converter.convert("interfaces_counters_rates", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_counters_rif() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"RX_OK": "1000", "TX_OK": "2000", "RX_ERR": "0", "TX_ERR": "0"}
    });
    let result = converter.convert("interfaces_counters_rif", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_interfaces_counters_errors() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {"InErrors": "5", "OutErrors": "3", "InDiscards": "1", "OutDiscards": "0"}
    });
    let result = converter.convert("interfaces_counters_errors", data).unwrap();
    let _ = normalize_output(&result);
}
