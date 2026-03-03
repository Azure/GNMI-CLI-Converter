
/// Source: `show_ip_route_common.py` variable `show_ipv6_route_expected_output`
#[allow(dead_code)]
pub const SHOW_IPV6_ROUTE_EXPECTED_OUTPUT: &str = r#"Codes: K - kernel route, C - connected, S - static, R - RIP,
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
C>*fe80::/64 is directly connected, eth0, 1d11h34m
"#;

// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Source: show_ip_route_common.py route format
pub const MOCK_IPV6_ROUTE: &str = r#"{
  "::/0":[
    {"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]},
    {"protocol":"kernel","selected":false,"installed":true,"distance":210,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fd00::1","interfaceName":"eth0","fib":true,"active":true}]}
  ],
  "2064:100::1/128":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true}]}],
  "2064:100::3/128":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:10::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:11::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:20::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:21::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:30::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:31::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:40::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:41::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:50::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:51::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:60::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:61::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:70::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:71::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:80::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:81::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:90::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:91::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:a0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:a1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:b0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:b1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:c0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:c1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:d0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:d1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:e0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:e1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:f0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a800:0:f1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:10::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:11::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:20::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:21::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:30::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:31::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:40::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:41::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:50::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:51::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:60::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:61::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:70::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:71::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:80::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:81::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:90::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:91::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:a0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:a1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:b0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:b1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:c0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:c1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:d0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:d1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:e0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:e1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:f0::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "20c0:a801:0:f1::/64":[{"protocol":"bgp","selected":true,"installed":true,"distance":20,"metric":0,"uptime":"1d11h34m","nexthops":[{"ip":"fc00::2","interfaceName":"PortChannel0002","fib":true,"active":true},{"ip":"fc00::6","interfaceName":"PortChannel0005","fib":true,"active":true}]}],
  "2603:10e2:400::/128":[{"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Loopback4096","directlyConnected":true,"fib":true,"active":true}]}],
  "fc00::/126":[{"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"PortChannel0002","directlyConnected":true,"fib":true,"active":true}]}],
  "fc00::4/126":[{"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"PortChannel0005","directlyConnected":true,"fib":true,"active":true}]}],
  "fc00:1::32/128":[{"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Loopback0","directlyConnected":true,"fib":true,"active":true}]}],
  "fd00::/80":[{"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"eth0","directlyConnected":true,"fib":true,"active":true}]}],
  "fe80::/64":[
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"PortChannel0002","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"PortChannel0005","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Ethernet20","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Ethernet16","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Ethernet4","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Ethernet0","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Loopback4096","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":false,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"Loopback0","directlyConnected":true,"fib":true,"active":true}]},
    {"protocol":"connected","selected":true,"installed":true,"uptime":"1d11h34m","nexthops":[{"interfaceName":"eth0","directlyConnected":true,"fib":true,"active":true}]}
  ]
}"#;

/// Mock JSON for `show ipv6 bgp summary`
/// Source: bgp_commands_input test vectors
pub const MOCK_IPV6_BGP_SUMMARY: &str = r#"{
    "router_id": "10.1.0.1",
    "local_as": 65100,
    "vrfId": 0,
    "tableVersion": 6,
    "ribCount": 12,
    "ribMemory": 1152,
    "peerCount": 2,
    "peerMemory": 42640,
    "peerGroupCount": 1,
    "neighbors": [
        {"neighbor": "fc00::2", "version": "4", "remote_as": "65200", "msg_rcvd": "500", "msg_sent": "480", "tbl_ver": "0", "in_q": "0", "out_q": "0", "uptime": "01:30:00", "state": "Established", "received_prefixes": "5", "desc": "ARISTA01T1"},
        {"neighbor": "fc00::6", "version": "4", "remote_as": "65200", "msg_rcvd": "450", "msg_sent": "430", "tbl_ver": "0", "in_q": "0", "out_q": "0", "uptime": "01:15:00", "state": "Established", "received_prefixes": "4", "desc": "ARISTA02T1"}
    ]
}"#;

/// Mock JSON for `show ipv6 bgp neighbors`
pub const MOCK_IPV6_BGP_NEIGHBORS: &str = r#"{"output": "BGP neighbor is fc00::2, remote AS 65200, local AS 65100, external link\n  BGP version 4, remote router ID 10.0.0.2\n  BGP state = Established, up for 01:30:00\n  Last read 00:00:05, Last write 00:00:05\n  Hold time is 180, keepalive interval is 60 seconds\n  Neighbor capabilities:\n    4 Byte AS: advertised and received\n    AddPath:\n      IPv6 Unicast: RX advertised and received\n    Route refresh: advertised and received(old & new)\n    Address Family IPv6 Unicast: advertised and received\n    Hostname Capability: advertised (name: sonic,domain name: n/a) received (name: ARISTA01T1,domain name: n/a)\n    Graceful Restart Capability: advertised and received\n  Graceful restart information:\n    End-of-RIB send: IPv6 Unicast\n    End-of-RIB received: IPv6 Unicast\n  Message statistics:\n    Inq depth is 0\n    Outq depth is 0\n                         Sent       Rcvd\n    Opens:                  1          1\n    Notifications:          0          0\n    Updates:               10          5\n    Keepalives:           480        500\n    Route Refresh:          0          0\n    Capability:             0          0\n    Total:                491        506\n"}"#;

/// Mock JSON for `show ipv6 bgp network`
pub const MOCK_IPV6_BGP_NETWORK: &str = r#"{"output": "BGP table version is 6, local router ID is 10.1.0.1, vrf id 0\nDefault local pref 100, local AS 65100\nStatus codes:  s suppressed, d damped, h history, * valid, > best, = multipath,\n               i internal, r RIB-failure, S Stale, R Removed\nNexthop codes: @NHU = Unreachable Nexthop, @NHR = Recursive Nexthop\nOrigin codes:  i - IGP, e - EGP, ? - incomplete\n   Network          Next Hop            Metric LocPrf Weight Path\n*> 2001:db8:1::/64  fc00::2                  0            100 0 65200 i\n*> 2001:db8:2::/64  fc00::6                  0            100 0 65200 i\n\nDisplayed  2 routes and 2 total paths\n"}"#;

/// Mock JSON for `show ipv6 interfaces`
pub const MOCK_IPV6_INTERFACES: &str = r#"{
    "Ethernet0": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {"address": "fc00::1/126", "bgp_neighbor_name": "ARISTA01T1", "bgp_neighbor_ip": "fc00::2"},
            {"address": "fe80::1/64", "bgp_neighbor_name": "", "bgp_neighbor_ip": ""}
        ],
        "type": "eth"
    },
    "Ethernet4": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {"address": "fc00::5/126", "bgp_neighbor_name": "ARISTA02T1", "bgp_neighbor_ip": "fc00::6"},
            {"address": "fe80::5/64", "bgp_neighbor_name": "", "bgp_neighbor_ip": ""}
        ],
        "type": "eth"
    },
    "Loopback0": {
        "master": "",
        "admin_status": "up",
        "oper_status": "up",
        "ipv6_addresses": [
            {"address": "2001:db8::1/128", "bgp_neighbor_name": "", "bgp_neighbor_ip": ""}
        ],
        "type": "lo"
    }
}"#;

