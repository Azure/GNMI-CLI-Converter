//! Transceiver-related formatters for presence, info, status, eeprom, etc.

use crate::common::types::CommandConfig;
use crate::formatters::base::{
    tabulate_simple_auto, value_to_string, FormatError,
    OutputFormatter,
};
use crate::utils::sorting::natural_cmp;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::Value;
use std::collections::HashMap;

/// Formatter for transceiver presence output.
pub struct TransceiverPresenceFormatter;

impl TransceiverPresenceFormatter {
    /// Creates a new [`TransceiverPresenceFormatter`].
    pub fn new() -> Self {
        Self
    }

    /// Format table using tabulate "simple" format (left-aligned, no trailing padding)
    fn format_presence_table(headers: &[&str], rows: &[(String, String)]) -> String {
        if rows.is_empty() {
            return String::new();
        }

        // Calculate column widths using Python tabulate formula: max(header_len + 2, max_data_len)
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();

        for (port, presence) in rows {
            if port.len() > widths[0] {
                widths[0] = port.len();
            }
            if presence.len() > widths[1] {
                widths[1] = presence.len();
            }
        }

        let mut lines = Vec::new();

        // Header row (left-aligned, no trailing padding on last column)
        let header_line = format!(
            "{:<width0$}  {h1}",
            headers[0],
            h1 = headers[1],
            width0 = widths[0]
        );
        lines.push(header_line);

        // Separator row
        let sep_line = format!("{}  {}", "-".repeat(widths[0]), "-".repeat(widths[1]));
        lines.push(sep_line);

        // Data rows (no trailing padding on last column)
        for (port, presence) in rows {
            let data_line = format!(
                "{:<width0$}  {presence}",
                port,
                presence = presence,
                width0 = widths[0]
            );
            lines.push(data_line);
        }

        lines.join("\n")
    }
}

impl Default for TransceiverPresenceFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverPresenceFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Helper to normalize presence values (e.g., "Not Present" -> "Not present")
        fn normalize_presence(presence: &str) -> String {
            if presence.eq_ignore_ascii_case("not present") {
                "Not present".to_string()
            } else {
                presence.to_string()
            }
        }

        let mut rows: Vec<(String, String)> = Vec::new();

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let presence_raw = obj
                        .get("presence")
                        .or_else(|| obj.get("Presence"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let presence = normalize_presence(&presence_raw);

                    if !port.is_empty() {
                        rows.push((port, presence));
                    }
                }
            }
            // Sort by port name using natural sort
            rows.sort_by(|a, b| natural_cmp(&a.0, &b.0));
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for key in keys {
                if let Some(val) = map.get(key) {
                    let presence_raw = if let Value::Object(obj) = val {
                        obj.get("presence")
                            .or_else(|| obj.get("Presence"))
                            .map(value_to_string)
                            .unwrap_or_else(|| value_to_string(val))
                    } else {
                        value_to_string(val)
                    };
                    let presence = normalize_presence(&presence_raw);
                    rows.push((key.clone(), presence));
                }
            }
        }

        if rows.is_empty() {
            // Return header row for empty input
            let headers = ["Port", "Presence"];
            let header_line = format!("{:<6}  {}", headers[0], headers[1]);
            let sep_line = format!("{}  {}", "-".repeat(6), "-".repeat(10));
            return Ok(format!("{}\n{}", header_line, sep_line));
        }

        let headers = ["Port", "Presence"];
        Ok(Self::format_presence_table(&headers, &rows))
    }

    fn name(&self) -> &'static str {
        "transceiver_presence"
    }
}

/// Formatter for transceiver info output.
pub struct TransceiverInfoFormatter;

impl TransceiverInfoFormatter {
    /// Creates a new [`TransceiverInfoFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverInfoFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverInfoFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // TransceiverInfoFormatter delegates to the same format as EEPROM
        format_eeprom_output(data)
    }

    fn name(&self) -> &'static str {
        "transceiver_info"
    }
}

/// Formatter for transceiver status output (DOM data).
pub struct TransceiverStatusFormatter;

impl TransceiverStatusFormatter {
    /// Creates a new [`TransceiverStatusFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverStatusFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverStatusFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Ordered fields for transceiver status output
        let field_order = vec![
            "CMIS State (SW)",
            "Current module state",
            "Reason of entering the module fault state",
            "Datapath firmware fault",
            "Module firmware fault",
            "Module state changed",
            "Temperature high alarm flag",
            "Temperature high warning flag",
            "Temperature low warning flag",
            "Temperature low alarm flag",
            "Vcc high alarm flag",
            "Vcc high warning flag",
            "Vcc low warning flag",
            "Vcc low alarm flag",
        ];

        let mut lines = Vec::new();

        if let Value::Array(arr) = data {
            if arr.is_empty() {
                return Ok("Port  Temperature".to_string());
            }
            // Convert array items to interface-keyed format
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    if port.is_empty() {
                        continue;
                    }
                    lines.push(format!("{}: ", port));
                    // Output fields (excluding port key)
                    let mut printed_keys = std::collections::HashSet::new();
                    for field in &field_order {
                        if let Some(val) = obj.get(*field) {
                            lines.push(format!("        {}: {}", field, value_to_string(val)));
                            printed_keys.insert(field.to_string());
                        }
                    }
                    let mut remaining: Vec<&String> = obj
                        .keys()
                        .filter(|k| {
                            !printed_keys.contains(k.as_str())
                                && *k != "port"
                                && *k != "Port"
                                && *k != "interface"
                        })
                        .collect();
                    remaining.sort();
                    for key in remaining {
                        if let Some(val) = obj.get(key) {
                            lines.push(format!("        {}: {}", key, value_to_string(val)));
                        }
                    }
                }
            }
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for iface in keys {
                if let Some(status_data) = map.get(iface) {
                    // Parse JSON string if needed
                    let parsed = match status_data {
                        Value::String(s) => match serde_json::from_str::<Value>(s) {
                            Ok(v) => v,
                            Err(_) => {
                                lines.push(format!("{}: {}", iface, s));
                                continue;
                            }
                        },
                        _ => status_data.clone(),
                    };

                    if let Value::Object(obj) = &parsed {
                        // Check for REMOVED state
                        let cmis_state = obj
                            .get("CMIS State (SW)")
                            .map(value_to_string)
                            .unwrap_or_default();
                        if cmis_state == "REMOVED" {
                            lines
                                .push(format!("{}: Transceiver status info not applicable", iface));
                            lines.push(String::new());
                            continue;
                        }

                        lines.push(format!("{}: ", iface));

                        // Output ordered fields first
                        let mut printed_keys = std::collections::HashSet::new();
                        for field in &field_order {
                            if let Some(val) = obj.get(*field) {
                                lines.push(format!("        {}: {}", field, value_to_string(val)));
                                printed_keys.insert(field.to_string());
                            }
                        }

                        // Then remaining keys sorted alphabetically
                        let mut remaining: Vec<&String> = obj
                            .keys()
                            .filter(|k| !printed_keys.contains(k.as_str()))
                            .collect();
                        remaining.sort();
                        for key in remaining {
                            if let Some(val) = obj.get(key) {
                                lines.push(format!("        {}: {}", key, value_to_string(val)));
                            }
                        }
                        lines.push(String::new());
                    }
                }
            }
        }

        if lines.is_empty() {
            if let Value::String(s) = data {
                return Ok(s.clone());
            }
        }

        // Remove trailing empty line if present
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "transceiver_status"
    }
}

