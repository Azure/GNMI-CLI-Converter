//! Expected CLI output constants for IPv6 test cases.
//!
//! These constants are extracted from sonic-utilities test vectors
//! (show_ip_route_common.py, bgp_network_test_vector.py, bgp_neighbor_test_vector.py)
//! and adapted to match the exact output format of our Rust formatters.

// =============================================================================
// Mock JSON data constants
// =============================================================================

/// Mock JSON for IPv6 route — single-line FRR format (6 routes including kernel, connected, bgp).
pub const IPV6_ROUTE_SINGLE_LINE_JSON: &str = r#"{
    "fc00::/126": [{
        "protocol": "connected",
        "distance": 0,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01w1d03h",
        "nexthops": [{
            "directlyConnected": true,
            "interfaceName": "PortChannel102",
            "active": true,
            "fib": true,
            "linkDown": true,
            "weight": 1
        }]
    }],
    "fc00::4/126": [{
        "protocol": "connected",
        "distance": 0,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01w1d03h",
        "nexthops": [{
            "directlyConnected": true,
            "interfaceName": "PortChannel105",
            "active": true,
            "fib": true,
            "weight": 1
        }]
    }],
    "fe80::/64": [{
        "protocol": "connected",
        "distance": 0,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01w1d03h",
        "nexthops": [{
            "directlyConnected": true,
            "interfaceName": "PortChannel102",
            "active": true,
            "fib": true,
            "weight": 1
        }]
    }],
    "20c0:a8c0:0:80::/64": [{
        "protocol": "bgp",
        "distance": 20,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "00:20:59",
        "nexthops": [{
            "ip": "fc00::2",
            "interfaceName": "PortChannel102",
            "active": true,
            "fib": true,
            "weight": 1
        }]
    }],
    "20c0:a8c8::/64": [{
        "protocol": "bgp",
        "distance": 20,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "00:20:59",
        "nexthops": [{
            "ip": "fc00::6",
            "interfaceName": "PortChannel105",
            "active": true,
            "fib": true,
            "weight": 1
        }]
    }],
    "::/0": [{
        "protocol": "kernel",
        "distance": 0,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01w1d03h",
        "nexthops": [{
            "ip": "fc00::2",
            "interfaceName": "PortChannel102",
            "active": true,
            "fib": true
        }]
    }]
}"#;

/// Expected output for IPv6 route — single-line FRR format.
/// Derived from sonic-utilities show_ipv6_route_expected_output.
pub const IPV6_ROUTE_SINGLE_LINE_EXPECTED: &str = concat!(
    ":\n",
    "Codes: K - kernel route, C - connected, L - local, S - static,\n",
    "       R - RIPng, O - OSPFv3, I - IS-IS, B - BGP, N - NHRP,\n",
    "       T - Table, v - VNC, V - VNC-Direct, A - Babel, F - PBR,\n",
    "       f - OpenFabric, t - Table-Direct,\n",
    "       > - selected route, * - FIB route, q - queued, r - rejected, b - backup\n",
    "       t - trapped, o - offload failure\n",
    "\n",
    "K>* ::/0 [0/0] via fc00::2, PortChannel102, 01w1d03h\n",
    "B>* 20c0:a8c0:0:80::/64 [20/0] via fc00::2, PortChannel102, weight 1, 00:20:59\n",
    "B>* 20c0:a8c8::/64 [20/0] via fc00::6, PortChannel105, weight 1, 00:20:59\n",
    "C>* fc00::/126 is directly connected, PortChannel102 linkdown, weight 1, 01w1d03h\n",
    "C>* fc00::4/126 is directly connected, PortChannel105, weight 1, 01w1d03h\n",
    "C>* fe80::/64 is directly connected, PortChannel102, weight 1, 01w1d03h",
);

/// Mock JSON for IPv6 route — specific prefix query (detailed format).
/// Derived from sonic-utilities show_ipv6_route_multi_asic_specific_route_output.
pub const IPV6_ROUTE_SPECIFIC_PREFIX_JSON: &str = r#"{
    "2603:10e2:400::/128": [{
        "protocol": "connected",
        "distance": 0,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "2d22h00m",
        "nexthops": [{
            "directlyConnected": true,
            "interfaceName": "Loopback4096",
            "active": true,
            "fib": true
        }]
    }]
}"#;

