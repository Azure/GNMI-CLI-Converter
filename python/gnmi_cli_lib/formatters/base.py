"""
Base output formatters using the Strategy Pattern.

This module provides:
- OutputFormatter: Abstract base class for all formatters
- TableFormatter: Generic table formatter for watermark-style data
- TextFormatter: Plain text formatter for single values
- ListFormatter: List formatter for newline-separated items
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Tuple, Union
from tabulate import tabulate

from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.utils.sorting import natural_sort_key


class OutputFormatter(ABC):
    """
    Abstract base class for output formatters.
    
    This class defines the interface that all formatters must implement.
    Each formatter is responsible for converting structured data into
    human-readable CLI-style output.
    
    The Strategy Pattern allows different formatting strategies to be
    selected at runtime based on the command type.
    
    Example:
        >>> class MyFormatter(OutputFormatter):
        ...     def format(self, data, config, table_format="simple"):
        ...         return "Formatted output"
    """
    
    @abstractmethod
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """
        Format data according to the command configuration.
        
        Args:
            data: The data to format (usually a dict)
            config: Command configuration with formatting options
            table_format: Tabulate format (simple, grid, plain, etc.)
            options: Additional options (e.g., period for counters detailed)
        
        Returns:
            Formatted string output
        """
        pass


class TableFormatter(OutputFormatter):
    """
    Formats data as a table.
    
    This is the primary formatter for watermark commands and other
    tabular data. It supports both simple key-value tables and
    dynamic column tables.
    
    Simple table example (buffer_pool):
        Pool                    Bytes
        --------------------  -------
        egress_lossless_pool    12345
        ingress_lossless_pool   67890
    
    Dynamic column example (priority-group):
        Port       PG0    PG1    PG2    PG3
        -------  -----  -----  -----  -----
        Ethernet0  100    101    102    103
    """
    
    def format(self, data: Union[Dict, List], config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format data as a table with headers."""
        if not data:
            return ""
        
        lines = []
        
        # Build title
        title = config.title
        if title:
            lines.append(title)
        
        # Handle list input - convert to table directly
        if isinstance(data, list):
            table_data, headers = self._build_list_table(data, config)
        elif config.dynamic_columns:
            table_data, headers = self._build_dynamic_table(data)
        else:
            table_data = self._build_simple_table(data, config.value_fields)
            headers = config.headers
        
        if not table_data:
            return ""
        
        # Render table
        table_str = tabulate(
            table_data,
            headers=headers,
            tablefmt=table_format,
            numalign="right" if config.alignment == "right" else "left",
            stralign="right" if config.alignment == "right" else "left",
        )
        lines.append(table_str)
        
        return "\n".join(lines)
    
    def _build_list_table(self, data: List[Dict], config: CommandConfig) -> Tuple[List[List[str]], List[str]]:
        """
        Build table from list of dicts.
        
        Args:
            data: List of dicts where each dict is a row
            config: CommandConfig for headers and column order
        
        Returns:
            Tuple of (table_data, headers)
        """
        if not data:
            return [], []
        
        # Get headers from config or from first row's keys
        if config.headers:
            headers = config.headers
        else:
            # Collect all keys from all rows to ensure we have complete headers
            all_keys = set()
            for row_dict in data:
                if isinstance(row_dict, dict):
                    all_keys.update(row_dict.keys())
            headers = sorted(list(all_keys))
        
        # Build rows
        table_data = []
        for row_dict in data:
            if isinstance(row_dict, dict):
                row = [row_dict.get(h, "") for h in headers]
                table_data.append(row)
        
        return table_data, headers
    
    def _build_simple_table(self, data: Dict, value_fields: List[str]) -> List[List[str]]:
        """
        Build table for simple key-value structures.
        
        Args:
            data: Dict mapping keys to value dicts
            value_fields: List of field names to extract
        
        Returns:
            List of rows, each row is [key, value1, value2, ...]
        """
        table_data = []
        
        for key in sorted(data.keys()):
            row = [key]
            value = data[key]
            
            for field_name in value_fields:
                if isinstance(value, dict):
                    cell_value = value.get(field_name, "N/A")
                else:
                    cell_value = str(value)
                row.append(cell_value)
            
            table_data.append(row)
        
        return table_data
    
    def _build_dynamic_table(self, data: Dict) -> Tuple[List[List[str]], List[str]]:
        """
        Build table for dynamic column structures.
        
        This is used for priority-group and queue watermarks where
        columns are determined by the data keys (PG0, PG1, UC0, UC1, etc.).
        
        Args:
            data: Dict mapping ports to column dicts
        
        Returns:
            Tuple of (table_data, headers)
        """
        # Collect all unique columns from the data
        all_columns = set()
        for port_data in data.values():
            if isinstance(port_data, dict):
                all_columns.update(port_data.keys())
        
        # Natural sort for columns (PG0, PG1, ... or UC0, UC1, ...)
        sorted_columns = sorted(all_columns, key=natural_sort_key)
        headers = ["Port"] + sorted_columns
        
        # Build rows
        table_data = []
        for port in sorted(data.keys(), key=natural_sort_key):
            row = [port]
            port_data = data[port]
            
            for col in sorted_columns:
                if isinstance(port_data, dict):
                    cell_value = port_data.get(col, "N/A")
                else:
                    cell_value = "N/A"
                row.append(cell_value)
            
            table_data.append(row)
        
        return table_data, headers


