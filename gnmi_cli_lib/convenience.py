"""
Convenience functions for quick conversion.

This module provides simple wrapper functions for common conversion tasks.
These functions use the default registry and a shared converter instance.
"""

from typing import Dict, Union, Optional

from gnmi_cli_lib.converter import GnmiToCliConverter
from gnmi_cli_lib.registry import get_default_registry


# Shared converter instance
_converter: Optional[GnmiToCliConverter] = None


def _get_converter() -> GnmiToCliConverter:
    """Get the shared converter instance."""
    global _converter
    if _converter is None:
        _converter = GnmiToCliConverter()
    return _converter


def convert(command: str, subcommand: str, json_data: Union[Dict, str]) -> str:
    """
    Quick conversion function.
    
    Args:
        command: Command type
        subcommand: Subcommand
        json_data: JSON data
    
    Returns:
        CLI-formatted string
    """
    return _get_converter().convert(command, subcommand, json_data)


def convert_buffer_pool_watermark(json_data: Union[Dict, str],
                                   persistent: bool = False) -> str:
    """Convert buffer pool watermark (show buffer_pool watermark)."""
    subcommand = "persistent-watermark" if persistent else "watermark"
    return convert("buffer_pool", subcommand, json_data)


def convert_headroom_pool_watermark(json_data: Union[Dict, str],
                                     persistent: bool = False) -> str:
    """Convert headroom pool watermark (show headroom-pool watermark)."""
    subcommand = "persistent-watermark" if persistent else "watermark"
    return convert("headroom-pool", subcommand, json_data)


def convert_priority_group_watermark(json_data: Union[Dict, str],
                                      wm_type: str = "shared",
                                      persistent: bool = False) -> str:
    """Convert priority group watermark (show priority-group watermark)."""
    prefix = "persistent-watermark" if persistent else "watermark"
    subcommand = f"{prefix}-{wm_type}"
    return convert("priority-group", subcommand, json_data)


def convert_queue_watermark(json_data: Union[Dict, str],
                            queue_type: str = "unicast",
                            persistent: bool = False) -> str:
    """Convert queue watermark (show queue watermark)."""
    prefix = "persistent-watermark" if persistent else "watermark"
    subcommand = f"{prefix}-{queue_type}"
    return convert("queue", subcommand, json_data)


def convert_clock(json_data: Union[Dict, str]) -> str:
    """Convert clock date (show clock)."""
    return convert("clock", "date", json_data)


def convert_clock_timezones(json_data: Union[Dict, str]) -> str:
    """Convert clock timezones (show clock timezones)."""
    return convert("clock", "timezones", json_data)


def convert_dropcounters_capabilities(json_data: Union[Dict, str]) -> str:
    """Convert dropcounters capabilities (show dropcounters capabilities)."""
    return convert("dropcounters", "capabilities", json_data)


def convert_dropcounters_configuration(json_data: Union[Dict, str]) -> str:
    """Convert dropcounters configuration (show dropcounters configuration)."""
    return convert("dropcounters", "configuration", json_data)


def convert_dropcounters_counts(json_data: Union[Dict, str]) -> str:
    """Convert dropcounters counts (show dropcounters counts)."""
    return convert("dropcounters", "counts", json_data)


def convert_uptime(json_data: Union[Dict, str]) -> str:
    """Convert uptime (show uptime)."""
    return convert("uptime", "uptime", json_data)


def convert_version(json_data: Union[Dict, str]) -> str:
    """Convert version (show version)."""
    return convert("version", "version", json_data)


def convert_system_memory(json_data: Union[Dict, str]) -> str:
    """Convert system memory (show system-memory)."""
    return convert("system-memory", "memory", json_data)


def convert_lldp_table(json_data: Union[Dict, str]) -> str:
    """Convert LLDP table (show lldp table)."""
    return convert("lldp", "table", json_data)


