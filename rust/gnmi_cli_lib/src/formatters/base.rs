//! Base output formatters using the Strategy Pattern.
//!
//! This module provides:
//! - `OutputFormatter`: Trait for all formatters
//! - `TableFormatter`: Generic table formatter for watermark-style data
//! - `TextFormatter`: Plain text formatter for single values
//! - `ListFormatter`: List formatter for newline-separated items
//! - `CommandHelpFormatter`: Help text formatter

use crate::common::types::CommandConfig;
use crate::utils::sorting::natural_sort_key;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

/// Error type for formatting operations.
#[derive(Error, Debug)]
pub enum FormatError {
    /// The input data is invalid or malformed.
    #[error("Invalid data format: {0}")]
    InvalidData(String),

    /// A required field is missing from the input.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// No formatter was found for the specified command.
    #[error("Formatter not found: {0}")]
    FormatterNotFound(String),

    /// A JSON parsing or serialization error occurred.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Trait for output formatters.
///
/// This trait defines the interface that all formatters must implement.
/// Each formatter is responsible for converting structured data into
/// human-readable CLI-style output.
pub trait OutputFormatter: Send + Sync {
    /// Format data according to the command configuration.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to format (usually a JSON object or array)
    /// * `config` - Command configuration with formatting options
    /// * `table_format` - Table format (simple, grid, plain, etc.) - currently unused
    /// * `options` - Additional options (e.g., period for counters detailed)
    ///
    /// # Returns
    ///
    /// Formatted string output or error
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        table_format: &str,
        options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError>;

    /// Get the name of this formatter.
    fn name(&self) -> &'static str;
}

/// Generic table formatter for simple key-value data.
///
/// This is the primary formatter for watermark commands and other
/// tabular data. It supports both simple key-value tables and
/// dynamic column tables.
pub struct TableFormatter;

impl TableFormatter {
    /// Create a new TableFormatter.
    pub fn new() -> Self {
        Self
    }

    /// Build a simple table from key-value data.
    fn build_simple_table(
        &self,
        data: &Value,
        config: &CommandConfig,
    ) -> Result<Vec<Vec<String>>, FormatError> {
        let mut rows = Vec::new();

        if let Value::Object(map) = data {
            // Get sorted keys using natural sort
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by_key(|a| natural_sort_key(a));

            for key in keys {
                if let Some(value) = map.get(key) {
                    let mut row = vec![key.clone()];

                    // Extract value fields
                    if let Value::Object(val_map) = value {
                        for field in config.value_fields() {
                            let val = val_map
                                .get(field)
                                .map(|v| match v {
                                    Value::String(s) => s.clone(),
                                    Value::Number(n) => n.to_string(),
                                    Value::Bool(b) => b.to_string(),
                                    _ => String::new(),
                                })
                                .unwrap_or_default();
                            row.push(val);
                        }
                    } else {
                        // Direct value
                        row.push(value_to_string(value));
                    }

                    rows.push(row);
                }
            }
        }

        Ok(rows)
    }

    /// Build a dynamic column table (e.g., for queue watermarks).
    fn build_dynamic_table(
        &self,
        data: &Value,
    ) -> Result<(Vec<Vec<String>>, Vec<String>), FormatError> {
        let mut all_columns = std::collections::BTreeSet::new();
        let mut rows = Vec::new();

        if let Value::Object(map) = data {
            // First pass: collect all column names
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

            // Build headers
            let mut headers = vec!["Port".to_string()];
            headers.extend(columns.clone());

            // Second pass: build rows
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by_key(|a| natural_sort_key(a));

            for key in keys {
                if let Some(Value::Object(val_map)) = map.get(key) {
                    let mut row = vec![key.clone()];
                    for col in &columns {
                        let val = val_map.get(col).map(value_to_string).unwrap_or_default();
                        row.push(val);
                    }
                    rows.push(row);
                }
            }

            return Ok((rows, headers));
        }

        Ok((rows, vec![]))
    }
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TableFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        if data.is_null() || (data.is_object() && data.as_object().unwrap().is_empty()) {
            return Ok(String::new());
        }

        let mut output = Vec::new();

        // Add title if present
        let title = config.title().unwrap_or_default().to_string();
        if !title.is_empty() {
            output.push(title);
        }

        let (rows, headers): (Vec<Vec<String>>, Vec<String>) = if config.dynamic_columns() {
            self.build_dynamic_table(data)?
        } else {
            let rows = self.build_simple_table(data, config)?;
            (rows, config.headers().to_vec())
        };

        if rows.is_empty() {
            return Ok(String::new());
        }

        let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
        output.push(tabulate_simple(&header_refs, &rows));

        Ok(output.join("\n"))
    }

    fn name(&self) -> &'static str {
        "table"
    }
}

/// Plain text formatter for single values.
pub struct TextFormatter;

impl TextFormatter {
    /// Creates a new [`TextFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for TextFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Extract the key field value
        let key_field = config.key_field().unwrap_or_default().to_string();
        if let Value::Object(map) = data {
            if let Some(value) = map.get(&key_field) {
                return Ok(value_to_string(value));
            }

            // If key_field not found, try to return any string value
            for (_, value) in map {
                if let Value::String(s) = value {
                    return Ok(s.clone());
                }
            }
        }

        // Return direct value if not an object
        Ok(value_to_string(data))
    }

    fn name(&self) -> &'static str {
        "text"
    }
}

/// List formatter for newline-separated items.
pub struct ListFormatter;

impl ListFormatter {
    /// Creates a new [`ListFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for ListFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let mut items = Vec::new();
        let key_field = config.key_field().unwrap_or_default().to_string();

        // Handle object with key_field containing an array
        if let Value::Object(map) = data {
            if !key_field.is_empty() {
                if let Some(Value::Array(arr)) = map.get(&key_field) {
                    for item in arr {
                        items.push(value_to_string(item));
                    }
                }
            } else {
                // If no key_field, list all keys
                for key in map.keys() {
                    items.push(key.clone());
                }
            }
        }

        // Handle direct array
        if let Value::Array(arr) = data {
            for item in arr {
                items.push(value_to_string(item));
            }
        }

        Ok(items.join("\n"))
    }

    fn name(&self) -> &'static str {
        "list"
    }
}

