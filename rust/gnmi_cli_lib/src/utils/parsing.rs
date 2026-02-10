//! Parsing utilities for extracting and transforming data.
//!
//! This module provides helper functions for:
//! - Parsing JSON data (string or object)
//! - Extracting interface names from various key formats
//! - Parsing reason strings from drop counters

use serde_json::Value;
use std::collections::HashMap;

/// Parse a reasons string like `[REASON1,REASON2]` into a vector.
///
/// This is commonly used for dropcounters capabilities where reasons
/// are stored as a string representation of a list.
///
/// # Arguments
///
/// * `reasons_str` - String in format "[REASON1,REASON2,...]"
///
/// # Returns
///
/// Vector of reason strings, stripped of whitespace
///
/// # Examples
///
/// ```
/// use gnmi_cli_lib::utils::parsing::parse_reasons_string;
///
/// let reasons = parse_reasons_string("[IP_HEADER_ERROR,NO_L3_HEADER]");
/// assert_eq!(reasons, vec!["IP_HEADER_ERROR", "NO_L3_HEADER"]);
///
/// assert_eq!(parse_reasons_string("[]"), Vec::<String>::new());
/// assert_eq!(parse_reasons_string(""), Vec::<String>::new());
/// ```
pub fn parse_reasons_string(reasons_str: &str) -> Vec<String> {
    let cleaned = reasons_str.trim();

    // Remove brackets
    let inner = if cleaned.starts_with('[') && cleaned.ends_with(']') {
        &cleaned[1..cleaned.len() - 1]
    } else {
        cleaned
    };

    if inner.is_empty() {
        return Vec::new();
    }

    inner
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Extract interface name from various key formats.
///
/// Handles formats like:
/// - "PORT_TABLE:Ethernet0" -> "Ethernet0"
/// - "PORTCHANNEL|PortChannel101" -> "PortChannel101"
/// - "PORT|Ethernet0" -> "Ethernet0"
/// - "Ethernet0" -> "Ethernet0"
///
/// # Arguments
///
/// * `key` - Key string that may contain prefix
///
/// # Returns
///
/// Interface name without prefix
///
/// # Examples
///
/// ```
/// use gnmi_cli_lib::utils::parsing::extract_interface_name;
///
/// assert_eq!(extract_interface_name("PORT_TABLE:Ethernet0"), "Ethernet0");
/// assert_eq!(extract_interface_name("PORTCHANNEL|PortChannel101"), "PortChannel101");
/// assert_eq!(extract_interface_name("Ethernet0"), "Ethernet0");
/// ```
pub fn extract_interface_name(key: &str) -> &str {
    if let Some(pos) = key.rfind(':') {
        &key[pos + 1..]
    } else if let Some(pos) = key.rfind('|') {
        &key[pos + 1..]
    } else {
        key
    }
}

/// Parse JSON data from string or return the value as-is.
///
/// # Arguments
///
/// * `json_data` - JSON data as string or Value
///
/// # Returns
///
/// Parsed JSON Value
///
/// # Errors
///
/// Returns error if string is not valid JSON
pub fn parse_json_string(json_str: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

/// Parse JSON data from a string, returning a Value.
///
/// This is the primary function for parsing JSON input from external sources.
///
/// # Arguments
///
/// * `json_str` - JSON string to parse
///
/// # Returns
///
/// Parsed JSON Value
///
/// # Errors
///
/// Returns error if string is not valid JSON
pub fn parse_json_data(json_str: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

/// Convert a JSON array of objects to a HashMap keyed by a field.
///
/// # Arguments
///
/// * `data` - JSON array of objects
/// * `key_field` - Field name to use as key
///
/// # Returns
///
/// HashMap with key field values as keys and objects as values
pub fn array_to_dict(data: &Value, key_field: &str) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    if let Value::Array(arr) = data {
        for item in arr {
            if let Value::Object(obj) = item {
                if let Some(Value::String(key)) = obj.get(key_field) {
                    result.insert(key.clone(), item.clone());
                }
            }
        }
    }

    result
}

/// Transform interface data from array format to dict format.
///
/// # Arguments
///
/// * `data` - JSON array of interface objects
/// * `field_map` - Mapping from source field names to target field names
///
/// # Returns
///
/// HashMap with interface names as keys
pub fn transform_interface_data(
    data: &Value,
    field_map: &HashMap<&str, &str>,
) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    if let Value::Array(arr) = data {
        for item in arr {
            if let Value::Object(obj) = item {
                // Try to find interface name
                let interface = obj
                    .get("Interface")
                    .or_else(|| obj.get("interface"))
                    .or_else(|| obj.get("Name"))
                    .or_else(|| obj.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if !interface.is_empty() {
                    // Transform fields according to mapping
                    let mut transformed = serde_json::Map::new();
                    for (src, dst) in field_map {
                        if let Some(value) = obj.get(*src) {
                            transformed.insert(dst.to_string(), value.clone());
                        }
                    }
                    // Also include original fields not in mapping
                    for (key, value) in obj {
                        if !field_map.contains_key(key.as_str()) {
                            transformed.insert(key.clone(), value.clone());
                        }
                    }
                    result.insert(interface.to_string(), Value::Object(transformed));
                }
            }
        }
    }

    result
}

/// Extract a string value from a JSON object.
///
/// # Arguments
///
/// * `obj` - JSON object
/// * `keys` - List of possible key names to try
///
/// # Returns
///
/// String value if found, empty string otherwise
pub fn get_string_value(obj: &Value, keys: &[&str]) -> String {
    if let Value::Object(map) = obj {
        for key in keys {
            if let Some(value) = map.get(*key) {
                return match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => String::new(),
                };
            }
        }
    }
    String::new()
}

/// Extract an integer value from a JSON object.
///
/// # Arguments
///
/// * `obj` - JSON object
/// * `keys` - List of possible key names to try
///
/// # Returns
///
/// Integer value if found, 0 otherwise
pub fn get_int_value(obj: &Value, keys: &[&str]) -> i64 {
    if let Value::Object(map) = obj {
        for key in keys {
            if let Some(value) = map.get(*key) {
                return match value {
                    Value::Number(n) => n.as_i64().unwrap_or(0),
                    Value::String(s) => s.parse().unwrap_or(0),
                    _ => 0,
                };
            }
        }
    }
    0
}

/// Check if a JSON value is empty (null, empty string, empty array, empty object).
pub fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    }
}