/// Expected output for IPv6 route — specific prefix detailed format.
pub const IPV6_ROUTE_SPECIFIC_PREFIX_EXPECTED: &str = concat!(
    "Routing entry for 2603:10e2:400::/128\n",
    "  Known via \"connected\", distance 0, metric 0, best\n",
    "  Last update 2d22h00m ago\n",
    "  * directly connected, Loopback4096",
);

/// Mock JSON for IPv6 route — BGP via nexthop route.
pub const IPV6_ROUTE_BGP_VIA_JSON: &str = r#"{
    "20c0:a808::/64": [{
        "protocol": "bgp",
        "distance": 20,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01:06:23",
        "nexthops": [{
            "ip": "fc00::72",
            "interfaceName": "Ethernet0",
            "active": true,
            "fib": true,
            "weight": 1
        }]
    }]
}"#;

/// Expected output for IPv6 route — BGP via nexthop.
pub const IPV6_ROUTE_BGP_VIA_EXPECTED: &str = concat!(
    ":\n",
    "Codes: K - kernel route, C - connected, L - local, S - static,\n",
    "       R - RIPng, O - OSPFv3, I - IS-IS, B - BGP, N - NHRP,\n",
    "       T - Table, v - VNC, V - VNC-Direct, A - Babel, F - PBR,\n",
    "       f - OpenFabric, t - Table-Direct,\n",
    "       > - selected route, * - FIB route, q - queued, r - rejected, b - backup\n",
    "       t - trapped, o - offload failure\n",
    "\n",
    "B>* 20c0:a808::/64 [20/0] via fc00::72, Ethernet0, weight 1, 01:06:23",
);

/// Mock JSON for IPv6 route — empty routes.
pub const IPV6_ROUTE_EMPTY_JSON: &str = r#"{}"#;

/// Expected output for IPv6 route — empty routes (just the header).
pub const IPV6_ROUTE_EMPTY_EXPECTED: &str = concat!(
    ":\n",
    "Codes: K - kernel route, C - connected, L - local, S - static,\n",
    "       R - RIPng, O - OSPFv3, I - IS-IS, B - BGP, N - NHRP,\n",
    "       T - Table, v - VNC, V - VNC-Direct, A - Babel, F - PBR,\n",
    "       f - OpenFabric, t - Table-Direct,\n",
    "       > - selected route, * - FIB route, q - queued, r - rejected, b - backup\n",
    "       t - trapped, o - offload failure",
);

/// Mock JSON for IPv6 route — specific prefix with BGP via.
pub const IPV6_ROUTE_SPECIFIC_PREFIX_BGP_JSON: &str = r#"{
    "20c0:a820:0:80::/64": [{
        "protocol": "bgp",
        "distance": 20,
        "metric": 0,
        "selected": true,
        "installed": true,
        "uptime": "01:06:23",
        "nexthops": [{
            "ip": "fc00::72",
            "interfaceName": "Ethernet0",
            "active": true,
            "fib": true
        }]
    }]
}"#;

/// Expected output for IPv6 route — specific prefix with BGP via.
pub const IPV6_ROUTE_SPECIFIC_PREFIX_BGP_EXPECTED: &str = concat!(
    "Routing entry for 20c0:a820:0:80::/64\n",
    "  Known via \"bgp\", distance 20, metric 0, best\n",
    "  Last update 01:06:23 ago\n",
    "  * via fc00::72, Ethernet0",
);

// =============================================================================
// IPv6 BGP Summary
// =============================================================================

/// Mock JSON for IPv6 BGP summary with 2 peers.
/// Derived from sonic-utilities BGP summary test data.
pub const IPV6_BGP_SUMMARY_JSON: &str = r#"{
    "routerId": "10.1.0.32",
    "localAs": 65100,
    "vrfId": 0,
    "tableVersion": 12800,
    "ribCount": 12809,
    "ribMemory": 2427712,
    "peerCount": 4,
    "peerMemory": 82720,
    "peerGroupCount": 2,
    "peerGroupMemory": 128,
    "peers": {
        "fc00::72": {
            "hostname": "ARISTA01T1",
            "remoteAs": 64600,
            "localAs": 65100,
            "version": 4,
            "msgRcvd": 3202,
            "msgSent": 3206,
            "tableVersion": 0,
            "outq": 0,
            "inq": 0,
            "peerUptime": "01:06:23",
            "peerUptimeMsec": 3983000,
            "pfxRcd": 6400,
            "state": "Established",
            "neighborName": "ARISTA01T1"
        },
        "fc00::76": {
            "hostname": "ARISTA03T1",
            "remoteAs": 64600,
            "localAs": 65100,
            "version": 4,
            "msgRcvd": 3202,
            "msgSent": 3206,
            "tableVersion": 0,
            "outq": 0,
            "inq": 0,
            "peerUptime": "01:06:23",
            "peerUptimeMsec": 3983000,
            "pfxRcd": 6400,
            "state": "Established",
            "neighborName": "ARISTA03T1"
        }
    }
}"#;

