"""
Data classes for clean configuration and type definitions.

This module provides the core data structures used throughout the library:
- CommandConfig: Configuration for a single command/subcommand combination
- CliMapping: Maps CLI command string to internal command/subcommand

These classes use Python dataclasses for clean, immutable configuration.
"""

from dataclasses import dataclass, field
from typing import List


@dataclass
class CommandConfig:
    """
    Configuration for a single command/subcommand combination.
    
    This class defines how a command's output should be formatted,
    including the title, headers, and output type.
    
    Attributes:
        title: Title displayed above the table (e.g., "Shared pool maximum occupancy:")
        output_type: Type of formatter to use ("table", "text", "list", or custom)
        headers: Column headers for table output
        key_field: Field name for the primary key (used by text/list formatters)
        value_fields: List of field names to extract values from
        dynamic_columns: If True, columns are determined from data keys
        alignment: Text alignment ("right" or "left")
        description: Human-readable description of the command
        optional_arg: Name of optional argument (e.g., "interface_name"), empty if none
    
    Example:
        >>> config = CommandConfig(
        ...     title="Interface Status",
        ...     output_type="interface_status",
        ...     description="Show interface status information"
        ... )
    """
    title: str = ""
    output_type: str = "table"  # "table", "text", "list", or custom formatter name
    headers: List[str] = field(default_factory=list)
    key_field: str = ""
    value_fields: List[str] = field(default_factory=lambda: ["Bytes"])
    dynamic_columns: bool = False
    alignment: str = "right"
    description: str = ""
    optional_arg: str = ""
    
    def __post_init__(self):
        """Validate configuration after initialization."""
        valid_alignments = ("right", "left")
        if self.alignment not in valid_alignments:
            raise ValueError(f"alignment must be one of {valid_alignments}")


@dataclass
class CliMapping:
    """
    Maps a CLI command string to internal command/subcommand pair.
    
    This class provides the mapping between user-facing CLI commands
    (like "show interfaces status") and the internal command/subcommand
    pair used by the converter.
    
    Attributes:
        cli_command: The CLI command string (e.g., "show interfaces status")
        gnmi_path: The corresponding gNMI path (e.g., "SHOW/interfaces/status")
        command: Internal command name (e.g., "interfaces")
        subcommand: Internal subcommand name (e.g., "status")
        description: Human-readable description
        out_of_process_cmd: Shell command for out-of-process execution
    
    Example:
        >>> mapping = CliMapping(
        ...     cli_command="show interfaces status",
        ...     gnmi_path="SHOW/interfaces/status",
        ...     command="interfaces",
        ...     subcommand="status",
        ...     description="Interface status information"
        ... )
    """
    cli_command: str
    gnmi_path: str
    command: str
    subcommand: str
    description: str = ""
    out_of_process_cmd: str = ""
    
    def __post_init__(self):
        """Validate mapping after initialization."""
        if not self.cli_command:
            raise ValueError("cli_command cannot be empty")
        if not self.command:
            raise ValueError("command cannot be empty")
        if not self.subcommand:
            raise ValueError("subcommand cannot be empty")
    
    def to_dict(self) -> dict:
        """Convert to dictionary for JSON serialization."""
        return {
            "cli_command": self.cli_command,
            "gnmi_path": self.gnmi_path,
            "command": self.command,
            "subcommand": self.subcommand,
            "description": self.description,
            "out_of_process_cmd": self.out_of_process_cmd,
        }
