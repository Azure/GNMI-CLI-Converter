
// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Mock JSON for `show lldp table`
pub const MOCK_LLDP_TABLE: &str = r#"{
    "capability_codes_helper": "B - Bridge, R - Router, W - WLAN Access Point, S - Station",
    "neighbors": [
        {"local_interface": "Ethernet0", "neighbor_name": "ARISTA01T1", "neighbor_port": "Ethernet1", "capability": "BR", "remote_port_descr": ""},
        {"local_interface": "Ethernet4", "neighbor_name": "ARISTA02T1", "neighbor_port": "Ethernet1", "capability": "BR", "remote_port_descr": ""},
        {"local_interface": "Ethernet8", "neighbor_name": "ARISTA03T1", "neighbor_port": "Ethernet1", "capability": "R", "remote_port_descr": ""}
    ],
    "total": 3
}"#;

/// Mock JSON for `show lldp neighbors`
pub const MOCK_LLDP_NEIGHBORS: &str = r#"{
    "title": "LLDP neighbors:",
    "interfaces": {
        "Ethernet0": {
            "via": "LLDP",
            "RID": "1",
            "Time": "0 day, 03:45:22",
            "Chassis": {
                "ChassisID": "00:11:22:33:44:55",
                "SysName": "ARISTA01T1",
                "SysDescr": "Arista Networks EOS version 4.27.3F",
                "MgmtIP": "10.250.0.51",
                "Capability": [{"enabled": true, "type": "Bridge"}, {"enabled": true, "type": "Router"}]
            },
            "Port": {"PortID": "Ethernet1", "PortDescr": "ARISTA01T1:Ethernet1", "TTL": "120"}
        }
    }
}"#;

/// Mock JSON for `show vlan brief`
/// Source: vlan_test.py `show_vlan_brief_output`
pub const MOCK_VLAN_BRIEF: &str = r#"[
    {
        "vlan_id": "1000",
        "ip_address": "192.168.0.1/21",
        "ports": [
            {"name": "Ethernet4", "port_tagging": "untagged"},
            {"name": "Ethernet8", "port_tagging": "untagged"},
            {"name": "Ethernet12", "port_tagging": "untagged"},
            {"name": "Ethernet16", "port_tagging": "untagged"}
        ],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": []
    },
    {
        "vlan_id": "2000",
        "ip_address": "192.168.1.1/24",
        "ports": [
            {"name": "Ethernet24", "port_tagging": "untagged"},
            {"name": "Ethernet28", "port_tagging": "untagged"}
        ],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": []
    }
]"#;

/// Mock JSON for `show vlan brief` — empty VLAN
/// Source: vlan_test.py `show_vlan_brief_empty_output`
pub const MOCK_VLAN_BRIEF_EMPTY: &str = r#"[]"#;

/// Mock JSON for `show mac`
/// Source: fdbshow_input/fdbshow_masic_test_vectors.py
pub const MOCK_MAC_TABLE: &str = r#"{
    "entries": [
        {"vlan": "1000", "macAddress": "00:11:22:33:44:55", "type": "Dynamic", "port": "Ethernet4"},
        {"vlan": "1000", "macAddress": "AA:BB:CC:DD:EE:FF", "type": "Static", "port": "Ethernet8"},
        {"vlan": "2000", "macAddress": "11:22:33:44:55:66", "type": "Dynamic", "port": "Ethernet24"}
    ],
    "total": 3
}"#;

/// Mock JSON for `show mac aging-time`
pub const MOCK_MAC_AGING_TIME: &str = r#"{"fdb_aging_time": "600s"}"#;

/// Mock JSON for `show ip bgp summary`
pub const MOCK_BGP_SUMMARY: &str = r#"{
    "router_id": "10.1.0.1",
    "local_as": "65100",
    "neighbors": [
        {"neighbor": "10.0.0.1", "version": "4", "remote_as": "65200", "msg_rcvd": "1000", "msg_sent": "950", "tbl_ver": "0", "in_q": "0", "out_q": "0", "uptime": "03:30:00", "state": "Established", "received_prefixes": "10", "desc": "ARISTA01T1"},
        {"neighbor": "10.0.0.3", "version": "4", "remote_as": "65200", "msg_rcvd": "800", "msg_sent": "750", "tbl_ver": "0", "in_q": "0", "out_q": "0", "uptime": "02:15:00", "state": "Established", "received_prefixes": "8", "desc": "ARISTA02T1"}
    ]
}"#;

/// Mock JSON for NDP table
pub const MOCK_NDP_TABLE: &str = r#"{
    "total_entries": 3,
    "entries": [
        {"address": "fe80::1", "mac_address": "00:11:22:33:44:55", "iface": "Ethernet0", "vlan": "", "status": "REACHABLE"},
        {"address": "fe80::2", "mac_address": "AA:BB:CC:DD:EE:FF", "iface": "Ethernet4", "vlan": "", "status": "STALE"},
        {"address": "2001:db8::1", "mac_address": "11:22:33:44:55:66", "iface": "Ethernet8", "vlan": "", "status": "REACHABLE"}
    ]
}"#;

