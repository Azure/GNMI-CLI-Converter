//! Queue-related formatters for priority groups, queue counters, etc.

use crate::common::types::CommandConfig;
use crate::formatters::base::{
    tabulate_simple, tabulate_simple_aligned, tabulate_simple_auto, value_to_string, FormatError, OutputFormatter,
};
use crate::utils::parsing::parse_queue_name;
use crate::utils::sorting::natural_cmp;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::Value;
use std::collections::HashMap;

/// Formatter for queue counters output.
pub struct QueueCountersFormatter;

impl QueueCountersFormatter {
    /// Creates a new [`QueueCountersFormatter`].
    pub fn new() -> Self {
        Self
    }

    /// Convert queue number to TxQ name (UC0-UC7, MC8+).
    fn queue_num_to_txq(q_num: i64) -> String {
        if q_num < 8 {
            format!("UC{}", q_num)
        } else {
            format!("MC{}", q_num)
        }
    }
}

impl Default for QueueCountersFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for QueueCountersFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let options = _options.cloned().unwrap_or_default();
        let nonzero_filter = options.get("nonzero").is_some_and(|v| v == "true");

        let counter_columns = vec!["Counter/pkts", "Counter/bytes", "Drop/pkts", "Drop/bytes"];
        let trim_columns = vec!["Trim/pkts", "TrimSent/pkts", "TrimDrop/pkts"];

        // Handle Array input
        if let Value::Array(arr) = data {
            if arr.is_empty() {
                // Return header row for empty array
                let headers = [
                    "Port",
                    "TxQ",
                    "Counter/pkts",
                    "Counter/bytes",
                    "Drop/pkts",
                    "Drop/bytes",
                ];
                // Use h.len() + 2 minimum width to match tabulate padding
                let widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
                let header_line: String = headers
                    .iter()
                    .enumerate()
                    .map(|(i, h)| format!("{:>width$}", h, width = widths[i]))
                    .collect::<Vec<_>>()
                    .join("  ");
                let sep_line: String = widths
                    .iter()
                    .map(|w| "-".repeat(*w))
                    .collect::<Vec<_>>()
                    .join("  ");
                return Ok(format!("For namespace :\n{}\n{}", header_line, sep_line));
            }

            // Collect all column keys from array items (case-insensitive matching)
            let counter_col_lower: Vec<String> =
                counter_columns.iter().map(|c| c.to_lowercase()).collect();
            let trim_col_lower: Vec<String> =
                trim_columns.iter().map(|c| c.to_lowercase()).collect();

            let mut rows: Vec<Vec<String>> = Vec::new();
            let mut has_counter = false;
            let mut has_trim = false;

            for item in arr {
                if let Value::Object(obj) = item {
                    // Detect column presence
                    for key in obj.keys() {
                        let lower = key.to_lowercase();
                        if counter_col_lower.contains(&lower) {
                            has_counter = true;
                        }
                        if trim_col_lower.contains(&lower) {
                            has_trim = true;
                        }
                    }

                    // Extract all values in order
                    let mut row = Vec::new();
                    for col in &counter_columns {
                        let val = obj
                            .get(*col)
                            .or_else(|| obj.get(&col.to_lowercase()))
                            .map(value_to_string)
                            .unwrap_or_default();
                        row.push(val);
                    }
                    for col in &trim_columns {
                        let val = obj
                            .get(*col)
                            .or_else(|| obj.get(&col.to_lowercase()))
                            .map(value_to_string)
                            .unwrap_or_default();
                        row.push(val);
                    }
                    rows.push(row);
                }
            }

            // Build headers
            let mut headers: Vec<&str> = Vec::new();
            if has_counter {
                headers.extend_from_slice(&counter_columns);
            }
            if has_trim {
                headers.extend_from_slice(&trim_columns);
            }
            if headers.is_empty() {
                headers.extend_from_slice(&counter_columns);
            }

            // Calculate widths: max(header_len + 2, max_data_len) to match tabulate
            let mut widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
            for row in &rows {
                for (i, val) in row.iter().enumerate() {
                    if i < widths.len() && val.len() > widths[i] {
                        widths[i] = val.len();
                    }
                }
            }

            let header_line: String = headers
                .iter()
                .enumerate()
                .map(|(i, h)| format!("{:>width$}", h, width = widths[i]))
                .collect::<Vec<_>>()
                .join("  ");
            let sep_line: String = widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<_>>()
                .join("  ");

            let mut lines = vec!["For namespace :".to_string(), header_line, sep_line];
            for row in &rows {
                let vals: Vec<String> = row
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i < widths.len())
                    .map(|(i, val)| format!("{:>width$}", val, width = widths[i]))
                    .collect();
                lines.push(vals.join("  "));
            }

            return Ok(lines.join("\n"));
        }

        let map = match data {
            Value::Object(m) => m,
            _ => return Ok(String::new()),
        };

        if map.is_empty() {
            return Ok(String::new());
        }

        // Detect which column sets are present
        let mut has_counter = false;
        let mut has_trim = false;
        for (_key, val) in map.iter() {
            if let Value::Object(q) = val {
                for col in &counter_columns {
                    if q.contains_key(*col) {
                        has_counter = true;
                    }
                }
                for col in &trim_columns {
                    if q.contains_key(*col) {
                        has_trim = true;
                    }
                }
                if has_counter && has_trim {
                    break;
                }
            }
        }

        // Group queues by interface: "EthernetX:Y" -> port=EthernetX, q_num=Y
        let mut interfaces: HashMap<String, Vec<(i64, String, &Value)>> = HashMap::new();
        for (key, val) in map.iter() {
            let (port, q_num) = if let Some(pos) = key.rfind(':') {
                let port_name = &key[..pos];
                let q_str = &key[pos + 1..];
                let q = q_str.parse::<i64>().unwrap_or(0);
                (port_name.to_string(), q)
            } else {
                (key.clone(), 0)
            };

            interfaces
                .entry(port)
                .or_default()
                .push((q_num, key.clone(), val));
        }

        // Build headers based on what columns are present
        let mut headers: Vec<&str> = vec!["Port", "TxQ"];
        if has_counter {
            headers.extend_from_slice(&counter_columns);
        }
        if has_trim {
            headers.extend_from_slice(&trim_columns);
        }

        // Build output with one paragraph per interface
        let mut paragraphs: Vec<String> = Vec::new();

        let mut sorted_ports: Vec<&String> = interfaces.keys().collect();
        sorted_ports.sort_by(|a, b| natural_cmp(a, b));

        for port in sorted_ports {
            let queues = interfaces.get(port.as_str()).unwrap();
            let mut sorted_queues = queues.clone();
            sorted_queues.sort_by_key(|(q_num, _, _)| *q_num);

            let mut table_rows: Vec<Vec<String>> = Vec::new();
            for (q_num, _key, val) in &sorted_queues {
                if let Value::Object(q) = val {
                    // For --nonzero, skip if queue data is empty
                    if nonzero_filter && q.is_empty() {
                        continue;
                    }

                    let txq = Self::queue_num_to_txq(*q_num);
                    let mut row = vec![port.to_string(), txq];

                    if has_counter {
                        for col in &counter_columns {
                            let v = q
                                .get(*col)
                                .map(value_to_string)
                                .unwrap_or_else(|| "0".to_string());
                            row.push(v);
                        }
                    }
                    if has_trim {
                        for col in &trim_columns {
                            let v = q
                                .get(*col)
                                .map(value_to_string)
                                .unwrap_or_else(|| "0".to_string());
                            row.push(v);
                        }
                    }
                    table_rows.push(row);
                }
            }

            if table_rows.is_empty() {
                continue;
            }

            // Build tabulate "simple" format table with right-aligned columns
            // Use h.len() + 2 minimum width to match tabulate padding
            let mut widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
            for row in &table_rows {
                for (i, val) in row.iter().enumerate() {
                    if i < widths.len() && val.len() > widths[i] {
                        widths[i] = val.len();
                    }
                }
            }

            // Header line (right-aligned)
            let header_line: String = headers
                .iter()
                .enumerate()
                .map(|(i, h)| format!("{:>width$}", h, width = widths[i]))
                .collect::<Vec<_>>()
                .join("  ");

            // Separator line
            let sep_line: String = widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<_>>()
                .join("  ");

            // Data rows (right-aligned)
            let mut lines = vec!["For namespace :".to_string(), header_line, sep_line];
            for row in &table_rows {
                let line: String = row
                    .iter()
                    .enumerate()
                    .map(|(i, val)| {
                        let w = widths[i];
                        format!("{:>width$}", val, width = w)
                    })
                    .collect::<Vec<_>>()
                    .join("  ");
                lines.push(line);
            }

            paragraphs.push(lines.join("\n"));
        }

        if paragraphs.is_empty() {
            return Ok(String::new());
        }

        Ok(paragraphs.join("\n\n"))
    }

    fn name(&self) -> &'static str {
        "queue_counters"
    }
}

