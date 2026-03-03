//! Raw expected CLI output and mock JSON input data for interfaces formatters.
//!
//! Data extracted from sonic-utilities test vectors:
//!   - interfaces_test.py
//!
//! These constants represent the expected CLI output from SONiC show commands.
//! They are used as reference data for verifying formatter correctness.

/// Source: `interfaces_test.py` variable `show_interfaces_alias_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_ALIAS_OUTPUT: &str = r#"Name         Alias
-----------  -------
Ethernet0    etp1
Ethernet4    etp2
Ethernet8    etp3
Ethernet12   etp4
Ethernet16   etp5
Ethernet20   etp6
Ethernet24   etp7
Ethernet28   etp8
Ethernet32   etp9
Ethernet36   etp10
Ethernet40   etp11
Ethernet44   etp12
Ethernet48   etp13
Ethernet52   etp14
Ethernet56   etp15
Ethernet60   etp16
Ethernet64   etp17
Ethernet68   etp18
Ethernet72   etp19
Ethernet76   etp20
Ethernet80   etp21
Ethernet84   etp22
Ethernet88   etp23
Ethernet92   etp24
Ethernet96   etp25
Ethernet100  etp26
Ethernet104  etp27
Ethernet108  etp28
Ethernet112  etp29
Ethernet116  etp30
Ethernet120  etp31
Ethernet124  etp32
"#;

/// Source: `interfaces_test.py` variable `show_interfaces_alias_Ethernet0_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_ALIAS_ETHERNET0_OUTPUT: &str = r#"Name       Alias
---------  -------
Ethernet0  etp1
"#;

/// Source: `interfaces_test.py` variable `show_interfaces_neighbor_expected_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_NEIGHBOR_EXPECTED_OUTPUT: &str = r#"LocalPort    Neighbor    NeighborPort    NeighborLoopback    NeighborMgmt    NeighborType
-----------  ----------  --------------  ------------------  --------------  --------------
Ethernet112  ARISTA01T1  Ethernet1       None                10.250.0.51     LeafRouter
Ethernet116  ARISTA02T1  Ethernet1       None                10.250.0.52     LeafRouter
Ethernet120  ARISTA03T1  Ethernet1       None                10.250.0.53     LeafRouter
Ethernet124  ARISTA04T1  Ethernet1       None                10.250.0.54     LeafRouter
"#;

/// Source: `interfaces_test.py` variable `show_interfaces_portchannel_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_PORTCHANNEL_OUTPUT: &str = r#"Flags: A - active, I - inactive, Up - up, Dw - Down, N/A - not available,
       S - selected, D - deselected, * - not synced
  No.  Team Dev         Protocol     Ports
-----  ---------------  -----------  --------------
 0001  PortChannel0001  LACP(A)(Dw)  Ethernet112(D)
 0002  PortChannel0002  LACP(A)(Up)  Ethernet116(S)
 0003  PortChannel0003  LACP(A)(Up)  Ethernet120(S)
 0004  PortChannel0004  LACP(A)(Up)  N/A
 1001  PortChannel1001  N/A
"#;

/// Source: `interfaces_test.py` variable `show_interfaces_switchport_status_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_SWITCHPORT_STATUS_OUTPUT: &str = r#"Interface        Mode
---------------  ------
Ethernet0        routed
Ethernet4        trunk
Ethernet8        routed
Ethernet12       routed
Ethernet16       trunk
Ethernet20       routed
Ethernet24       trunk
Ethernet28       trunk
Ethernet36       routed
Ethernet40       routed
Ethernet44       routed
Ethernet48       routed
Ethernet52       routed
Ethernet56       routed
Ethernet60       routed
Ethernet64       routed
Ethernet68       routed
Ethernet72       routed
Ethernet76       routed
Ethernet80       routed
Ethernet84       routed
Ethernet88       routed
Ethernet92       routed
Ethernet96       routed
Ethernet100      routed
Ethernet104      routed
Ethernet108      routed
Ethernet116      routed
Ethernet124      routed
PortChannel0001  routed
PortChannel0002  routed
PortChannel0003  routed
PortChannel0004  routed
PortChannel1001  trunk
"#;

