"""
Test data for self-contained testing.

This module contains embedded test data for validating the library
without requiring external data sources or live SONiC devices.

The data is organized by command type and includes:
- GNMI_JSON_TEST_DATA: Sample gNMI JSON responses
- EXPECTED_CLI_OUTPUTS: Expected CLI-formatted output
"""

# =============================================================================
# Expected CLI Outputs (for validation)
# =============================================================================

EXPECTED_CLI_OUTPUTS = {
    "buffer_pool": {
        "watermark": """\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool    12345
    egress_lossy_pool    67890
ingress_lossless_pool    24680
""",
        "persistent-watermark": """\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool     9216
    egress_lossy_pool     6656
ingress_lossless_pool    40192
""",
    },
    "headroom-pool": {
        "watermark": """\
Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool   432640
""",
        "persistent-watermark": """\
Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool   863616
""",
    },
    "priority-group": {
        "watermark-shared": """\
Ingress shared pool occupancy per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    100    101    102    103    104    105    106    107
Ethernet4    400    401    402    403    404    405    406    407
Ethernet8    800    801    802    803    804    805    806    807
""",
        "watermark-headroom": """\
Ingress headroom per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    100    101    102    103    104    105    106    107
""",
        "persistent-watermark-shared": "",
        "persistent-watermark-headroom": "",
    },
    "queue": {
        "watermark-unicast": """\
Egress shared pool occupancy per unicast queue:
     Port    UC0    UC1
---------  -----  -----
Ethernet0    128      0
""",
        "watermark-multicast": """\
Egress shared pool occupancy per multicast queue:
     Port    MC10    MC11
---------  ------  ------
Ethernet0     N/A     N/A
""",
        "watermark-all": """\
Egress shared pool occupancy per all queues:
     Port    UC0    UC1    MC2
---------  -----  -----  -----
Ethernet0    128      0    256
""",
        "persistent-watermark-unicast": "",
        "persistent-watermark-multicast": "",
        "persistent-watermark-all": "",
        "counters": """\
     Port    Cnt/pkts    Cnt/bytes    Drop/pkts    Drop/bytes
---------  ----------  -----------  -----------  ------------
Ethernet0         100        25600            2           512
Ethernet0         200        51200            0             0
""",
        "wredcounters": """\
     Port    WredDrp/pkts    WredDrp/bytes    EcnMrk/pkts    EcnMrk/bytes
---------  --------------  ---------------  -------------  --------------
Ethernet0               2              512              0               0
""",
    },
    "clock": {
        "date": "Mon Mar 25 08:25:16 PM UTC 2019\n",
        "timezones": """\
America/Anchorage
America/Anguilla
America/Argentina/Buenos_Aires
America/Aruba
Asia/Aden
Asia/Almaty
Atlantic/Bermuda
CET
CST6CDT
Etc/GMT
Etc/GMT+0
Etc/GMT+10
Etc/GMT+2
Etc/GMT+3
Etc/GMT+4
Etc/GMT-1
Etc/GMT-10
Etc/GMT-2
Etc/GMT-3
Etc/GMT-4
UTC
Universal
""",
    },
    "dropcounters": {
        "capabilities": """\
Counter Type           Total
-------------------  -------
PORT_INGRESS_DROPS        10
SWITCH_EGRESS_DROPS        2

PORT_INGRESS_DROPS
        MPLS_MISS
        FDB_AND_BLACKHOLE_DISCARDS
        IP_HEADER_ERROR
        L3_EGRESS_LINK_DOWN

SWITCH_EGRESS_DROPS
        ACL_ANY
        L2_ANY
""",
        "configuration": """\
Counter    Alias          Group          Type                 Reasons    Description
---------  -------------  -------------  -------------------  ---------  -------------------------------------------
DEBUG_0    DEBUG_0        N/A            PORT_INGRESS_DROPS   None       N/A
DEBUG_1    SWITCH_DROPS   PACKET_DROPS   SWITCH_EGRESS_DROPS  None       Outgoing packet drops, tracked at the s...
""",
        "counts": """\
    IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS    DEBUG_0    DEBUG_2
---------  -------  --------  ----------  --------  ----------  ---------  ---------
Ethernet0        D        10         100         0           0         80         20
Ethernet4      N/A         0        1000         0           0        800        100
Ethernet8      N/A       100          10         0           0         10          0
""",
    },
    "uptime": {
        "uptime": "up 3 weeks, 4 days, 10 hours, 15 minutes\n",
    },
    "version": {
        "version": """\
SONiC Software Version: SONiC.test_branch.1-a8fbac59d
SONiC OS Version: 11
Distribution: Debian 11.4
Kernel: 5.10.0-18-2-amd64
Build commit: a8fbac59d
Build date: 2025-07-18
Built by: build_user
Platform: x86_64-mlnx_msn2700-r0
HwSKU: Mellanox-SN2700
ASIC: mellanox
ASIC Count: 1
Serial Number: MT1234567890
Model Number: MSN2700-CS2FO
Hardware Revision: A1
Uptime: 07:42:51 up 16 days, 14:51
Date: Fri 18 Jul 2025 18:00:00
""",
    },
    "system-memory": {
        "memory": """\
              total        used        free      shared  buff/cache   available
Mem:          64252       15017        7526         563       41708       46228
Swap:             0           0           0
""",
    },
    "lldp": {
        "table": """\
Capability codes: (R) Router, (B) Bridge, (O) Other
LocalPort    RemoteDevice    RemotePortId    Capability    RemotePortDescr
-----------  --------------  --------------  ------------  -----------------
Ethernet0    switch1         Ethernet48      BR            Uplink to spine
Ethernet4    switch2         Ethernet52      R             Connection to leaf

Total entries displayed: 2
""",
        "neighbors": """\
Capability codes: (R) Router, (B) Bridge, (O) Other
LocalPort    RemoteDevice    RemotePortId    Capability    RemotePortDescr
-----------  --------------  --------------  ------------  -----------------
Ethernet0    switch1         Ethernet48      BR            Uplink to spine

Total entries displayed: 1
""",
    },
    "vlan": {
        "brief": """\
+---------+-----------+-----------+----------------+-----------------------+
|   VLAN  | Name      | Status    | Ports          | DHCP Helper Address   |
+=========+===========+===========+================+=======================+
|    1    | Vlan1     | Active    | Ethernet120(U) | 192.0.0.1             |
+---------+-----------+-----------+----------------+-----------------------+
""",
    },
    "reboot-cause": {
        "reboot-cause": """\
Cause: reboot
Time: Thu Jul 10 08:05:33 PM UTC 2025
User: admin
""",
        "history": """\
Name                            Cause        Time                             User    Comment
------------------------------  -----------  -------------------------------  ------  ---------
REBOOT_CAUSE|2025_07_10_20_06_34  reboot       Thu Jul 10 08:05:33 PM UTC 2025  admin   N/A
REBOOT_CAUSE|2025_07_10_20_12_49  fast-reboot  Thu Jul 10 08:10:49 PM UTC 2025  admin   N/A
""",
    },
    "interfaces": {
        "description": """\
Interface    Admin    Alias    Description           Oper
-----------  -------  -------  --------------------  ------
Ethernet0    up       etp0     ARISTA01T1:Ethernet1  up
Ethernet4    up       etp1     Servers1:eth0         down
""",
        "status": """\
Interface     Admin    Oper    Speed    MTU     Alias    Description
------------  -------  ------  -------  ------  -------  --------------------
Ethernet0     up       up      100000   9100    etp0     ARISTA01T1:Ethernet1
Ethernet40    up       up      100000   9100    etp10    Servers4:eth0
""",
        "counters": """\
Interface    RX_OK    RX_BPS       RX_UTIL    RX_ERR    RX_DRP    TX_OK    TX_BPS       TX_UTIL    TX_ERR    TX_DRP
-----------  -------  -----------  ---------  --------  --------  -------  -----------  ---------  --------  --------
Ethernet0    149903   50.00 Kb/s   0.00%      0         957       144782   50.00 Kb/s   0.00%      0         2
""",
        "errors": """\
Port Errors             Count    Last timestamp(UTC)
----------------------  -------  ---------------------
oper error status       3        2024-01-15T10:30:45Z
mac local fault         2        2024-01-15T10:25:30Z
""",
        "alias": """\
Interface    Alias
-----------  -------
Ethernet0    etp1
Ethernet8    etp2
""",
        "portchannel": """\
PortChannel       Admin    MTU     TPID
----------------  -------  ------  --------
PortChannel101    up       9100    0x8100
PortChannel102    up       9100    0x8100
""",
        "flap": """\
Interface    Flap Count    Last Up Time                Last Down Time
-----------  ------------  --------------------------  --------------------------
Ethernet0    3             Fri Jul 18 19:59:28 2025    Fri Jul 18 19:59:20 2025
""",
        "naming-mode": "default\n",
        # Detailed counter variants
        "counters-detailed": """\
Interface    RX_OK    RX_BPS       RX_UTIL    RX_ERR    RX_DRP    TX_OK    TX_BPS       TX_UTIL    TX_ERR    TX_DRP
-----------  -------  -----------  ---------  --------  --------  -------  -----------  ---------  --------  --------
Ethernet0    149903   50.00 Kb/s   0.00%      0         957       144782   50.00 Kb/s   0.00%      0         2
""",
        "counters-errors": """\
Interface    RX_ERR    TX_ERR
-----------  --------  --------
Ethernet0    0         0
""",
        "counters-rates": """\
Interface    RX_BPS       TX_BPS
-----------  -----------  -----------
Ethernet0    50.00 Kb/s   50.00 Kb/s
""",
        "counters-rif": """\
Interface    RX_OK    TX_OK
-----------  -------  -------
Ethernet0    149903   144782
""",
        "counters-fec-stats": """\
Interface    FEC Corrected    FEC Uncorrectable
-----------  ---------------  -------------------
Ethernet0    100              0
""",
        "counters-fec-histogram": """\
Interface    Bin0    Bin1    Bin2
-----------  ------  ------  ------
Ethernet0    1000    500     100
""",
        "counters-trim": """\
Interface    Trimmed Packets
-----------  -----------------
Ethernet0    50
""",
        # Neighbor
        "neighbor": """\
Interface    Neighbor
-----------  ----------
Ethernet0    switch1
Ethernet4    switch2
""",
        # Neighbor expected (same as neighbor but uses different subcommand)
        "neighbor-expected": """\
LocalPort    Neighbor      NeighborPort    NeighborLoopback    NeighborMgmt     NeighborType
-----------  ------------  --------------  ------------------  ---------------  -----------------
Ethernet0    ARISTA01FT2   Ethernet1       None                172.16.41.234    FabricSpineRouter
Ethernet4    ARISTA02FT2   Ethernet2       None                172.16.41.235    FabricSpineRouter
""",
        # FEC status
        "fec-status": """\
Interface    FEC    Status
-----------  -----  ---------
Ethernet0    rs     enabled
Ethernet4    none   disabled
""",
        # Transceiver subcommands
        "transceiver-eeprom": """\
Interface    Vendor     Serial
-----------  ---------  ---------
Ethernet0    Mellanox   MT12345
""",
        "transceiver-error-status": """\
Interface    Error
-----------  -------
Ethernet0    none
""",
        "transceiver-info": """\
Interface    Type      Vendor
-----------  --------  ---------
Ethernet0    QSFP28    Mellanox
""",
        "transceiver-lpmode": """\
Interface    LP Mode
-----------  ---------
Ethernet0    off
""",
        "transceiver-pm": """\
Interface    RX Power    TX Power
-----------  ----------  ----------
Ethernet0    -5.5 dBm    -3.2 dBm
""",
        "transceiver-presence": """\
Interface    Presence
-----------  ----------
Ethernet0    present
""",
        "transceiver-status": """\
Interface    Status
-----------  --------
Ethernet0    ok
""",
        # Switchport subcommands
        "switchport-config": """\
Interface    Mode      VLAN
-----------  --------  ------
Ethernet0    access    1000
""",
        "switchport-status": """\
Interface    Admin    Oper    Mode
-----------  -------  ------  --------
Ethernet0    up       up      access
""",
    },
    "ndp": {
        "ndp": """\
Address                    Interface     MAC Address          State
-------------------------  ------------  -------------------  ----------
fc00::176                  Ethernet60    aa:f9:f4:cb:d2:3f    REACHABLE
fe80::bace:f6ff:fee5:500d  Vlan1000      b8:ce:f6:e5:50:0d    REACHABLE
""",
    },
    "srv6": {
        "stats": """\
SID                Packets    Bytes
-----------------  ---------  -------
2001:db8:1::/48    12345      67890
2001:db8:2::/48    23456      78901
""",
    },
    "mac": {
        "mac": """\
No.    Vlan    Mac Address          Port       Type
-----  ------  -------------------  ---------  --------
1      1000    aa:bb:cc:dd:ee:ff    Ethernet0  Dynamic
2      1000    11:22:33:44:55:66    Ethernet4  Static

Total number of entries 2
""",
        "aging-time": "600\n",
    },
    "mmu": {
        "mmu": """\
Pool: ingress_lossless_pool
  Mode: dynamic
  Size: 12345

Profile: pg_lossless_100000_40m_profile
  Pool: ingress_lossless_pool
""",
    },
    "services": {
        "services": """\
Service: snmp
PID      User    %CPU    %MEM    Command
-------  ------  ------  ------  -----------------
123      root    0.1     0.5     python snmpd.py
""",
    },
    "processes": {
        "processes": """\
Usage: show processes [OPTIONS] COMMAND [ARGS]...

  Show processes information

Options:
  -?, -h, --help  Show this message and exit.

Commands:
  cpu      Show processes information cpu
  memory   Show processes information memory
  summary  Show processes information summary
""",
        "summary": """\
PID    PPID    %CPU    %MEM    CMD
-----  ------  ------  ------  --------
1      0       0.1     0.2     systemd
123    1       1.5     2.3     python
""",
        "cpu": """\
PID    PPID    %CPU    %MEM    CMD
-----  ------  ------  ------  ----------
999    1       50.0    5.0     syncd
888    1       25.0    3.0     orchagent
""",
        "memory": """\
PID    PPID    %CPU    %MEM    CMD
-----  ------  ------  ------  ----------
777    1       5.0     25.0    database
666    1       3.0     15.0    teamd
""",
    },
    "watermark": {
        "telemetry": "Telemetry interval: 120 seconds\n",
    },
    "ipv6": {
        "prefix-list": "",
        "interfaces": "",
        "route": "",
        "protocol": "",
        "bgp-neighbors": "",
        "bgp-network": "",
        "bgp-summary": "",
        "link-local-mode": "disabled\n",
        "fib": "",
    },
}

