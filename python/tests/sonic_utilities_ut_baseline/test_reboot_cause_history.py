"""
Baseline UT: show reboot-cause history (Cmd #63)

Source:
  - sonic-utilities UT File : reboot_cause_test.py
  - sonic-utilities Test Func: test_reboot_cause_history
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #63

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show reboot-cause history"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "2020_10_09_04_53_58": {
        "cause": "warm-reboot",
        "time": "Fri Oct  9 04:51:47 UTC 2020",
        "user": "admin",
        "comment": "N/A"
    },
    "2020_10_09_02_33_06": {
        "cause": "reboot",
        "time": "Fri Oct  9 02:29:44 UTC 2020",
        "user": "admin",
        "comment": "N/A"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Name                 Cause        Time                          User    Comment
-------------------  -----------  ----------------------------  ------  ---------
2020_10_09_04_53_58  warm-reboot  Fri Oct  9 04:51:47 UTC 2020  admin   N/A
2020_10_09_02_33_06  reboot       Fri Oct  9 02:29:44 UTC 2020  admin   N/A"""


@pytest.mark.sonic_baseline
class TestRebootCauseHistory:
    """
    Baseline test for 'show reboot-cause history' (Cmd #63).

    Mirrors sonic-utilities test:
      File: reboot_cause_test.py
      Func: test_reboot_cause_history
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
