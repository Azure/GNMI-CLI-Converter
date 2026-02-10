//! Watermark formatters for buffer pool, headroom pool, and queue watermarks.

use crate::common::types::CommandConfig;
use crate::formatters::base::{
    tabulate_simple, tabulate_simple_auto, value_to_string, FormatError, OutputFormatter,
};
use crate::utils::sorting::natural_sort_key;
use serde_json::Value;
use std::collections::HashMap;

/// Formatter for watermark telemetry interval.
pub struct WatermarkTelemetryFormatter;

impl WatermarkTelemetryFormatter {
    /// Creates a new [`WatermarkTelemetryFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for WatermarkTelemetryFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for WatermarkTelemetryFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if let Value::Object(map) = data {
            if let Some(interval) = map.get("interval").or_else(|| map.get("Interval")) {
                let mut val = value_to_string(interval);
                // Strip trailing "s" suffix (JSON has "120s", Python strips it)
                if val.ends_with('s') {
                    val.pop();
                }
                return Ok(format!("Telemetry interval {} second(s)", val));
            }
        }

        // Direct number value
        if let Value::Number(n) = data {
            return Ok(format!("Telemetry interval {} second(s)", n));
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "watermark_telemetry"
    }
}

/// Formatter for queue watermarks (unicast, multicast, all).
pub struct QueueWatermarkFormatter;

impl QueueWatermarkFormatter {
    /// Creates a new [`QueueWatermarkFormatter`].
    pub fn new() -> Self {
        Self
    }

    fn build_watermark_table(
        &self,
        data: &Value,
        config: &CommandConfig,
    ) -> Result<String, FormatError> {
        let mut output = Vec::new();

        // Add title if present
        let title = config.title().unwrap_or_default().to_string();
        if !title.is_empty() {
            output.push(title);
        }

        if let Value::Object(map) = data {
            if map.is_empty() {
                return Ok("Object map is empty!".to_string());
            }

            // Collect all queue columns
            let mut all_columns = std::collections::BTreeSet::new();
            for (_, value) in map {
                if let Value::Object(val_map) = value {
                    for key in val_map.keys() {
                        all_columns.insert(key.clone());
                    }
                }
            }

            // Sort columns naturally
            let mut columns: Vec<String> = all_columns.into_iter().collect();
            columns.sort_by_key(|a| natural_sort_key(a));

            // Build headers: Port + dynamic columns
            let mut headers_owned = vec!["Port".to_string()];
            headers_owned.extend(columns.clone());
            let headers: Vec<&str> = headers_owned.iter().map(|s| s.as_str()).collect();

            // Sort ports naturally
            let mut ports: Vec<&String> = map.keys().collect();
            ports.sort_by_key(|a| natural_sort_key(a));

            // Rows
            let mut rows: Vec<Vec<String>> = Vec::new();
            for port in ports {
                if let Some(Value::Object(val_map)) = map.get(port) {
                    let mut row = vec![port.to_string()];
                    for col in &columns {
                        let val = val_map
                            .get(col)
                            .map(value_to_string)
                            .unwrap_or_else(|| "N/A".to_string());
                        row.push(val);
                    }
                    rows.push(row);
                }
            }

            output.push(tabulate_simple_auto(&headers, &rows));
        }

        Ok(output.join("\n"))
    }
}

impl Default for QueueWatermarkFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for QueueWatermarkFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        self.build_watermark_table(data, config)
    }

    fn name(&self) -> &'static str {
        "queue_watermark"
    }
}

/// Formatter for headroom pool watermarks.
pub struct HeadroomPoolFormatter;

impl HeadroomPoolFormatter {
    /// Creates a new [`HeadroomPoolFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for HeadroomPoolFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for HeadroomPoolFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut output = Vec::new();

        // Add title from config (no fallback — title must be set in command registration)
        let title = config.title().unwrap_or_default().to_string();
        if !title.is_empty() {
            output.push(title);
        }