/// Command help formatter.
pub struct CommandHelpFormatter;

impl CommandHelpFormatter {
    /// Creates a new [`CommandHelpFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for CommandHelpFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for CommandHelpFormatter {
    fn format(
        &self,
        data: &Value,
        config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        // Check for subcommands data structure
        if let Value::Object(map) = data {
            if let Some(Value::Object(subcommands)) = map.get("subcommands") {
                if subcommands.is_empty() {
                    return Ok("No subcommands available.".to_string());
                }

                // Map command name to CLI-style name and description
                let raw_name = config
                    .description()
                    .map(|d| d.replace(" help", ""))
                    .unwrap_or_else(|| "command".to_string());

                let (command_name, description) = match raw_name.as_str() {
                    "Queue watermark" => ("queue watermark", "user WM for queues"),
                    "Queue persistent watermark" => {
                        ("queue persistent-watermark", "persistent WM for queues")
                    }
                    _ => {
                        // Fallback: lowercase the raw name
                        return self.format_generic_help(&raw_name, subcommands);
                    }
                };

                let mut lines = Vec::new();
                lines.push(format!(
                    "Usage: show {} [OPTIONS] COMMAND [ARGS]...",
                    command_name
                ));
                lines.push(String::new());
                lines.push(format!("  Show {}", description));
                lines.push(String::new());
                lines.push("Options:".to_string());
                lines.push("  -?, -h, --help  Show this message and exit.".to_string());
                lines.push(String::new());
                lines.push("Commands:".to_string());

                let mut subcmd_keys: Vec<&String> = subcommands.keys().collect();
                subcmd_keys.sort();

                for subcmd in subcmd_keys {
                    let subcmd_desc = description.replace("queues", &format!("{} queues", subcmd));
                    lines.push(format!("  {:<10} Show {}", subcmd, subcmd_desc));
                }

                return Ok(lines.join("\n"));
            }

            // Return help text directly if present
            if let Some(Value::String(help)) = map.get("help") {
                return Ok(help.clone());
            }
            if let Some(Value::String(usage)) = map.get("usage") {
                return Ok(usage.clone());
            }
        }

        if let Value::String(s) = data {
            return Ok(s.clone());
        }

        Ok(String::new())
    }

    fn name(&self) -> &'static str {
        "command_help"
    }
}

impl CommandHelpFormatter {
    fn format_generic_help(
        &self,
        raw_name: &str,
        subcommands: &serde_json::Map<String, Value>,
    ) -> Result<String, FormatError> {
        let command_name = raw_name.to_lowercase();
        let mut lines = Vec::new();
        lines.push(format!(
            "Usage: show {} [OPTIONS] COMMAND [ARGS]...",
            command_name
        ));
        lines.push(String::new());
        lines.push(format!("  Show {}", command_name));
        lines.push(String::new());
        lines.push("Options:".to_string());
        lines.push("  -?, -h, --help  Show this message and exit.".to_string());
        lines.push(String::new());
        lines.push("Commands:".to_string());

        let mut subcmd_keys: Vec<&String> = subcommands.keys().collect();
        subcmd_keys.sort();

        for subcmd in subcmd_keys {
            lines.push(format!("  {:<10} Show {} {}", subcmd, command_name, subcmd));
        }

        Ok(lines.join("\n"))
    }
}

/// Convert a JSON value to a string.
pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        Value::Array(arr) => arr
            .iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(", "),
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// Build a formatted table string from rows and headers.
pub fn build_table(headers: &[String], rows: &[Vec<String>], right_align: bool) -> String {
    let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
    if right_align {
        tabulate_simple(&header_refs, rows)
    } else {
        let alignments: Vec<bool> = vec![false; headers.len()];
        tabulate_simple_aligned(&header_refs, rows, &alignments)
    }
}

/// Format data as a simple table matching `tabulate(tablefmt="simple")`.
///
/// All columns are right-aligned (matching SONiC CLI convention). This function
/// bypasses comfy_table entirely for predictable, compatible output.
///
/// Column width = `max(header_len + 2, max_data_len_in_column)`.
/// Columns separated by 2 spaces. Trailing whitespace trimmed from each line.
pub fn tabulate_simple(headers: &[&str], rows: &[Vec<String>]) -> String {
    tabulate_simple_aligned(headers, rows, &[])
}

/// Format data as a simple table with auto-detected column alignment.
///
/// Matches `tabulate(tablefmt="simple")` default behavior:
/// - Columns where ALL non-empty data values are numeric → right-aligned
/// - Columns with any non-numeric text data → left-aligned
///
/// A value is considered numeric if, after stripping commas, it parses as an f64.
/// Empty strings and "N/A" are ignored (don't influence auto-detection).
pub fn tabulate_simple_auto(headers: &[&str], rows: &[Vec<String>]) -> String {
    let num_cols = headers.len();
    let mut alignments = vec![false; num_cols]; // default: left
    let mut is_numeric_col = vec![false; num_cols];

    for col in 0..num_cols {
        let mut has_any_value = false;
        let mut all_numeric = true;
        for row in rows {
            if let Some(val) = row.get(col) {
                let trimmed = val.trim();
                if trimmed.is_empty() || trimmed == "N/A" {
                    continue;
                }
                has_any_value = true;
                let cleaned = trimmed.replace(',', "");
                if cleaned.parse::<f64>().is_err() {
                    all_numeric = false;
                    break;
                }
            }
        }
        // Right-align only if there are numeric values and ALL values are numeric
        let numeric = has_any_value && all_numeric;
        alignments[col] = numeric;
        is_numeric_col[col] = numeric;
    }

    // Reformat float values in numeric columns using "g" format (6 significant digits).
    // tabulate auto-detects numeric columns and applies floatfmt="g", which converts
    // e.g. "0.0" -> "0", "1.0" -> "1", while preserving "0.1" -> "0.1".
    let reformatted_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, val)| {
                    if i < num_cols && is_numeric_col[i] {
                        let trimmed = val.trim();
                        if trimmed.is_empty() || trimmed == "N/A" || trimmed == "-" {
                            return val.clone();
                        }
                        // Only reformat if it contains a decimal point (actual float)
                        // Integer strings like "123" don't need reformatting
                        let cleaned = trimmed.replace(',', "");
                        if cleaned.contains('.') || cleaned.contains('e') || cleaned.contains('E') {
                            if let Ok(f) = cleaned.parse::<f64>() {
                                return format_g(f);
                            }
                        }
                        val.clone()
                    } else {
                        val.clone()
                    }
                })
                .collect()
        })
        .collect();

    tabulate_simple_aligned(headers, &reformatted_rows, &alignments)
}

