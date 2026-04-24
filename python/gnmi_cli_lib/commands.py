"""
Command registration module.

This module contains the function to register all standard SONiC commands
with the command registry. It separates command definitions from the
registry logic for better maintainability.
"""

from gnmi_cli_lib.common.types import CommandConfig, CliMapping
from gnmi_cli_lib.registry import CommandRegistry


def register_all_commands(registry: CommandRegistry) -> None:
    """
    Register all standard SONiC commands with the registry.
    
    Args:
        registry: CommandRegistry instance to populate
    """
    _register_buffer_pool_commands(registry)
    _register_headroom_pool_commands(registry)
    _register_priority_group_commands(registry)
    _register_queue_commands(registry)
    _register_clock_commands(registry)
    _register_dropcounters_commands(registry)
    _register_system_commands(registry)
    _register_lldp_commands(registry)
    _register_vlan_commands(registry)
    _register_reboot_cause_commands(registry)
    _register_interface_commands(registry)
    _register_network_commands(registry)
    _register_ipv6_commands(registry)
    _register_intermediate_commands(registry)
    _register_cli_mappings(registry)


def _register_buffer_pool_commands(registry: CommandRegistry) -> None:
    """Register buffer pool commands."""
    for subcommand in ["watermark", "persistent-watermark"]:
        registry.register_command("buffer_pool", subcommand, CommandConfig(
            title="Shared pool maximum occupancy:",
            output_type="table",
            headers=["Pool", "Bytes"],
            key_field="pool_name",
            value_fields=["Bytes"],
            description=f"Buffer pool {subcommand.replace('-', ' ')}"
        ))


def _register_headroom_pool_commands(registry: CommandRegistry) -> None:
    """Register headroom pool commands."""
    for subcommand in ["watermark", "persistent-watermark"]:
        config = CommandConfig(
            title="Headroom pool maximum occupancy:",
            output_type="headroom_pool",
            headers=["Pool", "Bytes"],
            key_field="pool_name",
            value_fields=["Bytes"],
            description=f"Headroom pool {subcommand.replace('-', ' ')}"
        )
        # Use hyphen naming: headroom-pool (as per README)
        registry.register_command("headroom-pool", subcommand, config)


def _register_priority_group_commands(registry: CommandRegistry) -> None:
    """Register priority group commands."""
    pg_configs = [
        ("watermark-shared", "Ingress shared pool occupancy per PG:"),
        ("persistent-watermark-shared", "Ingress shared pool occupancy per PG:"),
        ("watermark-headroom", "Ingress headroom per PG:"),
        ("persistent-watermark-headroom", "Ingress headroom per PG:"),
    ]
    for subcommand, title in pg_configs:
        registry.register_command("priority-group", subcommand, CommandConfig(
            title=title,
            output_type="queue_watermark",
            headers=["Port"] + [f"PG{i}" for i in range(16)],
            key_field="port",
            dynamic_columns=True,
            description=f"Priority group {subcommand.replace('-', ' ')}"
        ))


def _register_queue_commands(registry: CommandRegistry) -> None:
    """Register queue commands."""
    # Watermark commands
    queue_configs = [
        ("watermark-unicast", "Egress shared pool occupancy per unicast queue:"),
        ("persistent-watermark-unicast", "Egress shared pool occupancy per unicast queue:"),
        ("watermark-multicast", "Egress shared pool occupancy per multicast queue:"),
        ("persistent-watermark-multicast", "Egress shared pool occupancy per multicast queue:"),
        ("watermark-all", "Egress shared pool occupancy per all queues:"),
        ("persistent-watermark-all", "Egress shared pool occupancy per all queues:"),
    ]
    for subcommand, title in queue_configs:
        registry.register_command("queue", subcommand, CommandConfig(
            title=title,
            output_type="queue_watermark",
            key_field="port",
            dynamic_columns=True,
            description=f"Queue {subcommand.replace('-', ' ')}"
        ))
    
    # NOTE: Self-referential help entries for ``show queue``,
    # ``show queue watermark`` and ``show queue persistent-watermark`` are
    # registered centrally in ``_register_intermediate_commands`` so that all
    # intermediate (non-leaf) commands live in one place.

    # Counter commands
    registry.register_command("queue", "counters", CommandConfig(
        output_type="queue_counters",
        description="Queue counters",
        optional_arg="interface_name"
    ))
    
    registry.register_command("queue", "wredcounters", CommandConfig(
        output_type="wred_counters",
        description="Queue WRED counters",
        optional_arg="interface_name"
    ))