/// Normalize output by trimming whitespace and removing trailing newlines.
///
/// Preserves leading whitespace (important for table alignment).
pub fn normalize_output(output: &str) -> String {
    let output = output.trim_end();
    let lines: Vec<&str> = output.lines().collect();

    // Remove leading empty lines
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    // Remove trailing empty lines
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map(|p| p + 1)
        .unwrap_or(lines.len());

    lines[start..end]
        .iter()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Normalize a line by collapsing multiple spaces to single space.
pub fn normalize_line_spacing(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts.join(" ")
}

/// Extract all string/number values from a JSON structure recursively.
///
/// Used for coverage checking - ensures all values appear in output.
pub fn extract_json_values(data: &Value) -> Vec<String> {
    let mut values = Vec::new();
    extract_values_recursive(data, &mut values);
    values
}

fn extract_values_recursive(value: &Value, values: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            if !s.is_empty() {
                values.push(s.clone());
            }
        }
        Value::Number(n) => {
            values.push(n.to_string());
        }
        Value::Bool(b) => {
            values.push(b.to_string());
        }
        Value::Array(arr) => {
            for item in arr {
                extract_values_recursive(item, values);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                extract_values_recursive(v, values);
            }
        }
        Value::Null => {}
    }
}

/// Parse queue name like "Ethernet0:UC1" into (port, queue_type, queue_num).
pub fn parse_queue_name(queue_name: &str) -> Option<(String, String, u32)> {
    let parts: Vec<&str> = queue_name.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let port = parts[0].to_string();
    let queue_part = parts[1];

    // Extract queue type and number (e.g., "UC1" -> ("UC", 1))
    let queue_type_end = queue_part
        .chars()
        .position(|c| c.is_ascii_digit())
        .unwrap_or(queue_part.len());
    let queue_type = &queue_part[..queue_type_end];
    let queue_num: u32 = queue_part[queue_type_end..].parse().unwrap_or(0);

    Some((port, queue_type.to_string(), queue_num))
}

