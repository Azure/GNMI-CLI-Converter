"""
Baseline UT: show ipv6 bgp neighbors (Cmd #39)

Source:
  - sonic-utilities UT File : show_bgp_neighbor_test.py
  - sonic-utilities Test Func: test_bgp_neighbors
  - Expected CLI Output Var : bgp_v6_neighbors_output (from bgp_commands_input/bgp_neighbor_test_vector.py)
  - Mapping Doc Section     : ### #39

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 bgp neighbors"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "fc00::72": {
        "remoteAs": 64600,
        "localAs": 65100,
        "nbrExternalLink": True,
        "nbrDesc": "ARISTA01T1",
        "peerGroup": "PEER_V6",
        "bgpVersion": 4,
        "remoteRouterId": "100.1.0.29",
        "localRouterId": "10.1.0.32",
        "bgpState": "Established",
        "bgpTimerUpString": "01:06:23",
        "bgpTimerLastRead": 2000,
        "bgpTimerLastWrite": 0,
        "bgpTimerHoldTimeMsecs": 10000,
        "bgpTimerKeepAliveIntervalMsecs": 3000,
        "bgpTimerConfiguredHoldTimeMsecs": 10000,
        "bgpTimerConfiguredKeepAliveIntervalMsecs": 3000,
        "neighborCapabilities": {
            "4byteAs": "advertisedAndReceived",
            "addPath": {
                "ipv6Unicast": {
                    "rxAdvertisedAndReceived": True
                }
            },
            "routeRefresh": "advertisedAndReceivedNew",
            "multiprotocolExtensions": {
                "ipv6Unicast": "advertisedAndReceived"
            },
            "hostName": {
                "advHostName": "vlab-01",
                "advDomainName": "n/a",
                "rcvHostName": ""
            },
            "gracefulRestart": "advertisedAndReceived",
            "gracefulRestartRemoteTimerMsecs": 300000,
            "addressFamiliesByPeer": "none"
        },
        "gracefulRestartInfo": {
            "endOfRibSend": {
                "ipv6Unicast": True
            },
            "endOfRibRecv": {
                "ipv6Unicast": True
            },
            "localGrMode": "Restart*",
            "remoteGrMode": "Helper",
            "rBit": False,
            "timers": {
                "configuredRestartTimer": 240,
                "receivedRestartTimer": 300
            },
            "ipv6Unicast": {
                "fBit": False,
                "endOfRibStatus": {
                    "endOfRibSend": True,
                    "endOfRibSentAfterUpdate": False,
                    "endOfRibRecv": True
                },
                "timers": {
                    "stalePathTimer": 360,
                    "selectionDeferralTimer": 360
                }
            }
        },
        "messageStats": {
            "depthInq": 0,
            "depthOutq": 0,
            "opensSent": 1,
            "opensRecv": 1,
            "notificationsSent": 0,
            "notificationsRecv": 0,
            "updatesSent": 3206,
            "updatesRecv": 3202,
            "keepalivesSent": 1328,
            "keepalivesRecv": 1329,
            "routeRefreshSent": 0,
            "routeRefreshRecv": 0,
            "capabilitySent": 0,
            "capabilityRecv": 0,
            "totalSent": 4535,
            "totalRecv": 4532
        },
        "minBtwnAdvertisementRunsTimerMsecs": 0,
        "addressFamilyInfo": {
            "ipv6Unicast": {
                "peerGroupMember": "PEER_V6",
                "updateGroupId": 2,
                "subGroupId": 2,
                "packetQueueLength": 0,
                "inboundSoftConfigPermit": True,
                "commAttriSentToNbr": "all",
                "inboundPathPolicyConfig": True,
                "outboundPathPolicyConfig": True,
                "routeMapForIncomingAdvertisements": "FROM_BGP_PEER_V6",
                "routeMapForOutgoingAdvertisements": "TO_BGP_PEER_V6",
                "acceptedPrefixCounter": 6400
            }
        },
        "connectionsEstablished": 1,
        "connectionsDropped": 0,
        "lastResetTimerMsecs": 4006000,
        "lastResetDueTo": "Waiting for peer OPEN",
        "hostLocal": "fc00::71",
        "portLocal": 59726,
        "hostForeign": "fc00::72",
        "portForeign": 179,
        "nexthop": "10.0.0.56",
        "nexthopGlobal": "fc00::71",
        "nexthopLocal": "fe80::5054:ff:fea9:41c2",
        "bgpConnection": "sharedNetwork",
        "connectRetryTimer": 10,
        "estimatedRttInMsecs": 4,
        "readThread": "on",
        "writeThread": "on",
        "fdUsed": 30
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
BGP neighbor is fc00::72, remote AS 64600, local AS 65100, external link
  Local Role: undefined
  Remote Role: undefined
 Description: ARISTA01T1
 Member of peer-group PEER_V6 for session parameters
  BGP version 4, remote router ID 100.1.0.29, local router ID 10.1.0.32
  BGP state = Established, up for 01:06:23
  Last read 00:00:02, Last write 00:00:00
  Hold time is 10 seconds, keepalive interval is 3 seconds
  Configured hold time is 10 seconds, keepalive interval is 3 seconds
  Configured tcp-mss is 0, synced tcp-mss is 0
  Configured conditional advertisements interval is 60 seconds
  Connect retry timer: 10 seconds
  Local host: fc00::71, Local port: 59726
  Foreign host: fc00::72, Foreign port: 179
  Nexthop: 10.0.0.56
  Nexthop global: fc00::71
  Nexthop local: fe80::5054:ff:fea9:41c2
  Read thread: on, Write thread: on
  Estimated round trip time: 4 ms
  BGP connection: shared network
  Neighbor capabilities:
    4 Byte AS: advertised and received
    AddPath:
      IPv6 Unicast: RX advertised and received
    Route refresh: advertisedAndReceivedNew
    Address Family IPv6 Unicast: advertised and received
    Hostname Capability: advertised (name: vlab-01,domain name: n/a) not received
    Graceful Restart Capability: advertised and received
      Remote Restart timer is 300 seconds
      Address families by peer:
        none
  Graceful restart information:
    End-of-RIB send: IPv6 Unicast
    End-of-RIB received: IPv6 Unicast
    Local GR Mode: Restart*

    Remote GR Mode: Helper

    R bit: False
    N bit: False
    Timers:
      Configured Restart Time(sec): 240
      Received Restart Time(sec): 300
      Configured LLGR Stale Path Time(sec): 0
    IPv6 Unicast:
      F bit: False
      End-of-RIB sent: Yes
      End-of-RIB sent after update: No
      End-of-RIB received: Yes
      Timers:
        Configured Stale Path Time(sec): 360
        LLGR Stale Path Time(sec): 0
  Message statistics:
    Inq depth is 0
    Outq depth is 0
                         Sent       Rcvd
    Opens:            1          1
    Notifications:            0          0
    Updates:         3206       3202
    Keepalives:         1328       1329
    Route Refresh:            0          0
    Capability:            0          0
    Total:         4535       4532
  Minimum time between advertisement runs is 0 seconds

 For address family: IPv6 Unicast
  PEER_V6 peer-group member
  Update group 2, subgroup 2
  Packet Queue length 0
  Inbound soft reconfiguration allowed
  Community attribute sent to this neighbor(all)
  Inbound path policy configured
  Outbound path policy configured
  Route map for incoming advertisements is *FROM_BGP_PEER_V6
  Route map for outgoing advertisements is *TO_BGP_PEER_V6
  6400 accepted prefixes, 0 sent prefixes

  Connections established 1; dropped 0
  Last reset 01:06:46,   Waiting for peer OPEN (n/a)
"""


@pytest.mark.sonic_baseline
class TestIpv6BgpNeighbors:
    """
    Baseline test for 'show ipv6 bgp neighbors' (Cmd #39).

    Mirrors sonic-utilities test:
      File: show_bgp_neighbor_test.py
      Func: test_bgp_neighbors
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