        if let Value::Object(map) = data {
            let headers: &[&str] = &["Pool", "Bytes"];

            if map.is_empty() {
                output.push(tabulate_simple_auto(headers, &[]));
                return Ok(output.join("\n"));
            }

            // Sort pools naturally
            let mut pools: Vec<&String> = map.keys().collect();
            pools.sort_by_key(|a| natural_sort_key(a));

            let mut rows: Vec<Vec<String>> = Vec::new();
            for pool in pools {
                if let Some(value) = map.get(pool) {
                    let bytes = if let Value::Object(val_map) = value {
                        val_map
                            .get("Bytes")
                            .or_else(|| val_map.get("bytes"))
                            .map(value_to_string)
                            .unwrap_or_else(|| "N/A".to_string())
                    } else {
                        value_to_string(value)
                    };
                    rows.push(vec![pool.to_string(), bytes]);
                }
            }

            output.push(tabulate_simple_auto(headers, &rows));
        }

        Ok(output.join("\n"))
    }

    fn name(&self) -> &'static str {
        "headroom_pool"
    }
}

/// Formatter for buffer pool watermarks (uses all-right alignment to match Python TableFormatter).
pub struct BufferPoolFormatter;

impl BufferPoolFormatter {
    /// Creates a new [`BufferPoolFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for BufferPoolFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for BufferPoolFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut output = Vec::new();

        let title = config.title().unwrap_or_default().to_string();
        if !title.is_empty() {
            output.push(title);
        }

        if let Value::Object(map) = data {
            let headers: &[&str] = &["Pool", "Bytes"];

            if map.is_empty() {
                output.push(tabulate_simple(headers, &[]));
                return Ok(output.join("\n"));
            }

            let mut pools: Vec<&String> = map.keys().collect();
            pools.sort_by_key(|a| natural_sort_key(a));

            let mut rows: Vec<Vec<String>> = Vec::new();
            for pool in pools {
                if let Some(value) = map.get(pool) {
                    let bytes = if let Value::Object(val_map) = value {
                        val_map
                            .get("Bytes")
                            .or_else(|| val_map.get("bytes"))
                            .map(value_to_string)
                            .unwrap_or_else(|| "N/A".to_string())
                    } else {
                        value_to_string(value)
                    };
                    rows.push(vec![pool.to_string(), bytes]);
                }
            }

            output.push(tabulate_simple(headers, &rows));
        }

