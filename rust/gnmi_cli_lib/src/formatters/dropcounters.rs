//! Drop counters formatters for capabilities, configuration, and counters.

use crate::common::types::CommandConfig;
use crate::formatters::base::{
    tabulate_simple, value_to_string, FormatError, OutputFormatter,
};
use crate::utils::sorting::natural_cmp;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::Value;
use std::collections::HashMap;

/// Formatter for drop counter capabilities output.
pub struct DropCounterCapabilitiesFormatter;

impl DropCounterCapabilitiesFormatter {
    /// Creates a new [`DropCounterCapabilitiesFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for DropCounterCapabilitiesFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for DropCounterCapabilitiesFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Collect counter data: (counter_type, count, reasons_list)
        let mut entries: Vec<(String, String, Vec<String>)> = Vec::new();

        match data {
            Value::Array(arr) => {
                for item in arr {
                    if let Value::Object(obj) = item {
                        let counter_type = obj
                            .get("counter_type")
                            .or_else(|| obj.get("Counter Type"))
                            .or_else(|| obj.get("type"))
                            .or_else(|| obj.get("Type"))
                            .map(value_to_string)
                            .unwrap_or_default();

                        let count = obj
                            .get("count")
                            .or_else(|| obj.get("total"))
                            .or_else(|| obj.get("Total"))
                            .map(value_to_string)
                            .unwrap_or_default();

                        let reasons = obj
                            .get("reasons")
                            .or_else(|| obj.get("drop_reasons"))
                            .or_else(|| obj.get("Drop Reasons"))
                            .map(parse_reasons_string)
                            .unwrap_or_default();

                        entries.push((counter_type, count, reasons));
                    }
                }
            }
            Value::Object(map) => {
                for (counter_type, type_data) in map {
                    if let Value::Object(obj) = type_data {
                        let count = obj
                            .get("count")
                            .or_else(|| obj.get("total"))
                            .or_else(|| obj.get("Total"))
                            .map(value_to_string)
                            .unwrap_or_default();

                        let reasons = obj
                            .get("reasons")
                            .or_else(|| obj.get("drop_reasons"))
                            .or_else(|| obj.get("Drop Reasons"))
                            .map(parse_reasons_string)
                            .unwrap_or_default();

                        entries.push((counter_type.clone(), count, reasons));
                    }
                }
            }
            _ => {}
        }

        // Sort entries by counter_type for consistent output
        entries.sort_by(|a, b| natural_cmp(&a.0, &b.0));

        // Calculate column widths
        let counter_type_header = "Counter Type";
        let total_header = "Total";

        let max_type_len = entries
            .iter()
            .map(|(t, _, _)| t.len())
            .max()
            .unwrap_or(0)
            .max(counter_type_header.len());

        // Add 2 padding for right-aligned column (matching Python tabulate numalign="right" behavior)
        let max_count_len = entries
            .iter()
            .map(|(_, c, _)| c.len())
            .max()
            .unwrap_or(0)
            .max(total_header.len())
            + 2;

        let mut output = String::new();

        // Header line
        output.push_str(&format!(
            "{:<width1$}  {:>width2$}\n",
            counter_type_header,
            total_header,
            width1 = max_type_len,
            width2 = max_count_len
        ));

        // Separator line
        output.push_str(&format!(
            "{:-<width1$}  {:-<width2$}\n",
            "",
            "",
            width1 = max_type_len,
            width2 = max_count_len
        ));

        // Data rows
        for (counter_type, count, _) in &entries {
            output.push_str(&format!(
                "{:<width1$}  {:>width2$}\n",
                counter_type,
                count,
                width1 = max_type_len,
                width2 = max_count_len
            ));
        }

        // Reasons section for each counter type (only if reasons are non-empty)
        for (counter_type, _, reasons) in &entries {
            if !reasons.is_empty() {
                output.push('\n');
                output.push_str(counter_type);
                output.push('\n');
                for reason in reasons {
                    output.push_str(&format!("        {}\n", reason));
                }
            }
        }

