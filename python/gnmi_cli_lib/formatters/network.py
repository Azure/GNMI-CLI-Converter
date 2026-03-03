"""
Network-related formatters.

This module contains formatters for network commands:
- LldpTableFormatter: show lldp table / neighbors
- VlanBriefFormatter: show vlan brief
- NdpFormatter: show ndp
- MacFormatter: show mac
- Srv6Formatter: show srv6 stats
"""

import ipaddress
from typing import Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import DEFAULT_LLDP_CAPABILITY_CODES
from gnmi_cli_lib.utils.sorting import natural_sort_key


def ipv6_sort_key(addr: str):
    """Sort key for IPv6 addresses - sorts by integer value."""
    try:
        return int(ipaddress.ip_address(addr))
    except (ValueError, TypeError):
        # Return a large number for invalid addresses so they sort last
        return float('inf')


class LldpTableFormatter(OutputFormatter):
    """
    Formats LLDP table output.
    
    Example input:
        {"capability_codes_helper": "Capability codes: (R) Router, ...",
         "neighbors": [{"localPort": "Ethernet0", "remoteDevice": "switch1", ...}],
         "total": 2}
    
    Example output:
        Capability codes: (R) Router, (B) Bridge, (O) Other
        LocalPort  RemoteDevice  RemotePortID  Capability  RemotePortDescr
        ---------  ------------  ------------  ----------  ---------------
        Ethernet0  switch1       Ethernet48    BR          Uplink to spine
        --------------------------------------------------
        Total entries displayed:  2
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format LLDP table."""
        lines = []
        
        # Add capability codes helper
        cap_helper = data.get("capability_codes_helper", DEFAULT_LLDP_CAPABILITY_CODES)
        lines.append(cap_helper)
        
        # Build neighbors table
        neighbors = data.get("neighbors", [])
        if neighbors:
            # Sort neighbors by localPort using natural sort
            sorted_neighbors = sorted(
                neighbors,
                key=lambda x: natural_sort_key(x.get("localPort", ""))
            )
            
            table_data = []
            for neighbor in sorted_neighbors:
                if isinstance(neighbor, dict):
                    row = [
                        neighbor.get("localPort", ""),
                        neighbor.get("remoteDevice", ""),
                        neighbor.get("remotePortId", ""),
                        neighbor.get("capability", ""),
                        neighbor.get("remotePortDescr", ""),
                    ]
                    table_data.append(row)
            
            headers = ["LocalPort", "RemoteDevice", "RemotePortID",
                      "Capability", "RemotePortDescr"]
            table_str = tabulate(table_data, headers=headers, tablefmt=table_format)
            lines.append(table_str)
        
        # Add total
        total = data.get("total", 0)
        lines.append("-" * 50)
        lines.append(f"Total entries displayed:  {total}")
        
        return "\n".join(lines)


