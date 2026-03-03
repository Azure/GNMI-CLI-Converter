"""
Baseline UT: show interfaces alias (Cmd #09)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_alias
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #9

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces alias"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "PORT|Ethernet0": {
        "alias": "etp1"
    },
    "PORT|Ethernet4": {
        "alias": "etp2"
    },
    "PORT|Ethernet8": {
        "alias": "etp3"
    },
    "PORT|Ethernet12": {
        "alias": "etp4"
    },
    "PORT|Ethernet16": {
        "alias": "etp5"
    },
    "PORT|Ethernet20": {
        "alias": "etp6"
    },
    "PORT|Ethernet24": {
        "alias": "etp7"
    },
    "PORT|Ethernet28": {
        "alias": "etp8"
    },
    "PORT|Ethernet32": {
        "alias": "etp9"
    },
    "PORT|Ethernet36": {
        "alias": "etp10"
    },
    "PORT|Ethernet40": {
        "alias": "etp11"
    },
    "PORT|Ethernet44": {
        "alias": "etp12"
    },
    "PORT|Ethernet48": {
        "alias": "etp13"
    },
    "PORT|Ethernet52": {
        "alias": "etp14"
    },
    "PORT|Ethernet56": {
        "alias": "etp15"
    },
    "PORT|Ethernet60": {
        "alias": "etp16"
    },
    "PORT|Ethernet64": {
        "alias": "etp17"
    },
    "PORT|Ethernet68": {
        "alias": "etp18"
    },
    "PORT|Ethernet72": {
        "alias": "etp19"
    },
    "PORT|Ethernet76": {
        "alias": "etp20"
    },
    "PORT|Ethernet80": {
        "alias": "etp21"
    },
    "PORT|Ethernet84": {
        "alias": "etp22"
    },
    "PORT|Ethernet88": {
        "alias": "etp23"
    },
    "PORT|Ethernet92": {
        "alias": "etp24"
    },
    "PORT|Ethernet96": {
        "alias": "etp25"
    },
    "PORT|Ethernet100": {
        "alias": "etp26"
    },
    "PORT|Ethernet104": {
        "alias": "etp27"
    },
    "PORT|Ethernet108": {
        "alias": "etp28"
    },
    "PORT|Ethernet112": {
        "alias": "etp29"
    },
    "PORT|Ethernet116": {
        "alias": "etp30"
    },
    "PORT|Ethernet120": {
        "alias": "etp31"
    },
    "PORT|Ethernet124": {
        "alias": "etp32"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Name         Alias
-----------  -------
Ethernet0    etp1
Ethernet4    etp2
Ethernet8    etp3
Ethernet12   etp4
Ethernet16   etp5
Ethernet20   etp6
Ethernet24   etp7
Ethernet28   etp8
Ethernet32   etp9
Ethernet36   etp10
Ethernet40   etp11
Ethernet44   etp12
Ethernet48   etp13
Ethernet52   etp14
Ethernet56   etp15
Ethernet60   etp16
Ethernet64   etp17
Ethernet68   etp18
Ethernet72   etp19
Ethernet76   etp20
Ethernet80   etp21
Ethernet84   etp22
Ethernet88   etp23
Ethernet92   etp24
Ethernet96   etp25
Ethernet100  etp26
Ethernet104  etp27
Ethernet108  etp28
Ethernet112  etp29
Ethernet116  etp30
Ethernet120  etp31
Ethernet124  etp32"""


@pytest.mark.sonic_baseline
class TestInterfacesAlias:
    """
    Baseline test for 'show interfaces alias' (Cmd #09).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_alias
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
