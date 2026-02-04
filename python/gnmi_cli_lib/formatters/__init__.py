"""
Output formatters module.

This module contains all output formatters organized by category:
- base: Abstract base class and generic formatters (table, text, list)
- watermark: Watermark-specific formatters
- interface: Interface command formatters
- system: System command formatters (version, uptime, memory, etc.)
- network: Network command formatters (LLDP, VLAN, MAC, NDP, etc.)
- queue: Queue and WRED counter formatters
- dropcounters: Drop counter formatters
"""

# Base formatters
from gnmi_cli_lib.formatters.base import (
    OutputFormatter,
    TableFormatter,
    TextFormatter,
    ListFormatter,
    CommandHelpFormatter,
)

# Watermark formatters
from gnmi_cli_lib.formatters.watermark import WatermarkTelemetryFormatter

# Interface formatters
from gnmi_cli_lib.formatters.interface import (
    InterfaceStatusFormatter,
    InterfaceDescriptionFormatter,
    InterfaceCountersFormatter,
    InterfaceErrorsFormatter,
    InterfaceAliasFormatter,
    InterfaceFlapFormatter,
    NamingModeFormatter,
    PortchannelFormatter,
)

# System formatters
from gnmi_cli_lib.formatters.system import (
    UptimeFormatter,
    VersionFormatter,
    SystemMemoryFormatter,
    ServicesFormatter,
    ProcessesFormatter,
    RebootCauseFormatter,
    MmuFormatter,
)

# Network formatters
from gnmi_cli_lib.formatters.network import (
    LldpTableFormatter,
    VlanBriefFormatter,
    NdpFormatter,
    MacFormatter,
    Srv6Formatter,
)

# Queue formatters
from gnmi_cli_lib.formatters.queue import (
    QueueCountersFormatter,
    WredCountersFormatter,
)

# Dropcounters formatters
from gnmi_cli_lib.formatters.dropcounters import (
    DropcountersCapabilitiesFormatter,
    DropcountersCountsFormatter,
    DropcountersConfigurationFormatter,
)

__all__ = [
    # Base
    "OutputFormatter",
    "TableFormatter",
    "TextFormatter",
    "ListFormatter",
    "CommandHelpFormatter",
    # Watermark
    "WatermarkTelemetryFormatter",
    # Interface
    "InterfaceStatusFormatter",
    "InterfaceDescriptionFormatter",
    "InterfaceCountersFormatter",
    "InterfaceErrorsFormatter",
    "InterfaceAliasFormatter",
    "InterfaceFlapFormatter",
    "NamingModeFormatter",
    "PortchannelFormatter",
    # System
    "UptimeFormatter",
    "VersionFormatter",
    "SystemMemoryFormatter",
    "ServicesFormatter",
    "ProcessesFormatter",
    "RebootCauseFormatter",
    "MmuFormatter",
    # Network
    "LldpTableFormatter",
    "VlanBriefFormatter",
    "NdpFormatter",
    "MacFormatter",
    "Srv6Formatter",
    # Queue
    "QueueCountersFormatter",
    "WredCountersFormatter",
    # Dropcounters
    "DropcountersCapabilitiesFormatter",
    "DropcountersCountsFormatter",
    "DropcountersConfigurationFormatter",
]

