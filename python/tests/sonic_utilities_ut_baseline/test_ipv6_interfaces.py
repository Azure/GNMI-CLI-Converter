"""
Baseline UT: show ipv6 interfaces (Cmd #36)

Source:
  - sonic-utilities UT File : show_ip_int_test.py
  - sonic-utilities Test Func: test_show_ip_intf_v6
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #36

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 interfaces"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "admin_status": "error",
        "oper_status": "down",
        "master": "",
        "ipv6_addresses": [
            {
                "address": "2100::1/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            },
            {
                "address": "aa00::1/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            },
            {
                "address": "fe80::64be:a1ff:fe85:c6c4/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            }
        ]
    },
    "PortChannel0001": {
        "admin_status": "error",
        "oper_status": "down",
        "master": "",
        "ipv6_addresses": [
            {
                "address": "ab00::1/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            },
            {
                "address": "fe80::cc8d:60ff:fe08:139f/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            }
        ]
    },
    "Vlan100": {
        "admin_status": "error",
        "oper_status": "down",
        "master": "",
        "ipv6_addresses": [
            {
                "address": "cc00::1/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            },
            {
                "address": "fe80::c029:3fff:fe41:cf56/64",
                "bgp_neighbor_name": "N/A",
                "bgp_neighbor_ip": "N/A"
            }
        ]
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Interface        Master    IPv6 address/mask                             Admin/Oper    BGP Neighbor    Neighbor IP
---------------  --------  --------------------------------------------  ------------  --------------  -------------
Ethernet0                  2100::1/64                                    error/down    N/A             N/A
                           aa00::1/64                                                  N/A             N/A
                           fe80::64be:a1ff:fe85:c6c4%Ethernet0/64                      N/A             N/A
PortChannel0001            ab00::1/64                                    error/down    N/A             N/A
                           fe80::cc8d:60ff:fe08:139f%PortChannel0001/64                N/A             N/A
Vlan100                    cc00::1/64                                    error/down    N/A             N/A
                           fe80::c029:3fff:fe41:cf56%Vlan100/64                        N/A             N/A"""


@pytest.mark.sonic_baseline
class TestIpv6Interfaces:
    """
    Baseline test for 'show ipv6 interfaces' (Cmd #36).

    Mirrors sonic-utilities test:
      File: show_ip_int_test.py
      Func: test_show_ip_intf_v6
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