/// Formatter for transceiver EEPROM output.
pub struct TransceiverEepromFormatter;

impl TransceiverEepromFormatter {
    /// Creates a new [`TransceiverEepromFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverEepromFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverEepromFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        format_eeprom_output(data)
    }

    fn name(&self) -> &'static str {
        "transceiver_eeprom"
    }
}

/// Shared eeprom formatting logic used by both TransceiverEepromFormatter and TransceiverInfoFormatter
fn format_eeprom_output(data: &Value) -> Result<String, FormatError> {
    let mut lines = Vec::new();

    if let Value::Array(arr) = data {
        if arr.is_empty() {
            // Return header-like output for empty arrays
            return Ok("Port  Identifier".to_string());
        }
        // Convert array items to interface-keyed format and delegate
        for item in arr {
            if let Value::Object(obj) = item {
                let port = obj
                    .get("port")
                    .or_else(|| obj.get("Port"))
                    .or_else(|| obj.get("interface"))
                    .map(value_to_string)
                    .unwrap_or_default();
                if port.is_empty() {
                    continue;
                }
                // Build a new object without the port key
                let mut fields = serde_json::Map::new();
                for (k, v) in obj {
                    if k != "port" && k != "Port" && k != "interface" {
                        fields.insert(k.clone(), v.clone());
                    }
                }
                if fields.is_empty() {
                    lines.push(format!("{}: SFP EEPROM Not detected", port));
                } else {
                    lines.push(format!("{}: SFP EEPROM detected", port));
                    let mut sorted_keys: Vec<&String> = fields.keys().collect();
                    sorted_keys.sort();
                    for key in sorted_keys {
                        if let Some(val) = fields.get(key) {
                            if key == "Application Advertisement" {
                                format_app_advertisement(&mut lines, val);
                            } else {
                                let val_str = value_to_string(val);
                                lines.push(format!("        {}: {}", key, val_str));
                            }
                        }
                    }
                }
                lines.push(String::new());
            }
        }
        // Remove trailing empty lines and return
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        return Ok(lines.join("\n"));
    }

    if let Value::Object(map) = data {
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort_by(|a, b| natural_cmp(a, b));

        for iface in keys {
            if let Some(eeprom_data) = map.get(iface) {
                // Parse the value - it may be a double-encoded JSON string
                let parsed = match eeprom_data {
                    Value::String(s) => {
                        if s.is_empty() || s == "{}" {
                            // Empty EEPROM data
                            lines.push(format!("{}: SFP EEPROM Not detected", iface));
                            lines.push(String::new());
                            continue;
                        }
                        // Try to parse as JSON
                        match serde_json::from_str::<Value>(s) {
                            Ok(Value::Object(obj)) if obj.is_empty() => {
                                lines.push(format!("{}: SFP EEPROM Not detected", iface));
                                lines.push(String::new());
                                continue;
                            }
                            Ok(v) => v,
                            Err(_) => {
                                // Not valid JSON, treat as raw string
                                lines.push(format!("{}: SFP EEPROM detected", iface));
                                lines.push(format!("        {}", s));
                                lines.push(String::new());
                                continue;
                            }
                        }
                    }
                    Value::Object(obj) if obj.is_empty() => {
                        lines.push(format!("{}: SFP EEPROM Not detected", iface));
                        lines.push(String::new());
                        continue;
                    }
                    Value::Object(_) => eeprom_data.clone(),
                    Value::Null => {
                        lines.push(format!("{}: SFP EEPROM Not detected", iface));
                        lines.push(String::new());
                        continue;
                    }
                    _ => {
                        lines.push(format!("{}: SFP EEPROM Not detected", iface));
                        lines.push(String::new());
                        continue;
                    }
                };

                if let Value::Object(obj) = &parsed {
                    if obj.is_empty() {
                        lines.push(format!("{}: SFP EEPROM Not detected", iface));
                        lines.push(String::new());
                        continue;
                    }

                    lines.push(format!("{}: SFP EEPROM detected", iface));

                    // Sort keys alphabetically (case-sensitive to match Python sorted())
                    let mut sorted_keys: Vec<&String> = obj.keys().collect();
                    sorted_keys.sort();

                    for key in sorted_keys {
                        if let Some(val) = obj.get(key) {
                            if key == "Application Advertisement" {
                                // Special handling for Application Advertisement
                                format_app_advertisement(&mut lines, val);
                            } else {
                                match val {
                                    Value::Object(inner) if inner.is_empty() => {
                                        lines.push(format!("        {}:", key));
                                    }
                                    Value::Array(arr) if arr.is_empty() => {
                                        lines.push(format!("        {}:", key));
                                    }
                                    _ => {
                                        let val_str = value_to_string(val);
                                        lines.push(format!("        {}: {}", key, val_str));
                                    }
                                }
                            }
                        }
                    }
                    lines.push(String::new());
                }
            }
        }
    }

    if lines.is_empty() {
        if let Value::String(s) = data {
            return Ok(s.clone());
        }
    }

    // Remove trailing empty lines
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }

    Ok(lines.join("\n"))
}

/// Sort key for Application Advertisement entries by speed.
/// Matches Python's _get_app_adv_sort_key: start-anchored patterns, returns (order, entry) tuple.
fn app_adv_sort_key(entry: &str) -> (i64, String) {
    let upper = entry.to_uppercase();
    let order = if upper.starts_with("800G-ETC") {
        0
    } else if upper.starts_with("400GBASE") {
        1
    } else if let Some(stripped) = upper.strip_prefix("400G") {
        // Match "400G CR" — check that char after "400G" (possibly whitespace) is followed by "CR"
        let rest = stripped.trim_start();
        if rest.starts_with("CR") { 2 } else { 99 }
    } else if upper.starts_with("200GBASE") {
        3
    } else if upper.starts_with("100GBASE") {
        4
    } else if upper.starts_with("40GBASE") {
        5
    } else {
        99
    };
    (order, entry.to_string())
}

