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
    Formats dropcounters counts output as two separate tables:
    one for port-level counters (IFACE) and one for switch-level counters (DEVICE).

    This matches the sonic-utilities ``dropstat`` behaviour which calls
    ``show_port_drop_counts`` and ``show_switch_drop_counts`` separately.

    Example input:
        {
            "Ethernet0": {"State": "D", "RX_ERR": "10", "RX_DROPS": "100", ...},
            "sonic_drops_test": {"SWITCH_DROPS": "1000", "lowercase_counter": "0"}
        }

    Example output:
            IFACE    STATE    RX_ERR    RX_DROPS    TX_ERR    TX_DROPS
        ---------  -------  --------  ----------  --------  ----------
        Ethernet0        D        10         100         0           0

                  DEVICE    SWITCH_DROPS    lowercase_counter
        ----------------  --------------  -------------------
        sonic_drops_test            1000                    0
    """

    def format(
        self,
        data: Dict,
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format drop counters as two tables (port + switch)."""
        if not data:
            return ""

        # Separate port-level entries (have "State") from switch-level entries
        port_entries: Dict[str, Dict] = {}
        switch_entries: Dict[str, Dict] = {}
        for key, value in data.items():
            if isinstance(value, dict):
                if "State" in value:
                    port_entries[key] = value
                else:
                    switch_entries[key] = value

        sections: List[str] = []

        # ── Port-level table (IFACE) ──
        if port_entries:
            port_table = self._build_port_table(port_entries, table_format)
            if port_table:
                sections.append(port_table)

        # ── Switch-level table (DEVICE) ──
        if switch_entries:
            switch_table = self._build_switch_table(switch_entries, table_format)
            if switch_table:
                sections.append(switch_table)

        if not sections:
            return ""

        # Join with blank line between tables (matches sonic-utilities dropstat)
        return "\n\n".join(sections) + "\n"

    # ------------------------------------------------------------------ #

    def _build_port_table(self, entries: Dict[str, Dict],
                          table_format: str) -> str:
        """Build the IFACE table for port-level drop counters."""
        # Collect columns present across all port entries
        all_columns: set = set()
        for port_data in entries.values():
            all_columns.update(port_data.keys())

        # Order: standard columns first, then debug counters sorted
        debug_columns = sorted(c for c in all_columns
                               if c not in STANDARD_PORT_COLUMNS)
        ordered_columns: List[str] = [
            c for c in STANDARD_PORT_COLUMNS if c in all_columns
        ]
        ordered_columns.extend(debug_columns)

        headers = ["IFACE"] + [
            col.upper() if col == "State" else col
            for col in ordered_columns
        ]

        table_data = []
        for port in sorted(entries.keys(), key=natural_sort_key):
            row = [port]
            for col in ordered_columns:
                row.append(entries[port].get(col, "N/A"))
            table_data.append(row)

        if not table_data:
            return ""

        return tabulate(
            table_data,
            headers=headers,
            tablefmt=table_format,
            numalign="right",
            stralign="right",
        )

    def _build_switch_table(self, entries: Dict[str, Dict],
                            table_format: str) -> str:
        """Build the DEVICE table for switch-level drop counters."""
        # Collect columns across all switch entries, preserve order
        all_columns: set = set()
        for sw_data in entries.values():
            all_columns.update(sw_data.keys())

        ordered_columns = sorted(all_columns)
        headers = ["DEVICE"] + ordered_columns

        table_data = []
        for device in sorted(entries.keys(), key=natural_sort_key):
            row = [device]
            for col in ordered_columns:
                row.append(entries[device].get(col, "N/A"))
            table_data.append(row)

        if not table_data:
            return ""

        return tabulate(
            table_data,
            headers=headers,
            tablefmt=table_format,
            numalign="right",
            stralign="right",
        )


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

