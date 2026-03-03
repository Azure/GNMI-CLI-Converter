"""
Baseline UT: show ipv6 fib (Cmd #43)

Source:
  - sonic-utilities UT File : fibshow_test.py
  - sonic-utilities Test Func: test_show_ipv6_fib
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #43

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 fib"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "total": 3,
    "entries": [
        {
            "index": 1,
            "vrf": "",
            "route": "20c0:fe28:0:80::/64",
            "nexthop": "fc00::72,fc00::76,fc00::7a,fc00::7e",
            "ifname": "PortChannel101,PortChannel102,PortChannel103,PortChannel104"
        },
        {
            "index": 2,
            "vrf": "",
            "route": "20c0:fe28::/64",
            "nexthop": "fc00::72,fc00::76,fc00::7a,fc00::7e",
            "ifname": "PortChannel101,PortChannel102,PortChannel103,PortChannel104"
        },
        {
            "index": 3,
            "vrf": "",
            "route": "20c0:fe30:0:80::/64",
            "nexthop": "",
            "ifname": "PortChannel101,PortChannel102,PortChannel103,PortChannel104"
        }
    ]
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
  No.  Vrf    Route                Nexthop                              Ifname
-----  -----  -------------------  -----------------------------------  -----------------------------------------------------------
    1         20c0:fe28:0:80::/64  fc00::72,fc00::76,fc00::7a,fc00::7e  PortChannel101,PortChannel102,PortChannel103,PortChannel104
    2         20c0:fe28::/64       fc00::72,fc00::76,fc00::7a,fc00::7e  PortChannel101,PortChannel102,PortChannel103,PortChannel104
    3         20c0:fe30:0:80::/64                                       PortChannel101,PortChannel102,PortChannel103,PortChannel104
Total number of entries 3"""


@pytest.mark.sonic_baseline
class TestIpv6Fib:
    """
    Baseline test for 'show ipv6 fib' (Cmd #43).

    Mirrors sonic-utilities test:
      File: fibshow_test.py
      Func: test_show_ipv6_fib
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