/// Source: `interfaces_test.py` variable `show_interfaces_switchport_config_output`
#[allow(dead_code)]
pub const SHOW_INTERFACES_SWITCHPORT_CONFIG_OUTPUT: &str = r#"Interface        Mode    Untagged    Tagged
---------------  ------  ----------  --------
Ethernet0        routed
Ethernet4        trunk   1000
Ethernet8        routed  1000
Ethernet12       routed  1000
Ethernet16       trunk   1000
Ethernet20       routed
Ethernet24       trunk   2000
Ethernet28       trunk   2000
Ethernet36       routed
Ethernet40       routed
Ethernet44       routed
Ethernet48       routed
Ethernet52       routed
Ethernet56       routed
Ethernet60       routed
Ethernet64       routed
Ethernet68       routed
Ethernet72       routed
Ethernet76       routed
Ethernet80       routed
Ethernet84       routed
Ethernet88       routed
Ethernet92       routed
Ethernet96       routed
Ethernet100      routed
Ethernet104      routed
Ethernet108      routed
Ethernet116      routed
Ethernet124      routed
PortChannel0001  routed
PortChannel0002  routed
PortChannel0003  routed
PortChannel0004  routed
PortChannel1001  trunk               4000
"#;

// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Source: interfaces_test.py `show_interfaces_alias_output`
pub const MOCK_INTERFACES_ALIAS: &str = r#"[
    {"name": "Ethernet0", "alias": "etp1"},
    {"name": "Ethernet4", "alias": "etp2"},
    {"name": "Ethernet8", "alias": "etp3"},
    {"name": "Ethernet12", "alias": "etp4"},
    {"name": "Ethernet16", "alias": "etp5"},
    {"name": "Ethernet20", "alias": "etp6"},
    {"name": "Ethernet24", "alias": "etp7"},
    {"name": "Ethernet28", "alias": "etp8"},
    {"name": "Ethernet32", "alias": "etp9"},
    {"name": "Ethernet36", "alias": "etp10"},
    {"name": "Ethernet40", "alias": "etp11"},
    {"name": "Ethernet44", "alias": "etp12"},
    {"name": "Ethernet48", "alias": "etp13"},
    {"name": "Ethernet52", "alias": "etp14"},
    {"name": "Ethernet56", "alias": "etp15"},
    {"name": "Ethernet60", "alias": "etp16"},
    {"name": "Ethernet64", "alias": "etp17"},
    {"name": "Ethernet68", "alias": "etp18"},
    {"name": "Ethernet72", "alias": "etp19"},
    {"name": "Ethernet76", "alias": "etp20"},
    {"name": "Ethernet80", "alias": "etp21"},
    {"name": "Ethernet84", "alias": "etp22"},
    {"name": "Ethernet88", "alias": "etp23"},
    {"name": "Ethernet92", "alias": "etp24"},
    {"name": "Ethernet96", "alias": "etp25"},
    {"name": "Ethernet100", "alias": "etp26"},
    {"name": "Ethernet104", "alias": "etp27"},
    {"name": "Ethernet108", "alias": "etp28"},
    {"name": "Ethernet112", "alias": "etp29"},
    {"name": "Ethernet116", "alias": "etp30"},
    {"name": "Ethernet120", "alias": "etp31"},
    {"name": "Ethernet124", "alias": "etp32"}
]"#;

/// Mock JSON for `show interfaces alias` — single interface
/// Source: interfaces_test.py `show_interfaces_alias_Ethernet0_output`
pub const MOCK_INTERFACES_ALIAS_ETH0: &str = r#"[
    {"name": "Ethernet0", "alias": "etp1"}
]"#;

