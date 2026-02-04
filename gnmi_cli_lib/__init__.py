"""
gnmi_cli_lib - Modular gNMI to SONiC CLI Conversion Library

A production-ready, self-contained Python library for transforming gNMI JSON
responses into the human-readable format used by SONiC CLI show commands.

Architecture:
    - formatters/: Output formatters for different command types
    - common/: Shared types, constants, and data classes
    - utils/: Utility functions (sorting, parsing, etc.)
    - real_device_tests/: Real device verification suite (398 tests)
    - registry.py: Central command registry
    - converter.py: Main converter class
    - parser.py: Unified command parser (single entry point)
    - mappings.py: CLI-to-gNMI path mappings

Example Usage:
    >>> from gnmi_cli_lib import GnmiToCliConverter, convert, CommandParser
    >>> 
    >>> # Using the unified command parser (recommended)
    >>> parser = CommandParser()
    >>> output = parser.parse_and_convert("show interfaces status", json_data)
    >>> 
    >>> # Using the converter class directly
    >>> converter = GnmiToCliConverter()
    >>> output = converter.convert("buffer_pool", "watermark", {"pool1": {"Bytes": "1000"}})
    >>> 
    >>> # Using convenience function
    >>> output = convert("interfaces", "status", json_data)

Command Naming Convention:
    All commands follow the format: show <command> <subcommand> [optional_arg|<required_arg>]
    
    Examples:
    - show buffer_pool watermark
    - show headroom-pool persistent-watermark
    - show interfaces status [interface_name]
    - show mac aging-time
    - show ipv6 prefix-list [name]

Supported Commands (70 commands across 20 categories):
    - Watermark: buffer_pool, headroom-pool, priority-group, queue
    - System: version, uptime, memory, processes, services
    - Interface: status, counters, description, transceiver, FEC
    - Network: LLDP, MAC, NDP, VLAN, BGP
    - And many more (90+ CLI command mappings)

Testing:
    - 322 unit tests (53% code coverage)
    - 398 real device integration tests (100% pass rate)
    - Tested on 4 production SONiC devices

Production Status:
    - Version 2.4.0 - Enhanced test coverage
    - 100% pass rate verified against 398 real device test cases
    - Self-contained: only requires Python 3.8+ and tabulate
    - Self-contained with only 'tabulate' as external dependency
"""

__version__ = "2.4.0"
__author__ = "SONiC Team"

# Core converter
from gnmi_cli_lib.converter import GnmiToCliConverter

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

# Convenience functions
from gnmi_cli_lib.convenience import (
    convert,
    convert_buffer_pool_watermark,
    convert_headroom_pool_watermark,
    convert_priority_group_watermark,
    convert_queue_watermark,
    convert_clock,
    convert_clock_timezones,
    convert_dropcounters_capabilities,
    convert_dropcounters_configuration,
    convert_dropcounters_counts,
    convert_uptime,
    convert_version,
    convert_system_memory,
    convert_lldp_table,
    convert_lldp_neighbors,
    convert_vlan_brief,
    convert_reboot_cause,
    convert_queue_counters,
    convert_queue_wredcounters,
    convert_interface_description,
    convert_interface_status,
    convert_interface_counters,
    convert_interface_errors,
    convert_portchannel,
    convert_ndp,
    convert_srv6_stats,
    convert_mac,
    convert_mac_aging_time,
    convert_mmu,
    convert_services,
    convert_processes,
    convert_watermark_telemetry,
    convert_interface_alias,
    convert_interface_flap,
    convert_interface_naming_mode,
    convert_interface_neighbor_expected,
    convert_interface_transceiver,
    convert_interface_fec_status,
    convert_interface_switchport,
    convert_interface_counters_detailed,
    convert_interface_counters_rates,
    convert_interface_counters_rif,
    convert_interface_counters_fec_stats,
    convert_interface_counters_fec_histogram,
    convert_interface_counters_trim,
    convert_ipv6_prefix_list,
    convert_ipv6_link_local_mode,
    convert_ipv6_route,
    convert_ipv6_bgp_summary,
    convert_ipv6_bgp_neighbors,
    convert_ipv6_interfaces,
    get_gnmi_path,
    list_supported_commands,
)

# CLI to gNMI mapping
from gnmi_cli_lib.mappings import CLI_TO_GNMI_MAPPING

# Test data
from gnmi_cli_lib.test_data import GNMI_JSON_TEST_DATA, EXPECTED_CLI_OUTPUTS

__all__ = [
    # Version
    "__version__",
    # Core classes
    "GnmiToCliConverter",
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
    # Convenience functions
    "convert",
    "convert_buffer_pool_watermark",
    "convert_headroom_pool_watermark",
    "convert_priority_group_watermark",
    "convert_queue_watermark",
    "convert_clock",
    "convert_clock_timezones",
    "convert_dropcounters_capabilities",
    "convert_dropcounters_configuration",
    "convert_dropcounters_counts",
    "convert_uptime",
    "convert_version",
    "convert_system_memory",
    "convert_lldp_table",
    "convert_lldp_neighbors",
    "convert_vlan_brief",
    "convert_reboot_cause",
    "convert_queue_counters",
    "convert_queue_wredcounters",
    "convert_interface_description",
    "convert_interface_status",
    "convert_interface_counters",
    "convert_interface_errors",
    "convert_portchannel",
    "convert_ndp",
    "convert_srv6_stats",
    "convert_mac",
    "convert_mac_aging_time",
    "convert_mmu",
    "convert_services",
    "convert_processes",
    "convert_watermark_telemetry",
    "convert_interface_alias",
    "convert_interface_flap",
    "convert_interface_naming_mode",
    "convert_interface_neighbor_expected",
    "convert_interface_transceiver",
    "convert_interface_fec_status",
    "convert_interface_switchport",
    "convert_interface_counters_detailed",
    "convert_interface_counters_rates",
    "convert_interface_counters_rif",
    "convert_interface_counters_fec_stats",
    "convert_interface_counters_fec_histogram",
    "convert_interface_counters_trim",
    "convert_ipv6_prefix_list",
    "convert_ipv6_link_local_mode",
    "convert_ipv6_route",
    "convert_ipv6_bgp_summary",
    "convert_ipv6_bgp_neighbors",
    "convert_ipv6_interfaces",
    "get_gnmi_path",
    "list_supported_commands",
    # Mappings
    "CLI_TO_GNMI_MAPPING",
    # Test data
    "GNMI_JSON_TEST_DATA",
    "EXPECTED_CLI_OUTPUTS",
]

