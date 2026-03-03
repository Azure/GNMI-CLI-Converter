"""
Baseline UT: show dropcounters counts (Cmd #06)

Source:
  - sonic-utilities UT File : drops_group_test.py
  - sonic-utilities Test Func: test_show_counts
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #6

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show dropcounters counts"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "State": "D",
        "RX_ERR": "10",
        "RX_DROPS": "100",
        "TX_ERR": "0",
        "TX_DROPS": "0",
        "DEBUG_0": "80",
        "DEBUG_2": "20"
    },
    "Ethernet4": {
        "State": "N/A",
        "RX_ERR": "0",
        "RX_DROPS": "1000",
        "TX_ERR": "0",
        "TX_DROPS": "0",
        "DEBUG_0": "800",
        "DEBUG_2": "100"
    },
    "Ethernet8": {
        "State": "N/A",
        "RX_ERR": "100",
        "RX_DROPS": "10",
        "TX_ERR": "0",
        "TX_DROPS": "0",
        "DEBUG_0": "10",
        "DEBUG_2": "0"
    },
    "sonic_drops_test": {
        "SWITCH_DROPS": "1000",
        "lowercase_counter": "0"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
    IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS    DEBUG_0    DEBUG_2
---------  -------  --------  ----------  --------  ----------  ---------  ---------
Ethernet0        D        10         100         0           0         80         20
Ethernet4      N/A         0        1000         0           0        800        100
Ethernet8      N/A       100          10         0           0         10          0

          DEVICE    SWITCH_DROPS    lowercase_counter
----------------  --------------  -------------------
sonic_drops_test            1000                    0"""


@pytest.mark.sonic_baseline
class TestDropcountersCounts:
    """
    Baseline test for 'show dropcounters counts' (Cmd #06).

    Mirrors sonic-utilities test:
      File: drops_group_test.py
      Func: test_show_counts
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
