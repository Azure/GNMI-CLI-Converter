"""
Baseline UT: show ipv6 link-local-mode (Cmd #42)

Source:
  - sonic-utilities UT File : ipv6_link_local_test.py
  - sonic-utilities Test Func: test_show_ipv6_link_local_mode
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #42

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 link-local-mode"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "port": "Ethernet0",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet4",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet8",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet12",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet16",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet20",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet24",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet28",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet32",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet36",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet40",
        "mode": "Enabled"
    },
    {
        "port": "Ethernet44",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet48",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet52",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet56",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet60",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet64",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet68",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet72",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet76",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet80",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet84",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet88",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet92",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet96",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet100",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet104",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet108",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet112",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet116",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet120",
        "mode": "Disabled"
    },
    {
        "port": "Ethernet124",
        "mode": "Disabled"
    },
    {
        "port": "PortChannel1001",
        "mode": "Disabled"
    },
    {
        "port": "PortChannel0001",
        "mode": "Disabled"
    },
    {
        "port": "PortChannel0002",
        "mode": "Disabled"
    },
    {
        "port": "PortChannel0003",
        "mode": "Disabled"
    },
    {
        "port": "PortChannel0004",
        "mode": "Disabled"
    },
    {
        "port": "Vlan1000",
        "mode": "Disabled"
    },
    {
        "port": "Vlan2000",
        "mode": "Disabled"
    },
    {
        "port": "Vlan3000",
        "mode": "Disabled"
    },
    {
        "port": "Vlan4000",
        "mode": "Disabled"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
+------------------+----------+
| Interface Name   | Mode     |
+==================+==========+
| Ethernet0        | Disabled |
+------------------+----------+
| Ethernet4        | Disabled |
+------------------+----------+
| Ethernet8        | Disabled |
+------------------+----------+
| Ethernet12       | Disabled |
+------------------+----------+
| Ethernet16       | Disabled |
+------------------+----------+
| Ethernet20       | Disabled |
+------------------+----------+
| Ethernet24       | Disabled |
+------------------+----------+
| Ethernet28       | Disabled |
+------------------+----------+
| Ethernet32       | Disabled |
+------------------+----------+
| Ethernet36       | Disabled |
+------------------+----------+
| Ethernet40       | Enabled  |
+------------------+----------+
| Ethernet44       | Disabled |
+------------------+----------+
| Ethernet48       | Disabled |
+------------------+----------+
| Ethernet52       | Disabled |
+------------------+----------+
| Ethernet56       | Disabled |
+------------------+----------+
| Ethernet60       | Disabled |
+------------------+----------+
| Ethernet64       | Disabled |
+------------------+----------+
| Ethernet68       | Disabled |
+------------------+----------+
| Ethernet72       | Disabled |
+------------------+----------+
| Ethernet76       | Disabled |
+------------------+----------+
| Ethernet80       | Disabled |
+------------------+----------+
| Ethernet84       | Disabled |
+------------------+----------+
| Ethernet88       | Disabled |
+------------------+----------+
| Ethernet92       | Disabled |
+------------------+----------+
| Ethernet96       | Disabled |
+------------------+----------+
| Ethernet100      | Disabled |
+------------------+----------+
| Ethernet104      | Disabled |
+------------------+----------+
| Ethernet108      | Disabled |
+------------------+----------+
| Ethernet112      | Disabled |
+------------------+----------+
| Ethernet116      | Disabled |
+------------------+----------+
| Ethernet120      | Disabled |
+------------------+----------+
| Ethernet124      | Disabled |
+------------------+----------+
| PortChannel1001  | Disabled |
+------------------+----------+
| PortChannel0001  | Disabled |
+------------------+----------+
| PortChannel0002  | Disabled |
+------------------+----------+
| PortChannel0003  | Disabled |
+------------------+----------+
| PortChannel0004  | Disabled |
+------------------+----------+
| Vlan1000         | Disabled |
+------------------+----------+
| Vlan2000         | Disabled |
+------------------+----------+
| Vlan3000         | Disabled |
+------------------+----------+
| Vlan4000         | Disabled |
+------------------+----------+"""


@pytest.mark.sonic_baseline
class TestIpv6LinkLocalMode:
    """
    Baseline test for 'show ipv6 link-local-mode' (Cmd #42).

    Mirrors sonic-utilities test:
      File: ipv6_link_local_test.py
      Func: test_show_ipv6_link_local_mode
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
