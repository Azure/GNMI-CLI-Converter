//! Formatters module - contains all output formatters for CLI commands.
//!
//! This module provides formatters for converting gNMI JSON data into
//! human-readable CLI output. Each formatter implements the `OutputFormatter`
//! trait and handles a specific category of CLI commands.

pub mod base;
pub mod dropcounters;
pub mod interface;
pub mod ipv6;
pub mod network;
pub mod queue;
pub mod system;
pub mod transceiver;
pub mod watermark;

// Re-export commonly used items
pub use base::{
    add_header_separator, add_header_separator_smart, build_table, tabulate_grid, tabulate_simple,
    tabulate_simple_aligned, tabulate_simple_auto, value_to_string, CommandHelpFormatter, FormatError, ListFormatter,
    OutputFormatter, TableFormatter, TextFormatter,
};

// Re-export watermark formatters
pub use watermark::{BufferPoolFormatter, HeadroomPoolFormatter, QueueWatermarkFormatter, WatermarkTelemetryFormatter};

// Re-export system formatters
pub use system::{
    ClockFormatter, MmuFormatter, ProcessesCpuFormatter, ProcessesFormatter, RebootCauseFormatter,
    ServicesFormatter, SystemMemoryFormatter, UptimeFormatter, VersionFormatter,
};

// Re-export interface formatters
pub use interface::{
    CountersTrimFormatter, FecHistogramFormatter, FecStatsFormatter, FecStatusFormatter,
    InterfaceAliasFormatter, InterfaceCountersDetailedFormatter, InterfaceCountersErrorsFormatter,
    InterfaceCountersFormatter, InterfaceCountersRatesFormatter, InterfaceCountersRifFormatter,
    InterfaceDescriptionFormatter, InterfaceErrorsFormatter, InterfaceFlapFormatter,
    InterfaceNamingModeFormatter, InterfaceStatusFormatter, NeighborExpectedFormatter,
    PortChannelFormatter, SwitchportConfigFormatter, SwitchportStatusFormatter,
};

// Re-export transceiver formatters
pub use transceiver::{
    TransceiverEepromFormatter, TransceiverErrorStatusFormatter, TransceiverFirmwareFormatter,
    TransceiverInfoFormatter, TransceiverLpmodeFormatter, TransceiverPmFormatter,
    TransceiverPresenceFormatter, TransceiverStatusFormatter,
};

// Re-export network formatters
pub use network::{
    ArpTableFormatter, BgpSummaryFormatter, IpRouteFormatter, LldpNeighborFormatter,
    LldpTableFormatter, MacAgingTimeFormatter, MacTableFormatter, NtpStatusFormatter,
    Srv6Formatter, VlanBriefFormatter, VlanConfigFormatter,
};

// Re-export queue formatters
pub use queue::{
    MulticastQueueFormatter, PriorityGroupFormatter, QueueCountersFormatter,
    SharedHeadroomPoolFormatter, WredCountersFormatter,
};

// Re-export drop counter formatters
pub use dropcounters::{
    DropCounterCapabilitiesFormatter, DropCounterConfigFormatter, DropCounterCountsFormatter,
    DropReasonsFormatter,
};

// Re-export IPv6 formatters
pub use ipv6::{
    Bgpv6SummaryFormatter, Dhcpv6RelayFormatter, Icmpv6StatsFormatter, Ipv6BgpNeighborsFormatter,
    Ipv6BgpNetworkFormatter, Ipv6FibFormatter, Ipv6InterfaceFormatter, Ipv6LinkLocalFormatter,
    Ipv6LinkLocalModeFormatter, Ipv6NeighborFormatter, Ipv6PrefixListFormatter,
    Ipv6ProtocolFormatter, Ipv6RouteFormatter, NdpTableFormatter, Ospfv3NeighborFormatter,
};

use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all available formatters.
pub struct FormatterRegistry {
    formatters: HashMap<String, Arc<dyn OutputFormatter + Send + Sync>>,
}

