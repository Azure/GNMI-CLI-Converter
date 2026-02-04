"""
Watermark-related formatters.

This module contains formatters specific to watermark commands:
- WatermarkTelemetryFormatter: Formats watermark telemetry interval
- QueueWatermarkFormatter: Formats queue watermark output with proper title
"""

from typing import Dict, Optional

from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.utils.sorting import natural_sort_key


class WatermarkTelemetryFormatter(OutputFormatter):
    """
    Formats watermark telemetry interval output.
    
    Example input: {"interval": "120s"}
    Example output: Telemetry interval 120 second(s)
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format watermark telemetry interval."""
        interval = data.get("interval", "N/A")
        
        # Remove 's' suffix if present
        if isinstance(interval, str) and interval.endswith("s"):
            interval = interval[:-1]
        
        return f"Telemetry interval {interval} second(s)"


class HeadroomPoolFormatter(OutputFormatter):
    """
    Formats headroom pool watermark output.
    
    This formatter ALWAYS outputs the title and table headers, even when 
    data is empty. This matches the expected CLI behavior.
    
    Example input (with data):
        {"ingress_lossless_pool": {"Bytes": "432640"}}
    
    Example output (with data):
        Headroom pool maximum occupancy:
                     Pool    Bytes
        ---------------------  -------
        ingress_lossless_pool   432640
    
    Example output (empty data):
        Headroom pool maximum occupancy:
          Pool    Bytes
        ------  -------
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format headroom pool data with title and headers always shown."""
        lines = []
        
        # Build title
        title = config.title
        if title:
            lines.append(title)
        
        # Always use fixed headers for headroom-pool: Pool, Bytes
        headers = config.headers if config.headers else ["Pool", "Bytes"]
        
        # Build table data from dict
        table_data = []
        if data:
            for pool_name in sorted(data.keys(), key=natural_sort_key):
                pool_data = data[pool_name]
                if isinstance(pool_data, dict):
                    bytes_val = pool_data.get("Bytes", pool_data.get("bytes", "N/A"))
                    table_data.append([pool_name, bytes_val])
                elif isinstance(pool_data, (str, int)):
                    # Direct value format: {"pool_name": "bytes_value"}
                    table_data.append([pool_name, pool_data])
        
        # Always render table (even if empty - shows headers only)
        table_str = tabulate(
            table_data,
            headers=headers,
            tablefmt=table_format,
            numalign="right"
        )
        lines.append(table_str)
        
        return "\n".join(lines)


class QueueWatermarkFormatter(OutputFormatter):
    """
    Formats queue watermark output with proper title and dynamic columns.
    
    Example input:
        {"Ethernet0": {"UC0": "0", "UC1": "0", "UC2": "0", "UC3": "0", 
                       "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "7620"}}
    
    Example output:
        Egress shared pool occupancy per unicast queue:
               Port    UC0    UC1    UC2    UC3    UC4    UC5    UC6    UC7
        -----------  -----  -----  -----  -----  -----  -----  -----  -----
          Ethernet0      0      0      0      0      0      0      0   7620
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format queue watermark with title and dynamic columns."""
        if not data:
            return "Object map is empty!"
        
        lines = []
        
        # Add title if specified
        if config.title:
            lines.append(config.title)
        
        # Collect all unique columns from the data
        all_columns = set()
        for port_data in data.values():
            if isinstance(port_data, dict):
                all_columns.update(port_data.keys())
        
        # Natural sort for columns (UC0, UC1, ... or MC0, MC1, ...)
        sorted_columns = sorted(all_columns, key=natural_sort_key)
        headers = ["Port"] + sorted_columns
        
        # Build rows
        table_data = []
        for port in sorted(data.keys(), key=natural_sort_key):
            port_data = data[port]
            if isinstance(port_data, dict):
                row = [port]
                for col in sorted_columns:
                    row.append(port_data.get(col, "N/A"))
                table_data.append(row)
        
        if not table_data:
            return ""
        
        table_str = tabulate(
            table_data, 
            headers=headers, 
            tablefmt=table_format,
            numalign="right"
        )
        lines.append(table_str)
        
        return "\n".join(lines)