/// Mock JSON for SRv6 stats
pub const MOCK_SRV6_STATS: &str = r#"{
    "fc00:0:1::": {"packets": "1000", "bytes": "128000"},
    "fc00:0:2::": {"packets": "500", "bytes": "64000"}
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: mock data — `show lldp table`
pub const SHOW_LLDP_TABLE_OUTPUT: &str = r#"B - Bridge, R - Router, W - WLAN Access Point, S - Station
  LocalPort    RemoteDevice    RemotePortID    Capability    RemotePortDescr
-----------  --------------  --------------  ------------  -----------------
Ethernet0    ARISTA01T1      Ethernet1       BR
Ethernet4    ARISTA02T1      Ethernet1       BR
Ethernet8    ARISTA03T1      Ethernet1       R
--------------------------------------------------
Total entries displayed:  3"#;

/// Source: mock data — `show lldp neighbors`
pub const SHOW_LLDP_NEIGHBORS_OUTPUT: &str = r#"-------------------------------------------------------------------------------
LLDP neighbors::
-------------------------------------------------------------------------------
Interface:    Ethernet0, via: LLDP, RID: 1, Time: 0 day, 03:45:22
  Chassis:     
    ChassisID:    00:11:22:33:44:55
    SysName:      ARISTA01T1
    SysDescr:     Arista Networks EOS version 4.27.3F
    MgmtIP:       10.250.0.51
    Capability:   {"enabled":true,"type":"Bridge"}
    Capability:   {"enabled":true,"type":"Router"}
  Port:        
    PortID:       Ethernet1
    PortDescr:    ARISTA01T1:Ethernet1
    TTL:          120
-------------------------------------------------------------------------------"#;

/// Source: `vlan_test.py` — `show vlan brief`
pub const SHOW_VLAN_BRIEF_OUTPUT: &str = r#"+-----------+----------------+------------+----------------+-------------+-----------------------+
| VLAN ID   | IP Address     | Ports      | Port Tagging   | Proxy ARP   | DHCP Helper Address   |
+===========+================+============+================+=============+=======================+
| 1000      | 192.168.0.1/21 | Ethernet4  | untagged       | disabled    |                       |
|           |                | Ethernet8  | untagged       |             |                       |
|           |                | Ethernet12 | untagged       |             |                       |
|           |                | Ethernet16 | untagged       |             |                       |
+-----------+----------------+------------+----------------+-------------+-----------------------+
| 2000      | 192.168.1.1/24 | Ethernet24 | untagged       | disabled    |                       |
|           |                | Ethernet28 | untagged       |             |                       |
+-----------+----------------+------------+----------------+-------------+-----------------------+"#;

/// Source: `vlan_test.py` — `show vlan brief` (empty)
pub const SHOW_VLAN_BRIEF_EMPTY_OUTPUT: &str = r#"+-----------+--------------+---------+----------------+-------------+-----------------------+
| VLAN ID   | IP Address   | Ports   | Port Tagging   | Proxy ARP   | DHCP Helper Address   |
+===========+==============+=========+================+=============+=======================+
+-----------+--------------+---------+----------------+-------------+-----------------------+"#;

/// Source: `fdbshow_input/fdbshow_masic_test_vectors.py` — `show mac`
pub const SHOW_MAC_TABLE_OUTPUT: &str = r#"  No.    Vlan         MacAddress        Port     Type
-----  ------  -----------------  ----------  -------
    1    1000  00:11:22:33:44:55  Ethernet4   Dynamic
    2    1000  aa:bb:cc:dd:ee:ff  Ethernet8   Static
    3    2000  11:22:33:44:55:66  Ethernet24  Dynamic
Total number of entries 3"#;

/// Source: mock data — `show mac aging_time`
pub const SHOW_MAC_AGING_TIME_OUTPUT: &str = r#"Aging time for switch is 600 seconds"#;

/// Source: mock data — `show ndp`
pub const SHOW_NDP_TABLE_OUTPUT: &str = r#"    Address         MacAddress      Iface    Vlan     Status
-----------  -----------------  ---------  ------  ---------
2001:db8::1  11:22:33:44:55:66  Ethernet8          REACHABLE
fe80::1      00:11:22:33:44:55  Ethernet0          REACHABLE
fe80::2      aa:bb:cc:dd:ee:ff  Ethernet4          STALE
Total number of entries 3"#;

// EXPECTED_BGP_SUMMARY: command not registered (error)

/// Source: mock data — `show srv6 stats`
pub const SHOW_SRV6_STATS_OUTPUT: &str = r#"     MySID    Packets    Bytes
----------  ---------  -------
fc00:0:1::       1000   128000
fc00:0:2::        500    64000"#;
