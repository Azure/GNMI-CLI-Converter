"""
Interface-related formatters.

This module contains formatters for interface commands:
- InterfaceStatusFormatter: show interfaces status
- InterfaceDescriptionFormatter: show interfaces description
- InterfaceCountersFormatter: show interfaces counters
- InterfaceErrorsFormatter: show interfaces errors
- InterfaceAliasFormatter: show interfaces alias
- InterfaceFlapFormatter: show interfaces flap
- NamingModeFormatter: show interfaces naming_mode
- PortchannelFormatter: show interfaces portchannel
"""

from typing import Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import INTERFACE_COLUMN_ORDER, INTERFACE_COUNTERS_FIELD_MAP
from gnmi_cli_lib.utils.sorting import natural_sort_key
from gnmi_cli_lib.utils.parsing import (
    extract_interface_name,
    array_to_dict,
    transform_interface_data,
    INTERFACE_STATUS_FIELD_MAP,
    INTERFACE_FLAP_FIELD_MAP,
    INTERFACE_ALIAS_FIELD_MAP,
)


class InterfaceStatusFormatter(OutputFormatter):
    """
    Formats interface status output.
    
    Handles both dict and array input formats from gNMI.
    
    Example input (array format from gNMI):
        [{"Interface": "Ethernet0", "Admin": "up", "Oper": "up",
          "Speed": "800G", "Alias": "etp1", "Lanes": "17,18,19,20", 
          "FEC": "rs", "MTU": "9100", "Vlan": "routed", 
          "Type": "OSFP 8X Pluggable Transceiver", "Asym": "off"}]
    
    Example input (dict format):
        {"Ethernet0": {"admin_status": "up", "oper_status": "up", ...}}
    
    Example output:
        Interface  Lanes                Speed  MTU   FEC  Alias  Vlan    Oper  Admin  Type                           Asym PFC
        ---------  -----------------  -------  ----  ---  -----  ------  ----  -----  -----------------------------  --------
        Ethernet0  17,18,19,20,21...    800G  9100   rs   etp1  routed    up     up  OSFP 8X Pluggable Transceiver       off
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface status as table."""
        if not data:
            return ""
        
        # Transform array to dict if needed
        if isinstance(data, list):
            data = transform_interface_data(data, INTERFACE_STATUS_FIELD_MAP)
        
        table_data = []
        for key in sorted(data.keys(), key=natural_sort_key):
            intf = extract_interface_name(key)
            intf_data = data[key]
            
            if isinstance(intf_data, dict):
                # Support both gNMI field names and internal names
                row = [
                    intf,
                    intf_data.get("Lanes", intf_data.get("lanes", "")),
                    intf_data.get("Speed", intf_data.get("speed", "")),
                    intf_data.get("MTU", intf_data.get("mtu", "")),
                    intf_data.get("FEC", intf_data.get("fec", "")),
                    intf_data.get("Alias", intf_data.get("alias", "")),
                    intf_data.get("Vlan", intf_data.get("vlan", "")),
                    intf_data.get("Oper", intf_data.get("oper_status", "")),
                    intf_data.get("Admin", intf_data.get("admin_status", "")),
                    intf_data.get("Type", intf_data.get("type", "")),
                    intf_data.get("Asym", intf_data.get("asym_pfc", "")),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Lanes", "Speed", "MTU", "FEC", "Alias", 
                   "Vlan", "Oper", "Admin", "Type", "Asym PFC"]
        # Right-align Interface and Lanes columns
        col_alignments = ["right", "right", "right", "right", "right", "right",
                         "right", "right", "right", "left", "right"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, colalign=col_alignments)


