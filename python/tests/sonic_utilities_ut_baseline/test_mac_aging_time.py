"""
Baseline UT: show mac aging_time (Cmd #47)

Source:
  - sonic-utilities UT File : fdbshow_test.py
  - sonic-utilities Test Func: test_show_mac_aging_time
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #47

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show mac aging_time"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "fdb_aging_time": "600s"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Aging time for switch is 600 seconds"""


@pytest.mark.sonic_baseline
class TestMacAgingTime:
    """
    Baseline test for 'show mac aging_time' (Cmd #47).

    Mirrors sonic-utilities test:
      File: fdbshow_test.py
      Func: test_show_mac_aging_time
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
