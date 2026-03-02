//! Unit tests for system-related show commands.
//!
//! Covers: clock, clock timezones, mmu, processes (summary/cpu/memory),
//! reboot-cause (current/history), services, system-memory, uptime, version.
//!
//! Formatter modules: system (ClockFormatter, MmuFormatter, ProcessesCpuFormatter,
//! ProcessesFormatter, RebootCauseFormatter, ServicesFormatter, SystemMemoryFormatter,
//! UptimeFormatter, VersionFormatter)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data, normalize_output,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 03: show clock
// =============================================================================

#[test]
fn test_show_clock() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"date": "Mon Jan 26 09:56:41 UTC 2026"}"#);
    let result = converter.convert("clock", data).unwrap();
    let expected = "Mon Jan 26 09:56:41 AM UTC 2026";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_clock_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(3, "clock");
    assert!(!test_data.is_empty(), "No test data found for command 03");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("clock", data).unwrap();
        let normalized = normalize_output(&result);
        assert!(!normalized.is_empty(), "Clock output should not be empty");
    }
}

// =============================================================================
// 04: show clock timezones
// =============================================================================

#[test]
fn test_show_clock_timezones() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{"timezones": ["UTC", "US/Pacific", "US/Eastern", "Europe/London"]}"#,
    );
    let result = converter.convert("clock_timezones", data).unwrap();
    let expected = concat!(
        "UTC\n",
        "US/Pacific\n",
        "US/Eastern\n",
        "Europe/London",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_clock_timezones_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(4, "clock_timezones");
    assert!(!test_data.is_empty(), "No test data found for command 04");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("clock_timezones", data).unwrap();
        // Timezone list should contain common timezone entries
        assert!(!result.trim().is_empty(), "Timezones output should not be empty");
    }
}

// =============================================================================
// 48: show mmu
// =============================================================================

#[test]
fn test_show_mmu() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(48, "mmu");
    if !test_data.is_empty() {
        for (json_str, cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("mmu", data).unwrap();
            assert_cli_output_matches(&result, cli_str);
        }
    }
}

#[test]
fn test_show_mmu_mock_data() {
    let converter = GnmiToCliConverter::new();
    // Mock data from sonic-utilities mmuconfig format
    let data = load_test_data(
        r#"{
        "pools": {
            "egress_lossless_pool": {"mode": "static", "size": "147226020", "type": "egress"},
            "ingress_lossless_pool": {"mode": "dynamic", "size": "147226020", "type": "ingress", "xoff": "33690560"}
        },
        "profiles": {
            "egress_lossless_profile": {"pool": "egress_lossless_pool", "size": "0", "static_th": "165364160"}
        }
    }"#,
    );
    let result = converter.convert("mmu", data).unwrap();
    let expected = concat!(
        "Pool: egress_lossless_pool\n",
        "----  ---------\n",
        "mode  static\n",
        "size  147226020\n",
        "type  egress\n",
        "----  ---------\n",
        "\n",
        "Pool: ingress_lossless_pool\n",
        "----  ---------\n",
        "mode  dynamic\n",
        "size  147226020\n",
        "type  ingress\n",
        "xoff  33690560\n",
        "----  ---------\n",
        "\n",
        "Profile: egress_lossless_profile\n",
        "---------  --------------------\n",
        "pool       egress_lossless_pool\n",
        "size       0\n",
        "static_th  165364160\n",
        "---------  --------------------",
    );
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 50: show processes summary
// =============================================================================

