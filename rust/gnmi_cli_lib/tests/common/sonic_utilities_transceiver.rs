
// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Source: sfp_test.py `test_sfp_eeprom_with_dom_output`
pub const MOCK_TRANSCEIVER_EEPROM_SFP_DOM: &str = r#"{
    "Ethernet0": {
        "type": "SFP",
        "Connector": "LC",
        "Encoding": "64B/66B",
        "Extended Identifier": "SFP/SFP+",
        "Extended RateSelect Compliance": "QSFP+ Rate Select Version 1",
        "Identifier": "SFP/SFP+",
        "Length Cable Assembly(m)": "0",
        "Length OM1(m)": "0",
        "Length OM2(m)": "0",
        "Length OM3(2m)": "0",
        "Length SMF(Km)": "0",
        "Nominal Bit Rate(100Mbs)": "255",
        "Specification compliance": "{'10/40G Ethernet Compliance Code': '40GBASE-CR4'}",
        "Vendor Date Code(YYYY-MM-DD Lot)": "2018-01-01",
        "Vendor Name": "VENDOR1",
        "Vendor OUI": "00-00-00",
        "Vendor PN": "SFP-PART-001",
        "Vendor Rev": "A",
        "Vendor SN": "SN0001",
        "RXPower": "-1.5dBm",
        "TXBias": "6.0mA",
        "TXPower": "-1.0dBm",
        "Temperature": "25.0C",
        "Voltage": "3.3V"
    }
}"#;

/// Mock JSON for `show interfaces transceiver eeprom` — QSFP-DD with DOM
/// Source: sfp_test.py `test_qsfp_dd_eeprom_with_dom_output`
pub const MOCK_TRANSCEIVER_EEPROM_QSFPDD: &str = r#"{
    "Ethernet0": {
        "type": "QSFP-DD",
        "Active Firmware": "1.0",
        "Application Advertisement": "400GAUI-8 C2M (Annex 120E) - Host Assign 0x01, 400G-FR4 - Media Assign 0x01\n400GAUI-8 C2M (Annex 120E) - Host Assign 0x01, 400G-LR4-6 - Media Assign 0x01",
        "CMIS Rev": "4.0",
        "Connector": "LC",
        "Encoding": "N/A",
        "Extended Identifier": "Power Class 8 (20.0W Max)",
        "Extended RateSelect Compliance": "N/A",
        "Host Lane Count": "8",
        "Identifier": "QSFP-DD Double Density 8X Pluggable Transceiver",
        "Inactive Firmware": "0.9",
        "Length Cable Assembly(m)": "0",
        "Media Interface Technology": "1550 nm DFB",
        "Media Lane Count": "4",
        "Module Hardware Rev": "1.0",
        "Nominal Bit Rate(100Mbs)": "0",
        "Specification compliance": "sm_media_interface",
        "Supported Max Laser Frequency": "N/A",
        "Supported Max TX Power": "N/A",
        "Supported Min Laser Frequency": "N/A",
        "Supported Min TX Power": "N/A",
        "Vendor Date Code(YYYY-MM-DD Lot)": "2020-01-01",
        "Vendor Name": "VENDOR2",
        "Vendor OUI": "00-00-01",
        "Vendor PN": "QSFPDD-PART-001",
        "Vendor Rev": "B",
        "Vendor SN": "SN0002"
    }
}"#;

/// Mock JSON for `show interfaces transceiver presence`
/// Source: sfp_test.py `test_sfp_presence_all_output`
pub const MOCK_TRANSCEIVER_PRESENCE: &str = r#"[
    {"port": "Ethernet0", "presence": "Present"},
    {"port": "Ethernet4", "presence": "Present"},
    {"port": "Ethernet8", "presence": "Not present"},
    {"port": "Ethernet12", "presence": "Present"},
    {"port": "Ethernet16", "presence": "Not present"}
]"#;

/// Mock JSON for `show interfaces transceiver status` — QSFP status
/// Source: sfp_test.py `test_qsfp_status_output`
pub const MOCK_TRANSCEIVER_STATUS_QSFP: &str = r#"{
    "Ethernet0": {
        "CMIS State": "READY",
        "Module State": "ModuleReady",
        "Module Fault Cause": "No Fault detected",
        "Temperature": "25.5",
        "Vcc": "3.30"
    }
}"#;

/// Mock JSON for `show interfaces transceiver status` — CMIS REMOVED
pub const MOCK_TRANSCEIVER_STATUS_REMOVED: &str = r#"{
    "Ethernet0": {
        "CMIS State": "REMOVED"
    }
}"#;

