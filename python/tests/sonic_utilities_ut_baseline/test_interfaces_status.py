"""
Baseline UT: show interfaces status (Cmd #25)

Source:
  - sonic-utilities UT File : intfutil_test.py
  - sonic-utilities Test Func: test_intf_status
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #25

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces status"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Interface": "Ethernet0",
        "Lanes": "0",
        "Speed": "25G",
        "MTU": "9100",
        "FEC": "rs",
        "Alias": "Ethernet0",
        "Vlan": "routed",
        "Oper": "down",
        "Admin": "up",
        "Type": "QSFP28 or later",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet16",
        "Lanes": "16",
        "Speed": "100M",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "etp5",
        "Vlan": "trunk",
        "Oper": "up",
        "Admin": "up",
        "Type": "RJ45",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet24",
        "Lanes": "24",
        "Speed": "1G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "etp6",
        "Vlan": "trunk",
        "Oper": "up",
        "Admin": "up",
        "Type": "DPU-NPU Data Port",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet28",
        "Lanes": "28",
        "Speed": "1000M",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "etp8",
        "Vlan": "trunk",
        "Oper": "up",
        "Admin": "up",
        "Type": "RJ45",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet32",
        "Lanes": "13,14,15,16",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "rs",
        "Alias": "etp9",
        "Vlan": "PortChannel1001",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet36",
        "Lanes": "9,10,11,12",
        "Speed": "10M",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "etp10",
        "Vlan": "routed",
        "Oper": "up",
        "Admin": "up",
        "Type": "RJ45",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet112",
        "Lanes": "93,94,95,96",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "rs",
        "Alias": "etp29",
        "Vlan": "PortChannel0001",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet116",
        "Lanes": "89,90,91,92",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "rs",
        "Alias": "etp30",
        "Vlan": "PortChannel0002",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet120",
        "Lanes": "101,102,103,104",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "rs",
        "Alias": "etp31",
        "Vlan": "PortChannel0003",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "off"
    },
    {
        "Interface": "Ethernet124",
        "Lanes": "97,98,99,100",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "auto",
        "Alias": "etp32",
        "Vlan": "PortChannel0004",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "off"
    },
    {
        "Interface": "PortChannel0001",
        "Lanes": "N/A",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "N/A",
        "Vlan": "routed",
        "Oper": "down",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "N/A"
    },
    {
        "Interface": "PortChannel0002",
        "Lanes": "N/A",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "N/A",
        "Vlan": "routed",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "N/A"
    },
    {
        "Interface": "PortChannel0003",
        "Lanes": "N/A",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "N/A",
        "Vlan": "routed",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "N/A"
    },
    {
        "Interface": "PortChannel0004",
        "Lanes": "N/A",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "N/A",
        "Vlan": "routed",
        "Oper": "up",
        "Admin": "up",
        "Type": "N/A",
        "Asym": "N/A"
    },
    {
        "Interface": "PortChannel1001",
        "Lanes": "N/A",
        "Speed": "40G",
        "MTU": "9100",
        "FEC": "N/A",
        "Alias": "N/A",
        "Vlan": "trunk",
        "Oper": "N/A",
        "Admin": "N/A",
        "Type": "N/A",
        "Asym": "N/A"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
      Interface            Lanes    Speed    MTU    FEC      Alias             Vlan    Oper    Admin               Type    Asym PFC
---------------  ---------------  -------  -----  -----  ---------  ---------------  ------  -------  -----------------  ----------
      Ethernet0                0      25G   9100     rs  Ethernet0           routed    down       up    QSFP28 or later         off
     Ethernet16               16     100M   9100    N/A       etp5            trunk      up       up               RJ45         off
     Ethernet24               24       1G   9100    N/A       etp6            trunk      up       up  DPU-NPU Data Port         off
     Ethernet28               28    1000M   9100    N/A       etp8            trunk      up       up               RJ45         off
     Ethernet32      13,14,15,16      40G   9100     rs       etp9  PortChannel1001      up       up                N/A         off
     Ethernet36       9,10,11,12      10M   9100    N/A      etp10           routed      up       up               RJ45         off
    Ethernet112      93,94,95,96      40G   9100     rs      etp29  PortChannel0001      up       up                N/A         off
    Ethernet116      89,90,91,92      40G   9100     rs      etp30  PortChannel0002      up       up                N/A         off
    Ethernet120  101,102,103,104      40G   9100     rs      etp31  PortChannel0003      up       up                N/A         off
    Ethernet124     97,98,99,100      40G   9100   auto      etp32  PortChannel0004      up       up                N/A         off
PortChannel0001              N/A      40G   9100    N/A        N/A           routed    down       up                N/A         N/A
PortChannel0002              N/A      40G   9100    N/A        N/A           routed      up       up                N/A         N/A
PortChannel0003              N/A      40G   9100    N/A        N/A           routed      up       up                N/A         N/A
PortChannel0004              N/A      40G   9100    N/A        N/A           routed      up       up                N/A         N/A
PortChannel1001              N/A      40G   9100    N/A        N/A            trunk     N/A      N/A                N/A         N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesStatus:
    """
    Baseline test for 'show interfaces status' (Cmd #25).

    Mirrors sonic-utilities test:
      File: intfutil_test.py
      Func: test_intf_status
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