/// Mock JSON for `show ipv6 link-local-mode`
pub const MOCK_IPV6_LINK_LOCAL_MODE: &str = r#"[
    {"port": "Ethernet0", "mode": "enabled"},
    {"port": "Ethernet4", "mode": "enabled"},
    {"port": "Ethernet8", "mode": "disabled"},
    {"port": "Vlan1000", "mode": "enabled"}
]"#;

/// Mock JSON for `show ipv6 prefix-list`
pub const MOCK_IPV6_PREFIX_LIST: &str = r#"{
    "ZEBRA": {
        "PL_ALLOW_ALL_V6": {
            "entries": [
                {"sequenceNumber": 5, "type": "permit", "prefix": "::/0", "minimumPrefixLength": 0, "maximumPrefixLength": 128}
            ]
        }
    },
    "BGP": {
        "FROM_ARISTA_V6": {
            "entries": [
                {"sequenceNumber": 10, "type": "permit", "prefix": "2001:db8::/32", "minimumPrefixLength": 32, "maximumPrefixLength": 64},
                {"sequenceNumber": 20, "type": "deny", "prefix": "::/0", "minimumPrefixLength": 0, "maximumPrefixLength": 128}
            ]
        }
    }
}"#;

/// Mock JSON for `show ipv6 protocol`
pub const MOCK_IPV6_PROTOCOL: &str = r#"[
    {
        "VRF": "default",
        "Protocols": [
            {"Protocol": "bgp", "route-map": "FROM_BGP_V6"},
            {"Protocol": "connected", "route-map": ""},
            {"Protocol": "static", "route-map": ""}
        ]
    }
]"#;