        Ok(output.join("\n"))
    }

    fn name(&self) -> &'static str {
        "buffer_pool"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_watermark_telemetry_formatter() {
        let formatter = WatermarkTelemetryFormatter::new();
        let data = json!({"interval": "120"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("120"));
        assert!(result.contains("second(s)"));
    }

    #[test]
    fn test_watermark_telemetry_formatter_number() {
        let formatter = WatermarkTelemetryFormatter::new();
        let data = json!(300);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("300"));
    }

    #[test]
    fn test_queue_watermark_formatter() {
        let formatter = QueueWatermarkFormatter::new();
        let data = json!({
            "Ethernet0": {"UC0": "100", "UC1": "200"},
            "Ethernet4": {"UC0": "150", "UC1": "250"}
        });
        let config = CommandConfig::new()
            .with_title("Egress shared pool occupancy per unicast queue:")
            .with_dynamic_columns(true);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("UC0"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_queue_watermark_formatter_empty() {
        let formatter = QueueWatermarkFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Object map is empty!");
    }

    #[test]
    fn test_headroom_pool_formatter() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "ingress_lossless_pool": {"Bytes": "12345"}
        });
        let config = CommandConfig::new().with_title("Headroom pool maximum occupancy:");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ingress_lossless_pool"));
        assert!(result.contains("12345"));
    }

    #[test]
    fn test_headroom_pool_natural_sort() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "pool10": {"Bytes": "100"},
            "pool2": {"Bytes": "200"},
            "pool1": {"Bytes": "300"}
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Check natural sort order
        let pos1 = result.find("pool1").unwrap();
        let pos2 = result.find("pool2").unwrap();
        let pos10 = result.find("pool10").unwrap();
        assert!(pos1 < pos2 && pos2 < pos10);
    }

    #[test]
    fn test_watermark_telemetry_string() {
        let formatter = WatermarkTelemetryFormatter::new();
        // String values not handled - returns empty
        let data = json!("60");
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_watermark_telemetry_empty_dict() {
        let formatter = WatermarkTelemetryFormatter::new();
        // Empty dict without interval key returns empty
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_watermark_telemetry_with_interval_key() {
        let formatter = WatermarkTelemetryFormatter::new();
        let data = json!({"Interval": "180"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("180"));
    }

    #[test]
    fn test_watermark_telemetry_strips_s_suffix() {
        let formatter = WatermarkTelemetryFormatter::new();
        let data = json!({"interval": "120s"});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Telemetry interval 120 second(s)");
    }

    #[test]
    fn test_queue_watermark_with_title() {
        let formatter = QueueWatermarkFormatter::new();
        let data = json!({
            "Ethernet0": {"UC0": "50"}
        });
        let config = CommandConfig::new().with_title("Custom title:");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Custom title:"));
        assert!(result.contains("Ethernet0"));
    }

    #[test]
    fn test_queue_watermark_unsorted_keys() {
        let formatter = QueueWatermarkFormatter::new();
        let data = json!({
            "Ethernet10": {"UC0": "10"},
            "Ethernet2": {"UC0": "20"},
            "Ethernet1": {"UC0": "30"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Natural sort should order: 1, 2, 10
        let pos1 = result.find("Ethernet1").unwrap();
        let pos2 = result.find("Ethernet2").unwrap();
        let pos10 = result.find("Ethernet10").unwrap();
        assert!(pos1 < pos2 && pos2 < pos10);
    }

    #[test]
    fn test_headroom_pool_simple_value() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "pool1": "1234"
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("pool1"));
        assert!(result.contains("1234"));
    }

    #[test]
    fn test_headroom_pool_bytes_lowercase() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "pool1": {"bytes": "5678"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("5678"));
    }

    #[test]
    fn test_headroom_pool_empty() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty() || result.contains("Pool"));
    }

    #[test]
    fn test_headroom_pool_with_title() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "pool1": {"Bytes": "100"}
        });
        let config = CommandConfig::new().with_title("Pool occupancy:");
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Pool occupancy:"));
    }

    // =======================================================================
    // Phase 3.6 — Additional edge case tests for watermark formatters (GAP-B6)
    // =======================================================================

    #[test]
    fn test_watermark_telemetry_default() {
        let formatter = WatermarkTelemetryFormatter::default();
        assert_eq!(formatter.name(), "watermark_telemetry");
    }

    #[test]
    fn test_queue_watermark_default() {
        let formatter = QueueWatermarkFormatter::default();
        assert_eq!(formatter.name(), "queue_watermark");
    }

    #[test]
    fn test_headroom_pool_default() {
        let formatter = HeadroomPoolFormatter::default();
        assert_eq!(formatter.name(), "headroom_pool");
    }

    #[test]
    fn test_queue_watermark_missing_columns_in_some_ports() {
        let formatter = QueueWatermarkFormatter::new();
        let data = json!({
            "Ethernet0": {"UC0": "100", "UC1": "200"},
            "Ethernet4": {"UC0": "150"}
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Ethernet4 is missing UC1, should show N/A for it
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("Ethernet4"));
        assert!(result.contains("N/A"));
    }

    #[test]
    fn test_headroom_pool_no_bytes_key() {
        let formatter = HeadroomPoolFormatter::new();
        let data = json!({
            "pool_x": {"OtherKey": "999"}
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("pool_x"));
        assert!(result.contains("N/A"));
    }
}