/// Expected output for IPv6 BGP summary with 2 peers.
pub const IPV6_BGP_SUMMARY_EXPECTED: &str = concat!(
    "\n",
    "IPv6 Unicast Summary:\n",
    "BGP router identifier 10.1.0.32, local AS number 65100 vrf-id 0\n",
    "BGP table version 12800\n",
    "RIB entries 12809, using 2427712 bytes of memory\n",
    "Peers 4, using 82720 KiB of memory\n",
    "Peer groups 2, using 128 bytes of memory\n",
    "\n",
    "\n",
    "Neighbhor      V     AS    MsgRcvd    MsgSent    TblVer    InQ    OutQ  Up/Down      State/PfxRcd  NeighborName\n",
    "-----------  ---  -----  ---------  ---------  --------  -----  ------  ---------  --------------  --------------\n",
    "fc00::72       4  64600       3202       3206         0      0       0  01:06:23             6400  ARISTA01T1\n",
    "fc00::76       4  64600       3202       3206         0      0       0  01:06:23             6400  ARISTA03T1",
);

// =============================================================================
// IPv6 BGP Network (passthrough)
// =============================================================================

/// Mock JSON for IPv6 BGP network — passthrough output.
/// Derived from sonic-utilities bgp_v6_network.
pub const IPV6_BGP_NETWORK_JSON: &str = r#"{"output": "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\nDefault local pref 100, local AS 65100"}"#;

/// Expected output for IPv6 BGP network — passthrough.
pub const IPV6_BGP_NETWORK_EXPECTED: &str = concat!(
    "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\n",
    "Default local pref 100, local AS 65100",
);

/// Mock JSON for IPv6 BGP network — extended table passthrough.
/// Derived from sonic-utilities bgp_v6_network with full routing table.
pub const IPV6_BGP_NETWORK_FULL_TABLE_JSON: &str = r#"{"output": "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\nDefault local pref 100, local AS 65100\nStatus codes:  s suppressed, d damped, h history, * valid, > best, = multipath,\n               i internal, r RIB-failure, S Stale, R Removed\nOrigin codes:  i - IGP, e - EGP, ? - incomplete\n\n   Network          Next Hop            Metric LocPrf Weight Path\n*> 2064:100::1d/128 fc00::72                               0 64600 i"}"#;

/// Expected output for IPv6 BGP network — extended table passthrough.
pub const IPV6_BGP_NETWORK_FULL_TABLE_EXPECTED: &str = concat!(
    "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\n",
    "Default local pref 100, local AS 65100\n",
    "Status codes:  s suppressed, d damped, h history, * valid, > best, = multipath,\n",
    "               i internal, r RIB-failure, S Stale, R Removed\n",
    "Origin codes:  i - IGP, e - EGP, ? - incomplete\n",
    "\n",
    "   Network          Next Hop            Metric LocPrf Weight Path\n",
    "*> 2064:100::1d/128 fc00::72                               0 64600 i",
);

/// Mock JSON for IPv6 BGP network — IP address detail passthrough.
/// Derived from sonic-utilities bgp_v6_network_ip_address.
pub const IPV6_BGP_NETWORK_IP_ADDR_JSON: &str = r#"{"output": "BGP routing table entry for 20c0:a820:0:80::/64\nPaths: (4 available, best #4, table default)\n  Advertised to non peer-group peers:\n  fc00::72 fc00::76 fc00::7a fc00::7e\n  64600 65504\n    fc00::72 from fc00::72 (100.1.0.29)\n      Origin IGP, valid, external, multipath, best (Router ID)\n      Community: 5060:12345\n      Last update: Tue Apr 20 05:54:16 2021"}"#;