# =============================================================================
# gNMI JSON Test Data
# =============================================================================

GNMI_JSON_TEST_DATA = {
    "buffer_pool": {
        "watermark": {
            "egress_lossless_pool": {"Bytes": "12345"},
            "egress_lossy_pool": {"Bytes": "67890"},
            "ingress_lossless_pool": {"Bytes": "24680"}
        },
        "persistent-watermark": {
            "egress_lossless_pool": {"Bytes": "9216"},
            "egress_lossy_pool": {"Bytes": "6656"},
            "ingress_lossless_pool": {"Bytes": "40192"}
        },
        "raw_user_watermarks": {
            "USER_WATERMARKS:oid:0x7000000000001": {
                "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "12345"
            },
            "USER_WATERMARKS:oid:0x7000000000002": {
                "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "67890"
            },
            "USER_WATERMARKS:oid:0x7000000000003": {
                "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "24680"
            }
        },
        "raw_persistent_watermarks": {
            "PERSISTENT_WATERMARKS:oid:0x7000000000001": {
                "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "9216"
            },
        },
        "name_map": {
            "COUNTERS_BUFFER_POOL_NAME_MAP": {
                "egress_lossless_pool": "oid:0x7000000000001",
                "egress_lossy_pool": "oid:0x7000000000002",
                "ingress_lossless_pool": "oid:0x7000000000003"
            }
        }
    },
    "headroom-pool": {
        "watermark": {
            "ingress_lossless_pool": {"Bytes": "432640"}
        },
        "persistent-watermark": {
            "ingress_lossless_pool": {"Bytes": "863616"}
        },
    },
    "queue": {
        "watermark-unicast": {
            "Ethernet0": {"UC0": "128", "UC1": "0"},
            "Ethernet40": {"UC0": "1024", "UC1": "2048"},
        },
        "watermark-multicast": {
            "Ethernet0": {"MC10": "N/A", "MC11": "N/A"},
        },
        "watermark-all": {
            "Ethernet0": {"UC0": "128", "UC1": "0", "MC2": "256"},
        },
        "persistent-watermark-unicast": {},
        "persistent-watermark-multicast": {},
        "persistent-watermark-all": {},
        "counters": {
            "Ethernet0:0": {
                "Counter/pkts": "100", "Counter/bytes": "25600",
                "Drop/pkts": "2", "Drop/bytes": "512"
            },
            "Ethernet0:1": {
                "Counter/pkts": "200", "Counter/bytes": "51200",
                "Drop/pkts": "0", "Drop/bytes": "0"
            }
        },
        "wredcounters": {
            "Ethernet0:0": {
                "WredDrp/pkts": "2", "WredDrp/bytes": "512",
                "EcnMarked/pkts": "0", "EcnMarked/bytes": "0"
            },
            "Ethernet40:1": {
                "WredDrp/pkts": "5", "WredDrp/bytes": "500",
                "EcnMarked/pkts": "10", "EcnMarked/bytes": "1000"
            }
        }
    },
    "clock": {
        "date": {"date": "Mon Mar 25 20:25:16 UTC 2019"},
        "date_zero_epoch": {"date": "Thu Jan  1 00:00:00 UTC 1970"},
        "date_normal": {"date": "Fri Jul 18 18:00:00 UTC 2025"},
        "timezones": {
            "timezones": [
                "America/Anchorage", "America/Anguilla", "America/Argentina/Buenos_Aires",
                "America/Aruba", "Asia/Aden", "Asia/Almaty", "Atlantic/Bermuda",
                "CET", "CST6CDT", "Etc/GMT", "Etc/GMT+0", "Etc/GMT+10",
                "Etc/GMT+2", "Etc/GMT+3", "Etc/GMT+4", "Etc/GMT-1", "Etc/GMT-10",
                "Etc/GMT-2", "Etc/GMT-3", "Etc/GMT-4", "UTC", "Universal"
            ]
        },
    },
    "priority-group": {
        "watermark-shared": {
            "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103",
                          "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"},
            "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402", "PG3": "403",
                          "PG4": "404", "PG5": "405", "PG6": "406", "PG7": "407"},
            "Ethernet8": {"PG0": "800", "PG1": "801", "PG2": "802", "PG3": "803",
                          "PG4": "804", "PG5": "805", "PG6": "806", "PG7": "807"}
        },
        "watermark-headroom": {
            "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103",
                          "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"}
        },
        "persistent-watermark-shared": {},
        "persistent-watermark-headroom": {},
    },
    "dropcounters": {
        "capabilities": {
            "PORT_INGRESS_DROPS": {
                "count": "10",
                "reasons": "[MPLS_MISS,FDB_AND_BLACKHOLE_DISCARDS,IP_HEADER_ERROR,L3_EGRESS_LINK_DOWN]"
            },
            "SWITCH_EGRESS_DROPS": {
                "count": "2",
                "reasons": "[ACL_ANY,L2_ANY]"
            }
        },
        "configuration": [
            {
                "name": "DEBUG_0",
                "alias": "DEBUG_0",
                "group": "N/A",
                "type": "PORT_INGRESS_DROPS",
                "reason": "None",
                "description": "N/A"
            },
            {
                "name": "DEBUG_1",
                "alias": "SWITCH_DROPS",
                "group": "PACKET_DROPS",
                "type": "SWITCH_EGRESS_DROPS",
                "reason": "None",
                "description": "Outgoing packet drops, tracked at the switch level"
            }
        ],
        "counts": {
            "Ethernet0": {
                "State": "D",
                "RX_ERR": "10",
                "RX_DROPS": "100",
                "TX_ERR": "0",
                "TX_DROPS": "0",
                "DEBUG_0": "80",
                "DEBUG_2": "20"
            },
            "Ethernet4": {
                "State": "N/A",
                "RX_ERR": "0",
                "RX_DROPS": "1000",
                "TX_ERR": "0",
                "TX_DROPS": "0",
                "DEBUG_0": "800",
                "DEBUG_2": "100"
            },
            "Ethernet8": {
                "State": "N/A",
                "RX_ERR": "100",
                "RX_DROPS": "10",
                "TX_ERR": "0",
                "TX_DROPS": "0",
                "DEBUG_0": "10",
                "DEBUG_2": "0"
            }
        }
    },
    "uptime": {
        "uptime": {"uptime": "up 3 weeks, 4 days, 10 hours, 15 minutes"}
    },
    "version": {
        "version": {
            "sonic_software_version": "SONiC.test_branch.1-a8fbac59d",
            "sonic_os_version": "11",
            "distribution": "Debian 11.4",
            "kernel": "5.10.0-18-2-amd64",
            "build_commit": "a8fbac59d",
            "build_date": "2025-07-18",
            "built_by": "build_user",
            "platform": "x86_64-mlnx_msn2700-r0",
            "hwsku": "Mellanox-SN2700",
            "asic": "mellanox",
            "asic_count": "1",
            "serial_number": "MT1234567890",
            "model_number": "MSN2700-CS2FO",
            "hardware_revision": "A1",
            "uptime": "07:42:51 up 16 days, 14:51",
            "date": "Fri 18 Jul 2025 18:00:00"
        }
    },
    "system-memory": {
        "memory": [
            {"type": "Mem", "total": "64252", "used": "15017", "free": "7526",
             "shared": "563", "buff/cache": "41708", "available": "46228"},
            {"type": "Swap", "total": "0", "used": "0", "free": "0"}
        ]
    },
    "lldp": {
        "table": {
            "capability_codes_helper": "Capability codes: (R) Router, (B) Bridge, (O) Other",
            "neighbors": [
                {"localPort": "Ethernet0", "remoteDevice": "switch1",
                 "remotePortId": "Ethernet48", "capability": "BR",
                 "remotePortDescr": "Uplink to spine"},
                {"localPort": "Ethernet4", "remoteDevice": "switch2",
                 "remotePortId": "Ethernet52", "capability": "R",
                 "remotePortDescr": "Connection to leaf"}
            ],
            "total": 2
        },
        "neighbors": {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "via": "LLDP",
                    "RID": "64",
                    "Time": "1 day, 02:12:01",
                    "Chassis": {
                        "ChassisID": "mac 1e:b4:64:a0:bd:0c",
                        "SysName": "ARISTA01FT2",
                        "SysDescr": "Arista Networks EOS version 4.32.5M",
                        "MgmtIP": "172.16.41.234",
                        "MgmtIface": "999000",
                        "Capability": ["Bridge, on", "Router, on"]
                    },
                    "Port": {
                        "PortID": "ifname Ethernet1",
                        "PortDescr": "",
                        "TTL": "120",
                        "MFS": "9236"
                    }
                }
            }
        }
    },
    "vlan": {
        "brief": {
            "Vlan1": {
                "vlan_id": "1",
                "ip_address": ["192.168.0.1/21"],
                "ports": [{"name": "Ethernet120", "port_tagging": "untagged"}],
                "proxy_arp": "disabled",
                "dhcp_helper_addresses": ["192.0.0.1"]
            }
        }
    },
    "reboot-cause": {
        "reboot-cause": {
            "cause": "reboot",
            "time": "Thu Jul 10 08:05:33 PM UTC 2025",
            "user": "admin"
        },
        "history": {
            "REBOOT_CAUSE|2025_07_10_20_06_34": {
                "cause": "reboot",
                "comment": "N/A",
                "time": "Thu Jul 10 08:05:33 PM UTC 2025",
                "user": "admin"
            },
            "REBOOT_CAUSE|2025_07_10_20_12_49": {
                "cause": "fast-reboot",
                "comment": "N/A",
                "time": "Thu Jul 10 08:10:49 PM UTC 2025",
                "user": "admin"
            }
        }
    },
    "interfaces": {
        "description": {
            "Ethernet0": {"Admin": "up", "Alias": "etp0",
                         "Description": "ARISTA01T1:Ethernet1", "Oper": "up"},
            "Ethernet4": {"Admin": "up", "Alias": "etp1",
                         "Description": "Servers1:eth0", "Oper": "down"}
        },
        "status": {
            "PORT_TABLE:Ethernet0": {
                "admin_status": "up", "oper_status": "up",
                "speed": "100000", "mtu": "9100",
                "alias": "etp0", "description": "ARISTA01T1:Ethernet1"
            },
            "PORT_TABLE:Ethernet40": {
                "admin_status": "up", "oper_status": "up",
                "speed": "100000", "mtu": "9100",
                "alias": "etp10", "description": "Servers4:eth0"
            }
        },
        "counters": {
            "Ethernet0": {
                "RX_OK": "149903", "RX_BPS": "50.00 Kb/s",
                "RX_UTIL": "0.00%", "RX_ERR": "0", "RX_DRP": "957",
                "TX_OK": "144782", "TX_BPS": "50.00 Kb/s",
                "TX_UTIL": "0.00%", "TX_ERR": "0", "TX_DRP": "2"
            }
        },
        "errors": [
            {"Port Errors": "oper error status", "Count": "3",
             "Last timestamp(UTC)": "2024-01-15T10:30:45Z"},
            {"Port Errors": "mac local fault", "Count": "2",
             "Last timestamp(UTC)": "2024-01-15T10:25:30Z"}
        ],
        "alias": {
            "PORT|Ethernet0": {"alias": "etp1"},
            "PORT|Ethernet8": {"alias": "etp2"}
        },
        "portchannel": {
            "PORTCHANNEL|PortChannel101": {
                "admin_status": "up", "mtu": "9100", "tpid": "0x8100"
            },
            "PORTCHANNEL|PortChannel102": {
                "admin_status": "up", "mtu": "9100", "tpid": "0x8100"
            }
        },
        "flap": {
            "PORT_TABLE:Ethernet0": {
                "flap_count": "3",
                "last_up_time": "Fri Jul 18 19:59:28 2025",
                "last_down_time": "Fri Jul 18 19:59:20 2025"
            }
        },
        "naming-mode": {"naming_mode": "default"},
        # Detailed counter variants (use same data as counters)
        "counters-detailed": {
            "Ethernet0": {
                "RX_OK": "149903", "RX_BPS": "50.00 Kb/s",
                "RX_UTIL": "0.00%", "RX_ERR": "0", "RX_DRP": "957",
                "TX_OK": "144782", "TX_BPS": "50.00 Kb/s",
                "TX_UTIL": "0.00%", "TX_ERR": "0", "TX_DRP": "2"
            }
        },
        "counters-errors": {
            "Ethernet0": {"RX_ERR": "0", "TX_ERR": "0"}
        },
        "counters-rates": {
            "Ethernet0": {"RX_BPS": "50.00 Kb/s", "TX_BPS": "50.00 Kb/s"}
        },
        "counters-rif": {
            "Ethernet0": {"RX_OK": "149903", "TX_OK": "144782"}
        },
        "counters-fec-stats": {
            "Ethernet0": {"fec_corrected": "100", "fec_uncorrectable": "0"}
        },
        "counters-fec-histogram": {
            "Ethernet0": {"bin0": "1000", "bin1": "500", "bin2": "100"}
        },
        "counters-trim": {
            "Ethernet0": {"trimmed_packets": "50"}
        },
        # Neighbor
        "neighbor": {
            "Ethernet0": {"neighbor": "switch1"},
            "Ethernet4": {"neighbor": "switch2"}
        },
        # Neighbor expected (same as neighbor but uses different subcommand)
        "neighbor-expected": {
            "Ethernet0": {"Neighbor": "ARISTA01FT2", "NeighborPort": "Ethernet1",
                         "NeighborLoopback": "None", "NeighborMgmt": "172.16.41.234",
                         "NeighborType": "FabricSpineRouter"},
            "Ethernet4": {"Neighbor": "ARISTA02FT2", "NeighborPort": "Ethernet2",
                         "NeighborLoopback": "None", "NeighborMgmt": "172.16.41.235",
                         "NeighborType": "FabricSpineRouter"}
        },
        # FEC status
        "fec-status": {
            "Ethernet0": {"fec": "rs", "status": "enabled"},
            "Ethernet4": {"fec": "none", "status": "disabled"}
        },
        # Transceiver subcommands
        "transceiver-eeprom": {
            "Ethernet0": {"vendor": "Mellanox", "serial": "MT12345"}
        },
        "transceiver-error-status": {
            "Ethernet0": {"error": "none"}
        },
        "transceiver-info": {
            "Ethernet0": {"type": "QSFP28", "vendor": "Mellanox"}
        },
        "transceiver-lpmode": {
            "Ethernet0": {"lpmode": "off"}
        },
        "transceiver-pm": {
            "Ethernet0": {"rx_power": "-5.5 dBm", "tx_power": "-3.2 dBm"}
        },
        "transceiver-presence": {
            "Ethernet0": {"presence": "present"}
        },
        "transceiver-status": {
            "Ethernet0": {"status": "ok"}
        },
        # Switchport subcommands
        "switchport-config": {
            "Ethernet0": {"mode": "access", "vlan": "1000"}
        },
        "switchport-status": {
            "Ethernet0": {"admin": "up", "oper": "up", "mode": "access"}
        }
    },
    "ndp": {
        "ndp": [
            {"address": "fc00::176", "interface": "Ethernet60",
             "mac": "aa:f9:f4:cb:d2:3f", "state": "REACHABLE"},
            {"address": "fe80::bace:f6ff:fee5:500d", "interface": "Vlan1000",
             "mac": "b8:ce:f6:e5:50:0d", "state": "REACHABLE"}
        ]
    },
    "srv6": {
        "stats": {
            "2001:db8:1::/48": {"packets": "12345", "bytes": "67890"},
            "2001:db8:2::/48": {"packets": "23456", "bytes": "78901"}
        }
    },
    "mac": {
        "mac": [
            {"No.": "1", "vlan": "1000", "macAddress": "aa:bb:cc:dd:ee:ff",
             "port": "Ethernet0", "type": "Dynamic"},
            {"No.": "2", "vlan": "1000", "macAddress": "11:22:33:44:55:66",
             "port": "Ethernet4", "type": "Static"}
        ],
        "aging-time": {"fdb_aging_time": "600"}
    },
    "mmu": {
        "mmu": {
            "pool": {"ingress_lossless_pool": {"mode": "dynamic", "size": "12345"}},
            "profile": {"pg_lossless_100000_40m_profile": {"pool": "ingress_lossless_pool"}}
        }
    },
    "services": {
        "services": [
            {
                "dockerProcessName": "snmp",
                "processes": [
                    {"pid": "123", "user": "root", "cpuPercentage": "0.1",
                     "memPercentage": "0.5", "command": "python snmpd.py"}
                ]
            }
        ]
    },
    "processes": {
        "processes": {
            "subcommands": {
                "cpu": "show/processes/cpu: Show processes information sorted by cpu usage",
                "memory": "show/processes/memory: Show processes information sorted by memory usage",
                "summary": "show/processes/summary: Show processes info"
            }
        },
        "summary": [
            {"PID": "1", "PPID": "0", "%CPU": "0.1", "%MEM": "0.2", "CMD": "systemd"},
            {"PID": "123", "PPID": "1", "%CPU": "1.5", "%MEM": "2.3", "CMD": "python"}
        ],
        "cpu": {
            "uptime": "09:57:01 up 1 day,  4:17,  1 user,  load average: 1.13, 0.87, 0.90",
            "tasks": "493 total,   1 running, 479 sleeping,   0 stopped,  13 zombie",
            "cpu_usage": "0.0 us,  5.3 sy,  0.0 ni, 94.7 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st",
            "memory_usage": "31905.5 total,  21900.5 free,   5500.3 used,   5422.1 buff/cache",
            "swap_usage": "0.0 total,      0.0 free,      0.0 used.  26405.2 avail Mem",
            "processes": [
                {"pid": "210272", "user": "root", "pr": "20", "ni": "0", "virt": "4927068", "res": "1.5g", "shr": "634152", "s": "S", "cpu": "50.0", "mem": "4.8", "time": "8,18", "command": "syncd"},
                {"pid": "1401", "user": "uuidd", "pr": "20", "ni": "0", "virt": "124428", "res": "56644", "shr": "10372", "s": "S", "cpu": "6.2", "mem": "0.2", "time": "317:32.27", "command": "redis-s+"}
            ]
        },
        "memory": {
            "uptime": "09:57:01 up 1 day,  4:17,  1 user,  load average: 1.13, 0.87, 0.90",
            "tasks": "493 total,   1 running, 479 sleeping,   0 stopped,  13 zombie",
            "cpu_usage": "0.0 us,  5.3 sy,  0.0 ni, 94.7 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st",
            "memory_usage": "31905.5 total,  21900.5 free,   5500.3 used,   5422.1 buff/cache",
            "swap_usage": "0.0 total,      0.0 free,      0.0 used.  26405.2 avail Mem",
            "processes": [
                {"pid": "777", "user": "root", "pr": "20", "ni": "0", "virt": "1000000", "res": "500000", "shr": "100000", "s": "S", "cpu": "5.0", "mem": "25.0", "time": "10:00.00", "command": "database"},
                {"pid": "666", "user": "root", "pr": "20", "ni": "0", "virt": "800000", "res": "400000", "shr": "80000", "s": "S", "cpu": "3.0", "mem": "15.0", "time": "5:00.00", "command": "teamd"}
            ]
        }
    },
    "watermark": {
        "telemetry": {"interval": "120"}
    },
    "ipv6": {
        "prefix-list": {},
        "interfaces": {},
        "route": {},
        "protocol": {},
        "bgp-neighbors": {},
        "bgp-network": {},
        "bgp-summary": {},
        "link-local-mode": {"link_local_mode": "disabled"},
        "fib": {}
    }
}


def get_test_data(command: str, subcommand: str,
                  data_type: str = "gnmi") -> dict:
    """
    Get test data for a command.
    
    Args:
        command: Command type
        subcommand: Subcommand
        data_type: "gnmi" for input data, "cli" for expected output
    
    Returns:
        Test data dictionary or expected output string
    """
    if data_type == "gnmi":
        return GNMI_JSON_TEST_DATA.get(command, {}).get(subcommand, {})
    elif data_type == "cli":
        return EXPECTED_CLI_OUTPUTS.get(command, {}).get(subcommand, "")
    else:
        raise ValueError(f"Unknown data_type: {data_type}")
