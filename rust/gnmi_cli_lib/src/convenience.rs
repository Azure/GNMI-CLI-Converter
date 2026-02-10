//! Convenience functions for quick conversion.
//!
//! This module provides simple wrapper functions for common conversion tasks.
//! These functions use a shared converter instance for efficiency.

use crate::converter::{ConvertResult, GnmiToCliConverter};
use crate::registry::create_default_registry;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    /// Shared converter instance.
    static ref CONVERTER: Mutex<GnmiToCliConverter> = Mutex::new(GnmiToCliConverter::new());
}

/// Quick conversion function.
///
/// # Arguments
///
/// * `command_name` - Internal command name (e.g., "buffer_pool_watermark")
/// * `json_data` - JSON data to convert
///
/// # Returns
///
/// CLI-formatted string.
pub fn convert(command_name: &str, json_data: Value) -> ConvertResult<String> {
    let converter = CONVERTER.lock().unwrap();
    converter.convert(command_name, json_data)
}

// =============================================================================
// Buffer Pool Commands
// =============================================================================

/// Convert buffer pool watermark (show buffer_pool watermark).
pub fn convert_buffer_pool_watermark(json_data: Value, persistent: bool) -> ConvertResult<String> {
    let cmd = if persistent {
        "buffer_pool_persistent_watermark"
    } else {
        "buffer_pool_watermark"
    };
    convert(cmd, json_data)
}

/// Convert headroom pool watermark (show headroom-pool watermark).
pub fn convert_headroom_pool_watermark(
    json_data: Value,
    persistent: bool,
) -> ConvertResult<String> {
    let cmd = if persistent {
        "headroom_pool_persistent_watermark"
    } else {
        "headroom_pool_watermark"
    };
    convert(cmd, json_data)
}

// =============================================================================
// Priority Group Commands
// =============================================================================

/// Convert priority group watermark (show priority-group watermark).
///
/// # Arguments
///
/// * `json_data` - JSON data
/// * `wm_type` - Watermark type ("shared" or "headroom")
/// * `persistent` - Use persistent watermarks
pub fn convert_priority_group_watermark(
    json_data: Value,
    wm_type: &str,
    persistent: bool,
) -> ConvertResult<String> {
    let prefix = if persistent {
        "persistent_watermark"
    } else {
        "watermark"
    };
    let cmd = format!("priority_group_{}_{}", prefix, wm_type);
    convert(&cmd, json_data)
}

// =============================================================================
// Queue Commands
// =============================================================================

/// Convert queue watermark (show queue watermark).
///
/// # Arguments
///
/// * `json_data` - JSON data
/// * `queue_type` - Queue type ("unicast", "multicast", or "all")
/// * `persistent` - Use persistent watermarks
pub fn convert_queue_watermark(
    json_data: Value,
    queue_type: &str,
    persistent: bool,
) -> ConvertResult<String> {
    let prefix = if persistent {
        "persistent_watermark"
    } else {
        "watermark"
    };
    let cmd = format!("queue_{}_{}", prefix, queue_type);
    convert(&cmd, json_data)
}

/// Convert queue counters (show queue counters).
pub fn convert_queue_counters(json_data: Value) -> ConvertResult<String> {
    convert("queue_counters", json_data)
}

/// Convert queue WRED counters (show queue wredcounters).
pub fn convert_queue_wredcounters(json_data: Value) -> ConvertResult<String> {
    convert("queue_wredcounters", json_data)
}

// =============================================================================
// Clock Commands
// =============================================================================

/// Convert clock date (show clock).
pub fn convert_clock(json_data: Value) -> ConvertResult<String> {
    convert("clock", json_data)
}

/// Convert clock timezones (show clock timezones).
pub fn convert_clock_timezones(json_data: Value) -> ConvertResult<String> {
    convert("clock_timezones", json_data)
}

// =============================================================================
// Drop Counters Commands
// =============================================================================

/// Convert dropcounters capabilities (show dropcounters capabilities).
pub fn convert_dropcounters_capabilities(json_data: Value) -> ConvertResult<String> {
    convert("dropcounters_capabilities", json_data)
}

/// Convert dropcounters configuration (show dropcounters configuration).
pub fn convert_dropcounters_configuration(json_data: Value) -> ConvertResult<String> {
    convert("dropcounters_configuration", json_data)
}

/// Convert dropcounters counts (show dropcounters counts).
pub fn convert_dropcounters_counts(json_data: Value) -> ConvertResult<String> {
    convert("dropcounters_counts", json_data)
}

