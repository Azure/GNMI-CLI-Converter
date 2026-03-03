//! Integration tests for system formatters.
//!
//! Tests clock, uptime, version, system memory, services, processes,
//! reboot-cause, MMU, and watermark telemetry commands using mock data.
//!
//! Source: sonic-utilities/tests/mmuconfig_input/mmuconfig_test_vectors.py
//!
//! ## Formatter coverage
//! - `clock` (ClockFormatter)
//! - `uptime` (UptimeFormatter)
//! - `version` (VersionFormatter)
//! - `system_memory` (SystemMemoryFormatter)
//! - `services` (ServicesFormatter)
//! - `processes_summary` / `processes_cpu` (ProcessFormatter)
//! - `reboot_cause` (RebootCauseFormatter)
//! - `mmu` (MmuFormatter)

mod common;

use common::assert_exact_match;
use common::sonic_utilities_system::*;
use gnmi_cli_lib::converter::GnmiToCliConverter;

// =============================================================================
// Clock tests
// =============================================================================

// #03 - show clock
#[test]
fn test_show_clock() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_CLOCK).unwrap();
    let result = converter.convert("clock", data).unwrap();
    assert_exact_match(&result, SHOW_CLOCK_OUTPUT, "clock");
}

// #03 - show clock (AM/PM)
#[test]
fn test_show_clock_ampm() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_CLOCK_AMPM).unwrap();
    let result = converter.convert("clock", data).unwrap();
    assert_exact_match(&result, SHOW_CLOCK_AMPM_OUTPUT, "clock ampm");
}

// #03 - show clock (alias: clock_clock)
#[test]
fn test_show_clock_alias() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_CLOCK).unwrap();
    let result = converter.convert("clock_clock", data).unwrap();
    assert_exact_match(&result, SHOW_CLOCK_OUTPUT, "clock_clock alias");
}

// =============================================================================
// Uptime tests
// =============================================================================

// #67 - show uptime
#[test]
fn test_show_uptime() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_UPTIME).unwrap();
    let result = converter.convert("uptime", data).unwrap();
    assert_exact_match(&result, SHOW_UPTIME_OUTPUT, "uptime");
}

// #67 - show uptime (components)
#[test]
fn test_show_uptime_components() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_UPTIME_COMPONENTS).unwrap();
    let result = converter.convert("uptime", data).unwrap();
    assert_exact_match(&result, SHOW_UPTIME_COMPONENTS_OUTPUT, "uptime components");
}

// #67 - show uptime (alias: uptime_uptime)
#[test]
fn test_show_uptime_alias() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_UPTIME).unwrap();
    let result = converter.convert("uptime_uptime", data).unwrap();
    assert_exact_match(&result, SHOW_UPTIME_OUTPUT, "uptime_uptime alias");
}

// =============================================================================
// Version tests
// =============================================================================

// #68 - show version
#[test]
fn test_show_version() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_VERSION).unwrap();
    let result = converter.convert("version", data).unwrap();
    assert_exact_match(&result, SHOW_VERSION_OUTPUT, "version");
}

// =============================================================================
// System memory tests
// =============================================================================

// #66 - show system-memory
#[test]
fn test_show_system_memory() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_SYSTEM_MEMORY).unwrap();
    let result = converter.convert("system_memory", data).unwrap();
    assert_exact_match(&result, SHOW_SYSTEM_MEMORY_OUTPUT, "system_memory");
}

// =============================================================================
// Services tests
// =============================================================================

// #64 - show services
#[test]
fn test_show_services() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_SERVICES).unwrap();
    let result = converter.convert("services", data).unwrap();
    assert_exact_match(&result, SHOW_SERVICES_OUTPUT, "services");
}

// =============================================================================
// Processes tests
// =============================================================================

// #50 - show processes summary
#[test]
fn test_show_processes_summary() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PROCESSES_SUMMARY).unwrap();
    let result = converter.convert("processes_summary", data).unwrap();
    assert_exact_match(&result, SHOW_PROCESSES_SUMMARY_OUTPUT, "processes_summary");
}

