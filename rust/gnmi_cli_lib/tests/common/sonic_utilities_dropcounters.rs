
// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Mock JSON for `show dropcounters capabilities`
pub const MOCK_DROPCOUNTERS_CAPABILITIES: &str = r#"[
    {"counter_type": "PORT_INGRESS_DROPS", "count": "gillsw7", "reasons": "[L2_ANY,L3_ANY,SMAC_MULTICAST,SMAC_EQUALS_DMAC]"},
    {"counter_type": "PORT_EGRESS_DROPS", "count": "4", "reasons": "[L2_ANY,L3_ANY]"},
    {"counter_type": "SWITCH_INGRESS_DROPS", "count": "2", "reasons": "[ACL_ANY,DHCP]"},
    {"counter_type": "SWITCH_EGRESS_DROPS", "count": "1", "reasons": "[L3_EGRESS_LINK_DOWN]"}
]"#;

/// Mock JSON for `show dropcounters configuration`
pub const MOCK_DROPCOUNTERS_CONFIG: &str = r#"[
    {"Counter": "DEBUG_0", "Alias": "RxErr", "Group": "PORT_INGRESS_DROPS", "Type": "PORT_INGRESS_DROPS", "Reasons": "[SMAC_MULTICAST,SMAC_EQUALS_DMAC]", "Description": "Debug counter for RX errors"},
    {"Counter": "DEBUG_1", "Alias": "TxErr", "Group": "PORT_EGRESS_DROPS", "Type": "PORT_EGRESS_DROPS", "Reasons": "[L2_ANY]", "Description": "Debug counter for TX errors"}
]"#;

/// Mock JSON for `show dropcounters counts`
pub const MOCK_DROPCOUNTERS_COUNTS: &str = r#"{
    "Ethernet0": {"State": "U", "RX_ERR": "0", "RX_DROPS": "5", "TX_ERR": "0", "TX_DROPS": "2", "DEBUG_0": "3", "DEBUG_1": "0"},
    "Ethernet4": {"State": "U", "RX_ERR": "1", "RX_DROPS": "10", "TX_ERR": "0", "TX_DROPS": "0", "DEBUG_0": "8", "DEBUG_1": "1"},
    "Ethernet8": {"State": "D", "RX_ERR": "0", "RX_DROPS": "0", "TX_ERR": "0", "TX_DROPS": "0", "DEBUG_0": "0", "DEBUG_1": "0"}
}"#;

/// Mock JSON for `show dropcounters counts` — empty
pub const MOCK_DROPCOUNTERS_COUNTS_EMPTY: &str = r#"{}"#;

/// Mock JSON for `show dropcounters reasons`
pub const MOCK_DROP_REASONS: &str = r#"{
    "PORT_INGRESS_DROPS": {
        "L2_ANY": "Any L2-related ingress drop reason",
        "L3_ANY": "Any L3-related ingress drop reason",
        "SMAC_MULTICAST": "Source MAC is multicast",
        "SMAC_EQUALS_DMAC": "Source MAC equals destination MAC"
    },
    "PORT_EGRESS_DROPS": {
        "L2_ANY": "Any L2-related egress drop reason",
        "L3_ANY": "Any L3-related egress drop reason"
    }
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `single_asic_dropstat_test.py` — `show dropcounters capabilities`
pub const SHOW_DROPCOUNTERS_CAPABILITIES_OUTPUT: &str = r#"Counter Type              Total
--------------------  ---------
PORT_EGRESS_DROPS             4
PORT_INGRESS_DROPS      gillsw7
SWITCH_EGRESS_DROPS           1
SWITCH_INGRESS_DROPS          2

PORT_EGRESS_DROPS
        L2_ANY
        L3_ANY

PORT_INGRESS_DROPS
        L2_ANY
        L3_ANY
        SMAC_MULTICAST
        SMAC_EQUALS_DMAC

SWITCH_EGRESS_DROPS
        L3_EGRESS_LINK_DOWN

SWITCH_INGRESS_DROPS
        ACL_ANY
        DHCP"#;

/// Source: `single_asic_dropstat_test.py` — `show dropcounters configuration`
pub const SHOW_DROPCOUNTERS_CONFIG_OUTPUT: &str = r#" Counter  Alias  Group               Type                Reasons                            Description                 
 DEBUG_0  RxErr  PORT_INGRESS_DROPS  PORT_INGRESS_DROPS  [SMAC_MULTICAST,SMAC_EQUALS_DMAC]  Debug counter for RX errors 
 DEBUG_1  TxErr  PORT_EGRESS_DROPS   PORT_EGRESS_DROPS   [L2_ANY]                           Debug counter for TX errors"#;

/// Source: `single_asic_dropstat_test.py` — `show dropcounters counts`
pub const SHOW_DROPCOUNTERS_COUNTS_OUTPUT: &str = r#"    IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS    DEBUG_0    DEBUG_1
---------  -------  --------  ----------  --------  ----------  ---------  ---------
Ethernet0        U         0           5         0           2          3          0
Ethernet4        U         1          10         0           0          8          1
Ethernet8        D         0           0         0           0          0          0"#;

/// Source: `single_asic_dropstat_test.py` — `show dropcounters counts` (empty)
pub const SHOW_DROPCOUNTERS_COUNTS_EMPTY_OUTPUT: &str = "";

/// Source: `single_asic_dropstat_test.py` — `show dropcounters reasons`
pub const SHOW_DROP_REASONS_OUTPUT: &str = r#"Counter Type          Total
------------------  -------
PORT_EGRESS_DROPS          
PORT_INGRESS_DROPS"#;