/// Mock JSON for `show interfaces transceiver lpmode`
pub const MOCK_TRANSCEIVER_LPMODE: &str = r#"[
    {"port": "Ethernet0", "lpmode": "On"},
    {"port": "Ethernet4", "lpmode": "Off"},
    {"port": "Ethernet8", "lpmode": "On"},
    {"port": "Ethernet12", "lpmode": "Off"}
]"#;

/// Mock JSON for `show interfaces transceiver error-status`
pub const MOCK_TRANSCEIVER_ERROR_STATUS: &str = r#"[
    {"Port": "Ethernet0", "Error Status": "OK"},
    {"Port": "Ethernet4", "Error Status": "OK"},
    {"Port": "Ethernet8", "Error Status": "Unplugged"},
    {"Port": "Ethernet12", "Error Status": "Power Budget Exceeded"}
]"#;

/// Mock JSON for `show interfaces transceiver pm`
pub const MOCK_TRANSCEIVER_PM: &str = r#"{
    "Ethernet0": {
        "current": {"temp": "25.5", "voltage": "3.30"},
        "min": {"temp": "20.0", "voltage": "3.25"},
        "max": {"temp": "30.0", "voltage": "3.35"}
    }
}"#;

/// Mock JSON for `show interfaces transceiver pm` — N/A case
pub const MOCK_TRANSCEIVER_PM_NA: &str = r#"{
    "Ethernet0": "N/A"
}"#;

/// Mock JSON for `show interfaces transceiver firmware`
pub const MOCK_TRANSCEIVER_FIRMWARE: &str = r#"[
    {"port": "Ethernet0", "active_firmware": "1.0", "inactive_firmware": "0.9"},
    {"port": "Ethernet4", "active_firmware": "2.1", "inactive_firmware": "2.0"},
    {"port": "Ethernet8", "active_firmware": "1.5", "inactive_firmware": "1.4"}
]"#;

/// Mock JSON for `show interfaces transceiver info` — empty/not detected
pub const MOCK_TRANSCEIVER_INFO_EMPTY: &str = r#"{
    "Ethernet0": null
}"#;

/// Mock JSON for `show interfaces transceiver eeprom` — all ports
/// Source: sfp_test.py `test_sfp_eeprom_all_output`
pub const MOCK_TRANSCEIVER_EEPROM_ALL: &str = r#"{
    "Ethernet0": {
        "Connector": "LC",
        "Identifier": "SFP/SFP+",
        "Vendor Name": "VENDOR1",
        "Vendor PN": "SFP-PART-001",
        "Vendor SN": "SN0001"
    },
    "Ethernet4": {
        "Connector": "LC",
        "Identifier": "QSFP28",
        "Vendor Name": "VENDOR2",
        "Vendor PN": "QSFP-PART-001",
        "Vendor SN": "SN0002"
    }
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `sfp_test.py` — `show interfaces transceiver eeprom` (SFP with DOM)
pub const SHOW_TRANSCEIVER_EEPROM_SFP_DOM_OUTPUT: &str = r#"Ethernet0: SFP EEPROM detected
        Connector: LC
        Encoding: 64B/66B
        Extended Identifier: SFP/SFP+
        Extended RateSelect Compliance: QSFP+ Rate Select Version 1
        Identifier: SFP/SFP+
        Length Cable Assembly(m): 0
        Length OM1(m): 0
        Length OM2(m): 0
        Length OM3(2m): 0
        Length SMF(Km): 0
        Nominal Bit Rate(100Mbs): 255
        RXPower: -1.5dBm
        Specification compliance: {'10/40G Ethernet Compliance Code': '40GBASE-CR4'}
        TXBias: 6.0mA
        TXPower: -1.0dBm
        Temperature: 25.0C
        Vendor Date Code(YYYY-MM-DD Lot): 2018-01-01
        Vendor Name: VENDOR1
        Vendor OUI: 00-00-00
        Vendor PN: SFP-PART-001
        Vendor Rev: A
        Vendor SN: SN0001
        Voltage: 3.3V
        type: SFP"#;