// #51 - show processes cpu
#[test]
fn test_show_processes_cpu() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_PROCESSES_CPU).unwrap();
    let result = converter.convert("processes_cpu", data).unwrap();
    assert_exact_match(&result, SHOW_PROCESSES_CPU_OUTPUT, "processes_cpu");
}

// =============================================================================
// Reboot cause tests
// =============================================================================

// #63 - show reboot-cause
#[test]
fn test_show_reboot_cause() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_REBOOT_CAUSE).unwrap();
    let result = converter.convert("reboot_cause", data).unwrap();
    assert_exact_match(&result, SHOW_REBOOT_CAUSE_OUTPUT, "reboot_cause");
}

// #63 - show reboot-cause (aliases)
#[test]
fn test_show_reboot_cause_aliases() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_REBOOT_CAUSE).unwrap();
    // All aliases should produce same output
    let aliases = ["reboot-cause", "reboot_cause_current", "reboot-cause_current"];
    for alias in &aliases {
        let result = converter.convert(alias, data.clone()).unwrap();
        assert_exact_match(&result, SHOW_REBOOT_CAUSE_OUTPUT, &format!("reboot_cause alias {}", alias));
    }
}

// #63 - show reboot-cause history
#[test]
fn test_show_reboot_cause_history() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_REBOOT_CAUSE_HISTORY).unwrap();
    let result = converter.convert("reboot_cause_history", data).unwrap();
    assert_exact_match(&result, SHOW_REBOOT_CAUSE_HISTORY_OUTPUT, "reboot_cause_history");
}

// #63 - show reboot-cause (empty)
#[test]
fn test_show_reboot_cause_empty() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_REBOOT_CAUSE_EMPTY).unwrap();
    let result = converter.convert("reboot_cause", data).unwrap();
    assert_exact_match(&result, SHOW_REBOOT_CAUSE_EMPTY_OUTPUT, "reboot_cause empty");
}

// =============================================================================
// MMU tests
// Source: mmuconfig_test_vectors.py show_mmu_config
// =============================================================================

// #48 - show mmu
#[test]
fn test_show_mmu() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_MMU).unwrap();
    let result = converter.convert("mmu", data).unwrap();
    assert_exact_match(&result, SHOW_MMU_OUTPUT, "mmu");
}

// #48 - show mmu (alias: mmu_mmu)
#[test]
fn test_show_mmu_alias() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_MMU).unwrap();
    let result = converter.convert("mmu_mmu", data).unwrap();
    assert_exact_match(&result, SHOW_MMU_OUTPUT, "mmu_mmu alias");
}

// =============================================================================
// Watermark telemetry tests
// =============================================================================

// #70 - show watermark telemetry interval
#[test]
fn test_show_watermark_telemetry() {
    let converter = GnmiToCliConverter::new();
    let data: serde_json::Value = serde_json::from_str(MOCK_WATERMARK_TELEMETRY_INTERVAL).unwrap();
    let result = converter.convert("watermark_telemetry", data).unwrap();
    assert_exact_match(&result, SHOW_WATERMARK_TELEMETRY_INTERVAL_OUTPUT, "watermark_telemetry interval");
}

// =============================================================================
// Command registration tests
// =============================================================================

// registration test
#[test]
fn test_all_system_commands_registered() {
    let converter = GnmiToCliConverter::new();
    let commands = converter.available_commands();
    let expected = [
        "clock",
        "uptime",
        "version",
        "system_memory",
        "services",
        "processes_summary",
        "processes_cpu",
        "reboot_cause",
        "reboot_cause_history",
        "mmu",
        "watermark_telemetry",
    ];
    for cmd in &expected {
        assert!(commands.contains(&cmd.to_string()),
                "Missing system command: {}", cmd);
    }
}
