//! Command registration module.
//!
//! This module contains all standard SONiC command definitions and their
//! registration with the command registry.

use crate::common::types::{CliMapping, CommandConfig};
use crate::registry::CommandRegistry;

/// Register all standard SONiC commands with the registry.
pub fn register_all_commands(registry: &mut CommandRegistry) {
    register_buffer_pool_commands(registry);
    register_headroom_pool_commands(registry);
    register_priority_group_commands(registry);
    register_queue_commands(registry);
    register_clock_commands(registry);
    register_dropcounters_commands(registry);
    register_system_commands(registry);
    register_lldp_commands(registry);
    register_vlan_commands(registry);
    register_reboot_cause_commands(registry);
    register_interface_commands(registry);
    register_network_commands(registry);
    register_ipv6_commands(registry);
    register_intermediate_commands(registry);
    register_cli_mappings(registry);
}

/// Register buffer pool commands.
fn register_buffer_pool_commands(registry: &mut CommandRegistry) {
    for subcommand in ["watermark", "persistent-watermark"] {
        let description = format!("Buffer pool {}", subcommand.replace('-', " "));
        let name = format!("buffer_pool_{}", subcommand.replace('-', "_"));

        let config = CommandConfig::builder()
            .name(&name)
            .category("buffer_pool")
            .description(&description)
            .cli_command(&format!("show buffer_pool {}", subcommand))
            .formatter("buffer_pool")
            .title("Shared pool maximum occupancy:")
            .build();

        registry.register(name.clone(), config.clone());
        registry.register(name.replace("buffer_pool", "buffer-pool"), config);
    }
}

/// Register headroom pool commands.
fn register_headroom_pool_commands(registry: &mut CommandRegistry) {
    for subcommand in ["watermark", "persistent-watermark"] {
        let description = format!("Headroom pool {}", subcommand.replace('-', " "));
        let name = format!("headroom_pool_{}", subcommand.replace('-', "_"));

        let config = CommandConfig::builder()
            .name(&name)
            .category("headroom-pool")
            .description(&description)
            .cli_command(&format!("show headroom-pool {}", subcommand))
            .formatter("headroom_pool")
            .title("Headroom pool maximum occupancy:")
            .build();

        registry.register(name.clone(), config.clone());
        registry.register(name.replace("headroom_pool", "headroom-pool"), config);
    }
}

/// Register priority group commands.
fn register_priority_group_commands(registry: &mut CommandRegistry) {
    let pg_configs = [
        ("watermark-shared", "Ingress shared pool occupancy per PG:"),
        (
            "persistent-watermark-shared",
            "Ingress shared pool occupancy per PG:",
        ),
        ("watermark-headroom", "Ingress headroom per PG:"),
        ("persistent-watermark-headroom", "Ingress headroom per PG:"),
    ];

    for (subcommand, title) in pg_configs {
        let description = format!("Priority group {}", subcommand.replace('-', " "));
        let name = format!("priority_group_{}", subcommand.replace('-', "_"));

        let config = CommandConfig::builder()
            .name(&name)
            .category("priority-group")
            .description(&description)
            .cli_command(&format!("show priority-group {}", subcommand))
            .formatter("queue_watermark")
            .title(title)
            .build();

        registry.register(name.clone(), config.clone());
        registry.register(name.replace("priority_group", "priority-group"), config);
    }
}

/// Register queue commands.
fn register_queue_commands(registry: &mut CommandRegistry) {
    // Watermark commands
    let queue_configs = [
        (
            "watermark-unicast",
            "Egress shared pool occupancy per unicast queue:",
        ),
        (
            "persistent-watermark-unicast",
            "Egress shared pool occupancy per unicast queue:",
        ),
        (
            "watermark-multicast",
            "Egress shared pool occupancy per multicast queue:",
        ),
        (
            "persistent-watermark-multicast",
            "Egress shared pool occupancy per multicast queue:",
        ),
        (
            "watermark-all",
            "Egress shared pool occupancy per all queues:",
        ),
        (
            "persistent-watermark-all",
            "Egress shared pool occupancy per all queues:",
        ),
    ];

    for (subcommand, title) in queue_configs {
        let description = format!("Queue {}", subcommand.replace('-', " "));
        let name = format!("queue_{}", subcommand.replace('-', "_"));

        let config = CommandConfig::builder()
            .name(&name)
            .category("queue")
            .description(&description)
            .cli_command(&format!("show queue {}", subcommand))
            .formatter("queue_watermark")
            .title(title)
            .build();

        registry.register(name, config);
    }

    // NOTE: Self-referential help entries for ``show queue``,
    // ``show queue watermark`` and ``show queue persistent-watermark`` are
    // registered centrally in ``register_intermediate_commands`` so that all
    // intermediate (non-leaf) commands live in one place.

    // Counter commands
    registry.register(
        "queue_counters".to_string(),
        CommandConfig::builder()
            .name("queue_counters")
            .category("queue")
            .description("Queue counters")
            .cli_command("show queue counters")
            .formatter("queue_counters")
            .build(),
    );

    registry.register(
        "queue_wredcounters".to_string(),
        CommandConfig::builder()
            .name("queue_wredcounters")
            .category("queue")
            .description("Queue WRED counters")
            .cli_command("show queue wredcounters")
            .formatter("wred_counters")
            .build(),
    );
}

/// Register clock commands.
fn register_clock_commands(registry: &mut CommandRegistry) {
    let clock_config = CommandConfig::builder()
        .name("clock")
        .category("clock")
        .description("Current date and time")
        .cli_command("show clock")
        .formatter("clock")
        .build();

    registry.register("clock".to_string(), clock_config.clone());
    registry.register("clock_clock".to_string(), clock_config);

    registry.register(
        "clock_date".to_string(),
        CommandConfig::builder()
            .name("clock_date")
            .category("clock")
            .description("Current date and time")
            .cli_command("show clock")
            .formatter("clock")
            .build(),
    );

    registry.register(
        "clock_timezones".to_string(),
        CommandConfig::builder()
            .name("clock_timezones")
            .category("clock")
            .description("Available timezones")
            .cli_command("show clock timezones")
            .formatter("list")
            .key_field("timezones")
            .build(),
    );
}

/// Register dropcounters commands.
fn register_dropcounters_commands(registry: &mut CommandRegistry) {
    registry.register(
        "dropcounters_capabilities".to_string(),
        CommandConfig::builder()
            .name("dropcounters_capabilities")
            .category("dropcounters")
            .description("Drop counter capabilities by type")
            .cli_command("show dropcounters capabilities")
            .formatter("dropcounters_capabilities")
            .build(),
    );

    registry.register(
        "dropcounters_configuration".to_string(),
        CommandConfig::builder()
            .name("dropcounters_configuration")
            .category("dropcounters")
            .description("Drop counter configuration")
            .cli_command("show dropcounters configuration")
            .formatter("dropcounters_configuration")
            .column_order(&[
                "Counter",
                "Alias",
                "Group",
                "Type",
                "Reasons",
                "Description",
            ])
            .build(),
    );

    registry.register(
        "dropcounters_counts".to_string(),
        CommandConfig::builder()
            .name("dropcounters_counts")
            .category("dropcounters")
            .description("Port drop counters")
            .cli_command("show dropcounters counts")
            .formatter("dropcounters_counts")
            .build(),
    );
}