def _register_clock_commands(registry: CommandRegistry) -> None:
    """Register clock commands."""
    # show clock (without subcommand - shows current time)
    registry.register_command("clock", "clock", CommandConfig(
        output_type="clock",
        key_field="date",
        description="Current date and time"
    ))
    
    registry.register_command("clock", "date", CommandConfig(
        output_type="clock",
        key_field="date",
        description="Current date and time"
    ))
    
    registry.register_command("clock", "timezones", CommandConfig(
        output_type="list",
        key_field="timezones",
        description="Available timezones"
    ))


def _register_dropcounters_commands(registry: CommandRegistry) -> None:
    """Register dropcounters commands."""
    registry.register_command("dropcounters", "capabilities", CommandConfig(
        output_type="dropcounters_capabilities",
        description="Drop counter capabilities by type"
    ))
    
    registry.register_command("dropcounters", "configuration", CommandConfig(
        title="",
        output_type="dropcounters_configuration",
        headers=["Counter", "Alias", "Group", "Type", "Reasons", "Description"],
        key_field="name",
        value_fields=["alias", "group", "type", "reason", "description"],
        description="Drop counter configuration"
    ))
    
    registry.register_command("dropcounters", "counts", CommandConfig(
        output_type="dropcounters_counts",
        description="Port drop counters"
    ))


def _register_system_commands(registry: CommandRegistry) -> None:
    """Register system commands."""
    registry.register_command("uptime", "uptime", CommandConfig(
        output_type="uptime",
        description="System uptime"
    ))
    
    registry.register_command("version", "version", CommandConfig(
        output_type="version",
        description="SONiC version information"
    ))
    
    # show system-memory (single command, no subcommand)
    registry.register_command("system-memory", "system-memory", CommandConfig(
        output_type="system_memory",
        description="System memory usage"
    ))
    
    registry.register_command("system-memory", "memory", CommandConfig(
        output_type="system_memory",
        description="System memory usage"
    ))
    
    registry.register_command("services", "services", CommandConfig(
        output_type="services",
        description="Docker services"
    ))
    
    # Processes commands - each uses a different formatter for its specific format
    registry.register_command("processes", "summary", CommandConfig(
        output_type="processes",
        description="Processes sorted by summary"
    ))
    registry.register_command("processes", "cpu", CommandConfig(
        output_type="processes_cpu",
        description="Processes sorted by cpu (top-like format)"
    ))
    registry.register_command("processes", "memory", CommandConfig(
        output_type="processes_memory",
        description="Processes sorted by memory (top-like format)"
    ))
    # NOTE: Self-referential ``(processes, processes)`` entry for bare
    # ``show processes`` is registered centrally in
    # ``_register_intermediate_commands``.

    # Watermark telemetry
    registry.register_command("watermark", "telemetry", CommandConfig(
        output_type="watermark_telemetry",
        description="Watermark telemetry interval"
    ))
    
    # MMU
    registry.register_command("mmu", "mmu", CommandConfig(
        output_type="mmu",
        description="MMU configuration"
    ))


def _register_lldp_commands(registry: CommandRegistry) -> None:
    """Register LLDP commands."""
    registry.register_command("lldp", "table", CommandConfig(
        output_type="lldp_table",
        description="LLDP neighbor table",
        optional_arg="interface_name"
    ))
    
    registry.register_command("lldp", "neighbors", CommandConfig(
        output_type="lldp_neighbors",
        description="LLDP neighbors detailed view",
        optional_arg="interface_name"
    ))