impl FormatterRegistry {
    /// Create a new formatter registry with all available formatters.
    pub fn new() -> Self {
        let mut formatters: HashMap<String, Arc<dyn OutputFormatter + Send + Sync>> =
            HashMap::new();

        // Base formatters
        formatters.insert("table".to_string(), Arc::new(TableFormatter::new()));
        formatters.insert("text".to_string(), Arc::new(TextFormatter::new()));
        formatters.insert("list".to_string(), Arc::new(ListFormatter::new()));
        formatters.insert("help".to_string(), Arc::new(CommandHelpFormatter::new()));
        formatters.insert(
            "command_help".to_string(),
            Arc::new(CommandHelpFormatter::new()),
        );

        // Watermark formatters
        formatters.insert(
            "watermark_telemetry".to_string(),
            Arc::new(WatermarkTelemetryFormatter::new()),
        );
        formatters.insert(
            "queue_watermark".to_string(),
            Arc::new(QueueWatermarkFormatter::new()),
        );
        formatters.insert(
            "headroom_pool".to_string(),
            Arc::new(HeadroomPoolFormatter::new()),
        );
        // Alias for buffer_pool
        formatters.insert(
            "buffer_pool".to_string(),
            Arc::new(BufferPoolFormatter::new()),
        );

        // System formatters
        formatters.insert("clock".to_string(), Arc::new(ClockFormatter::new()));
        formatters.insert("uptime".to_string(), Arc::new(UptimeFormatter::new()));
        formatters.insert("version".to_string(), Arc::new(VersionFormatter::new()));
        formatters.insert(
            "system_memory".to_string(),
            Arc::new(SystemMemoryFormatter::new()),
        );
        formatters.insert("services".to_string(), Arc::new(ServicesFormatter::new()));
        formatters.insert("processes".to_string(), Arc::new(ProcessesFormatter::new()));
        formatters.insert(
            "processes_cpu".to_string(),
            Arc::new(ProcessesCpuFormatter::new()),
        );
        formatters.insert(
            "processes_memory".to_string(),
            Arc::new(ProcessesCpuFormatter::new()),
        );
        formatters.insert(
            "reboot_cause".to_string(),
            Arc::new(RebootCauseFormatter::new()),
        );
        formatters.insert(
            "reboot_cause_history".to_string(),
            Arc::new(RebootCauseFormatter::new()),
        );
        formatters.insert("mmu".to_string(), Arc::new(MmuFormatter::new()));

        // Interface formatters
        formatters.insert(
            "interface_status".to_string(),
            Arc::new(InterfaceStatusFormatter::new()),
        );
        formatters.insert(
            "interface_description".to_string(),
            Arc::new(InterfaceDescriptionFormatter::new()),
        );
        formatters.insert(
            "interface_counters".to_string(),
            Arc::new(InterfaceCountersFormatter::new()),
        );
        formatters.insert(
            "interface_counters_detailed".to_string(),
            Arc::new(InterfaceCountersDetailedFormatter::new()),
        );
        formatters.insert(
            "interface_counters_errors".to_string(),
            Arc::new(InterfaceCountersErrorsFormatter::new()),
        );
        formatters.insert(
            "interface_counters_rates".to_string(),
            Arc::new(InterfaceCountersRatesFormatter::new()),
        );
        formatters.insert(
            "interface_counters_rif".to_string(),
            Arc::new(InterfaceCountersRifFormatter::new()),
        );
        formatters.insert(
            "interface_errors".to_string(),
            Arc::new(InterfaceErrorsFormatter::new()),
        );
        formatters.insert(
            "interface_alias".to_string(),
            Arc::new(InterfaceAliasFormatter::new()),
        );
        formatters.insert(
            "interface_naming_mode".to_string(),
            Arc::new(InterfaceNamingModeFormatter::new()),
        );
        formatters.insert(
            "naming_mode".to_string(),
            Arc::new(InterfaceNamingModeFormatter::new()),
        );
        formatters.insert(
            "interface_flap".to_string(),
            Arc::new(InterfaceFlapFormatter::new()),
        );
        formatters.insert(
            "portchannel".to_string(),
            Arc::new(PortChannelFormatter::new()),
        );
        formatters.insert(
            "fec_status".to_string(),
            Arc::new(FecStatusFormatter::new()),
        );
        formatters.insert(
            "fec_histogram".to_string(),
            Arc::new(FecHistogramFormatter::new()),
        );
        formatters.insert("fec_stats".to_string(), Arc::new(FecStatsFormatter::new()));
        formatters.insert(
            "switchport_config".to_string(),
            Arc::new(SwitchportConfigFormatter::new()),
        );
        formatters.insert(
            "switchport_status".to_string(),
            Arc::new(SwitchportStatusFormatter::new()),
        );
        formatters.insert(
            "neighbor_expected".to_string(),
            Arc::new(NeighborExpectedFormatter::new()),
        );
        formatters.insert(
            "counters_trim".to_string(),
            Arc::new(CountersTrimFormatter::new()),
        );

        // Transceiver formatters
        formatters.insert(
            "transceiver_presence".to_string(),
            Arc::new(TransceiverPresenceFormatter::new()),
        );
        formatters.insert(
            "transceiver_info".to_string(),
            Arc::new(TransceiverInfoFormatter::new()),
        );
        formatters.insert(
            "transceiver_status".to_string(),
            Arc::new(TransceiverStatusFormatter::new()),
        );
        formatters.insert(
            "transceiver_eeprom".to_string(),
            Arc::new(TransceiverEepromFormatter::new()),
        );
        formatters.insert(
            "transceiver_error_status".to_string(),
            Arc::new(TransceiverErrorStatusFormatter::new()),
        );
        formatters.insert(
            "transceiver_lpmode".to_string(),
            Arc::new(TransceiverLpmodeFormatter::new()),
        );
        formatters.insert(
            "transceiver_pm".to_string(),
            Arc::new(TransceiverPmFormatter::new()),
        );
        formatters.insert(
            "transceiver_firmware".to_string(),
            Arc::new(TransceiverFirmwareFormatter::new()),
        );

        // Network formatters
        formatters.insert(
            "lldp_table".to_string(),
            Arc::new(LldpTableFormatter::new()),
        );
        formatters.insert(
            "lldp_neighbor".to_string(),
            Arc::new(LldpNeighborFormatter::new()),
        );
        formatters.insert(
            "lldp_neighbors".to_string(),
            Arc::new(LldpNeighborFormatter::new()),
        );
        formatters.insert(
            "vlan_brief".to_string(),
            Arc::new(VlanBriefFormatter::new()),
        );
        formatters.insert(
            "vlan_config".to_string(),
            Arc::new(VlanConfigFormatter::new()),
        );
        formatters.insert("arp_table".to_string(), Arc::new(ArpTableFormatter::new()));
        formatters.insert("mac_table".to_string(), Arc::new(MacTableFormatter::new()));
        formatters.insert("ip_route".to_string(), Arc::new(IpRouteFormatter::new()));
        formatters.insert(
            "bgp_summary".to_string(),
            Arc::new(BgpSummaryFormatter::new()),
        );
        formatters.insert(
            "ntp_status".to_string(),
            Arc::new(NtpStatusFormatter::new()),
        );
        formatters.insert("srv6".to_string(), Arc::new(Srv6Formatter::new()));
        formatters.insert("srv6_stats".to_string(), Arc::new(Srv6Formatter::new()));
        formatters.insert(
            "mac_aging_time".to_string(),
            Arc::new(MacAgingTimeFormatter::new()),
        );
        formatters.insert("mac".to_string(), Arc::new(MacTableFormatter::new()));
        formatters.insert("mac_mac".to_string(), Arc::new(MacTableFormatter::new()));
        formatters.insert("ndp".to_string(), Arc::new(NdpTableFormatter::new()));
        formatters.insert("ndp_ndp".to_string(), Arc::new(NdpTableFormatter::new()));

        // Queue formatters
        formatters.insert(
            "queue_counters".to_string(),
            Arc::new(QueueCountersFormatter::new()),
        );
        formatters.insert(
            "wred_counters".to_string(),
            Arc::new(WredCountersFormatter::new()),
        );
        formatters.insert(
            "queue_wredcounters".to_string(),
            Arc::new(WredCountersFormatter::new()),
        );
        formatters.insert(
            "priority_group".to_string(),
            Arc::new(PriorityGroupFormatter::new()),
        );
        formatters.insert(
            "multicast_queue".to_string(),
            Arc::new(MulticastQueueFormatter::new()),
        );
        formatters.insert(
            "shared_headroom_pool".to_string(),
            Arc::new(SharedHeadroomPoolFormatter::new()),
        );

        // Drop counter formatters
        formatters.insert(
            "dropcounter_capabilities".to_string(),
            Arc::new(DropCounterCapabilitiesFormatter::new()),
        );
        // Alias for commands usage
        formatters.insert(
            "dropcounters_capabilities".to_string(),
            Arc::new(DropCounterCapabilitiesFormatter::new()),
        );
        formatters.insert(
            "dropcounter_config".to_string(),
            Arc::new(DropCounterConfigFormatter::new()),
        );
        formatters.insert(
            "dropcounters_configuration".to_string(),
            Arc::new(DropCounterConfigFormatter::new()),
        );
        formatters.insert(
            "dropcounter_counts".to_string(),
            Arc::new(DropCounterCountsFormatter::new()),
        );
        formatters.insert(
            "dropcounters_counts".to_string(),
            Arc::new(DropCounterCountsFormatter::new()),
        );
        formatters.insert(
            "drop_reasons".to_string(),
            Arc::new(DropReasonsFormatter::new()),
        );

        // IPv6 formatters
        formatters.insert(
            "ipv6_neighbor".to_string(),
            Arc::new(Ipv6NeighborFormatter::new()),
        );
        formatters.insert(
            "ipv6_route".to_string(),
            Arc::new(Ipv6RouteFormatter::new()),
        );
        formatters.insert(
            "ipv6_interface".to_string(),
            Arc::new(Ipv6InterfaceFormatter::new()),
        );
        formatters.insert(
            "ipv6_interfaces".to_string(),
            Arc::new(Ipv6InterfaceFormatter::new()),
        );
        formatters.insert(
            "ipv6_link_local".to_string(),
            Arc::new(Ipv6LinkLocalFormatter::new()),
        );
        formatters.insert(
            "ipv6_link_local_mode".to_string(),
            Arc::new(Ipv6LinkLocalModeFormatter::new()),
        );
        formatters.insert("ndp_table".to_string(), Arc::new(NdpTableFormatter::new()));
        formatters.insert(
            "ospfv3_neighbor".to_string(),
            Arc::new(Ospfv3NeighborFormatter::new()),
        );
        formatters.insert(
            "bgpv6_summary".to_string(),
            Arc::new(Bgpv6SummaryFormatter::new()),
        );
        formatters.insert(
            "ipv6_bgp_summary".to_string(),
            Arc::new(Bgpv6SummaryFormatter::new()),
        );
        formatters.insert(
            "dhcpv6_relay".to_string(),
            Arc::new(Dhcpv6RelayFormatter::new()),
        );
        formatters.insert(
            "icmpv6_stats".to_string(),
            Arc::new(Icmpv6StatsFormatter::new()),
        );
        formatters.insert(
            "ipv6_bgp_network".to_string(),
            Arc::new(Ipv6BgpNetworkFormatter::new()),
        );
        formatters.insert("ipv6_fib".to_string(), Arc::new(Ipv6FibFormatter::new()));
        formatters.insert(
            "ipv6_prefix_list".to_string(),
            Arc::new(Ipv6PrefixListFormatter::new()),
        );
        formatters.insert(
            "ipv6_protocol".to_string(),
            Arc::new(Ipv6ProtocolFormatter::new()),
        );
        formatters.insert(
            "ipv6_bgp_neighbors".to_string(),
            Arc::new(Ipv6BgpNeighborsFormatter::new()),
        );
        // Alias for interface neighbor
        formatters.insert(
            "interface_neighbor".to_string(),
            Arc::new(NeighborExpectedFormatter::new()),
        );

        Self { formatters }
    }

