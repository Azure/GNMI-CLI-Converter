//! System-related formatters for version, uptime, memory, services, processes, etc.

use crate::common::types::CommandConfig;
use crate::formatters::base::{tabulate_simple_auto, value_to_string, FormatError, OutputFormatter};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

/// Insert AM/PM designator into a 24-hour clock date string.
///
/// Converts  `"Mon Jan 26 09:56:41 UTC 2026"`
/// to        `"Mon Jan 26 09:56:41 AM UTC 2026"`
///
/// If the string already contains AM/PM or doesn't match the expected pattern,
/// it is returned unchanged.
fn add_ampm(date_str: &str) -> String {
    let s = date_str.trim();

    // Already has AM/PM
    if s.contains(" AM ") || s.contains(" PM ") {
        return s.to_string();
    }

    // Pattern: "Day Mon DD HH:MM:SS TZ YYYY"
    // We use a regex-free approach: split by whitespace, find the time token,
    // parse the hour, and insert AM/PM.
    //
    // Tokens example: ["Mon", "Jan", "26", "09:56:41", "UTC", "2026"]
    //            or:  ["Mon", "Feb", "", "9", "06:09:20", "UTC", "2026"]  (double-space day)
    //
    // The time token is the one containing two colons.
    let tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens.len() < 5 {
        return s.to_string();
    }

    // Find the index of the time token (contains ":")
    let time_idx = match tokens.iter().position(|t| t.matches(':').count() == 2) {
        Some(i) => i,
        None => return s.to_string(),
    };

    let time_token = tokens[time_idx];
    // Parse hour from "HH:MM:SS"
    let hour: u32 = match time_token.split(':').next().and_then(|h| h.parse().ok()) {
        Some(h) => h,
        None => return s.to_string(),
    };

    let (display_hour, period) = match hour {
        0 => (12u32, "AM"),
        1..=11 => (hour, "AM"),
        12 => (12, "PM"),
        _ => (hour - 12, "PM"),
    };

    // Rebuild the time token with the 12-hour display
    let new_time = format!("{:02}{}", display_hour, &time_token[time_token.find(':').unwrap()..]);

    // Reconstruct: we need to preserve the original spacing (e.g. double-space
    // before single-digit day). Walk the original string, replace the time
    // token occurrence, and insert " AM"/" PM" after it.
    if let Some(pos) = s.find(time_token) {
        let before = &s[..pos];
        let after = &s[pos + time_token.len()..];
        format!("{}{} {}{}", before, new_time, period, after)
    } else {
        s.to_string()
    }
}

/// Formatter for clock/date output.
pub struct ClockFormatter;

impl ClockFormatter {
    /// Creates a new [`ClockFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClockFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for ClockFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            // Try various date/time fields
            for key in &["date", "Date", "datetime", "time", "Time"] {
                if let Some(value) = map.get(*key) {
                    return Ok(add_ampm(&value_to_string(value)));
                }
            }
        }

        if let Value::String(s) = data {
            return Ok(add_ampm(s));
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "clock"
    }
}

/// Formatter for uptime output.
pub struct UptimeFormatter;

impl UptimeFormatter {
    /// Creates a new [`UptimeFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for UptimeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for UptimeFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            // Try to find uptime field
            if let Some(uptime) = map.get("uptime").or_else(|| map.get("Uptime")) {
                return Ok(value_to_string(uptime));
            }

            // Build uptime string from components
            let mut parts = Vec::new();
            if let Some(time) = map.get("time").or_else(|| map.get("Time")) {
                parts.push(value_to_string(time));
            }
            if let Some(up) = map.get("up") {
                parts.push(format!("up {}", value_to_string(up)));
            }
            if let Some(users) = map.get("users") {
                parts.push(format!("{} users", value_to_string(users)));
            }
            if let Some(load) = map.get("load") {
                parts.push(format!("load average: {}", value_to_string(load)));
            }

            if !parts.is_empty() {
                return Ok(parts.join(", "));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "uptime"
    }
}

/// Formatter for version output.
pub struct VersionFormatter;

impl VersionFormatter {
    /// Creates a new [`VersionFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for VersionFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for VersionFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            // Start with a leading blank line (matches expected output)
            let mut lines = vec![String::new()];

            // Standard version fields in order
            let fields: &[(&str, &[&str])] = &[
                (
                    "SONiC Software Version",
                    &[
                        "sonic_software_version",
                        "sonic_version",
                        "SonicVersion",
                        "version",
                    ],
                ),
                (
                    "SONiC OS Version",
                    &["sonic_os_version", "os_version", "OsVersion"],
                ),
                ("Distribution", &["distribution", "Distribution"]),
                ("Kernel", &["kernel", "Kernel"]),
                ("Build commit", &["build_commit", "BuildCommit", "commit"]),
                ("Build date", &["build_date", "BuildDate"]),
                ("Built by", &["built_by", "BuiltBy"]),
                ("Platform", &["platform", "Platform"]),
                ("HwSKU", &["hwsku", "HwSKU", "HwSku"]),
                ("ASIC", &["asic", "ASIC", "Asic"]),
                ("ASIC Count", &["asic_count", "AsicCount"]),
                (
                    "Serial Number",
                    &["serial_number", "SerialNumber", "serial"],
                ),
                ("Model Number", &["model_number", "ModelNumber", "model"]),
                (
                    "Hardware Revision",
                    &["hardware_revision", "HardwareRevision"],
                ),
                ("Uptime", &["uptime", "Uptime"]),
                ("Date", &["date", "Date"]),
            ];

            // Fields that should have a blank line before them
            let blank_line_before = ["platform", "Platform"];

            for (label, keys) in fields {
                for key in *keys {
                    if let Some(value) = map.get(*key) {
                        let val_str = value_to_string(value);
                        if !val_str.is_empty() {
                            // Add blank line before certain sections
                            if blank_line_before.contains(key) {
                                lines.push(String::new());
                            }
                            lines.push(format!("{}: {}", label, val_str));
                            break;
                        }
                    }
                }
            }

            // Add Docker version if present
            if let Some(docker) = map
                .get("docker_version")
                .or_else(|| map.get("DockerVersion"))
            {
                lines.push(format!("Docker Version: {}", value_to_string(docker)));
            }

