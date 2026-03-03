"""
Baseline UT: show interfaces counters (Cmd #10)

Source:
  - sonic-utilities UT File : portstat_test.py
  - sonic-utilities Test Func: test_show_intf_counters
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #10

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "STATE": "D",
        "RX_OK": "8",
        "RX_BPS": "2000.00 MB/s",
        "RX_UTIL": "64.00%",
        "RX_ERR": "10",
        "RX_DRP": "100",
        "RX_OVR": "N/A",
        "TX_OK": "10",
        "TX_BPS": "1500.00 MB/s",
        "TX_UTIL": "48.00%",
        "TX_ERR": "N/A",
        "TX_DRP": "N/A",
        "TX_OVR": "N/A"
    },
    "Ethernet4": {
        "STATE": "N/A",
        "RX_OK": "4",
        "RX_BPS": "204.80 KB/s",
        "RX_UTIL": "N/A",
        "RX_ERR": "0",
        "RX_DRP": "1000",
        "RX_OVR": "N/A",
        "TX_OK": "40",
        "TX_BPS": "204.85 KB/s",
        "TX_UTIL": "N/A",
        "TX_ERR": "N/A",
        "TX_DRP": "N/A",
        "TX_OVR": "N/A"
    },
    "Ethernet8": {
        "STATE": "N/A",
        "RX_OK": "6",
        "RX_BPS": "1350.00 KB/s",
        "RX_UTIL": "N/A",
        "RX_ERR": "100",
        "RX_DRP": "10",
        "RX_OVR": "N/A",
        "TX_OK": "60",
        "TX_BPS": "13.37 MB/s",
        "TX_UTIL": "N/A",
        "TX_ERR": "N/A",
        "TX_DRP": "N/A",
        "TX_OVR": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
    IFACE    STATE    RX_OK        RX_BPS    RX_UTIL    RX_ERR    RX_DRP    RX_OVR    TX_OK        TX_BPS    TX_UTIL    TX_ERR    TX_DRP    TX_OVR
---------  -------  -------  ------------  ---------  --------  --------  --------  -------  ------------  ---------  --------  --------  --------
Ethernet0        D        8  2000.00 MB/s     64.00%        10       100       N/A       10  1500.00 MB/s     48.00%       N/A       N/A       N/A
Ethernet4      N/A        4   204.80 KB/s        N/A         0     1,000       N/A       40   204.85 KB/s        N/A       N/A       N/A       N/A
Ethernet8      N/A        6  1350.00 KB/s        N/A       100        10       N/A       60    13.37 MB/s        N/A       N/A       N/A       N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesCounters:
    """
    Baseline test for 'show interfaces counters' (Cmd #10).

    Mirrors sonic-utilities test:
      File: portstat_test.py
      Func: test_show_intf_counters
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
