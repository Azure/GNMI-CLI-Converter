"""
Baseline UT: show interfaces neighbor expected (Cmd #23)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_neighbor_expected
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #23

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces neighbor expected"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet112": {
        "Neighbor": "ARISTA01T1",
        "NeighborPort": "Ethernet1",
        "NeighborLoopback": "None",
        "NeighborMgmt": "10.250.0.51",
        "NeighborType": "LeafRouter"
    },
    "Ethernet116": {
        "Neighbor": "ARISTA02T1",
        "NeighborPort": "Ethernet1",
        "NeighborLoopback": "None",
        "NeighborMgmt": "10.250.0.52",
        "NeighborType": "LeafRouter"
    },
    "Ethernet120": {
        "Neighbor": "ARISTA03T1",
        "NeighborPort": "Ethernet1",
        "NeighborLoopback": "None",
        "NeighborMgmt": "10.250.0.53",
        "NeighborType": "LeafRouter"
    },
    "Ethernet124": {
        "Neighbor": "ARISTA04T1",
        "NeighborPort": "Ethernet1",
        "NeighborLoopback": "None",
        "NeighborMgmt": "10.250.0.54",
        "NeighborType": "LeafRouter"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
LocalPort    Neighbor    NeighborPort    NeighborLoopback    NeighborMgmt    NeighborType
-----------  ----------  --------------  ------------------  --------------  --------------
Ethernet112  ARISTA01T1  Ethernet1       None                10.250.0.51     LeafRouter
Ethernet116  ARISTA02T1  Ethernet1       None                10.250.0.52     LeafRouter
Ethernet120  ARISTA03T1  Ethernet1       None                10.250.0.53     LeafRouter
Ethernet124  ARISTA04T1  Ethernet1       None                10.250.0.54     LeafRouter"""


@pytest.mark.sonic_baseline
class TestInterfacesNeighborExpected:
    """
    Baseline test for 'show interfaces neighbor expected' (Cmd #23).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_neighbor_expected
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
