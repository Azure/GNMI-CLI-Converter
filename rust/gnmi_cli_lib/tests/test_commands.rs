//! Integration tests for command registration.

use gnmi_cli_lib::registry::create_default_registry;

// =============================================================================
// Category existence tests
// =============================================================================

#[test]
fn test_all_expected_categories_exist() {
    let registry = create_default_registry();
    let cats = registry.categories();

    let expected_cats = vec![
        "interfaces",
        "queue",
        "ipv6",
        "system",
        "lldp",
        "clock",
        "dropcounters",
        "reboot-cause",
        "vlan",
    ];

    for cat in expected_cats {
        assert!(cats.contains(&cat.to_string()), "Missing category: {}", cat);
    }
}

#[test]
fn test_registry_has_many_commands() {
    let registry = create_default_registry();
    assert!(
        registry.len() >= 70,
        "Expected at least 70 registered commands, got {}",
        registry.len()
    );
}

// =============================================================================
// Buffer pool commands
// =============================================================================

#[test]
fn test_buffer_pool_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("buffer_pool_watermark"));
    assert!(registry.contains("buffer_pool_persistent_watermark"));
}

// =============================================================================
// Headroom pool commands
// =============================================================================

#[test]
fn test_headroom_pool_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("headroom_pool_watermark"));
    assert!(registry.contains("headroom_pool_persistent_watermark"));
}

// =============================================================================
// Priority group commands
// =============================================================================

#[test]
fn test_priority_group_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("priority_group_watermark_shared"));
    assert!(registry.contains("priority_group_persistent_watermark_shared"));
    assert!(registry.contains("priority_group_watermark_headroom"));
    assert!(registry.contains("priority_group_persistent_watermark_headroom"));
}

// =============================================================================
// Queue commands
// =============================================================================

#[test]
fn test_queue_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("queue_watermark_unicast"));
    assert!(registry.contains("queue_persistent_watermark_unicast"));
    assert!(registry.contains("queue_watermark_multicast"));
    assert!(registry.contains("queue_persistent_watermark_multicast"));
    assert!(registry.contains("queue_watermark_all"));
    assert!(registry.contains("queue_persistent_watermark_all"));
    assert!(registry.contains("queue_counters"));
    assert!(registry.contains("queue_wredcounters"));
}

// =============================================================================
// Clock commands
// =============================================================================

#[test]
fn test_clock_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("clock"));
    assert!(registry.contains("clock_timezones"));
}

// =============================================================================
// Drop counter commands
// =============================================================================

#[test]
fn test_dropcounters_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("dropcounters_capabilities"));
    assert!(registry.contains("dropcounters_configuration"));
    assert!(registry.contains("dropcounters_counts"));
}

// =============================================================================
// System commands
// =============================================================================

#[test]
fn test_system_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("uptime"));
    assert!(registry.contains("version"));
    assert!(registry.contains("system_memory"));
    assert!(registry.contains("services"));
    assert!(registry.contains("processes_summary"));
    assert!(registry.contains("processes_cpu"));
    assert!(registry.contains("processes_memory"));
    assert!(registry.contains("mmu"));
    assert!(registry.contains("watermark_telemetry"));
}

// =============================================================================
// LLDP commands
// =============================================================================

#[test]
fn test_lldp_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("lldp_table"));
    assert!(registry.contains("lldp_neighbors"));
}

// =============================================================================
// VLAN commands
// =============================================================================

#[test]
fn test_vlan_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("vlan_brief"));
}

// =============================================================================
// Reboot cause commands
// =============================================================================

#[test]
fn test_reboot_cause_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("reboot_cause"));
    assert!(registry.contains("reboot_cause_current"));
    assert!(registry.contains("reboot_cause_history"));
    assert!(registry.contains("reboot-cause"));
    assert!(registry.contains("reboot-cause_current"));
}

// =============================================================================
// Interface commands
// =============================================================================

