"""
Baseline UT: show ipv6 bgp network (Cmd #40)

Source:
  - sonic-utilities UT File : show_bgp_network_test.py
  - sonic-utilities Test Func: test_bgp_network
  - Expected CLI Output Var : bgp_v6_network (from bgp_commands_input/bgp_network_test_vector.py)
  - Mapping Doc Section     : ### #40

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 bgp network"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
  "output": (
    "BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0\n"
    "Default local pref 100, local AS 65100\n"
    "Status codes:  s suppressed, d damped, h history, * valid, > best, = multipath,\n"
    "               i internal, r RIB-failure, S Stale, R Removed\n"
    "Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self\n"
    "Origin codes:  i - IGP, e - EGP, ? - incomplete\n"
    "\n"
    "   Network          Next Hop            Metric LocPrf Weight Path\n"
    "*= ::/0             fc00::7e                               0 64600 65534 6666 6667 i\n"
    "*=                  fc00::7a                               0 64600 65534 6666 6667 i\n"
    "*=                  fc00::76                               0 64600 65534 6666 6667 i\n"
    "*>                  fc00::72                               0 64600 65534 6666 6667 i\n"
    "*> 2064:100::1d/128 fc00::72                               0 64600 i\n"
    "*> 2064:100::1e/128 fc00::76                               0 64600 i\n"
    "*> 2064:100::1f/128 fc00::7a                               0 64600 i\n"
    "*> 2064:100::20/128 fc00::7e                               0 64600 i\n"
    "*= 20c0:a808::/64   fc00::7e                               0 64600 65501 i\n"
    "*=                  fc00::7a                               0 64600 65501 i\n"
    "*=                  fc00::76                               0 64600 65501 i\n"
    "*>                  fc00::72                               0 64600 65501 i\n"
    "*= 20c0:a808:0:80::/64\n"
    "                    fc00::7e                               0 64600 65501 i\n"
    "*=                  fc00::7a                               0 64600 65501 i\n"
    "*=                  fc00::76                               0 64600 65501 i\n"
    "*>                  fc00::72                               0 64600 65501 i\n"
    "*= 20c0:a810::/64   fc00::7e                               0 64600 65502 i\n"
    "*=                  fc00::7a                               0 64600 65502 i\n"
    "*=                  fc00::76                               0 64600 65502 i\n"
    "*>                  fc00::72                               0 64600 65502 i\n"
    "*= 20c0:a810:0:80::/64\n"
    "                    fc00::7e                               0 64600 65502 i\n"
    "*=                  fc00::7a                               0 64600 65502 i\n"
    "*=                  fc00::76                               0 64600 65502 i\n"
    "*>                  fc00::72                               0 64600 65502 i\n"
    "*= 20c0:a818::/64   fc00::7e                               0 64600 65503 i\n"
    "*=                  fc00::7a                               0 64600 65503 i\n"
    "*=                  fc00::76                               0 64600 65503 i\n"
    "*>                  fc00::72                               0 64600 65503 i\n"
    "*= 20c0:a818:0:80::/64\n"
    "                    fc00::7e                               0 64600 65503 i\n"
    "*=                  fc00::7a                               0 64600 65503 i\n"
    "*=                  fc00::76                               0 64600 65503 i\n"
    "*>                  fc00::72                               0 64600 65503 i\n"
    "*= 20c0:a820::/64   fc00::7e                               0 64600 65504 i\n"
    "*=                  fc00::7a                               0 64600 65504 i\n"
    "*=                  fc00::76                               0 64600 65504 i\n"
    "*>                  fc00::72                               0 64600 65504 i\n"
    "*= 20c0:a820:0:80::/64\n"
    "                    fc00::7e                               0 64600 65504 i\n"
    "*=                  fc00::7a                               0 64600 65504 i\n"
    "*=                  fc00::76                               0 64600 65504 i\n"
    "*>                  fc00::72                               0 64600 65504 i"
  )
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
BGP table version is 6407, local router ID is 10.1.0.32, vrf id 0
Default local pref 100, local AS 65100
Status codes:  s suppressed, d damped, h history, * valid, > best, = multipath,
               i internal, r RIB-failure, S Stale, R Removed
Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self
Origin codes:  i - IGP, e - EGP, ? - incomplete

   Network          Next Hop            Metric LocPrf Weight Path
*= ::/0             fc00::7e                               0 64600 65534 6666 6667 i
*=                  fc00::7a                               0 64600 65534 6666 6667 i
*=                  fc00::76                               0 64600 65534 6666 6667 i
*>                  fc00::72                               0 64600 65534 6666 6667 i
*> 2064:100::1d/128 fc00::72                               0 64600 i
*> 2064:100::1e/128 fc00::76                               0 64600 i
*> 2064:100::1f/128 fc00::7a                               0 64600 i
*> 2064:100::20/128 fc00::7e                               0 64600 i
*= 20c0:a808::/64   fc00::7e                               0 64600 65501 i
*=                  fc00::7a                               0 64600 65501 i
*=                  fc00::76                               0 64600 65501 i
*>                  fc00::72                               0 64600 65501 i
*= 20c0:a808:0:80::/64
                    fc00::7e                               0 64600 65501 i
*=                  fc00::7a                               0 64600 65501 i
*=                  fc00::76                               0 64600 65501 i
*>                  fc00::72                               0 64600 65501 i
*= 20c0:a810::/64   fc00::7e                               0 64600 65502 i
*=                  fc00::7a                               0 64600 65502 i
*=                  fc00::76                               0 64600 65502 i
*>                  fc00::72                               0 64600 65502 i
*= 20c0:a810:0:80::/64
                    fc00::7e                               0 64600 65502 i
*=                  fc00::7a                               0 64600 65502 i
*=                  fc00::76                               0 64600 65502 i
*>                  fc00::72                               0 64600 65502 i
*= 20c0:a818::/64   fc00::7e                               0 64600 65503 i
*=                  fc00::7a                               0 64600 65503 i
*=                  fc00::76                               0 64600 65503 i
*>                  fc00::72                               0 64600 65503 i
*= 20c0:a818:0:80::/64
                    fc00::7e                               0 64600 65503 i
*=                  fc00::7a                               0 64600 65503 i
*=                  fc00::76                               0 64600 65503 i
*>                  fc00::72                               0 64600 65503 i
*= 20c0:a820::/64   fc00::7e                               0 64600 65504 i
*=                  fc00::7a                               0 64600 65504 i
*=                  fc00::76                               0 64600 65504 i
*>                  fc00::72                               0 64600 65504 i
*= 20c0:a820:0:80::/64
                    fc00::7e                               0 64600 65504 i
*=                  fc00::7a                               0 64600 65504 i
*=                  fc00::76                               0 64600 65504 i
*>                  fc00::72                               0 64600 65504 i"""


@pytest.mark.sonic_baseline
class TestIpv6BgpNetwork:
    """
    Baseline test for 'show ipv6 bgp network' (Cmd #40).

    Mirrors sonic-utilities test:
      File: show_bgp_network_test.py
      Func: test_bgp_network
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
