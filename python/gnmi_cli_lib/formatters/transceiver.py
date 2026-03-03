"""
Transceiver-related formatters.

This module contains specialized formatters for interface transceiver commands:
- TransceiverPresenceFormatter: show interfaces transceiver presence
- TransceiverErrorStatusFormatter: show interfaces transceiver error-status
- TransceiverEepromFormatter: show interfaces transceiver eeprom
- TransceiverStatusFormatter: show interfaces transceiver status
- TransceiverInfoFormatter: show interfaces transceiver info
- TransceiverLpmodeFormatter: show interfaces transceiver lpmode
- TransceiverPmFormatter: show interfaces transceiver pm
- FecStatusFormatter: show interfaces fec status

Note: The data can come as either:
1. A list of dicts: [{"Port": "Ethernet0", "value": ...}, {"Port": "Ethernet8", "value": ...}]
2. A dict with port names as keys: {"Ethernet0": {...}, "Ethernet8": {...}}

All formatters handle both input formats.
"""

import json
from typing import Any, Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.utils.sorting import natural_sort_key

import re


def _get_app_adv_sort_key(item: str) -> tuple:
    """
    Returns a sort key for Application Advertisement items.
    
    Sort order (by speed, descending):
    1. 800G-ETC-CR8
    2. 400GBASE-CR4 (Clause 162)
    3. 400G CR8
    4. 200GBASE-CR4 (Clause 136)
    5. 100GBASE-CR4 (Clause 92)
    6. 40GBASE-CR4 (Clause 85)
    
    The sort key is (speed_order, original_string) where speed_order
    is a numeric value that reflects the desired order.
    """
    # Define speed prefixes and their sort order (lower = earlier)
    speed_patterns = [
        (r'^800G-ETC', 0),      # 800G-ETC-CR8
        (r'^400GBASE', 1),      # 400GBASE-CR4
        (r'^400G\s*CR', 2),     # 400G CR8
        (r'^200GBASE', 3),      # 200GBASE-CR4
        (r'^100GBASE', 4),      # 100GBASE-CR4
        (r'^40GBASE', 5),       # 40GBASE-CR4
    ]
    
    for pattern, order in speed_patterns:
        if re.match(pattern, item):
            return (order, item)
    
    # Unknown patterns go last
    return (99, item)


def _normalize_to_port_dict(data: Union[List, Dict], port_key: str = "Port") -> Dict:
    """
    Normalize input data to a dict with port names as keys.
    
    Handles both:
    - List of dicts: [{"Port": "Ethernet0", "value": ...}, ...]
    - Dict with port keys: {"Ethernet0": {...}, ...}
    
    Args:
        data: Input data (list or dict)
        port_key: The key name for the port field (e.g., "Port", "Interface", "interface")
    
    Returns:
        Dict with port names as keys
    """
    if isinstance(data, list):
        result = {}
        for item in data:
            if isinstance(item, dict):
                # Look for port key in various forms
                port = item.get(port_key, item.get("Interface", item.get("interface", item.get("IFACE", ""))))
                if port:
                    # Create a copy without the port key
                    item_data = {k: v for k, v in item.items() if k not in [port_key, "Interface", "interface", "IFACE"]}
                    result[port] = item_data
        return result
    elif isinstance(data, dict):
        return data
    return {}