// =============================================================================
// System Commands
// =============================================================================

/// Convert uptime (show uptime).
pub fn convert_uptime(json_data: Value) -> ConvertResult<String> {
    convert("uptime", json_data)
}

/// Convert version (show version).
pub fn convert_version(json_data: Value) -> ConvertResult<String> {
    convert("version", json_data)
}

/// Convert system memory (show system-memory).
pub fn convert_system_memory(json_data: Value) -> ConvertResult<String> {
    convert("system_memory", json_data)
}

/// Convert services (show services).
pub fn convert_services(json_data: Value) -> ConvertResult<String> {
    convert("services", json_data)
}

/// Convert processes (show processes [summary|cpu|memory]).
pub fn convert_processes(json_data: Value, sort_by: &str) -> ConvertResult<String> {
    let cmd = format!("processes_{}", sort_by);
    convert(&cmd, json_data)
}

/// Convert MMU config (show mmu).
pub fn convert_mmu(json_data: Value) -> ConvertResult<String> {
    convert("mmu", json_data)
}

/// Convert watermark telemetry interval (show watermark telemetry interval).
pub fn convert_watermark_telemetry(json_data: Value) -> ConvertResult<String> {
    convert("watermark_telemetry", json_data)
}

// =============================================================================
// LLDP Commands
// =============================================================================

/// Convert LLDP table (show lldp table).
pub fn convert_lldp_table(json_data: Value) -> ConvertResult<String> {
    convert("lldp_table", json_data)
}

/// Convert LLDP neighbors (show lldp neighbors).
pub fn convert_lldp_neighbors(json_data: Value) -> ConvertResult<String> {
    convert("lldp_neighbors", json_data)
}

// =============================================================================
// VLAN Commands
// =============================================================================

/// Convert VLAN brief (show vlan brief).
pub fn convert_vlan_brief(json_data: Value) -> ConvertResult<String> {
    convert("vlan_brief", json_data)
}

// =============================================================================
// Reboot Cause Commands
// =============================================================================

/// Convert reboot cause (show reboot-cause \[history\]).
pub fn convert_reboot_cause(json_data: Value, history: bool) -> ConvertResult<String> {
    let cmd = if history {
        "reboot_cause_history"
    } else {
        "reboot_cause"
    };
    convert(cmd, json_data)
}

// =============================================================================
// Interface Commands
// =============================================================================

/// Convert interface description (show interfaces description).
pub fn convert_interface_description(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_description", json_data)
}

/// Convert interface status (show interfaces status).
pub fn convert_interface_status(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_status", json_data)
}

/// Convert interface counters (show interfaces counters).
pub fn convert_interface_counters(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters", json_data)
}

/// Convert interface counters detailed (show interfaces counters detailed).
pub fn convert_interface_counters_detailed(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_detailed", json_data)
}

/// Convert interface counters rates (show interfaces counters rates).
pub fn convert_interface_counters_rates(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_rates", json_data)
}

/// Convert interface counters rif (show interfaces counters rif).
pub fn convert_interface_counters_rif(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_rif", json_data)
}

/// Convert interface counters fec-stats (show interfaces counters fec-stats).
pub fn convert_interface_counters_fec_stats(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_fec_stats", json_data)
}

/// Convert interface counters fec-histogram (show interfaces counters fec-histogram).
pub fn convert_interface_counters_fec_histogram(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_fec_histogram", json_data)
}

/// Convert interface counters trim (show interfaces counters trim).
pub fn convert_interface_counters_trim(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_counters_trim", json_data)
}

/// Convert interface errors (show interfaces errors).
pub fn convert_interface_errors(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_errors", json_data)
}

/// Convert interface alias (show interfaces alias).
pub fn convert_interface_alias(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_alias", json_data)
}

/// Convert interface flap (show interfaces flap).
pub fn convert_interface_flap(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_flap", json_data)
}

/// Convert interface naming mode (show interfaces naming-mode).
pub fn convert_interface_naming_mode(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_naming_mode", json_data)
}

/// Convert interface neighbor expected (show interfaces neighbor expected).
pub fn convert_interface_neighbor_expected(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_neighbor_expected", json_data)
}

/// Convert portchannel (show interfaces portchannel).
pub fn convert_portchannel(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_portchannel", json_data)
}

/// Convert interface FEC status (show interfaces fec status).
pub fn convert_interface_fec_status(json_data: Value) -> ConvertResult<String> {
    convert("interfaces_fec_status", json_data)
}

