"""
Baseline UT: show interfaces counters trim (Cmd #17)

Source:
  - sonic-utilities UT File : portstat_test.py
  - sonic-utilities Test Func: test_show_port_trim_counters
  - Expected CLI Output Var : trim_counters_all (from portstat_input/assert_show_output.py)
  - Mapping Doc Section     : ### #17

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters trim"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "STATE": "D",
        "TRIM_PKTS": "0",
        "TRIM_TX_PKTS": "0",
        "TRIM_DRP_PKTS": "0"
    },
    "Ethernet4": {
        "STATE": "N/A",
        "TRIM_PKTS": "100",
        "TRIM_TX_PKTS": "50",
        "TRIM_DRP_PKTS": "50"
    },
    "Ethernet8": {
        "STATE": "N/A",
        "TRIM_PKTS": "N/A",
        "TRIM_TX_PKTS": "N/A",
        "TRIM_DRP_PKTS": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
    IFACE    STATE    TRIM_PKTS    TRIM_TX_PKTS    TRIM_DRP_PKTS
---------  -------  -----------  --------------  ---------------
Ethernet0        D            0               0                0
Ethernet4      N/A          100              50               50
Ethernet8      N/A          N/A             N/A              N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesCountersTrim:
    """
    Baseline test for 'show interfaces counters trim' (Cmd #17).

    Mirrors sonic-utilities test:
      File: portstat_test.py
      Func: test_show_port_trim_counters
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