/// Mock JSON for `show interfaces neighbor expected`
/// Source: interfaces_test.py `show_interfaces_neighbor_expected_output`
pub const MOCK_INTERFACES_NEIGHBOR: &str = r#"[
    {"LocalPort": "Ethernet112", "Neighbor": "ARISTA01T1", "NeighborPort": "Ethernet1", "NeighborLoopback": "None", "NeighborMgmt": "10.250.0.51", "NeighborType": "LeafRouter"},
    {"LocalPort": "Ethernet116", "Neighbor": "ARISTA02T1", "NeighborPort": "Ethernet1", "NeighborLoopback": "None", "NeighborMgmt": "10.250.0.52", "NeighborType": "LeafRouter"},
    {"LocalPort": "Ethernet120", "Neighbor": "ARISTA03T1", "NeighborPort": "Ethernet1", "NeighborLoopback": "None", "NeighborMgmt": "10.250.0.53", "NeighborType": "LeafRouter"},
    {"LocalPort": "Ethernet124", "Neighbor": "ARISTA04T1", "NeighborPort": "Ethernet1", "NeighborLoopback": "None", "NeighborMgmt": "10.250.0.54", "NeighborType": "LeafRouter"}
]"#;

/// Mock JSON for `show interfaces portchannel`
/// Source: interfaces_test.py `show_interfaces_portchannel_output`
pub const MOCK_INTERFACES_PORTCHANNEL: &str = r#"{
    "0001": {
        "Team Dev": "PortChannel0001",
        "Protocol": {"name": "LACP", "active": true, "operational_status": "down"},
        "Ports": [{"name": "Ethernet112", "selected": false}]
    },
    "0002": {
        "Team Dev": "PortChannel0002",
        "Protocol": {"name": "LACP", "active": true, "operational_status": "up"},
        "Ports": [{"name": "Ethernet116", "selected": true}]
    },
    "0003": {
        "Team Dev": "PortChannel0003",
        "Protocol": {"name": "LACP", "active": true, "operational_status": "up"},
        "Ports": [{"name": "Ethernet120", "selected": true}]
    },
    "0004": {
        "Team Dev": "PortChannel0004",
        "Protocol": {"name": "LACP", "active": true, "operational_status": "up"},
        "Ports": "N/A"
    },
    "1001": {
        "Team Dev": "PortChannel1001",
        "Protocol": "N/A"
    }
}"#;

/// Mock JSON for `show interfaces switchport status`
/// Source: interfaces_test.py `show_interfaces_switchport_status_output`
pub const MOCK_SWITCHPORT_STATUS: &str = r#"[
    {"interface": "Ethernet0", "mode": "routed"},
    {"interface": "Ethernet4", "mode": "trunk"},
    {"interface": "Ethernet8", "mode": "routed"},
    {"interface": "Ethernet12", "mode": "routed"},
    {"interface": "Ethernet16", "mode": "trunk"},
    {"interface": "Ethernet20", "mode": "routed"},
    {"interface": "Ethernet24", "mode": "trunk"},
    {"interface": "Ethernet28", "mode": "trunk"},
    {"interface": "Ethernet36", "mode": "routed"},
    {"interface": "Ethernet40", "mode": "routed"},
    {"interface": "Ethernet44", "mode": "routed"},
    {"interface": "Ethernet48", "mode": "routed"},
    {"interface": "Ethernet52", "mode": "routed"},
    {"interface": "Ethernet56", "mode": "routed"},
    {"interface": "Ethernet60", "mode": "routed"},
    {"interface": "Ethernet64", "mode": "routed"},
    {"interface": "Ethernet68", "mode": "routed"},
    {"interface": "Ethernet72", "mode": "routed"},
    {"interface": "Ethernet76", "mode": "routed"},
    {"interface": "Ethernet80", "mode": "routed"},
    {"interface": "Ethernet84", "mode": "routed"},
    {"interface": "Ethernet88", "mode": "routed"},
    {"interface": "Ethernet92", "mode": "routed"},
    {"interface": "Ethernet96", "mode": "routed"},
    {"interface": "Ethernet100", "mode": "routed"},
    {"interface": "Ethernet104", "mode": "routed"},
    {"interface": "Ethernet108", "mode": "routed"},
    {"interface": "Ethernet116", "mode": "routed"},
    {"interface": "Ethernet124", "mode": "routed"},
    {"interface": "PortChannel0001", "mode": "routed"},
    {"interface": "PortChannel0002", "mode": "routed"},
    {"interface": "PortChannel0003", "mode": "routed"},
    {"interface": "PortChannel0004", "mode": "routed"},
    {"interface": "PortChannel1001", "mode": "trunk"}
]"#;

