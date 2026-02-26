| **Index** | **Command** | **Priority** | **Sub commands 1** | **Sub commands 2** | **Show command** | **gnmi xpath** | **minArgs** | **maxArgs** | **argument** | **request argument** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | buffer_pool | 0 | watermark | | show buffer_pool watermark | buffer_pool/watermark | 0 | 0 | | FALSE |
| 2 | buffer_pool | 0 | persistent-watermark | | show buffer_pool persistent-watermark | buffer_pool/persistent-watermark | 0 | 0 | | FALSE |
| 3 | clock | 0 | | | show clock | clock | 0 | 0 | | FALSE |
| 4 | clock | 0 | timezones | | show clock timezones | clock/timezones | 0 | 0 | | FALSE |
| 5 | dropcounters | 0 | capabilities | | show dropcounters capabilities | dropcounters/capabilities | 0 | 0 | | FALSE |
| 6 | dropcounters | 0 | configuration | | show dropcounters configuration | dropcounters/configuration | 0 | 0 | | FALSE |
| 7 | dropcounters | 0 | counts | | show dropcounters counts | dropcounters/counts | 0 | 0 | | FALSE |
| 8 | headroom-pool | 0 | watermark | | show headroom-pool watermark | headroom-pool/watermark | 0 | 0 | | FALSE |
| 9 | headroom-pool | 0 | persistent-watermark | | show headroom-pool persistent-watermark | headroom-pool/persistent-watermark | 0 | 0 | | FALSE |
| 10 | interfaces | 0 | alias | | show interfaces alias | interfaces/alias | 0 | 1 | | FALSE |
| 11 | interfaces | 0 | counters | | show interfaces counters | interfaces/counters | 0 | 0 | | FALSE |
| 12 | interfaces | 0 | counters | detailed | show interfaces counters detailed | interfaces/counters/detailed | 1 | 1 | INTERFACE_NAME | TRUE |
| 13 | interfaces | 0 | counters | errors | show interfaces counters errors | interfaces/counters/errors | 0 | 0 | | FALSE |
| 14 | interfaces | 0 | counters | fec-histogram | show interfaces counters fec-histogram | interfaces/counters/fec-histogram | 1 | 1 | INTERFACE_NAME | TRUE |
| 15 | interfaces | 0 | counters | fec-stats | show interfaces counters fec-stats | interfaces/counters/fec-stats | 0 | 0 | | FALSE |
| 16 | interfaces | 0 | counters | rates | show interfaces counters rates | interfaces/counters/rates | 0 | 0 | | FALSE |
| 17 | interfaces | 0 | counters | rif | show interfaces counters rif | interfaces/counters/rif | 0 | 1 | | FALSE |
| 18 | interfaces | 0 | counters | trim | show interfaces counters trim | interfaces/counters/trim | 0 | 1 | | FALSE |
| 19 | interfaces | 0 | description | | show interfaces description | interfaces/description | 0 | 1 | | FALSE |
| 20 | interfaces | 0 | errors | | show interfaces errors | interfaces/errors | 1 | 1 | INTERFACE_NAME | TRUE |
| 21 | interfaces | 0 | fec | status | show interfaces fec status | interfaces/fec/status | 0 | 1 | | FALSE |
| 22 | interfaces | 0 | flap | | show interfaces flap | interfaces/flap | 0 | 1 | | FALSE |
| 23 | interfaces | 0 | naming-mode | | show interfaces naming-mode | interfaces/naming-mode | 0 | 0 | | FALSE |
| 24 | interfaces | 0 | neighbor | expected | show interfaces neighbor expected | interfaces/neighbor/expected | 0 | 0 | | FALSE |
| 25 | interfaces | 0 | portchannel | | show interfaces portchannel | interfaces/portchannel | 0 | 0 | | FALSE |
| 26 | interfaces | 0 | status | | show interfaces status | interfaces/status | 0 | 1 | | FALSE |
| 27 | interfaces | 0 | switchport | config | show interfaces switchport config | interfaces/switchport/config | 0 | 0 | | FALSE |
| 28 | interfaces | 0 | switchport | status | show interfaces switchport status | interfaces/switchport/status | 0 | 0 | | FALSE |
| 29 | interfaces | 0 | transceiver | eeprom | show interfaces transceiver eeprom | interfaces/transceiver/eeprom | 0 | 1 | | FALSE |
| 30 | interfaces | 0 | transceiver | error-status | show interfaces transceiver error-status | interfaces/transceiver/error-status | 0 | 1 | | FALSE |
| 31 | interfaces | 0 | transceiver | info | show interfaces transceiver info | interfaces/transceiver/info | 0 | 1 | | FALSE |
| 32 | interfaces | 0 | transceiver | lpmode | show interfaces transceiver lpmode | interfaces/transceiver/lpmode | 0 | 1 | | FALSE |
| 33 | interfaces | 0 | transceiver | pm | show interfaces transceiver pm | interfaces/transceiver/pm | 0 | 1 | | FALSE |
| 34 | interfaces | 0 | transceiver | presence | show interfaces transceiver presence | interfaces/transceiver/presence | 0 | 1 | | FALSE |
| 35 | interfaces | 0 | transceiver | status | show interfaces transceiver status | interfaces/transceiver/status | 0 | 1 | | FALSE |
| 36 | ipv6 | 0 | prefix-list | | show ipv6 prefix-list | ipv6/prefix-list | 0 | 1 | | FALSE |
| 37 | ipv6 | 0 | interfaces | | show ipv6 interfaces | ipv6/interfaces | 0 | 0 | | FALSE |
| 38 | ipv6 | 0 | route | | show ipv6 route | ipv6/route | 0 | -1 | | FALSE |
| 39 | ipv6 | 0 | protocol | | show ipv6 protocol | ipv6/protocol | 0 | 0 | | FALSE |
| 40 | ipv6 | 0 | bgp | neighbors | show ipv6 bgp neighbors | ipv6/bgp/neighbors | 0 | 2 | | FALSE |
| 41 | ipv6 | 0 | bgp | network | show ipv6 bgp network | ipv6/bgp/network | 0 | 2 | | FALSE |
| 42 | ipv6 | 0 | bgp | summary | show ipv6 bgp summary | ipv6/bgp/summary | 0 | 0 | | FALSE |
| 43 | ipv6 | 0 | link-local-mode | | show ipv6 link-local-mode | ipv6/link-local-mode | 0 | 0 | | FALSE |
| 44 | ipv6 | 0 | fib | | show ipv6 fib | ipv6/fib | 0 | 1 | | FALSE |
| 45 | lldp | 0 | neighbors | | show lldp neighbors | lldp/neighbors | 0 | 1 | | FALSE |
| 46 | lldp | 0 | table | | show lldp table | lldp/table | 0 | 0 | | FALSE |
| 47 | mac | 0 | | | show mac | mac | 0 | 0 | | FALSE |
| 48 | mac | 0 | aging-time | | show mac aging-time | mac/aging-time | 0 | 0 | | FALSE |
| 49 | mmu | 0 | | | show mmu | mmu | 0 | 0 | | FALSE |
| 50 | ndp | 0 | | | show ndp | ndp | 0 | 1 | | FALSE |
| 51 | processes | 0 | summary | | show processes summary | processes/summary | 0 | 0 | | FALSE |
| 52 | processes | 0 | cpu | | show processes cpu | processes/cpu | 0 | 0 | | FALSE |
| 53 | processes | 0 | memory | | show processes memory | processes/memory | 0 | 0 | | FALSE |
| 54 | queue | 0 | counters | | show queue counters | queue/counters | 0 | 1 | | FALSE |
| 55 | queue | 0 | persistent-watermark | | show queue persistent-watermark | queue/persistent-watermark | 0 | 0 | | FALSE |
| 56 | queue | 0 | persistent-watermark | all | show queue persistent-watermark all | queue/persistent-watermark/all | 0 | 0 | | FALSE |
| 57 | queue | 0 | persistent-watermark | multicast | show queue persistent-watermark multicast | queue/persistent-watermark/multicast | 0 | 0 | | FALSE |
| 58 | queue | 0 | persistent-watermark | unicast | show queue persistent-watermark unicast | queue/persistent-watermark/unicast | 0 | 0 | | FALSE |
| 59 | queue | 0 | watermark | | show queue watermark | queue/watermark | 0 | 0 | | FALSE |
| 60 | queue | 0 | watermark | all | show queue watermark all | queue/watermark/all | 0 | 0 | | FALSE |
| 61 | queue | 0 | watermark | multicast | show queue watermark multicast | queue/watermark/multicast | 0 | 0 | | FALSE |
| 62 | queue | 0 | watermark | unicast | show queue watermark unicast | queue/watermark/unicast | 0 | 0 | | FALSE |
| 63 | queue | 0 | wredcounters | | show queue wredcounters | queue/wredcounters | 0 | 1 | | FALSE |
| 64 | priority-group | 0 | watermark-headroom | | show priority-group watermark-headroom | priority-group/watermark-headroom | 0 | 0 | | FALSE |
| 65 | priority-group | 0 | watermark-shared | | show priority-group watermark-shared | priority-group/watermark-shared | 0 | 0 | | FALSE |
| 66 | priority-group | 0 | persistent-watermark-headroom | | show priority-group persistent-watermark-headroom | priority-group/persistent-watermark-headroom | 0 | 0 | | FALSE |
| 67 | priority-group | 0 | persistent-watermark-shared | | show priority-group persistent-watermark-shared | priority-group/persistent-watermark-shared | 0 | 0 | | FALSE |
| 68 | reboot-cause | 0 | current | | show reboot-cause | reboot-cause/current | 0 | 0 | | FALSE |
| 69 | reboot-cause | 0 | history | | show reboot-cause history | reboot-cause/history | 0 | 0 | | FALSE |
| 70 | services | 0 | | | show services | services | 0 | 0 | | FALSE |
| 71 | srv6 | 0 | stats | | show srv6 stats | srv6/stats | 0 | 1 | | FALSE |
| 72 | system-memory | 0 | | | show system-memory | system-memory | 0 | 0 | | FALSE |
| 73 | uptime | 0 | | | show uptime | uptime | 0 | 0 | | FALSE |
| 74 | version | 0 | | | show version | version | 0 | 0 | | FALSE |
| 75 | vlan | 0 | brief | | show vlan brief | vlan/brief | 0 | 0 | | FALSE |
| 76 | watermark | 0 | telemetry | interval | show watermark telemetry interval | watermark/telemetry/interval | 0 | 0 | | FALSE |
||||||||||||
