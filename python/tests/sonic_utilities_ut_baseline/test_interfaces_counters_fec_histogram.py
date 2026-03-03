"""
Baseline UT: show interfaces counters fec-histogram (Cmd #13)

Source:
  - sonic-utilities UT File : portstat_test.py
  - sonic-utilities Test Func: test_show_intf_counters_fec_histogram
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #13

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters fec-histogram"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "BinIndex": "BIN0",
        "Codewords": "1000000"
    },
    {
        "BinIndex": "BIN1",
        "Codewords": "900000"
    },
    {
        "BinIndex": "BIN2",
        "Codewords": "800000"
    },
    {
        "BinIndex": "BIN3",
        "Codewords": "700000"
    },
    {
        "BinIndex": "BIN4",
        "Codewords": "600000"
    },
    {
        "BinIndex": "BIN5",
        "Codewords": "500000"
    },
    {
        "BinIndex": "BIN6",
        "Codewords": "400000"
    },
    {
        "BinIndex": "BIN7",
        "Codewords": "300000"
    },
    {
        "BinIndex": "BIN8",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN9",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN10",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN11",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN12",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN13",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN14",
        "Codewords": "0"
    },
    {
        "BinIndex": "BIN15",
        "Codewords": "0"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Symbol Errors Per Codeword      Codewords
----------------------------  -----------
BIN0                              1000000
BIN1                               900000
BIN2                               800000
BIN3                               700000
BIN4                               600000
BIN5                               500000
BIN6                               400000
BIN7                               300000
BIN8                                    0
BIN9                                    0
BIN10                                   0
BIN11                                   0
BIN12                                   0
BIN13                                   0
BIN14                                   0
BIN15                                   0"""


@pytest.mark.sonic_baseline
class TestInterfacesCountersFecHistogram:
    """
    Baseline test for 'show interfaces counters fec-histogram' (Cmd #13).

    Mirrors sonic-utilities test:
      File: portstat_test.py
      Func: test_show_intf_counters_fec_histogram
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
