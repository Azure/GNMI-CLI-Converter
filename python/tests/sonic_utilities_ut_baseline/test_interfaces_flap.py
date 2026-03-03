"""
Baseline UT: show interfaces flap (Cmd #21)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_intf_flap_with_data
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #21

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces flap"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Interface": "Ethernet0",
        "Flap Count": "3",
        "Admin": "Up",
        "Oper": "Down",
        "Link Down TimeStamp(UTC)": "Sat Jan 17 00:04:42 2025",
        "Link Up TimeStamp(UTC)": "Sat Jan 18 00:08:42 2025"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Interface      Flap Count  Admin    Oper    Link Down TimeStamp(UTC)    Link Up TimeStamp(UTC)
-----------  ------------  -------  ------  --------------------------  ------------------------
Ethernet0               3  Up       Down    Sat Jan 17 00:04:42 2025    Sat Jan 18 00:08:42 2025"""


@pytest.mark.sonic_baseline
class TestInterfacesFlap:
    """
    Baseline test for 'show interfaces flap' (Cmd #21).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_intf_flap_with_data
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
