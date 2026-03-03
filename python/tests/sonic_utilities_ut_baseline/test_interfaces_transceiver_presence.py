"""
Baseline UT: show interfaces transceiver presence (Cmd #33)

Source:
  - sonic-utilities UT File : sfp_test.py
  - sonic-utilities Test Func: test_sfp_presence_all
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #33

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces transceiver presence"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": "Present",
    "Ethernet4": "Not present",
    "Ethernet64": "Present"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Port        Presence
----------  -----------
Ethernet0   Present
Ethernet4   Not present
Ethernet64  Present"""


@pytest.mark.sonic_baseline
class TestInterfacesTransceiverPresence:
    """
    Baseline test for 'show interfaces transceiver presence' (Cmd #33).

    Mirrors sonic-utilities test:
      File: sfp_test.py
      Func: test_sfp_presence_all
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
