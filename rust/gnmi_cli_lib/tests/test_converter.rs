//! Integration tests for GnmiToCliConverter.

use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::json;

// =============================================================================
// Basic converter tests
// =============================================================================

#[test]
fn test_converter_has_many_commands() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    assert!(
        commands.len() >= 70,
        "Expected at least 70 commands, got {}",
        commands.len()
    );
}

#[test]
fn test_converter_has_multiple_categories() {
    let converter = GnmiToCliConverter::new();
    let categories = converter.categories();
    assert!(
        categories.len() >= 10,
        "Expected at least 10 categories, got {}",
        categories.len()
    );
}

#[test]
fn test_convert_unknown_command_returns_error() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert("nonexistent_command", json!({}));
    assert!(result.is_err());
}

// =============================================================================
// Buffer pool commands
// =============================================================================

#[test]
fn test_convert_buffer_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "1000"},
        "egress_lossy_pool": {"Bytes": "2000"}
    });
    let result = converter.convert("buffer_pool_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let output = result.unwrap();
    assert!(output.contains("ingress_lossless_pool"));
    assert!(output.contains("egress_lossy_pool"));
}

#[test]
fn test_convert_buffer_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "500"}
    });
    let result = converter.convert("buffer_pool_persistent_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Interface commands
// =============================================================================

#[test]
fn test_convert_interfaces_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "admin_status": "up",
            "oper_status": "up",
            "speed": "100G"
        }
    });
    let result = converter.convert("interfaces_status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_description() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "description": "To Spine1",
            "admin_status": "up",
            "oper_status": "up"
        }
    });
    let result = converter.convert("interfaces_description", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_ok": "100",
            "tx_ok": "200"
        }
    });
    let result = converter.convert("interfaces_counters", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_detailed() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_ok": "100",
            "tx_ok": "200"
        }
    });
    let result = converter.convert("interfaces_counters_detailed", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_errors() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_err": "0",
            "tx_err": "0"
        }
    });
    let result = converter.convert("interfaces_counters_errors", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_rates() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_bps": "1000",
            "tx_bps": "2000"
        }
    });
    let result = converter.convert("interfaces_counters_rates", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_rif() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_ok": "100",
            "tx_ok": "200"
        }
    });
    let result = converter.convert("interfaces_counters_rif", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_fec_stats() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_counters_fec_stats", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_counters_trim() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_counters_trim", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_errors() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "rx_err": "0",
            "tx_err": "0"
        }
    });
    let result = converter.convert("interfaces_errors", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_alias() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "alias": "etp1"
        }
    });
    let result = converter.convert("interfaces_alias", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_naming_mode() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"mode": "default"});
    let result = converter.convert("interfaces_naming_mode", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_flap() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {
            "flap_count": "2"
        }
    });
    let result = converter.convert("interfaces_flap", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_portchannel() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("interfaces_portchannel", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_neighbor_expected() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("interfaces_neighbor_expected", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_fec_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_fec_status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_fec_histogram() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_counters_fec_histogram", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_switchport_config() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("interfaces_switchport_config", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_interfaces_switchport_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("interfaces_switchport_status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Transceiver commands
// =============================================================================

#[test]
fn test_convert_transceiver_eeprom() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_eeprom", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_info() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_info", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_presence() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_presence", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_lpmode() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_lpmode", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_error_status() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_error_status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_transceiver_pm() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "Ethernet0": {}
    });
    let result = converter.convert("interfaces_transceiver_pm", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Clock commands
// =============================================================================

#[test]
fn test_convert_clock() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"date": "2026-02-06 12:00:00"});
    let result = converter.convert("clock", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_clock_timezones() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"timezones": ["UTC", "US/Pacific", "US/Eastern"]});
    let result = converter.convert("clock_timezones", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Drop counter commands
// =============================================================================

#[test]
fn test_convert_dropcounters_capabilities() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("dropcounters_capabilities", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_dropcounters_configuration() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("dropcounters_configuration", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_dropcounters_counts() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("dropcounters_counts", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// System commands
// =============================================================================

#[test]
fn test_convert_uptime() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"uptime": "10 days, 5:30:00"});
    let result = converter.convert("uptime", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_version() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"version": "SONiC.20240101"});
    let result = converter.convert("version", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_system_memory() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("system_memory", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_services() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("services", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_processes_summary() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("processes_summary", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_processes_cpu() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("processes_cpu", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_processes_memory() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("processes_memory", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_mmu() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("mmu", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_watermark_telemetry() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("watermark_telemetry", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// LLDP commands
// =============================================================================

#[test]
fn test_convert_lldp_table() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("lldp_table", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_lldp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("lldp_neighbors", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// VLAN commands
// =============================================================================

#[test]
fn test_convert_vlan_brief() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("vlan_brief", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Reboot cause commands
// =============================================================================

#[test]
fn test_convert_reboot_cause() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"cause": "Power cycle"});
    let result = converter.convert("reboot_cause", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_reboot_cause_current() {
    let converter = GnmiToCliConverter::new();
    let data = json!({"cause": "Power cycle"});
    let result = converter.convert("reboot_cause_current", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_reboot_cause_history() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("reboot_cause_history", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Queue commands
// =============================================================================

#[test]
fn test_convert_queue_counters() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("queue_counters", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_queue_wredcounters() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("queue_wredcounters", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_queue_watermark_unicast() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("queue_watermark_unicast", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Network commands
// =============================================================================

#[test]
fn test_convert_ndp() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ndp", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_srv6_stats() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("srv6_stats", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_mac() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("mac", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_mac_aging_time() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("mac_aging_time", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// IPv6 commands
// =============================================================================

#[test]
fn test_convert_ipv6_route() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_route", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_bgp_summary() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_bgp_summary", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_bgp_neighbors() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_bgp_neighbors", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_bgp_network() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_bgp_network", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_interfaces() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_interfaces", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_prefix_list() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_prefix_list", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_link_local_mode() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_link_local_mode", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_fib() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_fib", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_ipv6_protocol() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("ipv6_protocol", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Headroom pool commands
// =============================================================================

#[test]
fn test_convert_headroom_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "500"}
    });
    let result = converter.convert("headroom_pool_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_headroom_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "500"}
    });
    let result = converter.convert("headroom_pool_persistent_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Priority group commands
// =============================================================================

#[test]
fn test_convert_priority_group_watermark_shared() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("priority_group_watermark_shared", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_convert_priority_group_watermark_headroom() {
    let converter = GnmiToCliConverter::new();
    let data = json!({});
    let result = converter.convert("priority_group_watermark_headroom", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Table format tests
// =============================================================================

#[test]
fn test_converter_with_grid_format() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Grid);
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "1000"}
    });
    let result = converter.convert("buffer_pool_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_converter_with_plain_format() {
    let converter =
        GnmiToCliConverter::with_table_format(gnmi_cli_lib::converter::TableFormat::Plain);
    let data = json!({
        "ingress_lossless_pool": {"Bytes": "1000"}
    });
    let result = converter.convert("buffer_pool_watermark", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

// =============================================================================
// Error handling tests
// =============================================================================

#[test]
fn test_convert_str_invalid_json() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert_str("buffer_pool_watermark", "not valid json");
    assert!(result.is_err());
}

#[test]
fn test_convert_from_unknown_cli_command() {
    let converter = GnmiToCliConverter::new();
    let result = converter.convert_from_cli_command("unknown cli command", json!({}));
    assert!(result.is_err());
}