/// Register system commands.
fn register_system_commands(registry: &mut CommandRegistry) {
    let uptime_config = CommandConfig::builder()
        .name("uptime")
        .category("system")
        .description("System uptime")
        .cli_command("show uptime")
        .formatter("uptime")
        .build();

    registry.register("uptime".to_string(), uptime_config.clone());
    registry.register("uptime_uptime".to_string(), uptime_config);

    registry.register(
        "version".to_string(),
        CommandConfig::builder()
            .name("version")
            .category("system")
            .description("SONiC version information")
            .cli_command("show version")
            .formatter("version")
            .build(),
    );
    registry.register(
        "version_version".to_string(),
        CommandConfig::builder()
            .name("version")
            .category("system")
            .description("SONiC version information")
            .cli_command("show version")
            .formatter("version")
            .build(),
    );

    let sys_mem_config = CommandConfig::builder()
        .name("system_memory")
        .category("system")
        .description("System memory usage")
        .cli_command("show system-memory")
        .formatter("system_memory")
        .build();

    registry.register("system_memory".to_string(), sys_mem_config.clone());
    registry.register(
        "system_memory_system_memory".to_string(),
        sys_mem_config.clone(),
    );
    registry.register("system-memory_system_memory".to_string(), sys_mem_config);

    let services_config = CommandConfig::builder()
        .name("services")
        .category("system")
        .description("Docker services")
        .cli_command("show services")
        .formatter("services")
        .build();

    registry.register("services".to_string(), services_config.clone());
    registry.register("services_services".to_string(), services_config);

    // Processes commands
    registry.register(
        "processes_summary".to_string(),
        CommandConfig::builder()
            .name("processes_summary")
            .category("processes")
            .description("Processes sorted by summary")
            .cli_command("show processes summary")
            .formatter("processes")
            .build(),
    );

    registry.register(
        "processes_cpu".to_string(),
        CommandConfig::builder()
            .name("processes_cpu")
            .category("processes")
            .description("Processes sorted by cpu (top-like format)")
            .cli_command("show processes cpu")
            .formatter("processes_cpu")
            .build(),
    );

    registry.register(
        "processes_memory".to_string(),
        CommandConfig::builder()
            .name("processes_memory")
            .category("processes")
            .description("Processes sorted by memory (top-like format)")
            .cli_command("show processes memory")
            .formatter("processes_memory")
            .build(),
    );

    // NOTE: Self-referential ``processes_processes`` entry for bare
    // ``show processes`` is registered centrally in
    // ``register_intermediate_commands``.

    // Watermark telemetry
    registry.register(
        "watermark_telemetry".to_string(),
        CommandConfig::builder()
            .name("watermark_telemetry")
            .category("watermark")
            .description("Watermark telemetry interval")
            .cli_command("show watermark telemetry interval")
            .formatter("watermark_telemetry")
            .build(),
    );

    // MMU
    let mmu_config = CommandConfig::builder()
        .name("mmu")
        .category("system")
        .description("MMU configuration")
        .cli_command("show mmu")
        .formatter("mmu")
        .build();

    registry.register("mmu".to_string(), mmu_config.clone());
    registry.register("mmu_mmu".to_string(), mmu_config);
}

/// Register LLDP commands.
fn register_lldp_commands(registry: &mut CommandRegistry) {
    registry.register(
        "lldp_table".to_string(),
        CommandConfig::builder()
            .name("lldp_table")
            .category("lldp")
            .description("LLDP neighbor table")
            .cli_command("show lldp table")
            .formatter("lldp_table")
            .build(),
    );

    registry.register(
        "lldp_neighbors".to_string(),
        CommandConfig::builder()
            .name("lldp_neighbors")
            .category("lldp")
            .description("LLDP neighbors detailed view")
            .cli_command("show lldp neighbors")
            .formatter("lldp_neighbors")
            .build(),
    );
}

/// Register VLAN commands.
fn register_vlan_commands(registry: &mut CommandRegistry) {
    registry.register(
        "vlan_brief".to_string(),
        CommandConfig::builder()
            .name("vlan_brief")
            .category("vlan")
            .description("VLAN brief information")
            .cli_command("show vlan brief")
            .formatter("vlan_brief")
            .build(),
    );
}

/// Register reboot cause commands.
fn register_reboot_cause_commands(registry: &mut CommandRegistry) {
    let base_config = CommandConfig::builder()
        .name("reboot_cause")
        .category("reboot-cause")
        .description("Current reboot cause")
        .cli_command("show reboot-cause")
        .formatter("reboot_cause")
        .build();

    registry.register("reboot_cause".to_string(), base_config.clone());
    registry.register("reboot-cause".to_string(), base_config.clone());
    registry.register("reboot_cause_current".to_string(), base_config.clone());
    registry.register("reboot-cause_current".to_string(), base_config.clone());
    // Also likely reboot-cause_reboot-cause
    registry.register("reboot-cause_reboot-cause".to_string(), base_config);

    let history_config = CommandConfig::builder()
        .name("reboot_cause_history")
        .category("reboot-cause")
        .description("Reboot cause history")
        .cli_command("show reboot-cause history")
        .formatter("reboot_cause_history")
        .build();

    registry.register("reboot_cause_history".to_string(), history_config.clone());
    registry.register("reboot-cause_history".to_string(), history_config);
}

/// Register interface commands.
fn register_interface_commands(registry: &mut CommandRegistry) {
    registry.register(
        "interfaces_description".to_string(),
        CommandConfig::builder()
            .name("interfaces_description")
            .category("interfaces")
            .description("Interface descriptions")
            .cli_command("show interfaces description")
            .formatter("interface_description")
            .build(),
    );

    registry.register(
        "interfaces_status".to_string(),
        CommandConfig::builder()
            .name("interfaces_status")
            .category("interfaces")
            .description("Interface status")
            .cli_command("show interfaces status")
            .formatter("interface_status")
            .build(),
    );

    registry.register(
        "interfaces_counters".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters")
            .category("interfaces")
            .description("Interface counters")
            .cli_command("show interfaces counters")
            .formatter("interface_counters")
            .build(),
    );

    registry.register(
        "interfaces_counters_detailed".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_detailed")
            .category("interfaces")
            .description("Interface counters detailed")
            .cli_command("show interfaces counters detailed")
            .formatter("interface_counters_detailed")
            .build(),
    );

    // Counter variants - each with its own specialized formatter
    registry.register(
        "interfaces_counters_errors".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_errors")
            .category("interfaces")
            .description("Interface counters errors")
            .cli_command("show interfaces counters-errors")
            .formatter("interface_counters_errors")
            .build(),
    );

    registry.register(
        "interfaces_counters_rates".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_rates")
            .category("interfaces")
            .description("Interface counters rates")
            .cli_command("show interfaces counters-rates")
            .formatter("interface_counters_rates")
            .build(),
    );

    registry.register(
        "interfaces_counters_rif".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_rif")
            .category("interfaces")
            .description("Interface counters rif")
            .cli_command("show interfaces counters-rif")
            .formatter("interface_counters_rif")
            .build(),
    );

    registry.register(
        "interfaces_counters_fec_stats".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_fec_stats")
            .category("interfaces")
            .description("Interface counters fec stats")
            .cli_command("show interfaces counters-fec-stats")
            .formatter("fec_stats")
            .build(),
    );

    registry.register(
        "interfaces_counters_trim".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_trim")
            .category("interfaces")
            .description("Interface counters trim")
            .cli_command("show interfaces counters-trim")
            .formatter("counters_trim")
            .build(),
    );

    // FEC histogram uses its own formatter
    registry.register(
        "interfaces_counters_fec_histogram".to_string(),
        CommandConfig::builder()
            .name("interfaces_counters_fec_histogram")
            .category("interfaces")
            .description("FEC histogram")
            .cli_command("show interfaces counters fec-histogram")
            .formatter("fec_histogram")
            .build(),
    );

    registry.register(
        "interfaces_errors".to_string(),
        CommandConfig::builder()
            .name("interfaces_errors")
            .category("interfaces")
            .description("Interface errors")
            .cli_command("show interfaces errors")
            .formatter("interface_errors")
            .build(),
    );

    registry.register(
        "interfaces_alias".to_string(),
        CommandConfig::builder()
            .name("interfaces_alias")
            .category("interfaces")
            .description("Interface aliases")
            .cli_command("show interfaces alias")
            .formatter("interface_alias")
            .build(),
    );

    registry.register(
        "interfaces_naming_mode".to_string(),
        CommandConfig::builder()
            .name("interfaces_naming_mode")
            .category("interfaces")
            .description("Interface naming mode")
            .cli_command("show interfaces naming-mode")
            .formatter("naming_mode")
            .build(),
    );

    registry.register(
        "interfaces_flap".to_string(),
        CommandConfig::builder()
            .name("interfaces_flap")
            .category("interfaces")
            .description("Interface flap counts")
            .cli_command("show interfaces flap")
            .formatter("interface_flap")
            .build(),
    );

    registry.register(
        "interfaces_portchannel".to_string(),
        CommandConfig::builder()
            .name("interfaces_portchannel")
            .category("interfaces")
            .description("Port channel information")
            .cli_command("show interfaces portchannel")
            .formatter("portchannel")
            .build(),
    );

    registry.register(
        "interfaces_neighbor_expected".to_string(),
        CommandConfig::builder()
            .name("interfaces_neighbor_expected")
            .category("interfaces")
            .description("Interface neighbors expected")
            .cli_command("show interfaces neighbor expected")
            .formatter("interface_neighbor")
            .build(),
    );

    // Transceiver subcommands
    let transceiver_commands = [
        (
            "transceiver-eeprom",
            "Transceiver EEPROM",
            "transceiver_eeprom",
        ),
        (
            "transceiver-error-status",
            "Transceiver error status",
            "transceiver_error_status",
        ),
        ("transceiver-info", "Transceiver info", "transceiver_info"),
        (
            "transceiver-lpmode",
            "Transceiver low-power mode",
            "transceiver_lpmode",
        ),
        (
            "transceiver-pm",
            "Transceiver performance monitoring",
            "transceiver_pm",
        ),
        (
            "transceiver-presence",
            "Transceiver presence",
            "transceiver_presence",
        ),
        (
            "transceiver-status",
            "Transceiver status",
            "transceiver_status",
        ),
    ];

    for (subcmd, desc, formatter) in transceiver_commands {
        let name = format!("interfaces_{}", subcmd.replace('-', "_"));

        registry.register(
            name.clone(),
            CommandConfig::builder()
                .name(&name)
                .category("interfaces")
                .description(desc)
                .cli_command(&format!("show interfaces {}", subcmd))
                .formatter(formatter)
                .build(),
        );
    }

    // FEC status
    registry.register(
        "interfaces_fec_status".to_string(),
        CommandConfig::builder()
            .name("interfaces_fec_status")
            .category("interfaces")
            .description("FEC status")
            .cli_command("show interfaces fec status")
            .formatter("fec_status")
            .build(),
    );

    // Switchport subcommands
    registry.register(
        "interfaces_switchport_config".to_string(),
        CommandConfig::builder()
            .name("interfaces_switchport_config")
            .category("interfaces")
            .description("Switchport config")
            .cli_command("show interfaces switchport config")
            .formatter("switchport_config")
            .build(),
    );

    registry.register(
        "interfaces_switchport_status".to_string(),
        CommandConfig::builder()
            .name("interfaces_switchport_status")
            .category("interfaces")
            .description("Switchport status")
            .cli_command("show interfaces switchport status")
            .formatter("switchport_status")
            .build(),
    );
}

