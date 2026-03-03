"""
Baseline UT: show vlan brief (Cmd #69)

Source:
  - sonic-utilities UT File : vlan_test.py
  - sonic-utilities Test Func: test_show_vlan_brief
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #69

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show vlan brief"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Vlan1000": {
        "vlan_id": "1000",
        "ip_address": [
            "192.168.0.1/21",
            "fc02:1000::1/64"
        ],
        "ports": [
            {
                "name": "Ethernet4",
                "port_tagging": "untagged"
            },
            {
                "name": "Ethernet8",
                "port_tagging": "untagged"
            },
            {
                "name": "Ethernet12",
                "port_tagging": "untagged"
            },
            {
                "name": "Ethernet16",
                "port_tagging": "untagged"
            }
        ],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": []
    },
    "Vlan2000": {
        "vlan_id": "2000",
        "ip_address": [
            "192.168.0.10/21",
            "fc02:1011::1/64"
        ],
        "ports": [
            {
                "name": "Ethernet24",
                "port_tagging": "untagged"
            },
            {
                "name": "Ethernet28",
                "port_tagging": "untagged"
            }
        ],
        "proxy_arp": "enabled",
        "dhcp_helper_addresses": []
    },
    "Vlan3000": {
        "vlan_id": "3000",
        "ip_address": [],
        "ports": [],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": []
    },
    "Vlan4000": {
        "vlan_id": "4000",
        "ip_address": [],
        "ports": [
            {
                "name": "PortChannel1001",
                "port_tagging": "tagged"
            }
        ],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": []
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
+-----------+-----------------+-----------------+----------------+-------------+
|   VLAN ID | IP Address      | Ports           | Port Tagging   | Proxy ARP   |
+===========+=================+=================+================+=============+
|      1000 | 192.168.0.1/21  | Ethernet4       | untagged       | disabled    |
|           | fc02:1000::1/64 | Ethernet8       | untagged       |             |
|           |                 | Ethernet12      | untagged       |             |
|           |                 | Ethernet16      | untagged       |             |
+-----------+-----------------+-----------------+----------------+-------------+
|      2000 | 192.168.0.10/21 | Ethernet24      | untagged       | enabled     |
|           | fc02:1011::1/64 | Ethernet28      | untagged       |             |
+-----------+-----------------+-----------------+----------------+-------------+
|      3000 |                 |                 |                | disabled    |
+-----------+-----------------+-----------------+----------------+-------------+
|      4000 |                 | PortChannel1001 | tagged         | disabled    |
+-----------+-----------------+-----------------+----------------+-------------+"""


@pytest.mark.sonic_baseline
class TestVlanBrief:
    """
    Baseline test for 'show vlan brief' (Cmd #69).

    Mirrors sonic-utilities test:
      File: vlan_test.py
      Func: test_show_vlan_brief
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
