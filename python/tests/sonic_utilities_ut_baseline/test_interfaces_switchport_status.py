"""
Baseline UT: show interfaces switchport status (Cmd #27)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_switchport_status
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #27

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces switchport status"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Interface": "Ethernet0",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet4",
        "Mode": "trunk"
    },
    {
        "Interface": "Ethernet8",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet12",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet16",
        "Mode": "trunk"
    },
    {
        "Interface": "Ethernet20",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet24",
        "Mode": "trunk"
    },
    {
        "Interface": "Ethernet28",
        "Mode": "trunk"
    },
    {
        "Interface": "Ethernet36",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet40",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet44",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet48",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet52",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet56",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet60",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet64",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet68",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet72",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet76",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet80",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet84",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet88",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet92",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet96",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet100",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet104",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet108",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet116",
        "Mode": "routed"
    },
    {
        "Interface": "Ethernet124",
        "Mode": "routed"
    },
    {
        "Interface": "PortChannel0001",
        "Mode": "routed"
    },
    {
        "Interface": "PortChannel0002",
        "Mode": "routed"
    },
    {
        "Interface": "PortChannel0003",
        "Mode": "routed"
    },
    {
        "Interface": "PortChannel0004",
        "Mode": "routed"
    },
    {
        "Interface": "PortChannel1001",
        "Mode": "trunk"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Interface        Mode
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
PortChannel1001  trunk"""


@pytest.mark.sonic_baseline
class TestInterfacesSwitchportStatus:
    """
    Baseline test for 'show interfaces switchport status' (Cmd #27).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_switchport_status
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