/// Format Application Advertisement entries with proper alignment
fn format_app_advertisement(lines: &mut Vec<String>, val: &Value) {
    let prefix = "        Application Advertisement: ";
    let continuation_indent = " ".repeat(prefix.len());

    match val {
        Value::String(s) => {
            if s.is_empty() {
                lines.push("        Application Advertisement:".to_string());
                return;
            }
            // Multiple entries separated by newlines
            let entries: Vec<&str> = s.split('\n').collect();
            let mut sorted_entries: Vec<&str> = entries
                .iter()
                .map(|e| e.trim())
                .filter(|e| !e.is_empty())
                .collect();
            sorted_entries.sort_by_key(|a| app_adv_sort_key(a));

            for (i, entry) in sorted_entries.iter().enumerate() {
                if i == 0 {
                    lines.push(format!("{}{}", prefix, entry));
                } else {
                    lines.push(format!("{}{}", continuation_indent, entry));
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                lines.push("        Application Advertisement:".to_string());
                return;
            }
            let mut entries: Vec<String> = arr.iter().map(value_to_string).collect();
            entries.sort_by_key(|a| app_adv_sort_key(a));

            for (i, entry) in entries.iter().enumerate() {
                if i == 0 {
                    lines.push(format!("{}{}", prefix, entry));
                } else {
                    lines.push(format!("{}{}", continuation_indent, entry));
                }
            }
        }
        _ => {
            let val_str = value_to_string(val);
            lines.push(format!("{}{}", prefix, val_str));
        }
    }
}

/// Formatter for transceiver error status output.
pub struct TransceiverErrorStatusFormatter;

impl TransceiverErrorStatusFormatter {
    /// Creates a new [`TransceiverErrorStatusFormatter`].
    pub fn new() -> Self {
        Self
    }

    /// Compute error status from port data fields.
    /// Maps status/cmis_state/error fields to human-readable status.
    fn compute_error_status(port_data: &serde_json::Map<String, Value>) -> String {
        // Check for direct error_status field first
        if let Some(es) = port_data.get("error_status") {
            return value_to_string(es);
        }
        let status = port_data
            .get("status")
            .map(value_to_string)
            .unwrap_or_else(|| "0".to_string());
        let cmis_state = port_data
            .get("cmis_state")
            .map(value_to_string)
            .unwrap_or_default();
        let error = port_data
            .get("error")
            .map(value_to_string)
            .unwrap_or_else(|| "N/A".to_string());

        // When status is "1" and either no cmis_state or cmis_state is READY, it's OK
        if status == "1" {
            if cmis_state.is_empty() || cmis_state == "N/A" || cmis_state == "READY" {
                "OK".to_string()
            } else if !error.is_empty() && error != "N/A" {
                error
            } else {
                cmis_state
            }
        } else {
            // status is "0" - unplugged or error
            if !error.is_empty() && error != "N/A" {
                error
            } else {
                "Unplugged".to_string()
            }
        }
    }

    /// Check if data is single interface data (has status/error/cmis_state/error_status at top level)
    fn is_single_interface_data(obj: &serde_json::Map<String, Value>) -> bool {
        obj.contains_key("status")
            || obj.contains_key("error")
            || obj.contains_key("cmis_state")
            || obj.contains_key("error_status")
    }

    /// Format table using tabulate "simple" format (left-aligned, no trailing padding)
    fn format_error_status_table(headers: &[&str], rows: &[Vec<String>]) -> String {
        // Calculate column widths using Python tabulate formula: max(header_len + 2, max_data_len)
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() && cell.len() > widths[i] {
                    widths[i] = cell.len();
                }
            }
        }

        let mut lines = Vec::new();

        // Header row (left-aligned, no trailing padding on last column)
        let header_line: String = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                if i == headers.len() - 1 {
                    h.to_string()
                } else {
                    format!("{:<width$}", h, width = widths[i])
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        lines.push(header_line);

        // Separator row
        let sep_line: String = widths
            .iter()
            .map(|&w| "-".repeat(w))
            .collect::<Vec<_>>()
            .join("  ");
        lines.push(sep_line);

        // Data rows (left-aligned, no trailing padding on last column)
        for row in rows {
            let row_line: String = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let width = widths.get(i).copied().unwrap_or(cell.len());
                    if i == row.len() - 1 {
                        cell.clone()
                    } else {
                        format!("{:<width$}", cell, width = width)
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            lines.push(row_line);
        }

        lines.join("\n")
    }
}

impl Default for TransceiverErrorStatusFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverErrorStatusFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut table_data: Vec<Vec<String>> = Vec::new();

        if let Value::Array(arr) = data {
            if arr.is_empty() {
                // Return header row for empty input
                let headers = ["Port", "Error Status"];
                return Ok(Self::format_error_status_table(&headers, &[]));
            }
            // List of dicts format
            let mut items: Vec<(String, String)> = Vec::new();
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("Port")
                        .or_else(|| obj.get("port"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let error_status = obj
                        .get("Error Status")
                        .or_else(|| obj.get("error_status"))
                        .or_else(|| obj.get("status"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    if !port.is_empty() {
                        items.push((port, error_status));
                    }
                }
            }
            items.sort_by(|a, b| natural_cmp(&a.0, &b.0));
            for (port, status) in items {
                table_data.push(vec![port, status]);
            }
        } else if let Value::Object(map) = data {
            if Self::is_single_interface_data(map) && !map.values().any(|v| v.is_object()) {
                // Single interface data at top level (not interface-keyed dict)
                let interface_name = options
                    .and_then(|o| o.get("argument"))
                    .cloned()
                    .unwrap_or_else(|| "Unknown".to_string());
                let error_status = Self::compute_error_status(map);
                table_data.push(vec![interface_name, error_status]);
            } else {
                // Dict format with interface names as keys
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort_by(|a, b| natural_cmp(a, b));

                for key in keys {
                    if let Some(Value::Object(port_data)) = map.get(key) {
                        let error_status = Self::compute_error_status(port_data);
                        table_data.push(vec![key.clone(), error_status]);
                    } else if let Some(val) = map.get(key) {
                        table_data.push(vec![key.clone(), value_to_string(val)]);
                    }
                }
            }
        }

        if table_data.is_empty() {
            return Ok(String::new());
        }

        let headers = ["Port", "Error Status"];
        Ok(Self::format_error_status_table(&headers, &table_data))
    }

    fn name(&self) -> &'static str {
        "transceiver_error_status"
    }
}

/// Formatter for transceiver low power mode output.
pub struct TransceiverLpmodeFormatter;

