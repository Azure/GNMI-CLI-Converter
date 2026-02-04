"""
Command Registry - Central registry for all supported commands.

This module provides the CommandRegistry class which:
- Stores command configurations (CommandConfig)
- Stores CLI command mappings (CliMapping)
- Provides lookup methods for commands and formatters
- Centralizes the registration of new commands

The registry uses a singleton pattern for the default instance,
but custom registries can be created for testing or extensions.
"""

from typing import Dict, Optional

from gnmi_cli_lib.common.types import CommandConfig, CliMapping
from gnmi_cli_lib.formatters.base import (
    OutputFormatter,
    TableFormatter,
    TextFormatter,
    ListFormatter,
    CommandHelpFormatter,
)
from gnmi_cli_lib.formatters.watermark import (
    WatermarkTelemetryFormatter,
    QueueWatermarkFormatter,
    HeadroomPoolFormatter,
)
from gnmi_cli_lib.formatters.interface import (
    InterfaceStatusFormatter,
    InterfaceDescriptionFormatter,
    InterfaceCountersFormatter,
    InterfaceCountersDetailedFormatter,
    InterfaceErrorsFormatter,
    InterfaceAliasFormatter,
    InterfaceFlapFormatter,
    InterfaceNeighborFormatter,
    FecHistogramFormatter,
    NamingModeFormatter,
    PortchannelFormatter,
    SwitchportStatusFormatter,
    SwitchportConfigFormatter,
)
from gnmi_cli_lib.formatters.system import (
    ClockFormatter,
    UptimeFormatter,
    VersionFormatter,
    SystemMemoryFormatter,
    ServicesFormatter,
    ProcessesFormatter,
    ProcessesCpuFormatter,
    RebootCauseFormatter,
    MmuFormatter,
)
from gnmi_cli_lib.formatters.network import (
    LldpTableFormatter,
    LldpNeighborsFormatter,
    VlanBriefFormatter,
    NdpFormatter,
    MacFormatter,
    MacAgingTimeFormatter,
    Srv6Formatter,
)
from gnmi_cli_lib.formatters.queue import (
    QueueCountersFormatter,
    WredCountersFormatter,
)
from gnmi_cli_lib.formatters.dropcounters import (
    DropcountersCapabilitiesFormatter,
    DropcountersCountsFormatter,
    DropcountersConfigurationFormatter,
)
from gnmi_cli_lib.formatters.transceiver import (
    TransceiverPresenceFormatter,
    TransceiverErrorStatusFormatter,
    TransceiverEepromFormatter,
    TransceiverStatusFormatter,
    TransceiverInfoFormatter,
    TransceiverLpmodeFormatter,
    TransceiverPmFormatter,
    FecStatusFormatter,
)
from gnmi_cli_lib.formatters.ipv6 import (
    IPv6BgpNeighborsFormatter,
    IPv6BgpRoutesTableFormatter,
    IPv6BgpNetworkFormatter,
    IPv6BgpSummaryFormatter,
    IPv6FibFormatter,
    IPv6InterfacesFormatter,
    IPv6LinkLocalModeFormatter,
    IPv6PrefixListFormatter,
    IPv6ProtocolFormatter,
    IPv6RouteFormatter,
)