/// Register network commands (NDP, SRv6, MAC).
fn register_network_commands(registry: &mut CommandRegistry) {
    let ndp_config = CommandConfig::builder()
        .name("ndp")
        .category("network")
        .description("IPv6 NDP table")
        .cli_command("show ndp")
        .formatter("ndp")
        .build();

    registry.register("ndp".to_string(), ndp_config.clone());
    registry.register("ndp_ndp".to_string(), ndp_config);

    registry.register(
        "srv6_stats".to_string(),
        CommandConfig::builder()
            .name("srv6_stats")
            .category("srv6")
            .description("SRv6 statistics")
            .cli_command("show srv6 stats")
            .formatter("srv6")
            .build(),
    );

    let mac_config = CommandConfig::builder()
        .name("mac")
        .category("mac")
        .description("MAC address table")
        .cli_command("show mac")
        .formatter("mac")
        .build();

    registry.register("mac".to_string(), mac_config.clone());
    registry.register("mac_mac".to_string(), mac_config);

    registry.register(
        "mac_aging_time".to_string(),
        CommandConfig::builder()
            .name("mac_aging_time")
            .category("mac")
            .description("MAC aging time")
            .cli_command("show mac aging-time")
            .formatter("mac_aging_time")
            .build(),
    );
}

/// Register IPv6 commands.
fn register_ipv6_commands(registry: &mut CommandRegistry) {
    let ipv6_commands = [
        ("interfaces", "ipv6_interfaces", "IPv6 interfaces"),
        ("route", "ipv6_route", "IPv6 routes"),
        ("protocol", "ipv6_protocol", "IPv6 protocol"),
        ("bgp-neighbors", "ipv6_bgp_neighbors", "IPv6 BGP neighbors"),
        ("bgp-network", "ipv6_bgp_network", "IPv6 BGP network"),
        ("bgp-summary", "ipv6_bgp_summary", "IPv6 BGP summary"),
        ("fib", "ipv6_fib", "IPv6 FIB"),
    ];

    for (subcmd, formatter, desc) in ipv6_commands {
        let name = format!("ipv6_{}", subcmd.replace('-', "_"));

        registry.register(
            name.clone(),
            CommandConfig::builder()
                .name(&name)
                .category("ipv6")
                .description(desc)
                .cli_command(&format!("show ipv6 {}", subcmd))
                .formatter(formatter)
                .build(),
        );
    }

    registry.register(
        "ipv6_prefix_list".to_string(),
        CommandConfig::builder()
            .name("ipv6_prefix_list")
            .category("ipv6")
            .description("IPv6 prefix list")
            .cli_command("show ipv6 prefix-list")
            .formatter("ipv6_prefix_list")
            .build(),
    );

    registry.register(
        "ipv6_link_local_mode".to_string(),
        CommandConfig::builder()
            .name("ipv6_link_local_mode")
            .category("ipv6")
            .description("IPv6 link local mode")
            .cli_command("show ipv6 link-local-mode")
            .formatter("ipv6_link_local_mode")
            .column_order(&["port", "mode"])
            .build(),
    );
}

/// Register intermediate (non-leaf) show commands.
///
/// Each intermediate is a path prefix that has children but is itself
/// queryable via gNMI. The device returns a click-style "usage + subcommands"
/// payload which is rendered by the `command_help` formatter.
///
/// Registry keys use the two-segment naming convention
/// `"<category>_<subcommand>"` — for single-segment intermediates the
/// subcommand is the category itself (self-ref).
///
/// NOTE: `show watermark telemetry` is intentionally omitted because its
/// registry key `watermark_telemetry` is already claimed by the leaf
/// formatter behind `show watermark telemetry interval`.
fn register_intermediate_commands(registry: &mut CommandRegistry) {
    // (category, subcommand) — subcommand == category for single-segment.
    let intermediates: &[(&str, &str)] = &[
        // Single-segment intermediates:
        ("processes", "processes"),
        ("buffer_pool", "buffer_pool"),
        ("headroom-pool", "headroom-pool"),
        ("priority-group", "priority-group"),
        ("queue", "queue"),
        ("dropcounters", "dropcounters"),
        ("watermark", "watermark"),
        ("lldp", "lldp"),
        ("vlan", "vlan"),
        ("interfaces", "interfaces"),
        ("srv6", "srv6"),
        ("ipv6", "ipv6"),
        // Multi-segment intermediates:
        ("queue", "watermark"),
        ("queue", "persistent-watermark"),
        ("priority-group", "watermark"),
        ("priority-group", "persistent-watermark"),
        ("interfaces", "neighbor"),
        ("interfaces", "fec"),
        ("interfaces", "transceiver"),
        ("interfaces", "switchport"),
        ("ipv6", "bgp"),
    ];

    for (category, subcommand) in intermediates {
        let cli = if category == subcommand {
            format!("show {}", category)
        } else {
            format!("show {} {}", category, subcommand)
        };
        let key = format!("{}_{}", category, subcommand.replace('-', "_"));
        let description = if category == subcommand {
            format!("{} help", category)
        } else {
            format!("{} {} help", category, subcommand)
        };

        registry.register(
            key.clone(),
            CommandConfig::builder()
                .name(&key)
                .category(category)
                .description(&description)
                .cli_command(&cli)
                .formatter("command_help")
                .build(),
        );
    }
}

