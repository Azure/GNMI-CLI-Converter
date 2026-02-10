//! Integration tests for CommandParser.

use gnmi_cli_lib::parser::CommandParser;
use serde_json::json;

// =============================================================================
// Basic parsing tests
// =============================================================================

#[test]
fn test_parse_show_interfaces_status() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces status");
    assert!(parsed.is_valid);
    assert_eq!(parsed.command, "interfaces");
    assert_eq!(parsed.subcommand, "status");
}

#[test]
fn test_parse_show_buffer_pool_watermark() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show buffer_pool watermark");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_headroom_pool_watermark() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show headroom-pool watermark");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_clock() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show clock");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_clock_timezones() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show clock timezones");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_dropcounters_capabilities() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show dropcounters capabilities");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_dropcounters_counts() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show dropcounters counts");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_interfaces_counters() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces counters");
    assert!(parsed.is_valid);
    assert_eq!(parsed.command, "interfaces");
    assert_eq!(parsed.subcommand, "counters");
}

#[test]
fn test_parse_show_interfaces_counters_detailed() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces counters detailed");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_interfaces_description() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces description");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_uptime() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show uptime");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_version() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show version");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_lldp_table() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show lldp table");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_lldp_neighbors() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show lldp neighbors");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_vlan_brief() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show vlan brief");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_reboot_cause() {
    let parser = CommandParser::new();
    // "reboot-cause" requires a subcommand (current/history) in parser
    let parsed = parser.parse("show reboot-cause current");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_reboot_cause_history() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show reboot-cause history");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_queue_counters() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show queue counters");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_queue_wredcounters() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show queue wredcounters");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ndp() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ndp");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_mac() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show mac");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_mac_aging_time() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show mac aging-time");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_mmu() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show mmu");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_services() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show services");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_system_memory() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show system-memory");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_processes_summary() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show processes summary");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_processes_cpu() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show processes cpu");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_watermark_telemetry_interval() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show watermark telemetry interval");
    assert!(parsed.is_valid);
}

// =============================================================================
// IPv6 command parsing tests
// =============================================================================

#[test]
fn test_parse_show_ipv6_route() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 route");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ipv6_bgp_summary() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 bgp summary");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ipv6_bgp_neighbors() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 bgp neighbors");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ipv6_prefix_list() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 prefix-list");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ipv6_interfaces() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 interfaces");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_ipv6_fib() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show ipv6 fib");
    assert!(parsed.is_valid);
}

// =============================================================================
// Interface transceiver parsing tests
// =============================================================================

#[test]
fn test_parse_show_interfaces_transceiver_eeprom() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces transceiver eeprom");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_interfaces_transceiver_info() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces transceiver info");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_interfaces_transceiver_status() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces transceiver status");
    assert!(parsed.is_valid);
}

#[test]
fn test_parse_show_interfaces_transceiver_presence() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show interfaces transceiver presence");
    assert!(parsed.is_valid);
}

// =============================================================================
// Invalid command tests
// =============================================================================

#[test]
fn test_parse_invalid_command() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show nonexistent_command");
    assert!(!parsed.is_valid);
}

#[test]
fn test_parse_empty_string() {
    let parser = CommandParser::new();
    let parsed = parser.parse("");
    assert!(!parsed.is_valid);
}

#[test]
fn test_parse_just_show() {
    let parser = CommandParser::new();
    let parsed = parser.parse("show");
    assert!(!parsed.is_valid);
}

// =============================================================================
// List and help tests
// =============================================================================

#[test]
fn test_list_commands_not_empty() {
    let parser = CommandParser::new();
    let commands = parser.list_commands();
    assert!(!commands.is_empty());
}

#[test]
fn test_list_commands_has_many_entries() {
    let parser = CommandParser::new();
    let commands = parser.list_commands();
    assert!(
        commands.len() >= 15,
        "Expected at least 15 command categories, got {}",
        commands.len()
    );
}

#[test]
fn test_get_command_help() {
    let parser = CommandParser::new();
    let help = parser.get_command_help("interfaces", None);
    assert!(!help.is_empty());
}

#[test]
fn test_get_command_help_with_subcommand() {
    let parser = CommandParser::new();
    let help = parser.get_command_help("interfaces", Some("status"));
    assert!(!help.is_empty());
}

// =============================================================================
// parse_and_convert tests
// =============================================================================

#[test]
fn test_parse_and_convert_interfaces_status() {
    let parser = CommandParser::new();
    let data = json!({
        "Ethernet0": {
            "admin_status": "up",
            "oper_status": "up",
            "speed": "100G"
        }
    });
    let result = parser.parse_and_convert("show interfaces status", data);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_parse_and_convert_invalid_command() {
    let parser = CommandParser::new();
    let result = parser.parse_and_convert("show nonexistent_command", json!({}));
    assert!(result.is_err());
}

#[test]
fn test_get_gnmi_path() {
    let parser = CommandParser::new();
    let path = parser.get_gnmi_path("show interfaces status");
    assert!(path.is_some());
}
