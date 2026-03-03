"""
Baseline UT: show dropcounters capabilities (Cmd #05)

Source:
  - sonic-utilities UT File : drops_group_test.py
  - sonic-utilities Test Func: test_show_capabilities
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #5

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show dropcounters capabilities"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "PORT_INGRESS_DROPS": {
        "count": "4",
        "reasons": "[IP_HEADER_ERROR,NO_L3_HEADER]"
    },
    "SWITCH_EGRESS_DROPS": {
        "count": "2",
        "reasons": "[ACL_ANY,L2_ANY,L3_ANY]"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Counter Type           Total
-------------------  -------
PORT_INGRESS_DROPS         4
SWITCH_EGRESS_DROPS        2

PORT_INGRESS_DROPS
        IP_HEADER_ERROR
        NO_L3_HEADER

SWITCH_EGRESS_DROPS
        ACL_ANY
        L2_ANY
        L3_ANY"""


@pytest.mark.sonic_baseline
class TestDropcountersCapabilities:
    """
    Baseline test for 'show dropcounters capabilities' (Cmd #05).

    Mirrors sonic-utilities test:
      File: drops_group_test.py
      Func: test_show_capabilities
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