/// Register all CLI command mappings.
fn register_cli_mappings(registry: &mut CommandRegistry) {
    let cli_mappings = create_cli_mappings();

    for mapping in cli_mappings {
        // CLI mappings are stored in a separate map in the registry
        // For now, we register commands that have associated mappings
        let command_name = format!(
            "{}_{}",
            mapping.category,
            mapping.subcommand.replace('-', "_")
        );

        if !registry.contains(&command_name) {
            registry.register(
                command_name,
                CommandConfig::builder()
                    .name(&mapping.category)
                    .category(&mapping.category)
                    .description(&mapping.description)
                    .cli_command(&mapping.cli_command)
                    .telemetry_paths(&[&mapping.gnmi_path])
                    .build(),
            );
        }

        // Also store the CLI mapping for list_supported_commands()
        let cli_mapping = CliMapping::new(
            &mapping.cli_command,
            &mapping.gnmi_path,
            &mapping.category,
            &mapping.subcommand,
        )
        .with_description(&mapping.description);
        registry.register_cli_mapping(mapping.cli_command.clone(), cli_mapping);
    }
}

/// Create all CLI mappings.
fn create_cli_mappings() -> Vec<CliMappingDef> {
    vec![
        // Buffer pool
        CliMappingDef::new(
            "show buffer_pool watermark",
            "SHOW/buffer_pool/watermark",
            "buffer_pool",
            "watermark",
            "Buffer pool user watermarks",
        ),
        CliMappingDef::new(
            "show buffer_pool persistent-watermark",
            "SHOW/buffer_pool/persistent-watermark",
            "buffer_pool",
            "persistent-watermark",
            "Buffer pool persistent watermarks",
        ),
        // Headroom pool
        CliMappingDef::new(
            "show headroom-pool watermark",
            "SHOW/headroom-pool/watermark",
            "headroom-pool",
            "watermark",
            "Headroom pool user watermarks",
        ),
        CliMappingDef::new(
            "show headroom-pool persistent-watermark",
            "SHOW/headroom-pool/persistent-watermark",
            "headroom-pool",
            "persistent-watermark",
            "Headroom pool persistent watermarks",
        ),
        // Priority group
        CliMappingDef::new(
            "show priority-group watermark shared",
            "SHOW/priority-group/watermark/shared",
            "priority-group",
            "watermark-shared",
            "PG shared user watermarks",
        ),
        CliMappingDef::new(
            "show priority-group persistent-watermark shared",
            "SHOW/priority-group/persistent-watermark/shared",
            "priority-group",
            "persistent-watermark-shared",
            "PG shared persistent watermarks",
        ),
        CliMappingDef::new(
            "show priority-group watermark headroom",
            "SHOW/priority-group/watermark/headroom",
            "priority-group",
            "watermark-headroom",
            "PG headroom user watermarks",
        ),
        CliMappingDef::new(
            "show priority-group persistent-watermark headroom",
            "SHOW/priority-group/persistent-watermark/headroom",
            "priority-group",
            "persistent-watermark-headroom",
            "PG headroom persistent watermarks",
        ),
        // Queue watermarks
        CliMappingDef::new(
            "show queue watermark unicast",
            "SHOW/queue/watermark/unicast",
            "queue",
            "watermark-unicast",
            "Unicast queue user watermarks",
        ),
        CliMappingDef::new(
            "show queue persistent-watermark unicast",
            "SHOW/queue/persistent-watermark/unicast",
            "queue",
            "persistent-watermark-unicast",
            "Unicast queue persistent watermarks",
        ),
        CliMappingDef::new(
            "show queue watermark multicast",
            "SHOW/queue/watermark/multicast",
            "queue",
            "watermark-multicast",
            "Multicast queue user watermarks",
        ),
        CliMappingDef::new(
            "show queue persistent-watermark multicast",
            "SHOW/queue/persistent-watermark/multicast",
            "queue",
            "persistent-watermark-multicast",
            "Multicast queue persistent watermarks",
        ),
        CliMappingDef::new(
            "show queue watermark all",
            "SHOW/queue/watermark/all",
            "queue",
            "watermark-all",
            "All queue user watermarks",
        ),
        CliMappingDef::new(
            "show queue persistent-watermark all",
            "SHOW/queue/persistent-watermark/all",
            "queue",
            "persistent-watermark-all",
            "All queue persistent watermarks",
        ),
        // Clock
        CliMappingDef::new(
            "show clock",
            "SHOW/clock",
            "clock",
            "date",
            "Current date/time",
        ),
        CliMappingDef::new(
            "show clock timezones",
            "SHOW/clock/timezones",
            "clock",
            "timezones",
            "Available timezones",
        ),
        // Dropcounters
        CliMappingDef::new(
            "show dropcounters capabilities",
            "SHOW/dropcounters/capabilities",
            "dropcounters",
            "capabilities",
            "Drop counter capabilities",
        ),
        CliMappingDef::new(
            "show dropcounters configuration",
            "SHOW/dropcounters/configuration",
            "dropcounters",
            "configuration",
            "Drop counter configuration",
        ),
        CliMappingDef::new(
            "show dropcounters counts",
            "SHOW/dropcounters/counts",
            "dropcounters",
            "counts",
            "Drop counter counts",
        ),
        // System commands
        CliMappingDef::new(
            "show uptime",
            "SHOW/uptime",
            "uptime",
            "uptime",
            "System uptime",
        ),
        CliMappingDef::new(
            "show version",
            "SHOW/version",
            "version",
            "version",
            "SONiC version",
        ),
        CliMappingDef::new(
            "show system-memory",
            "SHOW/system-memory",
            "system-memory",
            "memory",
            "System memory",
        ),
        CliMappingDef::new(
            "show services",
            "SHOW/services",
            "services",
            "services",
            "Docker services",
        ),
        CliMappingDef::new(
            "show processes summary",
            "SHOW/processes/summary",
            "processes",
            "summary",
            "Process summary",
        ),
        CliMappingDef::new(
            "show processes cpu",
            "SHOW/processes/cpu",
            "processes",
            "cpu",
            "Processes by CPU",
        ),
        CliMappingDef::new(
            "show processes memory",
            "SHOW/processes/memory",
            "processes",
            "memory",
            "Processes by memory",
        ),
        CliMappingDef::new(
            "show watermark telemetry interval",
            "SHOW/watermark/telemetry/interval",
            "watermark",
            "telemetry",
            "Watermark telemetry interval",
        ),
        CliMappingDef::new("show mmu", "SHOW/mmu", "mmu", "mmu", "MMU configuration"),
        // LLDP
        CliMappingDef::new(
            "show lldp table",
            "SHOW/lldp/table",
            "lldp",
            "table",
            "LLDP table",
        ),
        CliMappingDef::new(
            "show lldp neighbors",
            "SHOW/lldp/neighbors",
            "lldp",
            "neighbors",
            "LLDP neighbors",
        ),
        // VLAN
        CliMappingDef::new(
            "show vlan brief",
            "SHOW/vlan/brief",
            "vlan",
            "brief",
            "VLAN brief",
        ),
        // Reboot cause
        CliMappingDef::new(
            "show reboot-cause",
            "SHOW/reboot-cause",
            "reboot-cause",
            "reboot-cause",
            "Reboot cause",
        ),
        CliMappingDef::new(
            "show reboot-cause history",
            "SHOW/reboot-cause/history",
            "reboot-cause",
            "history",
            "Reboot cause history",
        ),
        // Queue counters
        CliMappingDef::new(
            "show queue counters",
            "SHOW/queue/counters",
            "queue",
            "counters",
            "Queue counters",
        ),
        CliMappingDef::new(
            "show queue wredcounters",
            "SHOW/queue/wredcounters",
            "queue",
            "wredcounters",
            "Queue WRED counters",
        ),
        // Interface commands
        CliMappingDef::new(
            "show interfaces description",
            "SHOW/interfaces/description",
            "interfaces",
            "description",
            "Interface descriptions",
        ),
        CliMappingDef::new(
            "show interfaces status",
            "SHOW/interfaces/status",
            "interfaces",
            "status",
            "Interface status",
        ),
        CliMappingDef::new(
            "show interfaces counters",
            "SHOW/interfaces/counters",
            "interfaces",
            "counters",
            "Interface counters",
        ),
        CliMappingDef::new(
            "show interfaces counters detailed",
            "SHOW/interfaces/counters/detailed",
            "interfaces",
            "counters-detailed",
            "Detailed interface counters",
        ),
        CliMappingDef::new(
            "show interfaces counters errors",
            "SHOW/interfaces/counters/errors",
            "interfaces",
            "counters-errors",
            "Interface counter errors",
        ),
        CliMappingDef::new(
            "show interfaces counters rates",
            "SHOW/interfaces/counters/rates",
            "interfaces",
            "counters-rates",
            "Interface counter rates",
        ),
        CliMappingDef::new(
            "show interfaces counters rif",
            "SHOW/interfaces/counters/rif",
            "interfaces",
            "counters-rif",
            "RIF counters",
        ),
        CliMappingDef::new(
            "show interfaces counters fec-stats",
            "SHOW/interfaces/counters/fec-stats",
            "interfaces",
            "counters-fec-stats",
            "FEC statistics",
        ),
        CliMappingDef::new(
            "show interfaces counters fec-histogram",
            "SHOW/interfaces/counters/fec-histogram",
            "interfaces",
            "counters-fec-histogram",
            "FEC histogram",
        ),
        CliMappingDef::new(
            "show interfaces counters trim",
            "SHOW/interfaces/counters/trim",
            "interfaces",
            "counters-trim",
            "Interface trim counters",
        ),
        CliMappingDef::new(
            "show interfaces errors",
            "SHOW/interfaces/errors",
            "interfaces",
            "errors",
            "Interface errors",
        ),
        CliMappingDef::new(
            "show interfaces alias",
            "SHOW/interfaces/alias",
            "interfaces",
            "alias",
            "Interface aliases",
        ),
        CliMappingDef::new(
            "show interfaces naming-mode",
            "SHOW/interfaces/naming-mode",
            "interfaces",
            "naming-mode",
            "Interface naming mode",
        ),
        CliMappingDef::new(
            "show interfaces flap",
            "SHOW/interfaces/flap",
            "interfaces",
            "flap",
            "Interface flap counts",
        ),
        CliMappingDef::new(
            "show interfaces portchannel",
            "SHOW/interfaces/portchannel",
            "interfaces",
            "portchannel",
            "Port channel info",
        ),
        CliMappingDef::new(
            "show interfaces neighbor expected",
            "SHOW/interfaces/neighbor/expected",
            "interfaces",
            "neighbor-expected",
            "Interface neighbors expected",
        ),
        CliMappingDef::new(
            "show interfaces fec status",
            "SHOW/interfaces/fec/status",
            "interfaces",
            "fec-status",
            "FEC status",
        ),
        // Transceiver commands
        CliMappingDef::new(
            "show interfaces transceiver eeprom",
            "SHOW/interfaces/transceiver/eeprom",
            "interfaces",
            "transceiver-eeprom",
            "Transceiver EEPROM",
        ),
        CliMappingDef::new(
            "show interfaces transceiver error-status",
            "SHOW/interfaces/transceiver/error-status",
            "interfaces",
            "transceiver-error-status",
            "Transceiver error status",
        ),
        CliMappingDef::new(
            "show interfaces transceiver info",
            "SHOW/interfaces/transceiver/info",
            "interfaces",
            "transceiver-info",
            "Transceiver info",
        ),
        CliMappingDef::new(
            "show interfaces transceiver lpmode",
            "SHOW/interfaces/transceiver/lpmode",
            "interfaces",
            "transceiver-lpmode",
            "Transceiver LP mode",
        ),
        CliMappingDef::new(
            "show interfaces transceiver pm",
            "SHOW/interfaces/transceiver/pm",
            "interfaces",
            "transceiver-pm",
            "Transceiver PM",
        ),
        CliMappingDef::new(
            "show interfaces transceiver presence",
            "SHOW/interfaces/transceiver/presence",
            "interfaces",
            "transceiver-presence",
            "Transceiver presence",
        ),
        CliMappingDef::new(
            "show interfaces transceiver status",
            "SHOW/interfaces/transceiver/status",
            "interfaces",
            "transceiver-status",
            "Transceiver status",
        ),
        // Switchport commands
        CliMappingDef::new(
            "show interfaces switchport config",
            "SHOW/interfaces/switchport/config",
            "interfaces",
            "switchport-config",
            "Switchport config",
        ),
        CliMappingDef::new(
            "show interfaces switchport status",
            "SHOW/interfaces/switchport/status",
            "interfaces",
            "switchport-status",
            "Switchport status",
        ),
        // Network commands
        CliMappingDef::new("show ndp", "SHOW/ndp", "ndp", "ndp", "NDP table"),
        CliMappingDef::new(
            "show srv6 stats",
            "SHOW/srv6/stats",
            "srv6",
            "stats",
            "SRv6 statistics",
        ),
        CliMappingDef::new("show mac", "SHOW/mac", "mac", "mac", "MAC table"),
        CliMappingDef::new(
            "show mac aging-time",
            "SHOW/mac/aging-time",
            "mac",
            "aging-time",
            "MAC aging time",
        ),
        // IPv6 commands
        CliMappingDef::new(
            "show ipv6 prefix-list",
            "SHOW/ipv6/prefix-list",
            "ipv6",
            "prefix-list",
            "IPv6 prefix list",
        ),
        CliMappingDef::new(
            "show ipv6 interfaces",
            "SHOW/ipv6/interfaces",
            "ipv6",
            "interfaces",
            "IPv6 interfaces",
        ),
        CliMappingDef::new(
            "show ipv6 route",
            "SHOW/ipv6/route",
            "ipv6",
            "route",
            "IPv6 routes",
        ),
        CliMappingDef::new(
            "show ipv6 protocol",
            "SHOW/ipv6/protocol",
            "ipv6",
            "protocol",
            "IPv6 protocol",
        ),
        CliMappingDef::new(
            "show ipv6 bgp neighbors",
            "SHOW/ipv6/bgp/neighbors",
            "ipv6",
            "bgp-neighbors",
            "IPv6 BGP neighbors",
        ),
        CliMappingDef::new(
            "show ipv6 bgp network",
            "SHOW/ipv6/bgp/network",
            "ipv6",
            "bgp-network",
            "IPv6 BGP network",
        ),
        CliMappingDef::new(
            "show ipv6 bgp summary",
            "SHOW/ipv6/bgp/summary",
            "ipv6",
            "bgp-summary",
            "IPv6 BGP summary",
        ),
        CliMappingDef::new(
            "show ipv6 link-local-mode",
            "SHOW/ipv6/link-local-mode",
            "ipv6",
            "link-local-mode",
            "IPv6 link local mode",
        ),
        CliMappingDef::new("show ipv6 fib", "SHOW/ipv6/fib", "ipv6", "fib", "IPv6 FIB"),
        // ---- Intermediate (non-leaf) commands ----
        // Single-segment intermediates (returns click-style subcommand
        // discovery payload; rendered by CommandHelpFormatter).
        CliMappingDef::new(
            "show processes", "SHOW/processes",
            "processes", "processes", "Processes help",
        ),
        CliMappingDef::new(
            "show buffer_pool", "SHOW/buffer_pool",
            "buffer_pool", "buffer_pool", "Buffer pool help",
        ),
        CliMappingDef::new(
            "show headroom-pool", "SHOW/headroom-pool",
            "headroom-pool", "headroom-pool", "Headroom pool help",
        ),
        CliMappingDef::new(
            "show priority-group", "SHOW/priority-group",
            "priority-group", "priority-group", "Priority group help",
        ),
        CliMappingDef::new(
            "show queue", "SHOW/queue", "queue", "queue", "Queue help",
        ),
        CliMappingDef::new(
            "show dropcounters", "SHOW/dropcounters",
            "dropcounters", "dropcounters", "Dropcounters help",
        ),
        CliMappingDef::new(
            "show watermark", "SHOW/watermark",
            "watermark", "watermark", "Watermark help",
        ),
        CliMappingDef::new(
            "show lldp", "SHOW/lldp", "lldp", "lldp", "LLDP help",
        ),
        CliMappingDef::new(
            "show vlan", "SHOW/vlan", "vlan", "vlan", "VLAN help",
        ),
        CliMappingDef::new(
            "show interfaces", "SHOW/interfaces",
            "interfaces", "interfaces", "Interfaces help",
        ),
        CliMappingDef::new(
            "show srv6", "SHOW/srv6", "srv6", "srv6", "SRv6 help",
        ),
        CliMappingDef::new(
            "show ipv6", "SHOW/ipv6", "ipv6", "ipv6", "IPv6 help",
        ),
        // Multi-segment intermediates.
        CliMappingDef::new(
            "show queue watermark", "SHOW/queue/watermark",
            "queue", "watermark", "Queue watermark help",
        ),
        CliMappingDef::new(
            "show queue persistent-watermark",
            "SHOW/queue/persistent-watermark",
            "queue", "persistent-watermark",
            "Queue persistent-watermark help",
        ),
        CliMappingDef::new(
            "show priority-group watermark",
            "SHOW/priority-group/watermark",
            "priority-group", "watermark",
            "Priority group watermark help",
        ),
        CliMappingDef::new(
            "show priority-group persistent-watermark",
            "SHOW/priority-group/persistent-watermark",
            "priority-group", "persistent-watermark",
            "Priority group persistent-watermark help",
        ),
        CliMappingDef::new(
            "show interfaces neighbor", "SHOW/interfaces/neighbor",
            "interfaces", "neighbor", "Interfaces neighbor help",
        ),
        CliMappingDef::new(
            "show interfaces fec", "SHOW/interfaces/fec",
            "interfaces", "fec", "Interfaces fec help",
        ),
        CliMappingDef::new(
            "show interfaces transceiver", "SHOW/interfaces/transceiver",
            "interfaces", "transceiver", "Interfaces transceiver help",
        ),
        CliMappingDef::new(
            "show interfaces switchport", "SHOW/interfaces/switchport",
            "interfaces", "switchport", "Interfaces switchport help",
        ),
        CliMappingDef::new(
            "show ipv6 bgp", "SHOW/ipv6/bgp",
            "ipv6", "bgp", "IPv6 BGP help",
        ),
    ]
}