/// Convert interface switchport (show interfaces switchport config|status).
pub fn convert_interface_switchport(
    json_data: Value,
    config_or_status: &str,
) -> ConvertResult<String> {
    let cmd = format!("interfaces_switchport_{}", config_or_status);
    convert(&cmd, json_data)
}

// =============================================================================
// Transceiver Commands
// =============================================================================

/// Convert interface transceiver (show interfaces transceiver \[subcmd\]).
///
/// # Arguments
///
/// * `json_data` - JSON data
/// * `subcmd` - Subcommand (eeprom, error_status, info, lpmode, pm, presence, status)
pub fn convert_interface_transceiver(json_data: Value, subcmd: &str) -> ConvertResult<String> {
    let cmd = format!("interfaces_transceiver_{}", subcmd);
    convert(&cmd, json_data)
}

// =============================================================================
// Network Commands
// =============================================================================

/// Convert NDP (show ndp).
pub fn convert_ndp(json_data: Value) -> ConvertResult<String> {
    convert("ndp", json_data)
}

/// Convert SRv6 stats (show srv6 stats).
pub fn convert_srv6_stats(json_data: Value) -> ConvertResult<String> {
    convert("srv6_stats", json_data)
}

/// Convert MAC table (show mac).
pub fn convert_mac(json_data: Value) -> ConvertResult<String> {
    convert("mac", json_data)
}

/// Convert MAC aging time (show mac aging-time).
pub fn convert_mac_aging_time(json_data: Value) -> ConvertResult<String> {
    convert("mac_aging_time", json_data)
}

// =============================================================================
// IPv6 Commands
// =============================================================================

/// Convert IPv6 prefix list (show ipv6 prefix-list).
pub fn convert_ipv6_prefix_list(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_prefix_list", json_data)
}

/// Convert IPv6 link local mode (show ipv6 link-local-mode).
pub fn convert_ipv6_link_local_mode(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_link_local_mode", json_data)
}

/// Convert IPv6 route (show ipv6 route).
pub fn convert_ipv6_route(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_route", json_data)
}

/// Convert IPv6 BGP summary (show ipv6 bgp summary).
pub fn convert_ipv6_bgp_summary(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_bgp_summary", json_data)
}

/// Convert IPv6 BGP neighbors (show ipv6 bgp neighbors).
pub fn convert_ipv6_bgp_neighbors(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_bgp_neighbors", json_data)
}

