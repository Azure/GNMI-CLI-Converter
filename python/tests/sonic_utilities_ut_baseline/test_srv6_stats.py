"""
Baseline UT: show srv6 stats (Cmd #65)

Source:
  - sonic-utilities UT File : srv6stat_test.py
  - sonic-utilities Test Func: test_show
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #65

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show srv6 stats"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "1000:2:30::/48": {
        "packets": "100",
        "bytes": "102400"
    },
    "2000:2:30::/48": {
        "packets": "200",
        "bytes": "204800"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
MySID             Packets    Bytes
--------------  ---------  -------
1000:2:30::/48        100   102400
2000:2:30::/48        200   204800"""


@pytest.mark.sonic_baseline
class TestSrv6Stats:
    """
    Baseline test for 'show srv6 stats' (Cmd #65).

    Mirrors sonic-utilities test:
      File: srv6stat_test.py
      Func: test_show
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