/// Mock JSON for `show interfaces switchport config`
/// Source: interfaces_test.py `show_interfaces_switchport_config_output`
pub const MOCK_SWITCHPORT_CONFIG: &str = r#"[
    {"interface": "Ethernet0", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet4", "mode": "trunk", "untagged_vlan": "1000", "tagged_vlans": ""},
    {"interface": "Ethernet8", "mode": "routed", "untagged_vlan": "1000", "tagged_vlans": ""},
    {"interface": "Ethernet12", "mode": "routed", "untagged_vlan": "1000", "tagged_vlans": ""},
    {"interface": "Ethernet16", "mode": "trunk", "untagged_vlan": "1000", "tagged_vlans": ""},
    {"interface": "Ethernet20", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet24", "mode": "trunk", "untagged_vlan": "2000", "tagged_vlans": ""},
    {"interface": "Ethernet28", "mode": "trunk", "untagged_vlan": "2000", "tagged_vlans": ""},
    {"interface": "Ethernet36", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet40", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet44", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet48", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet52", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet56", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet60", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet64", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet68", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet72", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet76", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet80", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet84", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet88", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet92", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet96", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet100", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet104", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet108", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet116", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "Ethernet124", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "PortChannel0001", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "PortChannel0002", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "PortChannel0003", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "PortChannel0004", "mode": "routed", "untagged_vlan": "", "tagged_vlans": ""},
    {"interface": "PortChannel1001", "mode": "trunk", "untagged_vlan": "", "tagged_vlans": "4000"}
]"#;

/// Mock JSON for `show interfaces flap`
/// Source: interfaces_test.py `intf_flap_expected_output_with_data`
pub const MOCK_INTERFACES_FLAP: &str = r#"[
    {"Interface": "Ethernet0", "Flap Count": "5", "Admin": "up", "Oper": "up", "Link Down TimeStamp(UTC)": "2023-01-01 00:00:00", "Link Up TimeStamp(UTC)": "2023-01-01 00:05:00"},
    {"Interface": "Ethernet4", "Flap Count": "3", "Admin": "up", "Oper": "down", "Link Down TimeStamp(UTC)": "2023-01-02 12:00:00", "Link Up TimeStamp(UTC)": ""},
    {"Interface": "PortChannel0001", "Flap Count": "1", "Admin": "up", "Oper": "up", "Link Down TimeStamp(UTC)": "2023-01-01 06:00:00", "Link Up TimeStamp(UTC)": "2023-01-01 06:05:00"}
]"#;

/// Mock JSON for `show interfaces errors`
/// Source: interfaces_test.py `intf_errors_Ethernet64` (uses Port Errors / Count format)
pub const MOCK_INTERFACES_ERRORS: &str = r#"[
    {"Port Errors": "FCS Error Count", "Count": "0", "Last timestamp(UTC)": "N/A"},
    {"Port Errors": "Rx Length Error Count", "Count": "15", "Last timestamp(UTC)": "2023-12-01 10:30:00"},
    {"Port Errors": "Tx CRC Error Count", "Count": "0", "Last timestamp(UTC)": "N/A"},
    {"Port Errors": "Alignment Error Count", "Count": "3", "Last timestamp(UTC)": "2023-11-15 08:45:00"}
]"#;

/// Mock JSON for `show interfaces status`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_STATUS: &str = r#"[
    {"Interface": "Ethernet0", "Lanes": "0,1,2,3", "Speed": "100G", "MTU": "9100", "FEC": "rs", "Alias": "etp1", "Vlan": "routed", "Oper": "up", "Admin": "up", "Type": "QSFP28", "Asym PFC": "off"},
    {"Interface": "Ethernet4", "Lanes": "4,5,6,7", "Speed": "100G", "MTU": "9100", "FEC": "rs", "Alias": "etp2", "Vlan": "trunk", "Oper": "up", "Admin": "up", "Type": "QSFP28", "Asym PFC": "off"},
    {"Interface": "Ethernet8", "Lanes": "8,9,10,11", "Speed": "100G", "MTU": "9100", "FEC": "rs", "Alias": "etp3", "Vlan": "routed", "Oper": "down", "Admin": "up", "Type": "QSFP28", "Asym PFC": "off"}
]"#;

