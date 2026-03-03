"""
Baseline UT: show mac (Cmd #46)

Source:
  - sonic-utilities UT File : fdbshow_test.py
  - sonic-utilities Test Func: test_show_mac
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #46

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show mac"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "entries": [
        {
            "No.": "1",
            "vlan": "2",
            "macAddress": "11:22:33:44:55:66",
            "port": "Ethernet0",
            "type": "Dynamic"
        },
        {
            "No.": "2",
            "vlan": "3",
            "macAddress": "11:22:33:66:55:44",
            "port": "Ethernet4",
            "type": "Static"
        },
        {
            "No.": "3",
            "vlan": "4",
            "macAddress": "66:55:44:33:22:11",
            "port": "Ethernet0",
            "type": "Dynamic"
        },
        {
            "No.": "4",
            "vlan": "4",
            "macAddress": "77:66:44:33:22:11",
            "port": "1000000000fff",
            "type": "Dynamic"
        },
        {
            "No.": "5",
            "vlan": "5",
            "macAddress": "77:66:55:44:22:11",
            "port": "Ethernet4",
            "type": "Dynamic"
        }
    ],
    "total": 5
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
  No.    Vlan  MacAddress         Port           Type
-----  ------  -----------------  -------------  -------
    1       2  11:22:33:44:55:66  Ethernet0      Dynamic
    2       3  11:22:33:66:55:44  Ethernet4      Static
    3       4  66:55:44:33:22:11  Ethernet0      Dynamic
    4       4  77:66:44:33:22:11  1000000000fff  Dynamic
    5       5  77:66:55:44:22:11  Ethernet4      Dynamic
Total number of entries 5"""


@pytest.mark.sonic_baseline
class TestMac:
    """
    Baseline test for 'show mac' (Cmd #46).

    Mirrors sonic-utilities test:
      File: fdbshow_test.py
      Func: test_show_mac
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