/// Expected output for IPv6 BGP network — IP address detail passthrough.
pub const IPV6_BGP_NETWORK_IP_ADDR_EXPECTED: &str = concat!(
    "BGP routing table entry for 20c0:a820:0:80::/64\n",
    "Paths: (4 available, best #4, table default)\n",
    "  Advertised to non peer-group peers:\n",
    "  fc00::72 fc00::76 fc00::7a fc00::7e\n",
    "  64600 65504\n",
    "    fc00::72 from fc00::72 (100.1.0.29)\n",
    "      Origin IGP, valid, external, multipath, best (Router ID)\n",
    "      Community: 5060:12345\n",
    "      Last update: Tue Apr 20 05:54:16 2021",
);

// =============================================================================
// IPv6 Prefix List
// =============================================================================

/// Mock JSON for IPv6 prefix list with 3 entries.
pub const IPV6_PREFIX_LIST_JSON: &str = r#"{
    "ZEBRA": {
        "PL_ALLOW_LIST_DEPLOYMENT_ID_0_COMMUNITY_empty_V6": {
            "entries": [
                {
                    "sequenceNumber": 10,
                    "type": "permit",
                    "prefix": "::/0",
                    "minimumPrefixLength": "1",
                    "maximumPrefixLength": "64"
                },
                {
                    "sequenceNumber": 20,
                    "type": "deny",
                    "prefix": "::/0",
                    "minimumPrefixLength": "",
                    "maximumPrefixLength": "128"
                },
                {
                    "sequenceNumber": 30,
                    "type": "permit",
                    "prefix": "::/0",
                    "minimumPrefixLength": "",
                    "maximumPrefixLength": ""
                }
            ]
        }
    }
}"#;

/// Expected output for IPv6 prefix list with 3 entries.
pub const IPV6_PREFIX_LIST_EXPECTED: &str = concat!(
    "ZEBRA: ipv6 prefix-list PL_ALLOW_LIST_DEPLOYMENT_ID_0_COMMUNITY_empty_V6: 3 entries\n",
    "   seq 10 permit ::/0 ge 1 le 64\n",
    "   seq 20 deny ::/0 le 128\n",
    "   seq 30 permit ::/0",
);

/// Mock JSON for IPv6 prefix list with multiple protocols and lists.
pub const IPV6_PREFIX_LIST_MULTI_JSON: &str = r#"{
    "ZEBRA": {
        "PL_V6_LIST_A": {
            "entries": [
                {
                    "sequenceNumber": 5,
                    "type": "permit",
                    "prefix": "fc00::/64",
                    "minimumPrefixLength": "",
                    "maximumPrefixLength": ""
                }
            ]
        }
    },
    "BGP": {
        "PL_V6_BGP_FILTER": {
            "entries": [
                {
                    "sequenceNumber": 10,
                    "type": "deny",
                    "prefix": "::/0",
                    "minimumPrefixLength": "",
                    "maximumPrefixLength": "128"
                },
                {
                    "sequenceNumber": 20,
                    "type": "permit",
                    "prefix": "2001:db8::/32",
                    "minimumPrefixLength": "48",
                    "maximumPrefixLength": "64"
                }
            ]
        }
    }
}"#;

/// Expected output for IPv6 prefix list with multiple protocols.
pub const IPV6_PREFIX_LIST_MULTI_EXPECTED: &str = concat!(
    "ZEBRA: ipv6 prefix-list PL_V6_LIST_A: 1 entries\n",
    "   seq 5 permit fc00::/64\n",
    "BGP: ipv6 prefix-list PL_V6_BGP_FILTER: 2 entries\n",
    "   seq 10 deny ::/0 le 128\n",
    "   seq 20 permit 2001:db8::/32 ge 48 le 64",
);

// =============================================================================
// IPv6 Protocol
// =============================================================================

/// Mock JSON for IPv6 protocol with 4 protocols.
pub const IPV6_PROTOCOL_JSON: &str = r#"[{
    "VRF": "default",
    "Protocols": [
        {"Protocol": "system", "route-map": "none"},
        {"Protocol": "connected", "route-map": "none"},
        {"Protocol": "static", "route-map": "none"},
        {"Protocol": "bgp", "route-map": "RM_SET_SRC6"}
    ]
}]"#;

/// Expected output for IPv6 protocol with 4 protocols.
pub const IPV6_PROTOCOL_EXPECTED: &str = concat!(
    "VRF: default\n",
    "Protocol                  : route-map\n",
    "-------------------------------------\n",
    "system                    : none\n",
    "connected                 : none\n",
    "static                    : none\n",
    "bgp                       : RM_SET_SRC6",
);