/// Mock JSON for `show interfaces description`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_DESCRIPTION: &str = r#"[
    {"interface": "Ethernet0", "oper_status": "up", "admin_status": "up", "alias": "etp1", "description": "ARISTA01T1:Ethernet1"},
    {"interface": "Ethernet4", "oper_status": "up", "admin_status": "up", "alias": "etp2", "description": "ARISTA02T1:Ethernet1"},
    {"interface": "Ethernet8", "oper_status": "down", "admin_status": "up", "alias": "etp3", "description": "ARISTA03T1:Ethernet1"}
]"#;

/// Mock JSON for `show interfaces counters`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_COUNTERS: &str = r#"{
    "Ethernet0": {"RX_OK": "1000", "RX_BPS": "50000", "RX_PPS": "100", "RX_UTIL": "0.50%", "TX_OK": "2000", "TX_BPS": "100000", "TX_PPS": "200", "TX_UTIL": "1.00%", "STATE": "U"},
    "Ethernet4": {"RX_OK": "500", "RX_BPS": "25000", "RX_PPS": "50", "RX_UTIL": "0.25%", "TX_OK": "750", "TX_BPS": "37500", "TX_PPS": "75", "TX_UTIL": "0.38%", "STATE": "U"}
}"#;

/// Mock JSON for `show interfaces counters errors`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_COUNTERS_ERRORS: &str = r#"{
    "Ethernet0": {"RX_ERR": "0", "RX_DRP": "5", "RX_OVR": "0", "TX_ERR": "0", "TX_DRP": "2", "TX_OVR": "0", "STATE": "U"},
    "Ethernet4": {"RX_ERR": "1", "RX_DRP": "10", "RX_OVR": "0", "TX_ERR": "0", "TX_DRP": "3", "TX_OVR": "0", "STATE": "U"}
}"#;

/// Mock JSON for `show interfaces counters rates`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_COUNTERS_RATES: &str = r#"{
    "Ethernet0": {"RX_OK": "1000", "RX_BPS": "50000.00", "RX_PPS": "100.00", "RX_UTIL": "0.50%", "TX_OK": "2000", "TX_BPS": "100000.00", "TX_PPS": "200.00", "TX_UTIL": "1.00%", "STATE": "U"},
    "Ethernet4": {"RX_OK": "500", "RX_BPS": "25000.00", "RX_PPS": "50.00", "RX_UTIL": "0.25%", "TX_OK": "750", "TX_BPS": "37500.00", "TX_PPS": "75.00", "TX_UTIL": "0.38%", "STATE": "U"}
}"#;

/// Mock JSON for `show interfaces counters rif`
/// Source: derived from interfaces_test.py data format
pub const MOCK_INTERFACES_COUNTERS_RIF: &str = r#"{
    "Ethernet0": {"RX_OK": "1000", "RX_BPS": "50000", "RX_PPS": "100", "RX_ERR": "0", "TX_OK": "2000", "TX_BPS": "100000", "TX_PPS": "200", "TX_ERR": "0"},
    "Ethernet4": {"RX_OK": "500", "RX_BPS": "25000", "RX_PPS": "50", "RX_ERR": "1", "TX_OK": "750", "TX_BPS": "37500", "TX_PPS": "75", "TX_ERR": "0"}
}"#;

/// Mock JSON for `show interfaces naming_mode`
pub const MOCK_NAMING_MODE: &str = r#"{"naming_mode": "default"}"#;

/// Mock JSON for `show interfaces fec status`
pub const MOCK_FEC_STATUS: &str = r#"[
    {"Interface": "Ethernet0", "FEC Oper": "rs", "FEC Admin": "rs"},
    {"Interface": "Ethernet4", "FEC Oper": "none", "FEC Admin": "none"},
    {"Interface": "Ethernet8", "FEC Oper": "rs", "FEC Admin": "auto"}
]"#;

