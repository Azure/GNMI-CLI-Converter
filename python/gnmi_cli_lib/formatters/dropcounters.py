"""
Drop counters formatters.

This module contains formatters for dropcounters commands:
- DropcountersCapabilitiesFormatter: show dropcounters capabilities
- DropcountersCountsFormatter: show dropcounters counts
- DropcountersConfigurationFormatter: show dropcounters configuration
"""

from typing import Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import (
    STANDARD_PORT_COLUMNS,
    DROPCOUNTERS_FIELD_MAPPING,
    DROPCOUNTERS_HEADERS,
)
from gnmi_cli_lib.utils.sorting import natural_sort_key
from gnmi_cli_lib.utils.parsing import parse_reasons_string


class DropcountersCapabilitiesFormatter(OutputFormatter):
    """
    Formats dropcounters capabilities output.
    
    Example input:
        {"PORT_INGRESS_DROPS": {"count": "10", "reasons": "[REASON1,REASON2]"}}
    
    Example output:
        Counter Type           Total
        -------------------  -------
        PORT_INGRESS_DROPS        10
        
        PORT_INGRESS_DROPS
                REASON1
                REASON2
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format capabilities with summary table and reason lists."""
        # Build summary table
        table_data = []
        reasons_sections = []
        
        for counter_type in sorted(data.keys()):
            counter_info = data[counter_type]
            if isinstance(counter_info, dict):
                count = counter_info.get("count", "N/A")
                table_data.append([counter_type, count])
                
                # Parse reasons string: "[REASON1,REASON2,...]" -> list
                reasons_str = counter_info.get("reasons", "")
                if reasons_str:
                    reasons = parse_reasons_string(reasons_str)
                    if reasons:
                        reasons_sections.append((counter_type, reasons))
        
        # Generate output
        lines = []
        
        # Summary table
        if table_data:
            table_str = tabulate(
                table_data,
                headers=["Counter Type", "Total"],
                tablefmt=table_format,
                numalign="right",
            )
            lines.append(table_str)
        
        # Reasons sections
        for counter_type, reasons in reasons_sections:
            lines.append("")
            lines.append(counter_type)
            for reason in reasons:
                lines.append(f"        {reason}")
        
        return "\n".join(lines) + "\n"


class DropcountersCountsFormatter(OutputFormatter):
    """
    Formats dropcounters counts output for port-level drop counters.
    
    Example input:
        {"Ethernet0": {"State": "D", "RX_ERR": "10", "RX_DROPS": "100", ...}}
    
    Example output:
            IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS
        ---------  -------  --------  ----------  --------  ----------
        Ethernet0        D        10         100         0           0
    """
    
    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format port drop counters as table."""
        if not data:
            return ""
        
        # Collect all unique columns across all ports
        all_columns = set()
        for port_data in data.values():
            if isinstance(port_data, dict):
                all_columns.update(port_data.keys())
        
        # Order columns: standard first, then debug counters sorted
        debug_columns = sorted([c for c in all_columns
                               if c not in STANDARD_PORT_COLUMNS])
        ordered_columns = []
        for col in STANDARD_PORT_COLUMNS:
            if col in all_columns:
                ordered_columns.append(col)
        ordered_columns.extend(debug_columns)
        
        # Build header row (uppercase "State" -> "STATE")
        headers = ["IFACE"] + [col.upper() if col == "State" else col
                              for col in ordered_columns]
        
        # Build data rows
        table_data = []
        for port in sorted(data.keys(), key=natural_sort_key):
            port_data = data[port]
            if isinstance(port_data, dict):
                row = [port]
                for col in ordered_columns:
                    value = port_data.get(col, "N/A")
                    row.append(value)
                table_data.append(row)
        
        if not table_data:
            return ""
        
        # All columns should be right-aligned
        col_alignments = ["right"] * len(headers)
        
        table_str = tabulate(
            table_data,
            headers=headers,
            tablefmt=table_format,
            numalign="right",
            colalign=col_alignments,
        )
        
        return table_str + "\n"


class DropcountersConfigurationFormatter(OutputFormatter):
    """
    Formats dropcounters configuration output.
    
    Example input (array):
        [{"name": "DEBUG_0", "alias": "DEBUG_0", "group": "N/A",
          "type": "PORT_INGRESS_DROPS", "reason": "None", "description": "N/A"}]
    
    Example output:
        Counter    Alias    Group    Type                 Reasons    Description
        ---------  -------  -------  -------------------  ---------  -----------
        DEBUG_0    DEBUG_0  N/A      PORT_INGRESS_DROPS   None       N/A
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format configuration data as table."""
        # Handle both list and dict formats
        if isinstance(data, list):
            items = data
        elif isinstance(data, dict):
            # Convert dict to list if needed
            items = list(data.values()) if data else []
        else:
            return ""
        
        if not items:
            return ""
        
        # Build table rows
        table_data = []
        for item in items:
            if isinstance(item, dict):
                row = []
                for header in DROPCOUNTERS_HEADERS:
                    field = DROPCOUNTERS_FIELD_MAPPING.get(header, header.lower())
                    value = item.get(field, "N/A")
                    # Truncate long descriptions for display
                    if header == "Description" and len(str(value)) > 50:
                        value = str(value)[:47] + "..."
                    row.append(value)
                table_data.append(row)
        
        if not table_data:
            return ""
        
        table_str = tabulate(
            table_data,
            headers=DROPCOUNTERS_HEADERS,
            tablefmt=table_format,
        )
        
        return table_str + "\n"