/// Mock JSON for IPv6 protocol with multiple VRFs.
pub const IPV6_PROTOCOL_MULTI_VRF_JSON: &str = r#"[
    {
        "VRF": "default",
        "Protocols": [
            {"Protocol": "connected", "route-map": "none"},
            {"Protocol": "bgp", "route-map": "RM_SET_SRC6"}
        ]
    },
    {
        "VRF": "Vrf_mgmt",
        "Protocols": [
            {"Protocol": "connected", "route-map": "none"},
            {"Protocol": "static", "route-map": "none"}
        ]
    }
]"#;

/// Expected output for IPv6 protocol with multiple VRFs.
pub const IPV6_PROTOCOL_MULTI_VRF_EXPECTED: &str = concat!(
    "VRF: default\n",
    "Protocol                  : route-map\n",
    "-------------------------------------\n",
    "connected                 : none\n",
    "bgp                       : RM_SET_SRC6\n",
    "VRF: Vrf_mgmt\n",
    "Protocol                  : route-map\n",
    "-------------------------------------\n",
    "connected                 : none\n",
    "static                    : none",
);

// =============================================================================
// IPv6 FIB
// =============================================================================

/// Mock JSON for IPv6 FIB with 5 entries.
pub const IPV6_FIB_JSON: &str = r#"{
    "entries": [
        {"index": "1", "vrf": "default", "route": "::/0", "nexthop": "fc00::2", "ifname": "PortChannel102"},
        {"index": "2", "vrf": "default", "route": "fc00::/126", "nexthop": "::", "ifname": "PortChannel102"},
        {"index": "3", "vrf": "default", "route": "fc00::4/126", "nexthop": "::", "ifname": "PortChannel105"},
        {"index": "4", "vrf": "default", "route": "20c0:a8c0:0:80::/64", "nexthop": "fc00::2", "ifname": "PortChannel102"},
        {"index": "5", "vrf": "default", "route": "20c0:a8c8::/64", "nexthop": "fc00::6", "ifname": "PortChannel105"}
    ],
    "total": "5"
}"#;

/// Expected output for IPv6 FIB with 5 entries.
pub const IPV6_FIB_EXPECTED: &str = concat!(
    "  No.  Vrf      Route                Nexthop    Ifname\n",
    "-----  -------  -------------------  ---------  --------------\n",
    "    1  default  ::/0                 fc00::2    PortChannel102\n",
    "    2  default  fc00::/126           ::         PortChannel102\n",
    "    3  default  fc00::4/126          ::         PortChannel105\n",
    "    4  default  20c0:a8c0:0:80::/64  fc00::2    PortChannel102\n",
    "    5  default  20c0:a8c8::/64       fc00::6    PortChannel105\n",
    "Total number of entries 5",
);

/// Mock JSON for IPv6 FIB with a single entry.
pub const IPV6_FIB_SINGLE_JSON: &str = r#"{
    "entries": [
        {"index": "1", "vrf": "default", "route": "fc00:1::32/128", "nexthop": "::", "ifname": "Loopback0"}
    ],
    "total": "1"
}"#;

/// Expected output for IPv6 FIB with a single entry.
pub const IPV6_FIB_SINGLE_EXPECTED: &str = concat!(
    "  No.  Vrf      Route           Nexthop    Ifname\n",
    "-----  -------  --------------  ---------  ---------\n",
    "    1  default  fc00:1::32/128  ::         Loopback0\n",
    "Total number of entries 1",
);

// =============================================================================
// IPv6 Interfaces
// =============================================================================

/// Mock JSON for IPv6 interfaces with 2 interfaces and link-local addresses.
pub const IPV6_INTERFACES_JSON: &str = r#"{
    "Ethernet0": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {
                "address": "fc00::71/126",
                "bgp_neighbor_name": "ARISTA01T1",
                "bgp_neighbor_ip": "fc00::72"
            },
            {
                "address": "fe80::5054:ff:fea9:41c2/64",
                "bgp_neighbor_name": "",
                "bgp_neighbor_ip": ""
            }
        ]
    },
    "Loopback0": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {
                "address": "fc00:1::32/128",
                "bgp_neighbor_name": "",
                "bgp_neighbor_ip": ""
            }
        ]
    }
}"#;

