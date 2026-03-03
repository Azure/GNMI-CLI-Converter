"""
Baseline UT: show queue persistent-watermark unicast (Cmd #57)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_queue_unicast_persistent_wm
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #57

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show queue persistent-watermark unicast"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "UC0": "N/A",
        "UC1": "N/A",
        "UC2": "N/A",
        "UC3": "N/A",
        "UC4": "N/A",
        "UC5": "N/A",
        "UC6": "N/A",
        "UC7": "N/A"
    },
    "Ethernet4": {
        "UC0": "N/A",
        "UC1": "N/A",
        "UC2": "N/A",
        "UC3": "N/A",
        "UC4": "N/A",
        "UC5": "N/A",
        "UC6": "N/A",
        "UC7": "N/A"
    },
    "Ethernet-BP0": {
        "UC0": "N/A",
        "UC1": "N/A",
        "UC2": "N/A",
        "UC3": "N/A",
        "UC4": "N/A",
        "UC5": "N/A",
        "UC6": "N/A",
        "UC7": "N/A"
    },
    "Ethernet-BP4": {
        "UC0": "N/A",
        "UC1": "N/A",
        "UC2": "N/A",
        "UC3": "N/A",
        "UC4": "N/A",
        "UC5": "N/A",
        "UC6": "N/A",
        "UC7": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Egress shared pool occupancy per unicast queue:
        Port    UC0    UC1    UC2    UC3    UC4    UC5    UC6    UC7
------------  -----  -----  -----  -----  -----  -----  -----  -----
   Ethernet0    N/A    N/A    N/A    N/A    N/A    N/A    N/A    N/A
   Ethernet4    N/A    N/A    N/A    N/A    N/A    N/A    N/A    N/A
Ethernet-BP0    N/A    N/A    N/A    N/A    N/A    N/A    N/A    N/A
Ethernet-BP4    N/A    N/A    N/A    N/A    N/A    N/A    N/A    N/A"""


@pytest.mark.sonic_baseline
class TestQueuePersistentWatermarkUc:
    """
    Baseline test for 'show queue persistent-watermark unicast' (Cmd #57).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_queue_unicast_persistent_wm
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
