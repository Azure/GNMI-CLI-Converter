"""
gnmi_cli_lib - Modular gNMI to SONiC CLI Conversion Library

A production-ready, self-contained Python library for transforming gNMI JSON
responses into the human-readable format used by SONiC CLI show commands.

Architecture:
    - formatters/: Output formatters for different command types
    - common/: Shared types, constants, and data classes
    - utils/: Utility functions (sorting, parsing, etc.)
    - registry.py: Central command registry
    - parser.py: Unified command parser (single entry point)

Example Usage:
    >>> from gnmi_cli_lib import CommandParser
    >>>
    >>> # Using the unified command parser (recommended)
    >>> parser = CommandParser()
    >>> output = parser.parse_and_convert("show interfaces status", json_data)

Command Naming Convention:
    All commands follow the format: show <command> <subcommand> [optional_arg|<required_arg>]

    Examples:
    - show buffer_pool watermark
    - show headroom-pool persistent-watermark
    - show interfaces status [interface_name]
    - show mac aging-time
    - show ipv6 prefix-list [name]

Supported Commands (78 commands across 21 categories):
    - Watermark: buffer_pool, headroom-pool, priority-group, queue
    - System: version, uptime, memory, processes, services
    - Interface: status, counters, description, transceiver, FEC
    - Network: LLDP, MAC, NDP, VLAN, BGP
    - And more

Testing:
    - 1014 unit tests (93% code coverage)
    - 704 real device integration test cases (100% pass rate)
    - Tested on 6 production SONiC devices

Production Status:
    - Version 2.7.0
    - Self-contained: only requires Python 3.8+ and tabulate
"""

__version__ = "2.7.0"
__author__ = "SONiC Team"

# Unified command parser (single entry point)
from gnmi_cli_lib.parser import (
    CommandParser,
    ParsedCommand,
    get_default_parser,
    parse_command,
    parse_and_convert,
)

# Registry
from gnmi_cli_lib.registry import CommandRegistry, get_default_registry

# Common types and data classes
from gnmi_cli_lib.common.types import CommandConfig, CliMapping

__all__ = [
    # Version
    "__version__",
    # Core classes
    "CommandRegistry",
    "get_default_registry",
    # Unified command parser
    "CommandParser",
    "ParsedCommand",
    "get_default_parser",
    "parse_command",
    "parse_and_convert",
    # Types
    "CommandConfig",
    "CliMapping",
]