impl TransceiverLpmodeFormatter {
    /// Creates a new [`TransceiverLpmodeFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverLpmodeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverLpmodeFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers: &[&str] = &["Port", "Low-power Mode"];
        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let lpmode = obj
                        .get("lpmode")
                        .or_else(|| obj.get("Low-power Mode"))
                        .or_else(|| obj.get("Low Power Mode"))
                        .or_else(|| obj.get("low_power_mode"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    rows.push(vec![port, lpmode]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for key in keys {
                if let Some(val) = map.get(key) {
                    let lpmode = if let Value::Object(obj) = val {
                        obj.get("lpmode")
                            .or_else(|| obj.get("low_power_mode"))
                            .map(value_to_string)
                            .unwrap_or_else(|| value_to_string(val))
                    } else {
                        value_to_string(val)
                    };

                    rows.push(vec![key.to_string(), lpmode]);
                }
            }
        }

        Ok(tabulate_simple_auto(headers, &rows))
    }

    fn name(&self) -> &'static str {
        "transceiver_lpmode"
    }
}

/// Formatter for transceiver PM (performance monitoring) output.
pub struct TransceiverPmFormatter;

impl TransceiverPmFormatter {
    /// Creates a new [`TransceiverPmFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverPmFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverPmFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut lines = Vec::new();