        // Remove trailing newline to match expected format
        if output.ends_with('\n') {
            output.pop();
        }

        Ok(output)
    }

    fn name(&self) -> &'static str {
        "dropcounter_capabilities"
    }
}

/// Parse reasons from either a string like "[MPLS_MISS,FDB_DISCARDS,...]" or a JSON array.
fn parse_reasons_string(value: &Value) -> Vec<String> {
    match value {
        Value::Array(arr) => arr.iter().map(value_to_string).collect(),
        Value::String(s) => {
            // Handle string like "[MPLS_MISS,FDB_AND_BLACKHOLE_DISCARDS,...]"
            let trimmed = s.trim();
            let inner = if trimmed.starts_with('[') && trimmed.ends_with(']') {
                &trimmed[1..trimmed.len() - 1]
            } else {
                trimmed
            };
            inner
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => vec![],
    }
}

fn format_reasons(value: &Value) -> String {
    match value {
        Value::Array(arr) => arr
            .iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(", "),
        Value::String(s) => s.clone(),
        _ => value_to_string(value),
    }
}

/// Formatter for drop counter configuration output.
pub struct DropCounterConfigFormatter;

impl DropCounterConfigFormatter {
    /// Creates a new [`DropCounterConfigFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for DropCounterConfigFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for DropCounterConfigFormatter {
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
            Cell::new("Counter").set_alignment(CellAlignment::Left),
            Cell::new("Alias").set_alignment(CellAlignment::Left),
            Cell::new("Group").set_alignment(CellAlignment::Left),
            Cell::new("Type").set_alignment(CellAlignment::Left),
            Cell::new("Reasons").set_alignment(CellAlignment::Left),
            Cell::new("Description").set_alignment(CellAlignment::Left),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    add_config_row(&mut table, obj);
                }
            }
        } else if let Value::Object(map) = data {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            for key in keys {
                if let Some(Value::Object(obj)) = map.get(key) {
                    add_config_row(&mut table, obj);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "dropcounter_config"
    }
}

fn add_config_row(table: &mut Table, obj: &serde_json::Map<String, Value>) {
    let counter = obj
        .get("counter")
        .or_else(|| obj.get("Counter"))
        .or_else(|| obj.get("name"))
        .map(value_to_string)
        .unwrap_or_default();
    let alias = obj
        .get("alias")
        .or_else(|| obj.get("Alias"))
        .map(value_to_string)
        .unwrap_or_default();
    let group = obj
        .get("group")
        .or_else(|| obj.get("Group"))
        .map(value_to_string)
        .unwrap_or_default();
    let counter_type = obj
        .get("type")
        .or_else(|| obj.get("Type"))
        .map(value_to_string)
        .unwrap_or_default();
    let reasons = obj
        .get("reasons")
        .or_else(|| obj.get("Reasons"))
        .map(format_reasons)
        .unwrap_or_default();
    let description = obj
        .get("description")
        .or_else(|| obj.get("Description"))
        .map(value_to_string)
        .unwrap_or_default();

    table.add_row(vec![
        Cell::new(&counter).set_alignment(CellAlignment::Left),
        Cell::new(&alias).set_alignment(CellAlignment::Left),
        Cell::new(&group).set_alignment(CellAlignment::Left),
        Cell::new(&counter_type).set_alignment(CellAlignment::Left),
        Cell::new(&reasons).set_alignment(CellAlignment::Left),
        Cell::new(&description).set_alignment(CellAlignment::Left),
    ]);
}

/// Formatter for drop counter counts output.
pub struct DropCounterCountsFormatter;

impl DropCounterCountsFormatter {
    /// Creates a new [`DropCounterCountsFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for DropCounterCountsFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for DropCounterCountsFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Standard port columns in preferred order
        const STANDARD_COLUMNS: &[&str] = &["State", "RX_ERR", "RX_DROPS", "TX_ERR", "TX_DROPS"];

        // Collect all unique value-level keys from the data
        let mut all_columns = std::collections::BTreeSet::new();
        if let Value::Object(map) = data {
            for (_iface, port_data) in map {
                if let Value::Object(obj) = port_data {
                    for key in obj.keys() {
                        all_columns.insert(key.clone());
                    }
                }
            }
        } else if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    for key in obj.keys() {
                        if key != "iface" && key != "interface" {
                            all_columns.insert(key.clone());
                        }
                    }
                }
            }
        }

        // Order columns: standard first (in order), then remaining sorted
        let mut ordered_columns: Vec<String> = Vec::new();
        for &col in STANDARD_COLUMNS {
            if all_columns.contains(col) {
                ordered_columns.push(col.to_string());
            }
        }
        let mut debug_columns: Vec<String> = all_columns
            .iter()
            .filter(|c| !STANDARD_COLUMNS.contains(&c.as_str()))
            .cloned()
            .collect();
        debug_columns.sort();
        ordered_columns.extend(debug_columns);

        // Build header: "State" -> "STATE", others as-is
        let mut headers: Vec<String> = vec!["IFACE".to_string()];
        for col in &ordered_columns {
            if col == "State" || col == "state" {
                headers.push("STATE".to_string());
            } else {
                headers.push(col.clone());
            }
        }

        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Value::Object(map) = data {
            if map.is_empty() {
                return Ok(String::new());
            }

            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| natural_cmp(a, b));

            for iface in keys {
                if let Some(Value::Object(obj)) = map.get(iface) {
                    let mut row = vec![iface.to_string()];
                    for col in &ordered_columns {
                        let val = obj
                            .get(col)
                            .or_else(|| obj.get(&col.to_lowercase()))
                            .map(value_to_string)
                            .unwrap_or_default();
                        row.push(val);
                    }
                    rows.push(row);
                }
            }
        } else if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let iface = obj
                        .get("iface")
                        .or_else(|| obj.get("interface"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let mut row = vec![iface];
                    for col in &ordered_columns {
                        let val = obj
                            .get(col)
                            .or_else(|| obj.get(&col.to_lowercase()))
                            .map(value_to_string)
                            .unwrap_or_default();
                        row.push(val);
                    }
                    rows.push(row);
                }
            }
        }

        let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
        Ok(tabulate_simple(&header_refs, &rows))
    }

    fn name(&self) -> &'static str {
        "dropcounter_counts"
    }
}