/// Mock JSON for `show ipv6 fib`
pub const MOCK_IPV6_FIB: &str = r#"{
    "entries": [
        {"index": 1, "vrf": "default", "route": "::/0", "nexthop": "fc00::2", "ifname": "Ethernet0"},
        {"index": 2, "vrf": "default", "route": "fc00::/126", "nexthop": "", "ifname": "Ethernet0"},
        {"index": 3, "vrf": "default", "route": "2001:db8:1::/64", "nexthop": "fc00::2", "ifname": "Ethernet0"},
        {"index": 4, "vrf": "default", "route": "2001:db8:2::/64", "nexthop": "fc00::6", "ifname": "Ethernet4"}
    ],
    "total": 4
}"#;

/// Mock JSON for empty IPv6 route (edge case)
pub const MOCK_IPV6_ROUTE_EMPTY: &str = r#"{}"#;

/// Mock JSON for empty IPv6 BGP summary (edge case)
pub const MOCK_IPV6_BGP_SUMMARY_EMPTY: &str = r#"{
    "router_id": "10.1.0.1",
    "local_as": 65100,
    "neighbors": []
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `show_ip_route_common.py` — `show ipv6 route` (empty)
pub const SHOW_IPV6_ROUTE_EMPTY_OUTPUT: &str = r#"Codes: K - kernel route, C - connected, S - static, R - RIP,
       O - OSPF, I - IS-IS, B - BGP, E - EIGRP, N - NHRP,
       T - Table, v - VNC, V - VNC-Direct, A - Babel, D - SHARP,
       F - PBR, f - OpenFabric,
       > - selected route, * - FIB route, q - queued route, r - rejected route"#;

/// Source: `bgp_commands_input` test vectors — `show ipv6 bgp summary`
pub const SHOW_IPV6_BGP_SUMMARY_OUTPUT: &str = r#"
IPv6 Unicast Summary:
BGP router identifier 10.1.0.1, local AS number 65100 vrf-id 0
BGP table version 6
RIB entries 12, using 1152 bytes of memory
Peers 2, using 42640 KiB of memory
Peer groups 1, using 0 bytes of memory


Neighbhor      V     AS    MsgRcvd    MsgSent    TblVer    InQ    OutQ  Up/Down      State/PfxRcd  NeighborName
-----------  ---  -----  ---------  ---------  --------  -----  ------  ---------  --------------  --------------
fc00::2        4  65200        500        480                0       0  01:30:00      Established  
fc00::6        4  65200        450        430                0       0  01:15:00      Established"#;

/// Source: `bgp_commands_input` test vectors — `show ipv6 bgp summary` (empty)
pub const SHOW_IPV6_BGP_SUMMARY_EMPTY_OUTPUT: &str = r#"
IPv6 Unicast Summary:
BGP router identifier 10.1.0.1, local AS number 65100 vrf-id 0


