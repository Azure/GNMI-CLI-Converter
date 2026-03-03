"""
Baseline UT: show interfaces transceiver status (Cmd #34)

Source:
  - sonic-utilities UT File : sfp_test.py
  - sonic-utilities Test Func: test_qsfp_status
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #34

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces transceiver status"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet4": "{\"Tx fault flag on media lane 1\": \"False\", \"Tx fault flag on media lane 2\": \"False\", \"Tx fault flag on media lane 3\": \"False\", \"Tx fault flag on media lane 4\": \"False\", \"Rx loss of signal flag on media lane 1\": \"False\", \"Rx loss of signal flag on media lane 2\": \"False\", \"Rx loss of signal flag on media lane 3\": \"False\", \"Rx loss of signal flag on media lane 4\": \"False\", \"TX disable status on lane 1\": \"False\", \"TX disable status on lane 2\": \"False\", \"TX disable status on lane 3\": \"False\", \"TX disable status on lane 4\": \"False\", \"Disabled TX channels\": \"0\"}"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Ethernet4:
        Tx fault flag on media lane 1: False
        Tx fault flag on media lane 2: False
        Tx fault flag on media lane 3: False
        Tx fault flag on media lane 4: False
        Rx loss of signal flag on media lane 1: False
        Rx loss of signal flag on media lane 2: False
        Rx loss of signal flag on media lane 3: False
        Rx loss of signal flag on media lane 4: False
        TX disable status on lane 1: False
        TX disable status on lane 2: False
        TX disable status on lane 3: False
        TX disable status on lane 4: False
        Disabled TX channels: 0"""


@pytest.mark.sonic_baseline
class TestInterfacesTransceiverStatus:
    """
    Baseline test for 'show interfaces transceiver status' (Cmd #34).

    Mirrors sonic-utilities test:
      File: sfp_test.py
      Func: test_qsfp_status
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