class TransceiverPresenceFormatter(OutputFormatter):
    """
    Formats transceiver presence output.
    
    Example input (list format):
        [{"Port": "Ethernet0", "Presence": "Present"}, {"Port": "Ethernet8", "Presence": "Not present"}]
    
    Example input (dict format):
        {"Ethernet0": "Present", "Ethernet8": "Not present"}
    
    Example output:
        Port         Presence
        -----------  -----------
        Ethernet0    Present
        Ethernet8    Not present
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver presence as table."""
        if not data:
            return ""
        
        table_data = []
        
        if isinstance(data, list):
            # List of dicts: [{"Port": "...", "Presence": "..."}]
            sorted_data = sorted(data, key=lambda x: natural_sort_key(x.get("Port", "")))
            for item in sorted_data:
                port = item.get("Port", "")
                presence = item.get("Presence", item.get("presence", ""))
                # Normalize case: "Not Present" -> "Not present"
                if str(presence).lower() == "not present":
                    presence = "Not present"
                if port:
                    table_data.append([port, str(presence)])
        else:
            # Dict format: {"Ethernet0": "Present", ...}
            for port in sorted(data.keys(), key=natural_sort_key):
                presence = data[port]
                if isinstance(presence, dict):
                    presence = presence.get("presence", presence.get("Presence", ""))
                # Normalize case: "Not Present" -> "Not present"
                if str(presence).lower() == "not present":
                    presence = "Not present"
                table_data.append([port, str(presence)])
        
        if not table_data:
            return ""
        
        headers = ["Port", "Presence"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class TransceiverErrorStatusFormatter(OutputFormatter):
    """
    Formats transceiver error status output.
    
    Example input (list format):
        [{"Port": "Ethernet0", "Error Status": "OK"}, {"Port": "Ethernet88", "Error Status": "Unplugged"}]
    
    Example input (single interface dict format - when filtering by interface):
        {"cmis_state": "READY", "error": "N/A", "status": "1"}
    
    Example output:
        Port         Error Status
        -----------  --------------
        Ethernet0    OK
        Ethernet88   Unplugged
    """
    
    def _is_single_interface_data(self, data: Dict) -> bool:
        """
        Detect if data is for a single interface (flat structure with status/error fields).
        
        Single interface data has keys like 'status', 'error', 'cmis_state' at the top level,
        rather than interface names like 'Ethernet0' as keys.
        """
        if not isinstance(data, dict):
            return False
        
        # Single interface data has these characteristic keys
        single_interface_keys = {'status', 'error', 'cmis_state'}
        return bool(single_interface_keys & set(data.keys()))
    
    def _compute_error_status(self, port_data: Dict) -> str:
        """Compute error status from port data fields."""
        status = port_data.get("status", "0")
        cmis_state = port_data.get("cmis_state", "")
        error = port_data.get("error", "N/A")
        
        # When status is "1", the transceiver is operational - report OK
        # The status field is the definitive indicator; cmis_state may lag behind
        if status == "1":
            return "OK"
        else:
            # status is "0" - unplugged or error
            if error and error != "N/A":
                return error
            return "Unplugged"
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver error status as table."""
        if not data:
            return ""
        
        table_data = []
        
        if isinstance(data, list):
            # List of dicts
            sorted_data = sorted(data, key=lambda x: natural_sort_key(x.get("Port", "")))
            for item in sorted_data:
                port = item.get("Port", "")
                error_status = item.get("Error Status", item.get("error_status", ""))
                if port:
                    table_data.append([port, str(error_status)])
        elif self._is_single_interface_data(data):
            # Single interface data - get interface name from argument
            interface_name = argument or 'Unknown'
            error_status = self._compute_error_status(data)
            table_data.append([interface_name, error_status])
        else:
            # Dict format with interface names as keys
            for port in sorted(data.keys(), key=natural_sort_key):
                port_data = data[port]
                
                if isinstance(port_data, dict):
                    error_status = self._compute_error_status(port_data)
                else:
                    error_status = str(port_data)
                
                table_data.append([port, error_status])
        
        if not table_data:
            return ""
        
        headers = ["Port", "Error Status"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class TransceiverLpmodeFormatter(OutputFormatter):
    """
    Formats transceiver low-power mode output.
    
    Example input (list format):
        [{"Port": "Ethernet0", "Low-power Mode": "Off"}, {"Port": "Ethernet8", "Low-power Mode": "Off"}]
    
    Example output:
        Port         Low-power Mode
        -----------  ----------------
        Ethernet0    Off
        Ethernet8    Off
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver lpmode as table."""
        if not data:
            return ""
        
        table_data = []
        
        if isinstance(data, list):
            # List of dicts
            sorted_data = sorted(data, key=lambda x: natural_sort_key(x.get("Port", "")))
            for item in sorted_data:
                port = item.get("Port", "")
                lpmode = item.get("Low-power Mode", item.get("lpmode", "N/A"))
                if port:
                    table_data.append([port, str(lpmode)])
        else:
            # Dict format
            for port in sorted(data.keys(), key=natural_sort_key):
                port_data = data[port]
                
                if isinstance(port_data, dict):
                    lpmode = port_data.get("lpmode", port_data.get("Low-power Mode", "N/A"))
                else:
                    lpmode = str(port_data)
                
                table_data.append([port, lpmode])
        
        if not table_data:
            return ""
        
        headers = ["Port", "Low-power Mode"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class TransceiverEepromFormatter(OutputFormatter):
    """
    Formats transceiver EEPROM output as multi-paragraph per interface.
    
    Example input (dict format):
        {"Ethernet0": "{\"Active Firmware\":\"N/A\",\"CMIS Rev\":\"5.0\",...}"}
        
    Example output:
        Ethernet0: SFP EEPROM detected
                Active Firmware: N/A
                CMIS Rev: 5.0
                ...
        
        Ethernet8: SFP EEPROM detected
                ...
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver EEPROM as multi-paragraph output."""
        if not data:
            return ""
        
        output_lines = []
        
        # Normalize to dict format if list
        if isinstance(data, list):
            data = _normalize_to_port_dict(data, "Port")
        
        for port in sorted(data.keys(), key=natural_sort_key):
            port_data = data[port]
            
            # Parse JSON string if needed
            if isinstance(port_data, str):
                try:
                    port_data = json.loads(port_data)
                except json.JSONDecodeError:
                    output_lines.append(f"{port}: {port_data}")
                    output_lines.append("")
                    continue
            
            if not isinstance(port_data, dict):
                output_lines.append(f"{port}: {port_data}")
                output_lines.append("")
                continue
            
            # Check if transceiver is detected (empty dict = not detected)
            if not port_data:
                output_lines.append(f"{port}: SFP EEPROM Not detected")
                output_lines.append("")
                continue
            
            # Build paragraph output for detected transceiver
            output_lines.append(f"{port}: SFP EEPROM detected")
            
            # Sort keys for consistent output
            for key in sorted(port_data.keys()):
                value = port_data[key]
                
                # Handle Application Advertisement array specially
                if key == "Application Advertisement" and isinstance(value, list):
                    if value:
                        # Sort Application Advertisement by speed (descending)
                        sorted_app_adv = sorted(value, key=_get_app_adv_sort_key)
                        output_lines.append(f"        {key}: {sorted_app_adv[0]}")
                        indent = " " * (8 + len(key) + 2)
                        for item in sorted_app_adv[1:]:
                            output_lines.append(f"{indent}{item}")
                elif isinstance(value, dict) and not value:
                    # Empty dict - show just key with colon (no {})
                    output_lines.append(f"        {key}:")
                elif isinstance(value, list) and not value:
                    # Empty list - show just key with colon (no [])
                    output_lines.append(f"        {key}:")
                else:
                    output_lines.append(f"        {key}: {value}")
            
            output_lines.append("")
        
        return "\n".join(output_lines).rstrip()


class TransceiverStatusFormatter(OutputFormatter):
    """
    Formats transceiver status output as multi-paragraph per interface.
    
    Example input (dict format):
        {"Ethernet0": "{\"CMIS State (SW)\":\"READY\",\"Current module state\":\"ModuleReady\",...}"}
        
    Example output:
        Ethernet0:
                CMIS State (SW): READY
                Current module state: ModuleReady
                ...
    """
    
    FIELD_ORDER = [
        "CMIS State (SW)",
        "Current module state",
        "Reason of entering the module fault state",
        "Datapath firmware fault",
        "Module firmware fault",
        "Module state changed",
        "Temperature high alarm flag",
        "Temperature high warning flag",
        "Temperature low warning flag",
        "Temperature low alarm flag",
        "Vcc high alarm flag",
        "Vcc high warning flag",
        "Vcc low warning flag",
        "Vcc low alarm flag",
    ]
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver status as multi-paragraph output."""
        if not data:
            return ""
        
        output_lines = []
        
        # Normalize to dict format if list
        if isinstance(data, list):
            data = _normalize_to_port_dict(data, "Port")
        
        for port in sorted(data.keys(), key=natural_sort_key):
            port_data = data[port]
            
            # Parse JSON string if needed
            if isinstance(port_data, str):
                try:
                    port_data = json.loads(port_data)
                except json.JSONDecodeError:
                    output_lines.append(f"{port}: ")
                    output_lines.append(f"        {port_data}")
                    output_lines.append("")
                    continue
            
            if not isinstance(port_data, dict):
                output_lines.append(f"{port}: ")
                output_lines.append(f"        {port_data}")
                output_lines.append("")
                continue
            
            # Check if transceiver is removed
            cmis_state = port_data.get("CMIS State (SW)", "")
            if cmis_state == "REMOVED":
                output_lines.append(f"{port}: Transceiver status info not applicable")
                output_lines.append("")
                continue
            
            # Build paragraph output
            output_lines.append(f"{port}: ")
            
            # Output in defined order first, then remaining keys
            ordered_keys = [k for k in self.FIELD_ORDER if k in port_data]
            remaining_keys = sorted([k for k in port_data.keys() if k not in self.FIELD_ORDER])
            
            for key in ordered_keys + remaining_keys:
                value = port_data[key]
                output_lines.append(f"        {key}: {value}")
            
            output_lines.append("")
        
        return "\n".join(output_lines).rstrip()


class TransceiverInfoFormatter(OutputFormatter):
    """
    Formats transceiver info output (same format as EEPROM).
    Uses TransceiverEepromFormatter logic.
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver info using EEPROM format."""
        return TransceiverEepromFormatter().format(data, config, table_format)


class TransceiverPmFormatter(OutputFormatter):
    """
    Formats transceiver performance monitoring output.
    
    Example input:
        [{"interface": "Ethernet0", "description": "Transceiver performance monitoring not applicable"}]
    
    Example output:
        Ethernet0: Transceiver performance monitoring not applicable
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format transceiver PM as multi-paragraph output."""
        if not data:
            return ""
        
        output_lines = []
        
        # Handle list format: [{"interface": "...", "description": "..."}]
        if isinstance(data, list):
            # Sort by interface name
            sorted_data = sorted(data, key=lambda x: natural_sort_key(x.get("interface", x.get("Port", ""))))
            for item in sorted_data:
                interface = item.get("interface", item.get("Port", ""))
                description = item.get("description", "")
                if interface:
                    output_lines.append(f"{interface}: {description}")
                    output_lines.append("")
        else:
            # Dict format
            for port in sorted(data.keys(), key=natural_sort_key):
                port_data = data[port]
                
                # Parse JSON string if needed
                if isinstance(port_data, str):
                    try:
                        port_data = json.loads(port_data)
                    except json.JSONDecodeError:
                        pass
                
                if isinstance(port_data, dict):
                    description = port_data.get("description", "")
                    if description:
                        output_lines.append(f"{port}: {description}")
                    elif port_data.get("pm_data") == "N/A" or not port_data:
                        output_lines.append(f"{port}: Transceiver performance monitoring not applicable")
                    else:
                        output_lines.append(f"{port}:")
                        for key in sorted(port_data.keys()):
                            output_lines.append(f"        {key}: {port_data[key]}")
                else:
                    output_lines.append(f"{port}: Transceiver performance monitoring not applicable")
                
                output_lines.append("")
        
        return "\n".join(output_lines).rstrip()


class FecStatusFormatter(OutputFormatter):
    """
    Formats FEC status output with correct column order and right-aligned interface names.
    
    Example input (list format):
        [{"Interface": "Ethernet0", "FEC Oper": "rs", "FEC Admin": "rs"}, ...]
    
    Example output:
          Interface    FEC Oper    FEC Admin
        -----------  ----------  -----------
          Ethernet0          rs           rs
          Ethernet8          rs           rs
         Ethernet88         N/A           rs
    """
    
    def format(
        self,
        data: Union[List, Dict],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format FEC status as table with right-aligned interface names."""
        if not data:
            return ""
        
        table_data = []
        
        if isinstance(data, list):
            # List of dicts
            sorted_data = sorted(data, key=lambda x: natural_sort_key(x.get("Interface", "")))
            
            # Find max interface name length for padding
            max_len = max((len(item.get("Interface", "")) for item in sorted_data), default=0)
            
            for item in sorted_data:
                interface = item.get("Interface", "")
                fec_oper = item.get("FEC Oper", item.get("fec_oper", "N/A"))
                fec_admin = item.get("FEC Admin", item.get("fec_admin", "N/A"))
                
                if interface:
                    # Right-align interface name
                    padded_interface = interface.rjust(max_len)
                    table_data.append([padded_interface, fec_oper, fec_admin])
        else:
            # Dict format
            max_len = max(len(port) for port in data.keys()) if data else 0
            
            for port in sorted(data.keys(), key=natural_sort_key):
                port_data = data[port]
                
                if isinstance(port_data, dict):
                    fec_oper = port_data.get("FEC Oper", port_data.get("fec_oper", "N/A"))
                    fec_admin = port_data.get("FEC Admin", port_data.get("fec_admin", "N/A"))
                else:
                    fec_oper = str(port_data)
                    fec_admin = "N/A"
                
                padded_port = port.rjust(max_len)
                table_data.append([padded_port, fec_oper, fec_admin])
        
        if not table_data:
            return ""
        
        headers = ["Interface", "FEC Oper", "FEC Admin"]
        return tabulate(table_data, headers=headers, tablefmt=table_format,
                       colalign=("right", "right", "right"))