class CommandRegistry:
    """
    Central registry for all supported commands.
    
    This class provides a clean way to register and look up command
    configurations and CLI mappings. It follows the Registry Pattern
    to decouple command definitions from the converter logic.
    
    Attributes:
        _configs: Dict mapping command -> subcommand -> CommandConfig
        _cli_mappings: Dict mapping CLI command string -> CliMapping
        _formatters: Dict mapping output_type -> OutputFormatter instance
    
    Example:
        >>> registry = CommandRegistry()
        >>> registry.register_command("interfaces", "status", CommandConfig(
        ...     output_type="interface_status",
        ...     description="Interface status"
        ... ))
        >>> config = registry.get_config("interfaces", "status")
        >>> formatter = registry.get_formatter(config.output_type)
    """
    
    def __init__(self):
        """Initialize an empty registry with default formatters."""
        self._configs: Dict[str, Dict[str, CommandConfig]] = {}
        self._cli_mappings: Dict[str, CliMapping] = {}
        self._formatters: Dict[str, OutputFormatter] = self._create_default_formatters()
    
    def _create_default_formatters(self) -> Dict[str, OutputFormatter]:
        """Create the default set of formatters."""
        return {
            # Base formatters
            "table": TableFormatter(),
            "text": TextFormatter(),
            "list": ListFormatter(),
            "command_help": CommandHelpFormatter(),
            # Dropcounters formatters
            "dropcounters_capabilities": DropcountersCapabilitiesFormatter(),
            "dropcounters_counts": DropcountersCountsFormatter(),
            "dropcounters_configuration": DropcountersConfigurationFormatter(),
            # System formatters
            "clock": ClockFormatter(),
            "uptime": UptimeFormatter(),
            "version": VersionFormatter(),
            "system_memory": SystemMemoryFormatter(),
            "services": ServicesFormatter(),
            "processes": ProcessesFormatter(),
            "processes_cpu": ProcessesCpuFormatter(),
            "processes_memory": ProcessesCpuFormatter(),  # Same top-like format
            "reboot_cause": RebootCauseFormatter(),
            "mmu": MmuFormatter(),
            # Network formatters
            "lldp_table": LldpTableFormatter(),
            "lldp_neighbors": LldpNeighborsFormatter(),
            "vlan_brief": VlanBriefFormatter(),
            "ndp": NdpFormatter(),
            "mac": MacFormatter(),
            "mac_aging_time": MacAgingTimeFormatter(),
            "srv6": Srv6Formatter(),
            # Queue formatters
            "queue_counters": QueueCountersFormatter(),
            "wred_counters": WredCountersFormatter(),
            # Interface formatters
            "interface_description": InterfaceDescriptionFormatter(),
            "interface_status": InterfaceStatusFormatter(),
            "interface_counters": InterfaceCountersFormatter(),
            "interface_counters_detailed": InterfaceCountersDetailedFormatter(),
            "interface_errors": InterfaceErrorsFormatter(),
            "interface_alias": InterfaceAliasFormatter(),
            "interface_flap": InterfaceFlapFormatter(),
            "interface_neighbor": InterfaceNeighborFormatter(),
            "fec_histogram": FecHistogramFormatter(),
            "naming_mode": NamingModeFormatter(),
            "portchannel": PortchannelFormatter(),
            "switchport-status": SwitchportStatusFormatter(),
            "switchport-config": SwitchportConfigFormatter(),
            # Watermark formatters
            "watermark_telemetry": WatermarkTelemetryFormatter(),
            "queue_watermark": QueueWatermarkFormatter(),
            "headroom_pool": HeadroomPoolFormatter(),
            # Transceiver formatters
            "transceiver_presence": TransceiverPresenceFormatter(),
            "transceiver_error_status": TransceiverErrorStatusFormatter(),
            "transceiver_eeprom": TransceiverEepromFormatter(),
            "transceiver_status": TransceiverStatusFormatter(),
            "transceiver_info": TransceiverInfoFormatter(),
            "transceiver_lpmode": TransceiverLpmodeFormatter(),
            "transceiver_pm": TransceiverPmFormatter(),
            "fec_status": FecStatusFormatter(),
            # IPv6 formatters
            "ipv6_bgp_neighbors": IPv6BgpNeighborsFormatter(),
            "ipv6_bgp_routes_table": IPv6BgpRoutesTableFormatter(),
            "ipv6_bgp_network": IPv6BgpNetworkFormatter(),
            "ipv6_bgp_summary": IPv6BgpSummaryFormatter(),
            "ipv6_fib": IPv6FibFormatter(),
            "ipv6_interfaces": IPv6InterfacesFormatter(),
            "ipv6_link_local_mode": IPv6LinkLocalModeFormatter(),
            "ipv6_prefix_list": IPv6PrefixListFormatter(),
            "ipv6_protocol": IPv6ProtocolFormatter(),
            "ipv6_route": IPv6RouteFormatter(),
        }
    
    def register_command(self, command: str, subcommand: str,
                         config: CommandConfig) -> None:
        """
        Register a command configuration.
        
        Args:
            command: Command category (e.g., "interfaces", "queue")
            subcommand: Specific subcommand (e.g., "status", "counters")
            config: CommandConfig instance with formatting options
        """
        if command not in self._configs:
            self._configs[command] = {}
        self._configs[command][subcommand] = config
    
    def register_cli_mapping(self, mapping: CliMapping) -> None:
        """
        Register a CLI command mapping.
        
        Args:
            mapping: CliMapping instance linking CLI command to internal command
        """
        self._cli_mappings[mapping.cli_command] = mapping
    
    def register_formatter(self, name: str, formatter: OutputFormatter) -> None:
        """
        Register a custom formatter.
        
        Args:
            name: Formatter name (used in CommandConfig.output_type)
            formatter: OutputFormatter instance
        """
        self._formatters[name] = formatter
    
    def get_config(self, command: str, subcommand: str) -> CommandConfig:
        """
        Get configuration for a command/subcommand pair.
        
        Args:
            command: Command category
            subcommand: Specific subcommand
        
        Returns:
            CommandConfig for the command
        
        Raises:
            ValueError: If command or subcommand is not registered
        """
        if command not in self._configs:
            supported = list(self._configs.keys())
            raise ValueError(f"Unsupported command: {command}. Supported: {supported}")
        
        if subcommand not in self._configs[command]:
            supported = list(self._configs[command].keys())
            raise ValueError(
                f"Unsupported subcommand '{subcommand}' for command '{command}'. "
                f"Supported: {supported}"
            )
        
        return self._configs[command][subcommand]
    
    def get_cli_mapping(self, cli_command: str) -> CliMapping:
        """
        Get mapping for a CLI command string.
        
        Args:
            cli_command: CLI command string (e.g., "show interfaces status")
        
        Returns:
            CliMapping for the command
        
        Raises:
            ValueError: If CLI command is not registered
        """
        if cli_command not in self._cli_mappings:
            raise ValueError(f"Unknown CLI command: {cli_command}")
        return self._cli_mappings[cli_command]
    
    def get_formatter(self, output_type: str) -> OutputFormatter:
        """
        Get the formatter for an output type.
        
        Args:
            output_type: Formatter name (from CommandConfig.output_type)
        
        Returns:
            OutputFormatter instance
        
        Raises:
            ValueError: If output type is not registered
        """
        if output_type not in self._formatters:
            raise ValueError(f"Unknown output type: {output_type}")
        return self._formatters[output_type]
    
    def list_commands(self) -> Dict[str, Dict[str, CommandConfig]]:
        """
        Return all registered commands.
        
        Returns:
            Dict mapping command -> subcommand -> CommandConfig
        """
        return self._configs.copy()
    
    def list_cli_mappings(self) -> Dict[str, CliMapping]:
        """
        Return all CLI mappings.
        
        Returns:
            Dict mapping CLI command string -> CliMapping
        """
        return self._cli_mappings.copy()
    
    def list_formatters(self) -> Dict[str, OutputFormatter]:
        """
        Return all registered formatters.
        
        Returns:
            Dict mapping formatter name -> OutputFormatter
        """
        return self._formatters.copy()
    
    def get_all_commands(self) -> list:
        """
        Return a list of all registered command keys.
        
        Returns:
            List of (command, subcommand) tuples
        """
        result = []
        for cmd, subcmds in self._configs.items():
            for subcmd in subcmds:
                result.append((cmd, subcmd))
        return result
    
    def has_command(self, command: str, subcommand: str) -> bool:
        """Check if a command/subcommand is registered."""
        return command in self._configs and subcommand in self._configs[command]
    
    def has_cli_mapping(self, cli_command: str) -> bool:
        """Check if a CLI command is registered."""
        return cli_command in self._cli_mappings


# =============================================================================
# Default Registry Instance
# =============================================================================

_DEFAULT_REGISTRY: Optional[CommandRegistry] = None


def get_default_registry() -> CommandRegistry:
    """
    Get the default command registry (singleton).
    
    The default registry is lazily initialized on first access
    and includes all standard SONiC commands.
    
    Returns:
        CommandRegistry instance with all standard commands
    """
    global _DEFAULT_REGISTRY
    if _DEFAULT_REGISTRY is None:
        _DEFAULT_REGISTRY = _create_default_registry()
    return _DEFAULT_REGISTRY


def _create_default_registry() -> CommandRegistry:
    """Create and populate the default command registry."""
    registry = CommandRegistry()
    
    # Import and register all commands
    from gnmi_cli_lib.commands import register_all_commands
    register_all_commands(registry)
    
    return registry