class LldpNeighborsFormatter(OutputFormatter):
    """
    Formats LLDP neighbors output in detailed multi-paragraph format.
    
    Each interface gets its own section with chassis and port details.
    
    Example input:
        {"title": "LLDP neighbors",
         "interfaces": {
           "Ethernet0": {"via": "LLDP", "RID": "64", "Time": "1 day, 02:12:01",
                         "Chassis": {"ChassisID": "mac 1e:b4:...", "SysName": "ARISTA01FT2", ...},
                         "Port": {"PortID": "ifname Ethernet1", "TTL": "120", ...}}}}
    
    Example output:
        -------------------------------------------------------------------------------
        LLDP neighbors:
        -------------------------------------------------------------------------------
        Interface:    Ethernet0, via: LLDP, RID: 64, Time: 1 day, 02:12:01
          Chassis:     
            ChassisID:    mac 1e:b4:64:a0:bd:0c
            SysName:      ARISTA01FT2
            ...
          Port:        
            PortID:       ifname Ethernet1
            TTL:          120
            ...
        -------------------------------------------------------------------------------
    """
    
    SEPARATOR = "-" * 79
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format LLDP neighbors in detailed format."""
        if not data:
            return ""
        
        lines = []
        
        # Header
        title = data.get("title", "LLDP neighbors")
        lines.append(self.SEPARATOR)
        lines.append(f"{title}:")
        lines.append(self.SEPARATOR)
        
        # Get interfaces
        interfaces = data.get("interfaces", {})
        
        # Sort interfaces by RID (Record ID) for consistent ordering
        # This matches the order that lldpctl typically uses
        def get_sort_key(intf_name):
            intf_data = interfaces.get(intf_name, {})
            try:
                return int(intf_data.get("RID", "999"))
            except (ValueError, TypeError):
                return 999
        
        for intf_name in sorted(interfaces.keys(), key=get_sort_key):
            intf_data = interfaces[intf_name]
            if not isinstance(intf_data, dict):
                continue
            
            # Interface header line
            via = intf_data.get("via", "LLDP")
            rid = intf_data.get("RID", "")
            time_val = intf_data.get("Time", "")
            lines.append(f"Interface:    {intf_name}, via: {via}, RID: {rid}, Time: {time_val}")
            
            # Chassis section
            chassis = intf_data.get("Chassis", {})
            if chassis:
                lines.append("  Chassis:     ")
                
                # ChassisID
                chassis_id = chassis.get("ChassisID", "")
                if chassis_id:
                    lines.append(f"    ChassisID:    {chassis_id}")
                
                # SysName
                sys_name = chassis.get("SysName", "")
                if sys_name:
                    lines.append(f"    SysName:      {sys_name}")
                
                # SysDescr - may span multiple lines (split on \n)
                sys_descr = chassis.get("SysDescr", "")
                if sys_descr:
                    # Split on newlines (the data contains literal \n)
                    descr_lines = sys_descr.split("\n")
                    lines.append(f"    SysDescr:     {descr_lines[0]}")
                    for dl in descr_lines[1:]:
                        lines.append(f"                  {dl}")
                
                # MgmtIP
                mgmt_ip = chassis.get("MgmtIP", "")
                if mgmt_ip:
                    lines.append(f"    MgmtIP:       {mgmt_ip}")
                
                # MgmtIface
                mgmt_iface = chassis.get("MgmtIface", "")
                if mgmt_iface:
                    lines.append(f"    MgmtIface:    {mgmt_iface}")
                
                # Capabilities
                capabilities = chassis.get("Capability", [])
                if isinstance(capabilities, list):
                    for cap in capabilities:
                        lines.append(f"    Capability:   {cap}")
                elif capabilities:
                    lines.append(f"    Capability:   {capabilities}")
            
            # Port section
            port = intf_data.get("Port", {})
            if port:
                lines.append("  Port:        ")
                
                # PortID
                port_id = port.get("PortID", "")
                if port_id:
                    lines.append(f"    PortID:       {port_id}")
                
                # PortDescr
                port_descr = port.get("PortDescr", "")
                if port_descr:
                    lines.append(f"    PortDescr:    {port_descr}")
                
                # TTL
                ttl = port.get("TTL", "")
                if ttl:
                    lines.append(f"    TTL:          {ttl}")
                
                # MFS (Maximum Frame Size)
                mfs = port.get("MFS", "")
                if mfs:
                    lines.append(f"    MFS:          {mfs}")
                
                # Port aggregation info
                port_agg = port.get("PortAggregID", "")
                if port_agg:
                    lines.append(f"    Port is aggregated. PortAggregID: {port_agg}")
            
            # VLAN section - can be a list of VLAN dicts
            vlan_info = intf_data.get("VLAN", [])
            if vlan_info:
                if isinstance(vlan_info, list):
                    for vlan in vlan_info:
                        if isinstance(vlan, dict):
                            vlan_id = vlan.get("vlan-id", vlan.get("id", ""))
                            pvid = vlan.get("pvid", "")
                            pvid_str = ", pvid: yes" if pvid == "yes" else ""
                            lines.append(f"  VLAN:         {vlan_id}{pvid_str}")
                elif isinstance(vlan_info, dict):
                    vlan_id = vlan_info.get("vlan-id", vlan_info.get("id", ""))
                    pvid = vlan_info.get("pvid", "")
                    pvid_str = ", pvid: yes" if pvid == "yes" else ""
                    lines.append(f"  VLAN:         {vlan_id}{pvid_str}")
            
            # Unknown TLVs - can be nested structure
            unknown_tlvs = intf_data.get("UnknownTLVs", [])
            if unknown_tlvs:
                lines.append("  Unknown TLVs:")
                for item in unknown_tlvs:
                    if isinstance(item, dict):
                        # Check for nested 'unknown-tlv' key
                        tlv_list = item.get("unknown-tlv", [item])
                        for tlv in tlv_list:
                            if isinstance(tlv, dict):
                                # Keys can be lowercase or mixed case
                                oui = tlv.get("oui", tlv.get("OUI", ""))
                                subtype = tlv.get("subtype", tlv.get("SubType", ""))
                                length = tlv.get("len", tlv.get("Len", ""))
                                value = tlv.get("value", tlv.get("Value", ""))
                                lines.append(f"    TLV:          OUI: {oui}, SubType: {subtype}, Len: {length} {value}")
                    else:
                        lines.append(f"    TLV:          {item}")
            
            # Separator between interfaces
            lines.append(self.SEPARATOR)
        
        return "\n".join(lines)
    
    def _wrap_text(self, text: str, width: int) -> List[str]:
        """Wrap text to specified width, preserving words."""
        if len(text) <= width:
            return [text]
        
        lines = []
        current_line = ""
        
        # Split by existing newlines first
        for paragraph in text.split("\n"):
            words = paragraph.split()
            for word in words:
                if len(current_line) + len(word) + 1 <= width:
                    if current_line:
                        current_line += " " + word
                    else:
                        current_line = word
                else:
                    if current_line:
                        lines.append(current_line)
                    current_line = word
            if current_line:
                lines.append(current_line)
                current_line = ""
        
        if current_line:
            lines.append(current_line)
        
        return lines if lines else [text]


class VlanBriefFormatter(OutputFormatter):
    """
    Formats VLAN brief output with multi-row display for readability.
    
    Example input:
        {"Vlan1": {"vlan_id": "1", "ip_address": ["192.168.0.1/21"],
                   "ports": [{"name": "Ethernet120", "port_tagging": "untagged"}],
                   "proxy_arp": "disabled", "dhcp_helper_addresses": []}}
    
    Example output:
        +-----------+--------------+---------+----------------+-------------+-----------------------+
        |   VLAN ID | IP Address   | Ports   | Port Tagging   | Proxy ARP   | DHCP Helper Address   |
        +===========+==============+=========+================+=============+=======================+
        |         1 | 192.168.0.1  | Eth120  | untagged       | disabled    | 192.0.0.1             |
        |           |              | Eth121  | untagged       |             | 192.0.0.2             |
        +-----------+--------------+---------+----------------+-------------+-----------------------+
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format VLAN brief as table with grid format.

        Uses newline-separated cell values so that tabulate's grid format
        naturally produces internal row separators between VLANs, matching
        the sonic-utilities ``show vlan brief`` output.
        """
        # Determine whether any VLAN has DHCP helpers (plugin column)
        has_dhcp = any(
            isinstance(v, dict) and v.get("dhcp_helper_addresses")
            for v in data.values()
        )

        headers = ["VLAN ID", "IP Address", "Ports", "Port Tagging", "Proxy ARP"]
        if has_dhcp:
            headers.append("DHCP Helper Address")

        if not data:
            return tabulate([], headers=headers, tablefmt="grid")

        body = []
        for vlan_name in sorted(data.keys(), key=natural_sort_key):
            vlan_info = data[vlan_name]
            if not isinstance(vlan_info, dict):
                continue

            vlan_id = vlan_info.get("vlan_id", "")
            ip_addresses = vlan_info.get("ip_address") or []
            ports = vlan_info.get("ports") or []
            proxy_arp = vlan_info.get("proxy_arp", "disabled")
            dhcp_helpers = vlan_info.get("dhcp_helper_addresses") or []

            # Sort IP addresses: IPv4 first, then IPv6
            def ip_sort_key(ip):
                is_ipv6 = ':' in ip
                return (is_ipv6, ip)
            ip_addresses = sorted(ip_addresses, key=ip_sort_key)

            # Extract port names and tagging, sort naturally
            port_entries = []
            for p in ports:
                if isinstance(p, dict):
                    port_entries.append((p.get("name", ""), p.get("port_tagging", "")))
                else:
                    port_entries.append((str(p), ""))
            port_entries.sort(key=lambda x: natural_sort_key(x[0]))

            # Build newline-separated cell values (matching sonic-utilities approach)
            row = [
                vlan_id,
                "\n".join(ip_addresses),
                "\n".join(p[0] for p in port_entries),
                "\n".join(p[1] for p in port_entries),
                proxy_arp,
            ]
            if has_dhcp:
                dhcp_helpers = sorted(dhcp_helpers, key=lambda ip: tuple(
                    int(x) for x in ip.split('.')) if '.' in ip else (999,))
                row.append("\n".join(dhcp_helpers))

            body.append(row)

        return tabulate(body, headers=headers, tablefmt="grid")


class NdpFormatter(OutputFormatter):
    """
    Formats NDP (IPv6 Neighbor) output.
    
    Example input:
        {"total_entries": 246, "entries": [
            {"address": "fc00::176", "mac_address": "0A:E2:49:FA:67:83",
             "iface": "PortChannel186", "vlan": "-", "status": "REACHABLE"}
        ]}
    
    Example output:
        Address                       MacAddress         Iface           Vlan    Status
        ----------------------------  -----------------  --------------  ------  ---------
        fc00::176                     0a:e2:49:fa:67:83  PortChannel186  -       REACHABLE
        Total number of entries 246
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format NDP as table."""
        # Handle {"entries": [...], "total_entries": N} format
        if isinstance(data, dict):
            if "entries" in data:
                items = data.get("entries", [])
                total = data.get("total_entries", len(items))
            else:
                items = list(data.values()) if data else []
                total = len(items)
        elif isinstance(data, list):
            items = data
            total = len(items)
        else:
            items = []
            total = 0
        
        if not items:
            # Show empty table with headers and total count
            headers = ["Address", "MacAddress", "Iface", "Vlan", "Status"]
            table_str = tabulate([], headers=headers, tablefmt=table_format)
            return f"{table_str}\nTotal number of entries {total}"
        
        table_data = []
        for item in items:
            if isinstance(item, dict):
                # Get mac address and convert to lowercase
                mac = item.get("mac_address", item.get("mac", ""))
                if mac:
                    mac = mac.lower()
                
                row = [
                    item.get("address", ""),
                    mac,
                    item.get("iface", item.get("interface", "")),
                    item.get("vlan", "-"),
                    item.get("status", item.get("state", "")),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        # Sort by IPv6 address (natural sort - handles mixed numeric/text like fc00::1a, fc00::2)
        table_data.sort(key=lambda x: natural_sort_key(x[0]))
        
        headers = ["Address", "MacAddress", "Iface", "Vlan", "Status"]
        table_str = tabulate(table_data, headers=headers, tablefmt=table_format)
        
        # Add total count
        lines = [table_str]
        lines.append(f"Total number of entries {total}")
        
        return "\n".join(lines)


class MacFormatter(OutputFormatter):
    """
    Formats MAC address table output.
    
    Example input:
        {"entries": [], "total": 0}
    Or:
        [{"No.": "1", "vlan": "1000", "macAddress": "aa:bb:cc:dd:ee:ff",
          "port": "Ethernet0", "type": "Dynamic"}]
    
    Example output (with entries):
        No.    Vlan    MacAddress         Port       Type
        -----  ------  -----------------  ---------  ------
            1    1000  aa:bb:cc:dd:ee:ff  Ethernet0  Dynamic
        Total number of entries 1
    
    Example output (empty):
        No.    Vlan    MacAddress    Port    Type
        -----  ------  ------------  ------  ------
        Total number of entries 0
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format MAC table."""
        # Handle {"entries": [...], "total": N} format
        if isinstance(data, dict):
            if "entries" in data:
                items = data.get("entries", [])
                total = data.get("total", len(items))
            else:
                items = list(data.values()) if data else []
                total = len(items)
        elif isinstance(data, list):
            items = data
            total = len(items)
        else:
            items = []
            total = 0
        
        headers = ["No.", "Vlan", "MacAddress", "Port", "Type"]
        
        table_data = []
        for idx, item in enumerate(items, 1):
            if isinstance(item, dict):
                row = [
                    item.get("No.", str(idx)),
                    item.get("vlan", ""),
                    item.get("macAddress", item.get("mac", "")),
                    item.get("port", ""),
                    item.get("type", ""),
                ]
                table_data.append(row)
        
        # Always show table with headers, even if empty
        table_str = tabulate(table_data, headers=headers, tablefmt=table_format)
        
        # Add totals
        lines = [table_str]
        lines.append(f"Total number of entries {total}")
        
        return "\n".join(lines)


class Srv6Formatter(OutputFormatter):
    """
    Formats SRv6 statistics output.
    
    Example input:
        {"2001:db8:1::/48": {"packets": "12345", "bytes": "67890"}, ...}
    
    Example output:
        MySID             Packets    Bytes
        ---------------  --------  -------
        2001:db8:1::/48     12345    67890
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format SRv6 stats as table."""
        table_data = []
        
        # Handle empty data case - still show headers
        if data:
            for sid in sorted(data.keys()):
                sid_data = data[sid]
                if isinstance(sid_data, dict):
                    row = [
                        sid,
                        sid_data.get("packets", "0"),
                        sid_data.get("bytes", "0"),
                    ]
                    table_data.append(row)
        
        headers = ["MySID", "Packets", "Bytes"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, numalign="right")


class MacAgingTimeFormatter(OutputFormatter):
    """
    Formats MAC aging time output.
    
    Example input:
        {"fdb_aging_time": "600s"}
    
    Example output:
        Aging time for switch is 600 seconds
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format MAC aging time as human-readable text."""
        if not data:
            return ""
        
        # Get the aging time value
        aging_time = data.get("fdb_aging_time", "0")
        
        # Remove 's' suffix if present and extract numeric value
        if isinstance(aging_time, str) and aging_time.endswith("s"):
            aging_time = aging_time[:-1]
        
        return f"Aging time for switch is {aging_time} seconds"