/// Mock JSON for `show interfaces counters fec-stats`
pub const MOCK_FEC_STATS: &str = r#"{
    "Ethernet0": {"FEC_CORR": "1000", "FEC_UNCORR": "5", "FEC_SYMBOL_ERR": "50"},
    "Ethernet4": {"FEC_CORR": "500", "FEC_UNCORR": "0", "FEC_SYMBOL_ERR": "10"}
}"#;

/// Mock JSON for `show interfaces counters fec-histogram`
pub const MOCK_FEC_HISTOGRAM: &str = r#"{
    "Ethernet0": {"0": "1000000", "1": "50000", "2": "2500", "3": "100", "4": "5", "5": "0", "6": "0", "7": "0", "8": "0", "9": "0", "10": "0", "11": "0", "12": "0", "13": "0", "14": "0", "15": "0"}
}"#;

/// Mock JSON for `show interfaces counters trim`
pub const MOCK_COUNTERS_TRIM: &str = r#"{
    "Ethernet0": {"TRIM_PKTS": "100", "TRIM_TX_PKTS": "20", "TRIM_DRP_PKTS": "80", "STATE": "U"},
    "Ethernet4": {"TRIM_PKTS": "50", "TRIM_TX_PKTS": "10", "TRIM_DRP_PKTS": "40", "STATE": "U"}
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `interfaces_test.py` — `show interfaces flap`
pub const SHOW_INTERFACES_FLAP_OUTPUT: &str = r#"      Interface    Flap Count    Admin    Oper    Link Down TimeStamp(UTC)    Link Up TimeStamp(UTC)
---------------  ------------  -------  ------  --------------------------  ------------------------
Ethernet0                   5  up       up      2023-01-01 00:00:00         2023-01-01 00:05:00
Ethernet4                   3  up       down    2023-01-02 12:00:00
PortChannel0001             1  up       up      2023-01-01 06:00:00         2023-01-01 06:05:00"#;

/// Source: `interfaces_test.py` — `show interfaces errors`
pub const SHOW_INTERFACES_ERRORS_OUTPUT: &str = r#"          Port Errors    Count    Last timestamp(UTC)
---------------------  -------  ---------------------
FCS Error Count              0  N/A
Rx Length Error Count       15  2023-12-01 10:30:00
Tx CRC Error Count           0  N/A
Alignment Error Count        3  2023-11-15 08:45:00"#;

/// Source: `interfaces_test.py` — `show interfaces status`
pub const SHOW_INTERFACES_STATUS_OUTPUT: &str = r#"  Interface      Lanes    Speed    MTU    FEC    Alias    Vlan    Oper    Admin    Type    Asym PFC
-----------  ---------  -------  -----  -----  -------  ------  ------  -------  ------  ----------
  Ethernet0    0,1,2,3     100G   9100     rs     etp1  routed      up       up  QSFP28         off
  Ethernet4    4,5,6,7     100G   9100     rs     etp2   trunk      up       up  QSFP28         off
  Ethernet8  8,9,10,11     100G   9100     rs     etp3  routed    down       up  QSFP28         off"#;

/// Source: `interfaces_test.py` — `show interfaces description`
pub const SHOW_INTERFACES_DESCRIPTION_OUTPUT: &str = r#"  Interface    Oper    Admin    Alias           Description
-----------  ------  -------  -------  --------------------
  Ethernet0      up       up     etp1  ARISTA01T1:Ethernet1
  Ethernet4      up       up     etp2  ARISTA02T1:Ethernet1
  Ethernet8    down       up     etp3  ARISTA03T1:Ethernet1"#;

/// Source: `interfaces_test.py` — `show interfaces counters`
pub const SHOW_INTERFACES_COUNTERS_OUTPUT: &str = r#"    IFACE    STATE    RX_OK    RX_BPS    RX_UTIL    RX_ERR    RX_DRP    RX_OVR    TX_OK    TX_BPS    TX_UTIL    TX_ERR    TX_DRP    TX_OVR
---------  -------  -------  --------  ---------  --------  --------  --------  -------  --------  ---------  --------  --------  --------
Ethernet0        U    1,000     50000      0.50%       N/A       N/A       N/A    2,000    100000      1.00%       N/A       N/A       N/A
Ethernet4        U      500     25000      0.25%       N/A       N/A       N/A      750     37500      0.38%       N/A       N/A       N/A"#;