/// CLI mapping definition.
struct CliMappingDef {
    cli_command: String,
    gnmi_path: String,
    category: String,
    subcommand: String,
    description: String,
}

impl CliMappingDef {
    fn new(
        cli_command: &str,
        gnmi_path: &str,
        category: &str,
        subcommand: &str,
        description: &str,
    ) -> Self {
        Self {
            cli_command: cli_command.to_string(),
            gnmi_path: gnmi_path.to_string(),
            category: category.to_string(),
            subcommand: subcommand.to_string(),
            description: description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_commands() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);

        // Should have many commands registered
        assert!(registry.len() > 50);
    }

    #[test]
    fn test_buffer_pool_commands() {
        let mut registry = CommandRegistry::new();
        register_buffer_pool_commands(&mut registry);

        assert!(registry.contains("buffer_pool_watermark"));
        assert!(registry.contains("buffer_pool_persistent_watermark"));
    }

    #[test]
    fn test_queue_commands() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);

        assert!(registry.contains("queue_watermark_unicast"));
        assert!(registry.contains("queue_counters"));
    }

    #[test]
    fn test_interface_commands() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);

        assert!(registry.contains("interfaces_status"));
        assert!(registry.contains("interfaces_counters"));
        assert!(registry.contains("interfaces_transceiver_info"));
    }

    #[test]
    fn test_system_commands() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);

        assert!(registry.contains("uptime"));
        assert!(registry.contains("version"));
        assert!(registry.contains("services"));
    }

    #[test]
    fn test_ipv6_commands() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);

        assert!(registry.contains("ipv6_route"));
        assert!(registry.contains("ipv6_bgp_summary"));
    }

    #[test]
    fn test_cli_mappings_count() {
        let mappings = create_cli_mappings();
        assert!(mappings.len() > 60);
    }

    #[test]
    fn test_command_categories() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);

        let categories = registry.categories();
        assert!(categories.contains(&"interfaces".to_string()));
        assert!(categories.contains(&"queue".to_string()));
        assert!(categories.contains(&"ipv6".to_string()));
    }

    // --- Per-command registration tests (GAP-B1) ---

    #[test]
    fn test_buffer_pool_watermark_config() {
        let mut registry = CommandRegistry::new();
        register_buffer_pool_commands(&mut registry);
        let config = registry.get("buffer_pool_watermark").unwrap();
        assert_eq!(config.category(), Some("buffer_pool"));
        assert_eq!(config.formatter(), Some("buffer_pool"));
    }

    #[test]
    fn test_buffer_pool_persistent_watermark_config() {
        let mut registry = CommandRegistry::new();
        register_buffer_pool_commands(&mut registry);
        let config = registry.get("buffer_pool_persistent_watermark").unwrap();
        assert_eq!(config.category(), Some("buffer_pool"));
        assert_eq!(config.formatter(), Some("buffer_pool"));
    }

    #[test]
    fn test_buffer_pool_hyphen_aliases() {
        let mut registry = CommandRegistry::new();
        register_buffer_pool_commands(&mut registry);
        assert!(registry.contains("buffer-pool_watermark"));
        assert!(registry.contains("buffer-pool_persistent_watermark"));
    }

    #[test]
    fn test_headroom_pool_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_headroom_pool_commands(&mut registry);
        assert!(registry.contains("headroom_pool_watermark"));
        assert!(registry.contains("headroom_pool_persistent_watermark"));
        assert!(registry.contains("headroom-pool_watermark"));
        assert!(registry.contains("headroom-pool_persistent_watermark"));
    }

    #[test]
    fn test_headroom_pool_watermark_config() {
        let mut registry = CommandRegistry::new();
        register_headroom_pool_commands(&mut registry);
        let config = registry.get("headroom_pool_watermark").unwrap();
        assert_eq!(config.category(), Some("headroom-pool"));
        assert_eq!(config.formatter(), Some("headroom_pool"));
    }

    #[test]
    fn test_priority_group_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_priority_group_commands(&mut registry);
        assert!(registry.contains("priority_group_watermark_shared"));
        assert!(registry.contains("priority_group_persistent_watermark_shared"));
        assert!(registry.contains("priority_group_watermark_headroom"));
        assert!(registry.contains("priority_group_persistent_watermark_headroom"));
    }

    #[test]
    fn test_priority_group_config() {
        let mut registry = CommandRegistry::new();
        register_priority_group_commands(&mut registry);
        let config = registry.get("priority_group_watermark_shared").unwrap();
        assert_eq!(config.category(), Some("priority-group"));
        assert_eq!(config.formatter(), Some("queue_watermark"));
    }

    #[test]
    fn test_queue_watermark_unicast_config() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        let config = registry.get("queue_watermark_unicast").unwrap();
        assert_eq!(config.category(), Some("queue"));
        assert_eq!(config.formatter(), Some("queue_watermark"));
    }

    #[test]
    fn test_queue_watermark_multicast_config() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        let config = registry.get("queue_watermark_multicast").unwrap();
        assert_eq!(config.formatter(), Some("queue_watermark"));
    }

    #[test]
    fn test_queue_watermark_all_config() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        assert!(registry.contains("queue_watermark_all"));
        assert!(registry.contains("queue_persistent_watermark_all"));
    }

    #[test]
    fn test_queue_persistent_watermark_commands() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        assert!(registry.contains("queue_persistent_watermark_unicast"));
        assert!(registry.contains("queue_persistent_watermark_multicast"));
        assert!(registry.contains("queue_persistent_watermark_all"));
    }

    #[test]
    fn test_queue_counters_config() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        let config = registry.get("queue_counters").unwrap();
        assert_eq!(config.formatter(), Some("queue_counters"));
    }

    #[test]
    fn test_queue_wredcounters_config() {
        let mut registry = CommandRegistry::new();
        register_queue_commands(&mut registry);
        let config = registry.get("queue_wredcounters").unwrap();
        assert_eq!(config.formatter(), Some("wred_counters"));
    }

    #[test]
    fn test_queue_help_commands() {
        // Self-referential help entries are now registered centrally in
        // ``register_intermediate_commands``, not in ``register_queue_commands``.
        let mut registry = CommandRegistry::new();
        register_intermediate_commands(&mut registry);
        let config = registry.get("queue_watermark").unwrap();
        assert_eq!(config.formatter(), Some("command_help"));
        let config2 = registry.get("queue_persistent_watermark").unwrap();
        assert_eq!(config2.formatter(), Some("command_help"));
    }

    #[test]
    fn test_clock_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_clock_commands(&mut registry);
        assert!(registry.contains("clock"));
        assert!(registry.contains("clock_clock"));
        assert!(registry.contains("clock_date"));
        assert!(registry.contains("clock_timezones"));
    }

    #[test]
    fn test_clock_config() {
        let mut registry = CommandRegistry::new();
        register_clock_commands(&mut registry);
        let config = registry.get("clock").unwrap();
        assert_eq!(config.category(), Some("clock"));
        assert_eq!(config.formatter(), Some("clock"));
    }

    #[test]
    fn test_clock_timezones_config() {
        let mut registry = CommandRegistry::new();
        register_clock_commands(&mut registry);
        let config = registry.get("clock_timezones").unwrap();
        assert_eq!(config.formatter(), Some("list"));
    }

    #[test]
    fn test_dropcounters_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_dropcounters_commands(&mut registry);
        assert!(registry.contains("dropcounters_capabilities"));
        assert!(registry.contains("dropcounters_configuration"));
        assert!(registry.contains("dropcounters_counts"));
    }

    #[test]
    fn test_dropcounters_capabilities_config() {
        let mut registry = CommandRegistry::new();
        register_dropcounters_commands(&mut registry);
        let config = registry.get("dropcounters_capabilities").unwrap();
        assert_eq!(config.category(), Some("dropcounters"));
        assert_eq!(config.formatter(), Some("dropcounters_capabilities"));
    }

    #[test]
    fn test_dropcounters_counts_config() {
        let mut registry = CommandRegistry::new();
        register_dropcounters_commands(&mut registry);
        let config = registry.get("dropcounters_counts").unwrap();
        assert_eq!(config.formatter(), Some("dropcounters_counts"));
    }

    #[test]
    fn test_system_uptime_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("uptime").unwrap();
        assert_eq!(config.category(), Some("system"));
        assert_eq!(config.formatter(), Some("uptime"));
    }

    #[test]
    fn test_system_uptime_alias() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        assert!(registry.contains("uptime_uptime"));
    }

    #[test]
    fn test_system_version_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("version").unwrap();
        assert_eq!(config.formatter(), Some("version"));
    }

    #[test]
    fn test_system_memory_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("system_memory").unwrap();
        assert_eq!(config.formatter(), Some("system_memory"));
    }

    #[test]
    fn test_system_memory_aliases() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        assert!(registry.contains("system_memory_system_memory"));
        assert!(registry.contains("system-memory_system_memory"));
    }

    #[test]
    fn test_services_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("services").unwrap();
        assert_eq!(config.formatter(), Some("services"));
        assert!(registry.contains("services_services"));
    }

    #[test]
    fn test_processes_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        assert!(registry.contains("processes_summary"));
        assert!(registry.contains("processes_cpu"));
        assert!(registry.contains("processes_memory"));
    }

    #[test]
    fn test_processes_summary_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("processes_summary").unwrap();
        assert_eq!(config.category(), Some("processes"));
        assert_eq!(config.formatter(), Some("processes"));
    }

    #[test]
    fn test_watermark_telemetry_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("watermark_telemetry").unwrap();
        assert_eq!(config.category(), Some("watermark"));
        assert_eq!(config.formatter(), Some("watermark_telemetry"));
    }

    #[test]
    fn test_mmu_config() {
        let mut registry = CommandRegistry::new();
        register_system_commands(&mut registry);
        let config = registry.get("mmu").unwrap();
        assert_eq!(config.category(), Some("system"));
        assert_eq!(config.formatter(), Some("mmu"));
        assert!(registry.contains("mmu_mmu"));
    }

    #[test]
    fn test_lldp_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_lldp_commands(&mut registry);
        assert!(registry.contains("lldp_table"));
        assert!(registry.contains("lldp_neighbors"));
    }

    #[test]
    fn test_lldp_table_config() {
        let mut registry = CommandRegistry::new();
        register_lldp_commands(&mut registry);
        let config = registry.get("lldp_table").unwrap();
        assert_eq!(config.category(), Some("lldp"));
        assert_eq!(config.formatter(), Some("lldp_table"));
    }

    #[test]
    fn test_lldp_neighbors_config() {
        let mut registry = CommandRegistry::new();
        register_lldp_commands(&mut registry);
        let config = registry.get("lldp_neighbors").unwrap();
        assert_eq!(config.formatter(), Some("lldp_neighbors"));
    }

    #[test]
    fn test_vlan_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_vlan_commands(&mut registry);
        assert!(registry.contains("vlan_brief"));
    }

    #[test]
    fn test_vlan_brief_config() {
        let mut registry = CommandRegistry::new();
        register_vlan_commands(&mut registry);
        let config = registry.get("vlan_brief").unwrap();
        assert_eq!(config.category(), Some("vlan"));
        assert_eq!(config.formatter(), Some("vlan_brief"));
    }

    #[test]
    fn test_reboot_cause_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_reboot_cause_commands(&mut registry);
        assert!(registry.contains("reboot_cause"));
        assert!(registry.contains("reboot-cause"));
        assert!(registry.contains("reboot_cause_history"));
        assert!(registry.contains("reboot-cause_history"));
    }

    #[test]
    fn test_reboot_cause_config() {
        let mut registry = CommandRegistry::new();
        register_reboot_cause_commands(&mut registry);
        let config = registry.get("reboot_cause").unwrap();
        assert_eq!(config.category(), Some("reboot-cause"));
        assert_eq!(config.formatter(), Some("reboot_cause"));
    }

    #[test]
    fn test_reboot_cause_history_config() {
        let mut registry = CommandRegistry::new();
        register_reboot_cause_commands(&mut registry);
        let config = registry.get("reboot_cause_history").unwrap();
        assert_eq!(config.formatter(), Some("reboot_cause_history"));
    }

    #[test]
    fn test_interface_description_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_description").unwrap();
        assert_eq!(config.category(), Some("interfaces"));
        assert_eq!(config.formatter(), Some("interface_description"));
    }

    #[test]
    fn test_interface_status_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_status").unwrap();
        assert_eq!(config.formatter(), Some("interface_status"));
    }

    #[test]
    fn test_interface_counters_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters").unwrap();
        assert_eq!(config.formatter(), Some("interface_counters"));
    }

    #[test]
    fn test_interface_counters_detailed_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_detailed").unwrap();
        assert_eq!(config.formatter(), Some("interface_counters_detailed"));
    }

    #[test]
    fn test_interface_counters_errors_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_errors").unwrap();
        assert_eq!(config.formatter(), Some("interface_counters_errors"));
    }

    #[test]
    fn test_interface_counters_rates_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_rates").unwrap();
        assert_eq!(config.formatter(), Some("interface_counters_rates"));
    }

    #[test]
    fn test_interface_counters_rif_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_rif").unwrap();
        assert_eq!(config.formatter(), Some("interface_counters_rif"));
    }

    #[test]
    fn test_interface_counters_fec_stats_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_fec_stats").unwrap();
        assert_eq!(config.formatter(), Some("fec_stats"));
    }

    #[test]
    fn test_interface_counters_trim_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_trim").unwrap();
        assert_eq!(config.formatter(), Some("counters_trim"));
    }

    #[test]
    fn test_interface_fec_histogram_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_counters_fec_histogram").unwrap();
        assert_eq!(config.formatter(), Some("fec_histogram"));
    }

    #[test]
    fn test_interface_errors_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_errors").unwrap();
        assert_eq!(config.formatter(), Some("interface_errors"));
    }

    #[test]
    fn test_interface_alias_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_alias").unwrap();
        assert_eq!(config.formatter(), Some("interface_alias"));
    }

    #[test]
    fn test_interface_naming_mode_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_naming_mode").unwrap();
        assert_eq!(config.formatter(), Some("naming_mode"));
    }

    #[test]
    fn test_interface_flap_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_flap"));
        let config = registry.get("interfaces_flap").unwrap();
        assert_eq!(config.formatter(), Some("interface_flap"));
    }

    #[test]
    fn test_interface_fec_status_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_fec_status"));
        let config = registry.get("interfaces_fec_status").unwrap();
        assert_eq!(config.formatter(), Some("fec_status"));
    }

    #[test]
    fn test_interface_neighbor_expected_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_neighbor_expected"));
        let config = registry.get("interfaces_neighbor_expected").unwrap();
        assert_eq!(config.formatter(), Some("interface_neighbor"));
    }

    #[test]
    fn test_interface_portchannel_config() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_portchannel"));
        let config = registry.get("interfaces_portchannel").unwrap();
        assert_eq!(config.formatter(), Some("portchannel"));
    }

    #[test]
    fn test_interface_transceiver_commands() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_transceiver_info"));
        assert!(registry.contains("interfaces_transceiver_eeprom"));
        assert!(registry.contains("interfaces_transceiver_status"));
        assert!(registry.contains("interfaces_transceiver_presence"));
        assert!(registry.contains("interfaces_transceiver_lpmode"));
    }

    #[test]
    fn test_interface_switchport_commands() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        assert!(registry.contains("interfaces_switchport_config"));
        assert!(registry.contains("interfaces_switchport_status"));
    }

    #[test]
    fn test_switchport_config_formatter() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_switchport_config").unwrap();
        assert_eq!(config.formatter(), Some("switchport_config"));
    }

    #[test]
    fn test_switchport_status_formatter() {
        let mut registry = CommandRegistry::new();
        register_interface_commands(&mut registry);
        let config = registry.get("interfaces_switchport_status").unwrap();
        assert_eq!(config.formatter(), Some("switchport_status"));
    }

    #[test]
    fn test_network_commands_registered() {
        let mut registry = CommandRegistry::new();
        register_network_commands(&mut registry);
        assert!(registry.contains("mac"));
        assert!(registry.contains("mac_aging_time"));
        assert!(registry.contains("ndp"));
    }

    #[test]
    fn test_mac_config() {
        let mut registry = CommandRegistry::new();
        register_network_commands(&mut registry);
        let config = registry.get("mac").unwrap();
        assert_eq!(config.formatter(), Some("mac"));
    }

    #[test]
    fn test_ndp_config() {
        let mut registry = CommandRegistry::new();
        register_network_commands(&mut registry);
        let config = registry.get("ndp").unwrap();
        assert_eq!(config.formatter(), Some("ndp"));
    }

    #[test]
    fn test_srv6_config() {
        let mut registry = CommandRegistry::new();
        register_network_commands(&mut registry);
        assert!(registry.contains("srv6_stats"));
    }

    #[test]
    fn test_ipv6_commands_detailed() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        assert!(registry.contains("ipv6_route"));
        assert!(registry.contains("ipv6_bgp_summary"));
        assert!(registry.contains("ipv6_bgp_neighbors"));
        assert!(registry.contains("ipv6_bgp_network"));
        assert!(registry.contains("ipv6_interfaces"));
        assert!(registry.contains("ipv6_prefix_list"));
    }

    #[test]
    fn test_ipv6_route_config() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        let config = registry.get("ipv6_route").unwrap();
        assert_eq!(config.category(), Some("ipv6"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_config() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        let config = registry.get("ipv6_bgp_neighbors").unwrap();
        assert_eq!(config.category(), Some("ipv6"));
    }

    #[test]
    fn test_ipv6_link_local_mode_config() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        assert!(registry.contains("ipv6_link_local_mode"));
    }

    #[test]
    fn test_ipv6_fib_config() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        assert!(registry.contains("ipv6_fib"));
    }

    #[test]
    fn test_ipv6_protocol_config() {
        let mut registry = CommandRegistry::new();
        register_ipv6_commands(&mut registry);
        assert!(registry.contains("ipv6_protocol"));
    }

    #[test]
    fn test_cli_mappings_content() {
        let mappings = create_cli_mappings();
        assert!(mappings
            .iter()
            .any(|m| m.cli_command == "show interfaces status"));
        assert!(mappings.iter().any(|m| m.cli_command == "show version"));
        assert!(mappings.iter().any(|m| m.cli_command == "show clock"));
    }

    #[test]
    fn test_cli_mapping_fields() {
        let mappings = create_cli_mappings();
        for mapping in &mappings {
            assert!(
                !mapping.cli_command.is_empty(),
                "CLI command should not be empty"
            );
            assert!(!mapping.category.is_empty(), "Category should not be empty");
        }
    }

    #[test]
    fn test_register_idempotent() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);
        let count1 = registry.len();
        register_all_commands(&mut registry);
        let count2 = registry.len();
        assert_eq!(count1, count2);
    }

    #[test]
    fn test_explicitly_registered_commands_have_formatter() {
        // Commands explicitly registered by register_*_commands() should
        // all have a formatter. Commands auto-generated by register_cli_mappings()
        // may not (they are fallback entries).
        let mut explicit_registry = CommandRegistry::new();
        register_buffer_pool_commands(&mut explicit_registry);
        register_headroom_pool_commands(&mut explicit_registry);
        register_priority_group_commands(&mut explicit_registry);
        register_queue_commands(&mut explicit_registry);
        register_clock_commands(&mut explicit_registry);
        register_dropcounters_commands(&mut explicit_registry);
        register_system_commands(&mut explicit_registry);
        register_lldp_commands(&mut explicit_registry);
        register_vlan_commands(&mut explicit_registry);
        register_reboot_cause_commands(&mut explicit_registry);
        register_interface_commands(&mut explicit_registry);
        register_network_commands(&mut explicit_registry);
        register_ipv6_commands(&mut explicit_registry);
        for name in explicit_registry.command_names() {
            let config = explicit_registry.get(name).unwrap();
            assert!(
                config.formatter().is_some(),
                "Explicitly registered command '{}' should have a formatter",
                name
            );
        }
    }

    #[test]
    fn test_all_commands_have_category() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);
        for name in registry.command_names() {
            let config = registry.get(name).unwrap();
            assert!(
                config.category().is_some(),
                "Command '{}' should have a category",
                name
            );
        }
    }

    #[test]
    fn test_expected_category_list() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);
        let cats = registry.categories();
        let expected = [
            "buffer_pool",
            "headroom-pool",
            "priority-group",
            "queue",
            "clock",
            "dropcounters",
            "system",
            "lldp",
            "vlan",
            "reboot-cause",
            "interfaces",
        ];
        for cat in &expected {
            assert!(cats.contains(&cat.to_string()), "Missing category: {}", cat);
        }
    }

    #[test]
    fn test_command_count_minimum() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);
        assert!(
            registry.len() >= 70,
            "Expected >= 70 commands, got {}",
            registry.len()
        );
    }

    #[test]
    fn test_cli_mappings_registered_in_registry() {
        let mut registry = CommandRegistry::new();
        register_all_commands(&mut registry);
        let mappings = registry.list_cli_mappings();
        assert!(
            mappings.len() >= 60,
            "Expected >= 60 CLI mappings, got {}",
            mappings.len()
        );
    }
}