class InterfaceDescriptionFormatter(OutputFormatter):
    """
    Formats interface description output.
    
    Example input:
        {"Ethernet0": {"Admin": "up", "Alias": "etp0", 
                       "Description": "Server1", "Oper": "up"}}
    
    Example output:
        Interface  Oper  Admin  Alias  Description
        ---------  ----  -----  -----  -----------
        Ethernet0  up    up     etp0   Server1
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface description as table."""
        if not data:
            return ""
        
        table_data = []
        for intf in sorted(data.keys(), key=natural_sort_key):
            intf_data = data[intf]
            if isinstance(intf_data, dict):
                row = [
                    intf,
                    intf_data.get("Oper", ""),
                    intf_data.get("Admin", ""),
                    intf_data.get("Alias", ""),
                    intf_data.get("Description", ""),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Oper", "Admin", "Alias", "Description"]
        # Right-align all columns
        col_alignments = ["right", "right", "right", "right", "right"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, colalign=col_alignments)


class InterfaceCountersFormatter(OutputFormatter):
    """
    Formats interface counters output.
    
    Handles multiple counter types:
    - Basic counters (RX_OK, TX_OK, etc.)
    - Detailed counters
    - Error counters
    - Rate counters
    - RIF counters
    - FEC stats
    - Trim counters
    
    Example input (dict format):
        {"Ethernet0": {"RX_OK": "1000", "TX_OK": "2000", ...}}
    
    Example input (list format):
        [{"BinIndex": "BIN0", "Codewords": "14734918411993"}, ...]
    
    Example output:
        IFACE      RX_OK  RX_BPS    RX_UTIL  RX_ERR  ...
        ---------  -----  --------  -------  ------  ...
        Ethernet0  1000   50.00 Kb  0.00%    0       ...
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface counters as table."""
        if not data:
            return ""
        
        options = options or {}
        
        lines = []
        
        # Add period header if period option is specified
        if options.get("period"):
            period_val = options.get("period")
            lines.append(f"The rates are calculated within {period_val} seconds period")
        
        # Handle list input - use TableFormatter's list handling
        if isinstance(data, list):
            table_output = self._format_list_data(data, config, table_format)
            if table_output:
                lines.append(table_output)
            return "\n".join(lines) if lines else ""
        
        # Handle dict input - transform gNMI field names to CLI column names
        # Collect all columns present (after mapping)
        all_columns = set()
        transformed_data = {}
        
        for intf, intf_data in data.items():
            if isinstance(intf_data, dict):
                transformed = {}
                for gnmi_key, value in intf_data.items():
                    # Map to CLI column name (None means skip this field)
                    cli_key = INTERFACE_COUNTERS_FIELD_MAP.get(gnmi_key, gnmi_key)
                    if cli_key is not None:
                        transformed[cli_key] = value
                        all_columns.add(cli_key)
                transformed_data[intf] = transformed
        
        # Order columns: standard first, then extras
        ordered_columns = [c for c in INTERFACE_COLUMN_ORDER if c in all_columns]
        extra_columns = sorted([c for c in all_columns if c not in INTERFACE_COLUMN_ORDER])
        ordered_columns.extend(extra_columns)
        
        # Build table
        headers = ["IFACE"] + ordered_columns
        table_data = []
        
        for intf in sorted(transformed_data.keys(), key=natural_sort_key):
            intf_data = transformed_data[intf]
            row = [intf]
            for col in ordered_columns:
                value = intf_data.get(col, "N/A")
                # Format numeric values with commas for RX_OK, TX_OK
                if col in ("RX_OK", "TX_OK", "RX_ERR", "RX_DRP", "RX_OVR", "TX_ERR", "TX_DRP", "TX_OVR"):
                    try:
                        num_val = int(value)
                        value = f"{num_val:,}"
                    except (ValueError, TypeError):
                        pass  # Keep original value if not numeric
                row.append(value)
            table_data.append(row)
        
        if not table_data:
            return "\n".join(lines) if lines else ""
        
        # All columns should be right-aligned (especially IFACE column)
        col_alignments = ["right"] * len(headers)
        
        table_output = tabulate(table_data, headers=headers, tablefmt=table_format, numalign="right", colalign=col_alignments)
        lines.append(table_output)
        return "\n".join(lines)
    
    def _format_list_data(self, data: List[Dict], config: CommandConfig,
                          table_format: str) -> str:
        """Format list of dicts as table."""
        if not data:
            return ""
        
        # Collect all keys from all rows
        all_keys = set()
        for row_dict in data:
            if isinstance(row_dict, dict):
                all_keys.update(row_dict.keys())
        
        headers = sorted(list(all_keys))
        table_data = []
        
        for row_dict in data:
            if isinstance(row_dict, dict):
                row = [row_dict.get(h, "") for h in headers]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        # All columns should be right-aligned
        col_alignments = ["right"] * len(headers)
        
        return tabulate(table_data, headers=headers, tablefmt=table_format, numalign="right", colalign=col_alignments)


class InterfaceErrorsFormatter(OutputFormatter):
    """
    Formats interface errors output.
    
    Example input:
        [{"Port Errors": "oper error status", "Count": "3", 
          "Last timestamp(UTC)": "2024-01-15T10:30:45Z"}]
    
    Example output:
        Port Errors        Count  Last timestamp(UTC)
        ---------------  -------  --------------------
        oper error status      3  2024-01-15T10:30:45Z
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface errors as table."""
        if isinstance(data, dict):
            items = [data] if data else []
        elif isinstance(data, list):
            items = data
        else:
            return ""
        
        if not items:
            return ""
        
        headers = ["Port Errors", "Count", "Last timestamp(UTC)"]
        table_data = []
        
        for item in items:
            if isinstance(item, dict):
                row = [
                    item.get("Port Errors", ""),
                    item.get("Count", "0"),
                    item.get("Last timestamp(UTC)", "Never"),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class InterfaceAliasFormatter(OutputFormatter):
    """
    Formats interface alias output.
    
    Handles both dict and array input formats from gNMI.
    
    Example input (array format): [{"Interface": "Ethernet0", "Alias": "etp1"}, ...]
    Example input (dict format): {"PORT|Ethernet0": {"alias": "etp1"}, ...}
    
    Example output:
        Interface  Alias
        ---------  -----
        Ethernet0  etp1
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface alias as table."""
        if not data:
            return ""
        
        # Transform array to dict if needed
        if isinstance(data, list):
            data = transform_interface_data(data, INTERFACE_ALIAS_FIELD_MAP)
        
        table_data = []
        for key in sorted(data.keys(), key=natural_sort_key):
            intf = extract_interface_name(key)
            intf_data = data[key]
            
            if isinstance(intf_data, dict):
                # Support both gNMI field names and internal names
                alias = intf_data.get("Alias", intf_data.get("alias", ""))
                table_data.append([intf, alias])
        
        if not table_data:
            return ""
        
        headers = ["Name", "Alias"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class InterfaceFlapFormatter(OutputFormatter):
    """
    Formats interface flap count output.
    
    Handles both dict and array input formats from gNMI.
    
    Example input (array format from gNMI):
        [{"Interface": "Ethernet0", "Flap Count": "1", "Admin": "Up", "Oper": "Up",
          "Link Up TimeStamp(UTC)": "Sun Jan 25 07:44:50 2026",
          "Link Down TimeStamp(UTC)": "Never"}]
    
    Example output:
        Interface      Flap Count  Admin    Oper    Link Down TimeStamp(UTC)    Link Up TimeStamp(UTC)
        -----------  ------------  -------  ------  --------------------------  ------------------------
        Ethernet0               1  Up       Up      Never                       Sun Jan 25 07:44:50 2026
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format flap counts as table."""
        if not data:
            return ""
        
        # Valid interface name prefixes
        valid_prefixes = ("Ethernet", "PortChannel", "Vlan", "Loopback", "eth")
        
        # Handle list input directly (gNMI array format)
        if isinstance(data, list):
            table_data = []
            for item in data:
                if isinstance(item, dict):
                    intf = item.get("Interface", "")
                    # Skip non-interface entries
                    if not any(intf.startswith(p) for p in valid_prefixes):
                        continue
                    row = [
                        intf,
                        item.get("Flap Count", "0"),
                        item.get("Admin", ""),
                        item.get("Oper", ""),
                        item.get("Link Down TimeStamp(UTC)", ""),
                        item.get("Link Up TimeStamp(UTC)", ""),
                    ]
                    table_data.append(row)
            
            # Sort by interface name
            table_data.sort(key=lambda x: natural_sort_key(x[0]))
        else:
            # Handle dict input
            table_data = []
            for key in sorted(data.keys(), key=natural_sort_key):
                intf = extract_interface_name(key)
                intf_data = data[key]
                
                if isinstance(intf_data, dict):
                    row = [
                        intf,
                        intf_data.get("Flap Count", intf_data.get("flap_count", "0")),
                        intf_data.get("Admin", ""),
                        intf_data.get("Oper", ""),
                        intf_data.get("Link Down TimeStamp(UTC)", intf_data.get("last_down_time", "")),
                        intf_data.get("Link Up TimeStamp(UTC)", intf_data.get("last_up_time", "")),
                    ]
                    table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Flap Count", "Admin", "Oper", "Link Down TimeStamp(UTC)", "Link Up TimeStamp(UTC)"]
        col_alignments = ["left", "right", "left", "left", "left", "left"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, colalign=col_alignments)


class NamingModeFormatter(OutputFormatter):
    """
    Formats interface naming mode output.
    
    Example input: {"naming_mode": "default"}
    Example output: default
    
    Note: The actual CLI output is just the mode value, not "Naming mode: <value>".
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format naming mode as plain text (just the value)."""
        mode = data.get("naming_mode", data.get("mode", "default"))
        return mode


class InterfaceCountersDetailedFormatter(OutputFormatter):
    """
    Formats detailed interface counters per-line output.
    
    This formatter outputs counters in a per-line format with labels and dotted alignment.
    Used for 'show interfaces counters detailed <interface>'.
    
    Example input:
        {"Ethernet0": {"Rx64": "385527", "Rx65_127": "10323", ...}}
    
    Example output:
        Packets Received 64 Octets..................... 385,527
        Packets Received 65-127 Octets................. 10,323
        ...
    """
    
    # Mapping from gNMI keys to display labels in order.
    # Matches the hardcoded field order in sonic-utilities portstat.py
    # (cnstat_print / cnstat_diff_print detailed mode).
    FIELD_ORDER = [
        ("Rx64", "Packets Received 64 Octets"),
        ("Rx65_127", "Packets Received 65-127 Octets"),
        ("Rx128_255", "Packets Received 128-255 Octets"),
        ("Rx256_511", "Packets Received 256-511 Octets"),
        ("Rx512_1023", "Packets Received 512-1023 Octets"),
        ("Rx1024_1518", "Packets Received 1024-1518 Octets"),
        ("Rx1519_2047", "Packets Received 1519-2047 Octets"),
        ("Rx2048_4095", "Packets Received 2048-4095 Octets"),
        ("Rx4096_9216", "Packets Received 4096-9216 Octets"),
        ("Rx9217_16383", "Packets Received 9217-16383 Octets"),
        None,  # Blank line separator
        ("RxAll", "Total Packets Received Without Errors"),
        ("RxUnicast", "Unicast Packets Received"),
        ("RxMulticast", "Multicast Packets Received"),
        ("RxBroadcast", "Broadcast Packets Received"),
        None,  # Blank line separator
        ("RxJabbers", "Jabbers Received"),
        ("RxFragments", "Fragments Received"),
        ("RxUndersize", "Undersize Received"),
        ("RxOverruns", "Overruns Received"),
        None,  # Blank line separator
        ("Tx64", "Packets Transmitted 64 Octets"),
        ("Tx65_127", "Packets Transmitted 65-127 Octets"),
        ("Tx128_255", "Packets Transmitted 128-255 Octets"),
        ("Tx256_511", "Packets Transmitted 256-511 Octets"),
        ("Tx512_1023", "Packets Transmitted 512-1023 Octets"),
        ("Tx1024_1518", "Packets Transmitted 1024-1518 Octets"),
        ("Tx1519_2047", "Packets Transmitted 1519-2047 Octets"),
        ("Tx2048_4095", "Packets Transmitted 2048-4095 Octets"),
        ("Tx4096_9216", "Packets Transmitted 4096-9216 Octets"),
        ("Tx9217_16383", "Packets Transmitted 9217-16383 Octets"),
        None,  # Blank line separator
        ("TxAll", "Total Packets Transmitted Successfully"),
        ("TxUnicast", "Unicast Packets Transmitted"),
        ("TxMulticast", "Multicast Packets Transmitted"),
        ("TxBroadcast", "Broadcast Packets Transmitted"),
    ]

    # WRED fields — conditional: only output when present in JSON data.
    # In sonic-utilities these are gated by PORT_COUNTER_CAPABILITIES
    # (wred_*_pkt_stat_capable).  The converter mirrors this by checking
    # whether the JSON data contains the corresponding keys.
    WRED_FIELDS = [
        ("WredGreenDrp", "WRED Green Dropped Packets"),
        ("WredYellowDrp", "WRED Yellow Dropped Packets"),
        ("WredRedDrp", "WRED Red Dropped Packets"),
        ("WredTotalDrp", "WRED Total Dropped Packets"),
    ]

    # Trim + Timestamp — always output (unconditional in sonic-utilities)
    TAIL_FIELDS = [
        ("TrimPkts", "Trimmed Packets"),
        ("TrimSent", "Trimmed Sent Packets"),
        ("TrimDrp", "Trimmed Dropped Packets"),
        None,  # Blank line separator
        ("TimestampClearedCounters", "Time Since Counters Last Cleared"),
    ]
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format detailed interface counters as per-line output."""
        if not data:
            return ""
        
        options = options or {}
        
        # Get the first (and likely only) interface's data
        lines = []
        
        # Add period header if period option is specified
        if options.get("period"):
            period_val = options.get("period")
            lines.append(f"The rates are calculated within {period_val} seconds period")
        
        for intf, intf_data in data.items():
            if not isinstance(intf_data, dict):
                continue

            # --- Main fields (always output) ---
            self._append_fields(lines, intf_data, self.FIELD_ORDER)

            # --- WRED fields (conditional — only when present in data) ---
            wred_present = [
                fs for fs in self.WRED_FIELDS
                if fs is not None and fs[0] in intf_data
            ]
            if wred_present:
                lines.append("")  # Blank line before WRED group
                self._append_fields(lines, intf_data, self.WRED_FIELDS,
                                    skip_missing=True)
                lines.append("")  # Blank line after WRED group

            # --- Trim + Timestamp (always output) ---
            self._append_fields(lines, intf_data, self.TAIL_FIELDS)

            break  # Only process first interface (detailed is per-interface)
        
        return "\n".join(lines)

    @staticmethod
    def _append_fields(lines: list, intf_data: Dict,
                       field_specs: list, skip_missing: bool = False):
        """Append formatted lines for the given field specifications.

        Args:
            lines: Accumulator list of output lines.
            intf_data: Dict of counter values for one interface.
            field_specs: List of (key, label) tuples or None (blank line).
            skip_missing: If True, skip fields not present in *intf_data*
                          instead of defaulting to "0".
        """
        for field_spec in field_specs:
            if field_spec is None:
                lines.append("")  # Blank line
                continue

            key, label = field_spec

            if skip_missing and key not in intf_data:
                continue

            value = intf_data.get(key, "0")

            # Handle missing values - default to "0" for missing keys
            # but preserve "N/A" values as-is
            if value in (None, ""):
                value = "0"

            # Format numeric values with commas
            # (skip N/A and TimestampClearedCounters)
            if value != "N/A" and key != "TimestampClearedCounters":
                try:
                    num_val = int(value)
                    value = f"{num_val:,}"
                except (ValueError, TypeError):
                    pass
            elif value not in ("None", None, "", "N/A"):
                try:
                    num_val = int(value)
                    value = f"{num_val:,}"
                except (ValueError, TypeError):
                    pass

            # Format with dotted alignment - label is 47 chars wide
            dots = "." * (47 - len(label))
            lines.append(f"{label}{dots} {value}")


class PortchannelFormatter(OutputFormatter):
    """
    Formats portchannel output with flags legend.
    
    Example input:
        {"102": {"Ports": [{"in_sync": True, "name": "Ethernet256", "selected": True, "status": "enabled"}],
                 "Protocol": {"active": True, "name": "LACP", "operational_status": "up"},
                 "Team Dev": "PortChannel102"}}
    
    Example output:
        Flags: A - active, I - inactive, Up - up, Dw - Down, N/A - not available,
               S - selected, D - deselected, * - not synced
          No.  Team Dev        Protocol     Ports
        -----  --------------  -----------  --------------
          102  PortChannel102  LACP(A)(Up)  Ethernet256(S)
    """
    
    # Flags legend to prepend to output
    FLAGS_LEGEND = """Flags: A - active, I - inactive, Up - up, Dw - Down, N/A - not available,
       S - selected, D - deselected, * - not synced"""
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format portchannel as table with flags legend."""
        headers = ["No.", "Team Dev", "Protocol", "Ports"]
        
        # Always show flags legend and table header, even with empty data
        if not data:
            # Return flags legend with empty table (just headers)
            empty_table = tabulate([], headers=headers, tablefmt=table_format)
            return f"{self.FLAGS_LEGEND}\n{empty_table}"
        
        table_data = []
        for pc_num in sorted(data.keys(), key=natural_sort_key):
            pc_data = data[pc_num]
            
            if isinstance(pc_data, dict):
                team_dev = pc_data.get("Team Dev", f"PortChannel{pc_num}")
                
                # Format Protocol: LACP(A)(Up) or LACP(I)(Dw)
                protocol_info = pc_data.get("Protocol", {})
                if isinstance(protocol_info, dict):
                    proto_name = protocol_info.get("name", "N/A")
                    active = "A" if protocol_info.get("active", False) else "I"
                    oper_status = protocol_info.get("operational_status", "down")
                    oper_short = "Up" if oper_status == "up" else "Dw"
                    protocol_str = f"{proto_name}({active})({oper_short})"
                else:
                    protocol_str = str(protocol_info)
                
                # Format Ports: Ethernet256(S) or Ethernet256(D) or Ethernet256(S)*
                ports_list = pc_data.get("Ports", [])
                # Sort ports by name for consistent output
                if ports_list and isinstance(ports_list[0], dict):
                    ports_list = sorted(ports_list, key=lambda p: natural_sort_key(p.get("name", "")))
                port_strs = []
                for port in ports_list:
                    if isinstance(port, dict):
                        port_name = port.get("name", "")
                        selected = "S" if port.get("selected", False) else "D"
                        in_sync = "" if port.get("in_sync", True) else "*"
                        port_strs.append(f"{port_name}({selected}){in_sync}")
                    else:
                        port_strs.append(str(port))
                ports_str = " ".join(port_strs)
                
                row = [pc_num, team_dev, protocol_str, ports_str]
                table_data.append(row)
        
        if not table_data:
            # Return flags legend with empty table (just headers)
            empty_table = tabulate([], headers=headers, tablefmt=table_format)
            return f"{self.FLAGS_LEGEND}\n{empty_table}"
        
        table_output = tabulate(table_data, headers=headers, tablefmt=table_format)
        
        # Prepend flags legend
        return f"{self.FLAGS_LEGEND}\n{table_output}"


class InterfaceNeighborFormatter(OutputFormatter):
    """
    Formats interface neighbor expected output.
    
    Example input:
        {"Ethernet0": {"Neighbor": "ARISTA01FT2", "NeighborPort": "Ethernet1",
                       "NeighborLoopback": "None", "NeighborMgmt": "172.16.41.234",
                       "NeighborType": "FabricSpineRouter"}}
    
    Example output:
        LocalPort    Neighbor     NeighborPort    NeighborLoopback    NeighborMgmt    NeighborType
        ---------    ----------   ------------    ----------------    ------------    ------------
        Ethernet0    ARISTA01FT2  Ethernet1       None                172.16.41.234   FabricSpineRouter
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format interface neighbor as table with all columns."""
        if not data:
            return ""
        
        table_data = []
        for intf in sorted(data.keys(), key=natural_sort_key):
            intf_data = data[intf]
            if isinstance(intf_data, dict):
                row = [
                    intf,
                    intf_data.get("Neighbor", ""),
                    intf_data.get("NeighborPort", ""),
                    intf_data.get("NeighborLoopback", ""),
                    intf_data.get("NeighborMgmt", ""),
                    intf_data.get("NeighborType", ""),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["LocalPort", "Neighbor", "NeighborPort", "NeighborLoopback", 
                   "NeighborMgmt", "NeighborType"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class FecHistogramFormatter(OutputFormatter):
    """
    Formats FEC histogram output.
    
    Converts BinIndex to "Symbol Errors Per Codeword" column name.
    
    Example input:
        [{"BinIndex": "BIN0", "Codewords": "14734918411993"},
         {"BinIndex": "BIN1", "Codewords": "59315"}]
    
    Example output:
        Symbol Errors Per Codeword         Codewords
        ----------------------------  --------------
        BIN0                          14726325120232
        BIN1                                   59282
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format FEC histogram as table with renamed columns."""
        if not data:
            return ""
        
        # Handle list input
        if isinstance(data, list):
            table_data = []
            for row in data:
                if isinstance(row, dict):
                    bin_idx = row.get("BinIndex", "")
                    codewords = row.get("Codewords", "")
                    table_data.append([bin_idx, codewords])
        else:
            # Handle dict input
            table_data = []
            for key, value in data.items():
                if isinstance(value, dict):
                    bin_idx = value.get("BinIndex", key)
                    codewords = value.get("Codewords", "")
                    table_data.append([bin_idx, codewords])
        
        if not table_data:
            return ""
        
        headers = ["Symbol Errors Per Codeword", "Codewords"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class SwitchportStatusFormatter(OutputFormatter):
    """
    Formats switchport status output with left alignment.
    
    Example input:
        [{"Interface": "Ethernet0", "Mode": "routed"}]
    
    Example output:
        Interface       Mode
        --------------  ------
        Ethernet0       routed
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format switchport status as left-aligned table."""
        if not data:
            return ""
        
        # Valid interface name prefixes
        valid_prefixes = ("Ethernet", "PortChannel", "Vlan", "Loopback", "eth")
        
        # Handle list input
        if isinstance(data, list):
            table_data = []
            for item in data:
                if isinstance(item, dict):
                    intf = item.get("Interface", "")
                    # Skip non-interface entries
                    if not any(intf.startswith(p) for p in valid_prefixes):
                        continue
                    row = [
                        intf,
                        item.get("Mode", ""),
                    ]
                    table_data.append(row)
            table_data.sort(key=lambda x: natural_sort_key(x[0]))
        else:
            table_data = []
            for intf in sorted(data.keys(), key=natural_sort_key):
                intf_data = data[intf]
                if isinstance(intf_data, dict):
                    row = [intf, intf_data.get("Mode", "")]
                    table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Mode"]
        col_alignments = ["left", "left"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, colalign=col_alignments)


class SwitchportConfigFormatter(OutputFormatter):
    """
    Formats switchport config output with left alignment.
    
    Example input:
        [{"Interface": "Ethernet0", "Mode": "routed", "Untagged": "", "Tagged": ""}]
    
    Example output:
        Interface       Mode    Untagged    Tagged
        --------------  ------  ----------  --------
        Ethernet0       routed
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format switchport config as left-aligned table."""
        if not data:
            return ""
        
        # Valid interface name prefixes
        valid_prefixes = ("Ethernet", "PortChannel", "Vlan", "Loopback", "eth")
        
        # Handle list input
        if isinstance(data, list):
            table_data = []
            for item in data:
                if isinstance(item, dict):
                    intf = item.get("Interface", "")
                    # Skip non-interface entries
                    if not any(intf.startswith(p) for p in valid_prefixes):
                        continue
                    row = [
                        intf,
                        item.get("Mode", ""),
                        item.get("Untagged", ""),
                        item.get("Tagged", ""),
                    ]
                    table_data.append(row)
            table_data.sort(key=lambda x: natural_sort_key(x[0]))
        else:
            table_data = []
            for intf in sorted(data.keys(), key=natural_sort_key):
                intf_data = data[intf]
                if isinstance(intf_data, dict):
                    row = [
                        intf,
                        intf_data.get("Mode", ""),
                        intf_data.get("Untagged", ""),
                        intf_data.get("Tagged", ""),
                    ]
                    table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Mode", "Untagged", "Tagged"]
        col_alignments = ["left", "left", "left", "left"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, colalign=col_alignments)