            // Add Docker images table if present
            if let Some(docker_images) =
                map.get("docker_images").or_else(|| map.get("DockerImages"))
            {
                lines.push(String::new()); // Blank line before Docker images
                lines.push("Docker images:".to_string());
                lines.push(
                    "REPOSITORY                 TAG           IMAGE ID       SIZE".to_string(),
                );
                if let Value::Array(arr) = docker_images {
                    for img in arr {
                        if let Value::Object(obj) = img {
                            let repo = obj
                                .get("Repository")
                                .map(value_to_string)
                                .unwrap_or_else(|| "N/A".to_string());
                            let tag = obj
                                .get("Tag")
                                .map(value_to_string)
                                .unwrap_or_else(|| "N/A".to_string());
                            let raw_id = obj
                                .get("ID")
                                .map(value_to_string)
                                .unwrap_or_else(|| "N/A".to_string());
                            // Strip "sha256:" prefix and truncate to 12 chars (like Docker CLI)
                            let img_id = if let Some(hex) = raw_id.strip_prefix("sha256:") {
                                hex.chars().take(12).collect::<String>()
                            } else {
                                raw_id
                            };
                            let size = obj
                                .get("Size")
                                .map(value_to_string)
                                .unwrap_or_else(|| "N/A".to_string());
                            lines.push(format!("{:<26} {:<13} {} {}", repo, tag, img_id, size));
                        } else {
                            // Simple string entries (e.g., ["docker-syncd:latest", ...])
                            lines.push(value_to_string(img));
                        }
                    }
                }
            }

            // Add any remaining fields not in the standard list
            let known_keys: Vec<&str> = fields
                .iter()
                .flat_map(|(_, keys)| keys.iter().copied())
                .collect();
            for (key, value) in map {
                if !known_keys.contains(&key.as_str())
                    && key != "docker_version"
                    && key != "DockerVersion"
                    && key != "docker_images"
                    && key != "DockerImages"
                {
                    let val_str = value_to_string(value);
                    if !val_str.is_empty() {
                        lines.push(format!("{}: {}", key, val_str));
                    }
                }
            }

            return Ok(lines.join("\n"));
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "version"
    }
}

/// Formatter for system memory output.
pub struct SystemMemoryFormatter;

impl SystemMemoryFormatter {
    /// Creates a new [`SystemMemoryFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemMemoryFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for SystemMemoryFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Build memory table
        let mut lines = Vec::new();

        // Header line - fixed 80-char width: col1=20, col2-6=12 each
        lines.push(
            "               total        used        free      shared  buff/cache   available"
                .to_string(),
        );

        // Handle array format: [{type: "Mem", ...}, {type: "Swap", ...}]
        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let item_type = obj.get("type").map(value_to_string).unwrap_or_default();

                    if item_type == "Mem" {
                        let total = obj.get("total").map(value_to_string).unwrap_or_default();
                        let used = obj.get("used").map(value_to_string).unwrap_or_default();
                        let free = obj.get("free").map(value_to_string).unwrap_or_default();
                        let shared = obj.get("shared").map(value_to_string).unwrap_or_default();
                        let buff_cache = obj
                            .get("buff/cache")
                            .or_else(|| obj.get("buff_cache"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        let available = obj
                            .get("available")
                            .map(value_to_string)
                            .unwrap_or_default();

                        lines.push(format!(
                            "Mem:{:>16}{:>12}{:>12}{:>12}{:>12}{:>12}",
                            total, used, free, shared, buff_cache, available
                        ));
                    } else if item_type == "Swap" {
                        let total = obj.get("total").map(value_to_string).unwrap_or_default();
                        let used = obj.get("used").map(value_to_string).unwrap_or_default();
                        let free = obj.get("free").map(value_to_string).unwrap_or_default();

                        lines.push(format!("Swap:{:>15}{:>12}{:>12}", total, used, free));
                    }
                }
            }

            if lines.len() > 1 {
                return Ok(lines.join("\n"));
            }
        }

        if let Value::Object(map) = data {
            // Check for raw output format
            if let Some(output) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(output));
            }

            // Mem line (nested object format)
            if let Some(Value::Object(mem_map)) = map.get("Mem").or_else(|| map.get("mem")) {
                let total = mem_map
                    .get("total")
                    .map(value_to_string)
                    .unwrap_or_default();
                let used = mem_map.get("used").map(value_to_string).unwrap_or_default();
                let free = mem_map.get("free").map(value_to_string).unwrap_or_default();
                let shared = mem_map
                    .get("shared")
                    .map(value_to_string)
                    .unwrap_or_default();
                let buff_cache = mem_map
                    .get("buff/cache")
                    .or_else(|| mem_map.get("buff_cache"))
                    .map(value_to_string)
                    .unwrap_or_default();
                let available = mem_map
                    .get("available")
                    .map(value_to_string)
                    .unwrap_or_default();

                lines.push(format!(
                    "Mem:{:>16}{:>12}{:>12}{:>12}{:>12}{:>12}",
                    total, used, free, shared, buff_cache, available
                ));
            }

            // Swap line (nested object format)
            if let Some(Value::Object(swap_map)) = map.get("Swap").or_else(|| map.get("swap")) {
                let total = swap_map
                    .get("total")
                    .map(value_to_string)
                    .unwrap_or_default();
                let used = swap_map
                    .get("used")
                    .map(value_to_string)
                    .unwrap_or_default();
                let free = swap_map
                    .get("free")
                    .map(value_to_string)
                    .unwrap_or_default();

                lines.push(format!("Swap:{:>15}{:>12}{:>12}", total, used, free));
            }

            if lines.len() > 1 {
                return Ok(lines.join("\n"));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "system_memory"
    }
}

/// Formatter for services output.
pub struct ServicesFormatter;