/// Expected output for IPv6 interfaces.
/// Note: fe80::/64 link-local addresses get formatted with %interface suffix.
pub const IPV6_INTERFACES_EXPECTED: &str = concat!(
    "Interface    Master    IPv6 address/mask                     Admin/Oper    BGP Neighbor    Neighbor IP\n",
    "-----------  --------  ------------------------------------  ------------  --------------  -------------\n",
    "Ethernet0              fc00::71/126                          up/up         ARISTA01T1      fc00::72\n",
    "                       fe80::5054:ff:fea9:41c2%Ethernet0/64\n",
    "Loopback0              fc00:1::32/128                        up/up",
);

/// Mock JSON for IPv6 interfaces with single interface.
pub const IPV6_INTERFACES_SINGLE_JSON: &str = r#"{
    "PortChannel102": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {
                "address": "fc00::1/126",
                "bgp_neighbor_name": "ARISTA01T1",
                "bgp_neighbor_ip": "fc00::2"
            }
        ]
    }
}"#;

/// Expected output for IPv6 interfaces — single interface.
pub const IPV6_INTERFACES_SINGLE_EXPECTED: &str = concat!(
    "Interface       Master    IPv6 address/mask    Admin/Oper    BGP Neighbor    Neighbor IP\n",
    "--------------  --------  -------------------  ------------  --------------  -------------\n",
    "PortChannel102            fc00::1/126          up/up         ARISTA01T1      fc00::2",
);

// =============================================================================
// IPv6 Link-Local Mode
// =============================================================================

/// Mock JSON for IPv6 link-local mode with 3 ports.
pub const IPV6_LINK_LOCAL_MODE_JSON: &str = r#"[
    {"port": "Ethernet0", "mode": "-"},
    {"port": "Ethernet4", "mode": "-"},
    {"port": "PortChannel102", "mode": "-"}
]"#;

/// Expected output for IPv6 link-local mode as grid table.
pub const IPV6_LINK_LOCAL_MODE_EXPECTED: &str = concat!(
    "+------------------+--------+\n",
    "| Interface Name   | Mode   |\n",
    "+==================+========+\n",
    "| Ethernet0        | -      |\n",
    "+------------------+--------+\n",
    "| Ethernet4        | -      |\n",
    "+------------------+--------+\n",
    "| PortChannel102   | -      |\n",
    "+------------------+--------+",
);

/// Mock JSON for IPv6 link-local mode with enabled mode.
pub const IPV6_LINK_LOCAL_MODE_ENABLED_JSON: &str = r#"[
    {"port": "Ethernet0", "mode": "enabled"},
    {"port": "Ethernet4", "mode": "-"}
]"#;

/// Expected output for IPv6 link-local mode with enabled mode.
pub const IPV6_LINK_LOCAL_MODE_ENABLED_EXPECTED: &str = concat!(
    "+------------------+---------+\n",
    "| Interface Name   | Mode    |\n",
    "+==================+=========+\n",
    "| Ethernet0        | enabled |\n",
    "+------------------+---------+\n",
    "| Ethernet4        | -       |\n",
    "+------------------+---------+",
);

// =============================================================================
// IPv6 BGP Neighbors — Detail View
// =============================================================================