        match data {
            Value::Array(arr) => {
                // List format: [{"interface": "...", "description": "..."}, ...]
                for item in arr {
                    if let Value::Object(obj) = item {
                        let iface = obj
                            .get("interface")
                            .or_else(|| obj.get("port"))
                            .or_else(|| obj.get("Port"))
                            .map(value_to_string)
                            .unwrap_or_default();
                        if let Some(desc) = obj.get("description") {
                            lines.push(format!("{}: {}", iface, value_to_string(desc)));
                        } else {
                            // Check if item has PM data fields (current, min, max, etc.)
                            let pm_keys: Vec<&String> = obj
                                .keys()
                                .filter(|k| *k != "interface" && *k != "port" && *k != "Port")
                                .collect();
                            if pm_keys.is_empty() {
                                lines.push(format!(
                                    "{}: Transceiver performance monitoring not applicable",
                                    iface
                                ));
                            } else {
                                lines.push(format!("{}: ", iface));
                                let mut sorted_keys = pm_keys;
                                sorted_keys.sort();
                                for key in sorted_keys {
                                    if let Some(val) = obj.get(key) {
                                        lines.push(format!(
                                            "        {}: {}",
                                            key.to_uppercase(),
                                            value_to_string(val)
                                        ));
                                    }
                                }
                            }
                        }
                        lines.push(String::new());
                    }
                }
            }
            Value::Object(map) => {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort_by(|a, b| natural_cmp(a, b));

                for iface in keys {
                    if let Some(pm_data) = map.get(iface) {
                        match pm_data {
                            Value::String(s) => {
                                if s == "N/A" || s.is_empty() {
                                    lines.push(format!(
                                        "{}: Transceiver performance monitoring not applicable",
                                        iface
                                    ));
                                } else {
                                    // Try to parse as JSON
                                    match serde_json::from_str::<Value>(s) {
                                        Ok(Value::Object(obj)) => {
                                            if let Some(desc) = obj.get("description") {
                                                lines.push(format!(
                                                    "{}: {}",
                                                    iface,
                                                    value_to_string(desc)
                                                ));
                                            } else {
                                                lines.push(format!("{}: Transceiver performance monitoring not applicable", iface));
                                            }
                                        }
                                        _ => {
                                            lines.push(format!("{}: {}", iface, s));
                                        }
                                    }
                                }
                            }
                            Value::Object(obj) => {
                                if let Some(desc) = obj.get("description") {
                                    lines.push(format!("{}: {}", iface, value_to_string(desc)));
                                } else if obj.is_empty() {
                                    lines.push(format!(
                                        "{}: Transceiver performance monitoring not applicable",
                                        iface
                                    ));
                                } else {
                                    // Has PM data - format key-value pairs with uppercase headers
                                    lines.push(format!("{}: ", iface));
                                    let mut sorted_keys: Vec<&String> = obj.keys().collect();
                                    sorted_keys.sort();
                                    for key in sorted_keys {
                                        if let Some(val) = obj.get(key) {
                                            match val {
                                                Value::Object(inner) => {
                                                    lines.push(format!(
                                                        "        {}:",
                                                        key.to_uppercase()
                                                    ));
                                                    let mut inner_keys: Vec<&String> =
                                                        inner.keys().collect();
                                                    inner_keys.sort();
                                                    for ik in inner_keys {
                                                        if let Some(iv) = inner.get(ik) {
                                                            lines.push(format!(
                                                                "                {}: {}",
                                                                ik,
                                                                value_to_string(iv)
                                                            ));
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    lines.push(format!(
                                                        "        {}: {}",
                                                        key.to_uppercase(),
                                                        value_to_string(val)
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Value::Null => {
                                lines.push(format!(
                                    "{}: Transceiver performance monitoring not applicable",
                                    iface
                                ));
                            }
                            _ => {
                                lines.push(format!("{}: {}", iface, value_to_string(pm_data)));
                            }
                        }
                        lines.push(String::new());
                    }
                }
            }
            Value::String(s) => {
                return Ok(s.clone());
            }
            _ => {}
        }

        // Remove trailing empty lines
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "transceiver_pm"
    }
}

/// Formatter for transceiver firmware output.
pub struct TransceiverFirmwareFormatter;

impl TransceiverFirmwareFormatter {
    /// Creates a new [`TransceiverFirmwareFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransceiverFirmwareFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TransceiverFirmwareFormatter {
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
            Cell::new("Port").set_alignment(CellAlignment::Left),
            Cell::new("Active FW").set_alignment(CellAlignment::Left),
            Cell::new("Inactive FW").set_alignment(CellAlignment::Left),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let active = obj
                        .get("active_firmware")
                        .or_else(|| obj.get("Active FW"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let inactive = obj
                        .get("inactive_firmware")
                        .or_else(|| obj.get("Inactive FW"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&port).set_alignment(CellAlignment::Left),
                        Cell::new(&active).set_alignment(CellAlignment::Left),
                        Cell::new(&inactive).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for key in keys {
                if let Some(Value::Object(obj)) = map.get(key) {
                    let active = obj
                        .get("active_firmware")
                        .or_else(|| obj.get("active"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let inactive = obj
                        .get("inactive_firmware")
                        .or_else(|| obj.get("inactive"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(key).set_alignment(CellAlignment::Left),
                        Cell::new(&active).set_alignment(CellAlignment::Left),
                        Cell::new(&inactive).set_alignment(CellAlignment::Left),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "transceiver_firmware"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transceiver_presence_formatter_array() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "presence": "Present"},
            {"port": "Ethernet4", "presence": "Not Present"}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Present"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("Not present")); // normalized case
    }

    #[test]
    fn test_transceiver_presence_formatter_dict() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!({
            "Ethernet0": "Present",
            "Ethernet4": "Not Present"
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Present"));
    }

    #[test]
    fn test_transceiver_info_formatter() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "identifier": "QSFP28",
                "vendor_name": "Finisar",
                "vendor_part": "FTLC1112RDNL-HW",
                "vendor_serial": "XYZ123",
                "hardware_rev": "A",
                "connector": "LC",
                "type": "CR4",
                "length": "5m"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("QSFP28"));
        assert!(result.contains("Finisar"));
    }

    #[test]
    fn test_transceiver_status_formatter() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "temperature": "30.5 C",
                "voltage": "3.3 V",
                "rx_power": "-1.5 dBm",
                "tx_bias": "6.5 mA",
                "tx_power": "-0.5 dBm"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("30.5 C"));
        assert!(result.contains("-1.5 dBm"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": "100GBASE-CR4",
                "Connector Type": "Copper pigtail",
                "Encoding": "8B/10B"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"));
        assert!(result.contains("Application Advertisement"));
        assert!(result.contains("100GBASE-CR4"));
    }

    #[test]
    fn test_transceiver_error_status_formatter() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "error_status": "No Error"},
            {"port": "Ethernet4", "error_status": "Module Fault"}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("No Error"));
        assert!(result.contains("Module Fault"));
    }

    #[test]
    fn test_transceiver_lpmode_formatter() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!({
            "Ethernet0": {"lpmode": "On"},
            "Ethernet4": {"lpmode": "Off"}
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("On"));
    }

    #[test]
    fn test_transceiver_pm_formatter() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {
                "current": {
                    "temperature": "30.5",
                    "voltage": "3.3"
                },
                "min": {
                    "temperature": "28.0"
                },
                "max": {
                    "temperature": "35.0"
                }
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"));
        assert!(result.contains("CURRENT:"));
        assert!(result.contains("temperature: 30.5"));
    }

    #[test]
    fn test_transceiver_firmware_formatter() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "active_firmware": "1.2.3",
                "inactive_firmware": "1.1.0"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("1.2.3"));
        assert!(result.contains("1.1.0"));
    }

    #[test]
    fn test_extract_transceiver_info_row() {
        // Test transceiver info via the formatter instead of the removed helper
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": {
                "identifier": "QSFP28",
                "vendor_name": "Test",
                "vendor_part": "PT123",
                "vendor_serial": "SN123",
                "hardware_rev": "A1",
                "connector": "LC",
                "type": "LR4",
                "length": "10km"
            }
        });

        let config = crate::common::types::CommandConfig::builder()
            .name("interfaces_transceiver_info")
            .build();
        let result = formatter.format(&data, &config, "grid", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("QSFP28"));
        assert!(result.contains("Test"));
    }

    #[test]
    fn test_transceiver_info_formatter_dict() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": {
                "identifier": "QSFP28",
                "vendor_name": "Finisar",
                "vendor_part": "FTLC1112RDNL-HW",
                "vendor_serial": "XYZ123",
                "hardware_rev": "A"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("QSFP28"));
        assert!(result.contains("Finisar"));
    }

    #[test]
    fn test_transceiver_status_formatter_dict() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "temperature": "30.5 C",
                "voltage": "3.3 V",
                "rx_power": "-1.5 dBm",
                "tx_bias": "6.5 mA",
                "tx_power": "-0.5 dBm"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("30.5 C"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!([
            {
                "Port": "Ethernet0",
                "Application Advertisement": "100GBASE-CR4",
                "Connector Type": "Copper pigtail"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"));
        assert!(result.contains("Application Advertisement"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!("No EEPROM data available");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "No EEPROM data available");
    }

    #[test]
    fn test_transceiver_error_status_formatter_dict() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "error_status": "No Error"
            },
            "Ethernet4": "Module Fault"
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("No Error"));
        assert!(result.contains("Module Fault"));
    }

    #[test]
    fn test_transceiver_lpmode_formatter_array() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!([
            {"Port": "Ethernet0", "lpmode": "On"},
            {"Port": "Ethernet4", "low_power_mode": "Off"}
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("On"));
    }

    #[test]
    fn test_transceiver_lpmode_formatter_dict_value() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!({
            "Ethernet0": "On",
            "Ethernet4": "Off"
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("On"));
    }

    #[test]
    fn test_transceiver_firmware_formatter_dict() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!({
            "Ethernet0": {
                "active_firmware": "2.0.0",
                "inactive_firmware": "1.9.0"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("2.0.0"));
        assert!(result.contains("1.9.0"));
    }

    #[test]
    fn test_transceiver_pm_formatter_string() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!("No PM data");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "No PM data");
    }

    #[test]
    fn test_transceiver_presence_nested_object() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!({
            "Ethernet0": {"presence": "Present"},
            "Ethernet4": {"Presence": "Not Present"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Present"));
        assert!(result.contains("Not present")); // normalized case
    }

    #[test]
    fn test_transceiver_presence_array_with_interface() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!([
            {"interface": "Ethernet0", "Presence": "Present"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Present"));
    }

    #[test]
    fn test_transceiver_info_alternate_keys() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!([
            {
                "interface": "Ethernet0",
                "type_id": "QSFP28",
                "manufacturer": "Vendor",
                "model": "Model123",
                "serial": "SN456",
                "hw_rev": "B1",
                "Connector": "LC",
                "cable_type": "SMF",
                "cable_length": "10km"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("QSFP28"));
        assert!(result.contains("Vendor"));
    }

    #[test]
    fn test_transceiver_status_alternate_keys() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!([
            {
                "Port": "Ethernet0",
                "Temperature": "35.0 C",
                "Voltage": "3.3 V",
                "rxpower": "-2.0 dBm",
                "txbias": "7.0 mA",
                "txpower": "-1.0 dBm"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("35.0 C"));
    }

    #[test]
    fn test_transceiver_eeprom_nested_object() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Cable Length": {
                    "SMF (km)": "10",
                    "OM3 (m)": "N/A"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Cable Length:"));
    }

    #[test]
    fn test_transceiver_error_status_status_key() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "status": "OK"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("OK"));
    }

    #[test]
    fn test_transceiver_firmware_empty() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port") || result.contains("Firmware"));
    }

    #[test]
    fn test_transceiver_pm_formatter_array() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "current": {"temp": "30.0"}
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Array handler now extracts port and formats PM data
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("CURRENT:"));
    }

    #[test]
    fn test_transceiver_pm_empty_dict() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_transceiver_presence_empty() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port"));
    }

    #[test]
    fn test_transceiver_info_empty() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port") || result.contains("Identifier"));
    }

    #[test]
    fn test_transceiver_status_empty() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port") || result.contains("Temperature"));
    }

    #[test]
    fn test_transceiver_error_status_empty() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port") || result.contains("Error"));
    }

    // Additional tests for uncovered branches

    #[test]
    fn test_transceiver_presence_direct_string_value() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!({
            "Ethernet0": "Present",
            "Ethernet4": "Not Present"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Present"));
        assert!(result.contains("Not present"));
    }

    #[test]
    fn test_transceiver_info_dict_with_missing_fields() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": {
                "type": "QSFP28"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
    }

    #[test]
    fn test_transceiver_status_dict_with_missing_fields() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "temperature": "45.0 C"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
    }

