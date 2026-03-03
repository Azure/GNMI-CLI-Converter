"""
Baseline UT: show queue persistent-watermark all (Cmd #55)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_queue_all_persistent_wm
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #55

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show queue persistent-watermark all"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "ALL8": "0",
        "ALL9": "0",
        "ALL10": "0",
        "ALL11": "0",
        "ALL12": "0",
        "ALL13": "0",
        "ALL14": "0",
        "ALL15": "0"
    },
    "Ethernet4": {
        "ALL8": "0",
        "ALL9": "0",
        "ALL10": "0",
        "ALL11": "0",
        "ALL12": "0",
        "ALL13": "0",
        "ALL14": "0",
        "ALL15": "0"
    },
    "Ethernet-BP0": {
        "ALL8": "0",
        "ALL9": "0",
        "ALL10": "0",
        "ALL11": "0",
        "ALL12": "0",
        "ALL13": "0",
        "ALL14": "0",
        "ALL15": "0"
    },
    "Ethernet-BP4": {
        "ALL8": "N/A",
        "ALL9": "N/A",
        "ALL10": "N/A",
        "ALL11": "N/A",
        "ALL12": "N/A",
        "ALL13": "N/A",
        "ALL14": "N/A",
        "ALL15": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Egress shared pool occupancy per all queues:
        Port    ALL8    ALL9    ALL10    ALL11    ALL12    ALL13    ALL14    ALL15
------------  ------  ------  -------  -------  -------  -------  -------  -------
   Ethernet0       0       0        0        0        0        0        0        0
   Ethernet4       0       0        0        0        0        0        0        0
Ethernet-BP0       0       0        0        0        0        0        0        0
Ethernet-BP4     N/A     N/A      N/A      N/A      N/A      N/A      N/A      N/A"""


@pytest.mark.sonic_baseline
class TestQueuePersistentWatermarkAll:
    """
    Baseline test for 'show queue persistent-watermark all' (Cmd #55).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_queue_all_persistent_wm
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
