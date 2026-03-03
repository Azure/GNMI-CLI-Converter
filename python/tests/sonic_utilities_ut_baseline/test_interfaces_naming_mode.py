"""
Baseline UT: show interfaces naming_mode (Cmd #22)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_interfaces_naming_mode_default
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #22

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces naming_mode"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "naming_mode": "default"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
default"""


@pytest.mark.sonic_baseline
class TestInterfacesNamingMode:
    """
    Baseline test for 'show interfaces naming_mode' (Cmd #22).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_interfaces_naming_mode_default
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