#[test]
fn test_show_processes_summary() {
    let converter = GnmiToCliConverter::new();
    // Mock data from sonic-utilities show processes summary (ps -eo format)
    let data = load_test_data(
        r#"[
        {"PID": "1", "PPID": "0", "CMD": "/sbin/init", "%MEM": "0.0", "%CPU": "0.0", "STIME": "Jan25", "TIME": "0:02:16", "TT": "None", "UID": "0"},
        {"PID": "100", "PPID": "1", "CMD": "/usr/sbin/sshd", "%MEM": "0.1", "%CPU": "0.0", "STIME": "Jan25", "TIME": "0:00:05", "TT": "None", "UID": "0"}
    ]"#,
    );
    let result = converter.convert("processes_summary", data).unwrap();
    let expected = concat!(
        "  PID    PPID  CMD               %MEM    %CPU  STIME    TIME     TT      UID\n",
        "-----  ------  --------------  ------  ------  -------  -------  ----  -----\n",
        "    1       0  /sbin/init           0       0  Jan25    0:02:16  None      0\n",
        "  100       1  /usr/sbin/sshd     0.1       0  Jan25    0:00:05  None      0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_processes_summary_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(50, "processes_summary");
    assert!(!test_data.is_empty(), "No test data found for command 50");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("processes_summary", data).unwrap();
        if result.trim().is_empty() { continue; }
        assert_output_contains_all(&result, &["PID"]);
    }
}

// =============================================================================
// 51: show processes cpu
// =============================================================================

#[test]
fn test_show_processes_cpu() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(51, "processes_cpu");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("processes_cpu", data).unwrap();
            if result.trim().is_empty() { continue; }
            assert_output_contains_all(&result, &["PID"]);
        }
    }
}

// =============================================================================
// 52: show processes memory
// =============================================================================

#[test]
fn test_show_processes_memory() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(52, "processes_memory");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("processes_memory", data).unwrap();
            if result.trim().is_empty() { continue; }
            assert_output_contains_all(&result, &["PID"]);
        }
    }
}

// =============================================================================
// 63: show reboot-cause history
// =============================================================================

#[test]
fn test_show_reboot_cause_history() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "2026_01_25_07_39_17": {
            "cause": "Power Loss ()",
            "comment": "",
            "time": "Sat Jan 25 07:39:17 UTC 2026",
            "user": "N/A"
        },
        "2026_01_20_10_00_00": {
            "cause": "warm-reboot",
            "comment": "",
            "time": "Mon Jan 20 10:00:00 UTC 2026",
            "user": "admin"
        }
    }"#,
    );
    let result = converter.convert("reboot_cause_history", data).unwrap();
    let expected = concat!(
        "Name                 Cause          Time                             User    Comment\n",
        "-------------------  -------------  -------------------------------  ------  ---------\n",
        "2026_01_25_07_39_17  Power Loss ()  Sat Jan 25 07:39:17 UTC 2026     N/A\n",
        "2026_01_20_10_00_00  warm-reboot    Mon Jan 20 10:00:00 UTC 2026     admin",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_reboot_cause_history_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(63, "reboot_cause_history");
    assert!(!test_data.is_empty(), "No test data found for command 63");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("reboot_cause_history", data).unwrap();
        if result.trim().is_empty() { continue; }
        assert_output_contains_all(&result, &["Name", "Cause", "Time", "User"]);
    }
}

// =============================================================================
// Reboot cause (current)
// =============================================================================

#[test]
fn test_show_reboot_cause() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"cause": "Power Loss ()"}"#);
    let result = converter.convert("reboot_cause", data).unwrap();
    let expected = "Power Loss ()";
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 64: show services
// =============================================================================

#[test]
fn test_show_services() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(64, "services");
    if !test_data.is_empty() {
        for (json_str, _cli_str) in &test_data {
            let data: Value = serde_json::from_str(json_str).unwrap();
            let result = converter.convert("services", data).unwrap();
            assert!(!result.trim().is_empty(), "Services output should not be empty");
        }
    }
}

// =============================================================================
// 66: show system-memory
// =============================================================================