class TextFormatter(OutputFormatter):
    """
    Formats data as plain text.
    
    This formatter extracts a single value from the data dict
    and returns it as a string. Used for commands like "show clock".
    
    Example:
        Input: {"date": "Mon Mar 25 20:25:16 UTC 2019"}
        Output: Mon Mar 25 20:25:16 UTC 2019
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Return the value of the specified key field."""
        key_field = config.key_field or "date"
        return str(data.get(key_field, ""))


class ListFormatter(OutputFormatter):
    """
    Formats data as a newline-separated list.
    
    Used for commands that return a list of items, like
    "show clock timezones".
    
    Example:
        Input: {"timezones": ["America/Anchorage", "UTC", "Universal"]}
        Output:
            America/Anchorage
            UTC
            Universal
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Return items as newline-separated list."""
        key_field = config.key_field or "timezones"
        items = data.get(key_field, [])
        
        if isinstance(items, list):
            return "\n".join(str(item) for item in items)
        return str(items)


class CommandHelpFormatter(OutputFormatter):
    """
    Formats command help/usage information.
    
    Used for commands that return subcommand information rather than
    actual data, like "show queue watermark" (without unicast/multicast/all).
    
    Example:
        Input: {"subcommands": {"all": "...", "multicast": "...", "unicast": "..."}}
        Output:
            Usage: show queue persistent-watermark [OPTIONS] COMMAND [ARGS]...
            
              Show persistent WM for queues
            
            Options:
              -?, -h, --help  Show this message and exit.
            
            Commands:
              all        Show persistent WM for all queues
              multicast  Show persistent WM for multicast queues
              unicast    Show persistent WM for unicast queues
    """
    
    # Mapping from internal command names to CLI-style names
    COMMAND_NAME_MAP = {
        "Queue watermark": "queue watermark",
        "Queue persistent watermark": "queue persistent-watermark",
    }
    
    # Mapping from command type to description text
    DESCRIPTION_MAP = {
        "Queue watermark": "user WM for queues",
        "Queue persistent watermark": "persistent WM for queues",
    }
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format command help output from subcommands data."""
        subcommands = data.get("subcommands", {})
        
        if not subcommands:
            return "No subcommands available."
        
        # Build a help-style output
        lines = []
        raw_name = config.description.replace(" help", "") if config.description else "command"
        
        # Map to CLI-style name
        command_name = self.COMMAND_NAME_MAP.get(raw_name, raw_name.lower())
        description = self.DESCRIPTION_MAP.get(raw_name, raw_name)
        
        lines.append(f"Usage: show {command_name} [OPTIONS] COMMAND [ARGS]...")
        lines.append("")
        lines.append(f"  Show {description}")
        lines.append("")
        lines.append("Options:")
        lines.append("  -?, -h, --help  Show this message and exit.")
        lines.append("")
        lines.append("Commands:")
        
        for subcmd in sorted(subcommands.keys()):
            # Build subcmd description
            subcmd_desc = f"{description.replace('queues', subcmd + ' queues')}" if 'queues' in description else f"{description} {subcmd}"
            lines.append(f"  {subcmd:<10} Show {subcmd_desc}")
        
        return "\n".join(lines)

