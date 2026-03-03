"""
Baseline UT: show interfaces counters fec-stats (Cmd #14)

Source:
  - sonic-utilities UT File : portstat_test.py
  - sonic-utilities Test Func: test_show_intf_fec_counters
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #14

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters fec-stats"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "STATE": "D",
        "FEC_CORR": "130,402",
        "FEC_UNCORR": "3",
        "FEC_SYMBOL_ERR": "4",
        "FEC_PRE_BER": "N/A",
        "FEC_POST_BER": "N/A",
        "FEC_PRE_BER_MAX": "N/A"
    },
    "Ethernet4": {
        "STATE": "N/A",
        "FEC_CORR": "110,412",
        "FEC_UNCORR": "1",
        "FEC_SYMBOL_ERR": "0",
        "FEC_PRE_BER": "N/A",
        "FEC_POST_BER": "N/A",
        "FEC_PRE_BER_MAX": "N/A"
    },
    "Ethernet8": {
        "STATE": "N/A",
        "FEC_CORR": "100,317",
        "FEC_UNCORR": "0",
        "FEC_SYMBOL_ERR": "0",
        "FEC_PRE_BER": "N/A",
        "FEC_POST_BER": "N/A",
        "FEC_PRE_BER_MAX": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
    IFACE    STATE    FEC_CORR    FEC_UNCORR    FEC_SYMBOL_ERR    FEC_PRE_BER    FEC_POST_BER    FEC_PRE_BER_MAX
---------  -------  ----------  ------------  ----------------  -------------  --------------  -----------------
Ethernet0        D     130,402             3                 4            N/A             N/A                N/A
Ethernet4      N/A     110,412             1                 0            N/A             N/A                N/A
Ethernet8      N/A     100,317             0                 0            N/A             N/A                N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesCountersFecStats:
    """
    Baseline test for 'show interfaces counters fec-stats' (Cmd #14).

    Mirrors sonic-utilities test:
      File: portstat_test.py
      Func: test_show_intf_fec_counters
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