def convert_lldp_neighbors(json_data: Union[Dict, str]) -> str:
    """Convert LLDP neighbors (show lldp neighbors)."""
    return convert("lldp", "neighbors", json_data)


def convert_vlan_brief(json_data: Union[Dict, str]) -> str:
    """Convert VLAN brief (show vlan brief)."""
    return convert("vlan", "brief", json_data)


def convert_reboot_cause(json_data: Union[Dict, str], history: bool = False) -> str:
    """Convert reboot cause (show reboot-cause [history])."""
    subcommand = "history" if history else "current"
    return convert("reboot-cause", subcommand, json_data)


def convert_queue_counters(json_data: Union[Dict, str]) -> str:
    """Convert queue counters (show queue counters)."""
    return convert("queue", "counters", json_data)


def convert_queue_wredcounters(json_data: Union[Dict, str]) -> str:
    """Convert queue WRED counters (show queue wredcounters)."""
    return convert("queue", "wredcounters", json_data)


def convert_interface_description(json_data: Union[Dict, str]) -> str:
    """Convert interface description (show interfaces description)."""
    return convert("interfaces", "description", json_data)


def convert_interface_status(json_data: Union[Dict, str]) -> str:
    """Convert interface status (show interfaces status)."""
    return convert("interfaces", "status", json_data)


def convert_interface_counters(json_data: Union[Dict, str]) -> str:
    """Convert interface counters (show interfaces counters)."""
    return convert("interfaces", "counters", json_data)


def convert_interface_errors(json_data: Union[Dict, str]) -> str:
    """Convert interface errors (show interfaces errors)."""
    return convert("interfaces", "errors", json_data)


def convert_portchannel(json_data: Union[Dict, str]) -> str:
    """Convert portchannel (show interfaces portchannel)."""
    return convert("interfaces", "portchannel", json_data)


def convert_ndp(json_data: Union[Dict, str]) -> str:
    """Convert NDP (show ndp)."""
    return convert("ndp", "ndp", json_data)


def convert_srv6_stats(json_data: Union[Dict, str]) -> str:
    """Convert SRv6 stats (show srv6 stats)."""
    return convert("srv6", "stats", json_data)


def convert_mac(json_data: Union[Dict, str]) -> str:
    """Convert MAC table (show mac)."""
    return convert("mac", "mac", json_data)


def convert_mac_aging_time(json_data: Union[Dict, str]) -> str:
    """Convert MAC aging time (show mac aging-time)."""
    return convert("mac", "aging-time", json_data)


def convert_mmu(json_data: Union[Dict, str]) -> str:
    """Convert MMU config (show mmu)."""
    return convert("mmu", "mmu", json_data)


def convert_services(json_data: Union[Dict, str]) -> str:
    """Convert services (show services)."""
    return convert("services", "services", json_data)


def convert_processes(json_data: Union[Dict, str], sort_by: str = "summary") -> str:
    """Convert processes (show processes [summary|cpu|memory])."""
    return convert("processes", sort_by, json_data)


def convert_watermark_telemetry(json_data: Union[Dict, str]) -> str:
    """Convert watermark telemetry interval (show watermark telemetry interval)."""
    return convert("watermark", "telemetry", json_data)


def convert_interface_alias(json_data: Union[Dict, str]) -> str:
    """Convert interface alias (show interfaces alias)."""
    return convert("interfaces", "alias", json_data)


def convert_interface_flap(json_data: Union[Dict, str]) -> str:
    """Convert interface flap (show interfaces flap)."""
    return convert("interfaces", "flap", json_data)


def convert_interface_naming_mode(json_data: Union[Dict, str]) -> str:
    """Convert interface naming mode (show interfaces naming-mode)."""
    return convert("interfaces", "naming-mode", json_data)


def convert_interface_neighbor_expected(json_data: Union[Dict, str]) -> str:
    """Convert interface neighbor expected (show interfaces neighbor expected)."""
    return convert("interfaces", "neighbor-expected", json_data)


