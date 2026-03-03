"""
Baseline UT: show ipv6 bgp summary (Cmd #41)

Source:
  - sonic-utilities UT File : bgp_commands_test.py
  - sonic-utilities Test Func: test_bgp_summary_v6
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #41

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 bgp summary"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "ipv6Unicast": {
        "routerId": "10.1.0.32",
        "as": 65100,
        "vrfId": 0,
        "tableVersion": 8972,
        "ribCount": 12817,
        "ribMemory": 2358328,
        "peerCount": 24,
        "peerMemory": 502080,
        "peerGroupCount": 4,
        "peerGroupMemory": 256,
        "peers": {
            "fc00::1a": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6665,
                "msgSent": 6672,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA07T2"
            },
            "fc00::2": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6666,
                "msgSent": 7913,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA01T2"
            },
            "fc00::2a": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6666,
                "msgSent": 7913,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA11T2"
            },
            "fc00::3a": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6666,
                "msgSent": 7912,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA15T2"
            },
            "fc00::4a": {
                "version": 4,
                "remoteAs": 64003,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA03T0"
            },
            "fc00::4e": {
                "version": 4,
                "remoteAs": 64004,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA04T0"
            },
            "fc00::5a": {
                "version": 4,
                "remoteAs": 64007,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA07T0"
            },
            "fc00::5e": {
                "version": 4,
                "remoteAs": 64008,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA08T0"
            },
            "fc00::6a": {
                "version": 4,
                "remoteAs": 64011,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Connect",
                "NeighborName": "ARISTA11T0"
            },
            "fc00::6e": {
                "version": 4,
                "remoteAs": 64012,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA12T0"
            },
            "fc00::7a": {
                "version": 4,
                "remoteAs": 64015,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA15T0"
            },
            "fc00::7e": {
                "version": 4,
                "remoteAs": 64016,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA16T0"
            },
            "fc00::12": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6666,
                "msgSent": 7915,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA05T2"
            },
            "fc00::22": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6667,
                "msgSent": 7915,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h39m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA09T2"
            },
            "fc00::32": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6663,
                "msgSent": 6669,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h36m",
                "pfxRcd": 6402,
                "NeighborName": "ARISTA13T2"
            },
            "fc00::42": {
                "version": 4,
                "remoteAs": 64001,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA01T0"
            },
            "fc00::46": {
                "version": 4,
                "remoteAs": 64002,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA02T0"
            },
            "fc00::52": {
                "version": 4,
                "remoteAs": 64005,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA05T0"
            },
            "fc00::56": {
                "version": 4,
                "remoteAs": 64006,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA06T0"
            },
            "fc00::62": {
                "version": 4,
                "remoteAs": 64009,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA09T0"
            },
            "fc00::66": {
                "version": 4,
                "remoteAs": 64010,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA10T0"
            },
            "fc00::72": {
                "version": 4,
                "remoteAs": 64013,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "ARISTA13T0"
            },
            "fc00::76": {
                "version": 4,
                "remoteAs": 64014,
                "msgRcvd": 0,
                "msgSent": 0,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "never",
                "state": "Active",
                "NeighborName": "INT_NEIGH0"
            },
            "fc00::a": {
                "version": 4,
                "remoteAs": 65200,
                "msgRcvd": 6665,
                "msgSent": 6671,
                "tableVersion": 0,
                "inq": 0,
                "outq": 0,
                "peerUptime": "2d09h38m",
                "pfxRcd": 6402,
                "NeighborName": "INT_NEIGH1"
            },
            "Total numbe": {
                "version": 4,
                "remoteAs": "ighbo",
                "msgRcvd": 24,
                "msgSent": "",
                "tableVersion": "",
                "inq": "",
                "outq": "",
                "peerUptime": "",
                "state": "",
                "NeighborName": ""
            }
        }
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
IPv6 Unicast Summary:
BGP router identifier 10.1.0.32, local AS number 65100 vrf-id 0
BGP table version 8972
RIB entries 12817, using 2358328 bytes of memory
Peers 24, using 502080 KiB of memory
Peer groups 4, using 256 bytes of memory


Neighbhor      V  AS       MsgRcvd  MsgSent    TblVer    InQ    OutQ    Up/Down    State/PfxRcd    NeighborName
-----------  ---  -----  ---------  ---------  --------  -----  ------  ---------  --------------  --------------
Total numbe    4  ighbo         24
fc00::12       4  65200       6666  7915       0         0      0       2d09h39m   6402            ARISTA05T2
fc00::1a       4  65200       6665  6672       0         0      0       2d09h39m   6402            ARISTA07T2
fc00::2        4  65200       6666  7913       0         0      0       2d09h39m   6402            ARISTA01T2
fc00::22       4  65200       6667  7915       0         0      0       2d09h39m   6402            ARISTA09T2
fc00::2a       4  65200       6666  7913       0         0      0       2d09h39m   6402            ARISTA11T2
fc00::32       4  65200       6663  6669       0         0      0       2d09h36m   6402            ARISTA13T2
fc00::3a       4  65200       6666  7912       0         0      0       2d09h39m   6402            ARISTA15T2
fc00::42       4  64001          0  0          0         0      0       never      Active          ARISTA01T0
fc00::46       4  64002          0  0          0         0      0       never      Active          ARISTA02T0
fc00::4a       4  64003          0  0          0         0      0       never      Active          ARISTA03T0
fc00::4e       4  64004          0  0          0         0      0       never      Active          ARISTA04T0
fc00::52       4  64005          0  0          0         0      0       never      Active          ARISTA05T0
fc00::56       4  64006          0  0          0         0      0       never      Active          ARISTA06T0
fc00::5a       4  64007          0  0          0         0      0       never      Active          ARISTA07T0
fc00::5e       4  64008          0  0          0         0      0       never      Active          ARISTA08T0
fc00::62       4  64009          0  0          0         0      0       never      Active          ARISTA09T0
fc00::66       4  64010          0  0          0         0      0       never      Active          ARISTA10T0
fc00::6a       4  64011          0  0          0         0      0       never      Connect         ARISTA11T0
fc00::6e       4  64012          0  0          0         0      0       never      Active          ARISTA12T0
fc00::72       4  64013          0  0          0         0      0       never      Active          ARISTA13T0
fc00::76       4  64014          0  0          0         0      0       never      Active          INT_NEIGH0
fc00::7a       4  64015          0  0          0         0      0       never      Active          ARISTA15T0
fc00::7e       4  64016          0  0          0         0      0       never      Active          ARISTA16T0
fc00::a        4  65200       6665  6671       0         0      0       2d09h38m   6402            INT_NEIGH1"""


@pytest.mark.sonic_baseline
class TestIpv6BgpSummary:
    """
    Baseline test for 'show ipv6 bgp summary' (Cmd #41).

    Mirrors sonic-utilities test:
      File: bgp_commands_test.py
      Func: test_bgp_summary_v6
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