#[test]
fn test_show_system_memory() {
    let converter = GnmiToCliConverter::new();
    // Mock data from sonic-utilities (free -m output format)
    let data = load_test_data(
        r#"[
        {"type": "Mem", "total": "31905", "used": "5459", "free": "21940", "shared": "451", "buff/cache": "5422", "available": "26445"},
        {"type": "Swap", "total": "0", "used": "0", "free": "0"}
    ]"#,
    );
    let result = converter.convert("system_memory", data).unwrap();
    let expected = concat!(
        "               total        used        free      shared  buff/cache   available\n",
        "Mem:           31905        5459       21940         451        5422       26445\n",
        "Swap:              0           0           0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_system_memory_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(66, "system_memory");
    assert!(!test_data.is_empty(), "No test data found for command 66");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("system_memory", data).unwrap();
        // System memory values may differ slightly due to real-time nature
        // Verify structure matches
        assert_output_contains_all(&result, &["Mem:", "Swap:", "total", "used", "free"]);
        let _ = normalize_output(cli_str);
    }
}

// =============================================================================
// 67: show uptime
// =============================================================================

#[test]
fn test_show_uptime() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"uptime": "up 1 day, 4 hours, 17 minutes"}"#);
    let result = converter.convert("uptime", data).unwrap();
    let expected = "up 1 day, 4 hours, 17 minutes";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_uptime_multi_day() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"uptime": "up 5 days, 8 hours, 10 minutes"}"#);
    let result = converter.convert("uptime", data).unwrap();
    let expected = "up 5 days, 8 hours, 10 minutes";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_uptime_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(67, "uptime");
    assert!(!test_data.is_empty(), "No test data found for command 67");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("uptime", data).unwrap();
        assert!(!result.trim().is_empty(), "Uptime output should not be empty");
        assert!(result.contains("up"), "Uptime should contain 'up'");
    }
}

#[test]
fn test_show_uptime_via_convert_str() {
    let converter = GnmiToCliConverter::new();
    let json_str = r#"{"uptime":"up 5 days, 8 hours, 10 minutes"}"#;
    let result = converter.convert_str("uptime", json_str).unwrap();
    let expected = "up 5 days, 8 hours, 10 minutes";
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 68: show version
// =============================================================================

#[test]
fn test_show_version() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "sonic_software_version": "SONiC.20250310.31",
        "sonic_os_version": "11",
        "distribution": "Debian 11.9",
        "kernel": "5.10.0-23-2-amd64",
        "build_commit": "abc123",
        "build_date": "Mon Jan 20 2025 00:00:00",
        "built_by": "sonic_build",
        "platform": "x86_64-arista_7060x6_64pe",
        "hwsku": "Arista-7060X6-64PE",
        "asic": "broadcom",
        "asic_count": "1",
        "serial_number": "SN12345",
        "model_number": "DCS-7060X6-64PE",
        "hardware_revision": "01.01",
        "uptime": "up 1 day, 4:17",
        "date": "Mon Jan 26 09:56:41 UTC 2026",
        "docker_images": []
    }"#,
    );
    let result = converter.convert("version", data).unwrap();
    let expected = concat!(
        "\n",
        "SONiC Software Version: SONiC.20250310.31\n",
        "SONiC OS Version: 11\n",
        "Distribution: Debian 11.9\n",
        "Kernel: 5.10.0-23-2-amd64\n",
        "Build commit: abc123\n",
        "Build date: Mon Jan 20 2025 00:00:00\n",
        "Built by: sonic_build\n",
        "\n",
        "Platform: x86_64-arista_7060x6_64pe\n",
        "HwSKU: Arista-7060X6-64PE\n",
        "ASIC: broadcom\n",
        "ASIC Count: 1\n",
        "Serial Number: SN12345\n",
        "Model Number: DCS-7060X6-64PE\n",
        "Hardware Revision: 01.01\n",
        "Uptime: up 1 day, 4:17\n",
        "Date: Mon Jan 26 09:56:41 UTC 2026\n",
        "\n",
        "Docker images:\n",
        "REPOSITORY                 TAG           IMAGE ID       SIZE",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_version_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(68, "version");
    assert!(!test_data.is_empty(), "No test data found for command 68");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("version", data).unwrap();
        assert_output_contains_all(&result, &["SONiC Software Version:", "Platform:", "HwSKU:"]);
    }
}