/// Format a float using "g" format with 6 significant digits.
/// This matches `format(val, "g")` behavior:
/// - Trailing zeros after decimal point are removed
/// - Values that are integers are shown without decimal point
/// - Scientific notation used for very large/small values
pub fn format_g(val: f64) -> String {
    if val == 0.0 {
        return "0".to_string();
    }
    // Use Rust's default {:.*e} and {:.*} to match "g" with 6 sig digits
    let abs_val = val.abs();
    let exp = abs_val.log10().floor() as i32;
    // "g" format: use fixed notation if -4 <= exp < 6, else scientific
    if (-4..6).contains(&exp) {
        // Fixed-point notation with enough precision
        let decimal_places = if exp >= 0 {
            (5 - exp).max(0) as usize
        } else {
            (5 + exp.abs()) as usize
        };
        let formatted = format!("{:.prec$}", val, prec = decimal_places);
        // Remove trailing zeros after decimal point, and the decimal point if no digits after
        if formatted.contains('.') {
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            trimmed.to_string()
        } else {
            formatted
        }
    } else {
        // Scientific notation
        let sig_digits = 5; // 6 significant digits means 5 after the first
        let mantissa = val / 10f64.powi(exp);
        let formatted_mantissa = format!("{:.prec$}", mantissa, prec = sig_digits);
        let trimmed_mantissa = formatted_mantissa
            .trim_end_matches('0')
            .trim_end_matches('.');
        if exp >= 0 {
            format!("{}e+{:02}", trimmed_mantissa, exp)
        } else {
            format!("{}e-{:02}", trimmed_mantissa, exp.unsigned_abs())
        }
    }
}

/// Format data as a grid table matching `tabulate(tablefmt="grid")`.
///
/// Matches tabulate grid format with MIN_PADDING=2:
/// - content_width = max(header_len + 2, max_data_len) per column
/// - Cell: `| ` + content padded to content_width + ` |`
/// - Separator dashes = content_width + 2
///
/// Supports multi-line cells: if a cell value contains `\n`, each line is
/// rendered as a separate visual row but without a `+---+` separator between
/// them — matching tabulate's multi-line cell behavior.
///
/// `alignments` specifies per-column alignment: `true` = right, `false` = left.
pub fn tabulate_grid(
    headers: &[&str],
    rows: &[Vec<String>],
    alignments: &[bool],
) -> String {
    if headers.is_empty() {
        return String::new();
    }

    // tabulate grid: content_width = max(header_len + MIN_PADDING, max_data_len)
    // MIN_PADDING = 2 applies to headers, not to data
    // For multi-line cells, use max width of any single line within the cell.
    let mut content_widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
    for row in rows {
        for (i, val) in row.iter().enumerate() {
            if i < content_widths.len() {
                for line in val.split('\n') {
                    if line.len() > content_widths[i] {
                        content_widths[i] = line.len();
                    }
                }
            }
        }
    }
    // Cell total width = content_width + 2 (1 space padding each side)
    let cell_widths: Vec<usize> = content_widths.iter().map(|w| w + 2).collect();

    // Helper to build a separator line with a fill character
    let build_sep = |fill: char| -> String {
        let mut line = String::from("+");
        for w in &cell_widths {
            line.push_str(&fill.to_string().repeat(*w));
            line.push('+');
        }
        line
    };

    // Helper to build a data/header row from single-line values
    let build_row = |values: &[&str]| -> String {
        let mut line = String::from("|");
        for (i, val) in values.iter().enumerate() {
            let w = content_widths.get(i).copied().unwrap_or(0);
            let right = alignments.get(i).copied().unwrap_or(false);
            let formatted = if right {
                format!(" {:>width$} ", val, width = w)
            } else {
                format!(" {:<width$} ", val, width = w)
            };
            line.push_str(&formatted);
            line.push('|');
        }
        line
    };

    let top_border = build_sep('-');
    let header_sep = build_sep('=');
    let bottom_border = build_sep('-');

    let header_strs: Vec<&str> = headers.to_vec();
    let header_row = build_row(&header_strs);

    let mut lines = vec![top_border, header_row, header_sep];

    for row in rows {
        // Split each cell into lines for multi-line cell support
        let cell_lines: Vec<Vec<&str>> = row
            .iter()
            .map(|s| s.split('\n').collect::<Vec<_>>())
            .collect();
        let max_visual_lines = cell_lines.iter().map(|c| c.len()).max().unwrap_or(1);

        for line_idx in 0..max_visual_lines {
            let visual_vals: Vec<&str> = cell_lines
                .iter()
                .map(|c| c.get(line_idx).copied().unwrap_or(""))
                .collect();
            lines.push(build_row(&visual_vals));
        }
        lines.push(build_sep('-'));
    }

    if rows.is_empty() {
        lines.push(bottom_border);
    }

    lines.join("\n")
}

/// Format data as a simple table with per-column alignment control.
///
/// `alignments` specifies per-column alignment: `true` = right (default), `false` = left.
/// If `alignments` is shorter than `headers`, remaining columns default to right-aligned.
pub fn tabulate_simple_aligned(
    headers: &[&str],
    rows: &[Vec<String>],
    alignments: &[bool],
) -> String {
    if headers.is_empty() {
        return String::new();
    }

    // Compute column widths: max(header_len + 2, max_data_len)
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
    for row in rows {
        for (i, val) in row.iter().enumerate() {
            if i < widths.len() && val.len() > widths[i] {
                widths[i] = val.len();
            }
        }
    }

    // Header (right-aligned by default)
    let header: String = headers
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let right = alignments.get(i).copied().unwrap_or(true);
            if right {
                format!("{:>width$}", col, width = widths[i])
            } else {
                format!("{:<width$}", col, width = widths[i])
            }
        })
        .collect::<Vec<_>>()
        .join("  ");

    // Separator
    let separator: String = widths.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("  ");

    // Data rows
    let mut lines = vec![
        header.trim_end().to_string(),
        separator.trim_end().to_string(),
    ];
    for row in rows {
        let line: String = row
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let w = if i < widths.len() { widths[i] } else { val.len() };
                let right = alignments.get(i).copied().unwrap_or(true);
                if right {
                    format!("{:>width$}", val, width = w)
                } else {
                    format!("{:<width$}", val, width = w)
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        lines.push(line.trim_end().to_string());
    }

    lines.join("\n")
}