    #[test]
    fn test_transceiver_eeprom_string_fallback() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!("EEPROM read failed");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "EEPROM read failed");
    }

    #[test]
    fn test_transceiver_lpmode_dict_with_object() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!({
            "Ethernet0": {"lpmode": "On", "low_power_mode": "Enabled"},
            "Ethernet4": {"low_power_mode": "Off"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("On") || result.contains("Enabled"));
    }

    #[test]
    fn test_transceiver_pm_with_categories() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {
                "current": {
                    "temperature": {"instant": "35.0", "avg": "34.5", "min": "30.0", "max": "40.0"}
                },
                "15min": {
                    "temperature": {"instant": "36.0", "avg": "35.0", "min": "31.0", "max": "41.0"}
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("temperature") || result.contains("35.0"));
    }

    #[test]
    fn test_transceiver_firmware_array_with_ports() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "active_firmware": "3.0.0",
                "inactive_firmware": "2.5.0"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0") || result.contains("3.0.0"));
    }

    #[test]
    fn test_transceiver_pm_empty_categories() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {
                "current": {}
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0") || result.is_empty());
    }

    // =====================================================
    // TRANSCEIVER STATUS FORMATTER — DICT WITH FIELD ORDER
    // =====================================================

    #[test]
    fn test_transceiver_status_formatter_dict_full() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "CMIS State (SW)": "READY",
                "Temperature": "25.5C",
                "Voltage": "3.3V",
                "Tx Power": "-1.5 dBm",
                "Rx Power": "-2.0 dBm"
            },
            "Ethernet4": {
                "CMIS State (SW)": "REMOVED"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"));
        assert!(result.contains("Temperature: 25.5C"));
        assert!(result.contains("Transceiver status info not applicable"));
    }

    #[test]
    fn test_transceiver_status_formatter_json_string_value() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet8": "{\"Temperature\": \"30C\", \"Voltage\": \"3.3V\"}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet8:"));
        assert!(result.contains("Temperature: 30C"));
    }

    #[test]
    fn test_transceiver_status_formatter_invalid_json_string() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet12": "not valid json"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet12: not valid json"));
    }

    // =====================================================
    // TRANSCEIVER EEPROM FORMATTER — FULL DICT FORMAT
    // =====================================================

    #[test]
    fn test_transceiver_eeprom_formatter_dict_full() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Connector": "LC",
                "Identifier": "QSFP28",
                "Vendor Name": "CISCO",
                "Vendor SN": "ABC123"
            },
            "Ethernet4": {}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0: SFP EEPROM detected"));
        assert!(result.contains("Connector: LC"));
        assert!(result.contains("Identifier: QSFP28"));
        assert!(result.contains("Ethernet4: SFP EEPROM Not detected"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_dict_json_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet8": "{\"Identifier\": \"SFP+\", \"Vendor Name\": \"Finisar\"}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet8: SFP EEPROM detected"));
        assert!(result.contains("Identifier: SFP+"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_dict_empty_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet12": "",
            "Ethernet16": "{}",
            "Ethernet20": null
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet12: SFP EEPROM Not detected"));
        assert!(result.contains("Ethernet16: SFP EEPROM Not detected"));
        assert!(result.contains("Ethernet20: SFP EEPROM Not detected"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_dict_raw_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet24": "some raw eeprom data"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet24: SFP EEPROM detected"));
        assert!(result.contains("some raw eeprom data"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_with_app_advertisement() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Identifier": "OSFP",
                "Application Advertisement": "400GBASE-DR4\n100GBASE-SR4"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0: SFP EEPROM detected"));
        assert!(result.contains("Application Advertisement:"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_with_app_advertisement_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": ["400GBASE-DR4", "100GBASE-SR4", "800G-DR8"]
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Application Advertisement:"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_array_with_port() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!([
            {
                "port": "Ethernet0",
                "Identifier": "QSFP28",
                "Vendor Name": "CISCO"
            },
            {
                "port": "Ethernet4"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0: SFP EEPROM detected"));
        assert!(result.contains("Ethernet4: SFP EEPROM Not detected"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_empty_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port  Identifier"));
    }

    #[test]
    fn test_transceiver_eeprom_formatter_with_empty_inner_obj() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Field1": {},
                "Field2": []
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0: SFP EEPROM detected"));
        assert!(result.contains("Field1:"));
        assert!(result.contains("Field2:"));
    }

    // =====================================================
    // APP ADVERTISEMENT SORT KEY
    // =====================================================

    #[test]
    fn test_app_adv_sort_key_ordering() {
        // Matches Python's _get_app_adv_sort_key patterns (start-anchored)
        assert_eq!(app_adv_sort_key("800G-ETC-CR8").0, 0);
        assert_eq!(app_adv_sort_key("400GBASE-CR4").0, 1);
        assert_eq!(app_adv_sort_key("400G CR8").0, 2);
        assert_eq!(app_adv_sort_key("200GBASE-CR4").0, 3);
        assert_eq!(app_adv_sort_key("100GBASE-SR4").0, 4);
        assert_eq!(app_adv_sort_key("40GBASE-CR4").0, 5);
        assert_eq!(app_adv_sort_key("Unknown").0, 99);
        // Entries that DON'T match start-anchored patterns should get 99
        assert_eq!(app_adv_sort_key("800G S C2M (placeholder)").0, 99); // no "800G-ETC" prefix
        assert_eq!(app_adv_sort_key("100GAUI-1-L C2M").0, 99); // no "100GBASE" prefix
        assert_eq!(app_adv_sort_key("50GBASE-CR").0, 99); // not in Python's pattern list
        assert_eq!(app_adv_sort_key("25GBASE-CR").0, 99);
        assert_eq!(app_adv_sort_key("10GBASE-SR").0, 99);
        // Ordering: 800G-ETC < 400GBASE < 400G CR < 200GBASE < 100GBASE < 40GBASE < unknown
        assert!(app_adv_sort_key("800G-ETC-CR8") < app_adv_sort_key("400GBASE-CR4"));
        assert!(app_adv_sort_key("400GBASE-CR4") < app_adv_sort_key("400G CR8"));
        assert!(app_adv_sort_key("400G CR8") < app_adv_sort_key("200GBASE-SR4"));
        assert!(app_adv_sort_key("200GBASE-SR4") < app_adv_sort_key("100GBASE-SR4"));
        assert!(app_adv_sort_key("100GBASE-SR4") < app_adv_sort_key("40GBASE-CR4"));
        assert!(app_adv_sort_key("40GBASE-CR4") < app_adv_sort_key("Unknown"));
    }

    // =====================================================
    // TRANSCEIVER ERROR STATUS FORMATTER — FULL COVERAGE
    // =====================================================

    #[test]
    fn test_transceiver_error_status_formatter_full_dict() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "status": "OK",
                "error_code": "0x00",
                "description": "No error"
            },
            "Ethernet4": {
                "status": "ERROR",
                "error_code": "0x01",
                "description": "Module power fault"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0") || result.contains("OK"));
    }

    #[test]
    fn test_transceiver_error_status_formatter_null() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = Value::Null;
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    // ===== Coverage gap tests for compute_status edge cases (L673-678) =====

    #[test]
    fn test_error_status_compute_status_1_with_cmis_not_ready() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "status": "1",
                "cmis_state": "DP_INIT",
                "error": "N/A"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("DP_INIT"), "result: {}", result);
    }

    #[test]
    fn test_error_status_compute_status_1_with_error() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "status": "1",
                "cmis_state": "FAILED",
                "error": "Power budget exceeded"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Power budget exceeded"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_error_status_compute_status_0_with_error() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "status": "0",
                "error": "Module not detected"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Module not detected"), "result: {}", result);
    }

    #[test]
    fn test_error_status_compute_status_0_unplugged() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "status": "0",
                "error": "N/A"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Unplugged"), "result: {}", result);
    }

    // ===== Coverage gap tests for TransceiverErrorStatusFormatter edge cases (L806-814) =====

    #[test]
    fn test_transceiver_error_status_formatter_single_interface_numeric() {
        // Single interface data: status as number (not string), no string values
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "status": 1,
            "error": null
        });
        let config = CommandConfig::new();
        let mut opts = std::collections::HashMap::new();
        opts.insert("argument".to_string(), "Ethernet0".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
    }

    #[test]
    fn test_transceiver_error_status_formatter_single_interface_with_cmis_string_values() {
        // Real-world single interface data with string values (cmis_state, error, status)
        // This is the data returned for "show interfaces transceiver error-status Ethernet0"
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "cmis_state": "READY",
            "error": "N/A",
            "status": "1"
        });
        let config = CommandConfig::new();
        let mut opts = std::collections::HashMap::new();
        opts.insert("argument".to_string(), "Ethernet0".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(
            result.contains("Ethernet0"),
            "Expected 'Ethernet0' in result: {}",
            result
        );
        assert!(result.contains("OK"), "Expected 'OK' in result: {}", result);
        // Must NOT contain raw JSON keys as port names
        assert!(
            !result.contains("cmis_state"),
            "Should NOT contain 'cmis_state' as port: {}",
            result
        );
        assert!(
            !result.contains("N/A"),
            "Should NOT contain raw 'N/A': {}",
            result
        );
    }

    #[test]
    fn test_transceiver_error_status_formatter_single_interface_without_cmis() {
        // Single interface data without cmis_state (e.g., str2-8102-01 device)
        // Data for "show interfaces transceiver error-status Ethernet252"
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "error": "N/A",
            "status": "1"
        });
        let config = CommandConfig::new();
        let mut opts = std::collections::HashMap::new();
        opts.insert("argument".to_string(), "Ethernet252".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(
            result.contains("Ethernet252"),
            "Expected 'Ethernet252' in result: {}",
            result
        );
        assert!(result.contains("OK"), "Expected 'OK' in result: {}", result);
        assert!(
            !result.contains("error"),
            "Should NOT contain 'error' as port name: {}",
            result
        );
    }

    #[test]
    fn test_transceiver_error_status_formatter_single_interface_unplugged() {
        // Single interface with status=0 (Unplugged)
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "cmis_state": "REMOVED",
            "error": "N/A",
            "status": "0"
        });
        let config = CommandConfig::new();
        let mut opts = std::collections::HashMap::new();
        opts.insert("argument".to_string(), "Ethernet88".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(
            result.contains("Ethernet88"),
            "Expected 'Ethernet88' in result: {}",
            result
        );
        assert!(
            result.contains("Unplugged"),
            "Expected 'Unplugged' in result: {}",
            result
        );
    }

    #[test]
    fn test_transceiver_error_status_formatter_dict_non_object_value() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": "some_string_value",
            "Ethernet4": {"status": "1", "error": "N/A", "cmis_state": "READY"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("some_string_value"));
    }

    // ===== Coverage gap tests for TransceiverPmFormatter (L1006-1085) =====

    #[test]
    fn test_transceiver_pm_formatter_dict_string_na() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": "N/A",
            "Ethernet4": ""
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Ethernet0: Transceiver performance monitoring not applicable"),
            "result: {}",
            result
        );
        assert!(result.contains("Ethernet4: Transceiver performance monitoring not applicable"));
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_string_json() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": "{\"description\": \"PM data available\"}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PM data available"), "result: {}", result);
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_string_non_json() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": "raw text value"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("raw text value"), "result: {}", result);
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_object_with_description() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {"description": "Performance data for Ethernet0"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Performance data for Ethernet0"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_object_empty() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("not applicable"), "result: {}", result);
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_object_with_pm_data() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {
                "rx_power": {
                    "current": "-5.0",
                    "high_alarm": "-1.0",
                    "low_alarm": "-10.0"
                },
                "tx_bias": {
                    "current": "40.0"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"), "result: {}", result);
        assert!(result.contains("RX_POWER:"));
        assert!(result.contains("current: -5.0"));
        assert!(result.contains("TX_BIAS:"));
    }

    #[test]
    fn test_transceiver_pm_formatter_dict_null_value() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": null
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("not applicable"), "result: {}", result);
    }

    #[test]
    fn test_transceiver_pm_formatter_string_input() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!("raw string pm data");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "raw string pm data");
    }

    // ---- Round 3 coverage tests ----

    #[test]
    fn test_presence_formatter_array_format() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "presence": "Present"},
            {"port": "Ethernet4", "presence": "Not present"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("Present"));
        assert!(result.contains("Not present"));
    }

    #[test]
    fn test_presence_formatter_dict_format() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!({
            "Ethernet0": {"presence": "Present"},
            "Ethernet4": {"presence": "Not present"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("Present"));
    }

    #[test]
    fn test_presence_formatter_empty_array() {
        let formatter = TransceiverPresenceFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // empty array should produce header-only output or empty
        assert!(
            result.contains("Port") || result.is_empty(),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_info_formatter_dict_format() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": {
                "type": "QSFP28",
                "vendor_name": "Acme",
                "serial": "SN0001"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("EEPROM detected"), "result: {}", result);
        assert!(result.contains("QSFP28"));
    }

    #[test]
    fn test_info_formatter_empty_eeprom() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": {}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Not detected"), "result: {}", result);
    }

    #[test]
    fn test_info_formatter_null_eeprom() {
        let formatter = TransceiverInfoFormatter::new();
        let data = json!({
            "Ethernet0": null
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Not detected"), "result: {}", result);
    }

    #[test]
    fn test_eeprom_formatter_app_advertisement_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": ["400GBASE-DR4", "100GBASE-CR4"],
                "Identifier": "QSFP-DD"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Application Advertisement"),
            "result: {}",
            result
        );
        assert!(result.contains("400GBASE-DR4"));
        assert!(result.contains("100GBASE-CR4"));
    }

    #[test]
    fn test_eeprom_formatter_app_advertisement_string_multiline() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": "400GBASE-DR4\n100GBASE-CR4\n50GBASE-SR",
                "Identifier": "QSFP-DD"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Application Advertisement"),
            "result: {}",
            result
        );
        assert!(result.contains("400GBASE-DR4"));
    }

    #[test]
    fn test_eeprom_formatter_empty_object_value() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "EmptyField": {},
                "EmptyArray": []
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("EEPROM detected"), "result: {}", result);
        assert!(result.contains("EmptyField:"), "result: {}", result);
        assert!(result.contains("EmptyArray:"), "result: {}", result);
    }

    #[test]
    fn test_eeprom_formatter_json_string_value() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": "{\"Identifier\": \"QSFP-DD\", \"Vendor\": \"Acme\"}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("EEPROM detected"), "result: {}", result);
        assert!(result.contains("Identifier"));
    }

    #[test]
    fn test_eeprom_formatter_empty_json_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": "{}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Not detected"), "result: {}", result);
    }

    #[test]
    fn test_eeprom_formatter_non_json_string() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": "not-valid-json"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("EEPROM detected"), "result: {}", result);
        assert!(result.contains("not-valid-json"), "result: {}", result);
    }

    #[test]
    fn test_eeprom_formatter_array_format() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "Identifier": "QSFP28", "Vendor": "Acme"},
            {"port": "Ethernet4"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("EEPROM detected"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("Not detected"));
    }

    #[test]
    fn test_eeprom_formatter_empty_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Port") || result.contains("Identifier"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_eeprom_formatter_string_fallback() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!("raw eeprom data");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "raw eeprom data");
    }

    #[test]
    fn test_eeprom_formatter_app_advertisement_empty() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": "",
                "Identifier": "QSFP-DD"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Application Advertisement:"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_eeprom_formatter_app_advertisement_empty_array() {
        let formatter = TransceiverEepromFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Application Advertisement": [],
                "Identifier": "QSFP-DD"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Application Advertisement:"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_status_formatter_removed_cmis_state() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": {
                "CMIS State (SW)": "REMOVED"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("not applicable"), "result: {}", result);
    }

    #[test]
    fn test_status_formatter_json_string_value() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": "{\"Temperature\": \"25.0C\", \"Voltage\": \"3.3V\"}"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"), "result: {}", result);
        assert!(result.contains("Temperature"));
    }

    #[test]
    fn test_status_formatter_raw_string_value() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!({
            "Ethernet0": "not-json-data"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("Ethernet0: not-json-data"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_status_formatter_string_input() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!("raw status string");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "raw status string");
    }

    #[test]
    fn test_lpmode_formatter_array_format() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "lpmode": "On"},
            {"Port": "Ethernet4", "Low-power Mode": "Off"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("On"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("Off"));
    }

    #[test]
    fn test_lpmode_formatter_dict_format() {
        let formatter = TransceiverLpmodeFormatter::new();
        let data = json!({
            "Ethernet0": {"lpmode": "On"},
            "Ethernet4": "Off"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("On"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("Off"));
    }

    #[test]
    fn test_firmware_formatter_array_format() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "active_firmware": "1.0", "inactive_firmware": "0.9"},
            {"Port": "Ethernet4", "Active FW": "2.0", "Inactive FW": "1.5"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("1.0"));
        assert!(result.contains("0.9"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("2.0"));
    }

    #[test]
    fn test_firmware_formatter_dict_format() {
        let formatter = TransceiverFirmwareFormatter::new();
        let data = json!({
            "Ethernet0": {"active_firmware": "1.0", "inactive_firmware": "0.9"},
            "Ethernet4": {"active": "2.0", "inactive": "1.5"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("1.0"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("2.0"));
    }

    #[test]
    fn test_error_status_formatter_array_empty() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port"), "result: {}", result);
        assert!(result.contains("Error Status"));
    }

    #[test]
    fn test_error_status_formatter_array_with_items() {
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!([
            {"Port": "Ethernet0", "Error Status": "OK"},
            {"port": "Ethernet4", "status": "Error"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("OK"));
        assert!(result.contains("Ethernet4"));
    }

    #[test]
    fn test_error_status_formatter_dict_non_object_string_value() {
        // Dict with string values (not objects) → value_to_string branch at L855
        let formatter = TransceiverErrorStatusFormatter::new();
        let data = json!({
            "Ethernet0": "OK",
            "Ethernet4": "Error"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("OK"));
    }

    #[test]
    fn test_pm_formatter_array_with_pm_keys() {
        // Array item without "description" key but with other PM fields
        let formatter = TransceiverPmFormatter::new();
        let data = json!([
            {"interface": "Ethernet0", "rx_power": "-5.0", "tx_bias": "30.0"},
            {"interface": "Ethernet4"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"), "result: {}", result);
        assert!(result.contains("RX_POWER"));
        assert!(result.contains("not applicable"), "result: {}", result);
    }

    #[test]
    fn test_pm_formatter_dict_object_with_inner_objects() {
        // PM data with nested objects (e.g., per-lane data)
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {
                "rx_power": {
                    "lane0": "-5.0",
                    "lane1": "-4.8"
                },
                "tx_bias": {
                    "lane0": "30.0"
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"), "result: {}", result);
        assert!(result.contains("RX_POWER:"), "result: {}", result);
        assert!(result.contains("lane0: -5.0"), "result: {}", result);
        assert!(result.contains("TX_BIAS:"), "result: {}", result);
    }

    #[test]
    fn test_pm_formatter_dict_empty_object() {
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": {}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("not applicable"), "result: {}", result);
    }

    #[test]
    fn test_pm_formatter_dict_non_string_non_object() {
        // PM data with non-string, non-object value (e.g., number)
        let formatter = TransceiverPmFormatter::new();
        let data = json!({
            "Ethernet0": 42
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("42"), "result: {}", result);
    }

    #[test]
    fn test_status_formatter_array_format() {
        let formatter = TransceiverStatusFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "Temperature": "30.0", "Voltage": "3.3"},
            {"Port": "Ethernet4", "Temperature": "28.5"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0:"), "result: {}", result);
        assert!(result.contains("Temperature"), "result: {}", result);
        assert!(result.contains("Ethernet4:"), "result: {}", result);
    }
}
