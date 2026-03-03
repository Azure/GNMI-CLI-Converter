"""
Baseline UT: show ipv6 route (Cmd #37)

Source:
  - sonic-utilities UT File : ip_show_routes_test.py
  - sonic-utilities Test Func: test_show_ipv6_route
  - Expected CLI Output Var : show_ipv6_route_expected_output (from show_ip_route_common.py)
  - Mapping Doc Section     : ### #37

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show ipv6 route"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
  "output": (
    "Codes: K - kernel route, C - connected, S - static, R - RIP,\n"
    "       O - OSPF, I - IS-IS, B - BGP, E - EIGRP, N - NHRP,\n"
    "       T - Table, v - VNC, V - VNC-Direct, A - Babel, D - SHARP,\n"
    "       F - PBR, f - OpenFabric,\n"
    "       > - selected route, * - FIB route, q - queued route, r - rejected route\n"
    "\n"
    "B>*::/0 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *            via fc00::6, PortChannel0005, 1d11h34m\n"
    "K *::/0 [210/0] via fd00::1, eth0, 1d11h34m\n"
    "B>*2064:100::1/128 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "B>*2064:100::3/128 [20/0] via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                      via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                          via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:10::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:11::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:20::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:21::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:30::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:31::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:40::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:41::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:50::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:51::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:60::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:61::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:70::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:71::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:80::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:81::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:90::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:91::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:a0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:a1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:b0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:b1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:c0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:c1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:d0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:d1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:e0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:e1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:f0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a800:0:f1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                      via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                          via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:10::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:11::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:20::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:21::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:30::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:31::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:40::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:41::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:50::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:51::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:60::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:61::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:70::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:71::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:80::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:81::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:90::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:91::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:a0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:a1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:b0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:b1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:c0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:c1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:d0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:d1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:e0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:e1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:f0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "B>*20c0:a801:0:f1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m\n"
    "  *                           via fc00::6, PortChannel0005, 1d11h34m\n"
    "C>*2603:10e2:400::/128 is directly connected, Loopback4096, 1d11h34m\n"
    "C>*fc00::/126 is directly connected, PortChannel0002, 1d11h34m\n"
    "C>*fc00::4/126 is directly connected, PortChannel0005, 1d11h34m\n"
    "C>*fc00:1::32/128 is directly connected, Loopback0, 1d11h34m\n"
    "C>*fd00::/80 is directly connected, eth0, 1d11h34m\n"
    "C *fe80::/64 is directly connected, PortChannel0002, 1d11h34m\n"
    "C *fe80::/64 is directly connected, PortChannel0005, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Ethernet20, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Ethernet16, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Ethernet4, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Ethernet0, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Loopback4096, 1d11h34m\n"
    "C *fe80::/64 is directly connected, Loopback0, 1d11h34m\n"
    "C>*fe80::/64 is directly connected, eth0, 1d11h34m"
  )
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Codes: K - kernel route, C - connected, S - static, R - RIP,
       O - OSPF, I - IS-IS, B - BGP, E - EIGRP, N - NHRP,
       T - Table, v - VNC, V - VNC-Direct, A - Babel, D - SHARP,
       F - PBR, f - OpenFabric,
       > - selected route, * - FIB route, q - queued route, r - rejected route

B>*::/0 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *            via fc00::6, PortChannel0005, 1d11h34m
K *::/0 [210/0] via fd00::1, eth0, 1d11h34m
B>*2064:100::1/128 [20/0] via fc00::2, PortChannel0002, 1d11h34m
B>*2064:100::3/128 [20/0] via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                      via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                          via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:10::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:11::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:20::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:21::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:30::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:31::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:40::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:41::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:50::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:51::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:60::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:61::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:70::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:71::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:80::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:81::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:90::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:91::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:a0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:a1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:b0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:b1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:c0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:c1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:d0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:d1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:e0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:e1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:f0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a800:0:f1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                      via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                          via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:10::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:11::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:20::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:21::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:30::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:31::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:40::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:41::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:50::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:51::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:60::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:61::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:70::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:71::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:80::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:81::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:90::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:91::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:a0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:a1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:b0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:b1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:c0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:c1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:d0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:d1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:e0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:e1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:f0::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
B>*20c0:a801:0:f1::/64 [20/0] via fc00::2, PortChannel0002, 1d11h34m
  *                           via fc00::6, PortChannel0005, 1d11h34m
C>*2603:10e2:400::/128 is directly connected, Loopback4096, 1d11h34m
C>*fc00::/126 is directly connected, PortChannel0002, 1d11h34m
C>*fc00::4/126 is directly connected, PortChannel0005, 1d11h34m
C>*fc00:1::32/128 is directly connected, Loopback0, 1d11h34m
C>*fd00::/80 is directly connected, eth0, 1d11h34m
C *fe80::/64 is directly connected, PortChannel0002, 1d11h34m
C *fe80::/64 is directly connected, PortChannel0005, 1d11h34m
C *fe80::/64 is directly connected, Ethernet20, 1d11h34m
C *fe80::/64 is directly connected, Ethernet16, 1d11h34m
C *fe80::/64 is directly connected, Ethernet4, 1d11h34m
C *fe80::/64 is directly connected, Ethernet0, 1d11h34m
C *fe80::/64 is directly connected, Loopback4096, 1d11h34m
C *fe80::/64 is directly connected, Loopback0, 1d11h34m
C>*fe80::/64 is directly connected, eth0, 1d11h34m"""


@pytest.mark.sonic_baseline
class TestIpv6Route:
    """
    Baseline test for 'show ipv6 route' (Cmd #37).

    Mirrors sonic-utilities test:
      File: ip_show_routes_test.py
      Func: test_show_ipv6_route
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