/// Mock JSON for IPv6 BGP neighbors detail (1 neighbor).
/// Derived from sonic-utilities bgp_v6_neighbors_output.
pub const IPV6_BGP_NEIGHBORS_DETAIL_JSON: &str = r#"{
    "fc00::72": {
        "remoteAs": 64600,
        "localAs": 65100,
        "bgpVersion": 4,
        "remoteRouterId": "100.1.0.29",
        "localRouterId": "10.1.0.32",
        "bgpState": "Established",
        "neighborName": "ARISTA01T1",
        "bgpTimerUpMsec": 3983000,
        "bgpTimerUpString": "01:06:23",
        "peerGroup": "PEER_V6",
        "bgpTimerHoldTimeMsecs": 10000,
        "bgpTimerKeepAliveIntervalMsecs": 3000,
        "bgpTimerConfiguredHoldTimeMsecs": 10000,
        "bgpTimerConfiguredKeepAliveIntervalMsecs": 3000,
        "neighborCapabilities": {
            "4byteAs": "advertisedAndReceived",
            "addPath": {
                "ipv6Unicast": {"rxAdvertisedAndReceived": true}
            },
            "routeRefresh": "advertisedAndReceivedNew",
            "multiprotocolExtensions": {
                "ipv6Unicast": {"advertisedAndReceived": true}
            },
            "hostName": {
                "advHostName": "vlab-01",
                "advDomainName": "n/a",
                "rcvHostNameEmpty": true
            },
            "gracefulRestart": "advertisedAndReceived",
            "gracefulRestartRemoteTimerMsecs": 300000,
            "gracefulRestartAddressFamiliesByPeer": "none"
        },
        "gracefulRestartInfo": {
            "endOfRibSend": {"ipv6Unicast": true},
            "endOfRibRecv": {"ipv6Unicast": true},
            "localGrMode": "Restart*",
            "remoteGrMode": "Helper",
            "rBit": false,
            "timers": {
                "configuredRestartTimerMsecs": 240000,
                "receivedRestartTimerMsecs": 300000
            },
            "ipv6Unicast": {
                "fBit": false,
                "endOfRibStatus": {
                    "endOfRibSend": true,
                    "endOfRibSendAfterUpdate": false,
                    "endOfRibRecv": true
                },
                "timers": {
                    "stalePathTimerMsecs": 360000,
                    "selectionDeferralTimerMsecs": 360000
                }
            }
        },
        "messageStats": {
            "depthInq": 0,
            "depthOutq": 0,
            "opensSent": 1,
            "opensRecv": 1,
            "notificationsSent": 0,
            "notificationsRecv": 0,
            "updatesSent": 3206,
            "updatesRecv": 3202,
            "keepalivesSent": 1328,
            "keepalivesRecv": 1329,
            "routeRefreshSent": 0,
            "routeRefreshRecv": 0,
            "capabilitySent": 0,
            "capabilityRecv": 0,
            "totalSent": 4535,
            "totalRecv": 4532
        },
        "minTimeBtwnAdvertisementRunsMsecs": 0,
        "addressFamilyInfo": {
            "ipv6Unicast": {
                "peerGroup": "PEER_V6",
                "updateGroup": 2,
                "subGroup": 2,
                "packetQueueLength": 0,
                "commAttriSentToNbr": "extendedAndStandard",
                "inboundSoftReconfig": true,
                "inboundPathPolicyConfig": true,
                "outboundPathPolicyConfig": true,
                "incomingRouteMap": "FROM_BGP_PEER_V6",
                "outgoingRouteMap": "TO_BGP_PEER_V6",
                "acceptedPrefixCounter": 6400
            }
        },
        "connectionsEstablished": 1,
        "connectionsDropped": 0,
        "lastResetTimerMsecs": 4006000,
        "lastResetDueTo": "Waiting for peer OPEN",
        "hostLocal": "fc00::71",
        "portLocal": 59726,
        "hostForeign": "fc00::72",
        "portForeign": 179,
        "nexthop": "10.0.0.56",
        "nexthopGlobal": "fc00::71",
        "nexthopLocal": "fe80::5054:ff:fea9:41c2",
        "connectRetryTimer": 10,
        "estimatedRoundTripTime": 4,
        "readThread": "on",
        "writeThread": "on",
        "fdUsed": 30,
        "bgpConnection": "shared network"
    }
}"#;

/// Key substrings expected in IPv6 BGP neighbors detail output.
/// Full output is complex with dynamic formatting, so we check key sections.
pub const IPV6_BGP_NEIGHBORS_DETAIL_EXPECTED_SUBSTRINGS: &[&str] = &[
    "BGP neighbor is fc00::72, remote AS 64600, local AS 65100",
    "Member of peer-group PEER_V6 for session parameters",
    "BGP version 4, remote router ID 100.1.0.29, local router ID 10.1.0.32",
    "BGP state = Established, up for 01:06:23",
    "Hold time is 10 seconds, keepalive interval is 3 seconds",
    "4 Byte AS: advertised and received",
    "IPv6 Unicast: RX advertised and received",
    "Graceful Restart Capability: advertised and received",
    "Remote Restart timer is 300 seconds",
    "End-of-RIB send: IPv6 Unicast",
    "End-of-RIB received: IPv6 Unicast",
    "Local GR Mode: Restart*",
    "Remote GR Mode: Helper",
    "R bit: False",
    "Opens:",
    "Updates:",
    "Keepalives:",
    "Total:",
    "6400 accepted prefixes",
    "Connections established 1; dropped 0",
    "Local host: fc00::71, Local port: 59726",
    "Foreign host: fc00::72, Foreign port: 179",
    "Nexthop: 10.0.0.56",
    "Nexthop global: fc00::71",
    "Nexthop local: fe80::5054:ff:fea9:41c2",
    "BGP connection: shared network",
];