/// Parse port name and extract port number.
pub fn parse_port_number(port_name: &str) -> Option<u32> {
    // Find first digit sequence after "Ethernet" or similar prefix
    let num_start = port_name.chars().position(|c| c.is_ascii_digit())?;
    let num_str: String = port_name[num_start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    num_str.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_reasons_string_normal() {
        let reasons = parse_reasons_string("[IP_HEADER_ERROR,NO_L3_HEADER]");
        assert_eq!(reasons, vec!["IP_HEADER_ERROR", "NO_L3_HEADER"]);
    }

    #[test]
    fn test_parse_reasons_string_empty() {
        assert_eq!(parse_reasons_string("[]"), Vec::<String>::new());
        assert_eq!(parse_reasons_string(""), Vec::<String>::new());
    }

    #[test]
    fn test_parse_reasons_string_single() {
        let reasons = parse_reasons_string("[SINGLE_REASON]");
        assert_eq!(reasons, vec!["SINGLE_REASON"]);
    }

    #[test]
    fn test_parse_reasons_string_with_spaces() {
        let reasons = parse_reasons_string("[ REASON1 , REASON2 ]");
        assert_eq!(reasons, vec!["REASON1", "REASON2"]);
    }

    #[test]
    fn test_extract_interface_name_colon() {
        assert_eq!(extract_interface_name("PORT_TABLE:Ethernet0"), "Ethernet0");
    }

    #[test]
    fn test_extract_interface_name_pipe() {
        assert_eq!(
            extract_interface_name("PORTCHANNEL|PortChannel101"),
            "PortChannel101"
        );
    }

    #[test]
    fn test_extract_interface_name_plain() {
        assert_eq!(extract_interface_name("Ethernet0"), "Ethernet0");
    }

    #[test]
    fn test_extract_interface_name_multiple_colons() {
        assert_eq!(extract_interface_name("A:B:Ethernet0"), "Ethernet0");
    }

    #[test]
    fn test_parse_json_string_valid() {
        let result = parse_json_string(r#"{"key": "value"}"#).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_parse_json_string_invalid() {
        let result = parse_json_string("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_array_to_dict() {
        let data = json!([
            {"Interface": "Eth0", "Status": "up"},
            {"Interface": "Eth1", "Status": "down"}
        ]);

        let dict = array_to_dict(&data, "Interface");
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Eth0"));
        assert!(dict.contains_key("Eth1"));
    }

    #[test]
    fn test_array_to_dict_empty() {
        let data = json!([]);
        let dict = array_to_dict(&data, "Interface");
        assert!(dict.is_empty());
    }

    #[test]
    fn test_get_string_value() {
        let obj = json!({"Name": "test", "Value": 123});
        assert_eq!(get_string_value(&obj, &["Name"]), "test");
        assert_eq!(get_string_value(&obj, &["Value"]), "123");
        assert_eq!(get_string_value(&obj, &["Missing"]), "");
    }

    #[test]
    fn test_get_string_value_multiple_keys() {
        let obj = json!({"alias": "eth0"});
        assert_eq!(get_string_value(&obj, &["Name", "alias"]), "eth0");
    }

    #[test]
    fn test_get_int_value() {
        let obj = json!({"Count": 42});
        assert_eq!(get_int_value(&obj, &["Count"]), 42);
        assert_eq!(get_int_value(&obj, &["Missing"]), 0);
    }

    #[test]
    fn test_get_int_value_from_string() {
        let obj = json!({"Count": "42"});
        assert_eq!(get_int_value(&obj, &["Count"]), 42);
    }

    #[test]
    fn test_is_empty_value() {
        assert!(is_empty_value(&Value::Null));
        assert!(is_empty_value(&json!("")));
        assert!(is_empty_value(&json!([])));
        assert!(is_empty_value(&json!({})));
        assert!(!is_empty_value(&json!("test")));
        assert!(!is_empty_value(&json!(42)));
    }

    #[test]
    fn test_normalize_output() {
        let input = "\n\n  Line1  \n  Line2  \n\n";
        let expected = "  Line1\n  Line2";
        assert_eq!(normalize_output(input), expected);
    }

    #[test]
    fn test_normalize_line_spacing() {
        assert_eq!(normalize_line_spacing("  a   b   c  "), "a b c");
    }

    #[test]
    fn test_extract_json_values() {
        let data = json!({
            "name": "test",
            "count": 42,
            "nested": {
                "value": "inner"
            },
            "array": ["a", "b"]
        });

        let values = extract_json_values(&data);
        assert!(values.contains(&"test".to_string()));
        assert!(values.contains(&"42".to_string()));
        assert!(values.contains(&"inner".to_string()));
        assert!(values.contains(&"a".to_string()));
        assert!(values.contains(&"b".to_string()));
    }

    #[test]
    fn test_parse_queue_name() {
        let result = parse_queue_name("Ethernet0:UC1");
        assert_eq!(result, Some(("Ethernet0".to_string(), "UC".to_string(), 1)));

        let result = parse_queue_name("Ethernet4:MC10");
        assert_eq!(
            result,
            Some(("Ethernet4".to_string(), "MC".to_string(), 10))
        );

        assert_eq!(parse_queue_name("InvalidFormat"), None);
    }

    #[test]
    fn test_parse_port_number() {
        assert_eq!(parse_port_number("Ethernet0"), Some(0));
        assert_eq!(parse_port_number("Ethernet10"), Some(10));
        assert_eq!(parse_port_number("PortChannel101"), Some(101));
        assert_eq!(parse_port_number("NoNumber"), None);
    }

    #[test]
    fn test_transform_interface_data() {
        let data = json!([
            {"Interface": "Eth0", "Speed": "100G"},
            {"Interface": "Eth1", "Speed": "25G"}
        ]);

        let mut field_map = HashMap::new();
        field_map.insert("Interface", "interface");
        field_map.insert("Speed", "speed");

        let result = transform_interface_data(&data, &field_map);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("Eth0"));
        assert!(result.contains_key("Eth1"));
    }

    // --- Additional GAP-B1 tests ---

    #[test]
    fn test_parse_reasons_string_no_brackets() {
        let reasons = parse_reasons_string("REASON1,REASON2");
        assert_eq!(reasons, vec!["REASON1", "REASON2"]);
    }

    #[test]
    fn test_parse_reasons_string_whitespace_only() {
        let reasons = parse_reasons_string("   ");
        assert!(reasons.is_empty());
    }

    #[test]
    fn test_parse_reasons_string_many_reasons() {
        let reasons = parse_reasons_string("[A,B,C,D,E,F]");
        assert_eq!(reasons.len(), 6);
        assert_eq!(reasons[0], "A");
        assert_eq!(reasons[5], "F");
    }

    #[test]
    fn test_extract_interface_name_empty() {
        assert_eq!(extract_interface_name(""), "");
    }

    #[test]
    fn test_extract_interface_name_pipe_multiple() {
        assert_eq!(extract_interface_name("TABLE|Group|Eth0"), "Eth0");
    }

    #[test]
    fn test_parse_json_string_array() {
        let result = parse_json_string(r#"[1, 2, 3]"#).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_parse_json_string_number() {
        let result = parse_json_string("42").unwrap();
        assert_eq!(result.as_i64(), Some(42));
    }

    #[test]
    fn test_parse_json_data_valid() {
        let result = parse_json_data(r#"{"key": "value"}"#).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_parse_json_data_invalid() {
        let result = parse_json_data("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn test_array_to_dict_non_array() {
        let data = json!({"not": "array"});
        let dict = array_to_dict(&data, "key");
        assert!(dict.is_empty());
    }

    #[test]
    fn test_array_to_dict_missing_key() {
        let data = json!([{"a": "1"}, {"b": "2"}]);
        let dict = array_to_dict(&data, "Interface");
        assert!(dict.is_empty());
    }

    #[test]
    fn test_get_string_value_bool() {
        let obj = json!({"flag": true});
        assert_eq!(get_string_value(&obj, &["flag"]), "true");
    }

    #[test]
    fn test_get_string_value_null() {
        let obj = json!({"key": null});
        assert_eq!(get_string_value(&obj, &["key"]), "");
    }

    #[test]
    fn test_get_string_value_nested_ignored() {
        let obj = json!({"key": {"nested": "val"}});
        // Nested objects return empty string
        assert_eq!(get_string_value(&obj, &["key"]), "");
    }

    #[test]
    fn test_get_string_value_non_object() {
        let data = json!("just a string");
        assert_eq!(get_string_value(&data, &["anything"]), "");
    }

    #[test]
    fn test_get_int_value_negative() {
        let obj = json!({"val": -5});
        assert_eq!(get_int_value(&obj, &["val"]), -5);
    }

    #[test]
    fn test_get_int_value_string_invalid() {
        let obj = json!({"val": "not_a_number"});
        assert_eq!(get_int_value(&obj, &["val"]), 0);
    }

    #[test]
    fn test_get_int_value_non_object() {
        let data = json!(42);
        assert_eq!(get_int_value(&data, &["anything"]), 0);
    }

    #[test]
    fn test_is_empty_value_number() {
        assert!(!is_empty_value(&json!(0)));
        assert!(!is_empty_value(&json!(1)));
    }

    #[test]
    fn test_is_empty_value_bool() {
        assert!(!is_empty_value(&json!(false)));
        assert!(!is_empty_value(&json!(true)));
    }

    #[test]
    fn test_is_empty_value_nonempty_array() {
        assert!(!is_empty_value(&json!([1])));
    }

    #[test]
    fn test_is_empty_value_nonempty_object() {
        assert!(!is_empty_value(&json!({"k": "v"})));
    }

    #[test]
    fn test_normalize_output_empty() {
        assert_eq!(normalize_output(""), "");
    }

    #[test]
    fn test_normalize_output_single_line() {
        assert_eq!(normalize_output("  hello  "), "  hello");
    }

    #[test]
    fn test_normalize_output_preserves_indentation() {
        let input = "  col1  col2\n  val1  val2";
        let result = normalize_output(input);
        assert!(result.starts_with("  col1"));
    }

    #[test]
    fn test_normalize_line_spacing_multiple_spaces() {
        assert_eq!(normalize_line_spacing("a    b    c"), "a b c");
    }

    #[test]
    fn test_normalize_line_spacing_tabs() {
        assert_eq!(normalize_line_spacing("a\tb\tc"), "a b c");
    }

    #[test]
    fn test_normalize_line_spacing_empty() {
        assert_eq!(normalize_line_spacing(""), "");
    }

    #[test]
    fn test_extract_json_values_null() {
        let data = json!(null);
        let values = extract_json_values(&data);
        assert!(values.is_empty());
    }

    #[test]
    fn test_extract_json_values_bool() {
        let data = json!(true);
        let values = extract_json_values(&data);
        assert!(values.contains(&"true".to_string()));
    }

    #[test]
    fn test_extract_json_values_nested_array() {
        let data = json!({"items": [{"val": "a"}, {"val": "b"}]});
        let values = extract_json_values(&data);
        assert!(values.contains(&"a".to_string()));
        assert!(values.contains(&"b".to_string()));
    }

    #[test]
    fn test_parse_queue_name_all_cpu() {
        let result = parse_queue_name("CPU:ALL15");
        assert_eq!(result, Some(("CPU".to_string(), "ALL".to_string(), 15)));
    }

    #[test]
    fn test_parse_queue_name_no_number() {
        let result = parse_queue_name("Ethernet0:UC");
        // queue_type_end = len, queue_num parse fails -> 0
        assert_eq!(result, Some(("Ethernet0".to_string(), "UC".to_string(), 0)));
    }

    #[test]
    fn test_parse_queue_name_multiple_colons() {
        let result = parse_queue_name("A:B:C");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_port_number_portchannel() {
        assert_eq!(parse_port_number("PortChannel5"), Some(5));
    }

    #[test]
    fn test_parse_port_number_no_digits() {
        assert_eq!(parse_port_number("Loopback"), None);
    }

    #[test]
    fn test_parse_port_number_empty() {
        assert_eq!(parse_port_number(""), None);
    }

    #[test]
    fn test_transform_interface_data_empty() {
        let data = json!([]);
        let field_map = HashMap::new();
        let result = transform_interface_data(&data, &field_map);
        assert!(result.is_empty());
    }

    #[test]
    fn test_transform_interface_data_non_array() {
        let data = json!({"not": "array"});
        let field_map = HashMap::new();
        let result = transform_interface_data(&data, &field_map);
        assert!(result.is_empty());
    }

    // ---- Round 5 coverage tests ----

    #[test]
    fn test_get_int_value_non_numeric() {
        // Cover L247: non-Number, non-String value returns 0
        let data = json!({"count": [1, 2, 3]});
        let result = get_int_value(&data, &["count"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_transform_interface_data_missing_field() {
        // Cover L192: field not found in source object
        let data = json!([
            {"interface": "Ethernet0", "STATE": {"RX_OK": "100"}}
        ]);
        let mut field_map = HashMap::new();
        field_map.insert("NONEXISTENT_FIELD", "mapped_name");
        let result = transform_interface_data(&data, &field_map);
        // The mapping references a non-existent field, so it should skip it
        assert!(!result.is_empty());
    }
}