Neighbhor      V     AS    MsgRcvd    MsgSent    TblVer    InQ    OutQ  Up/Down      State/PfxRcd  NeighborName
-----------  ---  -----  ---------  ---------  --------  -----  ------  ---------  --------------  --------------"#;

/// Source: `bgp_commands_input` test vectors — `show ipv6 bgp neighbors`
pub const SHOW_IPV6_BGP_NEIGHBORS_OUTPUT: &str = "";

/// Source: `bgp_commands_input` test vectors — `show ipv6 bgp network`
pub const SHOW_IPV6_BGP_NETWORK_OUTPUT: &str = r#"BGP table version is 6, local router ID is 10.1.0.1, vrf id 0
Default local pref 100, local AS 65100
Status codes:  s suppressed, d damped, h history, * valid, > best, = multipath,
               i internal, r RIB-failure, S Stale, R Removed
Nexthop codes: @NHU = Unreachable Nexthop, @NHR = Recursive Nexthop
Origin codes:  i - IGP, e - EGP, ? - incomplete
   Network          Next Hop            Metric LocPrf Weight Path
*> 2001:db8:1::/64  fc00::2                  0            100 0 65200 i
*> 2001:db8:2::/64  fc00::6                  0            100 0 65200 i

Displayed  2 routes and 2 total paths"#;

/// Source: `show_ip_route_common.py` — `show ipv6 interfaces`
pub const SHOW_IPV6_INTERFACES_OUTPUT: &str = r#"  Interface    Master           IPv6 address/mask    Admin/Oper    BGP Neighbor    Neighbor IP
-----------  --------  --------------------------  ------------  --------------  -------------
Ethernet0              fc00::1/126 (eth)           up/up         ARISTA01T1      fc00::2
                       fe80::1%Ethernet0/64 (eth)
Ethernet4              fc00::5/126 (eth)           up/up         ARISTA02T1      fc00::6
                       fe80::5%Ethernet4/64 (eth)
Loopback0              2001:db8::1/128 (lo)        up/up"#;

/// Source: `show_ip_route_common.py` — `show ipv6 link-local-mode`
pub const SHOW_IPV6_LINK_LOCAL_MODE_OUTPUT: &str = r#"+------------------+----------+
| Interface Name   | Mode     |
+==================+==========+
| Ethernet0        | enabled  |
+------------------+----------+
| Ethernet4        | enabled  |
+------------------+----------+
| Ethernet8        | disabled |
+------------------+----------+
| Vlan1000         | enabled  |
+------------------+----------+"#;

/// Source: `show_ip_route_common.py` — `show ipv6 prefix-list`
pub const SHOW_IPV6_PREFIX_LIST_OUTPUT: &str = r#"ZEBRA: ipv6 prefix-list PL_ALLOW_ALL_V6: 1 entries
   seq 5 permit ::/0 ge 0 le 128
BGP: ipv6 prefix-list FROM_ARISTA_V6: 2 entries
   seq 10 permit 2001:db8::/32 ge 32 le 64
   seq 20 deny ::/0 ge 0 le 128"#;

/// Source: `show_ip_route_common.py` — `show ipv6 protocol`
pub const SHOW_IPV6_PROTOCOL_OUTPUT: &str = r#"VRF: default
Protocol                  : route-map
-------------------------------------
bgp                       : FROM_BGP_V6
connected                 : 
static                    :"#;

/// Source: `show_ip_route_common.py` — `show ipv6 fib`
pub const SHOW_IPV6_FIB_OUTPUT: &str = r#"  No.      Vrf            Route    Nexthop     Ifname
-----  -------  ---------------  ---------  ---------
    1  default  ::/0             fc00::2    Ethernet0
    2  default  fc00::/126                  Ethernet0
    3  default  2001:db8:1::/64  fc00::2    Ethernet0
    4  default  2001:db8:2::/64  fc00::6    Ethernet4
Total number of entries 4"#;