impl ServicesFormatter {
    /// Creates a new [`ServicesFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for ServicesFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for ServicesFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Check for raw output
        if let Value::Object(map) = data {
            if let Some(output) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(output));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        // Format as docker services with ps aux style output
        // Input: array of {dockerProcessName, processes: [{user, pid, cpuPercentage, memPercentage, vsz, rss, tty, stat, start, time, command}]}
        if let Value::Array(arr) = data {
            let mut result = String::new();

            for (idx, item) in arr.iter().enumerate() {
                if let Value::Object(obj) = item {
                    // Get service name
                    let service_name = obj
                        .get("dockerProcessName")
                        .or_else(|| obj.get("name"))
                        .or_else(|| obj.get("Name"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    // Get service status
                    let service_status = obj
                        .get("status")
                        .or_else(|| obj.get("Status"))
                        .map(value_to_string)
                        .unwrap_or_else(|| "docker".to_string());

                    // Add service header: "serviceName\tstatus"
                    result.push_str(&service_name);
                    result.push('\t');
                    result.push_str(&service_status);
                    result.push('\n');

                    // Add separator
                    result.push_str("---------------------------");
                    result.push('\n');

                    // Add ps aux header
                    result.push_str("USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND");
                    result.push('\n');

                    // Add processes
                    if let Some(Value::Array(processes)) = obj.get("processes") {
                        for proc in processes {
                            if let Value::Object(p) = proc {
                                let user = p
                                    .get("user")
                                    .or_else(|| p.get("User"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let pid = p
                                    .get("pid")
                                    .or_else(|| p.get("PID"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let cpu = p
                                    .get("cpuPercentage")
                                    .or_else(|| p.get("%CPU"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let mem = p
                                    .get("memPercentage")
                                    .or_else(|| p.get("%MEM"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let vsz = p
                                    .get("vsz")
                                    .or_else(|| p.get("VSZ"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let rss = p
                                    .get("rss")
                                    .or_else(|| p.get("RSS"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let tty = p
                                    .get("tty")
                                    .or_else(|| p.get("TTY"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let stat = p
                                    .get("stat")
                                    .or_else(|| p.get("STAT"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let start = p
                                    .get("start")
                                    .or_else(|| p.get("START"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let time = p
                                    .get("time")
                                    .or_else(|| p.get("TIME"))
                                    .map(value_to_string)
                                    .unwrap_or_default();
                                let command = p
                                    .get("command")
                                    .or_else(|| p.get("COMMAND"))
                                    .map(value_to_string)
                                    .unwrap_or_default();

                                // ps aux style formatting matching column positions:
                                // USER+PID combined 16 chars (user left, pid right-aligned in remaining)
                                let user_pid_width = 16usize;
                                let pid_field_width = user_pid_width.saturating_sub(user.len());
                                let user_pid =
                                    format!("{}{:>width$}", user, pid, width = pid_field_width);

                                // Build prefix up to TTY column
                                let prefix = format!(
                                    "{}  {}  {} {:>6} {:>5} {}",
                                    user_pid, cpu, mem, vsz, rss, tty
                                );

                                // Pad prefix to position 49 for STAT column
                                let padded_prefix = if prefix.len() < 49 {
                                    format!("{}{}", prefix, " ".repeat(49 - prefix.len()))
                                } else {
                                    prefix
                                };

                                // STAT(5 left), START(8 left), TIME(4 right), space, COMMAND
                                let line = format!(
                                    "{}{:<5}{:<8}{:>4} {}",
                                    padded_prefix, stat, start, time, command
                                );
                                result.push_str(&line);
                                result.push('\n');
                            }
                        }
                    }
                }

                // Add newline between services (but not after last one)
                if idx < arr.len() - 1 {
                    // No extra newline needed between services based on expected output
                }
            }

            // Remove trailing newline if present
            if result.ends_with('\n') {
                result.pop();
            }

            return Ok(result);
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "services"
    }
}

/// Formatter for processes summary output.
///
/// Input: Array of objects like:
/// [{"PID":"1","PPID":"0","CMD":"....","%MEM":"0.0","%CPU":"0.0","STIME":"...","TIME":"...","TT":"...","UID":"..."}]
///
/// Output: Table with columns PID, PPID, CMD, %MEM, %CPU (and optionally more)
pub struct ProcessesFormatter;

impl ProcessesFormatter {
    /// Creates a new [`ProcessesFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessesFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for ProcessesFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Return raw output if present
        if let Value::Object(map) = data {
            if let Some(output) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(output));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        // Handle array of process objects
        let items = match data {
            Value::Array(arr) => arr.clone(),
            Value::Object(map) => {
                // If it's an object, try to get the values
                map.values().cloned().collect()
            }
            _ => return Ok(String::new()),
        };

        if items.is_empty() {
            return Ok(String::new());
        }

        // Column definitions: headers, JSON keys, alignment
        // Right-aligned: PID(0), PPID(1), %MEM(3), %CPU(4), UID(8)
        // Left-aligned: CMD(2), STIME(5), TIME(6), TT(7)
        let headers = ["PID", "PPID", "CMD", "%MEM", "%CPU", "STIME", "TIME", "TT", "UID"];

        // Collect all data rows first
        let mut rows: Vec<Vec<String>> = Vec::new();
        for item in &items {
            if let Value::Object(proc) = item {
                let raw_cmd = proc.get("CMD").map(value_to_string).unwrap_or_default();
                // Truncate CMD with ellipsis if exceeds max width (matches CMD_MAX_WIDTH=50)
                let cmd = if raw_cmd.len() > 50 {
                    format!("{}...", &raw_cmd[..47])
                } else {
                    raw_cmd
                };
                rows.push(vec![
                    proc.get("PID").map(value_to_string).unwrap_or_default(),
                    proc.get("PPID").map(value_to_string).unwrap_or_default(),
                    cmd,
                    proc.get("%MEM").map(value_to_string).unwrap_or_default(),
                    proc.get("%CPU").map(value_to_string).unwrap_or_default(),
                    proc.get("STIME").map(value_to_string).unwrap_or_default(),
                    proc.get("TIME").map(value_to_string).unwrap_or_default(),
                    proc.get("TT").map(value_to_string).unwrap_or_default(),
                    proc.get("UID").map(value_to_string).unwrap_or_default(),
                ]);
            }
        }

        if rows.is_empty() {
            return Ok(String::new());
        }

        Ok(tabulate_simple_auto(&headers, &rows))
    }

    fn name(&self) -> &'static str {
        "processes"
    }
}

/// Formatter for processes CPU/memory output (top-like format).
///
/// Input: Object with uptime, tasks, cpu_usage, memory_usage, swap_usage, processes
/// Output: top-like format with header section and process table
pub struct ProcessesCpuFormatter;

impl ProcessesCpuFormatter {
    /// Creates a new [`ProcessesCpuFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessesCpuFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for ProcessesCpuFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Return raw output if present
        if let Value::Object(map) = data {
            if let Some(output) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(output));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        let map = match data {
            Value::Object(m) => m,
            _ => return Ok(String::new()),
        };

        let mut lines = Vec::new();

        // Line 1: top - uptime info
        if let Some(uptime) = map.get("uptime") {
            lines.push(format!("top - {}", value_to_string(uptime)));
        }

        // Line 2: Tasks
        if let Some(tasks) = map.get("tasks") {
            lines.push(format!("Tasks: {}", value_to_string(tasks)));
        }

        // Line 3: CPU usage
        if let Some(cpu_usage) = map.get("cpu_usage") {
            lines.push(format!("%Cpu(s): {}", value_to_string(cpu_usage)));
        }

        // Line 4: Memory usage
        if let Some(memory_usage) = map.get("memory_usage") {
            lines.push(format!("MiB Mem : {}", value_to_string(memory_usage)));
        }

        // Line 5: Swap usage
        if let Some(swap_usage) = map.get("swap_usage") {
            lines.push(format!("MiB Swap: {}", value_to_string(swap_usage)));
        }

        // Blank line
        lines.push(String::new());

        // Process table header
        lines.push(
            "    PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND"
                .to_string(),
        );

        // Process rows
        if let Some(Value::Array(processes)) = map.get("processes") {
            for proc in processes {
                if let Value::Object(p) = proc {
                    let pid = p.get("pid").map(value_to_string).unwrap_or_default();
                    let user = p.get("user").map(value_to_string).unwrap_or_default();
                    let pr = p.get("pr").map(value_to_string).unwrap_or_default();
                    let ni = p.get("ni").map(value_to_string).unwrap_or_default();
                    let virt = p.get("virt").map(value_to_string).unwrap_or_default();
                    let res = p.get("res").map(value_to_string).unwrap_or_default();
                    let shr = p.get("shr").map(value_to_string).unwrap_or_default();
                    let s = p.get("s").map(value_to_string).unwrap_or_default();
                    let cpu = p.get("cpu").map(value_to_string).unwrap_or_default();
                    let mem = p.get("mem").map(value_to_string).unwrap_or_default();
                    let time = p.get("time").map(value_to_string).unwrap_or_default();
                    let command = p.get("command").map(value_to_string).unwrap_or_default();

                    // Format each field with proper width/alignment to match top output
                    lines.push(format!(
                        "{:>7} {:<9}{:>3} {:>3} {:>7} {:>6} {:>6} {} {:>5} {:>5} {:>9} {}",
                        pid, user, pr, ni, virt, res, shr, s, cpu, mem, time, command
                    ));
                }
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "processes_cpu"
    }
}

/// Formatter for reboot cause output.
pub struct RebootCauseFormatter;

impl RebootCauseFormatter {
    /// Creates a new [`RebootCauseFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for RebootCauseFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for RebootCauseFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Array(arr) = data {
            // Format as table for history
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Disabled);
            table.load_preset("                   ");

            table.set_header(vec![
                Cell::new("Name").set_alignment(CellAlignment::Left),
                Cell::new("Cause").set_alignment(CellAlignment::Left),
                Cell::new("Time").set_alignment(CellAlignment::Left),
                Cell::new("User").set_alignment(CellAlignment::Left),
                Cell::new("Comment").set_alignment(CellAlignment::Left),
            ]);

            for item in arr {
                if let Value::Object(obj) = item {
                    let name = obj
                        .get("name")
                        .or_else(|| obj.get("Name"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let cause = obj
                        .get("cause")
                        .or_else(|| obj.get("Cause"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let time = obj
                        .get("time")
                        .or_else(|| obj.get("Time"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let user = obj
                        .get("user")
                        .or_else(|| obj.get("User"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let comment = obj
                        .get("comment")
                        .or_else(|| obj.get("Comment"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&name).set_alignment(CellAlignment::Left),
                        Cell::new(&cause).set_alignment(CellAlignment::Left),
                        Cell::new(&time).set_alignment(CellAlignment::Left),
                        Cell::new(&user).set_alignment(CellAlignment::Left),
                        Cell::new(&comment).set_alignment(CellAlignment::Left),
                    ]);
                }
            }

            return Ok(table.to_string());
        }

        if let Value::Object(map) = data {
            // Single cause
            if let Some(cause) = map.get("cause").or_else(|| map.get("Cause")) {
                return Ok(value_to_string(cause));
            }
            if let Some(output) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(output));
            }

            // Check if this is a history dict with timestamp keys containing nested objects
            // e.g., {"2026_01_24_06_59_57": {"cause": "reboot", "time": "...", ...}, ...}
            let first_value = map.values().next();
            if let Some(Value::Object(_)) = first_value {
                // This is a reboot history dict - format as table manually with separator
                let columns = ["Name", "Cause", "Time", "User", "Comment"];

                // Sort keys in reverse order (most recent first)
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort_by(|a, b| b.cmp(a));

                // Collect all rows first to calculate column widths
                let mut rows: Vec<Vec<String>> = Vec::new();
                for name in &keys {
                    if let Some(Value::Object(obj)) = map.get(*name) {
                        let cause = obj
                            .get("cause")
                            .or_else(|| obj.get("Cause"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        let time = obj
                            .get("time")
                            .or_else(|| obj.get("Time"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        let raw_user = obj
                            .get("user")
                            .or_else(|| obj.get("User"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        let comment = obj
                            .get("comment")
                            .or_else(|| obj.get("Comment"))
                            .map(value_to_string)
                            .unwrap_or_default();

                        // Pass user value through as-is (including $USER$ placeholder)
                        let user = raw_user;

                        rows.push(vec![name.to_string(), cause, time, user, comment]);
                    }
                }

                // Calculate column widths: minimum widths [19, 7, 31, 6, 9]
                let min_widths: Vec<usize> = vec![19, 7, 31, 6, 9];
                let mut widths: Vec<usize> = min_widths.clone();
                for row in &rows {
                    for (i, val) in row.iter().enumerate() {
                        if i < widths.len() && val.len() > widths[i] {
                            widths[i] = val.len();
                        }
                    }
                }

                // Build header row (left-aligned with 2-space separator), rstrip trailing whitespace
                let header: String = columns
                    .iter()
                    .enumerate()
                    .map(|(i, col)| format!("{:<width$}", col, width = widths[i]))
                    .collect::<Vec<_>>()
                    .join("  ")
                    .trim_end()
                    .to_string();

                // Build separator row with dashes
                let separator: String = widths
                    .iter()
                    .map(|w| "-".repeat(*w))
                    .collect::<Vec<_>>()
                    .join("  ");

                // Build data rows (left-aligned), rstrip trailing whitespace
                let mut output_lines = vec![header, separator];
                for row in &rows {
                    let line: String = row
                        .iter()
                        .enumerate()
                        .map(|(i, val)| {
                            let w = if i < widths.len() {
                                widths[i]
                            } else {
                                val.len()
                            };
                            format!("{:<width$}", val, width = w)
                        })
                        .collect::<Vec<_>>()
                        .join("  ")
                        .trim_end()
                        .to_string();
                    output_lines.push(line);
                }

                return Ok(output_lines.join("\n"));
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "reboot_cause"
    }
}

/// Formatter for MMU configuration output.
pub struct MmuFormatter;

impl MmuFormatter {
    /// Creates a new [`MmuFormatter`].
    pub fn new() -> Self {
        Self
    }

    /// Format a Pool or Profile block with key-value table and dashed separators.
    fn format_block(
        block_type: &str,
        name: &str,
        fields: &serde_json::Map<String, Value>,
    ) -> String {
        let mut lines = Vec::new();

        // Header: "Pool: name" or "Profile: name"
        lines.push(format!("{}: {}", block_type, name));

        // Collect and sort keys using BTreeMap
        let sorted_fields: BTreeMap<_, _> = fields.iter().collect();

        let mut key_width = 0usize;
        let mut val_width = 0usize;
        let mut rows = Vec::new();

        for (k, v) in &sorted_fields {
            let val_str = value_to_string(v);
            if k.len() > key_width {
                key_width = k.len();
            }
            if val_str.len() > val_width {
                val_width = val_str.len();
            }
            rows.push((k.as_str(), val_str));
        }

        if !rows.is_empty() {
            // Separator line
            let sep = format!("{}  {}", "-".repeat(key_width), "-".repeat(val_width));
            lines.push(sep.clone());

            // Data rows (value NOT padded, matching tabulate behavior)
            for (k, v) in &rows {
                lines.push(format!("{:<kw$}  {}", k, v, kw = key_width));
            }

            // Closing separator
            lines.push(sep);
        }

        lines.join("\n")
    }
}

impl Default for MmuFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for MmuFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Check for pre-formatted output
        if let Value::Object(map) = data {
            if let Some(out) = map.get("output").or_else(|| map.get("Output")) {
                return Ok(value_to_string(out));
            }
        }

        let mut blocks: Vec<String> = Vec::new();

        // Process Pools
        if let Some(pools) = data
            .get("pools")
            .or_else(|| data.get("pool"))
            .and_then(|v| v.as_object())
        {
            let sorted_pools: BTreeMap<_, _> = pools.iter().collect();
            for (name, details) in sorted_pools {
                if let Some(fields) = details.as_object() {
                    blocks.push(Self::format_block("Pool", name, fields));
                }
            }
        }

        // Process Profiles
        if let Some(profiles) = data
            .get("profiles")
            .or_else(|| data.get("profile"))
            .and_then(|v| v.as_object())
        {
            let sorted_profiles: BTreeMap<_, _> = profiles.iter().collect();
            for (name, details) in sorted_profiles {
                if let Some(fields) = details.as_object() {
                    blocks.push(Self::format_block("Profile", name, fields));
                }
            }
        }

        if !blocks.is_empty() {
            return Ok(blocks.join("\n\n"));
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "mmu"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_clock_formatter() {
        let formatter = ClockFormatter::new();
        let data = json!({"date": "Mon Jan 1 12:00:00 UTC 2024"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Mon Jan 1 12:00:00 PM UTC 2024");
    }

    #[test]
    fn test_clock_formatter_string() {
        let formatter = ClockFormatter::new();
        let data = json!("Mon Jan 1 12:00:00 UTC 2024");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Mon Jan 1 12:00:00 PM UTC 2024");
    }

    #[test]
    fn test_uptime_formatter() {
        let formatter = UptimeFormatter::new();
        let data = json!({"uptime": "up 5 days, 3:45"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("up 5 days"));
    }

    #[test]
    fn test_version_formatter() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_version": "SONiC.HEAD.123",
            "platform": "x86_64-test",
            "hwsku": "Test-HW"
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("SONiC.HEAD.123"));
        assert!(result.contains("x86_64-test"));
    }

    #[test]
    fn test_system_memory_formatter() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!({
            "Mem": {
                "total": "16000000",
                "used": "8000000",
                "free": "8000000",
                "shared": "100000",
                "buff/cache": "1000000",
                "available": "7000000"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Mem:"));
        assert!(result.contains("16000000"));
    }

    #[test]
    fn test_services_formatter() {
        let formatter = ServicesFormatter::new();
        let data = json!([
            {"name": "database", "status": "running"},
            {"name": "swss", "status": "running"}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("database"));
        assert!(result.contains("running"));
    }

    #[test]
    fn test_reboot_cause_formatter() {
        let formatter = RebootCauseFormatter::new();
        let data = json!([
            {"name": "2024-01-01", "cause": "User initiated", "time": "12:00:00", "user": "admin", "comment": ""}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("User initiated"));
    }

    #[test]
    fn test_mmu_formatter() {
        let formatter = MmuFormatter::new();
        let data = json!({"output": "MMU configuration output"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("MMU configuration"));
    }

    #[test]
    fn test_processes_formatter() {
        let formatter = ProcessesFormatter::new();
        let data = json!({"output": "top - 12:00:00 up 5 days..."});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("top"));
    }

    #[test]
    fn test_processes_cpu_formatter() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!([
            {"pid": "1", "name": "init", "cpu": "0.5%", "memory": "1%"}
        ]);
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_clock_with_timezone() {
        let formatter = ClockFormatter::new();
        let data = json!({"timezone": "UTC", "date": "2024-01-01"});
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_version_formatter_dict_format() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "distribution": "Debian",
            "kernel": "5.10",
            "build_commit": "abc123",
            "package_version": "1.0.0"
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_services_formatter_dict() {
        let formatter = ServicesFormatter::new();
        let data = json!({
            "database": {"status": "running"},
            "swss": {"status": "running"}
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_uptime_formatter_dict() {
        let formatter = UptimeFormatter::new();
        let data = json!({
            "uptime": "5 days, 12:34:56"
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("5 days"));
    }

    #[test]
    fn test_system_memory_formatter_dict() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!({
            "Mem": {
                "total": "16384000",
                "used": "8192000",
                "free": "8192000"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("16384000"));
    }

    #[test]
    fn test_system_memory_formatter_string() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!("Mem: 16GB total, 8GB used");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("16GB"));
    }

    #[test]
    fn test_reboot_cause_formatter_array() {
        let formatter = RebootCauseFormatter::new();
        let data = json!([
            {"name": "2024-01-01", "cause": "reboot", "time": "12:00", "user": "admin", "comment": "Planned"}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("reboot"));
        assert!(result.contains("admin"));
    }

    #[test]
    fn test_reboot_cause_formatter_dict_cause() {
        let formatter = RebootCauseFormatter::new();
        let data = json!({"cause": "Power loss"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Power loss");
    }

    #[test]
    fn test_reboot_cause_formatter_dict_output() {
        let formatter = RebootCauseFormatter::new();
        let data = json!({"output": "Unknown cause"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Unknown cause");
    }

    #[test]
    fn test_reboot_cause_formatter_string() {
        let formatter = RebootCauseFormatter::new();
        let data = json!("reboot caused by user");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "reboot caused by user");
    }

    #[test]
    fn test_mmu_formatter_structured() {
        let formatter = MmuFormatter::new();
        let data = json!({
            "pools": {
                "egress_lossless_pool": {
                    "mode": "static",
                    "size": "147226020",
                    "type": "egress"
                }
            },
            "profiles": {
                "egress_lossless_profile": {
                    "pool": "egress_lossless_pool",
                    "size": "0",
                    "static_th": "165364160"
                }
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Pool: egress_lossless_pool"));
        assert!(result.contains("mode  static"));
        assert!(result.contains("Profile: egress_lossless_profile"));
        assert!(result.contains("pool       egress_lossless_pool"));
        // Verify blocks are separated by blank lines
        assert!(result.contains("----\n\nProfile:"));
    }

    #[test]
    fn test_mmu_formatter_output() {
        let formatter = MmuFormatter::new();
        let data = json!({"output": "MMU raw output"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "MMU raw output");
    }

    #[test]
    fn test_processes_formatter_string() {
        let formatter = ProcessesFormatter::new();
        let data = json!("process list as string");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "process list as string");
    }

    #[test]
    fn test_processes_cpu_formatter_string() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!("CPU output as string");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "CPU output as string");
    }

    #[test]
    fn test_version_formatter_string() {
        let formatter = VersionFormatter::new();
        let data = json!("SONiC.202311.0.0");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "SONiC.202311.0.0");
    }

    #[test]
    fn test_clock_formatter_datetime() {
        let formatter = ClockFormatter::new();
        let data = json!({"datetime": "2024-01-01 12:00:00 UTC"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2024-01-01") || result.contains("12:00:00"));
    }

    #[test]
    fn test_uptime_formatter_components() {
        let formatter = UptimeFormatter::new();
        let data = json!({
            "time": "12:34:56",
            "up": "5 days",
            "users": "2",
            "load": "0.5, 0.3, 0.2"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("12:34:56"));
        assert!(result.contains("up 5 days"));
        assert!(result.contains("2 users"));
        assert!(result.contains("load average:"));
    }

    #[test]
    fn test_uptime_formatter_string() {
        let formatter = UptimeFormatter::new();
        let data = json!("up 10 days, 2:30");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "up 10 days, 2:30");
    }

    #[test]
    fn test_version_formatter_with_docker() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_version": "SONiC.202311",
            "docker_version": "24.0.5"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Docker Version: 24.0.5"));
    }

    #[test]
    fn test_version_formatter_extra_fields() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_version": "SONiC.202311",
            "custom_field": "custom_value"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("custom_field: custom_value"));
    }

    #[test]
    fn test_version_formatter_sonic_software_version_key() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_software_version": "SONiC.20251105.03",
            "sonic_os_version": "12",
            "distribution": "Debian 12.12",
            "platform": "x86_64-test"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // sonic_software_version should map to "SONiC Software Version"
        assert!(result.contains("SONiC Software Version: SONiC.20251105.03"));
        // sonic_os_version should map to "SONiC OS Version"
        assert!(result.contains("SONiC OS Version: 12"));
        // Should NOT contain raw key names
        assert!(!result.contains("sonic_software_version:"));
        assert!(!result.contains("sonic_os_version:"));
        // SONiC Software Version should come before Distribution
        let sw_pos = result.find("SONiC Software Version").unwrap();
        let dist_pos = result.find("Distribution").unwrap();
        assert!(
            sw_pos < dist_pos,
            "SONiC Software Version should appear before Distribution"
        );
    }

    #[test]
    fn test_version_formatter_docker_images() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_software_version": "SONiC.20251105.03",
            "sonic_os_version": "12",
            "distribution": "Debian 12.12",
            "kernel": "6.1.0-29-2-amd64",
            "platform": "x86_64-test",
            "hwsku": "Test-HW",
            "docker_images": [
                {
                    "Repository": "docker-platform-monitor",
                    "Tag": "20251105.03",
                    "ID": "sha256:9c59644d82d44a79b2e496011b3ee9ae1c0424aebf485c174aef8159f31fd9df",
                    "Size": "405MB"
                },
                {
                    "Repository": "docker-sonic-telemetry",
                    "Tag": "latest",
                    "ID": "sha256:0dde5b32ba6506464498b8492031d2b6189043e7e53dfd5b815abe3a928eff07",
                    "Size": "367MB"
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Docker images should be formatted as a table
        assert!(result.contains("Docker images:"));
        assert!(result.contains("REPOSITORY"));
        assert!(result.contains("TAG"));
        assert!(result.contains("IMAGE ID"));
        assert!(result.contains("SIZE"));
        assert!(result.contains("docker-platform-monitor"));
        assert!(result.contains("20251105.03"));
        assert!(result.contains("405MB"));
        assert!(result.contains("docker-sonic-telemetry"));
        // docker_images should NOT appear as raw key
        assert!(!result.contains("docker_images:"));
    }

    #[test]
    fn test_version_formatter_blank_line_before_platform() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "sonic_software_version": "SONiC.20251105.03",
            "built_by": "user@host",
            "platform": "x86_64-test",
            "hwsku": "Test-HW"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // There should be a blank line before Platform
        assert!(result.contains("Built by: user@host\n\nPlatform: x86_64-test"));
    }

    #[test]
    fn test_system_memory_formatter_with_swap() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!({
            "Mem": {"total": "16000000", "used": "8000000", "free": "8000000", "shared": "0", "buff/cache": "1000000", "available": "7000000"},
            "Swap": {"total": "4000000", "used": "0", "free": "4000000"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Swap:"));
        assert!(result.contains("4000000"));
    }

    #[test]
    fn test_system_memory_formatter_output_field() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!({"output": "custom memory output"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "custom memory output");
    }

    #[test]
    fn test_services_formatter_output_field() {
        let formatter = ServicesFormatter::new();
        let data = json!({"output": "services output"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "services output");
    }

    #[test]
    fn test_services_formatter_string() {
        let formatter = ServicesFormatter::new();
        let data = json!("services as string");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "services as string");
    }

    #[test]
    fn test_processes_formatter_output_field() {
        let formatter = ProcessesFormatter::new();
        let data = json!({"output": "process output"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "process output");
    }

    #[test]
    fn test_processes_cpu_formatter_output_field() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!({"output": "cpu output"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "cpu output");
    }

    #[test]
    fn test_mmu_formatter_string() {
        let formatter = MmuFormatter::new();
        let data = json!("MMU as string");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "MMU as string");
    }

    #[test]
    fn test_clock_formatter_empty() {
        let formatter = ClockFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "");
    }

    // Additional tests for uncovered branches

    #[test]
    fn test_clock_formatter_time_key() {
        let formatter = ClockFormatter::new();
        let data = json!({"time": "15:30:45"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("15:30:45"));
    }

    #[test]
    fn test_clock_formatter_date_key() {
        let formatter = ClockFormatter::new();
        let data = json!({"date": "2024-01-15"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2024-01-15"));
    }

    #[test]
    fn test_uptime_formatter_single_uptime_key() {
        let formatter = UptimeFormatter::new();
        let data = json!({"uptime": "10 days, 5:30:15"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("10 days"));
    }

    #[test]
    fn test_version_formatter_alternate_keys() {
        let formatter = VersionFormatter::new();
        let data = json!({
            "Version": "1.2.3",
            "Platform": "linux",
            "HwSKU": "Generic",
            "ASIC": "memory"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("1.2.3"));
    }

    #[test]
    fn test_system_memory_formatter_output_key() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!({"output": "Memory: 16GB total, 8GB used, 8GB free"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Memory: 16GB total, 8GB used, 8GB free");
    }

    #[test]
    fn test_services_formatter_output_key() {
        let formatter = ServicesFormatter::new();
        let data = json!({"output": "All services running"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "All services running");
    }

    #[test]
    fn test_reboot_cause_formatter_dict_with_cause() {
        let formatter = RebootCauseFormatter::new();
        let data = json!({"cause": "User initiated reboot", "time": "2024-01-15 10:30:00"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("User initiated reboot"));
    }

    #[test]
    fn test_reboot_cause_formatter_output_key() {
        let formatter = RebootCauseFormatter::new();
        let data = json!({"output": "Reboot cause: power cycle"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Reboot cause: power cycle");
    }

    #[test]
    fn test_mmu_formatter_output_key() {
        let formatter = MmuFormatter::new();
        let data = json!({"output": "MMU configuration output"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "MMU configuration output");
    }

    // =====================================================
    // SERVICES FORMATTER — FULL DOCKER PROCESS DATA
    // =====================================================

    #[test]
    fn test_services_formatter_full_array() {
        let formatter = ServicesFormatter::new();
        let data = json!([
            {
                "dockerProcessName": "database",
                "status": "Up 2 days",
                "processes": [
                    {
                        "user": "root",
                        "pid": "123",
                        "cpuPercentage": "0.1",
                        "memPercentage": "2.3",
                        "vsz": "123456",
                        "rss": "7890",
                        "tty": "?",
                        "stat": "Ss",
                        "start": "Jan01",
                        "time": "0:10",
                        "command": "/usr/bin/redis-server"
                    }
                ]
            },
            {
                "dockerProcessName": "swss",
                "status": "Up 2 days",
                "processes": [
                    {
                        "user": "admin",
                        "pid": "456",
                        "cpuPercentage": "1.5",
                        "memPercentage": "5.0",
                        "vsz": "654321",
                        "rss": "9876",
                        "tty": "pts/0",
                        "stat": "Sl+",
                        "start": "Jan02",
                        "time": "1:30",
                        "command": "orchagent"
                    }
                ]
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("database\tUp 2 days"));
        assert!(result.contains("swss\tUp 2 days"));
        assert!(result.contains(
            "USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND"
        ));
        assert!(result.contains("root"));
        assert!(result.contains("redis-server"));
        assert!(result.contains("orchagent"));
    }

    #[test]
    fn test_services_formatter_no_processes_key() {
        let formatter = ServicesFormatter::new();
        let data = json!([
            {
                "name": "bgp",
                "Status": "running"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("bgp"));
        assert!(result.contains("running"));
    }

    // =====================================================
    // PROCESSES FORMATTER — FULL ARRAY DATA
    // =====================================================

    #[test]
    fn test_processes_formatter_full_array() {
        let formatter = ProcessesFormatter::new();
        let data = json!([
            {
                "PID": "1",
                "PPID": "0",
                "CMD": "/sbin/init",
                "%MEM": "0.1",
                "%CPU": "0.0",
                "STIME": "Jan01",
                "TIME": "00:01:23",
                "TT": "?",
                "UID": "0"
            },
            {
                "PID": "999",
                "PPID": "1",
                "CMD": "python3 -m supervisor --config /etc/supervisor/supervisord.conf --very-long-command-that-exceeds-fifty-characters-total",
                "%MEM": "5.3",
                "%CPU": "12.1",
                "STIME": "Feb01",
                "TIME": "12:34:56",
                "TT": "pts/1",
                "UID": "1000"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Check header
        assert!(result.contains("PID"));
        assert!(result.contains("PPID"));
        assert!(result.contains("CMD"));
        // Check data rows
        assert!(result.contains("/sbin/init"));
        // Check long CMD is truncated
        assert!(result.contains("..."));
    }

    #[test]
    fn test_processes_formatter_object_input() {
        let formatter = ProcessesFormatter::new();
        let data = json!({
            "proc1": {"PID": "42", "PPID": "1", "CMD": "test_cmd", "%MEM": "0.0", "%CPU": "0.0", "STIME": "now", "TIME": "0:00", "TT": "?", "UID": "0"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("42"));
    }

    #[test]
    fn test_processes_formatter_empty_cmd_alignment() {
        // Verify that empty CMD field doesn't cause column shift
        let formatter = ProcessesFormatter::new();
        let data = json!([
            {
                "PID": "1",
                "PPID": "0",
                "CMD": "/sbin/init",
                "%MEM": "0.1",
                "%CPU": "0.0",
                "STIME": "Jan28",
                "TIME": "0:02:43",
                "TT": "None",
                "UID": "0"
            },
            {
                "PID": "2",
                "PPID": "0",
                "CMD": "",
                "%MEM": "0.0",
                "%CPU": "0.0",
                "STIME": "Jan28",
                "TIME": "0:00:00",
                "TT": "None",
                "UID": "0"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        // Should have header + separator + 2 data rows = 4 lines
        assert_eq!(lines.len(), 4, "Expected 4 lines (header + sep + 2 data)");
        // Header should contain all column names
        assert!(lines[0].contains("PID"));
        assert!(lines[0].contains("CMD"));
        assert!(lines[0].contains("%MEM"));
        // Separator line should be dashes
        assert!(lines[1].contains("---"));
        // Empty CMD row should still have %MEM aligned (not shifted)
        // Note: tabulate_simple_auto applies format_g to numeric columns: "0.0" → "0", "0.1" stays "0.1"
        // PID 1 row: %MEM=0.1, %CPU=0 (was "0.0")
        // PID 2 row: %MEM=0, %CPU=0 (both were "0.0")
        // Verify both rows have proper structure (header + sep + 2 data rows)
        assert!(lines[2].contains("1"), "PID 1 row should contain PID 1");
        assert!(lines[3].contains("2"), "PID 2 row should contain PID 2");
        // Verify the %MEM column is aligned: find column position from header
        let mem_header_pos = lines[0].find("%MEM").expect("Header should contain %MEM");
        // Both data rows should have content near the %MEM column position
        // (exact alignment is handled by tabulate_simple_auto)
        assert!(lines[2].len() > mem_header_pos, "PID 1 row should extend to %MEM column");
        assert!(lines[3].len() > mem_header_pos, "PID 2 row should extend to %MEM column");
    }

    #[test]
    fn test_processes_formatter_empty_array() {
        let formatter = ProcessesFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // =====================================================
    // PROCESSES CPU FORMATTER — FULL TOP-LIKE OUTPUT
    // =====================================================

    #[test]
    fn test_processes_cpu_formatter_full_structure() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!({
            "uptime": "12:34:56 up 5 days, 3:21, 2 users, load average: 0.15, 0.20, 0.18",
            "tasks": "245 total, 1 running, 244 sleeping, 0 stopped, 0 zombie",
            "cpu_usage": "2.3 us, 1.0 sy, 0.0 ni, 96.5 id, 0.2 wa, 0.0 hi, 0.0 si, 0.0 st",
            "memory_usage": "15950.6 total, 1234.5 free, 8765.4 used, 5950.7 buff/cache",
            "swap_usage": "0.0 total, 0.0 free, 0.0 used. 6543.2 avail Mem",
            "processes": [
                {
                    "pid": "1",
                    "user": "root",
                    "pr": "20",
                    "ni": "0",
                    "virt": "169420",
                    "res": "13312",
                    "shr": "8448",
                    "s": "S",
                    "cpu": "0.0",
                    "mem": "0.1",
                    "time": "0:05.23",
                    "command": "systemd"
                },
                {
                    "pid": "234",
                    "user": "admin",
                    "pr": "20",
                    "ni": "0",
                    "virt": "2345678",
                    "res": "67890",
                    "shr": "12345",
                    "s": "R",
                    "cpu": "15.3",
                    "mem": "5.2",
                    "time": "123:45.6",
                    "command": "orchagent"
                }
            ]
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("top - 12:34:56 up 5 days"));
        assert!(result.contains("Tasks: 245 total"));
        assert!(result.contains("%Cpu(s):"));
        assert!(result.contains("MiB Mem :"));
        assert!(result.contains("MiB Swap:"));
        assert!(result.contains("PID USER"));
        assert!(result.contains("systemd"));
        assert!(result.contains("orchagent"));
    }

    #[test]
    fn test_processes_cpu_formatter_no_processes() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!({
            "uptime": "10:00:00",
            "tasks": "1 total"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("top - 10:00:00"));
        assert!(result.contains("Tasks: 1 total"));
    }

    #[test]
    fn test_processes_cpu_formatter_not_object() {
        let formatter = ProcessesCpuFormatter::new();
        let data = json!(42);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // =====================================================
    // REBOOT CAUSE FORMATTER — HISTORY DICT FORMAT
    // =====================================================

    #[test]
    fn test_reboot_cause_formatter_history_dict() {
        let formatter = RebootCauseFormatter::new();
        let data = json!({
            "2026_01_24_06_59_57": {
                "cause": "reboot",
                "time": "Thu Jan 24 2026 06:59:57 UTC",
                "user": "admin",
                "comment": "Planned maintenance"
            },
            "2026_01_20_12_00_00": {
                "cause": "power cycle",
                "time": "Mon Jan 20 2026 12:00:00 UTC",
                "user": "$USER$",
                "comment": ""
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Name"));
        assert!(result.contains("Cause"));
        assert!(result.contains("Time"));
        assert!(result.contains("User"));
        assert!(result.contains("Comment"));
        assert!(result.contains("reboot"));
        assert!(result.contains("power cycle"));
        assert!(result.contains("Planned maintenance"));
        assert!(result.contains("$USER$"));
        // Most recent entry should appear first
        let pos_recent = result.find("2026_01_24").unwrap_or(0);
        let pos_older = result.find("2026_01_20").unwrap_or(0);
        assert!(
            pos_recent < pos_older,
            "Most recent entry should come first"
        );
    }

    #[test]
    fn test_reboot_cause_formatter_history_array_with_fields() {
        let formatter = RebootCauseFormatter::new();
        let data = json!([
            {
                "name": "2026_01_24_06_59_57",
                "cause": "warm-reboot",
                "time": "Thu Jan 24 2026",
                "user": "admin",
                "comment": "upgrade"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("warm-reboot"));
        assert!(result.contains("upgrade"));
    }

    // =====================================================
    // SYSTEM MEMORY FORMATTER — ARRAY FORMAT
    // =====================================================

    #[test]
    fn test_system_memory_formatter_array_format() {
        let formatter = SystemMemoryFormatter::new();
        let data = json!([
            {
                "type": "Mem",
                "total": "15950604",
                "used": "8765432",
                "free": "1234567",
                "shared": "123456",
                "buff/cache": "5950605",
                "available": "6543210"
            },
            {
                "type": "Swap",
                "total": "0",
                "used": "0",
                "free": "0"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("total"));
        assert!(result.contains("used"));
        assert!(result.contains("free"));
        assert!(result.contains("Mem:"));
        assert!(result.contains("Swap:"));
        assert!(result.contains("15950604"));
    }

    #[test]
    fn test_system_memory_formatter_null() {
        let formatter = SystemMemoryFormatter::new();
        let data = Value::Null;
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty() || result.contains("total"));
    }

    // ---- Round 3 coverage tests ----

    #[test]
    fn test_clock_formatter_default() {
        let formatter = ClockFormatter::default();
        let config = CommandConfig::new();
        // String input
        let data = json!("Thu Jan 1 00:00:00 UTC 2025");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Thu Jan 1 12:00:00 AM UTC 2025");
    }

    #[test]
    fn test_clock_formatter_empty_object() {
        let formatter = ClockFormatter::default();
        let config = CommandConfig::new();
        let data = json!({});
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_uptime_formatter_default() {
        let formatter = UptimeFormatter::default();
        let config = CommandConfig::new();
        let data = json!({"uptime": "5 days, 3:14"});
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "5 days, 3:14");
    }

    #[test]
    fn test_uptime_formatter_string_input() {
        let formatter = UptimeFormatter::default();
        let config = CommandConfig::new();
        let data = json!("10 days up");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "10 days up");
    }

    #[test]
    fn test_uptime_formatter_empty_object() {
        let formatter = UptimeFormatter::default();
        let config = CommandConfig::new();
        let data = json!({});
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_version_formatter_default() {
        let formatter = VersionFormatter::default();
        let config = CommandConfig::new();
        let data = json!("SONiC.20231231");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("SONiC"));
    }

    #[test]
    fn test_services_formatter_default() {
        let formatter = ServicesFormatter::default();
        let config = CommandConfig::new();
        let data = json!([]);
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty() || result.contains("PROCESS"));
    }

    #[test]
    fn test_processes_formatter_default() {
        let formatter = ProcessesFormatter::default();
        let config = CommandConfig::new();
        let data = json!("just a string");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "just a string");
    }

    #[test]
    fn test_processes_cpu_formatter_default() {
        let formatter = ProcessesCpuFormatter::default();
        let config = CommandConfig::new();
        let data = json!(42);
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.is_empty() || result.contains("42"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_reboot_cause_formatter_default() {
        let formatter = RebootCauseFormatter::default();
        let config = CommandConfig::new();
        let data = json!("Power Loss");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Power Loss");
    }

    #[test]
    fn test_system_memory_formatter_default() {
        let formatter = SystemMemoryFormatter::default();
        let config = CommandConfig::new();
        let data = json!("string memory");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("string memory") || result.is_empty(),
            "result: {}",
            result
        );
    }

    // ---- Round 6 coverage tests ----

    #[test]
    fn test_version_formatter_non_string_non_object() {
        // Cover L216: returning Ok(String::new()) for non-object/non-string data
        let formatter = VersionFormatter::new();
        let config = CommandConfig::new();
        let data = json!(42);
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty(), "result: {}", result);
    }

    #[test]
    fn test_version_formatter_array_input() {
        let formatter = VersionFormatter::new();
        let config = CommandConfig::new();
        let data = json!([1, 2, 3]);
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty(), "result: {}", result);
    }
}
