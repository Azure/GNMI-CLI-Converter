"""
Baseline UT: show interfaces description (Cmd #18)

Source:
  - sonic-utilities UT File : intfutil_test.py
  - sonic-utilities Test Func: test_show_interfaces_description
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #18

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces description"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "Oper": "down",
        "Admin": "up",
        "Alias": "Ethernet0",
        "Description": "ARISTA01T2:Ethernet1"
    },
    "Ethernet16": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp5",
        "Description": "ARISTA04T1:Ethernet16"
    },
    "Ethernet24": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp6",
        "Description": "ARISTA02T1:Ethernet24"
    },
    "Ethernet28": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp8",
        "Description": "ARISTA03T1:Ethernet28"
    },
    "Ethernet32": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp9",
        "Description": "Servers7:eth0"
    },
    "Ethernet36": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp10",
        "Description": "Servers8:eth0"
    },
    "Ethernet112": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp29",
        "Description": "ARISTA01T1:Ethernet1"
    },
    "Ethernet116": {
        "Oper": "up",
        "Admin": "up",
        "Alias": "etp30",
        "Description": "ARISTA02T1:Ethernet1"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
  Interface    Oper    Admin      Alias            Description
-----------  ------  -------  ---------  ---------------------
  Ethernet0    down       up  Ethernet0   ARISTA01T2:Ethernet1
 Ethernet16      up       up       etp5  ARISTA04T1:Ethernet16
 Ethernet24      up       up       etp6  ARISTA02T1:Ethernet24
 Ethernet28      up       up       etp8  ARISTA03T1:Ethernet28
 Ethernet32      up       up       etp9          Servers7:eth0
 Ethernet36      up       up      etp10          Servers8:eth0
Ethernet112      up       up      etp29   ARISTA01T1:Ethernet1
Ethernet116      up       up      etp30   ARISTA02T1:Ethernet1"""


@pytest.mark.sonic_baseline
class TestInterfacesDescription:
    """
    Baseline test for 'show interfaces description' (Cmd #18).

    Mirrors sonic-utilities test:
      File: intfutil_test.py
      Func: test_show_interfaces_description
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