/// Source: `interfaces_test.py` — `show interfaces counters errors`
pub const SHOW_INTERFACES_COUNTERS_ERRORS_OUTPUT: &str = r#"    IFACE    STATE    RX_ERR    RX_DRP    RX_OVR    TX_ERR    TX_DRP    TX_OVR
---------  -------  --------  --------  --------  --------  --------  --------
Ethernet0        U         0         5         0         0         2         0
Ethernet4        U         1        10         0         0         3         0"#;

/// Source: `interfaces_test.py` — `show interfaces counters rates`
pub const SHOW_INTERFACES_COUNTERS_RATES_OUTPUT: &str = r#"    IFACE    STATE    RX_OK    RX_BPS    RX_PPS    RX_UTIL    TX_OK    TX_BPS    TX_PPS    TX_UTIL
---------  -------  -------  --------  --------  ---------  -------  --------  --------  ---------
Ethernet0        U    1,000     50000       100      0.50%    2,000    100000       200      1.00%
Ethernet4        U      500     25000        50      0.25%      750     37500        75      0.38%"#;

/// Source: `interfaces_test.py` — `show interfaces counters rif`
pub const SHOW_INTERFACES_COUNTERS_RIF_OUTPUT: &str = r#"    IFACE    STATE    RX_OK    RX_BPS    RX_UTIL    RX_ERR    RX_DRP    RX_OVR    TX_OK    TX_BPS    TX_UTIL    TX_ERR    TX_DRP    TX_OVR
---------  -------  -------  --------  ---------  --------  --------  --------  -------  --------  ---------  --------  --------  --------
Ethernet0      N/A    1,000     50000        N/A         0       N/A       N/A    2,000    100000        N/A         0       N/A       N/A
Ethernet4      N/A      500     25000        N/A         1       N/A       N/A      750     37500        N/A         0       N/A       N/A"#;

/// Source: `interfaces_test.py` — `show interfaces fec status`
pub const SHOW_FEC_STATUS_OUTPUT: &str = r#"  Interface    FEC Oper    FEC Admin
-----------  ----------  -----------
  Ethernet0          rs           rs
  Ethernet4        none         none
  Ethernet8          rs         auto"#;

/// Source: `interfaces_test.py` — `show interfaces counters fec-stats`
pub const SHOW_FEC_STATS_OUTPUT: &str = r#"    IFACE    STATE    FEC_CORR    FEC_UNCORR    FEC_SYMBOL_ERR    FEC_PRE_BER    FEC_POST_BER
---------  -------  ----------  ------------  ----------------  -------------  --------------
Ethernet0      N/A        1000             5                50            N/A             N/A
Ethernet4      N/A         500             0                10            N/A             N/A"#;

/// Source: `interfaces_test.py` — `show interfaces counters fec-histogram`
pub const SHOW_FEC_HISTOGRAM_OUTPUT: &str = r#"Ethernet0:
  0: 1000000
  1: 50000
  2: 2500
  3: 100
  4: 5
  5: 0
  6: 0
  7: 0
  8: 0
  9: 0
  10: 0
  11: 0
  12: 0
  13: 0
  14: 0
  15: 0"#;

/// Source: `interfaces_test.py` — `show interfaces counters trim`
pub const SHOW_COUNTERS_TRIM_OUTPUT: &str = r#"    IFACE    STATE    TRIM_PKTS    TRIM_TX_PKTS    TRIM_DRP_PKTS
---------  -------  -----------  --------------  ---------------
Ethernet0        U          100              20               80
Ethernet4        U           50              10               40"#;

/// Source: `interfaces_test.py` — `show interfaces naming_mode`
pub const SHOW_NAMING_MODE_OUTPUT: &str = r#"default"#;
