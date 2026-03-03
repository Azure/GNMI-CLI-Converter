"""
Baseline UT: show interfaces fec status (Cmd #20)

Source:
  - sonic-utilities UT File : intfutil_test.py
  - sonic-utilities Test Func: test_show_interfaces_fec_status
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #20

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces fec status"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Interface": "Ethernet0",
        "FEC Oper": "N/A",
        "FEC Admin": "rs"
    },
    {
        "Interface": "Ethernet16",
        "FEC Oper": "N/A",
        "FEC Admin": "N/A"
    },
    {
        "Interface": "Ethernet24",
        "FEC Oper": "N/A",
        "FEC Admin": "N/A"
    },
    {
        "Interface": "Ethernet28",
        "FEC Oper": "N/A",
        "FEC Admin": "N/A"
    },
    {
        "Interface": "Ethernet32",
        "FEC Oper": "N/A",
        "FEC Admin": "rs"
    },
    {
        "Interface": "Ethernet36",
        "FEC Oper": "N/A",
        "FEC Admin": "N/A"
    },
    {
        "Interface": "Ethernet112",
        "FEC Oper": "N/A",
        "FEC Admin": "rs"
    },
    {
        "Interface": "Ethernet116",
        "FEC Oper": "N/A",
        "FEC Admin": "rs"
    },
    {
        "Interface": "Ethernet120",
        "FEC Oper": "N/A",
        "FEC Admin": "rs"
    },
    {
        "Interface": "Ethernet124",
        "FEC Oper": "rs",
        "FEC Admin": "auto"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
  Interface    FEC Oper    FEC Admin
-----------  ----------  -----------
  Ethernet0         N/A           rs
 Ethernet16         N/A          N/A
 Ethernet24         N/A          N/A
 Ethernet28         N/A          N/A
 Ethernet32         N/A           rs
 Ethernet36         N/A          N/A
Ethernet112         N/A           rs
Ethernet116         N/A           rs
Ethernet120         N/A           rs
Ethernet124          rs         auto"""


@pytest.mark.sonic_baseline
class TestInterfacesFecStatus:
    """
    Baseline test for 'show interfaces fec status' (Cmd #20).

    Mirrors sonic-utilities test:
      File: intfutil_test.py
      Func: test_show_interfaces_fec_status
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