def convert_interface_transceiver(json_data: Union[Dict, str],
                                   subcmd: str = "info") -> str:
    """Convert interface transceiver (show interfaces transceiver [subcmd]).
    
    Args:
        json_data: JSON data
        subcmd: Subcommand (eeprom, error-status, info, lpmode, pm, presence, status)
    """
    return convert("interfaces", f"transceiver-{subcmd}", json_data)


def convert_interface_fec_status(json_data: Union[Dict, str]) -> str:
    """Convert interface FEC status (show interfaces fec status)."""
    return convert("interfaces", "fec-status", json_data)


def convert_interface_switchport(json_data: Union[Dict, str],
                                  config_or_status: str = "status") -> str:
    """Convert interface switchport (show interfaces switchport config|status).
    
    Args:
        json_data: JSON data
        config_or_status: Either "config" or "status"
    """
    return convert("interfaces", f"switchport-{config_or_status}", json_data)


def convert_interface_counters_detailed(json_data: Union[Dict, str]) -> str:
    """Convert interface counters detailed (show interfaces counters detailed)."""
    return convert("interfaces", "counters-detailed", json_data)


def convert_interface_counters_rates(json_data: Union[Dict, str]) -> str:
    """Convert interface counters rates (show interfaces counters rates)."""
    return convert("interfaces", "counters-rates", json_data)


def convert_interface_counters_rif(json_data: Union[Dict, str]) -> str:
    """Convert interface counters rif (show interfaces counters rif)."""
    return convert("interfaces", "counters-rif", json_data)


def convert_interface_counters_fec_stats(json_data: Union[Dict, str]) -> str:
    """Convert interface counters fec-stats (show interfaces counters fec-stats)."""
    return convert("interfaces", "counters-fec-stats", json_data)


def convert_interface_counters_fec_histogram(json_data: Union[Dict, str]) -> str:
    """Convert interface counters fec-histogram (show interfaces counters fec-histogram)."""
    return convert("interfaces", "counters-fec-histogram", json_data)


def convert_interface_counters_trim(json_data: Union[Dict, str]) -> str:
    """Convert interface counters trim (show interfaces counters trim)."""
    return convert("interfaces", "counters-trim", json_data)


def convert_ipv6_prefix_list(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 prefix list (show ipv6 prefix-list)."""
    return convert("ipv6", "prefix-list", json_data)


def convert_ipv6_link_local_mode(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 link local mode (show ipv6 link-local-mode)."""
    return convert("ipv6", "link-local-mode", json_data)


def convert_ipv6_route(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 route (show ipv6 route)."""
    return convert("ipv6", "route", json_data)


def convert_ipv6_bgp_summary(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 BGP summary (show ipv6 bgp summary)."""
    return convert("ipv6", "bgp-summary", json_data)


def convert_ipv6_bgp_neighbors(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 BGP neighbors (show ipv6 bgp neighbors)."""
    return convert("ipv6", "bgp-neighbors", json_data)


def convert_ipv6_interfaces(json_data: Union[Dict, str]) -> str:
    """Convert IPv6 interfaces (show ipv6 interfaces)."""
    return convert("ipv6", "interfaces", json_data)


def get_gnmi_path(cli_command: str) -> str:
    """Get the gNMI path for a CLI command."""
    registry = get_default_registry()
    mapping = registry.get_cli_mapping(cli_command)
    return mapping.gnmi_path


def list_supported_commands() -> Dict:
    """Return CLI to gNMI mapping dictionary."""
    registry = get_default_registry()
    return {
        cmd: {
            "gnmi_path": mapping.gnmi_path,
            "command": mapping.command,
            "subcommand": mapping.subcommand,
            "description": mapping.description,
            "out_of_process_cmd": mapping.out_of_process_cmd,
        }
        for cmd, mapping in registry.list_cli_mappings().items()
    }

