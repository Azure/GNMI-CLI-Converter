"""
Baseline UT: show queue persistent-watermark multicast (Cmd #56)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_queue_multicast_persistent_wm
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #56

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show queue persistent-watermark multicast"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "MC8": "N/A",
        "MC9": "N/A",
        "MC10": "N/A",
        "MC11": "N/A",
        "MC12": "N/A",
        "MC13": "N/A",
        "MC14": "N/A",
        "MC15": "N/A"
    },
    "Ethernet4": {
        "MC8": "N/A",
        "MC9": "N/A",
        "MC10": "N/A",
        "MC11": "N/A",
        "MC12": "N/A",
        "MC13": "N/A",
        "MC14": "N/A",
        "MC15": "N/A"
    },
    "Ethernet-BP0": {
        "MC8": "N/A",
        "MC9": "N/A",
        "MC10": "N/A",
        "MC11": "N/A",
        "MC12": "N/A",
        "MC13": "N/A",
        "MC14": "N/A",
        "MC15": "N/A"
    },
    "Ethernet-BP4": {
        "MC8": "0",
        "MC9": "0",
        "MC10": "0",
        "MC11": "0",
        "MC12": "0",
        "MC13": "0",
        "MC14": "0",
        "MC15": "0"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Egress shared pool occupancy per multicast queue:
        Port    MC8    MC9    MC10    MC11    MC12    MC13    MC14    MC15
------------  -----  -----  ------  ------  ------  ------  ------  ------
   Ethernet0    N/A    N/A     N/A     N/A     N/A     N/A     N/A     N/A
   Ethernet4    N/A    N/A     N/A     N/A     N/A     N/A     N/A     N/A
Ethernet-BP0    N/A    N/A     N/A     N/A     N/A     N/A     N/A     N/A
Ethernet-BP4      0      0       0       0       0       0       0       0"""


@pytest.mark.sonic_baseline
class TestQueuePersistentWatermarkMc:
    """
    Baseline test for 'show queue persistent-watermark multicast' (Cmd #56).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_queue_multicast_persistent_wm
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