/// Formatter for detailed drop reasons output.
pub struct DropReasonsFormatter;

impl DropReasonsFormatter {
    /// Creates a new [`DropReasonsFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for DropReasonsFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for DropReasonsFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut lines = Vec::new();

        if let Value::Object(map) = data {
            // Group by counter type
            let mut types: Vec<&String> = map.keys().collect();
            types.sort();

            for counter_type in types {
                if let Some(Value::Object(reasons)) = map.get(counter_type) {
                    lines.push(format!("{}:", counter_type));
                    lines.push("-".repeat(40));

                    let mut reason_keys: Vec<&String> = reasons.keys().collect();
                    reason_keys.sort();

                    for reason in reason_keys {
                        if let Some(desc) = reasons.get(reason) {
                            lines.push(format!("  {}: {}", reason, value_to_string(desc)));
                        }
                    }

                    lines.push(String::new());
                }
            }
        } else if let Value::Array(arr) = data {
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Disabled);
            table.load_preset("                   ");

            table.set_header(vec![
                Cell::new("Reason").set_alignment(CellAlignment::Left),
                Cell::new("Type").set_alignment(CellAlignment::Left),
                Cell::new("Description").set_alignment(CellAlignment::Left),
            ]);

            for item in arr {
                if let Value::Object(obj) = item {
                    let reason = obj
                        .get("reason")
                        .or_else(|| obj.get("Reason"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let reason_type = obj
                        .get("type")
                        .or_else(|| obj.get("Type"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let description = obj
                        .get("description")
                        .or_else(|| obj.get("Description"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&reason).set_alignment(CellAlignment::Left),
                        Cell::new(&reason_type).set_alignment(CellAlignment::Left),
                        Cell::new(&description).set_alignment(CellAlignment::Left),
                    ]);
                }
            }

            return Ok(table.to_string());
        }

        if lines.is_empty() {
            if let Value::String(s) = data {
                return Ok(s.clone());
            }
        }

        Ok(lines.join("\n"))
    }

    fn name(&self) -> &'static str {
        "drop_reasons"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dropcounter_capabilities_formatter() {
        let formatter = DropCounterCapabilitiesFormatter::new();
        let data = json!([
            {
                "counter_type": "PORT_INGRESS_DROPS",
                "total": "2",
                "available": "2",
                "drop_reasons": ["SMAC_EQUALS_DMAC", "INGRESS_VLAN_FILTER"]
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PORT_INGRESS_DROPS"));
        assert!(result.contains("SMAC_EQUALS_DMAC"));
    }

    #[test]
    fn test_dropcounter_capabilities_dict() {
        let formatter = DropCounterCapabilitiesFormatter::new();
        let data = json!({
            "PORT_INGRESS_DROPS": {
                "total": "2",
                "available": "2",
                "drop_reasons": ["SMAC_EQUALS_DMAC"]
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PORT_INGRESS_DROPS"));
    }

    #[test]
    fn test_dropcounter_config_formatter() {
        let formatter = DropCounterConfigFormatter::new();
        let data = json!([
            {
                "counter": "DEBUG_1",
                "alias": "smac_dmac",
                "group": "PORT_INGRESS_DROPS",
                "type": "PORT_INGRESS_DROPS",
                "reasons": ["SMAC_EQUALS_DMAC"],
                "description": "SMAC equals DMAC drops"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("DEBUG_1"));
        assert!(result.contains("smac_dmac"));
        assert!(result.contains("SMAC_EQUALS_DMAC"));
    }

    #[test]
    fn test_dropcounter_counts_formatter() {
        let formatter = DropCounterCountsFormatter::new();
        let data = json!({
            "Ethernet0": {
                "State": "U",
                "RX_ERR": "1000",
                "TX_ERR": "50",
                "RX_DROPS": "10",
                "TX_DROPS": "5"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("1000"));
        assert!(result.contains("50"));
    }

    #[test]
    fn test_dropcounter_counts_dict() {
        let formatter = DropCounterCountsFormatter::new();
        let data = json!({
            "Ethernet4": {
                "State": "D",
                "RX_ERR": "500",
                "TX_ERR": "10",
                "RX_DROPS": "200",
                "TX_DROPS": "100"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("500"));
    }

    #[test]
    fn test_drop_reasons_formatter() {
        let formatter = DropReasonsFormatter::new();
        let data = json!({
            "PORT_INGRESS_DROPS": {
                "SMAC_EQUALS_DMAC": "Source MAC equals destination MAC",
                "INGRESS_VLAN_FILTER": "Ingress VLAN filtering"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PORT_INGRESS_DROPS:"));
        assert!(result.contains("SMAC_EQUALS_DMAC:"));
    }

    #[test]
    fn test_drop_reasons_formatter_array() {
        let formatter = DropReasonsFormatter::new();
        let data = json!([
            {
                "reason": "SMAC_EQUALS_DMAC",
                "type": "PORT_INGRESS_DROPS",
                "description": "Source MAC equals destination MAC"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("SMAC_EQUALS_DMAC"));
        assert!(result.contains("PORT_INGRESS_DROPS"));
    }

    #[test]
    fn test_format_reasons_array() {
        let val = json!(["REASON1", "REASON2", "REASON3"]);
        let result = format_reasons(&val);
        assert_eq!(result, "REASON1, REASON2, REASON3");
    }

    #[test]
    fn test_format_reasons_string() {
        let val = json!("REASON1");
        let result = format_reasons(&val);
        assert_eq!(result, "REASON1");
    }

    #[test]
    fn test_dropcounter_counts_dict_simple_value() {
        let formatter = DropCounterCountsFormatter::new();
        // Dict with IFACE -> stats object
        let data = json!({
            "Ethernet8": {
                "State": "U",
                "RX_ERR": "1000",
                "TX_ERR": "500",
                "RX_DROPS": "0",
                "TX_DROPS": "0"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet8"));
        assert!(result.contains("1000"));
    }

    #[test]
    fn test_dropcounter_config_dict() {
        let formatter = DropCounterConfigFormatter::new();
        let data = json!({
            "DEBUG_1": {
                "counter": "DEBUG_1",
                "alias": "alias1",
                "group": "PORT_DROPS",
                "type": "INGRESS",
                "reasons": ["VLAN_DROP"],
                "description": "Debug counter 1"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("DEBUG_1"));
        assert!(result.contains("alias1"));
    }

    #[test]
    fn test_drop_reasons_formatter_string() {
        let formatter = DropReasonsFormatter::new();
        let data = json!("No drop reasons available");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "No drop reasons available");
    }

    #[test]
    fn test_format_reasons_number() {
        let val = json!(12345);
        let result = format_reasons(&val);
        assert_eq!(result, "12345");
    }

    #[test]
    fn test_dropcounter_counts_array_format() {
        // Test the array fallback path in DropCounterCountsFormatter
        let formatter = DropCounterCountsFormatter::new();
        let data = json!([
            {
                "iface": "Ethernet0",
                "State": "U",
                "RX_ERR": "100",
                "TX_ERR": "50",
                "RX_DROPS": "10",
                "TX_DROPS": "5"
            },
            {
                "interface": "Ethernet4",
                "State": "D",
                "RX_ERR": "200",
                "TX_ERR": "25",
                "RX_DROPS": "20",
                "TX_DROPS": "10"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("100"));
        assert!(result.contains("200"));
    }

    #[test]
    fn test_dropcounter_counts_lowercase_fields() {
        // Test lowercase field variants
        let formatter = DropCounterCountsFormatter::new();
        let data = json!({
            "Ethernet12": {
                "state": "U",
                "rx_err": "300",
                "tx_err": "150",
                "rx_drops": "30",
                "tx_drops": "15"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet12"));
        assert!(result.contains("300"));
        assert!(result.contains("150"));
    }

    #[test]
    fn test_dropcounter_counts_empty_object() {
        // Test with empty object
        let formatter = DropCounterCountsFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return empty output for empty data (matching Python behavior)
        assert!(result.is_empty());
    }

    #[test]
    fn test_dropcounter_counts_null_input() {
        // Test with null input (neither object nor array)
        let formatter = DropCounterCountsFormatter::new();
        let data = json!(null);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return table with just headers
        assert!(result.contains("IFACE"));
    }

    #[test]
    fn test_drop_reasons_empty_object() {
        // Test with empty object
        let formatter = DropReasonsFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_drop_reasons_empty_array() {
        // Test with empty array
        let formatter = DropReasonsFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return table with just headers
        assert!(result.contains("Reason"));
    }

    #[test]
    fn test_dropcounter_config_empty_array() {
        let formatter = DropCounterConfigFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Counter"));
    }

    #[test]
    fn test_dropcounter_config_with_alternate_keys() {
        // Test with alternate key names (uppercase variants)
        let formatter = DropCounterConfigFormatter::new();
        let data = json!([
            {
                "Counter": "DEBUG_2",
                "Alias": "test_alias",
                "Group": "PORT_EGRESS_DROPS",
                "Type": "EGRESS",
                "Reasons": ["TX_DROP"],
                "Description": "Debug counter 2"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("DEBUG_2"));
        assert!(result.contains("test_alias"));
    }

    #[test]
    fn test_dropcounter_capabilities_empty_array() {
        let formatter = DropCounterCapabilitiesFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Counter Type"));
    }

    #[test]
    fn test_dropcounter_capabilities_with_alternate_keys() {
        // Test with alternate key names
        let formatter = DropCounterCapabilitiesFormatter::new();
        let data = json!([
            {
                "Counter Type": "SWITCH_DROPS",
                "Total": "5",
                "available": "3",
                "Drop Reasons": ["REASON_A", "REASON_B"]
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("SWITCH_DROPS"));
        assert!(result.contains("5"));
    }

    #[test]
    fn test_format_reasons_nested_object() {
        // Test format_reasons with nested object fallback
        let val = json!({"nested": "object"});
        let result = format_reasons(&val);
        assert!(result.contains("nested"));
    }

    // ---- Round 3 coverage tests ----

    #[test]
    fn test_parse_reasons_string_bracket_format() {
        // Test parse_reasons_string with "[REASON1,REASON2,...]" string format
        let val = json!("[MPLS_MISS,FDB_DISCARDS,L3_DROPS]");
        let result = parse_reasons_string(&val);
        assert_eq!(result, vec!["MPLS_MISS", "FDB_DISCARDS", "L3_DROPS"]);
    }

    #[test]
    fn test_parse_reasons_string_no_brackets() {
        let val = json!("MPLS_MISS,FDB_DISCARDS");
        let result = parse_reasons_string(&val);
        assert_eq!(result, vec!["MPLS_MISS", "FDB_DISCARDS"]);
    }

    #[test]
    fn test_parse_reasons_string_array_format() {
        let val = json!(["REASON_A", "REASON_B"]);
        let result = parse_reasons_string(&val);
        assert_eq!(result, vec!["REASON_A", "REASON_B"]);
    }

    #[test]
    fn test_parse_reasons_string_other_type() {
        let val = json!(42);
        let result = parse_reasons_string(&val);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_reasons_string_empty_brackets() {
        let val = json!("[]");
        let result = parse_reasons_string(&val);
        assert!(result.is_empty());
    }

    #[test]
    fn test_capabilities_formatter_with_reasons_string() {
        // Test DropCounterCapabilitiesFormatter where Drop Reasons is a bracket string
        let formatter = DropCounterCapabilitiesFormatter::new();
        let data = json!([
            {
                "Counter Type": "PORT_INGRESS_DROPS",
                "count": "2",
                "Drop Reasons": "[MPLS_MISS,FDB_DISCARDS]"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PORT_INGRESS_DROPS"), "result: {}", result);
        assert!(result.contains("MPLS_MISS"), "result: {}", result);
        assert!(result.contains("FDB_DISCARDS"), "result: {}", result);
    }

    #[test]
    fn test_config_formatter_basic() {
        let formatter = DropCounterConfigFormatter::new();
        let data = json!([
            {
                "Counter": "DEBUG_0",
                "Alias": "TestAlias",
                "Group": "PORT_INGRESS_DROPS",
                "Type": "PORT_INGRESS_DROPS",
                "Reasons": "MPLS_MISS",
                "Description": "Test drop"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("DEBUG_0"), "result: {}", result);
        assert!(result.contains("TestAlias"), "result: {}", result);
    }

    #[test]
    fn test_counts_formatter_basic() {
        let formatter = DropCounterCountsFormatter::new();
        let data = json!({
            "Ethernet0": {
                "State": "OK",
                "RX_ERR": "0",
                "RX_DROPS": "5",
                "TX_ERR": "0",
                "TX_DROPS": "0"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("OK"), "result: {}", result);
    }

    #[test]
    fn test_reasons_formatter_dict_format() {
        let formatter = DropReasonsFormatter::new();
        let data = json!({
            "PORT_INGRESS_DROPS": {
                "MPLS_MISS": "Packets missed MPLS lookup",
                "FDB_DISCARDS": "FDB table discards"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("PORT_INGRESS_DROPS:"), "result: {}", result);
        assert!(result.contains("MPLS_MISS"), "result: {}", result);
    }

    #[test]
    fn test_reasons_formatter_array_format() {
        let formatter = DropReasonsFormatter::new();
        let data = json!([
            {"reason": "MPLS_MISS", "type": "PORT", "description": "MPLS miss"},
            {"Reason": "FDB_DISCARDS", "Type": "PORT", "Description": "FDB disc"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("MPLS_MISS"), "result: {}", result);
        assert!(result.contains("FDB_DISCARDS"), "result: {}", result);
    }

    #[test]
    fn test_reasons_formatter_string_fallback() {
        let formatter = DropReasonsFormatter::new();
        let data = json!("raw reason text");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "raw reason text");
    }
}