#[test]
fn test_interface_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("interfaces_description"));
    assert!(registry.contains("interfaces_status"));
    assert!(registry.contains("interfaces_counters"));
    assert!(registry.contains("interfaces_counters_detailed"));
    assert!(registry.contains("interfaces_counters_errors"));
    assert!(registry.contains("interfaces_counters_rates"));
    assert!(registry.contains("interfaces_counters_rif"));
    assert!(registry.contains("interfaces_counters_fec_stats"));
    assert!(registry.contains("interfaces_counters_trim"));
    assert!(registry.contains("interfaces_counters_fec_histogram"));
    assert!(registry.contains("interfaces_errors"));
    assert!(registry.contains("interfaces_alias"));
    assert!(registry.contains("interfaces_naming_mode"));
    assert!(registry.contains("interfaces_flap"));
    assert!(registry.contains("interfaces_portchannel"));
    assert!(registry.contains("interfaces_neighbor_expected"));
    assert!(registry.contains("interfaces_fec_status"));
    assert!(registry.contains("interfaces_switchport_config"));
    assert!(registry.contains("interfaces_switchport_status"));
}

// =============================================================================
// Interface counter formatters correctness
// =============================================================================

#[test]
fn test_interface_counter_formatters_correct() {
    let registry = create_default_registry();

    let config = registry.get("interfaces_counters_errors").unwrap();
    assert_eq!(
        config.formatter(),
        Some("interface_counters_errors"),
        "interfaces_counters_errors should use interface_counters_errors formatter"
    );

    let config = registry.get("interfaces_counters_rates").unwrap();
    assert_eq!(
        config.formatter(),
        Some("interface_counters_rates"),
        "interfaces_counters_rates should use interface_counters_rates formatter"
    );

    let config = registry.get("interfaces_counters_rif").unwrap();
    assert_eq!(
        config.formatter(),
        Some("interface_counters_rif"),
        "interfaces_counters_rif should use interface_counters_rif formatter"
    );

    let config = registry.get("interfaces_counters_fec_stats").unwrap();
    assert_eq!(
        config.formatter(),
        Some("fec_stats"),
        "interfaces_counters_fec_stats should use fec_stats formatter"
    );

    let config = registry.get("interfaces_counters_trim").unwrap();
    assert_eq!(
        config.formatter(),
        Some("counters_trim"),
        "interfaces_counters_trim should use counters_trim formatter"
    );
}

// =============================================================================
// Transceiver commands
// =============================================================================

#[test]
fn test_transceiver_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("interfaces_transceiver_eeprom"));
    assert!(registry.contains("interfaces_transceiver_error_status"));
    assert!(registry.contains("interfaces_transceiver_info"));
    assert!(registry.contains("interfaces_transceiver_lpmode"));
    assert!(registry.contains("interfaces_transceiver_pm"));
    assert!(registry.contains("interfaces_transceiver_presence"));
    assert!(registry.contains("interfaces_transceiver_status"));
}

// =============================================================================
// Network commands
// =============================================================================

#[test]
fn test_network_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("ndp"));
    assert!(registry.contains("srv6_stats"));
    assert!(registry.contains("mac"));
    assert!(registry.contains("mac_aging_time"));
}

// =============================================================================
// IPv6 commands
// =============================================================================

#[test]
fn test_ipv6_commands_exist() {
    let registry = create_default_registry();
    assert!(registry.contains("ipv6_route"));
    assert!(registry.contains("ipv6_bgp_summary"));
    assert!(registry.contains("ipv6_bgp_neighbors"));
    assert!(registry.contains("ipv6_bgp_network"));
    assert!(registry.contains("ipv6_interfaces"));
    assert!(registry.contains("ipv6_fib"));
    assert!(registry.contains("ipv6_prefix_list"));
    assert!(registry.contains("ipv6_link_local_mode"));
    assert!(registry.contains("ipv6_protocol"));
}

// =============================================================================
// Formatter lookup tests
// =============================================================================

#[test]
fn test_all_commands_have_formatters() {
    let registry = create_default_registry();
    let names: Vec<String> = registry
        .command_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    for name in &names {
        let config = registry.get(name).unwrap();
        if let Some(formatter_name) = config.formatter() {
            let formatter = registry.get_formatter_by_name(formatter_name);
            assert!(
                formatter.is_some(),
                "Command '{}' references formatter '{}' which doesn't exist",
                name,
                formatter_name
            );
        }
    }
}