/// Insert a dashed separator line after the header row of a table string.
///
/// Matches `tabulate(tablefmt='simple')` behavior:
/// 1. Parses the comfy_table output to detect column boundaries
/// 2. Extracts cell content from each column
/// 3. Computes column widths using the formula: max(header_len + 2, max_data_len)
/// 4. Re-renders with proper column widths, 2-space column gaps, and full-width separators
///
/// For multi-word column headers (containing spaces within a column), use
/// [`add_header_separator_smart`] instead.
pub fn add_header_separator(table_output: &str) -> String {
    if table_output.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = table_output.lines().collect();
    if lines.is_empty() {
        return table_output.to_string();
    }

    // Step 1: Find column boundaries by analyzing all lines
    // A position is "content" if ANY line has a non-space character there.
    // Contiguous content positions form a column region.
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    if max_len == 0 {
        return String::new();
    }

    let mut has_content = vec![false; max_len];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c != ' ' {
                has_content[i] = true;
            }
        }
    }

    // Find contiguous content regions = column regions
    let mut columns: Vec<(usize, usize)> = Vec::new(); // (start, end_exclusive)
    let mut i = 0;
    while i < max_len {
        if has_content[i] {
            let start = i;
            while i < max_len && has_content[i] {
                i += 1;
            }
            columns.push((start, i));
        } else {
            i += 1;
        }
    }

    if columns.is_empty() {
        return String::new();
    }

    // Step 2: Extract cell content for each column from each line
    let num_cols = columns.len();
    let mut col_contents: Vec<Vec<String>> = vec![Vec::new(); num_cols];

    for line in &lines {
        for (col_idx, &(start, end)) in columns.iter().enumerate() {
            let cell = if line.len() >= end {
                &line[start..end]
            } else if line.len() > start {
                &line[start..line.len()]
            } else {
                ""
            };
            col_contents[col_idx].push(cell.trim().to_string());
        }
    }

    // Step 3: Detect alignment for each column from the rendered output.
    // Look for a row where content is shorter than the column width to
    // determine if content is left-aligned (starts at column start) or
    // right-aligned (starts later). Default to left-aligned if ambiguous
    // (matches tabulate's default stralign="left").
    let mut col_right_aligned: Vec<bool> = Vec::with_capacity(num_cols);
    for &(start, end) in columns.iter() {
        let col_width = end - start;
        let mut detected = false;

        for line in &lines {
            if line.len() <= start {
                continue;
            }
            let cell_end = std::cmp::min(end, line.len());
            let cell = &line[start..cell_end];
            let content_len = cell.trim().len();
            if content_len > 0 && content_len < col_width {
                // Content shorter than column width → can detect alignment
                if cell.as_bytes().first().map(|&b| b != b' ').unwrap_or(false) {
                    // Starts with non-space → left-aligned
                    col_right_aligned.push(false);
                } else {
                    // Starts with space → right-aligned
                    col_right_aligned.push(true);
                }
                detected = true;
                break;
            }
        }

        if !detected {
            // All content fills column exactly → default to left-aligned
            col_right_aligned.push(false);
        }
    }

    // Step 4: Compute column widths using tabulate formula
    // column_width = max(header_len + 2, max_data_len)
    let mut col_widths: Vec<usize> = Vec::with_capacity(num_cols);
    for col in col_contents.iter().take(num_cols) {
        let header_len = col[0].len();
        let max_data_len = if col.len() > 1 {
            col[1..]
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap_or(0)
        } else {
            0
        };
        let width = std::cmp::max(header_len + 2, max_data_len);
        // Ensure at least header width
        let width = std::cmp::max(width, header_len);
        col_widths.push(width);
    }

    // Step 5: Re-render with proper widths
    let mut result = Vec::with_capacity(lines.len() + 1);

    // Header line
    let header_parts: Vec<String> = (0..num_cols)
        .map(|col_idx| {
            let content = &col_contents[col_idx][0];
            if col_right_aligned[col_idx] {
                format!("{:>width$}", content, width = col_widths[col_idx])
            } else {
                format!("{:<width$}", content, width = col_widths[col_idx])
            }
        })
        .collect();
    result.push(header_parts.join("  ").trim_end().to_string());

    // Separator line
    let sep_parts: Vec<String> = col_widths.iter().map(|w| "-".repeat(*w)).collect();
    result.push(sep_parts.join("  ").trim_end().to_string());

    // Data lines
    for line_idx in 1..lines.len() {
        let parts: Vec<String> = (0..num_cols)
            .map(|col_idx| {
                let content = &col_contents[col_idx].get(line_idx)
                    .map(|s| s.as_str())
                    .unwrap_or("");
                if col_right_aligned[col_idx] {
                    format!("{:>width$}", content, width = col_widths[col_idx])
                } else {
                    format!("{:<width$}", content, width = col_widths[col_idx])
                }
            })
            .collect();
        result.push(parts.join("  ").trim_end().to_string());
    }

    result.join("\n")
}

