//! IPv6-related formatters for neighbors, routes, interfaces, etc.

use crate::common::types::{Alignment, CommandConfig};
use crate::formatters::base::{
    tabulate_grid, tabulate_simple_aligned, tabulate_simple_auto, value_to_string, FormatError, OutputFormatter,
};
use crate::utils::sorting::natural_cmp;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::Value;
use std::collections::HashMap;
use std::net::Ipv6Addr;
use std::str::FromStr;

/// Formatter for IPv6 neighbors output.
pub struct Ipv6NeighborFormatter;

impl Ipv6NeighborFormatter {
    /// Creates a new [`Ipv6NeighborFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6NeighborFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6NeighborFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Disabled);
        table.load_preset("                   ");

        table.set_header(vec![
            Cell::new("Address").set_alignment(CellAlignment::Left),
            Cell::new("MAC Address").set_alignment(CellAlignment::Left),
            Cell::new("Interface").set_alignment(CellAlignment::Left),
            Cell::new("VLAN").set_alignment(CellAlignment::Left),
            Cell::new("Status").set_alignment(CellAlignment::Left),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let address = obj
                        .get("address")
                        .or_else(|| obj.get("Address"))
                        .or_else(|| obj.get("ipv6"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let mac = obj
                        .get("macaddress")
                        .or_else(|| obj.get("MAC Address"))
                        .or_else(|| obj.get("mac"))
                        .or_else(|| obj.get("lladdr"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let iface = obj
                        .get("iface")
                        .or_else(|| obj.get("Interface"))
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let vlan = obj
                        .get("vlan")
                        .or_else(|| obj.get("VLAN"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let state = obj
                        .get("state")
                        .or_else(|| obj.get("State"))
                        .or_else(|| obj.get("status"))
                        .or_else(|| obj.get("Status"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&address).set_alignment(CellAlignment::Left),
                        Cell::new(&mac).set_alignment(CellAlignment::Left),
                        Cell::new(&iface).set_alignment(CellAlignment::Left),
                        Cell::new(&vlan).set_alignment(CellAlignment::Left),
                        Cell::new(&state).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            // Keyed by address
            let mut addrs: Vec<&String> = map.keys().collect();
            addrs.sort();

            for addr in addrs {
                if let Some(Value::Object(obj)) = map.get(addr) {
                    let mac = obj
                        .get("macaddress")
                        .or_else(|| obj.get("mac"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let iface = obj
                        .get("iface")
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let vlan = obj.get("vlan").map(value_to_string).unwrap_or_default();
                    let state = obj.get("state").map(value_to_string).unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(addr).set_alignment(CellAlignment::Left),
                        Cell::new(&mac).set_alignment(CellAlignment::Left),
                        Cell::new(&iface).set_alignment(CellAlignment::Left),
                        Cell::new(&vlan).set_alignment(CellAlignment::Left),
                        Cell::new(&state).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "ipv6_neighbor"
    }
}

/// Formatter for IPv6 interface addresses output.
pub struct Ipv6InterfaceFormatter;

impl Ipv6InterfaceFormatter {
    /// Creates a new [`Ipv6InterfaceFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6InterfaceFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6InterfaceFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers = ["Interface", "Master", "IPv6 address/mask", "Admin/Oper", "BGP Neighbor", "Neighbor IP"];
        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Value::Array(arr) = data {
            // Array of flat objects
            for item in arr {
                if let Value::Object(obj) = item {
                    let iface_name = obj
                        .get("interface")
                        .or_else(|| obj.get("Interface"))
                        .or_else(|| obj.get("name"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let master = obj
                        .get("master")
                        .or_else(|| obj.get("Master"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let ipv6_addr = obj
                        .get("ipv6_address")
                        .or_else(|| obj.get("IPv6 Address/Mask"))
                        .or_else(|| obj.get("address"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let status = if let Some(ao) = obj.get("Admin/Oper") {
                        value_to_string(ao)
                    } else {
                        let admin = obj
                            .get("admin_status")
                            .map(value_to_string)
                            .unwrap_or_default();
                        let oper = obj
                            .get("oper_status")
                            .map(value_to_string)
                            .unwrap_or_default();
                        if !admin.is_empty() && !oper.is_empty() {
                            format!("{}/{}", admin, oper)
                        } else {
                            String::new()
                        }
                    };
                    let bgp_neighbor = obj
                        .get("bgp_neighbor_name")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let neighbor_ip = obj
                        .get("bgp_neighbor_ip")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let addr_type = obj
                        .get("type")
                        .or_else(|| obj.get("Type"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    let formatted_addr = format_ipv6_with_interface(&ipv6_addr, &iface_name);
                    let formatted_addr_with_type = if !addr_type.is_empty() {
                        format!("{} ({})", formatted_addr, addr_type)
                    } else {
                        formatted_addr
                    };

                    rows.push(vec![
                        iface_name, master, formatted_addr_with_type,
                        status, bgp_neighbor, neighbor_ip,
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut ifaces: Vec<&String> = map.keys().collect();
            ifaces.sort_by(|a, b| natural_cmp(a, b));

            for iface_name in ifaces {
                if let Some(Value::Object(obj)) = map.get(iface_name) {
                    let master = obj
                        .get("master")
                        .or_else(|| obj.get("Master"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let admin = obj
                        .get("admin_status")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let oper = obj
                        .get("oper_status")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let status = if let Some(ao) = obj.get("Admin/Oper") {
                        value_to_string(ao)
                    } else if !admin.is_empty() && !oper.is_empty() {
                        format!("{}/{}", admin, oper)
                    } else {
                        String::new()
                    };
                    let addr_type = obj
                        .get("type")
                        .or_else(|| obj.get("Type"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    if let Some(Value::Array(addrs)) = obj.get("ipv6_addresses") {
                        if addrs.is_empty() {
                            continue;
                        }

                        for (idx, addr_obj) in addrs.iter().enumerate() {
                            if let Value::Object(addr_map) = addr_obj {
                                let raw_addr = addr_map
                                    .get("address")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let bgp_neighbor = addr_map
                                    .get("bgp_neighbor_name")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let neighbor_ip = addr_map
                                    .get("bgp_neighbor_ip")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let formatted_addr =
                                    format_ipv6_with_interface(&raw_addr, iface_name);
                                let per_addr_type = addr_map
                                    .get("type")
                                    .or_else(|| addr_map.get("Type"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let effective_type = if !per_addr_type.is_empty() {
                                    &per_addr_type
                                } else {
                                    &addr_type
                                };
                                let formatted_addr_with_type = if !effective_type.is_empty() {
                                    format!("{} ({})", formatted_addr, effective_type)
                                } else {
                                    formatted_addr
                                };

                                if idx == 0 {
                                    rows.push(vec![
                                        iface_name.clone(), master.clone(), formatted_addr_with_type,
                                        status.clone(), bgp_neighbor, neighbor_ip,
                                    ]);
                                } else {
                                    rows.push(vec![
                                        String::new(), String::new(), formatted_addr_with_type,
                                        String::new(), bgp_neighbor, neighbor_ip,
                                    ]);
                                }
                            }
                        }
                    } else if let Some(addr_val) = obj
                        .get("ipv6_address")
                        .or_else(|| obj.get("IPv6 Address/Mask"))
                        .or_else(|| obj.get("address"))
                    {
                        let ipv6_addr = value_to_string(addr_val);
                        let formatted_addr = format_ipv6_with_interface(&ipv6_addr, iface_name);
                        let formatted_addr_with_type = if !addr_type.is_empty() {
                            format!("{} ({})", formatted_addr, addr_type)
                        } else {
                            formatted_addr
                        };
                        let bgp_neighbor = obj
                            .get("bgp_neighbor_name")
                            .map(value_to_string)
                            .unwrap_or_default();
                        let neighbor_ip = obj
                            .get("bgp_neighbor_ip")
                            .map(value_to_string)
                            .unwrap_or_default();

                        rows.push(vec![
                            iface_name.clone(), master, formatted_addr_with_type,
                            status, bgp_neighbor, neighbor_ip,
                        ]);
                    }
                }
            }
        }

        // All columns are left-aligned
        let alignments = vec![false; headers.len()];
        Ok(tabulate_simple_aligned(&headers, &rows, &alignments))
    }

    fn name(&self) -> &'static str {
        "ipv6_interface"
    }
}

/// Format an IPv6 address with interface name suffix for link-local addresses.
/// Link-local addresses (fe80::) are formatted as "addr%interface/prefix"
/// Regular addresses keep their original format.
fn format_ipv6_with_interface(raw_addr: &str, iface_name: &str) -> String {
    if raw_addr.is_empty() {
        return String::new();
    }

    // Check if it's a link-local address (starts with fe80::)
    let addr_lower = raw_addr.to_lowercase();
    if addr_lower.starts_with("fe80::") {
        // Parse address and prefix length
        if let Some(slash_idx) = raw_addr.rfind('/') {
            let addr_part = &raw_addr[..slash_idx];
            let prefix_part = &raw_addr[slash_idx + 1..];
            return format!("{}%{}/{}", addr_part, iface_name, prefix_part);
        } else {
            return format!("{}%{}", raw_addr, iface_name);
        }
    }

    // Non-link-local: return as-is
    raw_addr.to_string()
}

/// Formatter for IPv6 link-local addresses output.
pub struct Ipv6LinkLocalFormatter;

impl Ipv6LinkLocalFormatter {
    /// Creates a new [`Ipv6LinkLocalFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6LinkLocalFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6LinkLocalFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Disabled);
        table.load_preset("                   ");

        table.set_header(vec![
            Cell::new("Interface").set_alignment(CellAlignment::Left),
            Cell::new("Link Local Address").set_alignment(CellAlignment::Left),
            Cell::new("State").set_alignment(CellAlignment::Left),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let iface = obj
                        .get("interface")
                        .or_else(|| obj.get("Interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let link_local = obj
                        .get("link_local")
                        .or_else(|| obj.get("Link Local Address"))
                        .or_else(|| obj.get("address"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let state = obj
                        .get("state")
                        .or_else(|| obj.get("State"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&iface).set_alignment(CellAlignment::Left),
                        Cell::new(&link_local).set_alignment(CellAlignment::Left),
                        Cell::new(&state).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut ifaces: Vec<&String> = map.keys().collect();
            ifaces.sort_by(|a, b| natural_cmp(a, b));

            for iface in ifaces {
                if let Some(link_local) = map.get(iface) {
                    let (addr, state) = if let Value::Object(obj) = link_local {
                        (
                            obj.get("address")
                                .or_else(|| obj.get("link_local"))
                                .map(value_to_string)
                                .unwrap_or_default(),
                            obj.get("state").map(value_to_string).unwrap_or_default(),
                        )
                    } else {
                        (value_to_string(link_local), String::new())
                    };

                    table.add_row(vec![
                        Cell::new(iface).set_alignment(CellAlignment::Left),
                        Cell::new(&addr).set_alignment(CellAlignment::Left),
                        Cell::new(&state).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "ipv6_link_local"
    }
}

/// Formatter for IPv6 link-local mode output.
/// Handles arrays with {port, mode} objects, rendered as grid table.
pub struct Ipv6LinkLocalModeFormatter;

impl Ipv6LinkLocalModeFormatter {
    /// Creates a new [`Ipv6LinkLocalModeFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6LinkLocalModeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6LinkLocalModeFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers = ["Interface Name", "Mode"];
        // All columns left-aligned
        let alignments = [false, false];
        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let mode = obj
                        .get("mode")
                        .or_else(|| obj.get("Mode"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    rows.push(vec![port, mode]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for key in keys {
                if let Some(val) = map.get(key) {
                    let mode = if let Value::Object(obj) = val {
                        obj.get("mode")
                            .map(value_to_string)
                            .unwrap_or_else(|| value_to_string(val))
                    } else {
                        value_to_string(val)
                    };
                    rows.push(vec![key.clone(), mode]);
                }
            }
        }

        Ok(tabulate_grid(&headers, &rows, &alignments))
    }

    fn name(&self) -> &'static str {
        "ipv6_link_local_mode"
    }
}

/// Formatter for NDP (Neighbor Discovery Protocol) table output.
pub struct NdpTableFormatter;

impl NdpTableFormatter {
    /// Creates a new [`NdpTableFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for NdpTableFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for NdpTableFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers = ["Address", "MacAddress", "Iface", "Vlan", "Status"];

        // Extract entries array from the data
        // Input can be: {"total_entries": N, "entries": [...]} or just an array
        let entries = if let Value::Object(obj) = data {
            obj.get("entries")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        } else if let Value::Array(arr) = data {
            arr.clone()
        } else {
            Vec::new()
        };

        // Collect entries to sort them
        let mut sorted_entries: Vec<(String, String, String, String, String)> = Vec::new();

        for item in &entries {
            if let Value::Object(obj) = item {
                let address = obj
                    .get("address")
                    .or_else(|| obj.get("Address"))
                    .map(value_to_string)
                    .unwrap_or_default();
                let mac = obj
                    .get("mac_address")
                    .or_else(|| obj.get("macaddress"))
                    .or_else(|| obj.get("MAC Address"))
                    .or_else(|| obj.get("mac"))
                    .map(value_to_string)
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                let iface = obj
                    .get("iface")
                    .or_else(|| obj.get("Interface"))
                    .map(value_to_string)
                    .unwrap_or_default();
                let vlan = obj
                    .get("vlan")
                    .or_else(|| obj.get("Vlan"))
                    .map(value_to_string)
                    .unwrap_or_else(|| "-".to_string());
                let status = obj
                    .get("status")
                    .or_else(|| obj.get("Status"))
                    .or_else(|| obj.get("flags"))
                    .or_else(|| obj.get("Flags"))
                    .or_else(|| obj.get("state"))
                    .map(value_to_string)
                    .unwrap_or_default();

                sorted_entries.push((address, mac, iface, vlan, status));
            }
        }

        // Sort entries by address using natural sort (matches Python's natural_sort_key)
        sorted_entries.sort_by(|a, b| natural_cmp(&a.0, &b.0));

        let rows: Vec<Vec<String>> = sorted_entries
            .iter()
            .map(|(address, mac, iface, vlan, status)| {
                vec![
                    address.clone(),
                    mac.clone(),
                    iface.clone(),
                    vlan.clone(),
                    status.clone(),
                ]
            })
            .collect();

        // Get total entries count
        let total_entries = if let Value::Object(obj) = data {
            obj.get("total_entries")
                .and_then(|v| v.as_i64())
                .unwrap_or(sorted_entries.len() as i64)
        } else {
            sorted_entries.len() as i64
        };

        let mut result = tabulate_simple_auto(&headers, &rows);
        result.push('\n');
        result.push_str(&format!("Total number of entries {} ", total_entries));

        Ok(result)
    }

    fn name(&self) -> &'static str {
        "ndp_table"
    }
}

/// Formatter for OSPFv3 neighbors output.
pub struct Ospfv3NeighborFormatter;

impl Ospfv3NeighborFormatter {
    /// Creates a new [`Ospfv3NeighborFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ospfv3NeighborFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ospfv3NeighborFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Disabled);
        table.load_preset("                   ");

        table.set_header(vec![
            Cell::new("Neighbor ID").set_alignment(CellAlignment::Left),
            Cell::new("Interface").set_alignment(CellAlignment::Left),
            Cell::new("State").set_alignment(CellAlignment::Left),
            Cell::new("DeadTime").set_alignment(CellAlignment::Right),
            Cell::new("Router Priority").set_alignment(CellAlignment::Right),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let neighbor_id = obj
                        .get("neighbor_id")
                        .or_else(|| obj.get("Neighbor ID"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let iface = obj
                        .get("interface")
                        .or_else(|| obj.get("Interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let state = obj
                        .get("state")
                        .or_else(|| obj.get("State"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let dead_time = obj
                        .get("dead_time")
                        .or_else(|| obj.get("DeadTime"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let priority = obj
                        .get("router_priority")
                        .or_else(|| obj.get("Router Priority"))
                        .or_else(|| obj.get("priority"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&neighbor_id).set_alignment(CellAlignment::Left),
                        Cell::new(&iface).set_alignment(CellAlignment::Left),
                        Cell::new(&state).set_alignment(CellAlignment::Left),
                        Cell::new(&dead_time).set_alignment(CellAlignment::Right),
                        Cell::new(&priority).set_alignment(CellAlignment::Right),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "ospfv3_neighbor"
    }
}

/// Formatter for BGPv6 summary output.
pub struct Bgpv6SummaryFormatter;

impl Bgpv6SummaryFormatter {
    /// Creates a new [`Bgpv6SummaryFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Bgpv6SummaryFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Bgpv6SummaryFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut lines = Vec::new();

        let mut data_source = data;
        // Handle nested ipv6Unicast structure if present
        if let Some(ipv6_unicast) = data.get("ipv6Unicast") {
            data_source = ipv6_unicast;
        }

        if let Value::Object(map) = data_source {
            // IPv6 Unicast Summary header
            lines.push(String::new());
            lines.push("IPv6 Unicast Summary:".to_string());

            // Router info with vrf-id
            if let Some(router_id) = map.get("router_id").or_else(|| map.get("routerId")) {
                let as_num = map
                    .get("local_as")
                    .or_else(|| map.get("localAs"))
                    .or_else(|| map.get("as"))
                    .map(value_to_string)
                    .unwrap_or_default();
                let vrf_id = map
                    .get("vrfId")
                    .or_else(|| map.get("vrf_id"))
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());

                lines.push(format!(
                    "BGP router identifier {}, local AS number {} vrf-id {}",
                    value_to_string(router_id),
                    as_num,
                    vrf_id
                ));
            }

            // BGP table version
            if let Some(table_version) =
                map.get("tableVersion").or_else(|| map.get("table_version"))
            {
                lines.push(format!(
                    "BGP table version {}",
                    value_to_string(table_version)
                ));
            }

            // RIB entries
            if let Some(rib_count) = map.get("ribCount").or_else(|| map.get("rib_count")) {
                let rib_memory = map
                    .get("ribMemory")
                    .or_else(|| map.get("rib_memory"))
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());
                lines.push(format!(
                    "RIB entries {}, using {} bytes of memory",
                    value_to_string(rib_count),
                    rib_memory
                ));
            }

            // Peers
            if let Some(peer_count) = map.get("peerCount").or_else(|| map.get("peer_count")) {
                let peer_memory = map
                    .get("peerMemory")
                    .or_else(|| map.get("peer_memory"))
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());
                lines.push(format!(
                    "Peers {}, using {} KiB of memory",
                    value_to_string(peer_count),
                    peer_memory
                ));
            }

            // Peer groups
            if let Some(pg_count) = map
                .get("peerGroupCount")
                .or_else(|| map.get("peer_group_count"))
            {
                let pg_memory = map
                    .get("peerGroupMemory")
                    .or_else(|| map.get("peer_group_memory"))
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());
                lines.push(format!(
                    "Peer groups {}, using {} bytes of memory",
                    value_to_string(pg_count),
                    pg_memory
                ));
            }

            // Two blank lines before table
            lines.push(String::new());
            lines.push(String::new());

            // Collect peer data into structured format for dynamic column width calculation
            let mut peer_data: Vec<BgpPeerRowData> = Vec::new();

            // Handle neighbors/peers from array or map (peers key)
            if let Some(Value::Array(arr)) =
                map.get("neighbors").or_else(|| map.get("ipv6Neighbors"))
            {
                for item in arr {
                    if let Value::Object(obj) = item {
                        let addr = obj
                            .get("neighbor")
                            .or_else(|| obj.get("Neighbor"))
                            .or_else(|| obj.get("address"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        peer_data.push(extract_bgp_peer_data(&addr, obj));
                    }
                }
            } else if let Some(Value::Object(peers)) = map.get("peers") {
                // Sort by key using plain alphabetical sort (matching Python's sorted())
                let mut peer_keys: Vec<&String> = peers.keys().collect();
                peer_keys.sort();
                for key in peer_keys {
                    if let Some(Value::Object(peer_obj)) = peers.get(key) {
                        peer_data.push(extract_bgp_peer_data(key, peer_obj));
                    }
                }
            }

            // Peers from neighbors array are already in insertion order;
            // for peers map, we sorted above. No further sorting needed.

            // Calculate dynamic column widths based on data
            // Column headers: Neighbhor, V, AS, MsgRcvd, MsgSent, TblVer, InQ, OutQ, Up/Down, State/PfxRcd, NeighborName
            let mut widths = [11usize, 3, 5, 9, 9, 8, 5, 6, 9, 14, 14]; // minimum widths from headers

            // Calculate max widths from data
            for p in &peer_data {
                widths[0] = widths[0].max(p.addr.len());
                widths[1] = widths[1].max(p.version.len());
                widths[2] = widths[2].max(p.remote_as.len());
                widths[3] = widths[3].max(p.msg_rcvd.len());
                widths[4] = widths[4].max(p.msg_sent.len());
                widths[5] = widths[5].max(p.tbl_ver.len());
                widths[6] = widths[6].max(p.in_q.len());
                widths[7] = widths[7].max(p.out_q.len());
                widths[8] = widths[8].max(p.up_down.len());
                widths[9] = widths[9].max(p.state_pfx.len());
                widths[10] = widths[10].max(p.neighbor_name.len());
            }

            // Generate header row with dynamic widths
            // Alignment: Neighbhor (left), V-OutQ (right), Up/Down (left), State/PfxRcd (right), NeighborName (left)
            lines.push(format!(
                "{:<w0$}  {:>w1$}  {:>w2$}  {:>w3$}  {:>w4$}  {:>w5$}  {:>w6$}  {:>w7$}  {:<w8$}  {:>w9$}  {}",
                "Neighbhor", "V", "AS", "MsgRcvd", "MsgSent", "TblVer", "InQ", "OutQ", "Up/Down", "State/PfxRcd", "NeighborName",
                w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4],
                w5 = widths[5], w6 = widths[6], w7 = widths[7], w8 = widths[8], w9 = widths[9]
            ));

            // Generate separator line
            lines.push(format!(
                "{:-<w0$}  {:-<w1$}  {:-<w2$}  {:-<w3$}  {:-<w4$}  {:-<w5$}  {:-<w6$}  {:-<w7$}  {:-<w8$}  {:-<w9$}  {:-<w10$}",
                "", "", "", "", "", "", "", "", "", "", "",
                w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4],
                w5 = widths[5], w6 = widths[6], w7 = widths[7], w8 = widths[8], w9 = widths[9], w10 = widths[10]
            ));

            // Add data rows
            // Alignment: addr (left), version-out_q (right), up_down (left), state_pfx (right), neighbor_name (left)
            for p in &peer_data {
                lines.push(format!(
                    "{:<w0$}  {:>w1$}  {:>w2$}  {:>w3$}  {:>w4$}  {:>w5$}  {:>w6$}  {:>w7$}  {:<w8$}  {:>w9$}  {}",
                    p.addr, p.version, p.remote_as, p.msg_rcvd, p.msg_sent, p.tbl_ver,
                    p.in_q, p.out_q, p.up_down, p.state_pfx, p.neighbor_name,
                    w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4],
                    w5 = widths[5], w6 = widths[6], w7 = widths[7], w8 = widths[8], w9 = widths[9]
                ));
            }

            // No footer - matches Python behavior (Python doesn't output "Total number of neighbors")
        }

        if lines.is_empty() {
            if let Value::String(s) = data {
                return Ok(s.clone());
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "bgpv6_summary"
    }
}

/// Data structure for a BGP peer row
struct BgpPeerRowData {
    addr: String,
    version: String,
    remote_as: String,
    msg_rcvd: String,
    msg_sent: String,
    tbl_ver: String,
    in_q: String,
    out_q: String,
    up_down: String,
    state_pfx: String,
    neighbor_name: String,
}

/// Extract BGP peer data from a JSON object
fn extract_bgp_peer_data(addr: &str, obj: &serde_json::Map<String, Value>) -> BgpPeerRowData {
    let version = obj
        .get("version")
        .map(value_to_string)
        .unwrap_or_else(|| "4".to_string());
    let remote_as = obj
        .get("remote_as")
        .or_else(|| obj.get("AS"))
        .or_else(|| obj.get("remoteAs"))
        .map(value_to_string)
        .unwrap_or_default();
    let msg_rcvd = obj
        .get("msg_rcvd")
        .or_else(|| obj.get("msgRcvd"))
        .map(value_to_string)
        .unwrap_or_default();
    let msg_sent = obj
        .get("msg_sent")
        .or_else(|| obj.get("msgSent"))
        .map(value_to_string)
        .unwrap_or_default();
    let tbl_ver = obj
        .get("table_version")
        .or_else(|| obj.get("TblVer"))
        .or_else(|| obj.get("tableVersion"))
        .map(value_to_string)
        .unwrap_or_default();
    let in_q = obj
        .get("in_q")
        .or_else(|| obj.get("InQ"))
        .or_else(|| obj.get("inq"))
        .map(value_to_string)
        .unwrap_or_else(|| "0".to_string());
    let out_q = obj
        .get("out_q")
        .or_else(|| obj.get("OutQ"))
        .or_else(|| obj.get("outq"))
        .map(value_to_string)
        .unwrap_or_else(|| "0".to_string());
    let up_down = obj
        .get("up_down")
        .or_else(|| obj.get("Up/Down"))
        .or_else(|| obj.get("upDown"))
        .or_else(|| obj.get("uptime"))
        .or_else(|| obj.get("peerUptime"))
        .map(value_to_string)
        .unwrap_or_default();

    // Handle state/pfxRcd — match Python: peer.get("pfxRcd", peer.get("state", ""))
    // If pfxRcd is present (even if "0"), use it. Otherwise fall back to state.
    let pfx_rcd = obj.get("pfxRcd").map(value_to_string);
    let raw_state = obj.get("state").map(value_to_string);
    let state_pfx = if let Some(pfx) = pfx_rcd {
        pfx
    } else if let Some(st) = raw_state {
        st
    } else {
        obj.get("state_pfx_rcd")
            .or_else(|| obj.get("State/PfxRcd"))
            .or_else(|| obj.get("statePfxRcd"))
            .map(value_to_string)
            .unwrap_or_default()
    };
    let neighbor_name = obj
        .get("neighbor_name")
        .or_else(|| obj.get("NeighborName"))
        .or_else(|| obj.get("neighborName"))
        .or_else(|| obj.get("hostname"))
        .or_else(|| obj.get("peerDesc"))
        .or_else(|| obj.get("description"))
        .map(value_to_string)
        .unwrap_or_default();

    BgpPeerRowData {
        addr: addr.to_string(),
        version,
        remote_as,
        msg_rcvd,
        msg_sent,
        tbl_ver,
        in_q,
        out_q,
        up_down,
        state_pfx,
        neighbor_name,
    }
}

/// Formatter for DHCPv6 relay output.
pub struct Dhcpv6RelayFormatter;

impl Dhcpv6RelayFormatter {
    /// Creates a new [`Dhcpv6RelayFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Dhcpv6RelayFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Dhcpv6RelayFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Disabled);
        table.load_preset("                   ");

        table.set_header(vec![
            Cell::new("Interface").set_alignment(CellAlignment::Left),
            Cell::new("VLAN").set_alignment(CellAlignment::Left),
            Cell::new("IPv6 Relay Servers").set_alignment(CellAlignment::Left),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let iface = obj
                        .get("interface")
                        .or_else(|| obj.get("Interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let vlan = obj
                        .get("vlan")
                        .or_else(|| obj.get("VLAN"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let servers = obj
                        .get("servers")
                        .or_else(|| obj.get("IPv6 Relay Servers"))
                        .map(|v| {
                            if let Value::Array(arr) = v {
                                arr.iter()
                                    .map(value_to_string)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            } else {
                                value_to_string(v)
                            }
                        })
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&iface).set_alignment(CellAlignment::Left),
                        Cell::new(&vlan).set_alignment(CellAlignment::Left),
                        Cell::new(&servers).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut ifaces: Vec<&String> = map.keys().collect();
            ifaces.sort_by(|a, b| natural_cmp(a, b));

            for iface in ifaces {
                if let Some(Value::Object(obj)) = map.get(iface) {
                    let vlan = obj.get("vlan").map(value_to_string).unwrap_or_default();
                    let servers = obj
                        .get("servers")
                        .map(|v| {
                            if let Value::Array(arr) = v {
                                arr.iter()
                                    .map(value_to_string)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            } else {
                                value_to_string(v)
                            }
                        })
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(iface).set_alignment(CellAlignment::Left),
                        Cell::new(&vlan).set_alignment(CellAlignment::Left),
                        Cell::new(&servers).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "dhcpv6_relay"
    }
}

/// Formatter for ICMPv6 statistics output.
pub struct Icmpv6StatsFormatter;

impl Icmpv6StatsFormatter {
    /// Creates a new [`Icmpv6StatsFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Icmpv6StatsFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Icmpv6StatsFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut lines = Vec::new();

        if let Value::Object(map) = data {
            if let Some(output) = map.get("output") {
                return Ok(value_to_string(output));
            }

            // Format ICMPv6 stats
            if let Some(Value::Object(rx)) = map.get("rx").or_else(|| map.get("received")) {
                lines.push("ICMPv6 Received:".to_string());
                for (key, val) in rx {
                    lines.push(format!("  {}: {}", key, value_to_string(val)));
                }
            }

            if let Some(Value::Object(tx)) = map.get("tx").or_else(|| map.get("transmitted")) {
                lines.push("\nICMPv6 Transmitted:".to_string());
                for (key, val) in tx {
                    lines.push(format!("  {}: {}", key, value_to_string(val)));
                }
            }

            // Handle flat structure
            if lines.is_empty() {
                for (key, val) in map {
                    lines.push(format!("{}: {}", key, value_to_string(val)));
                }
            }
        }

        if lines.is_empty() {
            if let Value::String(s) = data {
                return Ok(s.clone());
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "icmpv6_stats"
    }
}

/// Formatter for IPv6 BGP Network output.
pub struct Ipv6BgpNetworkFormatter;

impl Ipv6BgpNetworkFormatter {
    /// Creates a new [`Ipv6BgpNetworkFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6BgpNetworkFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6BgpNetworkFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            if let Some(Value::String(output)) = map.get("output") {
                return Ok(output.clone());
            }
        }
        Ok(value_to_string(data))
    }

    fn name(&self) -> &'static str {
        "ipv6_bgp_network"
    }
}

/// Formatter for IPv6 FIB output.
pub struct Ipv6FibFormatter;

impl Ipv6FibFormatter {
    /// Creates a new [`Ipv6FibFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6FibFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6FibFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let map = match data {
            Value::Object(map) => map,
            _ => return Ok(String::new()),
        };

        let entries = match map.get("entries") {
            Some(Value::Array(entries)) => entries,
            _ => return Ok(String::new()),
        };

        let headers = ["No.", "Vrf", "Route", "Nexthop", "Ifname"];

        let rows: Vec<Vec<String>> = entries
            .iter()
            .filter_map(|entry| {
                if let Value::Object(obj) = entry {
                    Some(vec![
                        obj.get("index").map(value_to_string).unwrap_or_default(),
                        obj.get("vrf").map(value_to_string).unwrap_or_default(),
                        obj.get("route").map(value_to_string).unwrap_or_default(),
                        obj.get("nexthop").map(value_to_string).unwrap_or_default(),
                        obj.get("ifname").map(value_to_string).unwrap_or_default(),
                    ])
                } else {
                    None
                }
            })
            .collect();

        let mut output = tabulate_simple_auto(&headers, &rows);
        let total = map
            .get("total")
            .map(value_to_string)
            .unwrap_or_else(|| entries.len().to_string());
        output.push_str(&format!("\nTotal number of entries {}", total));

        Ok(output)
    }

    fn name(&self) -> &'static str {
        "ipv6_fib"
    }
}

/// Formatter for IPv6 prefix-list output.
pub struct Ipv6PrefixListFormatter;

impl Ipv6PrefixListFormatter {
    /// Creates a new [`Ipv6PrefixListFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6PrefixListFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6PrefixListFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            if let Some(Value::String(output)) = map.get("output") {
                return Ok(output.clone());
            }
        }

        let map = match data {
            Value::Object(map) => map,
            _ => return Ok(value_to_string(data)),
        };

        let mut lines = Vec::new();
        let mut protocols = map.keys().cloned().collect::<Vec<_>>();
        protocols.sort_by_key(|value| if value == "ZEBRA" { 0 } else { 1 });

        for protocol in protocols {
            if let Some(Value::Object(protocol_data)) = map.get(&protocol) {
                let mut list_names = protocol_data.keys().cloned().collect::<Vec<_>>();
                list_names.sort();
                for pl_name in list_names {
                    if let Some(Value::Object(pl_data)) = protocol_data.get(&pl_name) {
                        let entries = pl_data
                            .get("entries")
                            .and_then(Value::as_array)
                            .cloned()
                            .unwrap_or_default();
                        lines.push(format!(
                            "{protocol}: ipv6 prefix-list {pl_name}: {} entries",
                            entries.len()
                        ));
                        for entry in entries {
                            if let Value::Object(entry_map) = entry {
                                let seq = entry_map
                                    .get("sequenceNumber")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let action = entry_map
                                    .get("type")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let prefix = entry_map
                                    .get("prefix")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let ge = entry_map
                                    .get("minimumPrefixLength")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let le = entry_map
                                    .get("maximumPrefixLength")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let mut entry_line = format!("   seq {seq} {action} {prefix}");
                                if !ge.is_empty() {
                                    entry_line.push_str(&format!(" ge {ge}"));
                                }
                                if !le.is_empty() {
                                    entry_line.push_str(&format!(" le {le}"));
                                }
                                lines.push(entry_line);
                            }
                        }
                    }
                }
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "ipv6_prefix_list"
    }
}

/// Formatter for IPv6 protocol output.
pub struct Ipv6ProtocolFormatter;

impl Ipv6ProtocolFormatter {
    /// Creates a new [`Ipv6ProtocolFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for Ipv6ProtocolFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6ProtocolFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            if let Some(Value::String(output)) = map.get("output") {
                return Ok(output.clone());
            }
        }

        if let Value::Array(items) = data {
            let mut lines = Vec::new();
            for item in items {
                if let Value::Object(map) = item {
                    let vrf = map
                        .get("VRF")
                        .map(value_to_string)
                        .unwrap_or_else(|| "default".to_string());
                    lines.push(format!("VRF: {vrf}"));
                    lines.push("Protocol                  : route-map".to_string());
                    lines.push("-------------------------------------".to_string());
                    if let Some(Value::Array(protocols)) = map.get("Protocols") {
                        for proto in protocols {
                            if let Value::Object(proto_map) = proto {
                                let name = proto_map
                                    .get("Protocol")
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let route_map = proto_map
                                    .get("route-map")
                                    .map(value_to_string)
                                    .unwrap_or_else(|| "none".to_string());
                                lines.push(format!("{name:<26}: {route_map}"));
                            }
                        }
                    }
                }
            }
            return Ok(lines.join("\n"));
        }

        Ok(value_to_string(data))
    }

    fn name(&self) -> &'static str {
        "ipv6_protocol"
    }
}

/// Formatter for IPv6 routes output.
pub struct Ipv6RouteFormatter;

impl Ipv6RouteFormatter {
    /// Creates a new [`Ipv6RouteFormatter`].
    pub fn new() -> Self {
        Self
    }

    fn prefix_sort_key(prefix: &str) -> (i32, Vec<u8>, u32) {
        if let Some((addr, len)) = prefix.split_once('/') {
            if let Ok(ip) = Ipv6Addr::from_str(addr) {
                return (0, ip.octets().to_vec(), len.parse::<u32>().unwrap_or(0));
            }
        }
        if let Ok(ip) = Ipv6Addr::from_str(prefix) {
            return (0, ip.octets().to_vec(), 0);
        }
        (1, prefix.as_bytes().to_vec(), 0)
    }

    fn format_nexthop(nh: &serde_json::Map<String, Value>) -> String {
        let active = nh.get("active").and_then(Value::as_bool).unwrap_or(false)
            || nh.get("fib").and_then(Value::as_bool).unwrap_or(false);
        let flag = if active { "*" } else { " " };
        if nh.get("directlyConnected").and_then(Value::as_bool) == Some(true) {
            let interface = nh
                .get("interfaceName")
                .map(value_to_string)
                .unwrap_or_default();
            return format!("  {flag} directly connected, {interface}");
        }
        let ip = nh
            .get("ip")
            .or_else(|| nh.get("nexthop"))
            .map(value_to_string)
            .unwrap_or_default();
        let interface = nh
            .get("interfaceName")
            .map(value_to_string)
            .unwrap_or_default();
        if !ip.is_empty() {
            if !interface.is_empty() {
                format!("  {flag} via {ip}, {interface}")
            } else {
                format!("  {flag} via {ip}")
            }
        } else if !interface.is_empty() {
            format!("  {flag} directly connected, {interface}")
        } else {
            String::new()
        }
    }

    fn format_route_entry(prefix: &str, route: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Routing entry for {prefix}"));
        let protocol = route
            .get("protocol")
            .map(value_to_string)
            .unwrap_or_else(|| "unknown".to_string());
        let distance = route
            .get("distance")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let metric = route
            .get("metric")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let is_best = route.get("selected").and_then(Value::as_bool) == Some(true)
            || route.get("destSelected").and_then(Value::as_bool) == Some(true);
        let best_suffix = if is_best { ", best" } else { "" };
        lines.push(format!(
            "  Known via \"{protocol}\", distance {distance}, metric {metric}{best_suffix}"
        ));
        if let Some(uptime) = route.get("uptime") {
            let text = value_to_string(uptime);
            if !text.is_empty() {
                lines.push(format!("  Last update {text} ago"));
            }
        }
        if let Some(Value::Array(nexthops)) = route.get("nexthops") {
            for nh in nexthops {
                if let Value::Object(nh_map) = nh {
                    let line = Self::format_nexthop(nh_map);
                    if !line.is_empty() {
                        lines.push(line);
                    }
                }
            }
        }
        lines
    }
}

impl Default for Ipv6RouteFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6RouteFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            if let Some(Value::String(output)) = map.get("output") {
                return Ok(output.clone());
            }
        }

        let map = match data {
            Value::Object(map) => map,
            _ => return Ok(value_to_string(data)),
        };

        let mut lines = Vec::new();
        let mut prefixes = map.keys().cloned().collect::<Vec<_>>();
        prefixes.sort_by_key(|prefix| Self::prefix_sort_key(prefix));
        for prefix in prefixes {
            let routes = map.get(&prefix).cloned().unwrap_or(Value::Null);
            let routes = match routes {
                Value::Object(map) => vec![Value::Object(map)],
                Value::Array(list) => list,
                _ => Vec::new(),
            };
            for route in routes {
                if let Value::Object(route_map) = route {
                    lines.extend(Self::format_route_entry(&prefix, &route_map));
                    lines.push(String::new());
                    lines.push(String::new());
                }
            }
        }

        Ok(lines.join("\n").trim_end().to_string())
    }

    fn name(&self) -> &'static str {
        "ipv6_route"
    }
}

/// Formatter for IPv6 BGP neighbors output.
/// Handles detailed neighbor info and route table views (advertised-routes, received-routes, routes).
pub struct Ipv6BgpNeighborsFormatter;

impl Ipv6BgpNeighborsFormatter {
    /// Creates a new [`Ipv6BgpNeighborsFormatter`].
    pub fn new() -> Self {
        Self
    }

    /// Format milliseconds as HH:MM:SS.
    fn format_time(msecs: i64) -> String {
        let seconds = msecs / 1000;
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }

    /// Format milliseconds as uptime string like '1d00h33m' or '04:43:30'.
    fn format_uptime(msecs: i64) -> String {
        if msecs <= 0 {
            return "never".to_string();
        }
        let seconds = msecs / 1000;
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        if days > 0 {
            format!("{}d{:02}h{:02}m", days, hours, minutes)
        } else {
            let secs = seconds % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, secs)
        }
    }

    /// Format address family name.
    fn format_af_name(af_key: &str) -> &'static str {
        match af_key {
            "ipv6Unicast" => "IPv6 Unicast",
            "ipv4Unicast" => "IPv4 Unicast",
            _ => "Unknown",
        }
    }

    /// Convert camelCase string to space-separated lowercase.
    /// e.g., "sharedNetwork" -> "shared network"
    #[allow(dead_code)]
    fn camel_to_space(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push(' ');
                result.push(ch.to_lowercase().next().unwrap_or(ch));
            } else {
                result.push(ch);
            }
        }
        result
    }

    /// Parse an IPv6 address string into a u128 for binary comparison.
    fn parse_ipv6_to_u128(addr: &str) -> u128 {
        // Try standard Rust parse first
        if let Ok(ip) = addr.parse::<std::net::Ipv6Addr>() {
            return u128::from(ip);
        }
        // Fallback: return 0 for unparseable addresses
        0
    }

    /// Format capability value.
    fn format_cap_value(value: &Value) -> String {
        match value {
            Value::Object(obj) => {
                if obj
                    .get("advertisedAndReceived")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    "advertised and received".to_string()
                } else if obj
                    .get("advertised")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    "advertised".to_string()
                } else if obj
                    .get("received")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    "received".to_string()
                } else {
                    "advertised and received".to_string()
                }
            }
            Value::String(s) => match s.as_str() {
                "advertisedAndReceived" => "advertised and received".to_string(),
                "advertised" => "advertised".to_string(),
                "received" => "received".to_string(),
                _ => s.clone(),
            },
            Value::Bool(true) => "advertised and received".to_string(),
            _ => value_to_string(value),
        }
    }

    /// Format neighbor capabilities.
    fn format_capabilities(caps: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut lines = Vec::new();

        // 4 byte AS
        if let Some(four_byte) = caps.get("4byteAs") {
            lines.push(format!(
                "    4 Byte AS: {}",
                Self::format_cap_value(four_byte)
            ));
        }

        // Extended message
        if let Some(ext_msg) = caps.get("extendedMessage") {
            lines.push(format!(
                "    Extended Message: {}",
                value_to_string(ext_msg)
            ));
        }

        // Add Path
        if let Some(Value::Object(add_path)) = caps.get("addPath") {
            lines.push("    AddPath:".to_string());
            for (af, af_data) in add_path {
                if let Value::Object(obj) = af_data {
                    if obj
                        .get("rxAdvertisedAndReceived")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        lines.push(format!(
                            "      {}: RX advertised and received",
                            Self::format_af_name(af)
                        ));
                    }
                }
            }
        }

        // Long-lived GR
        if let Some(llgr) = caps.get("longLivedGracefulRestart") {
            lines.push(format!(
                "    Long-lived Graceful Restart: {}",
                value_to_string(llgr)
            ));
        }

        // Route refresh
        if let Some(route_ref) = caps.get("routeRefresh") {
            lines.push(format!(
                "    Route refresh: {}",
                Self::format_cap_value(route_ref)
            ));
        }

        // Enhanced route refresh
        if let Some(enh_route_ref) = caps.get("enhancedRouteRefresh") {
            lines.push(format!(
                "    Enhanced Route Refresh: {}",
                Self::format_cap_value(enh_route_ref)
            ));
        }

        // Multiprotocol extensions
        if let Some(Value::Object(mp_ext)) = caps.get("multiprotocolExtensions") {
            for (af, status) in mp_ext {
                lines.push(format!(
                    "    Address Family {}: {}",
                    Self::format_af_name(af),
                    Self::format_cap_value(status)
                ));
            }
        }

        // Hostname capability
        if let Some(Value::Object(hostname)) = caps.get("hostName") {
            let local_name = hostname
                .get("advHostName")
                .map(value_to_string)
                .unwrap_or_default();
            let domain = hostname
                .get("advDomainName")
                .map(value_to_string)
                .unwrap_or_else(|| "n/a".to_string());
            let received = hostname
                .get("rcvHostName")
                .map(value_to_string)
                .unwrap_or_default();
            let recv_str = if !received.is_empty() {
                "received"
            } else {
                "not received"
            };
            if !local_name.is_empty() {
                lines.push(format!(
                    "    Hostname Capability: advertised (name: {},domain name: {}) {}",
                    local_name, domain, recv_str
                ));
            }
        }

        // Software version capability — only emit if value is a non-empty object (matches Python)
        if let Some(Value::Object(sw_ver)) = caps.get("softwareVersion") {
            if !sw_ver.is_empty() {
                lines.push("    Version Capability: not advertised not received".to_string());
            }
        }

        // Graceful restart capability
        if let Some(gr_cap) = caps.get("gracefulRestart") {
            lines.push(format!(
                "    Graceful Restart Capability: {}",
                Self::format_cap_value(gr_cap)
            ));
            if let Some(remote_timer_msecs) = caps
                .get("gracefulRestartRemoteTimerMsecs")
                .and_then(Value::as_i64)
            {
                let remote_timer = remote_timer_msecs / 1000;
                lines.push(format!(
                    "      Remote Restart timer is {} seconds",
                    remote_timer
                ));
            }

            // Address families by peer
            if let Some(af_by_peer_val) = caps.get("addressFamiliesByPeer") {
                match af_by_peer_val {
                    Value::Object(af_by_peer) => {
                        lines.push("      Address families by peer:".to_string());
                        if af_by_peer.is_empty() {
                            lines.push("        none".to_string());
                        } else {
                            for af in af_by_peer.keys() {
                                lines.push(format!(
                                    "        {}(not preserved)",
                                    Self::format_af_name(af)
                                ));
                            }
                        }
                    }
                    Value::String(s) => {
                        lines.push("      Address families by peer:".to_string());
                        lines.push(format!("        {}", s));
                    }
                    _ => {
                        lines.push("      Address families by peer:".to_string());
                        lines.push(format!("        {}", value_to_string(af_by_peer_val)));
                    }
                }
            }
        }

        lines
    }

    /// Format graceful restart information.
    fn format_graceful_restart(gr_info: &serde_json::Map<String, Value>) -> Vec<String> {
        let mut lines = vec!["  Graceful restart information:".to_string()];

        // End-of-RIB info
        if let Some(Value::Object(eor_send)) = gr_info.get("endOfRibSend") {
            for af in eor_send.keys() {
                lines.push(format!("    End-of-RIB send: {}", Self::format_af_name(af)));
            }
        }
        if let Some(Value::Object(eor_recv)) = gr_info.get("endOfRibRecv") {
            for af in eor_recv.keys() {
                lines.push(format!(
                    "    End-of-RIB received: {}",
                    Self::format_af_name(af)
                ));
            }
        }

        // Local/Remote GR mode
        if let Some(local_mode) = gr_info.get("localGrMode").map(value_to_string) {
            lines.push(format!("    Local GR Mode: {}", local_mode));
        }
        lines.push(String::new());

        if let Some(remote_mode) = gr_info.get("remoteGrMode").map(value_to_string) {
            lines.push(format!("    Remote GR Mode: {}", remote_mode));
        }
        lines.push(String::new());

        // R bit and N bit
        let r_bit = gr_info
            .get("rBit")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let n_bit = gr_info
            .get("nBit")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        lines.push(format!(
            "    R bit: {}",
            if r_bit { "True" } else { "False" }
        ));
        lines.push(format!(
            "    N bit: {}",
            if n_bit { "True" } else { "False" }
        ));

        // Timers
        if let Some(Value::Object(timers)) = gr_info.get("timers") {
            lines.push("    Timers:".to_string());
            let cfg_restart = timers
                .get("configuredRestartTimer")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let rcv_restart = timers
                .get("receivedRestartTimer")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let cfg_llgr = timers
                .get("configuredLlgrStaleTime")
                .and_then(Value::as_i64)
                .unwrap_or(0);
            lines.push(format!(
                "      Configured Restart Time(sec): {}",
                cfg_restart
            ));
            lines.push(format!("      Received Restart Time(sec): {}", rcv_restart));
            lines.push(format!(
                "      Configured LLGR Stale Path Time(sec): {}",
                cfg_llgr
            ));
        }

        // Per-AFI graceful restart info
        for af_key in &["ipv6Unicast", "ipv4Unicast"] {
            if let Some(Value::Object(af_gr)) = gr_info.get(*af_key) {
                lines.push(format!("    {}:", Self::format_af_name(af_key)));
                let f_bit = af_gr.get("fBit").and_then(Value::as_bool).unwrap_or(false);
                lines.push(format!(
                    "      F bit: {}",
                    if f_bit { "True" } else { "False" }
                ));

                if let Some(Value::Object(eor_status)) = af_gr.get("endOfRibStatus") {
                    let eor_sent = if eor_status
                        .get("endOfRibSend")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        "Yes"
                    } else {
                        "No"
                    };
                    let eor_sent_after = if eor_status
                        .get("endOfRibSentAfterUpdate")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        "Yes"
                    } else {
                        "No"
                    };
                    let eor_recv = if eor_status
                        .get("endOfRibRecv")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        "Yes"
                    } else {
                        "No"
                    };
                    lines.push(format!("      End-of-RIB sent: {}", eor_sent));
                    lines.push(format!(
                        "      End-of-RIB sent after update: {}",
                        eor_sent_after
                    ));
                    lines.push(format!("      End-of-RIB received: {}", eor_recv));
                }

                if let Some(Value::Object(af_timers)) = af_gr.get("timers") {
                    lines.push("      Timers:".to_string());
                    let stale_path = af_timers
                        .get("stalePathTimer")
                        .and_then(Value::as_i64)
                        .unwrap_or(0);
                    let llgr_stale = af_timers
                        .get("llgrStaleTime")
                        .and_then(Value::as_i64)
                        .unwrap_or(0);
                    lines.push(format!(
                        "        Configured Stale Path Time(sec): {}",
                        stale_path
                    ));
                    lines.push(format!("        LLGR Stale Path Time(sec): {}", llgr_stale));
                }
            }
        }

        lines
    }

    /// Format address family information.
    fn format_address_family(
        af_name: &str,
        af_data: &serde_json::Map<String, Value>,
        _peer_group: &str,
    ) -> Vec<String> {
        let mut lines = vec![format!(
            " For address family: {}",
            Self::format_af_name(af_name)
        )];

        // Peer group member
        if let Some(peer_group_member) = af_data.get("peerGroupMember").map(value_to_string) {
            if !peer_group_member.is_empty() {
                lines.push(format!("  {} peer-group member", peer_group_member));
            }
        }

        // Update group, subgroup
        if let Some(update_group) = af_data.get("updateGroupId") {
            let subgroup = af_data
                .get("subGroupId")
                .map(value_to_string)
                .unwrap_or_default();
            lines.push(format!(
                "  Update group {}, subgroup {}",
                value_to_string(update_group),
                subgroup
            ));
        }

        // Packet queue length
        let pkt_queue = af_data
            .get("packetQueueLength")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        lines.push(format!("  Packet Queue length {}", pkt_queue));

        // Inbound soft reconfig
        if af_data
            .get("inboundSoftConfigPermit")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            lines.push("  Inbound soft reconfiguration allowed".to_string());
        }

        // Allow AS in
        if let Some(allow_as_in) = af_data.get("allowAsInCount").and_then(Value::as_i64) {
            if allow_as_in > 0 {
                lines.push(format!(
                    "  Local AS allowed in path, {} occurrences",
                    allow_as_in
                ));
            }
        }

        // Community attribute sent to neighbor
        if let Some(comm_attr) = af_data.get("commAttriSentToNbr").map(value_to_string) {
            if !comm_attr.is_empty() {
                lines.push(format!(
                    "  Community attribute sent to this neighbor({})",
                    comm_attr
                ));
            }
        }

        // Path policy config
        if af_data
            .get("inboundPathPolicyConfig")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            lines.push("  Inbound path policy configured".to_string());
        }
        if af_data
            .get("outboundPathPolicyConfig")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            lines.push("  Outbound path policy configured".to_string());
        }

        // Route maps
        if let Some(in_routemap) = af_data
            .get("routeMapForIncomingAdvertisements")
            .map(value_to_string)
        {
            if !in_routemap.is_empty() {
                lines.push(format!(
                    "  Route map for incoming advertisements is *{}",
                    in_routemap
                ));
            }
        }
        if let Some(out_routemap) = af_data
            .get("routeMapForOutgoingAdvertisements")
            .map(value_to_string)
        {
            if !out_routemap.is_empty() {
                lines.push(format!(
                    "  Route map for outgoing advertisements is *{}",
                    out_routemap
                ));
            }
        }

        // Accepted and sent prefixes
        let accepted = af_data
            .get("acceptedPrefixCounter")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let sent = af_data
            .get("sentPrefixCounter")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        lines.push(format!(
            "  {} accepted prefixes, {} sent prefixes",
            accepted, sent
        ));

        // Maximum prefixes
        if let Some(max_prefix) = af_data.get("prefixAllowedMax").and_then(Value::as_i64) {
            if max_prefix > 0 {
                let warning_only = af_data
                    .get("prefixAllowedMaxWarning")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let warn_str = if warning_only { "(warning-only)" } else { "" };
                lines.push(format!(
                    "  Maximum prefixes allowed {} {}",
                    max_prefix, warn_str
                ));
                if let Some(warning_thresh) = af_data
                    .get("prefixAllowedWarningThresh")
                    .and_then(Value::as_i64)
                {
                    if warning_thresh > 0 {
                        lines.push(format!(
                            "  Threshold for warning message {}%",
                            warning_thresh
                        ));
                    }
                }
            }
        }

        lines.push(String::new());

        lines
    }

    /// Format a single neighbor's detailed output.
    fn format_neighbor(neighbor_ip: &str, data: &serde_json::Map<String, Value>) -> String {
        let mut lines = Vec::new();

        let remote_as = data
            .get("remoteAs")
            .map(value_to_string)
            .unwrap_or_else(|| "N/A".to_string());
        let local_as = data
            .get("localAs")
            .map(value_to_string)
            .unwrap_or_else(|| "N/A".to_string());
        let is_external = data
            .get("nbrExternalLink")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let link_type = if is_external {
            "external link"
        } else {
            "confed-external link"
        };

        lines.push(format!(
            "BGP neighbor is {}, remote AS {}, local AS {}, {}",
            neighbor_ip, remote_as, local_as, link_type
        ));

        let local_role = data
            .get("localRole")
            .map(value_to_string)
            .unwrap_or_else(|| "undefined".to_string());
        let remote_role = data
            .get("remoteRole")
            .map(value_to_string)
            .unwrap_or_else(|| "undefined".to_string());
        lines.push(format!("  Local Role: {}", local_role));
        lines.push(format!("  Remote Role: {}", remote_role));

        if let Some(desc) = data.get("nbrDesc").and_then(Value::as_str) {
            if !desc.is_empty() {
                lines.push(format!(" Description: {}", desc));
            }
        }

        if let Some(peer_group) = data.get("peerGroup").and_then(Value::as_str) {
            if !peer_group.is_empty() {
                lines.push(format!(
                    " Member of peer-group {} for session parameters",
                    peer_group
                ));
            }
        }

        let bgp_version = data
            .get("bgpVersion")
            .map(value_to_string)
            .unwrap_or_else(|| "4".to_string());
        let remote_router_id = data
            .get("remoteRouterId")
            .map(value_to_string)
            .unwrap_or_default();
        let local_router_id = data
            .get("localRouterId")
            .map(value_to_string)
            .unwrap_or_default();
        lines.push(format!(
            "  BGP version {}, remote router ID {}, local router ID {}",
            bgp_version, remote_router_id, local_router_id
        ));

        if !is_external {
            lines.push("  Neighbor under common administration".to_string());
        }

        if let Some(hostname) = data.get("hostname").and_then(Value::as_str) {
            if !hostname.is_empty() && hostname != "Unknown" {
                lines.push(format!("  Hostname: {}", hostname));
            }
        }

        if let Some(sw_version) = data.get("softwareVersion").and_then(Value::as_str) {
            if !sw_version.is_empty() && sw_version != "n/a" {
                lines.push(format!("  Software Version: {}", sw_version));
            }
        }

        if let Some(bgp_state) = data.get("bgpState").and_then(Value::as_str) {
            let up_time = data
                .get("bgpTimerUpString")
                .map(value_to_string)
                .unwrap_or_default();
            // Python suppresses the raw epoch timestamp — only show "up for <duration>"
            lines.push(format!("  BGP state = {}, up for {}", bgp_state, up_time));
        }

        // bgpInUpdateElapsedTimeMsecs — must come BEFORE "Last read" (matching Python order)
        if let Some(update_elapsed) = data
            .get("bgpInUpdateElapsedTimeMsecs")
            .and_then(Value::as_i64)
        {
            if update_elapsed > 0 {
                lines.push(format!(
                    "  Last update from {} {} ago",
                    neighbor_ip,
                    Self::format_uptime(update_elapsed)
                ));
            }
        }

        let last_read = data
            .get("bgpTimerLastRead")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let last_write = data
            .get("bgpTimerLastWrite")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        lines.push(format!(
            "  Last read {}, Last write {}",
            Self::format_time(last_read),
            Self::format_time(last_write)
        ));

        let hold_time = data
            .get("bgpTimerHoldTimeMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            / 1000;
        let keepalive = data
            .get("bgpTimerKeepAliveIntervalMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            / 1000;
        lines.push(format!(
            "  Hold time is {} seconds, keepalive interval is {} seconds",
            hold_time, keepalive
        ));

        let cfg_hold = data
            .get("bgpTimerConfiguredHoldTimeMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            / 1000;
        let cfg_keepalive = data
            .get("bgpTimerConfiguredKeepAliveIntervalMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            / 1000;
        lines.push(format!(
            "  Configured hold time is {} seconds, keepalive interval is {} seconds",
            cfg_hold, cfg_keepalive
        ));

        // TCP MSS
        let tcp_mss_cfg = data
            .get("bgpTcpMssConfigured")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let tcp_mss_synced = data
            .get("bgpTcpMssSynced")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        lines.push(format!(
            "  Configured tcp-mss is {}, synced tcp-mss is {}",
            tcp_mss_cfg, tcp_mss_synced
        ));

        // Conditional advertisements interval
        let cond_adv = data
            .get("bgpTimerConfiguredConditionalAdvertisementsSec")
            .and_then(Value::as_i64)
            .unwrap_or(60);
        lines.push(format!(
            "  Configured conditional advertisements interval is {} seconds",
            cond_adv
        ));

        // Connect retry timer (matching Python label: "Connect retry timer: N seconds")
        if let Some(connect_retry) = data.get("connectRetryTimer").and_then(Value::as_i64) {
            if connect_retry > 0 {
                lines.push(format!(
                    "  Connect retry timer: {} seconds",
                    connect_retry
                ));
            }
        }

        // Peer established epoch (BUG 1 fix — Python outputs this)
        if let Some(up_epoch) = data
            .get("bgpTimerUpEstablishedEpoch")
            .and_then(Value::as_i64)
        {
            if up_epoch > 0 {
                lines.push(format!("  Peer established epoch: {}", up_epoch));
            }
        }

        // Extended Optional Parameters Length (BUG 2 fix)
        if let Some(ext_opt) = data
            .get("extendedOptionalParametersLength")
            .and_then(Value::as_bool)
        {
            if ext_opt {
                lines.push(
                    "  Extended Optional Parameters Length: Enabled".to_string(),
                );
            }
        }

        // External BGP neighbor max hops
        if let Some(max_hops) = data
            .get("externalBgpNbrMaxHopsAway")
            .and_then(Value::as_i64)
        {
            let nbr_external = data
                .get("nbrExternalLink")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            if max_hops > 0 && nbr_external {
                lines.push(format!(
                    "  External BGP neighbor may be up to {} hops away.",
                    max_hops
                ));
            }
        }

        // Host local/foreign (with 2-space indent, matching Python)
        let host_local = data
            .get("hostLocal")
            .map(value_to_string)
            .unwrap_or_default();
        let host_foreign = data
            .get("hostForeign")
            .map(value_to_string)
            .unwrap_or_default();
        let port_local = data.get("portLocal").and_then(Value::as_i64).unwrap_or(0);
        let port_foreign = data.get("portForeign").and_then(Value::as_i64).unwrap_or(0);
        if !host_local.is_empty() && !host_foreign.is_empty() {
            lines.push(format!(
                "  Local host: {}, Local port: {}",
                host_local, port_local
            ));
            lines.push(format!(
                "  Foreign host: {}, Foreign port: {}",
                host_foreign, port_foreign
            ));
        }

        // Nexthop info (with 2-space indent, matching Python)
        if let Some(nexthop) = data.get("nexthop").map(value_to_string) {
            if !nexthop.is_empty() {
                lines.push(format!("  Nexthop: {}", nexthop));
            }
        }
        if let Some(nexthop_global) = data.get("nexthopGlobal").map(value_to_string) {
            if !nexthop_global.is_empty() {
                lines.push(format!("  Nexthop global: {}", nexthop_global));
            }
        }
        if let Some(nexthop_local) = data.get("nexthopLocal").map(value_to_string) {
            if !nexthop_local.is_empty() {
                lines.push(format!("  Nexthop local: {}", nexthop_local));
            }
        }

        // Read/write thread (with 2-space indent, comma-separated, no FD — matching Python)
        let read_thread = data
            .get("readThread")
            .map(value_to_string)
            .unwrap_or_default();
        let write_thread = data
            .get("writeThread")
            .map(value_to_string)
            .unwrap_or_default();
        if !read_thread.is_empty() || !write_thread.is_empty() {
            lines.push(format!(
                "  Read thread: {}, Write thread: {}",
                read_thread, write_thread
            ));
        }

        // Estimated RTT (with 2-space indent, matching Python)
        if let Some(rtt) = data.get("estimatedRttInMsecs").and_then(Value::as_i64) {
            if rtt > 0 {
                lines.push(format!("  Estimated round trip time: {} ms", rtt));
            }
        }

        // BGP connection (with 2-space indent, matching Python)
        if let Some(bgp_conn) = data.get("bgpConnection").map(value_to_string) {
            if !bgp_conn.is_empty() {
                lines.push(format!("  BGP connection: {}", bgp_conn));
            }
        }

        // Neighbor capabilities
        if let Some(Value::Object(caps)) = data.get("neighborCapabilities") {
            lines.push("  Neighbor capabilities:".to_string());
            lines.extend(Self::format_capabilities(caps));
        }

        // Graceful restart info
        if let Some(Value::Object(gr_info)) = data.get("gracefulRestartInfo") {
            lines.extend(Self::format_graceful_restart(gr_info));
        }

        // Message statistics
        if let Some(Value::Object(msg_stats)) = data.get("messageStats") {
            lines.push("  Message statistics:".to_string());
            let inq = msg_stats
                .get("depthInq")
                .map(value_to_string)
                .unwrap_or_else(|| "0".to_string());
            let outq = msg_stats
                .get("depthOutq")
                .map(value_to_string)
                .unwrap_or_else(|| "0".to_string());
            lines.push(format!("    Inq depth is {}", inq));
            lines.push(format!("    Outq depth is {}", outq));
            lines.push("                         Sent       Rcvd".to_string());

            let msg_types = [
                ("Opens", "opensSent", "opensRecv"),
                ("Notifications", "notificationsSent", "notificationsRecv"),
                ("Updates", "updatesSent", "updatesRecv"),
                ("Keepalives", "keepalivesSent", "keepalivesRecv"),
                ("Route Refresh", "routeRefreshSent", "routeRefreshRecv"),
                ("Capability", "capabilitySent", "capabilityRecv"),
                ("Total", "totalSent", "totalRecv"),
            ];

            for (label, sent_key, recv_key) in msg_types {
                let sent = msg_stats
                    .get(sent_key)
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());
                let recv = msg_stats
                    .get(recv_key)
                    .map(value_to_string)
                    .unwrap_or_else(|| "0".to_string());
                lines.push(format!("    {}:{:>13}{:>11}", label, sent, recv));
            }
        }

        let min_adv_time = data
            .get("minBtwnAdvertisementRunsTimerMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            / 1000;
        lines.push(format!(
            "  Minimum time between advertisement runs is {} seconds",
            min_adv_time
        ));
        lines.push(String::new());

        // Get peer group for determining NEXT_HOP behavior
        let peer_group = data
            .get("peerGroup")
            .map(value_to_string)
            .unwrap_or_default();

        // Address family info
        if let Some(Value::Object(af_info)) = data.get("addressFamilyInfo") {
            for (af_name, af_data) in af_info {
                if let Value::Object(af_map) = af_data {
                    lines.extend(Self::format_address_family(af_name, af_map, &peer_group));
                }
            }
        }

        // Connections established/dropped
        let conn_est = data
            .get("connectionsEstablished")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let conn_drop = data
            .get("connectionsDropped")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        lines.push(format!(
            "  Connections established {}; dropped {}",
            conn_est, conn_drop
        ));

        // Last reset info (BUG 6 fix — read lastResetCode from data instead of hardcoding)
        let last_reset_msec = data
            .get("lastResetTimerMsecs")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let last_reset_reason = data
            .get("lastResetDueTo")
            .map(value_to_string)
            .unwrap_or_default();
        if last_reset_msec > 0 || !last_reset_reason.is_empty() {
            let reset_time = Self::format_uptime(last_reset_msec);
            let last_reset_code = data
                .get("lastResetCode")
                .map(value_to_string)
                .unwrap_or_else(|| "n/a".to_string());
            lines.push(format!(
                "  Last reset {},   {} ({})",
                reset_time, last_reset_reason, last_reset_code
            ));
        }

        lines.push(String::new());
        lines.join("\n")
    }

    /// Format routes table (for advertised-routes, received-routes, routes views).
    fn format_routes_table(data: &Value) -> String {
        let mut lines = Vec::new();
        let map = match data.as_object() {
            Some(m) => m,
            None => return String::new(),
        };

        let table_version = map
            .get("bgpTableVersion")
            .or_else(|| map.get("tableVersion"))
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let router_id = map
            .get("bgpLocalRouterId")
            .or_else(|| map.get("routerId"))
            .map(value_to_string)
            .unwrap_or_default();
        let local_as = map.get("localAS").map(value_to_string).unwrap_or_default();
        let vrf_id = map
            .get("vrfId")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());

        lines.push(format!(
            "BGP table version is {}, local router ID is {}, vrf id {}",
            table_version, router_id, vrf_id
        ));
        lines.push(format!("Default local pref 100, local AS {}", local_as));
        lines.push("Status codes:  s suppressed, d damped, h history, u unsorted, * valid, > best, = multipath,".to_string());
        lines.push("               i internal, r RIB-failure, S Stale, R Removed".to_string());
        lines.push("Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self".to_string());
        lines.push("Origin codes:  i - IGP, e - EGP, ? - incomplete".to_string());
        lines.push("RPKI validation codes: V valid, I invalid, N Not found".to_string());
        lines.push(String::new());
        lines
            .push("    Network          Next Hop            Metric LocPrf Weight Path".to_string());

        // Get routes from various keys
        let routes = map
            .get("advertisedRoutes")
            .or_else(|| map.get("receivedRoutes"))
            .or_else(|| map.get("routes"))
            .and_then(Value::as_object);

        if let Some(routes_map) = routes {
            let mut prefixes: Vec<&String> = routes_map.keys().collect();
            // Sort by IPv6 network address (binary sort), then prefix length
            prefixes.sort_by(|a, b| {
                let parse_prefix = |s: &str| -> (u128, u8) {
                    let parts: Vec<&str> = s.split('/').collect();
                    let addr_str = parts[0];
                    let prefix_len = parts
                        .get(1)
                        .and_then(|p| p.parse::<u8>().ok())
                        .unwrap_or(128);
                    // Parse IPv6 address to u128
                    let addr = Self::parse_ipv6_to_u128(addr_str);
                    (addr, prefix_len)
                };
                let (a_addr, a_len) = parse_prefix(a);
                let (b_addr, b_len) = parse_prefix(b);
                a_addr.cmp(&b_addr).then(a_len.cmp(&b_len))
            });

            for prefix in prefixes {
                if let Some(route_info) = routes_map.get(prefix) {
                    let empty_map = serde_json::Map::new();
                    let route_obj = match route_info {
                        Value::Object(obj) => obj,
                        Value::Array(arr) if !arr.is_empty() => {
                            arr[0].as_object().unwrap_or(&empty_map)
                        }
                        _ => continue,
                    };

                    let mut status = String::from(" ");
                    if route_obj
                        .get("valid")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        status.push('*');
                    } else {
                        status.push(' ');
                    }
                    if route_obj
                        .get("best")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                        || route_obj
                            .get("bestpath")
                            .and_then(Value::as_bool)
                            .unwrap_or(false)
                    {
                        status.push('>');
                    } else if route_obj
                        .get("multipath")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        status.push('=');
                    } else {
                        status.push(' ');
                    }

                    let next_hop = route_obj
                        .get("nextHopGlobal")
                        .or_else(|| route_obj.get("nextHop"))
                        .map(value_to_string)
                        .or_else(|| {
                            // Fallback: nexthops[0].ip
                            route_obj
                                .get("nexthops")
                                .and_then(Value::as_array)
                                .and_then(|arr| arr.first())
                                .and_then(Value::as_object)
                                .and_then(|nh| nh.get("ip"))
                                .map(value_to_string)
                        })
                        .unwrap_or_else(|| "::".to_string());
                    let metric = route_obj
                        .get("metric")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let loc_prf = route_obj
                        .get("locPrf")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let weight = route_obj
                        .get("weight")
                        .map(value_to_string)
                        .unwrap_or_else(|| "0".to_string());
                    let path = route_obj
                        .get("path")
                        .map(value_to_string)
                        .unwrap_or_default();
                    let origin = match route_obj.get("origin").and_then(Value::as_str) {
                        Some("IGP") | Some("i") => "i",
                        Some("EGP") | Some("e") => "e",
                        Some("incomplete") | Some("?") => "?",
                        Some(s) => s,
                        None => "",
                    };

                    if prefix.len() < 17 {
                        lines.push(
                            format!(
                                "{} {:<17}{:<20}{:>7}{:>7}{:>6} {} {}",
                                status, prefix, next_hop, metric, loc_prf, weight, path, origin
                            )
                            .trim_end()
                            .to_string(),
                        );
                    } else {
                        lines.push(format!("{} {}", status, prefix).trim_end().to_string());
                        lines.push(
                            format!(
                                "                    {:<20}{:>7}{:>7}{:>6} {} {}",
                                next_hop, metric, loc_prf, weight, path, origin
                            )
                            .trim_end()
                            .to_string(),
                        );
                    }
                }
            }
        }

        let total_prefixes = map
            .get("totalPrefixCounter")
            .map(value_to_string)
            .unwrap_or_else(|| {
                routes
                    .map(|r| r.len().to_string())
                    .unwrap_or_else(|| "0".to_string())
            });
        let total_paths = map.get("totalPathCounter").map(value_to_string);
        lines.push(String::new());
        if let Some(tp) = total_paths {
            lines.push(format!(
                "Displayed {} routes and {} total paths",
                total_prefixes, tp
            ));
        } else {
            lines.push(format!("Displayed {} routes", total_prefixes));
        }

        lines.join("\n")
    }

    /// Check if data contains route view keys.
    fn is_routes_view(data: &Value) -> bool {
        if let Value::Object(map) = data {
            let route_view_keys = [
                "advertisedRoutes",
                "receivedRoutes",
                "bgpTableVersion",
                "tableVersion",
                "routerId",
            ];
            if route_view_keys.iter().any(|k| map.contains_key(*k)) {
                return true;
            }
            if let Some(Value::Object(routes)) = map.get("routes") {
                return routes.keys().any(|k| k.contains('/') || k.contains("::"));
            }
        }
        false
    }
}

impl Default for Ipv6BgpNeighborsFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for Ipv6BgpNeighborsFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if data.is_null() {
            return Ok(String::new());
        }

        // Check if this is a routes table view
        if Self::is_routes_view(data) {
            return Ok(Self::format_routes_table(data));
        }

        // Handle neighbor details view
        let map = match data.as_object() {
            Some(m) => m,
            None => return Ok(value_to_string(data)),
        };

        let mut lines = Vec::new();
        let mut neighbors: Vec<&String> = map.keys().collect();
        neighbors.sort_by(
            |a, b| match (Ipv6Addr::from_str(a), Ipv6Addr::from_str(b)) {
                (Ok(addr_a), Ok(addr_b)) => addr_a.cmp(&addr_b),
                _ => natural_cmp(a, b),
            },
        );

        for neighbor_ip in neighbors {
            if let Some(Value::Object(neighbor_data)) = map.get(neighbor_ip) {
                lines.push(Self::format_neighbor(neighbor_ip, neighbor_data));
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "ipv6_bgp_neighbors"
    }
}

#[allow(dead_code)]
fn _format_table_with_alignment(
    headers: &[String],
    rows: &[Vec<String>],
    _table_format: &str,
    alignments: &[Alignment],
) -> String {
    if headers.is_empty() {
        return String::new();
    }

    let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();
    for row in rows {
        for (idx, cell) in row.iter().enumerate() {
            if idx < widths.len() {
                widths[idx] = widths[idx].max(cell.len());
            }
        }
    }

    if _table_format == "grid" {
        let top_border = _grid_border('-', &widths);
        let header_border = _grid_border('=', &widths);
        let mut lines = vec![top_border.clone()];
        lines.push(format!(
            "| {} |",
            _join_row(headers, &widths, alignments, " | ")
        ));
        lines.push(header_border);
        for row in rows {
            lines.push(format!(
                "| {} |",
                _join_row(row, &widths, alignments, " | ")
            ));
            lines.push(top_border.clone());
        }
        return lines.join("\n");
    }

    let header_line = _join_row(headers, &widths, alignments, "  ");
    let separator = widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect::<Vec<_>>()
        .join("  ");
    let mut lines = vec![header_line, separator];
    for row in rows {
        lines.push(_join_row(row, &widths, alignments, "  "));
    }

    lines.join("\n")
}

#[allow(dead_code)]
fn _grid_border(ch: char, widths: &[usize]) -> String {
    let segments = widths
        .iter()
        .map(|width| ch.to_string().repeat(width + 2))
        .collect::<Vec<_>>();
    format!("+{}+", segments.join("+"))
}

#[allow(dead_code)]
fn _join_row(
    row: &[String],
    widths: &[usize],
    alignments: &[Alignment],
    separator: &str,
) -> String {
    row.iter()
        .enumerate()
        .map(|(idx, cell)| {
            let width = widths.get(idx).copied().unwrap_or(cell.len());
            let alignment = alignments.get(idx).copied().unwrap_or(Alignment::Right);
            match alignment {
                Alignment::Left => format!("{:<width$}", cell, width = width),
                Alignment::Right => format!("{:>width$}", cell, width = width),
            }
        })
        .collect::<Vec<_>>()
        .join(separator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ipv6_neighbor_formatter() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!([
            {
                "address": "fe80::1",
                "macaddress": "00:11:22:33:44:55",
                "iface": "Ethernet0",
                "vlan": "Vlan100",
                "state": "REACHABLE"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::1"));
        assert!(result.contains("00:11:22:33:44:55"));
        assert!(result.contains("REACHABLE"));
    }

    #[test]
    fn test_ipv6_route_formatter() {
        let formatter = Ipv6RouteFormatter::new();
        // The new Ipv6RouteFormatter expects dict format with prefix as key
        let data = json!({
            "::/0": {
                "protocol": "static",
                "distance": "1",
                "metric": "0",
                "nexthops": [
                    {
                        "ip": "2001:db8::1",
                        "interfaceName": "Ethernet0",
                        "active": true
                    }
                ]
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Routing entry for ::/0"));
        assert!(result.contains("via 2001:db8::1"));
    }

    #[test]
    fn test_ipv6_interface_formatter() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!([
            {
                "interface": "Ethernet0",
                "master": "",
                "ipv6_address": "2001:db8::1/64",
                "admin_status": "up",
                "oper_status": "up",
                "type": "global"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("2001:db8::1/64"));
    }

    #[test]
    fn test_ipv6_link_local_formatter() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!([
            {
                "interface": "Ethernet0",
                "link_local": "fe80::1",
                "state": "up"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("fe80::1"));
    }

    #[test]
    fn test_ndp_table_formatter() {
        let formatter = NdpTableFormatter::new();
        let data = json!([
            {
                "address": "fe80::1",
                "macaddress": "00:11:22:33:44:55",
                "iface": "Ethernet0",
                "flags": "router"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::1"));
        assert!(result.contains("router"));
    }

    #[test]
    fn test_ospfv3_neighbor_formatter() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!([
            {
                "neighbor_id": "1.1.1.1",
                "interface": "Ethernet0",
                "state": "Full/DR",
                "dead_time": "00:00:30",
                "router_priority": "1"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("1.1.1.1"));
        assert!(result.contains("Full/DR"));
    }

    #[test]
    fn test_bgpv6_summary_formatter() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.1",
            "local_as": "65001",
            "neighbors": [
                {
                    "neighbor": "2001:db8::2",
                    "version": "4",
                    "remote_as": "65002",
                    "msg_rcvd": "100",
                    "msg_sent": "100",
                    "uptime": "1d2h",
                    "state": "Established"
                }
            ]
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2001:db8::2"));
        assert!(result.contains("Established"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!([
            {
                "interface": "Vlan100",
                "vlan": "100",
                "servers": ["2001:db8::1", "2001:db8::2"]
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Vlan100"));
        assert!(result.contains("2001:db8::1"));
    }

    #[test]
    fn test_icmpv6_stats_formatter() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "rx": {
                "echo_request": "100",
                "echo_reply": "100"
            },
            "tx": {
                "echo_request": "50",
                "echo_reply": "50"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ICMPv6 Received:"));
        assert!(result.contains("echo_request: 100"));
    }

    // Additional tests for dict input format

    #[test]
    fn test_ipv6_neighbor_formatter_dict() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!({
            "fe80::1": {
                "macaddress": "00:11:22:33:44:55",
                "iface": "Ethernet0",
                "vlan": "Vlan100",
                "state": "REACHABLE"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ipv6_route_formatter_dict() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "::/0": {
                "nexthop": "2001:db8::1",
                "protocol": "static",
                "interface": "Ethernet0"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ipv6_interface_formatter_dict() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet0": {
                "ipv6_address": "2001:db8::1/64",
                "admin_status": "up",
                "oper_status": "up"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ipv6_link_local_formatter_dict() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet0": {
                "link_local": "fe80::1",
                "state": "up"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ndp_table_formatter_dict() {
        let formatter = NdpTableFormatter::new();
        let data = json!({
            "fe80::1": {
                "macaddress": "00:11:22:33:44:55",
                "iface": "Ethernet0",
                "flags": "router"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_dict() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!({
            "1.1.1.1": {
                "interface": "Ethernet0",
                "state": "Full/DR",
                "dead_time": "00:00:30"
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_dhcpv6_relay_formatter_dict() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!({
            "Vlan100": {
                "servers": ["2001:db8::1", "2001:db8::2"]
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_icmpv6_stats_formatter_string() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!("ICMPv6 raw output");
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ipv6_route_formatter_different_protocols() {
        let formatter = Ipv6RouteFormatter::new();
        // The new Ipv6RouteFormatter expects dict format with prefix as key
        let data = json!({
            "2001:db8::/32": {
                "protocol": "kernel",
                "distance": "0",
                "metric": "0",
                "nexthops": [{"directlyConnected": true, "interfaceName": "lo", "active": true}]
            },
            "2001:db8:1::/48": {
                "protocol": "connected",
                "distance": "0",
                "metric": "0",
                "nexthops": [{"directlyConnected": true, "interfaceName": "Ethernet0", "active": true}]
            },
            "2001:db8:2::/48": {
                "protocol": "bgp",
                "distance": "20",
                "metric": "0",
                "nexthops": [{"ip": "2001:db8::1", "interfaceName": "Ethernet1", "active": true}]
            },
            "2001:db8:3::/48": {
                "protocol": "ospf",
                "distance": "110",
                "metric": "20",
                "nexthops": [{"ip": "2001:db8::2", "interfaceName": "Ethernet2", "active": true}]
            },
            "2001:db8:4::/48": {
                "protocol": "ospfv3",
                "distance": "110",
                "metric": "30",
                "nexthops": [{"ip": "2001:db8::3", "interfaceName": "Ethernet3", "active": true}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Check that routes are formatted with routing entry headers
        assert!(result.contains("Routing entry for 2001:db8::/32"));
        assert!(result.contains("kernel"));
        assert!(result.contains("bgp"));
        assert!(result.contains("ospf"));
    }

    #[test]
    fn test_ipv6_route_formatter_empty_string_input() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!("No routes");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("No routes") || result.contains("Codes:"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_string_value() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet0": "fe80::1"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("fe80::1"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_string() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!("BGP not active");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "BGP not active");
    }

    #[test]
    fn test_bgpv6_summary_formatter_no_neighbors() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.1",
            "local_as": "65001"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("BGP router identifier 10.0.0.1"));
        assert!(result.contains("65001"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_with_output_field() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "output": "Formatted output data"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Formatted output data");
    }

    #[test]
    fn test_icmpv6_stats_formatter_flat_structure() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "echo_request": "50",
            "echo_reply": "50"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("echo_request: 50"));
        assert!(result.contains("echo_reply: 50"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_with_tx_section() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "rx": {"echo_request": "100"},
            "tx": {"echo_request": "50", "echo_reply": "25"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ICMPv6 Received:"));
        assert!(result.contains("ICMPv6 Transmitted:"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_string_servers() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!([
            {
                "interface": "Vlan100",
                "vlan": "100",
                "servers": "2001:db8::1"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2001:db8::1"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_dict_string_servers() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!({
            "Vlan100": {
                "vlan": "100",
                "servers": "2001:db8::1"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2001:db8::1"));
    }

    #[test]
    fn test_ndp_table_formatter_object() {
        let formatter = NdpTableFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_ipv6_interface_formatter_with_admin_oper() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet0": {
                "ipv6_address": "2001:db8::1/64",
                "Admin/Oper": "up/up",
                "type": "global"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("up/up"));
    }

    // Additional tests for uncovered branches

    #[test]
    fn test_ipv6_neighbor_formatter_alternate_keys() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!([
            {
                "Address": "fe80::1",
                "MacAddress": "00:11:22:33:44:55",
                "Interface": "Ethernet0",
                "State": "REACHABLE"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::1"));
        assert!(result.contains("REACHABLE"));
    }

    #[test]
    fn test_ipv6_route_formatter_string_fallback() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!("VRF not found");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("VRF not found") || result.contains("Codes:"));
    }

    #[test]
    fn test_ipv6_interface_formatter_dict_with_master() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet0": {
                "ipv6_address": "2001:db8::1/64",
                "admin_status": "up",
                "oper_status": "up",
                "master": "Bridge",
                "type": "global"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Bridge") || result.contains("2001:db8::1"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_dict_with_object() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet0": {
                "link_local": "fe80::1",
                "state": "up"
            },
            "Ethernet4": "fe80::2"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("fe80::1"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("fe80::2"));
    }

    #[test]
    fn test_ndp_table_formatter_alternate_keys() {
        let formatter = NdpTableFormatter::new();
        let data = json!([
            {
                "Address": "fe80::1",
                "MacAddress": "00:11:22:33:44:55",
                "Interface": "Ethernet0",
                "Flags": "R"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::1"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_ipv6unicast_peers() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.1",
            "local_as": "65001",
            "ipv6Unicast": {
                "peers": {
                    "2001:db8::2": {
                        "remoteAs": 65002,
                        "msgRcvd": 500,
                        "msgSent": 500,
                        "peerUptime": "2h30m",
                        "state": "Established",
                        "pfxRcd": 50
                    }
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2001:db8::2"));
        assert!(result.contains("65002"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_established_prefix() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.1",
            "local_as": "65001",
            "neighbors": [
                {
                    "neighbor": "2001:db8::2",
                    "remote_as": "65002",
                    "msg_rcvd": "1000",
                    "msg_sent": "1000",
                    "uptime": "1d",
                    "state": "Established",
                    "pfxRcd": "75"
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("75"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_idle_state() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.1",
            "local_as": "65001",
            "neighbors": [
                {
                    "neighbor": "2001:db8::2",
                    "remote_as": "65002",
                    "msg_rcvd": "0",
                    "msg_sent": "0",
                    "uptime": "never",
                    "state": "Idle"
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Idle"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_empty_servers() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!([
            {
                "interface": "Vlan100",
                "vlan": "100",
                "servers": []
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Vlan100"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_only_rx() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "rx": {
                "echo_request": "200",
                "neighbor_solicitation": "50"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ICMPv6 Received:"));
        assert!(result.contains("echo_request: 200"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_empty_sections() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "rx": {},
            "tx": {}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty() || result.contains("ICMPv6"));
    }

    #[test]
    fn test_ipv6_interface_formatter_down_status() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!([
            {
                "interface": "Ethernet0",
                "ipv6_address": "2001:db8::1/64",
                "admin_status": "down",
                "oper_status": "down"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("down/down"));
    }

    #[test]
    fn test_ipv6_bgp_network_formatter() {
        let formatter = Ipv6BgpNetworkFormatter::new();
        let data = json!({
            "output": "Network data found"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Network data found");
    }

    #[test]
    fn test_ipv6_bgp_network_formatter_string() {
        let formatter = Ipv6BgpNetworkFormatter::new();
        let data = json!("Raw string data");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Raw string data");
    }

    #[test]
    fn test_ipv6_fib_formatter() {
        let formatter = Ipv6FibFormatter::new();
        let data = json!({
            "total": 1,
            "entries": [
                {
                    "index": 1,
                    "vrf": "default",
                    "route": "2001:db8::/64",
                    "nexthop": "::",
                    "ifname": "Ethernet0"
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Total number of entries 1"));
        assert!(result.contains("2001:db8::/64"));
        assert!(result.contains("Ethernet0"));
    }

    #[test]
    fn test_ipv6_fib_formatter_empty() {
        let formatter = Ipv6FibFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // =====================================================
    // COMPREHENSIVE COVERAGE TESTS - DEFAULT IMPLEMENTATIONS
    // =====================================================

    #[test]
    fn test_ipv6_neighbor_formatter_default() {
        let formatter = Ipv6NeighborFormatter::default();
        assert_eq!(formatter.name(), "ipv6_neighbor");
    }

    #[test]
    fn test_ipv6_interface_formatter_default() {
        let formatter = Ipv6InterfaceFormatter::default();
        assert_eq!(formatter.name(), "ipv6_interface");
    }

    #[test]
    fn test_ipv6_link_local_formatter_default() {
        let formatter = Ipv6LinkLocalFormatter::default();
        assert_eq!(formatter.name(), "ipv6_link_local");
    }

    #[test]
    fn test_ndp_table_formatter_default() {
        let formatter = NdpTableFormatter::default();
        assert_eq!(formatter.name(), "ndp_table");
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_default() {
        let formatter = Ospfv3NeighborFormatter::default();
        assert_eq!(formatter.name(), "ospfv3_neighbor");
    }

    #[test]
    fn test_bgpv6_summary_formatter_default() {
        let formatter = Bgpv6SummaryFormatter::default();
        assert_eq!(formatter.name(), "bgpv6_summary");
    }

    #[test]
    fn test_dhcpv6_relay_formatter_default() {
        let formatter = Dhcpv6RelayFormatter::default();
        assert_eq!(formatter.name(), "dhcpv6_relay");
    }

    #[test]
    fn test_icmpv6_stats_formatter_default() {
        let formatter = Icmpv6StatsFormatter::default();
        assert_eq!(formatter.name(), "icmpv6_stats");
    }

    #[test]
    fn test_ipv6_bgp_network_formatter_default() {
        let formatter = Ipv6BgpNetworkFormatter::default();
        assert_eq!(formatter.name(), "ipv6_bgp_network");
    }

    #[test]
    fn test_ipv6_fib_formatter_default() {
        let formatter = Ipv6FibFormatter::default();
        assert_eq!(formatter.name(), "ipv6_fib");
    }

    #[test]
    fn test_ipv6_prefix_list_formatter_default() {
        let formatter = Ipv6PrefixListFormatter::default();
        assert_eq!(formatter.name(), "ipv6_prefix_list");
    }

    #[test]
    fn test_ipv6_protocol_formatter_default() {
        let formatter = Ipv6ProtocolFormatter::default();
        assert_eq!(formatter.name(), "ipv6_protocol");
    }

    #[test]
    fn test_ipv6_route_formatter_default() {
        let formatter = Ipv6RouteFormatter::default();
        assert_eq!(formatter.name(), "ipv6_route");
    }

    // =====================================================
    // EMPTY AND NULL INPUT TESTS
    // =====================================================

    #[test]
    fn test_ipv6_neighbor_formatter_empty_array() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_ipv6_neighbor_formatter_empty_object() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_ipv6_neighbor_formatter_null() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = Value::Null;
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_ipv6_interface_formatter_empty_array() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ipv6_interface_formatter_empty_object() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_empty_array() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_empty_object() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ndp_table_formatter_empty_array() {
        let formatter = NdpTableFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_empty_array() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Neighbor ID"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_empty_array() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_empty_object() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_empty_object() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty() || result.contains("Neighbor"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_empty_object() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_ipv6_prefix_list_formatter_empty_object() {
        let formatter = Ipv6PrefixListFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_ipv6_protocol_formatter_empty_array() {
        let formatter = Ipv6ProtocolFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // =====================================================
    // ALTERNATE KEY NAME TESTS
    // =====================================================

    #[test]
    fn test_ipv6_neighbor_formatter_ipv6_key() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!([{
            "ipv6": "fe80::abc",
            "lladdr": "aa:bb:cc:dd:ee:ff",
            "interface": "eth1",
            "VLAN": "200",
            "status": "STALE"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::abc"));
        assert!(result.contains("aa:bb:cc:dd:ee:ff"));
        assert!(result.contains("eth1"));
        assert!(result.contains("200"));
        assert!(result.contains("STALE"));
    }

    #[test]
    fn test_ipv6_neighbor_formatter_dict_mac_key() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!({
            "fe80::def": {
                "mac": "11:22:33:44:55:66",
                "interface": "Ethernet2",
                "vlan": "300",
                "state": "REACHABLE"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::def"));
        assert!(result.contains("11:22:33:44:55:66"));
        assert!(result.contains("Ethernet2"));
    }

    #[test]
    fn test_ipv6_interface_formatter_alternate_keys() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!([{
            "Interface": "Ethernet8",
            "Master": "PortChannel1",
            "IPv6 Address/Mask": "2001:db8::8/64",
            "Admin/Oper": "up/down",
            "Type": "global"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet8"));
        assert!(result.contains("PortChannel1"));
        assert!(result.contains("2001:db8::8/64"));
        assert!(result.contains("up/down"));
        assert!(result.contains("global"));
    }

    #[test]
    fn test_ipv6_interface_formatter_name_key() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!([{
            "name": "Loopback0",
            "address": "::1/128"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Loopback0"));
        assert!(result.contains("::1/128"));
    }

    #[test]
    fn test_ipv6_interface_formatter_dict_with_type_key() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet12": {
                "master": "Bridge",
                "address": "2001:db8::12/64",
                "admin_status": "up",
                "oper_status": "up",
                "Type": "link-local"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet12"));
        assert!(result.contains("link-local"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_alternate_keys() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!([{
            "Interface": "Ethernet4",
            "Link Local Address": "fe80::4",
            "State": "active"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("fe80::4"));
        assert!(result.contains("active"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_address_key() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!([{
            "interface": "Ethernet5",
            "address": "fe80::5",
            "state": "down"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet5"));
        assert!(result.contains("fe80::5"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_dict_address_key() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet6": {
                "address": "fe80::6",
                "state": "enabled"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet6"));
        assert!(result.contains("fe80::6"));
        assert!(result.contains("enabled"));
    }

    #[test]
    fn test_ndp_table_formatter_mac_key() {
        let formatter = NdpTableFormatter::new();
        let data = json!([{
            "address": "fe80::9",
            "mac": "99:88:77:66:55:44",
            "iface": "Ethernet9",
            "state": "permanent"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::9"));
        assert!(result.contains("99:88:77:66:55:44"));
        assert!(result.contains("permanent"));
    }

    #[test]
    fn test_ndp_table_formatter_flags_key() {
        let formatter = NdpTableFormatter::new();
        let data = json!([{
            "Address": "fe80::a",
            "MAC Address": "aa:aa:aa:aa:aa:aa",
            "Interface": "Ethernet10",
            "Flags": "router,permanent"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fe80::a"));
        assert!(result.contains("router,permanent"));
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_alternate_keys() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!([{
            "Neighbor ID": "2.2.2.2",
            "Interface": "Ethernet11",
            "State": "Full/BDR",
            "DeadTime": "00:00:35",
            "Router Priority": "128"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2.2.2.2"));
        assert!(result.contains("Full/BDR"));
        assert!(result.contains("00:00:35"));
        assert!(result.contains("128"));
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_priority_key() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!([{
            "neighbor_id": "3.3.3.3",
            "interface": "Ethernet12",
            "state": "2-Way/DROther",
            "dead_time": "00:00:25",
            "priority": "1"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("3.3.3.3"));
        assert!(result.contains("2-Way/DROther"));
        assert!(result.contains("1"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_camelcase_keys() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "routerId": "10.0.0.2",
            "localAs": "65100",
            "neighbors": [{
                "neighbor": "2001:db8::100",
                "remoteAs": "65200",
                "msgRcvd": "500",
                "msgSent": "600",
                "tableVersion": "100",
                "inq": "0",
                "outq": "0",
                "peerUptime": "3d4h",
                "state": "Established",
                "pfxRcd": "200"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("10.0.0.2"));
        assert!(result.contains("65100"));
        assert!(result.contains("2001:db8::100"));
        assert!(result.contains("200"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_as_key() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "routerId": "10.0.0.3",
            "as": "65300",
            "neighbors": []
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("65300"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_ipv6_neighbors_key() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.4",
            "local_as": "65400",
            "ipv6Neighbors": [{
                "address": "2001:db8::400",
                "AS": "65500",
                "TblVer": "50",
                "InQ": "1",
                "OutQ": "2",
                "Up/Down": "1h2m",
                "State/PfxRcd": "Active"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2001:db8::400"));
        assert!(result.contains("65500"));
        assert!(result.contains("Active"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_description_key() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.5",
            "local_as": "65500",
            "neighbors": [{
                "neighbor": "2001:db8::500",
                "remote_as": "65600",
                "uptime": "5d",
                "state": "Established",
                "pfxRcd": "100",
                "description": "Peer-Router-1"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Peer-Router-1"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_alternate_keys() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!([{
            "Interface": "Vlan200",
            "VLAN": "200",
            "IPv6 Relay Servers": "2001:db8::200"
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Vlan200"));
        assert!(result.contains("200"));
        assert!(result.contains("2001:db8::200"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_dict_array_servers() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!({
            "Vlan300": {
                "vlan": "300",
                "servers": ["2001:db8::a", "2001:db8::b", "2001:db8::c"]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Vlan300"));
        assert!(result.contains("2001:db8::a"));
        assert!(result.contains("2001:db8::b"));
        assert!(result.contains("2001:db8::c"));
    }

    #[test]
    fn test_icmpv6_stats_formatter_received_transmitted_keys() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "received": {
                "total": "1000",
                "errors": "5"
            },
            "transmitted": {
                "total": "800",
                "errors": "2"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ICMPv6 Received:"));
        assert!(result.contains("ICMPv6 Transmitted:"));
        assert!(result.contains("total: 1000"));
        assert!(result.contains("total: 800"));
    }

    // =====================================================
    // IPV6 ROUTE FORMATTER SPECIAL CASES
    // =====================================================

    #[test]
    fn test_ipv6_route_formatter_output_field() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "output": "Routing table output text"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Routing table output text");
    }

    #[test]
    fn test_ipv6_route_formatter_array_routes() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:1::/48": [
                {
                    "protocol": "bgp",
                    "distance": "20",
                    "metric": "100",
                    "selected": true,
                    "nexthops": [{"ip": "2001:db8::1", "active": true}]
                },
                {
                    "protocol": "ospf",
                    "distance": "110",
                    "metric": "50",
                    "nexthops": [{"ip": "2001:db8::2", "fib": true}]
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Routing entry for 2001:db8:1::/48"));
        assert!(result.contains("bgp"));
        assert!(result.contains("ospf"));
        assert!(result.contains("best"));
    }

    #[test]
    fn test_ipv6_route_formatter_nexthop_interface_only() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:2::/48": {
                "protocol": "connected",
                "distance": "0",
                "metric": "0",
                "nexthops": [{"interfaceName": "Ethernet0", "active": true}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("directly connected, Ethernet0"));
    }

    #[test]
    fn test_ipv6_route_formatter_nexthop_via_only() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:3::/48": {
                "protocol": "static",
                "distance": "1",
                "metric": "0",
                "nexthops": [{"ip": "2001:db8::ff", "active": true}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("via 2001:db8::ff"));
    }

    #[test]
    fn test_ipv6_route_formatter_nexthop_key() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:4::/48": {
                "protocol": "bgp",
                "distance": "20",
                "metric": "0",
                "nexthops": [{"nexthop": "2001:db8::aa", "interfaceName": "Ethernet5", "fib": true}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("via 2001:db8::aa"));
    }

    #[test]
    fn test_ipv6_route_formatter_dest_selected() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:5::/48": {
                "protocol": "ospfv3",
                "distance": "110",
                "metric": "20",
                "destSelected": true,
                "uptime": "01:30:00",
                "nexthops": [{"ip": "2001:db8::bb", "active": true}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("best"));
        assert!(result.contains("Last update 01:30:00 ago"));
    }

    #[test]
    fn test_ipv6_route_formatter_inactive_nexthop() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:6::/48": {
                "protocol": "bgp",
                "distance": "20",
                "metric": "0",
                "nexthops": [
                    {"ip": "2001:db8::1", "active": true, "interfaceName": "Ethernet1"},
                    {"ip": "2001:db8::2", "active": false, "interfaceName": "Ethernet2"}
                ]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("* via 2001:db8::1"));
        assert!(result.contains("  via 2001:db8::2"));
    }

    #[test]
    fn test_ipv6_route_formatter_empty_nexthop() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:7::/48": {
                "protocol": "static",
                "distance": "1",
                "metric": "0",
                "nexthops": [{}]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Routing entry for 2001:db8:7::/48"));
    }

    #[test]
    fn test_ipv6_route_formatter_prefix_sorting() {
        let formatter = Ipv6RouteFormatter::new();
        let data = json!({
            "2001:db8:10::/48": {"protocol": "bgp", "nexthops": []},
            "2001:db8:2::/48": {"protocol": "ospf", "nexthops": []},
            "2001:db8:1::/48": {"protocol": "static", "nexthops": []},
            "invalid-prefix": {"protocol": "local", "nexthops": []}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        let idx1 = result.find("2001:db8:1::/48").unwrap();
        let idx2 = result.find("2001:db8:2::/48").unwrap();
        let idx10 = result.find("2001:db8:10::/48").unwrap();
        assert!(idx1 < idx2);
        assert!(idx2 < idx10);
    }

    // =====================================================
    // IPV6 PREFIX LIST FORMATTER TESTS
    // =====================================================

    #[test]
    fn test_ipv6_prefix_list_formatter_output_field() {
        let formatter = Ipv6PrefixListFormatter::new();
        let data = json!({
            "output": "Prefix list output text"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Prefix list output text");
    }

    #[test]
    fn test_ipv6_prefix_list_formatter_string_input() {
        let formatter = Ipv6PrefixListFormatter::new();
        let data = json!("No prefix lists configured");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("No prefix lists configured"));
    }

    #[test]
    fn test_ipv6_prefix_list_formatter_full_structure() {
        let formatter = Ipv6PrefixListFormatter::new();
        let data = json!({
            "ZEBRA": {
                "PL_TEST1": {
                    "entries": [
                        {
                            "sequenceNumber": 5,
                            "type": "permit",
                            "prefix": "2001:db8::/32",
                            "minimumPrefixLength": "48",
                            "maximumPrefixLength": "64"
                        },
                        {
                            "sequenceNumber": 10,
                            "type": "deny",
                            "prefix": "2001:db8:1::/48"
                        }
                    ]
                }
            },
            "BGP": {
                "PL_BGP": {
                    "entries": [
                        {
                            "sequenceNumber": 100,
                            "type": "permit",
                            "prefix": "::/0"
                        }
                    ]
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ZEBRA: ipv6 prefix-list PL_TEST1: 2 entries"));
        assert!(result.contains("seq 5 permit 2001:db8::/32 ge 48 le 64"));
        assert!(result.contains("seq 10 deny 2001:db8:1::/48"));
        assert!(result.contains("BGP: ipv6 prefix-list PL_BGP: 1 entries"));
    }

    #[test]
    fn test_ipv6_prefix_list_formatter_empty_entries() {
        let formatter = Ipv6PrefixListFormatter::new();
        let data = json!({
            "ZEBRA": {
                "EMPTY_LIST": {
                    "entries": []
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ZEBRA: ipv6 prefix-list EMPTY_LIST: 0 entries"));
    }

    // =====================================================
    // IPV6 PROTOCOL FORMATTER TESTS
    // =====================================================

    #[test]
    fn test_ipv6_protocol_formatter_output_field() {
        let formatter = Ipv6ProtocolFormatter::new();
        let data = json!({
            "output": "Protocol output text"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Protocol output text");
    }

    #[test]
    fn test_ipv6_protocol_formatter_string_input() {
        let formatter = Ipv6ProtocolFormatter::new();
        let data = json!("Protocol string output");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Protocol string output"));
    }

    #[test]
    fn test_ipv6_protocol_formatter_full_structure() {
        let formatter = Ipv6ProtocolFormatter::new();
        let data = json!([{
            "VRF": "management",
            "Protocols": [
                {"Protocol": "bgp", "route-map": "BGP_OUT"},
                {"Protocol": "ospf", "route-map": "OSPF_RMAP"},
                {"Protocol": "static"}
            ]
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("VRF: management"));
        assert!(result.contains("Protocol"));
        assert!(result.contains("bgp"));
        assert!(result.contains("BGP_OUT"));
        assert!(result.contains("ospf"));
        assert!(result.contains("static"));
        assert!(result.contains("none"));
    }

    #[test]
    fn test_ipv6_protocol_formatter_default_vrf() {
        let formatter = Ipv6ProtocolFormatter::new();
        let data = json!([{
            "Protocols": [
                {"Protocol": "connected"}
            ]
        }]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("VRF: default"));
    }

    // =====================================================
    // IPV6 FIB FORMATTER TESTS
    // =====================================================

    #[test]
    fn test_ipv6_fib_formatter_no_entries() {
        let formatter = Ipv6FibFormatter::new();
        let data = json!({
            "total": 0,
            "entries": []
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Total number of entries 0"));
    }

    #[test]
    fn test_ipv6_fib_formatter_multiple_entries() {
        let formatter = Ipv6FibFormatter::new();
        let data = json!({
            "entries": [
                {"index": 1, "vrf": "default", "route": "::/0", "nexthop": "2001:db8::1", "ifname": "Ethernet0"},
                {"index": 2, "vrf": "default", "route": "2001:db8::/32", "nexthop": "::", "ifname": "Loopback0"},
                {"index": 3, "vrf": "mgmt", "route": "fe80::/10", "nexthop": "::", "ifname": "eth0"}
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Total number of entries 3"));
        assert!(result.contains("::/0"));
        assert!(result.contains("2001:db8::/32"));
        assert!(result.contains("mgmt"));
    }

    #[test]
    fn test_ipv6_fib_formatter_string_input() {
        let formatter = Ipv6FibFormatter::new();
        let data = json!("FIB table not available");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // =====================================================
    // IPV6 BGP NETWORK FORMATTER TESTS
    // =====================================================

    #[test]
    fn test_ipv6_bgp_network_formatter_object_no_output() {
        let formatter = Ipv6BgpNetworkFormatter::new();
        let data = json!({
            "network": "2001:db8::/32",
            "status": "active"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("network") || result.contains("2001:db8::/32"));
    }

    #[test]
    fn test_ipv6_bgp_network_formatter_array_input() {
        let formatter = Ipv6BgpNetworkFormatter::new();
        let data = json!([
            {"network": "2001:db8::/32"},
            {"network": "2001:db8:1::/48"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(!result.is_empty());
    }

    // =====================================================
    // FORMAT TABLE WITH ALIGNMENT TESTS (Grid Format)
    // =====================================================

    #[test]
    fn test_format_table_grid_format() {
        let headers = vec!["Col1".to_string(), "Col2".to_string()];
        let rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["CCC".to_string(), "D".to_string()],
        ];
        let alignments = vec![Alignment::Left, Alignment::Right];
        let result = _format_table_with_alignment(&headers, &rows, "grid", &alignments);
        assert!(result.contains("+"));
        assert!(result.contains("|"));
        assert!(result.contains("="));
    }

    #[test]
    fn test_format_table_simple_format() {
        let headers = vec!["Name".to_string(), "Value".to_string()];
        let rows = vec![vec!["test".to_string(), "123".to_string()]];
        let alignments = vec![Alignment::Left, Alignment::Right];
        let result = _format_table_with_alignment(&headers, &rows, "simple", &alignments);
        assert!(result.contains("Name"));
        assert!(result.contains("Value"));
        assert!(result.contains("----"));
    }

    #[test]
    fn test_format_table_empty_headers() {
        let headers: Vec<String> = vec![];
        let rows: Vec<Vec<String>> = vec![];
        let alignments: Vec<Alignment> = vec![];
        let result = _format_table_with_alignment(&headers, &rows, "simple", &alignments);
        assert!(result.is_empty());
    }

    #[test]
    fn test_format_table_empty_rows() {
        let headers = vec!["Header1".to_string()];
        let rows: Vec<Vec<String>> = vec![];
        let alignments = vec![Alignment::Left];
        let result = _format_table_with_alignment(&headers, &rows, "simple", &alignments);
        assert!(result.contains("Header1"));
    }

    // =====================================================
    // BGP NEIGHBOR ROW EDGE CASES
    // =====================================================

    #[test]
    fn test_bgpv6_summary_formatter_empty_version() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.6",
            "local_as": "65600",
            "neighbors": [{
                "neighbor": "2001:db8::600",
                "remote_as": "65700"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("4")); // default version
    }

    #[test]
    fn test_bgpv6_summary_formatter_non_established_state() {
        let formatter = Bgpv6SummaryFormatter::new();
        // When pfxRcd is present, it takes priority over state (matching Python behavior)
        let data = json!({
            "router_id": "10.0.0.7",
            "local_as": "65700",
            "neighbors": [{
                "neighbor": "2001:db8::700",
                "remote_as": "65800",
                "state": "Connect",
                "pfxRcd": "0"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // pfxRcd takes priority: shows "0" not "Connect"
        assert!(result.contains("0"));

        // When pfxRcd is absent, state is used
        let data2 = json!({
            "router_id": "10.0.0.7",
            "local_as": "65700",
            "neighbors": [{
                "neighbor": "2001:db8::700",
                "remote_as": "65800",
                "state": "Connect"
            }]
        });
        let result2 = formatter.format(&data2, &config, "simple", None).unwrap();
        assert!(result2.contains("Connect"));
    }

    #[test]
    fn test_bgpv6_summary_formatter_neighbor_name() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "router_id": "10.0.0.8",
            "local_as": "65800",
            "neighbors": [{
                "neighbor": "2001:db8::800",
                "remote_as": "65900",
                "state": "Established",
                "pfxRcd": "50",
                "NeighborName": "upstream-peer"
            }]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("upstream-peer"));
    }

    // =====================================================
    // NEIGHBOR DICT WITH NON-OBJECT VALUES
    // =====================================================

    #[test]
    fn test_ipv6_neighbor_formatter_dict_non_object_value() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!({
            "fe80::100": "invalid_value"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_dhcpv6_relay_formatter_dict_non_object_value() {
        let formatter = Dhcpv6RelayFormatter::new();
        let data = json!({
            "Vlan400": "invalid"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ipv6_interface_formatter_dict_non_object_value() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet99": "invalid"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    // =====================================================
    // ARRAY WITH NON-OBJECT ITEMS
    // =====================================================

    #[test]
    fn test_ipv6_neighbor_formatter_array_non_object() {
        let formatter = Ipv6NeighborFormatter::new();
        let data = json!(["string_item", 123, null]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    #[test]
    fn test_ipv6_interface_formatter_array_non_object() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!(["not_an_object"]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Interface"));
    }

    #[test]
    fn test_ospfv3_neighbor_formatter_array_non_object() {
        let formatter = Ospfv3NeighborFormatter::new();
        let data = json!([null, "string"]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Neighbor ID"));
    }

    #[test]
    fn test_ndp_table_formatter_array_non_object() {
        let formatter = NdpTableFormatter::new();
        let data = json!([123, true]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address"));
    }

    // =====================================================
    // ICMPV6 STATS EDGE CASES
    // =====================================================

    #[test]
    fn test_icmpv6_stats_formatter_null() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = Value::Null;
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_icmpv6_stats_formatter_only_tx() {
        let formatter = Icmpv6StatsFormatter::new();
        let data = json!({
            "tx": {
                "echo_reply": "100"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ICMPv6 Transmitted:"));
    }

    // =====================================================
    // LINK LOCAL STRING VALUE IN DICT
    // =====================================================

    #[test]
    fn test_ipv6_link_local_formatter_dict_null_value() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet7": null
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet7"));
    }

    #[test]
    fn test_ipv6_link_local_formatter_dict_number_value() {
        let formatter = Ipv6LinkLocalFormatter::new();
        let data = json!({
            "Ethernet8": 12345
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet8"));
        assert!(result.contains("12345"));
    }

    // =====================================================
    // IPV6 BGP NEIGHBORS FORMATTER
    // =====================================================

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_null() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = Value::Null;
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_string() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!("raw output text");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "raw output text");
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_basic_neighbor() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::2": {
                "remoteAs": 65100,
                "localAs": 65000,
                "nbrExternalLink": true,
                "localRole": "provider",
                "remoteRole": "customer",
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.2",
                "localRouterId": "10.0.0.1",
                "hostname": "leaf01",
                "softwareVersion": "FRR-8.0",
                "bgpState": "Established",
                "bgpTimerUpString": "1d02h03m",
                "bgpTimerUpEstablishedEpoch": 1700000000,
                "bgpTimerLastRead": 3000,
                "bgpTimerLastWrite": 5000,
                "bgpInUpdateElapsedTimeMsecs": 60000,
                "bgpTimerHoldTimeMsecs": 180000,
                "bgpTimerKeepAliveIntervalMsecs": 60000,
                "bgpTimerConfiguredHoldTimeMsecs": 180000,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 60000,
                "bgpTcpMssConfigured": 0,
                "bgpTcpMssSynced": 0,
                "bgpTimerConfiguredConditionalAdvertisementsSec": 60,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "lastResetTimerMsecs": 90000,
                "lastResetDueTo": "Admin reset",
                "externalBgpNbrMaxHopsAway": 2,
                "hostLocal": "fc00::1",
                "hostForeign": "fc00::2",
                "portLocal": 179,
                "portForeign": 54321,
                "nexthop": "fc00::1",
                "nexthopGlobal": "fc00::1",
                "nexthopLocal": "fe80::1",
                "bgpConnection": "shared network",
                "connectRetryTimer": 120,
                "estimatedRttInMsecs": 5,
                "readThread": "on",
                "writeThread": "on",
                "fdUsed": 25,
                "minBtwnAdvertisementRunsTimerMsecs": 5000,
                "peerGroup": "SPINE_V6"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result
            .contains("BGP neighbor is fc00::2, remote AS 65100, local AS 65000, external link"));
        assert!(result.contains("Local Role: provider"));
        assert!(result.contains("Remote Role: customer"));
        assert!(
            result.contains("BGP version 4, remote router ID 10.0.0.2, local router ID 10.0.0.1")
        );
        assert!(result.contains("Hostname: leaf01"));
        assert!(result.contains("Software Version: FRR-8.0"));
        assert!(result.contains("BGP state = Established"));
        // Peer established epoch now shown (matching Python)
        assert!(result.contains("Peer established epoch: 1700000000"), "Epoch should be present: {}", result);
        assert!(result.contains("up for 1d02h03m"));
        // "Last update from" should come before "Last read" (matching Python order)
        assert!(result.contains("Last update from fc00::2"), "result: {}", result);
        assert!(result.contains("Hold time is 180 seconds"));
        assert!(result.contains("keepalive interval is 60 seconds"));
        assert!(result.contains("Connections established 1; dropped 0"));
        assert!(result.contains("Admin reset"));
        assert!(result.contains("External BGP neighbor may be up to 2 hops away"));
        assert!(result.contains("  Local host: fc00::1, Local port: 179"));
        assert!(result.contains("  Foreign host: fc00::2, Foreign port: 54321"));
        assert!(result.contains("  Nexthop: fc00::1"));
        assert!(result.contains("  Nexthop global: fc00::1"));
        assert!(result.contains("  Nexthop local: fe80::1"));
        assert!(result.contains("  BGP connection: shared network"));
        assert!(result.contains("  Connect retry timer: 120 seconds"));
        assert!(result.contains("  Estimated round trip time: 5 ms"));
        assert!(result.contains("  Read thread: on, Write thread: on"));
        assert!(!result.contains("FD used:"), "FD used should not appear: {}", result);
        assert!(result.contains("Member of peer-group SPINE_V6"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_internal_link() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::3": {
                "remoteAs": 65000,
                "localAs": 65000,
                "nbrExternalLink": false,
                "localRole": "undefined",
                "remoteRole": "undefined",
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.3",
                "localRouterId": "10.0.0.1",
                "bgpState": "Established",
                "bgpTimerUpString": "00:30:00",
                "bgpTimerLastRead": 1000,
                "bgpTimerLastWrite": 2000,
                "bgpTimerHoldTimeMsecs": 180000,
                "bgpTimerKeepAliveIntervalMsecs": 60000,
                "bgpTimerConfiguredHoldTimeMsecs": 180000,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 60000,
                "connectionsEstablished": 2,
                "connectionsDropped": 1,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "readThread": "on",
                "writeThread": "off"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("confed-external link"));
        assert!(result.contains("Neighbor under common administration"));
        assert!(result.contains("  Read thread: on, Write thread: off"));
        // No FD output (matching Python)
        assert!(!result.contains("FD used:"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_with_capabilities() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::4": {
                "remoteAs": 65200,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.4",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "neighborCapabilities": {
                    "4byteAs": {"advertisedAndReceived": true},
                    "extendedMessage": "advertised",
                    "addPath": {
                        "ipv6Unicast": {"rxAdvertisedAndReceived": true}
                    },
                    "longLivedGracefulRestart": "advertised",
                    "routeRefresh": {"advertised": true},
                    "enhancedRouteRefresh": "received",
                    "multiprotocolExtensions": {
                        "ipv6Unicast": "advertisedAndReceived",
                        "ipv4Unicast": {"advertisedAndReceived": true}
                    },
                    "hostName": {
                        "advHostName": "spine01",
                        "advDomainName": "lab.local",
                        "rcvHostName": "leaf01"
                    },
                    "softwareVersion": {"version": "FRR-8.0"},
                    "gracefulRestart": true,
                    "gracefulRestartRemoteTimerMsecs": 120000,
                    "addressFamiliesByPeer": {
                        "ipv6Unicast": true
                    }
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Neighbor capabilities:"));
        assert!(result.contains("4 Byte AS: advertised and received"));
        assert!(result.contains("Extended Message: advertised"));
        assert!(result.contains("AddPath:"));
        assert!(result.contains("IPv6 Unicast: RX advertised and received"));
        assert!(result.contains("Long-lived Graceful Restart: advertised"));
        assert!(result.contains("Route refresh: advertised"));
        assert!(result.contains("Enhanced Route Refresh: received"));
        assert!(result.contains("Address Family IPv6 Unicast:"));
        assert!(result.contains(
            "Hostname Capability: advertised (name: spine01,domain name: lab.local) received"
        ));
        assert!(result.contains("Version Capability: not advertised not received"));
        assert!(result.contains("Graceful Restart Capability: advertised and received"));
        assert!(result.contains("Remote Restart timer is 120 seconds"));
        assert!(result.contains("Address families by peer:"));
        assert!(result.contains("IPv6 Unicast(not preserved)"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_with_graceful_restart() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::5": {
                "remoteAs": 65300,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.5",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "gracefulRestartInfo": {
                    "endOfRibSend": {"ipv6Unicast": true},
                    "endOfRibRecv": {"ipv6Unicast": true},
                    "localGrMode": "Helper*",
                    "remoteGrMode": "Helper",
                    "rBit": true,
                    "nBit": false,
                    "timers": {
                        "configuredRestartTimer": 120,
                        "receivedRestartTimer": 90,
                        "configuredLlgrStaleTime": 0
                    },
                    "ipv6Unicast": {
                        "fBit": true,
                        "endOfRibStatus": {
                            "endOfRibSend": true,
                            "endOfRibSentAfterUpdate": false,
                            "endOfRibRecv": true
                        }
                    }
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Graceful restart information:"));
        assert!(result.contains("End-of-RIB send: IPv6 Unicast"));
        assert!(result.contains("End-of-RIB received: IPv6 Unicast"));
        assert!(result.contains("Local GR Mode: Helper*"));
        assert!(result.contains("Remote GR Mode: Helper"));
        assert!(result.contains("R bit: True"));
        assert!(result.contains("N bit: False"));
        assert!(result.contains("Configured Restart Time(sec): 120"));
        assert!(result.contains("Received Restart Time(sec): 90"));
        assert!(result.contains("F bit: True"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_with_message_stats() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::6": {
                "remoteAs": 65400,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.6",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "messageStats": {
                    "depthInq": 0,
                    "depthOutq": 0,
                    "opensSent": 1,
                    "opensRecv": 1,
                    "notificationsSent": 0,
                    "notificationsRecv": 0,
                    "updatesSent": 50,
                    "updatesRecv": 30,
                    "keepalivesSent": 100,
                    "keepalivesRecv": 99,
                    "routeRefreshSent": 2,
                    "routeRefreshRecv": 1,
                    "capabilitySent": 2,
                    "capabilityRecv": 2,
                    "totalSent": 155,
                    "totalRecv": 133
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Message statistics:"));
        assert!(result.contains("Inq depth is 0"));
        assert!(result.contains("Outq depth is 0"));
        assert!(result.contains("Opens:"));
        assert!(result.contains("Total:"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_with_address_family() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::7": {
                "remoteAs": 65500,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.7",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "addressFamilyInfo": {
                    "ipv6Unicast": {
                        "routerAlwaysNextHop": true,
                        "commAttriSentToNbr": "standard",
                        "acceptedPrefixCounter": 10,
                        "sentPrefixCounter": 20,
                        "outboundPathPolicyConfig": true,
                        "inboundPathPolicyConfig": false,
                        "incomingUpdatePrefixFilterList": "FILTER_V6",
                        "outgoingUpdatePrefixFilterList": "OUT_V6",
                        "incomingUpdateAsPathFilterList": "AS_FILTER"
                    }
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // format_af_name converts "ipv6Unicast" -> "IPv6 Unicast"
        assert!(result.contains("IPv6 Unicast"), "result was: {}", result);
        assert!(result.contains("standard")); // commAttriSentToNbr
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_routes_view() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "bgpTableVersion": 10,
            "bgpLocalRouterId": "10.0.0.1",
            "localAS": 65000,
            "vrfId": "0",
            "advertisedRoutes": {
                "::/0": {
                    "valid": true,
                    "best": true,
                    "nextHopGlobal": "fc00::1",
                    "metric": 0,
                    "locPrf": 100,
                    "weight": 32768,
                    "path": "",
                    "origin": "IGP"
                },
                "2001:db8:1::/48": {
                    "valid": true,
                    "best": false,
                    "multipath": true,
                    "nextHopGlobal": "fc00::2",
                    "metric": 10,
                    "locPrf": 200,
                    "weight": 0,
                    "path": "65100 65200",
                    "origin": "incomplete"
                }
            },
            "totalPrefixCounter": 2,
            "totalPathCounter": 3
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("BGP table version is 10"));
        assert!(result.contains("local router ID is 10.0.0.1"));
        assert!(result.contains("Network          Next Hop"));
        assert!(result.contains("*>"));
        assert!(result.contains("*="));
        assert!(result.contains("Displayed 2 routes and 3 total paths"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_routes_view_long_prefix() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "bgpTableVersion": 5,
            "bgpLocalRouterId": "10.0.0.1",
            "localAS": 65000,
            "vrfId": "0",
            "advertisedRoutes": {
                "2001:db8:abcd:1234::/64": {
                    "valid": true,
                    "best": true,
                    "nextHopGlobal": "fc00::1",
                    "metric": 0,
                    "locPrf": 100,
                    "weight": 0,
                    "path": "",
                    "origin": "i"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Long prefix should wrap to 2 lines
        assert!(result.contains("2001:db8:abcd:1234::/64"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_routes_via_nexthops_array() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "tableVersion": 3,
            "routerId": "10.0.0.1",
            "localAS": 65000,
            "routes": {
                "2001:db8::/32": {
                    "valid": true,
                    "bestpath": true,
                    "nexthops": [{"ip": "fc00::99"}],
                    "weight": 0,
                    "path": "65100",
                    "origin": "EGP"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fc00::99"));
        assert!(result.contains("e")); // EGP -> e
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_routes_array_entry() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "bgpTableVersion": 1,
            "bgpLocalRouterId": "10.0.0.1",
            "localAS": 65000,
            "receivedRoutes": {
                "::/0": [
                    {
                        "valid": false,
                        "nextHop": "fc00::aa",
                        "weight": 0,
                        "path": "",
                        "origin": "?"
                    }
                ]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("fc00::aa"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_is_routes_view_with_routes_key() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "routes": {
                "2001:db8::/32": {"valid": true}
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should detect as routes view and format accordingly
        assert!(result.contains("BGP table version"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_multiple_neighbors_sorted() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::ff": {
                "remoteAs": 65002,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.255",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0
            },
            "fc00::1": {
                "remoteAs": 65001,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.1",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // fc00::1 should come before fc00::ff in sorted order
        let pos1 = result.find("fc00::1,").unwrap_or(0);
        let pos2 = result.find("fc00::ff,").unwrap_or(0);
        assert!(pos1 < pos2, "fc00::1 should come before fc00::ff");
    }

    #[test]
    fn test_ipv6_bgp_neighbors_format_time() {
        assert_eq!(Ipv6BgpNeighborsFormatter::format_time(0), "00:00:00");
        assert_eq!(Ipv6BgpNeighborsFormatter::format_time(3661000), "01:01:01");
        assert_eq!(Ipv6BgpNeighborsFormatter::format_time(86400000), "24:00:00");
    }

    #[test]
    fn test_ipv6_bgp_neighbors_format_uptime() {
        assert_eq!(Ipv6BgpNeighborsFormatter::format_uptime(0), "never");
        assert_eq!(Ipv6BgpNeighborsFormatter::format_uptime(-1), "never");
        assert_eq!(
            Ipv6BgpNeighborsFormatter::format_uptime(3661000),
            "01:01:01"
        );
        assert_eq!(
            Ipv6BgpNeighborsFormatter::format_uptime(90061000),
            "1d01h01m"
        );
    }

    #[test]
    fn test_ipv6_bgp_neighbors_format_af_name() {
        assert_eq!(
            Ipv6BgpNeighborsFormatter::format_af_name("ipv6Unicast"),
            "IPv6 Unicast"
        );
        assert_eq!(
            Ipv6BgpNeighborsFormatter::format_af_name("ipv4Unicast"),
            "IPv4 Unicast"
        );
        assert_eq!(
            Ipv6BgpNeighborsFormatter::format_af_name("other"),
            "Unknown"
        );
    }

    #[test]
    fn test_ipv6_bgp_neighbors_format_cap_value_variants() {
        // Bool true
        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!(true));
        assert_eq!(result, "advertised and received");

        // String variants
        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!("advertisedAndReceived"));
        assert_eq!(result, "advertised and received");

        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!("advertised"));
        assert_eq!(result, "advertised");

        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!("received"));
        assert_eq!(result, "received");

        // Object with received only
        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!({"received": true}));
        assert_eq!(result, "received");

        // Object with no flags
        let result = Ipv6BgpNeighborsFormatter::format_cap_value(&json!({}));
        assert_eq!(result, "advertised and received");
    }

    #[test]
    fn test_ipv6_bgp_neighbors_parse_ipv6_to_u128() {
        let result = Ipv6BgpNeighborsFormatter::parse_ipv6_to_u128("::1");
        assert_eq!(result, 1);

        let result = Ipv6BgpNeighborsFormatter::parse_ipv6_to_u128("not_an_ip");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ipv6_bgp_neighbors_address_families_by_peer_string() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::a": {
                "remoteAs": 65000,
                "localAs": 65000,
                "nbrExternalLink": false,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.10",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 0,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "neighborCapabilities": {
                    "gracefulRestart": {"advertised": true},
                    "addressFamiliesByPeer": "none"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Address families by peer:"));
        assert!(result.contains("none"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_with_nbr_desc() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::b": {
                "remoteAs": 65100,
                "localAs": 65000,
                "nbrExternalLink": true,
                "nbrDesc": "Uplink to spine",
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.11",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 0,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Description: Uplink to spine"));
    }

    #[test]
    fn test_ipv6_bgp_neighbors_hostname_empty_and_missing() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        // hostname = "Unknown" should not be displayed
        let data = json!({
            "fc00::c": {
                "remoteAs": 65100,
                "localAs": 65000,
                "nbrExternalLink": true,
                "hostname": "Unknown",
                "softwareVersion": "n/a",
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.12",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 0,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(!result.contains("Hostname:"));
        assert!(!result.contains("Software Version:"));
    }

    // ===== Coverage gap tests for Ipv6InterfaceFormatter ipv6_addresses array (L311-377) =====

    #[test]
    fn test_ipv6_interface_formatter_with_ipv6_addresses_array() {
        let formatter = Ipv6InterfaceFormatter::new();
        // Dict format with ipv6_addresses (plural) array
        let data = json!({
            "Ethernet0": {
                "master": "",
                "admin_status": "up",
                "oper_status": "up",
                "type": "global",
                "ipv6_addresses": [
                    {
                        "address": "fc00::1/64",
                        "type": "global",
                        "bgp_neighbor_name": "ARISTA01",
                        "bgp_neighbor_ip": "fc00::2"
                    },
                    {
                        "address": "fe80::1/10",
                        "bgp_neighbor_name": "",
                        "bgp_neighbor_ip": ""
                    }
                ]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("fc00::1/64"));
        assert!(result.contains("ARISTA01"));
        assert!(result.contains("fc00::2"));
        // Second address should be on a continuation line (no interface name)
        assert!(result.contains("fe80::1"));
    }

    #[test]
    fn test_ipv6_interface_formatter_with_singular_ipv6_address() {
        let formatter = Ipv6InterfaceFormatter::new();
        // Dict format with singular ipv6_address
        let data = json!({
            "Ethernet4": {
                "master": "",
                "admin_status": "up",
                "oper_status": "up",
                "ipv6_address": "fc00::5/64",
                "bgp_neighbor_name": "PEER01",
                "bgp_neighbor_ip": "fc00::6"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet4"), "result: {}", result);
        assert!(result.contains("fc00::5/64"));
        assert!(result.contains("PEER01"));
    }

    #[test]
    fn test_ipv6_interface_formatter_admin_oper_combined() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Admin/Oper": "up/down",
                "ipv6_address": "fc00::1/64"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("up/down"), "result: {}", result);
    }

    #[test]
    fn test_ipv6_interface_formatter_per_addr_type() {
        let formatter = Ipv6InterfaceFormatter::new();
        let data = json!({
            "Loopback0": {
                "master": "",
                "admin_status": "up",
                "oper_status": "up",
                "ipv6_addresses": [
                    {
                        "address": "fc00::1/128",
                        "type": "anycast",
                        "bgp_neighbor_name": "",
                        "bgp_neighbor_ip": ""
                    }
                ]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("anycast"), "result: {}", result);
    }

    // ===== Coverage gap tests for Ipv6LinkLocalModeFormatter (L589-625) =====

    #[test]
    fn test_ipv6_link_local_mode_formatter_array() {
        let formatter = Ipv6LinkLocalModeFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "mode": "enabled"},
            {"interface": "Ethernet4", "Mode": "disabled"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("enabled"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("disabled"));
    }

    #[test]
    fn test_ipv6_link_local_mode_formatter_dict() {
        let formatter = Ipv6LinkLocalModeFormatter::new();
        let data = json!({
            "Ethernet0": {"mode": "enabled"},
            "Ethernet4": "disabled"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("enabled"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("disabled"));
    }

    // ===== Coverage gap tests for Ipv6BgpSummaryFormatter rib/peer/peergroup (L907-960) =====

    #[test]
    fn test_ipv6_bgp_summary_formatter_with_rib_peer_peergroup() {
        let formatter = Bgpv6SummaryFormatter::new();
        let data = json!({
            "routerId": "10.0.0.1",
            "localAs": 65000,
            "vrfId": "default",
            "tableVersion": 42,
            "ribCount": 100,
            "ribMemory": 16000,
            "peerCount": 4,
            "peerMemory": 8192,
            "peerGroupCount": 2,
            "peerGroupMemory": 1024,
            "peers": {
                "fc00::2": {
                    "remoteAs": 65001,
                    "version": 4,
                    "msgRcvd": 100,
                    "msgSent": 200,
                    "tableVersion": 42,
                    "inq": 0,
                    "outq": 0,
                    "peerUptime": "01:00:00",
                    "state": "Established",
                    "pfxRcd": 10,
                    "hostname": "spine01"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("BGP router identifier 10.0.0.1"),
            "result: {}",
            result
        );
        assert!(result.contains("vrf-id default"));
        assert!(result.contains("BGP table version 42"));
        assert!(result.contains("RIB entries 100, using 16000 bytes"));
        assert!(result.contains("Peers 4, using 8192 KiB"));
        assert!(result.contains("Peer groups 2, using 1024 bytes"));
    }

    // ===== Coverage gap tests for Ipv6BgpNeighborsFormatter AF format sections (L2301-2421) =====

    #[test]
    fn test_ipv6_bgp_neighbors_formatter_af_detailed_fields() {
        let formatter = Ipv6BgpNeighborsFormatter::new();
        let data = json!({
            "fc00::7": {
                "remoteAs": 65500,
                "localAs": 65000,
                "nbrExternalLink": true,
                "bgpVersion": 4,
                "remoteRouterId": "10.0.0.7",
                "localRouterId": "10.0.0.1",
                "bgpTimerLastRead": 0,
                "bgpTimerLastWrite": 0,
                "bgpTimerHoldTimeMsecs": 0,
                "bgpTimerKeepAliveIntervalMsecs": 0,
                "bgpTimerConfiguredHoldTimeMsecs": 0,
                "bgpTimerConfiguredKeepAliveIntervalMsecs": 0,
                "connectionsEstablished": 1,
                "connectionsDropped": 0,
                "minBtwnAdvertisementRunsTimerMsecs": 0,
                "addressFamilyInfo": {
                    "ipv6Unicast": {
                        "peerGroupMember": "PEER_V6",
                        "updateGroupId": 1,
                        "subGroupId": 2,
                        "packetQueueLength": 5,
                        "inboundSoftConfigPermit": true,
                        "allowAsInCount": 3,
                        "commAttriSentToNbr": "extended",
                        "inboundPathPolicyConfig": true,
                        "outboundPathPolicyConfig": true,
                        "incomingUpdatePrefixFilterList": "PFX_IN",
                        "outgoingUpdatePrefixFilterList": "PFX_OUT",
                        "incomingUpdateAsPathFilterList": "AS_IN",
                        "routeMapForIncomingAdvertisements": "RM_IN",
                        "routeMapForOutgoingAdvertisements": "RM_OUT",
                        "acceptedPrefixCounter": 50,
                        "sentPrefixCounter": 30,
                        "routerAlwaysNextHop": true
                    }
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("IPv6 Unicast"), "result: {}", result);
        assert!(result.contains("PEER_V6 peer-group member"));
        assert!(result.contains("Update group 1, subgroup 2"));
        assert!(result.contains("Packet Queue length 5"));
        assert!(result.contains("Inbound soft reconfiguration allowed"));
        assert!(result.contains("3 occurrences"));
        assert!(result.contains("extended"));
    }
}