def _register_vlan_commands(registry: CommandRegistry) -> None:
    """Register VLAN commands."""
    registry.register_command("vlan", "brief", CommandConfig(
        output_type="vlan_brief",
        description="VLAN brief information"
    ))


def _register_reboot_cause_commands(registry: CommandRegistry) -> None:
    """Register reboot cause commands."""
    # Self-referential entry enables bare "show reboot-cause" (server path SHOW/reboot-cause)
    registry.register_command("reboot-cause", "reboot-cause", CommandConfig(
        output_type="reboot_cause",
        description="Reboot cause"
    ))

    registry.register_command("reboot-cause", "history", CommandConfig(
        output_type="reboot_cause",
        description="Reboot cause history"
    ))


def _register_interface_commands(registry: CommandRegistry) -> None:
    """Register interface commands."""
    registry.register_command("interfaces", "description", CommandConfig(
        output_type="interface_description",
        description="Interface descriptions",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "status", CommandConfig(
        output_type="interface_status",
        description="Interface status",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "counters", CommandConfig(
        output_type="interface_counters",
        description="Interface counters",
        optional_arg="interface_name"
    ))
    
    # Detailed counters - uses specialized per-line formatter
    registry.register_command("interfaces", "counters-detailed", CommandConfig(
        output_type="interface_counters_detailed",
        description="Interface counters detailed",
        optional_arg="interface_name"
    ))
    
    # Other counter variants - use standard table formatter
    for variant in ["counters-errors", "counters-rates",
                    "counters-rif", "counters-fec-stats",
                    "counters-trim"]:
        registry.register_command("interfaces", variant, CommandConfig(
            output_type="interface_counters",
            description=f"Interface {variant.replace('-', ' ')}",
            optional_arg="interface_name"
        ))
    
    # FEC histogram uses its own formatter
    registry.register_command("interfaces", "counters-fec-histogram", CommandConfig(
        output_type="fec_histogram",
        description="FEC histogram",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "errors", CommandConfig(
        output_type="interface_errors",
        description="Interface errors",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "alias", CommandConfig(
        output_type="interface_alias",
        description="Interface aliases",
        optional_arg="interface_name"
    ))
    
    # New naming with hyphen (as per README: show interfaces naming-mode)
    registry.register_command("interfaces", "naming-mode", CommandConfig(
        output_type="naming_mode",
        description="Interface naming mode"
    ))
    
    registry.register_command("interfaces", "flap", CommandConfig(
        output_type="interface_flap",
        description="Interface flap counts",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "portchannel", CommandConfig(
        output_type="portchannel",
        description="Port channel information"
    ))
    
    registry.register_command("interfaces", "neighbor-expected", CommandConfig(
        output_type="interface_neighbor",
        description="Interface neighbors expected"
    ))
    
    # Transceiver subcommands - use specialized formatters
    registry.register_command("interfaces", "transceiver-eeprom", CommandConfig(
        output_type="transceiver_eeprom",
        description="Transceiver EEPROM",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-error-status", CommandConfig(
        output_type="transceiver_error_status",
        description="Transceiver error status",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-info", CommandConfig(
        output_type="transceiver_info",
        description="Transceiver info",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-lpmode", CommandConfig(
        output_type="transceiver_lpmode",
        description="Transceiver low-power mode",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-pm", CommandConfig(
        output_type="transceiver_pm",
        description="Transceiver performance monitoring",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-presence", CommandConfig(
        output_type="transceiver_presence",
        description="Transceiver presence",
        optional_arg="interface_name"
    ))
    
    registry.register_command("interfaces", "transceiver-status", CommandConfig(
        output_type="transceiver_status",
        description="Transceiver status",
        optional_arg="interface_name"
    ))
    
    # FEC subcommands - use specialized formatter
    registry.register_command("interfaces", "fec-status", CommandConfig(
        output_type="fec_status",
        description="FEC status",
        optional_arg="interface_name"
    ))
    
    # Switchport subcommands
    registry.register_command("interfaces", "switchport-config", CommandConfig(
        output_type="switchport-config",
        description="Switchport config",
        optional_arg="interface_name"
    ))
    registry.register_command("interfaces", "switchport-status", CommandConfig(
        output_type="switchport-status",
        description="Switchport status",
        optional_arg="interface_name"
    ))


def _register_network_commands(registry: CommandRegistry) -> None:
    """Register network commands (NDP, SRv6, MAC)."""
    registry.register_command("ndp", "ndp", CommandConfig(
        output_type="ndp",
        description="IPv6 NDP table"
    ))
    
    registry.register_command("srv6", "stats", CommandConfig(
        output_type="srv6",
        description="SRv6 statistics",
        optional_arg="sid"
    ))
    
    registry.register_command("mac", "mac", CommandConfig(
        output_type="mac",
        description="MAC address table",
        optional_arg="address"
    ))
    
    # New naming with hyphen (as per README: show mac aging-time)
    registry.register_command("mac", "aging-time", CommandConfig(
        output_type="mac_aging_time",
        key_field="fdb_aging_time",
        description="MAC aging time"
    ))


def _register_ipv6_commands(registry: CommandRegistry) -> None:
    """Register IPv6 commands."""
    # IPv6 commands with dedicated formatters
    ipv6_commands = {
        "interfaces": "ipv6_interfaces",
        "route": "ipv6_route",
        "protocol": "ipv6_protocol",
        "bgp-neighbors": "ipv6_bgp_neighbors",
        "bgp-network": "ipv6_bgp_network",
        "bgp-summary": "ipv6_bgp_summary",
        "fib": "ipv6_fib",
    }
    
    for subcmd, output_type in ipv6_commands.items():
        optional_arg = "prefix" if subcmd == "route" else ""
        registry.register_command("ipv6", subcmd, CommandConfig(
            output_type=output_type,
            description=f"IPv6 {subcmd.replace('-', ' ')}",
            optional_arg=optional_arg
        ))
    
    # New naming with hyphen (as per README: show ipv6 prefix-list)
    registry.register_command("ipv6", "prefix-list", CommandConfig(
        output_type="ipv6_prefix_list",
        description="IPv6 prefix list",
        optional_arg="name"
    ))
    
    # New naming with hyphen (as per README: show ipv6 link-local-mode)
    registry.register_command("ipv6", "link-local-mode", CommandConfig(
        output_type="ipv6_link_local_mode",
        headers=["port", "mode"],
        description="IPv6 link local mode"
    ))


def _register_intermediate_commands(registry: CommandRegistry) -> None:
    """Register intermediate (non-leaf) show commands.

    Intermediate commands are path prefixes that have children but are
    themselves queryable via gNMI. On the real device, requesting the
    intermediate path returns a click-style "usage + subcommands" help
    payload. We register each one with the ``command_help`` formatter.

    Two shapes:
    * Single-segment self-refs (``(X, X)``): enables bare ``show X``.
    * Multi-segment ``(parent, child)``: enables ``show parent child``
      where ``child`` is itself an intermediate node.
    """
    # Single-segment intermediates: bare ``show <cmd>`` -> (cmd, cmd).
    single_segment = [
        "processes",
        "buffer_pool",
        "headroom-pool",
        "priority-group",
        "queue",
        "dropcounters",
        "watermark",
        "lldp",
        "vlan",
        "interfaces",
        "srv6",
        "ipv6",
    ]
    for command in single_segment:
        registry.register_command(command, command, CommandConfig(
            output_type="command_help",
            description=f"{command} help"
        ))

    # Multi-segment intermediates: (parent, child).
    multi_segment = [
        ("queue", "watermark"),
        ("queue", "persistent-watermark"),
        ("priority-group", "watermark"),
        ("priority-group", "persistent-watermark"),
        ("interfaces", "neighbor"),
        ("interfaces", "fec"),
        ("interfaces", "transceiver"),
        ("interfaces", "switchport"),
        ("ipv6", "bgp"),
    ]
    for command, subcommand in multi_segment:
        registry.register_command(command, subcommand, CommandConfig(
            output_type="command_help",
            description=f"{command} {subcommand} help"
        ))


def _register_cli_mappings(registry: CommandRegistry) -> None:
    """Register all CLI command mappings."""
    cli_mappings = [
        # Buffer pool (as per README: show buffer_pool watermark)
        CliMapping("show buffer_pool watermark", "SHOW/buffer_pool/watermark",
                   "buffer_pool", "watermark", "Buffer pool user watermarks"),
        CliMapping("show buffer_pool persistent-watermark", "SHOW/buffer_pool/persistent-watermark",
                   "buffer_pool", "persistent-watermark", "Buffer pool persistent watermarks"),
        
        # Headroom pool (as per README: show headroom-pool watermark)
        CliMapping("show headroom-pool watermark", "SHOW/headroom-pool/watermark",
                   "headroom-pool", "watermark", "Headroom pool user watermarks"),
        CliMapping("show headroom-pool persistent-watermark", "SHOW/headroom-pool/persistent-watermark",
                   "headroom-pool", "persistent-watermark", "Headroom pool persistent watermarks"),
        
        # Priority group
        CliMapping("show priority-group watermark shared", "SHOW/priority-group/watermark/shared",
                   "priority-group", "watermark-shared", "PG shared user watermarks"),
        CliMapping("show priority-group persistent-watermark shared", "SHOW/priority-group/persistent-watermark/shared",
                   "priority-group", "persistent-watermark-shared", "PG shared persistent watermarks"),
        CliMapping("show priority-group watermark headroom", "SHOW/priority-group/watermark/headroom",
                   "priority-group", "watermark-headroom", "PG headroom user watermarks"),
        CliMapping("show priority-group persistent-watermark headroom", "SHOW/priority-group/persistent-watermark/headroom",
                   "priority-group", "persistent-watermark-headroom", "PG headroom persistent watermarks"),
        
        # Queue watermarks
        CliMapping("show queue watermark unicast", "SHOW/queue/watermark/unicast",
                   "queue", "watermark-unicast", "Unicast queue user watermarks"),
        CliMapping("show queue persistent-watermark unicast", "SHOW/queue/persistent-watermark/unicast",
                   "queue", "persistent-watermark-unicast", "Unicast queue persistent watermarks"),
        CliMapping("show queue watermark multicast", "SHOW/queue/watermark/multicast",
                   "queue", "watermark-multicast", "Multicast queue user watermarks"),
        CliMapping("show queue persistent-watermark multicast", "SHOW/queue/persistent-watermark/multicast",
                   "queue", "persistent-watermark-multicast", "Multicast queue persistent watermarks"),
        CliMapping("show queue watermark all", "SHOW/queue/watermark/all",
                   "queue", "watermark-all", "All queue user watermarks"),
        CliMapping("show queue persistent-watermark all", "SHOW/queue/persistent-watermark/all",
                   "queue", "persistent-watermark-all", "All queue persistent watermarks"),
        
        # Clock
        CliMapping("show clock", "SHOW/clock", "clock", "date",
                   "Current date/time", "date"),
        CliMapping("show clock timezones", "SHOW/clock/timezones", "clock", "timezones",
                   "Available timezones", "timedatectl list-timezones"),
        
        # Dropcounters
        CliMapping("show dropcounters capabilities", "SHOW/dropcounters/capabilities",
                   "dropcounters", "capabilities",
                   "Drop counter capabilities", "dropconfig -c show_capabilities"),
        CliMapping("show dropcounters configuration", "SHOW/dropcounters/configuration",
                   "dropcounters", "configuration",
                   "Drop counter configuration", "dropconfig -c show_configuration"),
        CliMapping("show dropcounters counts", "SHOW/dropcounters/counts",
                   "dropcounters", "counts",
                   "Drop counter counts", "dropstat -c show"),
        
        # System commands
        CliMapping("show uptime", "SHOW/uptime", "uptime", "uptime",
                   "System uptime", "uptime -p"),
        CliMapping("show version", "SHOW/version", "version", "version",
                   "SONiC version", ""),
        CliMapping("show system-memory", "SHOW/system-memory", "system-memory", "memory",
                   "System memory", "free -m"),
        CliMapping("show services", "SHOW/services", "services", "services",
                   "Docker services", ""),
        CliMapping("show processes summary", "SHOW/processes/summary",
                   "processes", "summary", "Process summary", ""),
        CliMapping("show processes cpu", "SHOW/processes/cpu",
                   "processes", "cpu", "Processes by CPU", "top -bn 1 -o %CPU"),
        CliMapping("show processes memory", "SHOW/processes/memory",
                   "processes", "memory", "Processes by memory", "top -bn 1 -o %MEM"),
        CliMapping("show watermark telemetry interval", "SHOW/watermark/telemetry/interval",
                   "watermark", "telemetry", "Watermark telemetry interval", ""),
        CliMapping("show mmu", "SHOW/mmu", "mmu", "mmu", "MMU configuration", ""),
        
        # LLDP
        CliMapping("show lldp table", "SHOW/lldp/table", "lldp", "table",
                   "LLDP table", "lldpctl -f json"),
        CliMapping("show lldp neighbors", "SHOW/lldp/neighbors", "lldp", "neighbors",
                   "LLDP neighbors", "lldpctl -f json"),
        
        # VLAN
        CliMapping("show vlan brief", "SHOW/vlan/brief", "vlan", "brief",
                   "VLAN brief", ""),
        
        # Reboot cause
        CliMapping("show reboot-cause", "SHOW/reboot-cause", "reboot-cause", "reboot-cause",
                   "Reboot cause", ""),
        CliMapping("show reboot-cause history", "SHOW/reboot-cause/history",
                   "reboot-cause", "history", "Reboot cause history", ""),
        
        # Queue counters
        CliMapping("show queue counters", "SHOW/queue/counters", "queue", "counters",
                   "Queue counters", ""),
        CliMapping("show queue wredcounters", "SHOW/queue/wredcounters",
                   "queue", "wredcounters", "Queue WRED counters", ""),
        
        # Interface commands
        CliMapping("show interfaces description", "SHOW/interfaces/description",
                   "interfaces", "description", "Interface descriptions", ""),
        CliMapping("show interfaces status", "SHOW/interfaces/status",
                   "interfaces", "status", "Interface status", ""),
        CliMapping("show interfaces counters", "SHOW/interfaces/counters",
                   "interfaces", "counters", "Interface counters", ""),
        CliMapping("show interfaces counters detailed", "SHOW/interfaces/counters/detailed",
                   "interfaces", "counters-detailed", "Detailed interface counters", ""),
        CliMapping("show interfaces counters errors", "SHOW/interfaces/counters/errors",
                   "interfaces", "counters-errors", "Interface counter errors", ""),
        CliMapping("show interfaces counters rates", "SHOW/interfaces/counters/rates",
                   "interfaces", "counters-rates", "Interface counter rates", ""),
        CliMapping("show interfaces counters rif", "SHOW/interfaces/counters/rif",
                   "interfaces", "counters-rif", "RIF counters", ""),
        CliMapping("show interfaces counters fec-stats", "SHOW/interfaces/counters/fec-stats",
                   "interfaces", "counters-fec-stats", "FEC statistics", ""),
        CliMapping("show interfaces counters fec-histogram", "SHOW/interfaces/counters/fec-histogram",
                   "interfaces", "counters-fec-histogram", "FEC histogram", ""),
        CliMapping("show interfaces counters trim", "SHOW/interfaces/counters/trim",
                   "interfaces", "counters-trim", "Interface trim counters", ""),
        CliMapping("show interfaces errors", "SHOW/interfaces/errors",
                   "interfaces", "errors", "Interface errors", ""),
        CliMapping("show interfaces alias", "SHOW/interfaces/alias",
                   "interfaces", "alias", "Interface aliases", ""),
        CliMapping("show interfaces naming-mode", "SHOW/interfaces/naming-mode",
                   "interfaces", "naming-mode", "Interface naming mode", ""),
        CliMapping("show interfaces flap", "SHOW/interfaces/flap",
                   "interfaces", "flap", "Interface flap counts", ""),
        CliMapping("show interfaces portchannel", "SHOW/interfaces/portchannel",
                   "interfaces", "portchannel", "Port channel info", ""),
        CliMapping("show interfaces neighbor expected", "SHOW/interfaces/neighbor/expected",
                   "interfaces", "neighbor-expected", "Interface neighbors expected", ""),
        CliMapping("show interfaces fec status", "SHOW/interfaces/fec/status",
                   "interfaces", "fec-status", "FEC status", ""),
        
        # Transceiver commands
        CliMapping("show interfaces transceiver eeprom", "SHOW/interfaces/transceiver/eeprom",
                   "interfaces", "transceiver-eeprom", "Transceiver EEPROM", ""),
        CliMapping("show interfaces transceiver error-status", "SHOW/interfaces/transceiver/error-status",
                   "interfaces", "transceiver-error-status", "Transceiver error status", ""),
        CliMapping("show interfaces transceiver info", "SHOW/interfaces/transceiver/info",
                   "interfaces", "transceiver-info", "Transceiver info", ""),
        CliMapping("show interfaces transceiver lpmode", "SHOW/interfaces/transceiver/lpmode",
                   "interfaces", "transceiver-lpmode", "Transceiver LP mode", ""),
        CliMapping("show interfaces transceiver pm", "SHOW/interfaces/transceiver/pm",
                   "interfaces", "transceiver-pm", "Transceiver PM", ""),
        CliMapping("show interfaces transceiver presence", "SHOW/interfaces/transceiver/presence",
                   "interfaces", "transceiver-presence", "Transceiver presence", ""),
        CliMapping("show interfaces transceiver status", "SHOW/interfaces/transceiver/status",
                   "interfaces", "transceiver-status", "Transceiver status", ""),
        
        # Switchport commands
        CliMapping("show interfaces switchport config", "SHOW/interfaces/switchport/config",
                   "interfaces", "switchport-config", "Switchport config", ""),
        CliMapping("show interfaces switchport status", "SHOW/interfaces/switchport/status",
                   "interfaces", "switchport-status", "Switchport status", ""),
        
        # Network commands
        CliMapping("show ndp", "SHOW/ndp", "ndp", "ndp", "NDP table", "ip -6 neigh"),
        CliMapping("show srv6 stats", "SHOW/srv6/stats", "srv6", "stats",
                   "SRv6 statistics", ""),
        CliMapping("show mac", "SHOW/mac", "mac", "mac", "MAC table", ""),
        CliMapping("show mac aging-time", "SHOW/mac/aging-time", "mac", "aging-time",
                   "MAC aging time", ""),
        
        # IPv6 commands
        CliMapping("show ipv6 prefix-list", "SHOW/ipv6/prefix-list",
                   "ipv6", "prefix-list", "IPv6 prefix list",
                   "vtysh -c 'show ipv6 prefix-list'"),
        CliMapping("show ipv6 interfaces", "SHOW/ipv6/interfaces",
                   "ipv6", "interfaces", "IPv6 interfaces", ""),
        CliMapping("show ipv6 route", "SHOW/ipv6/route",
                   "ipv6", "route", "IPv6 routes",
                   "vtysh -c 'show ipv6 route json'"),
        CliMapping("show ipv6 protocol", "SHOW/ipv6/protocol",
                   "ipv6", "protocol", "IPv6 protocol",
                   "vtysh -c 'show ipv6 protocol'"),
        CliMapping("show ipv6 bgp neighbors", "SHOW/ipv6/bgp/neighbors",
                   "ipv6", "bgp-neighbors", "IPv6 BGP neighbors",
                   "vtysh -c 'show bgp ipv6 neighbors'"),
        CliMapping("show ipv6 bgp network", "SHOW/ipv6/bgp/network",
                   "ipv6", "bgp-network", "IPv6 BGP network",
                   "vtysh -c 'show bgp ipv6 network'"),
        CliMapping("show ipv6 bgp summary", "SHOW/ipv6/bgp/summary",
                   "ipv6", "bgp-summary", "IPv6 BGP summary",
                   "vtysh -c 'show bgp ipv6 summary json'"),
        CliMapping("show ipv6 link-local-mode", "SHOW/ipv6/link-local-mode",
                   "ipv6", "link-local-mode", "IPv6 link local mode", ""),
        CliMapping("show ipv6 fib", "SHOW/ipv6/fib", "ipv6", "fib",
                   "IPv6 FIB", ""),

        # ---- Intermediate (non-leaf) commands ----
        # Each returns a click-style "usage + subcommands" payload.
        # Single-segment intermediates (self-ref keys):
        CliMapping("show processes", "SHOW/processes",
                   "processes", "processes", "Processes help"),
        CliMapping("show buffer_pool", "SHOW/buffer_pool",
                   "buffer_pool", "buffer_pool", "Buffer pool help"),
        CliMapping("show headroom-pool", "SHOW/headroom-pool",
                   "headroom-pool", "headroom-pool", "Headroom pool help"),
        CliMapping("show priority-group", "SHOW/priority-group",
                   "priority-group", "priority-group",
                   "Priority group help"),
        CliMapping("show queue", "SHOW/queue", "queue", "queue",
                   "Queue help"),
        CliMapping("show dropcounters", "SHOW/dropcounters",
                   "dropcounters", "dropcounters", "Dropcounters help"),
        CliMapping("show watermark", "SHOW/watermark",
                   "watermark", "watermark", "Watermark help"),
        CliMapping("show lldp", "SHOW/lldp", "lldp", "lldp",
                   "LLDP help"),
        CliMapping("show vlan", "SHOW/vlan", "vlan", "vlan",
                   "VLAN help"),
        CliMapping("show interfaces", "SHOW/interfaces",
                   "interfaces", "interfaces", "Interfaces help"),
        CliMapping("show srv6", "SHOW/srv6", "srv6", "srv6",
                   "SRv6 help"),
        CliMapping("show ipv6", "SHOW/ipv6", "ipv6", "ipv6",
                   "IPv6 help"),
        # Multi-segment intermediates:
        CliMapping("show queue watermark",
                   "SHOW/queue/watermark",
                   "queue", "watermark",
                   "Queue watermark help"),
        CliMapping("show queue persistent-watermark",
                   "SHOW/queue/persistent-watermark",
                   "queue", "persistent-watermark",
                   "Queue persistent-watermark help"),
        CliMapping("show priority-group watermark",
                   "SHOW/priority-group/watermark",
                   "priority-group", "watermark",
                   "Priority group watermark help"),
        CliMapping("show priority-group persistent-watermark",
                   "SHOW/priority-group/persistent-watermark",
                   "priority-group", "persistent-watermark",
                   "Priority group persistent-watermark help"),
        CliMapping("show interfaces neighbor",
                   "SHOW/interfaces/neighbor",
                   "interfaces", "neighbor",
                   "Interfaces neighbor help"),
        CliMapping("show interfaces fec",
                   "SHOW/interfaces/fec",
                   "interfaces", "fec",
                   "Interfaces fec help"),
        CliMapping("show interfaces transceiver",
                   "SHOW/interfaces/transceiver",
                   "interfaces", "transceiver",
                   "Interfaces transceiver help"),
        CliMapping("show interfaces switchport",
                   "SHOW/interfaces/switchport",
                   "interfaces", "switchport",
                   "Interfaces switchport help"),
        CliMapping("show ipv6 bgp", "SHOW/ipv6/bgp",
                   "ipv6", "bgp", "IPv6 BGP help"),
    ]
    
    for mapping in cli_mappings:
        registry.register_cli_mapping(mapping)