// =============================================================================
// IPv6 BGP Neighbors — Advertised Routes View
// =============================================================================

/// Mock JSON for IPv6 BGP neighbors advertised routes.
/// Derived from sonic-utilities bgp_v6_neighbor_output_adv_routes.
pub const IPV6_BGP_NEIGHBORS_ADV_ROUTES_JSON: &str = r#"{
    "bgpTableVersion": 6407,
    "bgpLocalRouterId": "10.1.0.32",
    "localAS": 65100,
    "vrfId": 0,
    "defaultLocPrf": 100,
    "advertisedRoutes": {
        "::/0": {
            "network": "::/0",
            "nextHop": "::",
            "metric": "",
            "locPrf": "",
            "weight": 0,
            "path": "64600 65534 6666 6667 i"
        },
        "2064:100::1d/128": {
            "network": "2064:100::1d/128",
            "nextHop": "::",
            "metric": "",
            "locPrf": "",
            "weight": 0,
            "path": "64600 i"
        },
        "20c0:a808::/64": {
            "network": "20c0:a808::/64",
            "nextHop": "::",
            "metric": "",
            "locPrf": "",
            "weight": 0,
            "path": "64600 65501 i"
        }
    }
}"#;

/// Expected output for IPv6 BGP neighbors advertised routes.
pub const IPV6_BGP_NEIGHBORS_ADV_ROUTES_EXPECTED: &str = concat!(
    "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\n",
    "Default local pref 100, local AS 65100\n",
    "Status codes:  s suppressed, d damped, h history, u unsorted, * valid, > best, = multipath,\n",
    "               i internal, r RIB-failure, S Stale, R Removed\n",
    "Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self\n",
    "Origin codes:  i - IGP, e - EGP, ? - incomplete\n",
    "RPKI validation codes: V valid, I invalid, N Not found\n",
    "\n",
    "    Network          Next Hop            Metric LocPrf Weight Path\n",
    "    ::/0             ::                                     0 64600 65534 6666 6667 i\n",
    "    2064:100::1d/128 ::                                     0 64600 i\n",
    "    20c0:a808::/64   ::                                     0 64600 65501 i\n",
    "\n",
    "Displayed 3 routes",
);

// =============================================================================
// IPv6 BGP Neighbors — Received Routes View
// =============================================================================

/// Mock JSON for IPv6 BGP neighbors received routes.
/// Derived from sonic-utilities bgp_v6_neighbor_output_recv_routes.
pub const IPV6_BGP_NEIGHBORS_RECV_ROUTES_JSON: &str = r#"{
    "bgpTableVersion": 6407,
    "bgpLocalRouterId": "10.1.0.32",
    "localAS": 65100,
    "vrfId": 0,
    "defaultLocPrf": 100,
    "receivedRoutes": {
        "::/0": {
            "network": "::/0",
            "nextHop": "fc00::72",
            "metric": "",
            "locPrf": "",
            "weight": 0,
            "path": "64600 65534 6666 6667 i"
        },
        "2064:100::1d/128": {
            "network": "2064:100::1d/128",
            "nextHop": "fc00::72",
            "metric": "",
            "locPrf": "",
            "weight": 0,
            "path": "64600 i"
        }
    }
}"#;

/// Expected output for IPv6 BGP neighbors received routes.
pub const IPV6_BGP_NEIGHBORS_RECV_ROUTES_EXPECTED: &str = concat!(
    "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\n",
    "Default local pref 100, local AS 65100\n",
    "Status codes:  s suppressed, d damped, h history, u unsorted, * valid, > best, = multipath,\n",
    "               i internal, r RIB-failure, S Stale, R Removed\n",
    "Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self\n",
    "Origin codes:  i - IGP, e - EGP, ? - incomplete\n",
    "RPKI validation codes: V valid, I invalid, N Not found\n",
    "\n",
    "    Network          Next Hop            Metric LocPrf Weight Path\n",
    "    ::/0             fc00::72                               0 64600 65534 6666 6667 i\n",
    "    2064:100::1d/128 fc00::72                               0 64600 i\n",
    "\n",
    "Displayed 2 routes",
);