/// Source: `sfp_test.py` — `show interfaces transceiver eeprom` (QSFP-DD)
pub const SHOW_TRANSCEIVER_EEPROM_QSFPDD_OUTPUT: &str = r#"Ethernet0: SFP EEPROM detected
        Active Firmware: 1.0
        Application Advertisement: 400GAUI-8 C2M (Annex 120E) - Host Assign 0x01, 400G-FR4 - Media Assign 0x01
                                   400GAUI-8 C2M (Annex 120E) - Host Assign 0x01, 400G-LR4-6 - Media Assign 0x01
        CMIS Rev: 4.0
        Connector: LC
        Encoding: N/A
        Extended Identifier: Power Class 8 (20.0W Max)
        Extended RateSelect Compliance: N/A
        Host Lane Count: 8
        Identifier: QSFP-DD Double Density 8X Pluggable Transceiver
        Inactive Firmware: 0.9
        Length Cable Assembly(m): 0
        Media Interface Technology: 1550 nm DFB
        Media Lane Count: 4
        Module Hardware Rev: 1.0
        Nominal Bit Rate(100Mbs): 0
        Specification compliance: sm_media_interface
        Supported Max Laser Frequency: N/A
        Supported Max TX Power: N/A
        Supported Min Laser Frequency: N/A
        Supported Min TX Power: N/A
        Vendor Date Code(YYYY-MM-DD Lot): 2020-01-01
        Vendor Name: VENDOR2
        Vendor OUI: 00-00-01
        Vendor PN: QSFPDD-PART-001
        Vendor Rev: B
        Vendor SN: SN0002
        type: QSFP-DD"#;

/// Source: `sfp_test.py` — `show interfaces transceiver eeprom` (all)
pub const SHOW_TRANSCEIVER_EEPROM_ALL_OUTPUT: &str = r#"Ethernet0: SFP EEPROM detected
        Connector: LC
        Identifier: SFP/SFP+
        Vendor Name: VENDOR1
        Vendor PN: SFP-PART-001
        Vendor SN: SN0001

Ethernet4: SFP EEPROM detected
        Connector: LC
        Identifier: QSFP28
        Vendor Name: VENDOR2
        Vendor PN: QSFP-PART-001
        Vendor SN: SN0002"#;

/// Source: `sfp_test.py` — `show interfaces transceiver presence`
pub const SHOW_TRANSCEIVER_PRESENCE_OUTPUT: &str = r#"Port        Presence
----------  -----------
Ethernet0   Present
Ethernet4   Present
Ethernet8   Not present
Ethernet12  Present
Ethernet16  Not present"#;

/// Source: `sfp_test.py` — `show interfaces transceiver status` (QSFP)
pub const SHOW_TRANSCEIVER_STATUS_QSFP_OUTPUT: &str = r#"Ethernet0: 
        CMIS State: READY
        Module Fault Cause: No Fault detected
        Module State: ModuleReady
        Temperature: 25.5
        Vcc: 3.30"#;

/// Source: `sfp_test.py` — `show interfaces transceiver status` (removed)
pub const SHOW_TRANSCEIVER_STATUS_REMOVED_OUTPUT: &str = r#"Ethernet0: 
        CMIS State: REMOVED"#;

/// Source: `sfp_test.py` — `show interfaces transceiver lpmode`
pub const SHOW_TRANSCEIVER_LPMODE_OUTPUT: &str = r#"      Port    Low-power Mode
----------  ----------------
Ethernet0   On
Ethernet4   Off
Ethernet8   On
Ethernet12  Off"#;

/// Source: `sfp_test.py` — `show interfaces transceiver error-status`
pub const SHOW_TRANSCEIVER_ERROR_STATUS_OUTPUT: &str = r#"Port        Error Status
----------  ---------------------
Ethernet0   OK
Ethernet4   OK
Ethernet8   Unplugged
Ethernet12  Power Budget Exceeded"#;

/// Source: `sfp_test.py` — `show interfaces transceiver pm`
pub const SHOW_TRANSCEIVER_PM_OUTPUT: &str = r#"Ethernet0: 
        CURRENT:
                temp: 25.5
                voltage: 3.30
        MAX:
                temp: 30.0
                voltage: 3.35
        MIN:
                temp: 20.0
                voltage: 3.25"#;

/// Source: `sfp_test.py` — `show interfaces transceiver pm` (N/A)
pub const SHOW_TRANSCEIVER_PM_NA_OUTPUT: &str = r#"Ethernet0: Transceiver performance monitoring not applicable"#;

/// Source: `sfp_test.py` — `show interfaces transceiver firmware`
pub const SHOW_TRANSCEIVER_FIRMWARE_OUTPUT: &str = r#"Ethernet0: SFP EEPROM detected
        active_firmware: 1.0
        inactive_firmware: 0.9

Ethernet4: SFP EEPROM detected
        active_firmware: 2.1
        inactive_firmware: 2.0

Ethernet8: SFP EEPROM detected
        active_firmware: 1.5
        inactive_firmware: 1.4"#;

/// Source: `sfp_test.py` — `show interfaces transceiver info` (empty)
pub const SHOW_TRANSCEIVER_INFO_EMPTY_OUTPUT: &str = r#"Ethernet0: SFP EEPROM Not detected"#;