/// Formatter for priority group counters output.
pub struct PriorityGroupFormatter;

impl PriorityGroupFormatter {
    /// Creates a new [`PriorityGroupFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for PriorityGroupFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for PriorityGroupFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let pg_type = options
            .and_then(|o| o.get("type"))
            .map(|s| s.as_str())
            .unwrap_or("watermark");

        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Disabled);
        table.load_preset("                   ");

        // Set headers based on type
        let headers = match pg_type {
            "watermark" | "persistent-watermark" => {
                vec![
                    "Port", "PG0", "PG1", "PG2", "PG3", "PG4", "PG5", "PG6", "PG7",
                ]
            }
            "counters" => {
                vec![
                    "Port",
                    "PG",
                    "Ingress Packets",
                    "Ingress Bytes",
                    "Drop Packets",
                    "Drop Bytes",
                ]
            }
            _ => {
                vec![
                    "Port", "PG0", "PG1", "PG2", "PG3", "PG4", "PG5", "PG6", "PG7",
                ]
            }
        };

        let header_cells: Vec<Cell> = headers
            .iter()
            .map(|h| Cell::new(*h).set_alignment(CellAlignment::Right))
            .collect();
        table.set_header(header_cells);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let mut row = Vec::new();

                    for (i, h) in headers.iter().enumerate() {
                        let val = obj
                            .get(*h)
                            .or_else(|| obj.get(&h.to_lowercase()))
                            .map(value_to_string)
                            .unwrap_or_default();
                        if i == 0 {
                            row.push(Cell::new(&val).set_alignment(CellAlignment::Left));
                        } else {
                            row.push(Cell::new(&val).set_alignment(CellAlignment::Right));
                        }
                    }

                    table.add_row(row);
                }
            }
        } else if let Value::Object(map) = data {
            // Data keyed by interface
            let mut ports: Vec<&String> = map.keys().collect();
            ports.sort_by(|a, b| natural_cmp(a, b));

            for port in ports {
                if let Some(pg_data) = map.get(port) {
                    match pg_type {
                        "watermark" | "persistent-watermark" => {
                            // Format as row with PG0-PG7 columns
                            let mut row = vec![Cell::new(port).set_alignment(CellAlignment::Left)];

                            if let Value::Object(pgs) = pg_data {
                                for i in 0..8 {
                                    let pg_key = format!("PG{}", i);
                                    let val = pgs
                                        .get(&pg_key)
                                        .or_else(|| pgs.get(&format!("pg{}", i)))
                                        .map(value_to_string)
                                        .unwrap_or_else(|| "N/A".to_string());
                                    row.push(Cell::new(&val).set_alignment(CellAlignment::Right));
                                }
                            } else if let Value::Array(pgs) = pg_data {
                                for val in pgs.iter().take(8) {
                                    row.push(
                                        Cell::new(value_to_string(val))
                                            .set_alignment(CellAlignment::Right),
                                    );
                                }
                                // Fill remaining columns
                                for _ in pgs.len()..8 {
                                    row.push(Cell::new("N/A").set_alignment(CellAlignment::Right));
                                }
                            }

                            table.add_row(row);
                        }
                        "counters" => {
                            // Format as multiple rows per interface (one per PG)
                            if let Value::Object(pgs) = pg_data {
                                let mut pg_keys: Vec<&String> = pgs.keys().collect();
                                pg_keys.sort_by(|a, b| {
                                    let a_parsed = parse_queue_name(a);
                                    let b_parsed = parse_queue_name(b);
                                    match (a_parsed, b_parsed) {
                                        (Some((_, a_type, _)), Some((_, b_type, _))) => {
                                            a_type.cmp(&b_type)
                                        }
                                        (Some(_), None) => std::cmp::Ordering::Less,
                                        (None, Some(_)) => std::cmp::Ordering::Greater,
                                        (None, None) => a.cmp(b),
                                    }
                                });

                                for pg_key in pg_keys {
                                    if let Some(Value::Object(counters)) = pgs.get(pg_key) {
                                        let in_pkts = counters
                                            .get("ingress_packets")
                                            .or_else(|| counters.get("Ingress Packets"))
                                            .map(value_to_string)
                                            .unwrap_or_default();
                                        let in_bytes = counters
                                            .get("ingress_bytes")
                                            .or_else(|| counters.get("Ingress Bytes"))
                                            .map(value_to_string)
                                            .unwrap_or_default();
                                        let drop_pkts = counters
                                            .get("drop_packets")
                                            .or_else(|| counters.get("Drop Packets"))
                                            .map(value_to_string)
                                            .unwrap_or_default();
                                        let drop_bytes = counters
                                            .get("drop_bytes")
                                            .or_else(|| counters.get("Drop Bytes"))
                                            .map(value_to_string)
                                            .unwrap_or_default();

                                        table.add_row(vec![
                                            Cell::new(port).set_alignment(CellAlignment::Left),
                                            Cell::new(pg_key).set_alignment(CellAlignment::Left),
                                            Cell::new(&in_pkts).set_alignment(CellAlignment::Right),
                                            Cell::new(&in_bytes)
                                                .set_alignment(CellAlignment::Right),
                                            Cell::new(&drop_pkts)
                                                .set_alignment(CellAlignment::Right),
                                            Cell::new(&drop_bytes)
                                                .set_alignment(CellAlignment::Right),
                                        ]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "priority_group"
    }
}

/// Formatter for multicast queue counters output.
pub struct MulticastQueueFormatter;

impl MulticastQueueFormatter {
    /// Creates a new [`MulticastQueueFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for MulticastQueueFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for MulticastQueueFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers: &[&str] = &["Port", "MC0", "MC1", "MC2", "MC3", "MC4", "MC5", "MC6", "MC7"];
        let mut rows: Vec<Vec<String>> = Vec::new();

        if let Value::Object(map) = data {
            let mut ports: Vec<&String> = map.keys().collect();
            ports.sort_by(|a, b| natural_cmp(a, b));

            for port in ports {
                if let Some(mc_data) = map.get(port) {
                    let mut row = vec![port.to_string()];

                    if let Value::Object(mcs) = mc_data {
                        for i in 0..8 {
                            let mc_key = format!("MC{}", i);
                            let val = mcs
                                .get(&mc_key)
                                .or_else(|| mcs.get(&format!("mc{}", i)))
                                .map(value_to_string)
                                .unwrap_or_else(|| "N/A".to_string());
                            row.push(val);
                        }
                    } else if let Value::Array(mcs) = mc_data {
                        for val in mcs.iter().take(8) {
                            row.push(value_to_string(val));
                        }
                        for _ in mcs.len()..8 {
                            row.push("N/A".to_string());
                        }
                    }

                    rows.push(row);
                }
            }
        } else if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let port = obj
                        .get("port")
                        .or_else(|| obj.get("Port"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let mut row = vec![port];

                    for i in 0..8 {
                        let mc_key = format!("MC{}", i);
                        let val = obj
                            .get(&mc_key)
                            .or_else(|| obj.get(&format!("mc{}", i)))
                            .map(value_to_string)
                            .unwrap_or_default();
                        row.push(val);
                    }

                    rows.push(row);
                }
            }
        }

        Ok(tabulate_simple_auto(headers, &rows))
    }

    fn name(&self) -> &'static str {
        "multicast_queue"
    }
}

/// Formatter for shared headroom pool output.
pub struct SharedHeadroomPoolFormatter;

impl SharedHeadroomPoolFormatter {
    /// Creates a new [`SharedHeadroomPoolFormatter`].
    pub fn new() -> Self {
        Self
    }
}

impl Default for SharedHeadroomPoolFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for SharedHeadroomPoolFormatter {
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
            Cell::new("Pool").set_alignment(CellAlignment::Left),
            Cell::new("Current").set_alignment(CellAlignment::Right),
            Cell::new("Watermark").set_alignment(CellAlignment::Right),
        ]);

        if let Value::Array(arr) = data {
            for item in arr {
                if let Value::Object(obj) = item {
                    let pool = obj
                        .get("pool")
                        .or_else(|| obj.get("Pool"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let current = obj
                        .get("current")
                        .or_else(|| obj.get("Current"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let watermark = obj
                        .get("watermark")
                        .or_else(|| obj.get("Watermark"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(&pool).set_alignment(CellAlignment::Left),
                        Cell::new(&current).set_alignment(CellAlignment::Right),
                        Cell::new(&watermark).set_alignment(CellAlignment::Right),
                    ]);
                }
            }
        } else if let Value::Object(map) = data {
            for (pool, pool_data) in map {
                if let Value::Object(obj) = pool_data {
                    let current = obj
                        .get("current")
                        .or_else(|| obj.get("Current"))
                        .map(value_to_string)
                        .unwrap_or_default();
                    let watermark = obj
                        .get("watermark")
                        .or_else(|| obj.get("Watermark"))
                        .map(value_to_string)
                        .unwrap_or_default();

                    table.add_row(vec![
                        Cell::new(pool).set_alignment(CellAlignment::Left),
                        Cell::new(&current).set_alignment(CellAlignment::Right),
                        Cell::new(&watermark).set_alignment(CellAlignment::Right),
                    ]);
                }
            }
        }

        Ok(table.to_string())
    }

    fn name(&self) -> &'static str {
        "shared_headroom_pool"
    }
}

/// Formatter for WRED counters.
pub struct WredCountersFormatter;

impl WredCountersFormatter {
    /// Creates a new [`WredCountersFormatter`].
    pub fn new() -> Self {
        Self
    }

    fn queue_num_to_txq(q_num: i64) -> String {
        if q_num < 8 {
            format!("UC{}", q_num)
        } else {
            format!("MC{}", q_num)
        }
    }
}

impl Default for WredCountersFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for WredCountersFormatter {
    fn format(
        &self,
        data: &Value,
        _config: &CommandConfig,
        _table_format: &str,
        _options: Option<&HashMap<String, String>>,
    ) -> Result<String, FormatError> {
        let headers: &[&str] = &[
            "Port",
            "TxQ",
            "WredDrp/pkts",
            "WredDrp/bytes",
            "EcnMarked/pkts",
            "EcnMarked/bytes",
        ];

        let map = match data {
            Value::Object(m) => m,
            _ => return Ok(tabulate_simple(headers, &[])),
        };

        if map.is_empty() {
            return Ok(tabulate_simple(headers, &[]));
        }

        // Logic to parse keys "EthernetX:Y" -> port="EthernetX", q_num=Y
        let mut rows: Vec<Vec<String>> = Vec::new();

        for (key, val) in map {
            if let Value::Object(counters) = val {
                let (port, q_str) = match key.rsplit_once(':') {
                    Some(pair) => pair,
                    None => (key.as_str(), "0"), // Default if no colon
                };

                let q_num = q_str.parse::<i64>().unwrap_or(0);
                let txq = Self::queue_num_to_txq(q_num);

                let wred_drop_pkts = counters
                    .get("WredDrp/pkts")
                    .map(value_to_string)
                    .unwrap_or_else(|| "N/A".to_string());
                let wred_drop_bytes = counters
                    .get("WredDrp/bytes")
                    .map(value_to_string)
                    .unwrap_or_else(|| "N/A".to_string());
                let ecn_marked_pkts = counters
                    .get("EcnMarked/pkts")
                    .map(value_to_string)
                    .unwrap_or_else(|| "N/A".to_string());
                let ecn_marked_bytes = counters
                    .get("EcnMarked/bytes")
                    .map(value_to_string)
                    .unwrap_or_else(|| "N/A".to_string());

                rows.push(vec![
                    port.to_string(),
                    txq,
                    wred_drop_pkts,
                    wred_drop_bytes,
                    ecn_marked_pkts,
                    ecn_marked_bytes,
                ]);
            }
        }

        // Sort by port then queue; UC queues first (UC0-UC9), then MC queues (MC8-MC11)
        rows.sort_by(|a, b| {
            match natural_cmp(&a[0], &b[0]) {
                std::cmp::Ordering::Equal => {
                    // UC before MC: UC prefix = 0, MC prefix = 1
                    let a_is_mc = a[1].starts_with("MC");
                    let b_is_mc = b[1].starts_with("MC");
                    match (a_is_mc, b_is_mc) {
                        (false, true) => std::cmp::Ordering::Less,   // UC < MC
                        (true, false) => std::cmp::Ordering::Greater, // MC > UC
                        _ => natural_cmp(&a[1], &b[1]),               // same prefix: sort by number
                    }
                }
                ord => ord,
            }
        });

        // Group rows by port (column 0) — matching per-interface grouping
        let all_right = vec![true; headers.len()]; // all columns right-aligned
        let mut paragraphs: Vec<String> = Vec::new();
        let mut current_port: Option<&str> = None;
        let mut group_rows: Vec<Vec<String>> = Vec::new();

        for row in &rows {
            let port = row[0].as_str();
            if current_port.is_some() && current_port != Some(port) {
                // Flush previous group
                paragraphs.push(tabulate_simple_aligned(headers, &group_rows, &all_right));
                group_rows.clear();
            }
            current_port = Some(port);
            group_rows.push(row.clone());
        }
        // Flush last group
        if !group_rows.is_empty() {
            paragraphs.push(tabulate_simple_aligned(headers, &group_rows, &all_right));
        }

        // If all entries were non-object values and no rows were produced,
        // return headers-only table (consistent with empty-map behavior)
        if paragraphs.is_empty() {
            return Ok(tabulate_simple(headers, &[]));
        }

        Ok(paragraphs.join("\n\n"))
    }

    fn name(&self) -> &'static str {
        "wred_counters"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_queue_counters_formatter_array() {
        let formatter = QueueCountersFormatter::new();
        let data = json!([
            {
                "Counter/pkts": "Ethernet0",
                "Counter/bytes": "1000",
                "Drop/pkts": "0",
                "Drop/bytes": "0",
                "Trim/pkts": "0",
                "TrimSent/pkts": "0",
                "TrimDrop/pkts": "0",
                "WredDrp/pkts": "0",
                "WredDrp/bytes": "0",
                "EcnMarked/pkts": "0",
                "EcnMarked/bytes": "0"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Queue counters formatter has column headers including Counter/pkts etc.
        // The data array should produce rows with these values
        assert!(result.contains("1000"), "Result was: {}", result);
    }

    #[test]
    fn test_queue_counters_formatter_dict() {
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Counter/bytes": "1000",
                "Drop/pkts": "50"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Dict format uses keys as first column (port name)
        assert!(result.contains("1000"), "Result was: {}", result);
    }

    #[test]
    fn test_priority_group_formatter_watermark() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": "100",
                "PG1": "200",
                "PG2": "0",
                "PG3": "0",
                "PG4": "0",
                "PG5": "0",
                "PG6": "0",
                "PG7": "0"
            }
        });
        let config = CommandConfig::new();

        let mut options = HashMap::new();
        options.insert("type".to_string(), "watermark".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("100"));
        assert!(result.contains("200"));
    }

    #[test]
    fn test_priority_group_formatter_counters() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": {
                    "ingress_packets": "1000",
                    "ingress_bytes": "64000",
                    "drop_packets": "0",
                    "drop_bytes": "0"
                }
            }
        });
        let config = CommandConfig::new();

        let mut options = HashMap::new();
        options.insert("type".to_string(), "counters".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("PG0"));
        assert!(result.contains("1000"));
    }

    #[test]
    fn test_multicast_queue_formatter() {
        let formatter = MulticastQueueFormatter::new();
        let data = json!({
            "Ethernet0": {
                "MC0": "100",
                "MC1": "200",
                "MC2": "0",
                "MC3": "0",
                "MC4": "0",
                "MC5": "0",
                "MC6": "0",
                "MC7": "0"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_shared_headroom_pool_formatter() {
        let formatter = SharedHeadroomPoolFormatter::new();
        let data = json!([
            {
                "pool": "ingress_lossless_pool",
                "current": "1024",
                "watermark": "2048"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ingress_lossless_pool"));
        assert!(result.contains("1024"));
        assert!(result.contains("2048"));
    }

    #[test]
    fn test_shared_headroom_pool_dict() {
        let formatter = SharedHeadroomPoolFormatter::new();
        let data = json!({
            "ingress_lossless_pool": {
                "current": "1024",
                "watermark": "2048"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("ingress_lossless_pool"));
    }

    #[test]
    fn test_priority_group_formatter_persistent_watermark() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": "150",
                "PG1": "250"
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "persistent-watermark".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("150"));
    }

    #[test]
    fn test_priority_group_formatter_array_input() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!([
            {
                "Port": "Ethernet0",
                "PG0": "100",
                "PG1": "200",
                "PG2": "0",
                "PG3": "0",
                "PG4": "0",
                "PG5": "0",
                "PG6": "0",
                "PG7": "0"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("100"));
        assert!(result.contains("200"));
    }

    #[test]
    fn test_priority_group_formatter_array_data() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": ["10", "20", "30", "40", "50", "60", "70", "80"]
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "watermark".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("10"));
    }

    #[test]
    fn test_priority_group_formatter_array_data_short() {
        let formatter = PriorityGroupFormatter::new();
        // Less than 8 PGs - should fill with N/A
        let data = json!({
            "Ethernet0": ["10", "20", "30"]
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "watermark".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("N/A"));
    }

    #[test]
    fn test_multicast_queue_formatter_array_input() {
        let formatter = MulticastQueueFormatter::new();
        let data = json!([
            {
                "Port": "Ethernet0",
                "MC0": "100",
                "MC1": "200"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_multicast_queue_formatter_array_data() {
        let formatter = MulticastQueueFormatter::new();
        let data = json!({
            "Ethernet0": ["10", "20", "30", "40", "50", "60", "70", "80"]
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("10"));
    }

    #[test]
    fn test_multicast_queue_formatter_array_data_short() {
        let formatter = MulticastQueueFormatter::new();
        // Less than 8 MCs - should fill with N/A
        let data = json!({
            "Ethernet0": ["10", "20"]
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("N/A"));
    }

    #[test]
    fn test_priority_group_counters_alternate_keys() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": {
                    "Ingress Packets": "5000",
                    "Ingress Bytes": "320000",
                    "Drop Packets": "10",
                    "Drop Bytes": "640"
                }
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "counters".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("5000"));
        assert!(result.contains("10"));
    }

    #[test]
    fn test_priority_group_unknown_type() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": "100"
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "unknown_type".to_string());

        // Unknown type defaults to watermark style headers
        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Port"));
    }

    #[test]
    fn test_queue_counters_dict_nested() {
        let formatter = QueueCountersFormatter::new();
        // Dict with port -> queues -> counters
        let data = json!({
            "Ethernet0": {
                "UC0": {"Counter/pkts": "1000", "Counter/bytes": "64000"},
                "UC1": {"Counter/pkts": "500", "Counter/bytes": "32000"}
            }
        });
        let config = CommandConfig::new();
        let _ = formatter.format(&data, &config, "simple", None);
    }

    #[test]
    fn test_priority_group_no_options() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": "100",
                "PG1": "200"
            }
        });
        let config = CommandConfig::new();
        // No options - should default to watermark
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_queue_counters_empty_array() {
        let formatter = QueueCountersFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Returns header only
        assert!(result.contains("Counter"));
    }

    #[test]
    fn test_multicast_queue_empty_dict() {
        let formatter = MulticastQueueFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Returns header only
        assert!(result.contains("Port") || result.contains("MC0"));
    }

    #[test]
    fn test_shared_headroom_pool_empty() {
        let formatter = SharedHeadroomPoolFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Returns header only
        assert!(result.contains("Pool") || result.contains("Current"));
    }

    // Additional tests for uncovered branches

    #[test]
    fn test_queue_counters_formatter_nested_dict() {
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0": {
                "queues": {
                    "UC0": {"totalpacket": "1000", "totalbytes": "64000"},
                    "UC1": {"totalpacket": "500", "totalbytes": "32000"}
                }
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0") || result.contains("UC0"));
    }

    #[test]
    fn test_multicast_queue_formatter_array() {
        let formatter = MulticastQueueFormatter::new();
        let data = json!([
            {"port": "Ethernet0", "mc": "MC0", "totalpacket": "100", "totalbytes": "6400"},
            {"port": "Ethernet0", "mc": "MC1", "totalpacket": "50", "totalbytes": "3200"}
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0") || result.contains("MC0"));
    }

    // =====================================================================
    // NEW TESTS FOR WRED COUNTERS FORMATTER (previously untested)
    // =====================================================================

    #[test]
    fn test_wred_counters_formatter_basic() {
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:0": {
                "WredDrp/pkts": "100",
                "WredDrp/bytes": "6400",
                "EcnMarked/pkts": "50",
                "EcnMarked/bytes": "3200"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
        assert!(
            result.contains("UC0"),
            "Should contain UC0 for queue 0, result: {}",
            result
        );
        assert!(result.contains("100"), "Result was: {}", result);
    }

    #[test]
    fn test_wred_counters_formatter_mc_queue() {
        // Queue number >= 8 should produce MC queue names
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:8": {
                "WredDrp/pkts": "200",
                "WredDrp/bytes": "12800",
                "EcnMarked/pkts": "25",
                "EcnMarked/bytes": "1600"
            },
            "Ethernet0:9": {
                "WredDrp/pkts": "300",
                "WredDrp/bytes": "19200",
                "EcnMarked/pkts": "15",
                "EcnMarked/bytes": "960"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("MC8"),
            "Should contain MC8 for queue 8, result: {}",
            result
        );
        assert!(
            result.contains("MC9"),
            "Should contain MC9 for queue 9, result: {}",
            result
        );
    }

    #[test]
    fn test_wred_counters_formatter_no_colon_key() {
        // Key without colon should default to queue 0
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0": {
                "WredDrp/pkts": "500",
                "WredDrp/bytes": "32000",
                "EcnMarked/pkts": "100",
                "EcnMarked/bytes": "6400"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("UC0"),
            "Should default to UC0 when no colon, result: {}",
            result
        );
    }

    #[test]
    fn test_wred_counters_formatter_empty_map() {
        let formatter = WredCountersFormatter::new();
        let data = json!({});
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Port") || result.contains("TxQ"));
    }

    #[test]
    fn test_wred_counters_formatter_non_object_data() {
        let formatter = WredCountersFormatter::new();
        // Data is not an object - should return table with headers only
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Port") || result.contains("TxQ"));
    }

    #[test]
    fn test_wred_counters_formatter_missing_fields() {
        // Some counter fields missing should show N/A
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:0": {
                "WredDrp/pkts": "100"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("N/A"),
            "Missing fields should show N/A, result: {}",
            result
        );
    }

    #[test]
    fn test_wred_counters_formatter_multiple_ports() {
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:0": {
                "WredDrp/pkts": "100",
                "WredDrp/bytes": "6400",
                "EcnMarked/pkts": "50",
                "EcnMarked/bytes": "3200"
            },
            "Ethernet1:0": {
                "WredDrp/pkts": "200",
                "WredDrp/bytes": "12800",
                "EcnMarked/pkts": "75",
                "EcnMarked/bytes": "4800"
            },
            "Ethernet0:1": {
                "WredDrp/pkts": "150",
                "WredDrp/bytes": "9600",
                "EcnMarked/pkts": "60",
                "EcnMarked/bytes": "3840"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
        assert!(result.contains("Ethernet1"), "Result was: {}", result);
    }

    #[test]
    fn test_wred_counters_formatter_invalid_queue_number() {
        // Non-numeric queue number should default to 0
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:abc": {
                "WredDrp/pkts": "100",
                "WredDrp/bytes": "6400",
                "EcnMarked/pkts": "50",
                "EcnMarked/bytes": "3200"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("UC0"),
            "Invalid queue should default to UC0, result: {}",
            result
        );
    }

    #[test]
    fn test_wred_counters_name() {
        let formatter = WredCountersFormatter::new();
        assert_eq!(formatter.name(), "wred_counters");
    }

    #[test]
    fn test_wred_counters_default() {
        let formatter = WredCountersFormatter::default();
        assert_eq!(formatter.name(), "wred_counters");
    }

    // =====================================================================
    // QUEUE COUNTERS FORMATTER - ADDITIONAL COVERAGE
    // =====================================================================

    #[test]
    fn test_queue_counters_formatter_lowercase_columns() {
        let formatter = QueueCountersFormatter::new();
        // Use lowercase column names
        let data = json!([
            {
                "counter/pkts": "Ethernet0",
                "counter/bytes": "2000",
                "drop/pkts": "5",
                "drop/bytes": "320"
            }
        ]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("2000") || result.contains("Ethernet0"),
            "Result was: {}",
            result
        );
    }

    #[test]
    fn test_queue_counters_formatter_dict_lowercase() {
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0": {
                "counter/bytes": "5000",
                "drop/pkts": "10"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
    }

    #[test]
    fn test_queue_counters_formatter_non_object_array_items() {
        let formatter = QueueCountersFormatter::new();
        // Array with non-object items should be skipped
        let data = json!(["string_item", 123, null]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only, no data rows
        assert!(result.contains("Counter"));
    }

    #[test]
    fn test_queue_counters_name() {
        let formatter = QueueCountersFormatter::new();
        assert_eq!(formatter.name(), "queue_counters");
    }

    #[test]
    fn test_queue_counters_default() {
        let formatter = QueueCountersFormatter::default();
        assert_eq!(formatter.name(), "queue_counters");
    }

    // =====================================================================
    // PRIORITY GROUP FORMATTER - ADDITIONAL COVERAGE
    // =====================================================================

    #[test]
    fn test_priority_group_formatter_lowercase_pg_keys() {
        let formatter = PriorityGroupFormatter::new();
        // Use lowercase pg keys
        let data = json!({
            "Ethernet0": {
                "pg0": "100",
                "pg1": "200",
                "pg2": "0",
                "pg3": "0",
                "pg4": "0",
                "pg5": "0",
                "pg6": "0",
                "pg7": "0"
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "watermark".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
        assert!(result.contains("100"), "Result was: {}", result);
    }

    #[test]
    fn test_priority_group_formatter_counters_multiple_pgs() {
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": {
                    "ingress_packets": "1000",
                    "ingress_bytes": "64000",
                    "drop_packets": "0",
                    "drop_bytes": "0"
                },
                "PG1": {
                    "ingress_packets": "2000",
                    "ingress_bytes": "128000",
                    "drop_packets": "5",
                    "drop_bytes": "320"
                }
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "counters".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        assert!(result.contains("PG0"), "Result was: {}", result);
        assert!(result.contains("PG1"), "Result was: {}", result);
        assert!(result.contains("1000"), "Result was: {}", result);
        assert!(result.contains("2000"), "Result was: {}", result);
    }

    #[test]
    fn test_priority_group_formatter_counters_non_object_pg() {
        let formatter = PriorityGroupFormatter::new();
        // PG value is not an object - should be skipped
        let data = json!({
            "Ethernet0": {
                "PG0": "not_an_object"
            }
        });
        let config = CommandConfig::new();
        let mut options = HashMap::new();
        options.insert("type".to_string(), "counters".to_string());

        let result = formatter
            .format(&data, &config, "simple", Some(&options))
            .unwrap();
        // Should still produce table with headers
        assert!(result.contains("Port"));
    }

    #[test]
    fn test_priority_group_formatter_array_non_object_items() {
        let formatter = PriorityGroupFormatter::new();
        // Array with non-object items should be skipped
        let data = json!(["string_item", 123, null]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Port") || result.contains("PG0"));
    }

    #[test]
    fn test_priority_group_formatter_array_lowercase_headers() {
        let formatter = PriorityGroupFormatter::new();
        // Use lowercase keys in array objects
        let data = json!([
            {
                "port": "Ethernet0",
                "pg0": "100",
                "pg1": "200"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("100") || result.contains("200"),
            "Result was: {}",
            result
        );
    }

    #[test]
    fn test_priority_group_name() {
        let formatter = PriorityGroupFormatter::new();
        assert_eq!(formatter.name(), "priority_group");
    }

    #[test]
    fn test_priority_group_default() {
        let formatter = PriorityGroupFormatter::default();
        assert_eq!(formatter.name(), "priority_group");
    }

    // =====================================================================
    // MULTICAST QUEUE FORMATTER - ADDITIONAL COVERAGE
    // =====================================================================

    #[test]
    fn test_multicast_queue_formatter_lowercase_mc_keys() {
        let formatter = MulticastQueueFormatter::new();
        // Use lowercase mc keys
        let data = json!({
            "Ethernet0": {
                "mc0": "100",
                "mc1": "200",
                "mc2": "0",
                "mc3": "0",
                "mc4": "0",
                "mc5": "0",
                "mc6": "0",
                "mc7": "0"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
        assert!(result.contains("100"), "Result was: {}", result);
    }

    #[test]
    fn test_multicast_queue_formatter_array_port_key() {
        let formatter = MulticastQueueFormatter::new();
        // Use "Port" (uppercase) key
        let data = json!([
            {
                "Port": "Ethernet0",
                "mc0": "100",
                "mc1": "200"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "Result was: {}", result);
    }

    #[test]
    fn test_multicast_queue_formatter_array_non_object_items() {
        let formatter = MulticastQueueFormatter::new();
        // Array with non-object items should be skipped
        let data = json!(["string_item", 123, null]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Port") || result.contains("MC0"));
    }

    #[test]
    fn test_multicast_queue_name() {
        let formatter = MulticastQueueFormatter::new();
        assert_eq!(formatter.name(), "multicast_queue");
    }

    #[test]
    fn test_multicast_queue_default() {
        let formatter = MulticastQueueFormatter::default();
        assert_eq!(formatter.name(), "multicast_queue");
    }

    // =====================================================================
    // SHARED HEADROOM POOL FORMATTER - ADDITIONAL COVERAGE
    // =====================================================================

    #[test]
    fn test_shared_headroom_pool_uppercase_keys() {
        let formatter = SharedHeadroomPoolFormatter::new();
        // Use uppercase keys (Pool, Current, Watermark)
        let data = json!([
            {
                "Pool": "ingress_lossless_pool",
                "Current": "2048",
                "Watermark": "4096"
            }
        ]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("ingress_lossless_pool"),
            "Result was: {}",
            result
        );
        assert!(result.contains("2048"), "Result was: {}", result);
    }

    #[test]
    fn test_shared_headroom_pool_dict_uppercase_keys() {
        let formatter = SharedHeadroomPoolFormatter::new();
        let data = json!({
            "ingress_lossless_pool": {
                "Current": "2048",
                "Watermark": "4096"
            }
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("2048"), "Result was: {}", result);
        assert!(result.contains("4096"), "Result was: {}", result);
    }

    #[test]
    fn test_shared_headroom_pool_dict_non_object_values() {
        let formatter = SharedHeadroomPoolFormatter::new();
        // Dict values that are not objects should be skipped
        let data = json!({
            "pool1": "not_an_object",
            "pool2": 123
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Pool") || result.contains("Current"));
    }

    #[test]
    fn test_shared_headroom_pool_array_non_object_items() {
        let formatter = SharedHeadroomPoolFormatter::new();
        // Array with non-object items should be skipped
        let data = json!(["string_item", 123, null]);
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Pool") || result.contains("Current"));
    }

    #[test]
    fn test_shared_headroom_pool_name() {
        let formatter = SharedHeadroomPoolFormatter::new();
        assert_eq!(formatter.name(), "shared_headroom_pool");
    }

    #[test]
    fn test_shared_headroom_pool_default() {
        let formatter = SharedHeadroomPoolFormatter::default();
        assert_eq!(formatter.name(), "shared_headroom_pool");
    }

    // =====================================================================
    // WRED COUNTERS - EDGE CASES
    // =====================================================================

    #[test]
    fn test_wred_counters_formatter_non_object_counter_values() {
        let formatter = WredCountersFormatter::new();
        // Counter value is not an object - should be skipped
        let data = json!({
            "Ethernet0:0": "not_an_object",
            "Ethernet0:1": 123
        });
        let config = CommandConfig::new();

        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should return header only
        assert!(result.contains("Port") || result.contains("TxQ"));
    }

    #[test]
    fn test_wred_counters_formatter_boundary_queue_7() {
        // Queue 7 is the last UC queue
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet0:7": {
                "WredDrp/pkts": "100",
                "WredDrp/bytes": "6400",
                "EcnMarked/pkts": "50",
                "EcnMarked/bytes": "3200"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(
            result.contains("UC7"),
            "Queue 7 should be UC7, result: {}",
            result
        );
    }

    #[test]
    fn test_wred_counters_formatter_sorting() {
        let formatter = WredCountersFormatter::new();
        let data = json!({
            "Ethernet10:0": {
                "WredDrp/pkts": "300",
                "WredDrp/bytes": "19200",
                "EcnMarked/pkts": "100",
                "EcnMarked/bytes": "6400"
            },
            "Ethernet2:0": {
                "WredDrp/pkts": "200",
                "WredDrp/bytes": "12800",
                "EcnMarked/pkts": "50",
                "EcnMarked/bytes": "3200"
            },
            "Ethernet1:0": {
                "WredDrp/pkts": "100",
                "WredDrp/bytes": "6400",
                "EcnMarked/pkts": "25",
                "EcnMarked/bytes": "1600"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        // Should be sorted naturally: Ethernet1, Ethernet2, Ethernet10
        let eth1_pos = result.find("Ethernet1").unwrap_or(usize::MAX);
        let eth2_pos = result.find("Ethernet2").unwrap_or(usize::MAX);
        let eth10_pos = result.find("Ethernet10").unwrap_or(usize::MAX);
        assert!(
            eth1_pos < eth2_pos,
            "Ethernet1 should come before Ethernet2"
        );
        assert!(
            eth2_pos < eth10_pos,
            "Ethernet2 should come before Ethernet10"
        );
    }

    // ---- Round 3 coverage tests ----

    #[test]
    fn test_queue_counters_formatter_dict_with_trim_columns() {
        // Test dict format with Trim columns (L189-205 region)
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0:0": {
                "Counter/pkts": "100",
                "Counter/bytes": "6400",
                "Drop/pkts": "0",
                "Drop/bytes": "0",
                "Trim/pkts": "5",
                "TrimSent/pkts": "3",
                "TrimDrop/pkts": "2"
            },
            "Ethernet0:1": {
                "Counter/pkts": "200",
                "Counter/bytes": "12800",
                "Drop/pkts": "1",
                "Drop/bytes": "64"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Trim/pkts"), "result: {}", result);
        assert!(result.contains("UC0"), "result: {}", result);
        assert!(result.contains("UC1"), "result: {}", result);
    }

    #[test]
    fn test_queue_counters_formatter_dict_nonzero_filter() {
        // Test nonzero filter option (empty queue data skipped)
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0:0": {
                "Counter/pkts": "100",
                "Counter/bytes": "6400",
                "Drop/pkts": "0",
                "Drop/bytes": "0"
            },
            "Ethernet0:1": {}
        });
        let config = CommandConfig::new();
        let mut opts = HashMap::new();
        opts.insert("nonzero".to_string(), "true".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(result.contains("UC0"), "result: {}", result);
    }

    #[test]
    fn test_queue_counters_formatter_mc_queue() {
        // Test MC queue numbering (q_num >= 8)
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0:8": {
                "Counter/pkts": "50",
                "Counter/bytes": "3200",
                "Drop/pkts": "0",
                "Drop/bytes": "0"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("MC8"), "result: {}", result);
    }

    #[test]
    fn test_queue_counters_formatter_empty_array() {
        let formatter = QueueCountersFormatter::new();
        let data = json!([]);
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Counter/pkts"), "result: {}", result);
    }

    #[test]
    fn test_priority_group_formatter_counters_type() {
        // Test PG "counters" type (L453-457 region)
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": {
                "PG0": {
                    "ingress_packets": "100",
                    "ingress_bytes": "6400",
                    "drop_packets": "5",
                    "drop_bytes": "320"
                },
                "PG1": {
                    "Ingress Packets": "200",
                    "Ingress Bytes": "12800",
                    "Drop Packets": "0",
                    "Drop Bytes": "0"
                }
            }
        });
        let config = CommandConfig::new();
        let mut opts = HashMap::new();
        opts.insert("type".to_string(), "counters".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("PG0"), "result: {}", result);
        assert!(result.contains("100"), "result: {}", result);
    }

    #[test]
    fn test_priority_group_formatter_array_pg_data() {
        // Test PG watermark with array-format PG data (L257-262)
        let formatter = PriorityGroupFormatter::new();
        let data = json!({
            "Ethernet0": [100, 200, 300, 400, 500]
        });
        let config = CommandConfig::new();
        let mut opts = HashMap::new();
        opts.insert("type".to_string(), "watermark".to_string());
        let result = formatter
            .format(&data, &config, "simple", Some(&opts))
            .unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
        assert!(result.contains("100"), "result: {}", result);
        assert!(result.contains("N/A"), "result: {}", result); // filling remaining columns
    }

    #[test]
    fn test_queue_counters_formatter_no_colon_key() {
        // Test dict key without colon separator
        let formatter = QueueCountersFormatter::new();
        let data = json!({
            "Ethernet0": {
                "Counter/pkts": "100",
                "Counter/bytes": "6400",
                "Drop/pkts": "0",
                "Drop/bytes": "0"
            }
        });
        let config = CommandConfig::new();
        let result = formatter.format(&data, &config, "simple", None).unwrap();
        assert!(result.contains("Ethernet0"), "result: {}", result);
    }
}
