"""
Main Converter Class - GnmiToCliConverter

This module provides the main converter class that transforms gNMI JSON
responses into SONiC CLI-style output. It uses the CommandRegistry to
look up configurations and formatters.
"""

import json
from typing import Dict, Any, Optional, Union, List

from gnmi_cli_lib.registry import CommandRegistry, get_default_registry
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import (
    SAI_STAT_FIELDS,
    USER_WATERMARKS_PREFIX,
    PERSISTENT_WATERMARKS_PREFIX,
)
from gnmi_cli_lib.utils.parsing import parse_json_data


class GnmiToCliConverter:
    """
    Main converter class for transforming gNMI JSON to CLI output.
    
    This class uses the CommandRegistry to look up configurations and
    formatters, making it easy to extend with new commands.
    
    Attributes:
        table_format: Table format for tabulate (simple, grid, plain, etc.)
        registry: CommandRegistry instance for command lookup
    
    Example:
        >>> converter = GnmiToCliConverter()
        >>> json_data = {"pool1": {"Bytes": "1000"}}
        >>> output = converter.convert("buffer_pool", "watermark", json_data)
        >>> print(output)
        Shared pool maximum occupancy:
        Pool    Bytes
        ------  -----
        pool1    1000
    """
    
    @property
    def COMMAND_CONFIG(self) -> Dict:
        """Access command configuration dictionary."""
        configs = self.registry.list_commands()
        return {
            cmd: {
                subcmd: {
                    "title": config.title,
                    "headers": config.headers,
                    "key_field": config.key_field,
                    "value_fields": config.value_fields,
                    "dynamic_columns": config.dynamic_columns,
                    "alignment": config.alignment,
                    "output_type": config.output_type,
                }
                for subcmd, config in subcmds.items()
            }
            for cmd, subcmds in configs.items()
        }
    
    def __init__(self, table_format: str = "simple",
                 registry: Optional[CommandRegistry] = None):
        """
        Initialize the converter.
        
        Args:
            table_format: Table format for tabulate (simple, grid, plain, etc.)
            registry: Optional custom registry (uses default if not provided)
        """
        self.table_format = table_format
        self.registry = registry or get_default_registry()
    
    def convert(self, command: str, subcommand: str,
                json_data: Union[Dict, List, str],
                options: Optional[Dict[str, Any]] = None) -> str:
        """
        Convert gNMI JSON response to CLI-style output.
        
        Args:
            command: Command type (buffer_pool, queue, clock, etc.)
            subcommand: Subcommand (watermark, persistent-watermark, date, etc.)
            json_data: JSON data as dict or string
            options: Additional formatting options (unused, for extension)
        
        Returns:
            Formatted CLI-style string
        
        Raises:
            ValueError: If command/subcommand is not supported
            json.JSONDecodeError: If json_data string is invalid JSON
        
        Example:
            >>> converter = GnmiToCliConverter()
            >>> output = converter.convert("clock", "date", {"date": "Mon Mar 25..."})
            >>> print(output)
            Mon Mar 25...
        """
        # Parse JSON string if needed
        data = parse_json_data(json_data)
        
        # Get configuration and formatter
        config = self.registry.get_config(command, subcommand)
        formatter = self.registry.get_formatter(config.output_type)
        
        # Format and return
        return formatter.format(data, config, self.table_format)
    
    def convert_from_cli_command(self, cli_command: str,
                                  json_data: Union[Dict, List, str]) -> str:
        """
        Convert using a CLI command string.
        
        This method allows conversion using the user-facing CLI command
        string instead of the internal command/subcommand pair.
        
        Args:
            cli_command: CLI command (e.g., "show buffer_pool watermark")
            json_data: JSON data as dict or string
        
        Returns:
            Formatted CLI-style string
        
        Example:
            >>> converter = GnmiToCliConverter()
            >>> output = converter.convert_from_cli_command(
            ...     "show interfaces status",
            ...     {"PORT_TABLE:Ethernet0": {"admin_status": "up", ...}}
            ... )
        """
        mapping = self.registry.get_cli_mapping(cli_command)
        return self.convert(mapping.command, mapping.subcommand, json_data)

    def convert_raw_counters_db(self, command: str, subcommand: str,
                                raw_watermarks: Dict, name_map: Dict,
                                watermark_type: str = "user") -> str:
        """
        Convert raw COUNTERS_DB format to CLI output.
        
        Handles data with OID keys like:
        {"USER_WATERMARKS:oid:0x700...": {"SAI_BUFFER_POOL_STAT_...": "12345"}}
        
        Args:
            command: Command type (buffer_pool, headroom_pool)
            subcommand: Subcommand
            raw_watermarks: Raw watermark data from COUNTERS_DB
            name_map: Name mapping (COUNTERS_BUFFER_POOL_NAME_MAP, etc.)
            watermark_type: "user" or "persistent"
        
        Returns:
            Formatted CLI-style string
        
        Example:
            >>> converter = GnmiToCliConverter()
            >>> raw = {"USER_WATERMARKS:oid:0x7000": {"SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "1000"}}
            >>> name_map = {"COUNTERS_BUFFER_POOL_NAME_MAP": {"pool1": "oid:0x7000"}}
            >>> output = converter.convert_raw_counters_db(
            ...     "buffer_pool", "watermark", raw, name_map, "user"
            ... )
        """
        # Build reverse OID to name mapping
        oid_to_name = {}
        for map_key, mapping in name_map.items():
            if "NAME_MAP" in map_key:
                for name, oid in mapping.items():
                    oid_to_name[oid] = name
        
        # Determine prefix and stat field
        prefix = USER_WATERMARKS_PREFIX if watermark_type == "user" else PERSISTENT_WATERMARKS_PREFIX
        stat_field = SAI_STAT_FIELDS.get(command, SAI_STAT_FIELDS["buffer_pool"])
        
        # Transform to standard format
        transformed_data = {}
        for key, value in raw_watermarks.items():
            if key.startswith(prefix):
                oid = key[len(prefix):]
                if oid in oid_to_name:
                    pool_name = oid_to_name[oid]
                    bytes_value = value.get(stat_field, "N/A")
                    transformed_data[pool_name] = {"Bytes": bytes_value}
        
        return self.convert(command, subcommand, transformed_data)
    
    def _build_simple_table(self, data: Dict, config: Dict) -> List[List[str]]:
        """
        Build table for simple key-value structures.
        
        This method provides backwards compatibility for code that directly
        calls _build_simple_table on the converter instance.
        
        Args:
            data: Dict mapping keys to value dicts
            config: Config dict with value_fields
        
        Returns:
            List of rows
        """
        value_fields = config.get("value_fields", ["Bytes"])
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