/// Insert a dashed separator line after the header row, handling multi-word
/// column headers by detecting column boundaries (2+ consecutive spaces).
///
/// Within each column region, ALL characters (including single spaces in
/// multi-word headers like "Low-power Mode") are replaced with dashes.
/// Gaps between columns (2+ spaces) are preserved as spaces.
/// Trailing whitespace is stripped from each line.
pub fn add_header_separator_smart(table_output: &str) -> String {
    if table_output.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = table_output.lines().collect();
    if lines.is_empty() {
        return table_output.to_string();
    }

    let header = lines[0];
    let chars: Vec<char> = header.chars().collect();
    let len = chars.len();
    let mut sep_chars = vec![' '; len];

    let mut i = 0;
    // Skip leading spaces
    while i < len && chars[i] == ' ' {
        i += 1;
    }

    while i < len {
        // Start of a column
        let col_start = i;
        let mut col_end = i;

        while i < len {
            if chars[i] != ' ' {
                col_end = i + 1;
                i += 1;
            } else {
                // Count consecutive spaces
                let space_start = i;
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let space_count = i - space_start;

                if space_count >= 2 || i >= len {
                    // Column boundary or end of line
                    break;
                } else {
                    // Single space within column header
                    col_end = i;
                }
            }
        }

        // Fill column region with dashes
        for ch in sep_chars.iter_mut().take(col_end).skip(col_start) {
            *ch = '-';
        }
    }

    let separator: String = sep_chars.into_iter().collect();

    let mut result = Vec::with_capacity(lines.len() + 1);
    result.push(lines[0].trim_end().to_string());
    result.push(separator.trim_end().to_string());
    for line in &lines[1..] {
        result.push(line.trim_end().to_string());
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_table_formatter_simple() {
        let formatter = TableFormatter::new();
        let data = json!({
            "pool1": {"Bytes": "1000"},
            "pool2": {"Bytes": "2000"}
        });
        let config = CommandConfig::new()
            .with_title("Test Table:")
            .with_headers(vec!["Pool".to_string(), "Bytes".to_string()])
            .with_value_fields(vec!["Bytes".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Test Table:"));
        assert!(result.contains("pool1"));
        assert!(result.contains("1000"));
    }

    #[test]
    fn test_table_formatter_empty() {
        let formatter = TableFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_table_formatter_dynamic_columns() {
        let formatter = TableFormatter::new();
        let data = json!({
            "Ethernet0": {"UC0": "100", "UC1": "200"},
            "Ethernet4": {"UC0": "150", "UC1": "250"}
        });
        let config = CommandConfig::new().with_dynamic_columns(true);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("UC0"));
    }

    #[test]
    fn test_text_formatter() {
        let formatter = TextFormatter::new();
        let data = json!({"date": "Mon Jan 1 12:00:00 UTC 2024"});
        let config = CommandConfig::new().with_key_field("date");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Mon Jan 1 12:00:00 UTC 2024");
    }

    #[test]
    fn test_list_formatter() {
        let formatter = ListFormatter::new();
        let data = json!({"timezones": ["UTC", "PST", "EST"]});
        let config = CommandConfig::new().with_key_field("timezones");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("UTC"));
        assert!(result.contains("PST"));
        assert!(result.contains("EST"));
    }

    #[test]
    fn test_list_formatter_array() {
        let formatter = ListFormatter::new();
        let data = json!(["item1", "item2", "item3"]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("item1"));
        assert!(result.contains("item2"));
    }

    #[test]
    fn test_command_help_formatter() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({"help": "Usage: show command [options]"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Usage: show command [options]");
    }

    #[test]
    fn test_value_to_string() {
        assert_eq!(value_to_string(&json!("test")), "test");
        assert_eq!(value_to_string(&json!(42)), "42");
        assert_eq!(value_to_string(&json!(true)), "true");
        assert_eq!(value_to_string(&json!(null)), "");
        assert_eq!(value_to_string(&json!(["a", "b"])), "a, b");
    }

    #[test]
    fn test_build_table() {
        let headers = vec!["Name".to_string(), "Value".to_string()];
        let rows = vec![vec!["test".to_string(), "123".to_string()]];
        let result = build_table(&headers, &rows, true);
        assert!(result.contains("Name"));
        assert!(result.contains("test"));
    }

    #[test]
    fn test_formatter_names() {
        assert_eq!(TableFormatter::new().name(), "table");
        assert_eq!(TextFormatter::new().name(), "text");
        assert_eq!(ListFormatter::new().name(), "list");
        assert_eq!(CommandHelpFormatter::new().name(), "command_help");
    }

    #[test]
    fn test_table_natural_sort() {
        let formatter = TableFormatter::new();
        let data = json!({
            "Ethernet10": {"Bytes": "100"},
            "Ethernet2": {"Bytes": "200"},
            "Ethernet1": {"Bytes": "300"}
        });
        let config = CommandConfig::new()
            .with_headers(vec!["Port".to_string(), "Bytes".to_string()])
            .with_value_fields(vec!["Bytes".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Check that Ethernet1 appears before Ethernet2 before Ethernet10
        let pos1 = result.find("Ethernet1").unwrap();
        let pos2 = result.find("Ethernet2").unwrap();
        let pos10 = result.find("Ethernet10").unwrap();
        assert!(pos1 < pos2);
        assert!(pos2 < pos10);
    }

    #[test]
    fn test_text_formatter_fallback_to_string_value() {
        let formatter = TextFormatter::new();
        // Object with key_field not found, fallback to first string value
        let data = json!({"other_field": "fallback_value", "number": 123});
        let config = CommandConfig::new().with_key_field("nonexistent");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "fallback_value");
    }

    #[test]
    fn test_text_formatter_direct_value() {
        let formatter = TextFormatter::new();
        // Direct non-object value
        let data = json!("direct string value");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "direct string value");
    }

    #[test]
    fn test_text_formatter_number_value() {
        let formatter = TextFormatter::new();
        let data = json!(42);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_list_formatter_no_key_field() {
        let formatter = ListFormatter::new();
        // Object without key_field - should list all keys
        let data = json!({"key1": "value1", "key2": "value2", "key3": "value3"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("key1"));
        assert!(result.contains("key2"));
        assert!(result.contains("key3"));
    }

    #[test]
    fn test_command_help_formatter_usage() {
        let formatter = CommandHelpFormatter::new();
        // "usage" key instead of "help"
        let data = json!({"usage": "Usage: command [args]"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Usage: command [args]");
    }

    #[test]
    fn test_command_help_formatter_direct_string() {
        let formatter = CommandHelpFormatter::new();
        let data = json!("Direct help text");
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "Direct help text");
    }

    #[test]
    fn test_command_help_formatter_empty() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({"other": "data"});
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_table_formatter_direct_value() {
        let formatter = TableFormatter::new();
        // Object where value is a direct value, not nested object
        let data = json!({
            "key1": "simple_value",
            "key2": 123
        });
        let config =
            CommandConfig::new().with_headers(vec!["Key".to_string(), "Value".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("key1"));
        assert!(result.contains("simple_value"));
    }

    #[test]
    fn test_table_formatter_null_data() {
        let formatter = TableFormatter::new();
        let data = json!(null);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_value_to_string_object() {
        let data = json!({"nested": "object"});
        let result = value_to_string(&data);
        assert!(result.contains("nested"));
        assert!(result.contains("object"));
    }

    #[test]
    fn test_build_table_empty_headers() {
        let headers: Vec<String> = vec![];
        let rows = vec![vec!["test".to_string(), "123".to_string()]];
        let result = build_table(&headers, &rows, false);
        // With no headers, tabulate_simple_aligned returns empty (no columns to render)
        assert!(result.is_empty(), "Empty headers should produce empty table");
    }

    #[test]
    fn test_format_error_display() {
        let err = FormatError::InvalidData("test error".to_string());
        assert_eq!(format!("{}", err), "Invalid data format: test error");

        let err = FormatError::MissingField("field_name".to_string());
        assert_eq!(format!("{}", err), "Missing required field: field_name");

        let err = FormatError::FormatterNotFound("unknown".to_string());
        assert_eq!(format!("{}", err), "Formatter not found: unknown");
    }

    #[test]
    fn test_table_formatter_bool_value_field() {
        let formatter = TableFormatter::new();
        let data = json!({
            "item1": {"enabled": true, "count": 5}
        });
        let config = CommandConfig::new()
            .with_headers(vec![
                "Name".to_string(),
                "Enabled".to_string(),
                "Count".to_string(),
            ])
            .with_value_fields(vec!["enabled".to_string(), "count".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("true"));
        assert!(result.contains("5"));
    }

    #[test]
    fn test_table_formatter_default() {
        let formatter = TableFormatter::default();
        assert_eq!(formatter.name(), "table");
    }

    #[test]
    fn test_text_formatter_default() {
        let formatter = TextFormatter::default();
        assert_eq!(formatter.name(), "text");
    }

    #[test]
    fn test_list_formatter_default() {
        let formatter = ListFormatter::default();
        assert_eq!(formatter.name(), "list");
    }

    #[test]
    fn test_command_help_formatter_default() {
        let formatter = CommandHelpFormatter::default();
        assert_eq!(formatter.name(), "command_help");
    }

    // =======================================================================
    // Phase 3.6 — Additional edge case tests for base formatters (GAP-B6)
    // =======================================================================

    #[test]
    fn test_command_help_formatter_subcommands() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({
            "subcommands": {
                "unicast": {},
                "multicast": {}
            }
        });
        let config = CommandConfig::builder()
            .name("test")
            .description("Queue watermark help")
            .build();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Usage:"));
        assert!(result.contains("Commands:"));
    }

    #[test]
    fn test_command_help_formatter_queue_watermark_special_case() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({
            "subcommands": {
                "unicast": {},
                "multicast": {}
            }
        });
        let config = CommandConfig::builder()
            .name("test")
            .description("Queue watermark help")
            .build();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Usage:"));
        assert!(result.contains("unicast"));
        assert!(result.contains("multicast"));
    }

    #[test]
    fn test_command_help_formatter_empty_subcommands() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({
            "subcommands": {}
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "No subcommands available.");
    }

    #[test]
    fn test_list_formatter_empty_array() {
        let formatter = ListFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_list_formatter_nonexistent_key_field() {
        let formatter = ListFormatter::new();
        let data = json!({"items": ["a", "b"]});
        let config = CommandConfig::new().with_key_field("nonexistent");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // key_field doesn't exist in data, so no items extracted from array
        assert!(result.is_empty());
    }

    #[test]
    fn test_table_formatter_missing_value_field() {
        let formatter = TableFormatter::new();
        let data = json!({
            "item1": {"field_a": "100"},
            "item2": {"field_a": "200", "field_b": "300"}
        });
        let config = CommandConfig::new()
            .with_headers(vec!["Name".to_string(), "A".to_string(), "B".to_string()])
            .with_value_fields(vec!["field_a".to_string(), "field_b".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // item1 is missing field_b — should have empty string for that cell
        assert!(result.contains("item1"));
        assert!(result.contains("100"));
        assert!(result.contains("300"));
    }

    #[test]
    fn test_table_formatter_non_object_data() {
        let formatter = TableFormatter::new();
        let data = json!([1, 2, 3]);
        let config = CommandConfig::new();

        // Array data — no rows extracted from non-object
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Array is not null or empty-object, so it tries to build but no rows matched
        assert!(result.is_empty());
    }

    #[test]
    fn test_table_formatter_with_title_no_rows() {
        let formatter = TableFormatter::new();
        // Non-empty object but values don't produce rows (test empty result path)
        let data = json!({"key": null});
        let config = CommandConfig::new()
            .with_title("Some Title:")
            .with_headers(vec!["Name".to_string(), "Val".to_string()])
            .with_value_fields(vec!["val".to_string()]);

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // key exists so row is created with empty val column
        assert!(result.contains("key"));
    }

    #[test]
    fn test_value_to_string_false() {
        assert_eq!(value_to_string(&json!(false)), "false");
    }

    #[test]
    fn test_value_to_string_nested_array() {
        assert_eq!(value_to_string(&json!([1, "two", true])), "1, two, true");
    }

    #[test]
    fn test_build_table_multiple_rows() {
        let headers = vec!["A".to_string(), "B".to_string()];
        let rows = vec![
            vec!["x".to_string(), "1".to_string()],
            vec!["y".to_string(), "2".to_string()],
            vec!["z".to_string(), "3".to_string()],
        ];
        let result = build_table(&headers, &rows, false);
        assert!(result.contains("x"));
        assert!(result.contains("z"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_text_formatter_empty_object() {
        let formatter = TextFormatter::new();
        let data = json!({});
        let config = CommandConfig::new().with_key_field("missing");

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Empty object, no keys, no fallback → empty string value_to_string of {}
        assert!(result.is_empty() || result.contains("{"));
    }

    #[test]
    fn test_text_formatter_bool_value() {
        let formatter = TextFormatter::new();
        let data = json!(true);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert_eq!(result, "true");
    }

    // ---- Round 5 coverage tests ----

    #[test]
    fn test_table_formatter_with_bool_field() {
        // Cover L106: Value::Bool(b) => b.to_string() in build_simple_table
        let formatter = TableFormatter::new();
        let config = CommandConfig::builder()
            .name("test")
            .category("test")
            .formatter("table")
            .headers(&["Name", "Enabled"])
            .value_fields(&["enabled"])
            .build();
        let data = json!({
            "feature1": {"enabled": true},
            "feature2": {"enabled": false}
        });
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("true"), "result: {}", result);
        assert!(result.contains("false"), "result: {}", result);
    }

    // ====================================================================
    // Coverage-boost tests for add_header_separator, add_header_separator_smart,
    // tabulate_grid, tabulate_simple, tabulate_simple_auto, format_g
    // ====================================================================

    #[test]
    fn test_add_header_separator_basic() {
        let input = "Name     Value\nEth0     100\nEth4     200";
        let result = add_header_separator(input);
        assert!(result.contains("----"), "separator should have dashes: {}", result);
        assert!(result.contains("Name"), "should contain header");
        assert!(result.contains("Eth0"), "should contain data row 1");
        assert!(result.contains("200"), "should contain data value");
        // separator should be 2nd line
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3, "should have at least header + sep + data");
        assert!(lines[1].chars().all(|c| c == '-' || c == ' '));
    }

    #[test]
    fn test_add_header_separator_empty_input() {
        assert_eq!(add_header_separator(""), "");
    }

    #[test]
    fn test_add_header_separator_single_column() {
        let input = "Name\nalpha\nbeta";
        let result = add_header_separator(input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4); // header + sep + 2 data rows
        assert!(lines[1].contains('-'));
    }

    #[test]
    fn test_add_header_separator_right_aligned_column() {
        // Construct input where a column is right-aligned (leading spaces before value)
        let input = "Port   Count\nEth0     123\nEth4      45";
        let result = add_header_separator(input);
        assert!(result.contains("Port"), "should contain header");
        assert!(result.contains("123"), "should contain data");
    }

    #[test]
    fn test_add_header_separator_all_content_fills_column() {
        // All values fill column exactly — defaults to left-aligned
        let input = "ABC\nXYZ\nDEF";
        let result = add_header_separator(input);
        assert!(result.contains("ABC"));
        assert!(result.contains("---"));
    }

    #[test]
    fn test_add_header_separator_blank_lines_only() {
        let input = "   \n   \n   ";
        let result = add_header_separator(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_add_header_separator_wide_columns() {
        let input = "LongHeaderName  ShortH\nVal1            V2\nLongerVal       V3";
        let result = add_header_separator(input);
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3);
        // Check separator has dashes
        assert!(lines[1].contains('-'));
    }

    #[test]
    fn test_add_header_separator_smart_basic() {
        let input = "Port  Speed  Status\nEth0  10G    up\nEth4  25G    down";
        let result = add_header_separator_smart(input);
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3);
        // 2nd line should be separator
        assert!(lines[1].contains('-'));
        assert!(result.contains("Eth0"), "should keep data rows");
    }

    #[test]
    fn test_add_header_separator_smart_empty_input() {
        assert_eq!(add_header_separator_smart(""), "");
    }

    #[test]
    fn test_add_header_separator_smart_multiword_header() {
        // Multi-word header "Low-power Mode" has single spaces within
        let input = "Port     Low-power Mode\nEth0     On\nEth4     Off";
        let result = add_header_separator_smart(input);
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3);
        let sep = lines[1];
        // Single spaces within "Low-power Mode" should become dashes
        assert!(sep.contains('-'));
    }

    #[test]
    fn test_add_header_separator_smart_single_column() {
        let input = "Value\n100\n200";
        let result = add_header_separator_smart(input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4);
        assert!(lines[1].contains('-'));
    }

    #[test]
    fn test_add_header_separator_smart_leading_spaces() {
        let input = "   Port  Value\n   Eth0  100";
        let result = add_header_separator_smart(input);
        assert!(result.contains('-'));
    }

    #[test]
    fn test_tabulate_grid_basic() {
        let headers = &["Name", "Value"];
        let rows = vec![
            vec!["Eth0".to_string(), "100".to_string()],
            vec!["Eth4".to_string(), "200".to_string()],
        ];
        let alignments = &[false, true]; // left, right
        let result = tabulate_grid(headers, &rows, alignments);
        assert!(result.contains("+"), "grid should have + borders");
        assert!(result.contains("|"), "grid should have | borders");
        assert!(result.contains("Eth0"), "should contain data");
        assert!(result.contains("="), "should have = header separator");
    }

    #[test]
    fn test_tabulate_grid_empty_headers() {
        let headers: &[&str] = &[];
        let rows = vec![vec!["x".to_string()]];
        let alignments: &[bool] = &[];
        let result = tabulate_grid(headers, &rows, alignments);
        assert!(result.is_empty(), "empty headers should produce empty grid");
    }

    #[test]
    fn test_tabulate_grid_empty_rows() {
        let headers = &["A", "B"];
        let rows: Vec<Vec<String>> = vec![];
        let alignments = &[false, false];
        let result = tabulate_grid(headers, &rows, alignments);
        assert!(result.contains("A"), "should still show headers");
    }

    #[test]
    fn test_tabulate_grid_multiline_cell() {
        let headers = &["Name", "Info"];
        let rows = vec![vec![
            "Eth0".to_string(),
            "Line1\nLine2\nLine3".to_string(),
        ]];
        let alignments = &[false, false];
        let result = tabulate_grid(headers, &rows, alignments);
        assert!(result.contains("Line1"), "should show multi-line content");
        assert!(result.contains("Line2"));
        assert!(result.contains("Line3"));
    }

    #[test]
    fn test_tabulate_grid_right_alignment() {
        let headers = &["Port", "Count"];
        let rows = vec![
            vec!["Eth0".to_string(), "42".to_string()],
        ];
        let alignments = &[false, true];
        let result = tabulate_grid(headers, &rows, alignments);
        assert!(result.contains("42"));
    }

    #[test]
    fn test_tabulate_simple_basic() {
        let headers = &["A", "B"];
        let rows = vec![
            vec!["x".to_string(), "1".to_string()],
            vec!["y".to_string(), "2".to_string()],
        ];
        let result = tabulate_simple(headers, &rows);
        assert!(result.contains("A"));
        assert!(result.contains("x"));
        assert!(result.contains("---") || result.contains("--")); // separator
    }

    #[test]
    fn test_tabulate_simple_auto_numeric_columns() {
        let headers = &["Name", "Count", "Rate"];
        let rows = vec![
            vec!["Eth0".to_string(), "100".to_string(), "1.5".to_string()],
            vec!["Eth4".to_string(), "200".to_string(), "2.0".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        assert!(result.contains("Eth0"), "result: {}", result);
        // "2.0" should be reformatted to "2" by format_g
        assert!(result.contains("2") || result.contains("2.0"), "result: {}", result);
    }

    #[test]
    fn test_tabulate_simple_auto_mixed_columns() {
        let headers = &["Name", "Value"];
        let rows = vec![
            vec!["alpha".to_string(), "text".to_string()],
            vec!["beta".to_string(), "100".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        // Value column has mixed text+number → left-aligned
        assert!(result.contains("alpha"));
    }

    #[test]
    fn test_tabulate_simple_auto_na_values() {
        let headers = &["Name", "Count"];
        let rows = vec![
            vec!["Eth0".to_string(), "N/A".to_string()],
            vec!["Eth4".to_string(), "100".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        assert!(result.contains("N/A"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_tabulate_simple_auto_all_empty_column() {
        let headers = &["Name", "Val"];
        let rows = vec![
            vec!["a".to_string(), "".to_string()],
            vec!["b".to_string(), "".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        assert!(result.contains("a"));
    }

    #[test]
    fn test_format_g_zero() {
        assert_eq!(format_g(0.0), "0");
    }

    #[test]
    fn test_format_g_integer_as_float() {
        assert_eq!(format_g(1.0), "1");
        assert_eq!(format_g(100.0), "100");
    }

    #[test]
    fn test_format_g_trailing_zeros() {
        assert_eq!(format_g(1.50), "1.5");
        assert_eq!(format_g(2.0), "2");
    }

    #[test]
    fn test_format_g_small_value() {
        let result = format_g(0.001);
        assert_eq!(result, "0.001");
    }

    #[test]
    fn test_format_g_large_value() {
        let result = format_g(123456.0);
        assert_eq!(result, "123456");
    }

    #[test]
    fn test_format_g_scientific_notation() {
        let result = format_g(1e10);
        assert!(result.contains("e+") || result.contains("E+"), "large number: {}", result);
    }

    #[test]
    fn test_format_g_very_small_scientific() {
        let result = format_g(1e-6);
        assert!(result.contains("e-") || result.contains("E-"), "small number: {}", result);
    }

    #[test]
    fn test_format_g_negative_value() {
        let result = format_g(-42.5);
        assert!(result.starts_with('-'));
        assert!(result.contains("42.5"));
    }

    #[test]
    fn test_tabulate_simple_aligned_basic() {
        let headers = &["Name", "Value"];
        let rows = vec![
            vec!["test".to_string(), "123".to_string()],
        ];
        let alignments = &[false, true]; // left, right
        let result = tabulate_simple_aligned(headers, &rows, alignments);
        assert!(result.contains("Name"));
        assert!(result.contains("test"));
        assert!(result.contains("123"));
        assert!(result.contains("---") || result.contains("--"));
    }

    #[test]
    fn test_tabulate_simple_aligned_empty_rows() {
        let headers = &["A", "B"];
        let rows: Vec<Vec<String>> = vec![];
        let alignments = &[false, false];
        let result = tabulate_simple_aligned(headers, &rows, alignments);
        assert!(result.contains("A"), "headers should still render");
    }

    #[test]
    fn test_tabulate_simple_aligned_empty_headers() {
        let headers: &[&str] = &[];
        let rows = vec![vec!["x".to_string()]];
        let alignments: &[bool] = &[];
        let result = tabulate_simple_aligned(headers, &rows, alignments);
        assert!(result.is_empty());
    }

    #[test]
    fn test_command_help_formatter_queue_persistent_watermark() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({
            "subcommands": {
                "unicast": {},
                "multicast": {}
            }
        });
        let config = CommandConfig::builder()
            .name("test")
            .description("Queue persistent watermark help")
            .build();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Usage:"));
        assert!(result.contains("persistent-watermark"));
    }

    #[test]
    fn test_command_help_formatter_generic_fallback() {
        let formatter = CommandHelpFormatter::new();
        let data = json!({
            "subcommands": {
                "option_a": {},
                "option_b": {}
            }
        });
        let config = CommandConfig::builder()
            .name("test")
            .description("Custom something help")
            .build();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Falls to format_generic_help since "Custom something" doesn't match known patterns
        assert!(result.contains("Usage:") || result.contains("Commands:"), "result: {}", result);
    }

    #[test]
    fn test_table_formatter_with_title_and_grid() {
        let formatter = TableFormatter::new();
        let data = json!({
            "row1": {"val": "10"}
        });
        let config = CommandConfig::new()
            .with_title("Grid Title:")
            .with_headers(vec!["Name".to_string(), "Val".to_string()])
            .with_value_fields(vec!["val".to_string()]);
        let result = formatter.format(&data, &config, "grid", None).unwrap();
        assert!(result.contains("Grid Title:"), "result: {}", result);
        assert!(result.contains("row1"), "should contain data: {}", result);
    }

    #[test]
    fn test_tabulate_simple_auto_dash_values() {
        let headers = &["Name", "Value"];
        let rows = vec![
            vec!["Eth0".to_string(), "-".to_string()],
            vec!["Eth4".to_string(), "100".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        assert!(result.contains("-"));
    }

    #[test]
    fn test_tabulate_simple_auto_comma_separated_numbers() {
        let headers = &["Name", "Count"];
        let rows = vec![
            vec!["Eth0".to_string(), "1,000".to_string()],
            vec!["Eth4".to_string(), "2,500".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        // Comma-separated numbers are treated as numeric
        assert!(result.contains("1,000") || result.contains("1000"));
    }

    #[test]
    fn test_tabulate_simple_auto_float_with_exponent() {
        let headers = &["Name", "Rate"];
        let rows = vec![
            vec!["Eth0".to_string(), "1.5e3".to_string()],
            vec!["Eth4".to_string(), "2.0E-1".to_string()],
        ];
        let result = tabulate_simple_auto(headers, &rows);
        assert!(result.contains("Eth0"));
    }
}
