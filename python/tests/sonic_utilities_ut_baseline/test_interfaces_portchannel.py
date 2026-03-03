"""
Baseline UT: show interfaces portchannel (Cmd #24)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_portchannel
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #24

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces portchannel"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "0001": {
        "Team Dev": "PortChannel0001",
        "Protocol": {
            "name": "LACP",
            "active": True,
            "operational_status": "down"
        },
        "Ports": [
            {
                "name": "Ethernet112",
                "selected": False,
                "in_sync": True,
                "status": "enabled"
            }
        ]
    },
    "0002": {
        "Team Dev": "PortChannel0002",
        "Protocol": {
            "name": "LACP",
            "active": True,
            "operational_status": "up"
        },
        "Ports": [
            {
                "name": "Ethernet116",
                "selected": True,
                "in_sync": True,
                "status": "enabled"
            }
        ]
    },
    "0003": {
        "Team Dev": "PortChannel0003",
        "Protocol": {
            "name": "LACP",
            "active": True,
            "operational_status": "up"
        },
        "Ports": [
            {
                "name": "Ethernet120",
                "selected": True,
                "in_sync": True,
                "status": "enabled"
            }
        ]
    },
    "0004": {
        "Team Dev": "PortChannel0004",
        "Protocol": {
            "name": "LACP",
            "active": True,
            "operational_status": "up"
        },
        "Ports": [
            "N/A"
        ]
    },
    "1001": {
        "Team Dev": "PortChannel1001",
        "Protocol": "N/A",
        "Ports": []
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Flags: A - active, I - inactive, Up - up, Dw - Down, N/A - not available,
       S - selected, D - deselected, * - not synced
  No.  Team Dev         Protocol     Ports
-----  ---------------  -----------  --------------
 0001  PortChannel0001  LACP(A)(Dw)  Ethernet112(D)
 0002  PortChannel0002  LACP(A)(Up)  Ethernet116(S)
 0003  PortChannel0003  LACP(A)(Up)  Ethernet120(S)
 0004  PortChannel0004  LACP(A)(Up)  N/A
 1001  PortChannel1001  N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesPortchannel:
    """
    Baseline test for 'show interfaces portchannel' (Cmd #24).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_portchannel
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