/// Convert IPv6 interfaces (show ipv6 interfaces).
pub fn convert_ipv6_interfaces(json_data: Value) -> ConvertResult<String> {
    convert("ipv6_interfaces", json_data)
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Get the gNMI path for a CLI command.
pub fn get_gnmi_path(cli_command: &str) -> Option<String> {
    let parser = crate::parser::CommandParser::new();
    parser.get_gnmi_path(cli_command)
}

/// List all supported commands with their mapping details.
///
/// Returns a HashMap where keys are CLI command strings (e.g., "show interfaces status")
/// and values are HashMaps with details: gnmi_path, command, subcommand, description.
pub fn list_supported_commands() -> HashMap<String, HashMap<String, String>> {
    let registry = create_default_registry();
    registry
        .list_cli_mappings()
        .iter()
        .map(|(cli_cmd, mapping)| {
            let mut info = HashMap::new();
            info.insert("gnmi_path".to_string(), mapping.gnmi_path.clone());
            info.insert("command".to_string(), mapping.command.clone());
            info.insert("subcommand".to_string(), mapping.subcommand.clone());
            info.insert("description".to_string(), mapping.description.clone());
            info.insert(
                "out_of_process_cmd".to_string(),
                mapping.out_of_process_cmd.clone(),
            );
            (cli_cmd.clone(), info)
        })
        .collect()
}

/// List all supported command names (simple version).
pub fn list_supported_command_names() -> Vec<String> {
    let converter = GnmiToCliConverter::new();
    converter.available_commands()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_interface_status() {
        let data = json!({
            "Ethernet0": {
                "admin_status": "up",
                "oper_status": "up"
            }
        });
        let _ = convert_interface_status(data);
    }

    #[test]
    fn test_list_supported_commands() {
        let commands = list_supported_commands();
        assert!(!commands.is_empty());
        // Verify the HashMap values have the expected structure
        for (cmd, info) in &commands {
            assert!(!cmd.is_empty());
            assert!(
                info.contains_key("gnmi_path"),
                "Missing gnmi_path for {}",
                cmd
            );
            assert!(info.contains_key("command"), "Missing command for {}", cmd);
            assert!(
                info.contains_key("subcommand"),
                "Missing subcommand for {}",
                cmd
            );
        }
    }

    #[test]
    fn test_get_gnmi_path() {
        let path = get_gnmi_path("show interfaces status");
        assert!(path.is_some());
    }

    #[test]
    fn test_convert_buffer_pool_watermark() {
        let data = json!({
            "pool1": {"Bytes": "1000"}
        });
        let _ = convert_buffer_pool_watermark(data.clone(), false);
        let _ = convert_buffer_pool_watermark(data, true);
    }

    #[test]
    fn test_convert_queue_watermark() {
        let data = json!({});
        let _ = convert_queue_watermark(data.clone(), "unicast", false);
        let _ = convert_queue_watermark(data, "multicast", true);
    }

    #[test]
    fn test_convert_processes() {
        let data = json!({});
        let _ = convert_processes(data.clone(), "summary");
        let _ = convert_processes(data.clone(), "cpu");
        let _ = convert_processes(data, "memory");
    }

    #[test]
    fn test_convert_reboot_cause() {
        let data = json!({});
        let _ = convert_reboot_cause(data.clone(), false);
        let _ = convert_reboot_cause(data, true);
    }

    #[test]
    fn test_convert_interface_transceiver() {
        let data = json!({});
        let _ = convert_interface_transceiver(data.clone(), "info");
        let _ = convert_interface_transceiver(data.clone(), "presence");
        let _ = convert_interface_transceiver(data, "status");
    }

    // Additional tests for coverage

    #[test]
    fn test_convert_headroom_pool_watermark() {
        let data = json!({});
        let _ = convert_headroom_pool_watermark(data.clone(), false);
        let _ = convert_headroom_pool_watermark(data, true);
    }

    #[test]
    fn test_convert_priority_group_watermark() {
        let data = json!({});
        let _ = convert_priority_group_watermark(data.clone(), "shared", false);
        let _ = convert_priority_group_watermark(data.clone(), "headroom", true);
        let _ = convert_priority_group_watermark(data, "shared", true);
    }

    #[test]
    fn test_convert_clock() {
        let data = json!({});
        let _ = convert_clock(data);
    }

    #[test]
    fn test_convert_clock_timezones() {
        let data = json!({});
        let _ = convert_clock_timezones(data);
    }

    #[test]
    fn test_convert_dropcounters_capabilities() {
        let data = json!({});
        let _ = convert_dropcounters_capabilities(data);
    }

    #[test]
    fn test_convert_dropcounters_configuration() {
        let data = json!({});
        let _ = convert_dropcounters_configuration(data);
    }

    #[test]
    fn test_convert_dropcounters_counts() {
        let data = json!({});
        let _ = convert_dropcounters_counts(data);
    }

    #[test]
    fn test_convert_interface_alias() {
        let data = json!({});
        let _ = convert_interface_alias(data);
    }

    #[test]
    fn test_convert_interface_description() {
        let data = json!({});
        let _ = convert_interface_description(data);
    }

    #[test]
    fn test_convert_interface_counters() {
        let data = json!({});
        let _ = convert_interface_counters(data);
    }

    #[test]
    fn test_convert_interface_errors() {
        let data = json!({});
        let _ = convert_interface_errors(data);
    }

    #[test]
    fn test_convert_interface_fec_status() {
        let data = json!({});
        let _ = convert_interface_fec_status(data);
    }

    #[test]
    fn test_convert_interface_flap() {
        let data = json!({});
        let _ = convert_interface_flap(data);
    }

    #[test]
    fn test_convert_interface_naming_mode() {
        let data = json!({});
        let _ = convert_interface_naming_mode(data);
    }

    #[test]
    fn test_convert_interface_neighbor_expected() {
        let data = json!({});
        let _ = convert_interface_neighbor_expected(data);
    }

    #[test]
    fn test_convert_portchannel() {
        let data = json!({});
        let _ = convert("interfaces_portchannel", data);
    }

    #[test]
    fn test_convert_interface_switchport() {
        let data = json!({});
        let _ = convert_interface_switchport(data.clone(), "config");
        let _ = convert_interface_switchport(data, "status");
    }

    #[test]
    fn test_convert_lldp_neighbors() {
        let data = json!({});
        let _ = convert_lldp_neighbors(data);
    }

    #[test]
    fn test_convert_lldp_table() {
        let data = json!({});
        let _ = convert_lldp_table(data);
    }

    #[test]
    fn test_convert_vlan_brief() {
        let data = json!({});
        let _ = convert_vlan_brief(data);
    }

    #[test]
    fn test_convert_vlan() {
        let data = json!({});
        let _ = convert("vlan_config", data);
    }

    #[test]
    fn test_convert_ndp() {
        let data = json!({});
        let _ = convert_ndp(data);
    }

    #[test]
    fn test_convert_srv6_stats() {
        let data = json!({});
        let _ = convert_srv6_stats(data);
    }

    #[test]
    fn test_convert_mac() {
        let data = json!({});
        let _ = convert_mac(data);
    }

    #[test]
    fn test_convert_mac_table() {
        let data = json!({});
        let _ = convert("mac_count", data);
    }

    #[test]
    fn test_convert_ipv6_prefix_list() {
        let data = json!({});
        let _ = convert_ipv6_prefix_list(data);
    }

    #[test]
    fn test_convert_ipv6_route() {
        let data = json!({});
        let _ = convert_ipv6_route(data);
    }

    #[test]
    fn test_convert_ipv6_bgp_summary() {
        let data = json!({});
        let _ = convert_ipv6_bgp_summary(data);
    }

    #[test]
    fn test_convert_ipv6_bgp_neighbors() {
        let data = json!({});
        let _ = convert_ipv6_bgp_neighbors(data);
    }

    #[test]
    fn test_convert_ipv6_interfaces() {
        let data = json!({});
        let _ = convert_ipv6_interfaces(data);
    }

    #[test]
    fn test_generic_convert() {
        let data = json!({});
        // Test with known commands
        let _ = convert("buffer_pool_watermark", data.clone());
        let _ = convert("clock", data.clone());
        // Test with unknown command
        let result = convert("unknown_command", data);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_queue_counters() {
        let data = json!({});
        let _ = convert_queue_counters(data);
    }

    #[test]
    fn test_convert_queue_wredcounters() {
        let data = json!({});
        let _ = convert_queue_wredcounters(data);
    }

    #[test]
    fn test_convert_uptime() {
        let data = json!({"uptime": "5 days"});
        let _ = convert_uptime(data);
    }

    #[test]
    fn test_convert_version() {
        let data = json!({});
        let _ = convert_version(data);
    }

    #[test]
    fn test_convert_system_memory() {
        let data = json!({});
        let _ = convert_system_memory(data);
    }

    #[test]
    fn test_convert_services() {
        let data = json!({});
        let _ = convert_services(data);
    }

    #[test]
    fn test_convert_mmu() {
        let data = json!({});
        let _ = convert_mmu(data);
    }

    #[test]
    fn test_convert_watermark_telemetry() {
        let data = json!({});
        let _ = convert_watermark_telemetry(data);
    }

    #[test]
    fn test_convert_mac_aging_time() {
        let data = json!({});
        let _ = convert_mac_aging_time(data);
    }

    #[test]
    fn test_convert_ipv6_link_local_mode() {
        let data = json!({});
        let _ = convert_ipv6_link_local_mode(data);
    }

    #[test]
    fn test_convert_interface_counters_detailed() {
        let data = json!({});
        let _ = convert_interface_counters_detailed(data);
    }

    #[test]
    fn test_convert_interface_counters_rates() {
        let data = json!({});
        let _ = convert_interface_counters_rates(data);
    }

    #[test]
    fn test_convert_interface_counters_rif() {
        let data = json!({});
        let _ = convert_interface_counters_rif(data);
    }

    #[test]
    fn test_convert_interface_counters_fec_stats() {
        let data = json!({});
        let _ = convert_interface_counters_fec_stats(data);
    }

    #[test]
    fn test_convert_interface_counters_fec_histogram() {
        let data = json!({});
        let _ = convert_interface_counters_fec_histogram(data);
    }

    #[test]
    fn test_convert_interface_counters_trim() {
        let data = json!({});
        let _ = convert_interface_counters_trim(data);
    }

    #[test]
    fn test_convert_portchannel_direct() {
        let data = json!({});
        let _ = convert_portchannel(data);
    }

    // ---- Round 5 coverage tests ----

    #[test]
    fn test_list_supported_command_names() {
        // Cover L431-433
        let names = list_supported_command_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"clock".to_string()));
    }
}
