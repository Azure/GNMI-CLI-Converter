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
        self._valid_commands_cache: Optional[Dict[str, List[str]]] = None
        self._optional_args_cache: Optional[Dict[tuple, str]] = None
        self._optional_subcommand_cache: Optional[List[str]] = None

    @property
    def VALID_COMMANDS(self) -> Dict[str, List[str]]:
        """Valid commands and subcommands, derived from the registry."""
        if self._valid_commands_cache is None:
            self._valid_commands_cache = {
                cmd: list(subcmds.keys())
                for cmd, subcmds in self.registry.list_commands().items()
            }
        return self._valid_commands_cache

    @property
    def COMMANDS_WITH_OPTIONAL_ARGS(self) -> Dict[tuple, str]:
        """Commands that accept optional arguments, derived from the registry."""
        if self._optional_args_cache is None:
            self._optional_args_cache = {}
            for cmd, subcmds in self.registry.list_commands().items():
                for subcmd, config in subcmds.items():
                    if config.optional_arg:
                        self._optional_args_cache[(cmd, subcmd)] = config.optional_arg
        return self._optional_args_cache
    
    @property
    def COMMANDS_WITH_OPTIONAL_SUBCOMMAND(self) -> List[str]:
        """Commands that can work with or without a subcommand, derived from the registry.

        A command has an optional subcommand when the command name itself
        appears as one of its own subcommands (e.g., 'clock' has subcommand 'clock').
        """
        if self._optional_subcommand_cache is None:
            self._optional_subcommand_cache = [
                cmd for cmd, subcmds in self.VALID_COMMANDS.items()
                if cmd in subcmds
            ]
        return self._optional_subcommand_cache

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
        
        if command in self.COMMANDS_WITH_OPTIONAL_SUBCOMMAND:
            # First check if there's a valid subcommand in the remaining parts
            if len(parts) >= 2:
                # Try compound-subcommand matching first (e.g. "watermark shared"
                # -> "watermark-shared") so that commands which have BOTH a
                # self-referential intermediate entry AND compound leaf
                # subcommands (e.g. priority-group, queue, interfaces, ipv6)
                # route correctly.
                compound = self._normalize_subcommand(command, parts[1:])
                valid_subs = self.VALID_COMMANDS.get(command, [])
                if compound and compound["subcommand"] in valid_subs:
                    subcommand = compound["subcommand"]
                    remaining = compound.get("remaining", [])
                    if remaining:
                        argument = ' '.join(remaining)
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
        
        # Get configuration and formatter
        config = self.registry.get_config(command, subcommand)
        formatter = self.registry.get_formatter(config.output_type)
        
        return formatter.format(
            data, config, self.table_format,
            argument=argument or "",
            options=options or {}
        )
    
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

