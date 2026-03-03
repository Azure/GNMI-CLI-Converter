"""
Baseline UT: show interfaces switchport config (Cmd #26)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_switchport_config
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #26

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces switchport config"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Interface": "Ethernet0",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet4",
        "Mode": "trunk",
        "Untagged": "1000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet8",
        "Mode": "routed",
        "Untagged": "1000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet12",
        "Mode": "routed",
        "Untagged": "1000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet16",
        "Mode": "trunk",
        "Untagged": "1000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet20",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet24",
        "Mode": "trunk",
        "Untagged": "2000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet28",
        "Mode": "trunk",
        "Untagged": "2000",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet36",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet40",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet44",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet48",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet52",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet56",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet60",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet64",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet68",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet72",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet76",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet80",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet84",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet88",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet92",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet96",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet100",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet104",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet108",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet116",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "Ethernet124",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "PortChannel0001",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "PortChannel0002",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "PortChannel0003",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "PortChannel0004",
        "Mode": "routed",
        "Untagged": "",
        "Tagged": ""
    },
    {
        "Interface": "PortChannel1001",
        "Mode": "trunk",
        "Untagged": "",
        "Tagged": "4000"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Interface        Mode    Untagged    Tagged
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
PortChannel1001  trunk               4000"""


@pytest.mark.sonic_baseline
class TestInterfacesSwitchportConfig:
    """
    Baseline test for 'show interfaces switchport config' (Cmd #26).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_switchport_config
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