    /// Get a formatter by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn OutputFormatter + Send + Sync>> {
        self.formatters.get(name).cloned()
    }

    /// Get all formatter names.
    pub fn names(&self) -> Vec<&String> {
        self.formatters.keys().collect()
    }

    /// Register a custom formatter.
    pub fn register(&mut self, name: String, formatter: Arc<dyn OutputFormatter + Send + Sync>) {
        self.formatters.insert(name, formatter);
    }
}

impl Default for FormatterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_registry_new() {
        let registry = FormatterRegistry::new();
        assert!(!registry.formatters.is_empty());
    }

    #[test]
    fn test_formatter_registry_get() {
        let registry = FormatterRegistry::new();
        assert!(registry.get("table").is_some());
        assert!(registry.get("interface_status").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_formatter_registry_names() {
        let registry = FormatterRegistry::new();
        let names = registry.names();
        assert!(names.contains(&&"table".to_string()));
        assert!(names.contains(&&"clock".to_string()));
    }

    #[test]
    fn test_formatter_registry_register_custom() {
        let mut registry = FormatterRegistry::new();
        registry.register("custom".to_string(), Arc::new(TextFormatter::new()));
        assert!(registry.get("custom").is_some());
    }

    #[test]
    fn test_all_formatters_have_name() {
        let registry = FormatterRegistry::new();
        for (key, formatter) in &registry.formatters {
            let name = formatter.name();
            assert!(!name.is_empty(), "Formatter {} has empty name", key);
        }
    }

    // ---- Round 5 coverage tests ----

    #[test]
    fn test_formatter_registry_default() {
        // Cover L449-450: FormatterRegistry::default()
        let registry = FormatterRegistry::default();
        assert!(!registry.formatters.is_empty());
    }
}
