"""
Unified Command Parser - Single entry point for CLI command parsing.

This module provides a unified CommandParser class that serves as the single
entry point for parsing CLI commands and routing them to specific converters.

The parser follows the consistent naming convention:
    show <command> <subcommand> [optional_argument|<required_argument>]

Examples:
    - show buffer_pool watermark
    - show headroom-pool persistent-watermark
    - show interfaces status [interface_name]
    - show interfaces counters detailed <interface_name>
    - show mac aging-time
    - show ipv6 prefix-list [name]
"""

import re
from typing import Dict, Any, Optional, Union, List, Tuple
from dataclasses import dataclass

from gnmi_cli_lib.registry import CommandRegistry, get_default_registry
from gnmi_cli_lib.utils.parsing import parse_json_data


@dataclass
class ParsedCommand:
    """Represents a parsed CLI command."""
    raw_command: str
    show_prefix: bool
    command: str
    subcommand: str
    argument: Optional[str] = None
    options: Optional[Dict[str, Any]] = None
    is_valid: bool = True
    error_message: Optional[str] = None


class CommandParser:
    """
    Unified command parser and converter.
    
    This class provides a single consistent entry point for parsing CLI commands
    and converting JSON data to CLI-style output. It supports the standardized
    command format: show <command> <subcommand> [argument]
    
    Attributes:
        registry: CommandRegistry instance for command lookup
        table_format: Table format for tabulate output
    
    Examples:
        >>> parser = CommandParser()
        >>> # Parse and convert with a CLI command string
        >>> output = parser.parse_and_convert("show interfaces status", json_data)
        >>> 
        >>> # Parse command to get components
        >>> parsed = parser.parse("show interfaces status Ethernet0")
        >>> print(parsed.command)  # "interfaces"
        >>> print(parsed.subcommand)  # "status"
        >>> print(parsed.argument)  # "Ethernet0"
        >>> 
        >>> # Convert with explicit components
        >>> output = parser.convert("interfaces", "status", json_data)
    """
    
    # Valid commands and their subcommands (from README)
    VALID_COMMANDS = {
        "buffer_pool": ["watermark", "persistent-watermark"],
        "headroom-pool": ["watermark", "persistent-watermark"],
        "clock": ["clock", "date", "timezones"],
        "dropcounters": ["capabilities", "configuration", "counts"],
        "interfaces": [
            "alias", "counters", "counters-detailed", "counters-errors",
            "counters-fec-histogram", "counters-fec-stats", "counters-rates",
            "counters-rif", "counters-trim", "description", "errors",
            "fec-status", "flap", "naming-mode", "neighbor-expected", "portchannel",
            "status", "switchport-config", "switchport-status",
            "transceiver-eeprom", "transceiver-error-status", "transceiver-info",
            "transceiver-lpmode", "transceiver-pm", "transceiver-presence",
            "transceiver-status"
        ],
        "ipv6": [
            "bgp-neighbors", "bgp-network", "bgp-summary", "fib",
            "interfaces", "link-local-mode", "prefix-list", "protocol", "route"
        ],
        "lldp": ["neighbors", "table"],
        "mac": ["mac", "aging-time"],
        "mmu": ["mmu"],
        "ndp": ["ndp"],
        "processes": ["cpu", "memory", "summary"],
        "priority-group": [
            "persistent-watermark-headroom", "persistent-watermark-shared",
            "watermark-headroom", "watermark-shared"
        ],
        "queue": [
            "counters", "persistent-watermark", "persistent-watermark-all", "persistent-watermark-multicast",
            "persistent-watermark-unicast", "watermark", "watermark-all", "watermark-multicast",
            "watermark-unicast", "wredcounters"
        ],
        "reboot-cause": ["current", "history"],
        "services": ["services"],
        "srv6": ["stats"],
        "system-memory": ["system-memory", "memory"],
        "uptime": ["uptime"],
        "version": ["version"],
        "vlan": ["brief"],
        "watermark": ["telemetry"],
    }
    
    # Commands that accept optional arguments
    COMMANDS_WITH_OPTIONAL_ARGS = {
        ("interfaces", "alias"): "interface_name",
        ("interfaces", "counters"): "interface_name",
        ("interfaces", "counters-detailed"): "interface_name",
        ("interfaces", "counters-errors"): "interface_name",
        ("interfaces", "counters-fec-histogram"): "interface_name",
        ("interfaces", "counters-fec-stats"): "interface_name",
        ("interfaces", "counters-rates"): "interface_name",
        ("interfaces", "counters-rif"): "interface_name",
        ("interfaces", "counters-trim"): "interface_name",
        ("interfaces", "description"): "interface_name",
        ("interfaces", "errors"): "interface_name",
        ("interfaces", "fec-status"): "interface_name",
        ("interfaces", "flap"): "interface_name",
        ("interfaces", "status"): "interface_name",
        ("interfaces", "switchport-config"): "interface_name",
        ("interfaces", "switchport-status"): "interface_name",
        ("interfaces", "transceiver-eeprom"): "interface_name",
        ("interfaces", "transceiver-error-status"): "interface_name",
        ("interfaces", "transceiver-info"): "interface_name",
        ("interfaces", "transceiver-lpmode"): "interface_name",
        ("interfaces", "transceiver-pm"): "interface_name",
        ("interfaces", "transceiver-presence"): "interface_name",
        ("interfaces", "transceiver-status"): "interface_name",
        ("ipv6", "prefix-list"): "name",
        ("ipv6", "route"): "prefix",
        ("lldp", "neighbors"): "interface_name",
        ("lldp", "table"): "interface_name",
        ("mac", "mac"): "address",
        ("queue", "counters"): "interface_name",
        ("queue", "wredcounters"): "interface_name",
        ("srv6", "stats"): "sid",
    }
    
    def __init__(self, table_format: str = "simple",
                 registry: Optional[CommandRegistry] = None):
        """
        Initialize the command parser.
        
        Args:
            table_format: Table format for tabulate (simple, grid, plain, etc.)
            registry: Optional custom registry (uses default if not provided)
        """
        self.table_format = table_format
        self.registry = registry or get_default_registry()
    
    def parse(self, cli_command: str) -> ParsedCommand:
        """
        Parse a CLI command string into its components.
        
        Args:
            cli_command: CLI command string (e.g., "show interfaces status Ethernet0")
        
        Returns:
            ParsedCommand with parsed components
        
        Examples:
            >>> parser = CommandParser()
            >>> parsed = parser.parse("show interfaces status")
            >>> print(parsed.command, parsed.subcommand)
            interfaces status
            
            >>> parsed = parser.parse("show queue counters Ethernet0")
            >>> print(parsed.argument)
            Ethernet0
            
            >>> parsed = parser.parse("show interfaces counters detailed Ethernet0 --period 2")
            >>> print(parsed.options)
            {'period': '2'}
        """
        # Normalize whitespace
        cli_command = ' '.join(cli_command.strip().split())
        
        # Extract options (e.g., --period 2)
        options = self._extract_options(cli_command)
        # Remove options from command for parsing
        cli_command = self._remove_options(cli_command)
        
        # Check for 'show' prefix
        has_show = cli_command.lower().startswith("show ")
        if has_show:
            cli_command = cli_command[5:].strip()
        
        # Split into parts
        parts = cli_command.split()
        
        if len(parts) < 1:
            return ParsedCommand(
                raw_command=cli_command,
                show_prefix=has_show,
                command="",
                subcommand="",
                is_valid=False,
                error_message="Empty command"
            )
        
        # Handle special cases where command needs normalization
        command = self._normalize_command(parts[0])
        
        # Determine subcommand
        subcommand = ""
        argument = None
        
        # Commands that can work with or without a subcommand
        # e.g., "show clock" (command=subcommand) or "show clock timezones" (has real subcommand)
        COMMANDS_WITH_OPTIONAL_SUBCOMMAND = ["ndp", "uptime", "version", "mmu", "clock", "mac", "services", "system-memory"]
        
        if command in COMMANDS_WITH_OPTIONAL_SUBCOMMAND:
            # First check if there's a valid subcommand in the remaining parts
            if len(parts) >= 2:
                potential_sub = parts[1]
                # Normalize underscore to hyphen for comparison (CLI uses both)
                potential_sub_normalized = potential_sub.lower().replace('_', '-')
                # Check if this is a valid subcommand for this command
                if potential_sub_normalized in self.VALID_COMMANDS.get(command, []):
                    # Use the normalized subcommand
                    subcommand = potential_sub_normalized
                    if len(parts) >= 3:
                        argument = ' '.join(parts[2:])
                else:
                    # Not a valid subcommand, treat command as its own subcommand
                    # and remaining parts as arguments
                    subcommand = command
                    argument = ' '.join(parts[1:])
            else:
                # No additional parts, command is its own subcommand
                subcommand = command
        elif len(parts) >= 2:
            # Handle compound subcommands (e.g., "counters detailed" -> "counters-detailed")
            potential_subcommand = self._normalize_subcommand(command, parts[1:])
            if potential_subcommand:
                subcommand = potential_subcommand["subcommand"]
                remaining_parts = potential_subcommand["remaining"]
                if remaining_parts:
                    argument = ' '.join(remaining_parts)
        else:
            return ParsedCommand(
                raw_command=cli_command,
                show_prefix=has_show,
                command=command,
                subcommand="",
                is_valid=False,
                error_message=f"Missing subcommand for '{command}'"
            )
        
        # Validate command/subcommand
        if command not in self.VALID_COMMANDS:
            return ParsedCommand(
                raw_command=cli_command,
                show_prefix=has_show,
                command=command,
                subcommand=subcommand,
                argument=argument,
                is_valid=False,
                error_message=f"Unknown command: {command}"
            )
        
        if subcommand not in self.VALID_COMMANDS[command]:
            return ParsedCommand(
                raw_command=cli_command,
                show_prefix=has_show,
                command=command,
                subcommand=subcommand,
                argument=argument,
                is_valid=False,
                error_message=f"Unknown subcommand '{subcommand}' for '{command}'"
            )
        
        return ParsedCommand(
            raw_command=cli_command,
            show_prefix=has_show,
            command=command,
            subcommand=subcommand,
            argument=argument,
            options=options if options else None,
            is_valid=True
        )
    
    def _extract_options(self, cli_command: str) -> Dict[str, Any]:
        """
        Extract command-line options like --period 2 or --iface Ethernet0 from the command.
        
        Args:
            cli_command: Full CLI command string
        
        Returns:
            Dict of option name -> value
        """
        options = {}
        
        # Match --period <value>
        period_match = re.search(r'--period\s+(\d+)', cli_command)
        if period_match:
            options['period'] = period_match.group(1)
        
        # Match --iface <value> (interface name like Ethernet0, PortChannel1, etc.)
        iface_match = re.search(r'--iface\s+(\S+)', cli_command)
        if iface_match:
            options['iface'] = iface_match.group(1)
        
        # Boolean flags for queue counters
        if '--all' in cli_command:
            options['all'] = True
        if '--trim' in cli_command:
            options['trim'] = True
        if '--nonzero' in cli_command:
            options['nonzero'] = True
        
        # Boolean flag for dom (transceiver eeprom)
        if '--dom' in cli_command:
            options['dom'] = True
        
        return options
    
    def _remove_options(self, cli_command: str) -> str:
        """
        Remove command-line options from the command string.
        
        Args:
            cli_command: Full CLI command string
        
        Returns:
            Command string with options removed
        """
        # Remove --period <value>
        cli_command = re.sub(r'\s*--period\s+\d+', '', cli_command)
        # Remove --iface <value>
        cli_command = re.sub(r'\s*--iface\s+\S+', '', cli_command)
        return cli_command.strip()
    
    def _normalize_command(self, command: str) -> str:
        """Normalize command name to standard format."""
        return command.lower()
    
    def _normalize_subcommand(self, command: str, parts: List[str]) -> Optional[Dict]:
        """
        Normalize subcommand from parts, handling compound subcommands.
        
        Returns:
            Dict with 'subcommand' and 'remaining' parts, or None if invalid
        """
        if not parts:
            return None
        
        # Try compound subcommands first (2 or 3 words)
        for length in [3, 2]:
            if len(parts) >= length:
                potential = '-'.join(parts[:length]).lower()
                if command in self.VALID_COMMANDS and potential in self.VALID_COMMANDS[command]:
                    return {
                        "subcommand": potential,
                        "remaining": parts[length:]
                    }
        
        # Single word subcommand
        subcommand = parts[0].lower().replace('_', '-')
        return {
            "subcommand": subcommand,
            "remaining": parts[1:]
        }
    
    def convert(self, command: str, subcommand: str,
                json_data: Union[Dict, List, str],
                argument: Optional[str] = None,
                options: Optional[Dict[str, Any]] = None) -> str:
        """
        Convert JSON data to CLI output using command and subcommand.
        
        Args:
            command: Command type (e.g., "interfaces", "queue")
            subcommand: Subcommand (e.g., "status", "counters")
            json_data: JSON data as dict, list, or string
            argument: Optional argument for filtering (e.g., interface name)
            options: Optional command options (e.g., {"period": "2"})
        
        Returns:
            Formatted CLI-style string
        
        Raises:
            ValueError: If command/subcommand is not supported
        """
        # Parse JSON if needed
        data = parse_json_data(json_data)
        
        # Apply argument filter if provided
        if argument and isinstance(data, dict):
            data = self._filter_by_argument(command, subcommand, data, argument)
        
        # Get configuration and formatter
        # Map to registry's internal naming
        internal_command, internal_subcommand = self._map_to_internal(command, subcommand)
        
        config = self.registry.get_config(internal_command, internal_subcommand)
        formatter = self.registry.get_formatter(config.output_type)
        
        # Pass argument to formatter via options (for single-interface formatting)
        format_options = options.copy() if options else {}
        if argument:
            format_options['argument'] = argument
        
        return formatter.format(data, config, self.table_format, format_options if format_options else None)
    
    def _map_to_internal(self, command: str, subcommand: str) -> Tuple[str, str]:
        """Map public command names to internal registry names."""
        # Handle specific mappings where registry uses different names
        # The registry uses hyphen naming consistently, so we map underscore to hyphen
        mapping = {
            # (public_command, public_subcommand): (internal_command, internal_subcommand)
            # headroom-pool uses hyphen in both CLI and registry
            ("headroom-pool", "watermark"): ("headroom-pool", "watermark"),
            ("headroom-pool", "persistent-watermark"): ("headroom-pool", "persistent-watermark"),
            # mac aging-time: CLI uses hyphen, registry uses hyphen
            ("mac", "aging-time"): ("mac", "aging-time"),
            # interfaces naming-mode: CLI uses hyphen, registry uses hyphen 
            ("interfaces", "naming-mode"): ("interfaces", "naming-mode"),
            # ipv6 prefix-list: CLI uses hyphen, registry uses hyphen
            ("ipv6", "prefix-list"): ("ipv6", "prefix-list"),
            # ipv6 link-local-mode: CLI uses hyphen, registry uses hyphen
            ("ipv6", "link-local-mode"): ("ipv6", "link-local-mode"),
        }
        
        key = (command, subcommand)
        if key in mapping:
            return mapping[key]
        
        return (command, subcommand)
    
    def _filter_by_argument(self, command: str, subcommand: str,
                            data: Dict, argument: str) -> Dict:
        """Filter data based on argument (e.g., interface name)."""
        if not data:
            return data
        
        # Determine key pattern based on argument
        filtered = {}
        for key, value in data.items():
            # Check various patterns
            if argument in key or key.startswith(argument) or key.endswith(argument):
                filtered[key] = value
            elif ':' in key and argument in key.split(':')[0]:
                filtered[key] = value
        
        return filtered if filtered else data
    
    def parse_and_convert(self, cli_command: str,
                          json_data: Union[Dict, List, str]) -> str:
        """
        Parse CLI command and convert JSON data in one step.
        
        This is the main entry point for command parsing and conversion.
        
        Args:
            cli_command: CLI command string (e.g., "show interfaces status")
            json_data: JSON data to convert
        
        Returns:
            Formatted CLI-style string
        
        Raises:
            ValueError: If command is invalid or not supported
        
        Examples:
            >>> parser = CommandParser()
            >>> output = parser.parse_and_convert(
            ...     "show interfaces status",
            ...     {"Ethernet0": {"admin_status": "up", "oper_status": "up"}}
            ... )
        """
        parsed = self.parse(cli_command)
        
        if not parsed.is_valid:
            raise ValueError(parsed.error_message)
        
        return self.convert(
            parsed.command,
            parsed.subcommand,
            json_data,
            parsed.argument,
            parsed.options
        )
    
    def get_gnmi_path(self, cli_command: str) -> Optional[str]:
        """
        Get the gNMI path for a CLI command.
        
        Args:
            cli_command: CLI command string
        
        Returns:
            gNMI path string, or None if not found
        """
        try:
            mapping = self.registry.get_cli_mapping(cli_command)
            return mapping.gnmi_path
        except ValueError:
            # Try parsing and looking up by components
            parsed = self.parse(cli_command)
            if parsed.is_valid:
                normalized = f"show {parsed.command} {parsed.subcommand}"
                try:
                    mapping = self.registry.get_cli_mapping(normalized)
                    return mapping.gnmi_path
                except ValueError:
                    pass
        return None
    
    def list_commands(self) -> Dict[str, List[str]]:
        """
        List all supported commands and subcommands.
        
        Returns:
            Dict mapping command -> list of subcommands
        """
        return self.VALID_COMMANDS.copy()
    
    def get_command_help(self, command: str,
                         subcommand: Optional[str] = None) -> str:
        """
        Get help text for a command.
        
        Args:
            command: Command name
            subcommand: Optional subcommand name
        
        Returns:
            Help text string
        """
        if command not in self.VALID_COMMANDS:
            return f"Unknown command: {command}"
        
        if subcommand:
            if subcommand not in self.VALID_COMMANDS[command]:
                return f"Unknown subcommand: {subcommand}"
            
            # Check for optional arguments
            key = (command, subcommand)
            if key in self.COMMANDS_WITH_OPTIONAL_ARGS:
                arg_name = self.COMMANDS_WITH_OPTIONAL_ARGS[key]
                return f"show {command} {subcommand} [{arg_name}]"
            return f"show {command} {subcommand}"
        
        # List all subcommands
        subcommands = self.VALID_COMMANDS[command]
        lines = [f"show {command} <subcommand>", "", "Subcommands:"]
        for sc in sorted(subcommands):
            lines.append(f"  {sc}")
        return '\n'.join(lines)


# Singleton instance
_parser: Optional[CommandParser] = None


def get_default_parser() -> CommandParser:
    """Get the default command parser instance."""
    global _parser
    if _parser is None:
        _parser = CommandParser()
    return _parser


def parse_command(cli_command: str) -> ParsedCommand:
    """Parse a CLI command string (convenience function)."""
    return get_default_parser().parse(cli_command)


def parse_and_convert(cli_command: str,
                      json_data: Union[Dict, List, str]) -> str:
    """Parse and convert in one step (convenience function)."""
    return get_default_parser().parse_and_convert(cli_command, json_data)

