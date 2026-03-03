"""
Baseline UT: show interfaces transceiver info (Cmd #30)

Source:
  - sonic-utilities UT File : sfp_test.py
  - sonic-utilities Test Func: test_cmis_eeprom
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #30

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces transceiver info"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet64": "{\"Active Firmware\": \"X.X\", \"Active application selected code assigned to host lane 1\": \"1\", \"Active application selected code assigned to host lane 2\": \"1\", \"Active application selected code assigned to host lane 3\": \"1\", \"Active application selected code assigned to host lane 4\": \"1\", \"Active application selected code assigned to host lane 5\": \"1\", \"Active application selected code assigned to host lane 6\": \"1\", \"Active application selected code assigned to host lane 7\": \"1\", \"Active application selected code assigned to host lane 8\": \"1\", \"Application Advertisement\": [\"400GAUI-8 C2M (Annex 120E) - Host Assign (0x1) - 400ZR, DWDM, amplified - Media Assign (0x1)\", \"400GAUI-8 C2M (Annex 120E) - Host Assign (0x1) - 400ZR, Single Wavelength, Unamplified - Media Assign (0x1)\", \"100GAUI-2 C2M (Annex 135G) - Host Assign (0x55) - 400ZR, DWDM, amplified - Media Assign (0x1)\"], \"CMIS Rev\": \"4.1\", \"Connector\": \"LC\", \"E1 Active Firmware\": \"X.X\", \"E1 Inactive Firmware\": \"Y.Y\", \"E1 Server Firmware\": \"A.B.C.D\", \"E2 Active Firmware\": \"X.X\", \"E2 Inactive Firmware\": \"Y.Y\", \"E2 Server Firmware\": \"A.B.C.D\", \"Encoding\": \"N/A\", \"Extended Identifier\": \"Power Class 8 (20.0W Max)\", \"Extended RateSelect Compliance\": \"N/A\", \"Host Lane Count\": \"8\", \"Identifier\": \"QSFP-DD Double Density 8X Pluggable Transceiver\", \"Inactive Firmware\": \"X.X\", \"Length Cable Assembly(m)\": \"0.0\", \"Media Interface Technology\": \"1550 nm DFB\", \"Media Lane Count\": \"1\", \"Module Hardware Rev\": \"X.X\", \"Nominal Bit Rate(100Mbs)\": \"0\", \"Specification compliance\": \"sm_media_interface\", \"Supported Max Laser Frequency\": \"196100\", \"Supported Max TX Power\": \"4.0\", \"Supported Min Laser Frequency\": \"191300\", \"Supported Min TX Power\": \"-22.9\", \"Vendor Date Code(YYYY-MM-DD Lot)\": \"2021-11-19\", \"Vendor Name\": \"XXXX\", \"Vendor OUI\": \"XX-XX-XX\", \"Vendor PN\": \"XXX\", \"Vendor Rev\": \"XX\", \"Vendor SN\": \"0123456789\"}"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Ethernet64: SFP EEPROM detected
        Active Firmware: X.X
        Active application selected code assigned to host lane 1: 1
        Active application selected code assigned to host lane 2: 1
        Active application selected code assigned to host lane 3: 1
        Active application selected code assigned to host lane 4: 1
        Active application selected code assigned to host lane 5: 1
        Active application selected code assigned to host lane 6: 1
        Active application selected code assigned to host lane 7: 1
        Active application selected code assigned to host lane 8: 1
        Application Advertisement: 400GAUI-8 C2M (Annex 120E) - Host Assign (0x1) - 400ZR, DWDM, amplified - Media Assign (0x1)
                                   400GAUI-8 C2M (Annex 120E) - Host Assign (0x1) - 400ZR, Single Wavelength, Unamplified - Media Assign (0x1)
                                   100GAUI-2 C2M (Annex 135G) - Host Assign (0x55) - 400ZR, DWDM, amplified - Media Assign (0x1)
        CMIS Rev: 4.1
        Connector: LC
        E1 Active Firmware: X.X
        E1 Inactive Firmware: Y.Y
        E1 Server Firmware: A.B.C.D
        E2 Active Firmware: X.X
        E2 Inactive Firmware: Y.Y
        E2 Server Firmware: A.B.C.D
        Encoding: N/A
        Extended Identifier: Power Class 8 (20.0W Max)
        Extended RateSelect Compliance: N/A
        Host Lane Count: 8
        Identifier: QSFP-DD Double Density 8X Pluggable Transceiver
        Inactive Firmware: X.X
        Length Cable Assembly(m): 0.0
        Media Interface Technology: 1550 nm DFB
        Media Lane Count: 1
        Module Hardware Rev: X.X
        Nominal Bit Rate(100Mbs): 0
        Specification compliance: sm_media_interface
        Supported Max Laser Frequency: 196100
        Supported Max TX Power: 4.0
        Supported Min Laser Frequency: 191300
        Supported Min TX Power: -22.9
        Vendor Date Code(YYYY-MM-DD Lot): 2021-11-19
        Vendor Name: XXXX
        Vendor OUI: XX-XX-XX
        Vendor PN: XXX
        Vendor Rev: XX
        Vendor SN: 0123456789"""


@pytest.mark.sonic_baseline
class TestInterfacesTransceiverInfo:
    """
    Baseline test for 'show interfaces transceiver info' (Cmd #30).

    Mirrors sonic-utilities test:
      File: sfp_test.py
      Func: test_cmis_eeprom
    """

    def test_converter_output_matches_sonic_ut(self, command_parser):
        """JSON -> Converter -> CLI text should match sonic-utilities expected output."""
        actual = command_parser.parse_and_convert(CLI_COMMAND, json.dumps(JSON_INPUT))
        assert actual.strip() == EXPECTED_CLI_OUTPUT.strip(), (
            f"Converter output for '{CLI_COMMAND}' does not match "
            f"sonic-utilities baseline.\n"
            f"--- Expected ---\n{EXPECTED_CLI_OUTPUT}\n"
            f"--- Actual ---\n{actual}"
        )
