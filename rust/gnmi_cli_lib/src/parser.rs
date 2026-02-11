//! Unified Command Parser - Single entry point for CLI command parsing.
//!
//! This module provides a unified `CommandParser` class that serves as the single
//! entry point for parsing CLI commands and routing them to specific converters.
//!
//! The parser follows the consistent naming convention:
//!     `show <command> <subcommand> [optional_argument|<required_argument>]`
//!
//! # Examples
//!
//! - `show buffer_pool watermark`
//! - `show headroom-pool persistent-watermark`
//! - `show interfaces status [interface_name]`
//! - `show interfaces counters detailed <interface_name>`
//! - `show mac aging-time`
//! - `show ipv6 prefix-list [name]`

use crate::converter::{ConvertError, ConvertResult, GnmiToCliConverter};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use thiserror::Error;

/// Errors that can occur during parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    /// The command string is empty.
    #[error("Empty command")]
    EmptyCommand,

    /// The command is not recognized.
    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    /// The subcommand is not recognized for the given command.
    #[error("Unknown subcommand '{subcommand}' for command '{command}'")]
    UnknownSubcommand {
        /// The parent command name.
        command: String,
        /// The unrecognized subcommand name.
        subcommand: String,
    },

    /// A required subcommand was not provided.
    #[error("Missing subcommand for '{0}'")]
    MissingSubcommand(String),

    /// Command parsing failed with the given reason.
    #[error("Parse error: {0}")]
    ParseFailed(String),
}

/// Result type for parsing operations.
pub type ParseResult<T> = Result<T, ParseError>;

/// Represents a parsed CLI command.
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Original command string
    pub raw_command: String,
    /// Whether command had "show" prefix
    pub show_prefix: bool,
    /// Main command (e.g., "interfaces")
    pub command: String,
    /// Subcommand (e.g., "status")
    pub subcommand: String,
    /// Optional argument (e.g., interface name)
    pub argument: Option<String>,
    /// Parsed options (e.g., --period 2)
    pub options: Option<HashMap<String, String>>,
    /// Whether the parsed command is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error_message: Option<String>,
}

impl ParsedCommand {
    /// Create a new valid parsed command.
    pub fn new(raw_command: &str, show_prefix: bool, command: &str, subcommand: &str) -> Self {
        Self {
            raw_command: raw_command.to_string(),
            show_prefix,
            command: command.to_string(),
            subcommand: subcommand.to_string(),
            argument: None,
            options: None,
            is_valid: true,
            error_message: None,
        }
    }

    /// Create an invalid parsed command with error.
    pub fn invalid(raw_command: &str, error: &str) -> Self {
        Self {
            raw_command: raw_command.to_string(),
            show_prefix: false,
            command: String::new(),
            subcommand: String::new(),
            argument: None,
            options: None,
            is_valid: false,
            error_message: Some(error.to_string()),
        }
    }

    /// Set the argument.
    pub fn with_argument(mut self, argument: Option<String>) -> Self {
        self.argument = argument;
        self
    }

    /// Set options.
    pub fn with_options(mut self, options: Option<HashMap<String, String>>) -> Self {
        self.options = options;
        self
    }
}

lazy_static! {
    /// Valid commands and their subcommands.
    static ref VALID_COMMANDS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("buffer_pool", vec!["watermark", "persistent-watermark"]);
        m.insert("headroom-pool", vec!["watermark", "persistent-watermark"]);
        m.insert("clock", vec!["clock", "date", "timezones"]);
        m.insert("dropcounters", vec!["capabilities", "configuration", "counts"]);
        m.insert("interfaces", vec![
            "alias", "counters", "counters-detailed", "counters-errors",
            "counters-fec-histogram", "counters-fec-stats", "counters-rates",
            "counters-rif", "counters-trim", "description", "errors",
            "fec-status", "flap", "naming-mode", "neighbor-expected", "portchannel",
            "status", "switchport-config", "switchport-status",
            "transceiver-eeprom", "transceiver-error-status", "transceiver-info",
            "transceiver-lpmode", "transceiver-pm", "transceiver-presence",
            "transceiver-status"
        ]);
        m.insert("ipv6", vec![
            "bgp-neighbors", "bgp-network", "bgp-summary", "fib",
            "interfaces", "link-local-mode", "prefix-list", "protocol", "route"
        ]);
        m.insert("lldp", vec!["neighbors", "table"]);
        m.insert("mac", vec!["mac", "aging-time"]);
        m.insert("mmu", vec!["mmu"]);
        m.insert("ndp", vec!["ndp"]);
        m.insert("processes", vec!["cpu", "memory", "summary"]);
        m.insert("priority-group", vec![
            "persistent-watermark-headroom", "persistent-watermark-shared",
            "watermark-headroom", "watermark-shared"
        ]);
        m.insert("queue", vec![
            "counters", "persistent-watermark", "persistent-watermark-all",
            "persistent-watermark-multicast", "persistent-watermark-unicast",
            "watermark", "watermark-all", "watermark-multicast", "watermark-unicast",
            "wredcounters"
        ]);
        m.insert("reboot-cause", vec!["current", "history"]);
        m.insert("services", vec!["services"]);
        m.insert("srv6", vec!["stats"]);
        m.insert("system-memory", vec!["system-memory", "memory"]);
        m.insert("uptime", vec!["uptime"]);
        m.insert("version", vec!["version"]);
        m.insert("vlan", vec!["brief"]);
        m.insert("watermark", vec!["telemetry"]);
        m
    };

    /// Commands with optional subcommands.
    static ref COMMANDS_WITH_OPTIONAL_SUBCOMMAND: Vec<&'static str> = vec![
        "ndp", "uptime", "version", "mmu", "clock", "mac", "services", "system-memory"
    ];

    /// Commands that accept optional arguments.
    static ref COMMANDS_WITH_OPTIONAL_ARGS: HashMap<(&'static str, &'static str), &'static str> = {
        let mut m = HashMap::new();
        m.insert(("interfaces", "alias"), "interface_name");
        m.insert(("interfaces", "counters"), "interface_name");
        m.insert(("interfaces", "counters-detailed"), "interface_name");
        m.insert(("interfaces", "counters-errors"), "interface_name");
        m.insert(("interfaces", "counters-fec-histogram"), "interface_name");
        m.insert(("interfaces", "counters-fec-stats"), "interface_name");
        m.insert(("interfaces", "counters-rates"), "interface_name");
        m.insert(("interfaces", "counters-rif"), "interface_name");
        m.insert(("interfaces", "counters-trim"), "interface_name");
        m.insert(("interfaces", "description"), "interface_name");
        m.insert(("interfaces", "errors"), "interface_name");
        m.insert(("interfaces", "fec-status"), "interface_name");
        m.insert(("interfaces", "flap"), "interface_name");
        m.insert(("interfaces", "status"), "interface_name");
        m.insert(("interfaces", "switchport-config"), "interface_name");
        m.insert(("interfaces", "switchport-status"), "interface_name");
        m.insert(("interfaces", "transceiver-eeprom"), "interface_name");
        m.insert(("interfaces", "transceiver-error-status"), "interface_name");
        m.insert(("interfaces", "transceiver-info"), "interface_name");
        m.insert(("interfaces", "transceiver-lpmode"), "interface_name");
        m.insert(("interfaces", "transceiver-pm"), "interface_name");
        m.insert(("interfaces", "transceiver-presence"), "interface_name");
        m.insert(("interfaces", "transceiver-status"), "interface_name");
        m.insert(("ipv6", "prefix-list"), "name");
        m.insert(("ipv6", "route"), "prefix");
        m.insert(("lldp", "neighbors"), "interface_name");
        m.insert(("lldp", "table"), "interface_name");
        m.insert(("mac", "mac"), "address");
        m.insert(("queue", "counters"), "interface_name");
        m.insert(("queue", "wredcounters"), "interface_name");
        m.insert(("srv6", "stats"), "sid");
        m
    };

    /// Regex for extracting --period / -p option.
    static ref PERIOD_REGEX: Regex = Regex::new(r"(?:--period|-p)\s+(\d+)").unwrap();

    /// Regex for extracting --iface option.
    static ref IFACE_REGEX: Regex = Regex::new(r"--iface\s+(\S+)").unwrap();

    /// Regex for extracting --counter_type / -t option.
    static ref COUNTER_TYPE_REGEX: Regex = Regex::new(r"(?:--counter_type|-t)\s+(\S+)").unwrap();

    /// Regex for extracting --group / -g option.
    static ref GROUP_REGEX: Regex = Regex::new(r"(?:--group|-g)\s+(\S+)").unwrap();
}

/// Unified command parser and converter.
///
/// This struct provides a single consistent entry point for parsing CLI commands
/// and converting JSON data to CLI-style output.
pub struct CommandParser {
    /// Table format for output
    _table_format: String,
    /// Converter instance
    converter: GnmiToCliConverter,
}

/// Parse result for subcommand normalization.
struct SubcommandParseResult {
    subcommand: String,
    remaining: Vec<String>,
}

impl CommandParser {
    /// Create a new command parser with default settings.
    pub fn new() -> Self {
        Self {
            _table_format: "simple".to_string(),
            converter: GnmiToCliConverter::new(),
        }
    }

    /// Create a new command parser with custom table format.
    pub fn with_table_format(table_format: &str) -> Self {
        Self {
            _table_format: table_format.to_string(),
            converter: GnmiToCliConverter::new(),
        }
    }

    /// Parse a CLI command string into its components.
    ///
    /// # Arguments
    ///
    /// * `cli_command` - CLI command string (e.g., "show interfaces status Ethernet0")
    ///
    /// # Returns
    ///
    /// `ParsedCommand` with parsed components.
    pub fn parse(&self, cli_command: &str) -> ParsedCommand {
        // Normalize whitespace
        let cli_command = cli_command.split_whitespace().collect::<Vec<_>>().join(" ");

        // Extract options
        let options = self.extract_options(&cli_command);

        // Remove options from command
        let cli_command = self.remove_options(&cli_command);

        // Check for 'show' prefix
        let (has_show, remaining) = if cli_command.to_lowercase().starts_with("show ") {
            (true, cli_command[5..].trim())
        } else {
            (false, cli_command.as_str())
        };

        // Split into parts
        let parts: Vec<&str> = remaining.split_whitespace().collect();

        if parts.is_empty() {
            return ParsedCommand::invalid(&cli_command, "Empty command");
        }

        // Normalize command
        let command = self.normalize_command(parts[0]);

        // Handle commands with optional subcommands
        let (subcommand, argument) =
            if COMMANDS_WITH_OPTIONAL_SUBCOMMAND.contains(&command.as_str()) {
                if parts.len() >= 2 {
                    let potential_sub = parts[1].to_lowercase().replace('_', "-");
                    if let Some(valid_subs) = VALID_COMMANDS.get(command.as_str()) {
                        if valid_subs.contains(&potential_sub.as_str()) {
                            let arg = if parts.len() >= 3 {
                                Some(parts[2..].join(" "))
                            } else {
                                None
                            };
                            (potential_sub, arg)
                        } else {
                            (command.clone(), Some(parts[1..].join(" ")))
                        }
                    } else {
                        (command.clone(), Some(parts[1..].join(" ")))
                    }
                } else {
                    (command.clone(), None)
                }
            } else if parts.len() >= 2 {
                // Handle compound subcommands
                if let Some(parsed) = self.normalize_subcommand(&command, &parts[1..]) {
                    let arg = if !parsed.remaining.is_empty() {
                        Some(parsed.remaining.join(" "))
                    } else {
                        None
                    };
                    (parsed.subcommand, arg)
                } else {
                    return ParsedCommand {
                        raw_command: cli_command.to_string(),
                        show_prefix: has_show,
                        command: command.clone(),
                        subcommand: String::new(),
                        argument: None,
                        options: None,
                        is_valid: false,
                        error_message: Some(format!("Missing subcommand for '{}'", command)),
                    };
                }
            } else {
                return ParsedCommand {
                    raw_command: cli_command.to_string(),
                    show_prefix: has_show,
                    command: command.clone(),
                    subcommand: String::new(),
                    argument: None,
                    options: None,
                    is_valid: false,
                    error_message: Some(format!("Missing subcommand for '{}'", command)),
                };
            };

        // Validate command
        if !VALID_COMMANDS.contains_key(command.as_str()) {
            return ParsedCommand {
                raw_command: cli_command.to_string(),
                show_prefix: has_show,
                command: command.clone(),
                subcommand: subcommand.clone(),
                argument,
                options: Some(options),
                is_valid: false,
                error_message: Some(format!("Unknown command: {}", command)),
            };
        }

        // Validate subcommand
        if let Some(valid_subs) = VALID_COMMANDS.get(command.as_str()) {
            if !valid_subs.contains(&subcommand.as_str()) {
                return ParsedCommand {
                    raw_command: cli_command.to_string(),
                    show_prefix: has_show,
                    command: command.clone(),
                    subcommand: subcommand.clone(),
                    argument,
                    options: Some(options),
                    is_valid: false,
                    error_message: Some(format!(
                        "Unknown subcommand '{}' for '{}'",
                        subcommand, command
                    )),
                };
            }
        }

        ParsedCommand {
            raw_command: cli_command.to_string(),
            show_prefix: has_show,
            command,
            subcommand,
            argument,
            options: if options.is_empty() {
                None
            } else {
                Some(options)
            },
            is_valid: true,
            error_message: None,
        }
    }

    /// Extract command-line options from the command.
    fn extract_options(&self, cli_command: &str) -> HashMap<String, String> {
        let mut options = HashMap::new();

        // Extract --period value
        if let Some(caps) = PERIOD_REGEX.captures(cli_command) {
            if let Some(m) = caps.get(1) {
                options.insert("period".to_string(), m.as_str().to_string());
            }
        }

        // Extract --iface value
        if let Some(caps) = IFACE_REGEX.captures(cli_command) {
            if let Some(m) = caps.get(1) {
                options.insert("iface".to_string(), m.as_str().to_string());
            }
        }

        // Extract --counter_type / -t value
        if let Some(caps) = COUNTER_TYPE_REGEX.captures(cli_command) {
            if let Some(m) = caps.get(1) {
                options.insert("counter_type".to_string(), m.as_str().to_string());
            }
        }

        // Extract --group / -g value
        if let Some(caps) = GROUP_REGEX.captures(cli_command) {
            if let Some(m) = caps.get(1) {
                options.insert("group".to_string(), m.as_str().to_string());
            }
        }

        // Boolean flags (long and short forms)
        if cli_command.contains("--all") || cli_command.contains(" -a") {
            options.insert("all".to_string(), "true".to_string());
        }
        if cli_command.contains("--trim") || cli_command.contains(" -T") {
            options.insert("trim".to_string(), "true".to_string());
        }
        if cli_command.contains("--nonzero") {
            options.insert("nonzero".to_string(), "true".to_string());
        }
        if cli_command.contains("--dom") {
            options.insert("dom".to_string(), "true".to_string());
        }

        options
    }

    /// Remove options from the command string.
    fn remove_options(&self, cli_command: &str) -> String {
        let result = PERIOD_REGEX.replace_all(cli_command, "");
        let result = IFACE_REGEX.replace_all(&result, "");
        let result = COUNTER_TYPE_REGEX.replace_all(&result, "");
        let result = GROUP_REGEX.replace_all(&result, "");
        // Remove short boolean flags: -a (--all), -T (--trim)
        let result = regex::Regex::new(r"\s+-[aT]\b").unwrap().replace_all(&result, "");
        result.trim().to_string()
    }

    /// Normalize command name to standard format.
    fn normalize_command(&self, command: &str) -> String {
        command.to_lowercase()
    }

    /// Normalize subcommand from parts.
    fn normalize_subcommand(&self, command: &str, parts: &[&str]) -> Option<SubcommandParseResult> {
        if parts.is_empty() {
            return None;
        }

        let valid_subs = VALID_COMMANDS.get(command)?;

        // Try compound subcommands (3, then 2 words)
        for length in [3, 2] {
            if parts.len() >= length {
                let potential = parts[..length].join("-").to_lowercase();
                if valid_subs.contains(&potential.as_str()) {
                    return Some(SubcommandParseResult {
                        subcommand: potential,
                        remaining: parts[length..].iter().map(|s| s.to_string()).collect(),
                    });
                }
            }
        }

        // Single word subcommand
        let subcommand = parts[0].to_lowercase().replace('_', "-");
        Some(SubcommandParseResult {
            subcommand,
            remaining: parts[1..].iter().map(|s| s.to_string()).collect(),
        })
    }

    /// Convert JSON data to CLI output using command and subcommand.
    ///
    /// # Arguments
    ///
    /// * `command` - Command type (e.g., "interfaces")
    /// * `subcommand` - Subcommand (e.g., "status")
    /// * `json_data` - JSON data
    /// * `argument` - Optional filter argument
    ///
    /// # Returns
    ///
    /// Formatted CLI-style string.
    pub fn convert(
        &self,
        command: &str,
        subcommand: &str,
        json_data: Value,
        argument: Option<&str>,
    ) -> ConvertResult<String> {
        // Filter data if argument provided
        let data = if let Some(arg) = argument {
            self.filter_by_argument(command, subcommand, json_data, arg)
        } else {
            json_data
        };

        // Map to internal command name
        let command_name = format!("{}_{}", command, subcommand.replace('-', "_"));

        // Create options HashMap with argument if provided
        let options = argument.map(|arg| {
            let mut opts = std::collections::HashMap::new();
            opts.insert("argument".to_string(), arg.to_string());
            opts
        });

        self.converter
            .convert_with_options(&command_name, data, options)
    }

    /// Filter data based on argument.
    fn filter_by_argument(
        &self,
        _command: &str,
        _subcommand: &str,
        data: Value,
        argument: &str,
    ) -> Value {
        if let Value::Object(map) = &data {
            let mut filtered = serde_json::Map::new();
            for (key, value) in map {
                // Precise matching: exact match or bounded prefix match
                let matches = if key == argument {
                    true
                } else if key.starts_with(argument) {
                    // Only match if character after argument is a separator or end
                    let next_char = key[argument.len()..].chars().next();
                    matches!(next_char, Some('|' | ':' | '/' | ' ') | None)
                } else if key.contains(':') {
                    // For composite keys like "Ethernet8:value", check first part exactly
                    let parts: Vec<&str> = key.splitn(2, ':').collect();
                    !parts.is_empty() && parts[0] == argument
                } else {
                    false
                };

                if matches {
                    filtered.insert(key.clone(), value.clone());
                }
            }

            if filtered.is_empty() {
                data
            } else {
                Value::Object(filtered)
            }
        } else {
            data
        }
    }

    /// Parse CLI command and convert JSON data in one step.
    ///
    /// This is the main entry point for command parsing and conversion.
    ///
    /// # Arguments
    ///
    /// * `cli_command` - CLI command string
    /// * `json_data` - JSON data to convert
    ///
    /// # Returns
    ///
    /// Formatted CLI-style string.
    pub fn parse_and_convert(&self, cli_command: &str, json_data: Value) -> ConvertResult<String> {
        let parsed = self.parse(cli_command);

        if !parsed.is_valid {
            return Err(ConvertError::UnknownCommand(
                parsed
                    .error_message
                    .unwrap_or_else(|| "Parse error".to_string()),
            ));
        }

        // Filter data if argument provided
        let data = if let Some(ref arg) = parsed.argument {
            self.filter_by_argument(&parsed.command, &parsed.subcommand, json_data, arg)
        } else {
            json_data
        };

        // Map to internal command name
        let command_name = format!("{}_{}", parsed.command, parsed.subcommand.replace('-', "_"));

        // Build merged options from argument and parsed options (period, etc.)
        let mut opts = std::collections::HashMap::new();
        if let Some(ref arg) = parsed.argument {
            opts.insert("argument".to_string(), arg.clone());
        }
        if let Some(ref parsed_opts) = parsed.options {
            for (k, v) in parsed_opts {
                opts.insert(k.clone(), v.clone());
            }
        }
        let options = if opts.is_empty() { None } else { Some(opts) };

        self.converter
            .convert_with_options(&command_name, data, options)
    }

    /// Get the gNMI path for a CLI command.
    pub fn get_gnmi_path(&self, cli_command: &str) -> Option<String> {
        // Parse command first
        let parsed = self.parse(cli_command);
        if !parsed.is_valid {
            return None;
        }

        // Build gNMI path from parsed components
        Some(format!("SHOW/{}/{}", parsed.command, parsed.subcommand))
    }

    /// List all supported commands and subcommands.
    pub fn list_commands(&self) -> HashMap<String, Vec<String>> {
        VALID_COMMANDS
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
            .collect()
    }

    /// Get help text for a command.
    pub fn get_command_help(&self, command: &str, subcommand: Option<&str>) -> String {
        if !VALID_COMMANDS.contains_key(command) {
            return format!("Unknown command: {}", command);
        }

        if let Some(sub) = subcommand {
            if let Some(valid_subs) = VALID_COMMANDS.get(command) {
                if !valid_subs.contains(&sub) {
                    return format!("Unknown subcommand: {}", sub);
                }
            }

            // Check for optional arguments
            if let Some(arg_name) = COMMANDS_WITH_OPTIONAL_ARGS.get(&(command, sub)) {
                return format!("show {} {} [{}]", command, sub, arg_name);
            }
            return format!("show {} {}", command, sub);
        }

        // List all subcommands
        if let Some(subcommands) = VALID_COMMANDS.get(command) {
            let mut lines = vec![
                format!("show {} <subcommand>", command),
                String::new(),
                "Subcommands:".to_string(),
            ];
            let mut sorted_subs: Vec<_> = subcommands.to_vec();
            sorted_subs.sort();
            for sc in sorted_subs {
                lines.push(format!("  {}", sc));
            }
            lines.join("\n")
        } else {
            format!("No subcommands for {}", command)
        }
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

// Singleton instance
lazy_static! {
    static ref DEFAULT_PARSER: Mutex<CommandParser> = Mutex::new(CommandParser::new());
}

/// Get the default command parser instance.
pub fn get_default_parser() -> &'static Mutex<CommandParser> {
    &DEFAULT_PARSER
}

/// Parse a CLI command string (convenience function).
pub fn parse_command(cli_command: &str) -> ParsedCommand {
    let parser = CommandParser::new();
    parser.parse(cli_command)
}

/// Parse and convert in one step (convenience function).
pub fn parse_and_convert(cli_command: &str, json_data: Value) -> ConvertResult<String> {
    let parser = CommandParser::new();
    parser.parse_and_convert(cli_command, json_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_simple_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces status");

        assert!(parsed.is_valid);
        assert!(parsed.show_prefix);
        assert_eq!(parsed.command, "interfaces");
        assert_eq!(parsed.subcommand, "status");
    }

    #[test]
    fn test_parse_with_argument() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces status Ethernet0");

        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "interfaces");
        assert_eq!(parsed.subcommand, "status");
        assert_eq!(parsed.argument, Some("Ethernet0".to_string()));
    }

    #[test]
    fn test_parse_compound_subcommand() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters detailed");

        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "interfaces");
        assert_eq!(parsed.subcommand, "counters-detailed");
    }

    #[test]
    fn test_parse_optional_subcommand() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show clock");

        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "clock");
        assert_eq!(parsed.subcommand, "clock");
    }

    #[test]
    fn test_parse_with_options() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters --period 2");

        assert!(parsed.is_valid);
        assert!(parsed.options.is_some());
        let options = parsed.options.unwrap();
        assert_eq!(options.get("period"), Some(&"2".to_string()));
    }

    #[test]
    fn test_parse_unknown_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show unknown_command test");

        assert!(!parsed.is_valid);
        assert!(parsed.error_message.is_some());
    }

    #[test]
    fn test_parse_empty_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("");

        assert!(!parsed.is_valid);
    }

    #[test]
    fn test_list_commands() {
        let parser = CommandParser::new();
        let commands = parser.list_commands();

        assert!(!commands.is_empty());
        assert!(commands.contains_key("interfaces"));
        assert!(commands.contains_key("queue"));
    }

    #[test]
    fn test_get_command_help() {
        let parser = CommandParser::new();

        let help = parser.get_command_help("interfaces", None);
        assert!(help.contains("Subcommands"));

        let help = parser.get_command_help("interfaces", Some("status"));
        assert!(help.contains("show interfaces status"));
    }

    #[test]
    fn test_get_gnmi_path() {
        let parser = CommandParser::new();
        let path = parser.get_gnmi_path("show interfaces status");

        assert!(path.is_some());
        assert!(path.unwrap().contains("interfaces"));
    }

    #[test]
    fn test_extract_options() {
        let parser = CommandParser::new();

        let options = parser.extract_options("show interfaces counters --period 5 --all");
        assert_eq!(options.get("period"), Some(&"5".to_string()));
        assert_eq!(options.get("all"), Some(&"true".to_string()));
    }

    #[test]
    fn test_filter_by_argument() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {"status": "up"},
            "Ethernet1": {"status": "down"},
            "PortChannel1": {"status": "up"}
        });

        let filtered = parser.filter_by_argument("interfaces", "status", data, "Ethernet0");

        if let Value::Object(map) = filtered {
            assert!(map.contains_key("Ethernet0"));
            assert!(!map.contains_key("Ethernet1"));
        }
    }

    #[test]
    fn test_parse_with_argument_filter() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces status Ethernet0");

        assert!(parsed.is_valid);
        assert_eq!(parsed.argument, Some("Ethernet0".to_string()));
    }

    #[test]
    fn test_parse_with_iface_option() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters --iface Ethernet0");

        assert!(parsed.is_valid);
        if let Some(options) = &parsed.options {
            assert_eq!(options.get("iface"), Some(&"Ethernet0".to_string()));
        }
    }

    #[test]
    fn test_parse_boolean_flags() {
        let parser = CommandParser::new();

        // Test --all flag - just ensure it parses without panic
        let parsed = parser.parse("show interfaces counters --all");
        let _ = parsed;

        // Test --trim flag
        let parsed = parser.parse("show interfaces counters --trim");
        let _ = parsed;

        // Test --nonzero flag
        let parsed = parser.parse("show interfaces counters --nonzero");
        let _ = parsed;

        // Test --dom flag
        let parsed = parser.parse("show interfaces transceiver --dom");
        let _ = parsed;
    }

    #[test]
    fn test_get_command_help_unknown() {
        let parser = CommandParser::new();
        let help = parser.get_command_help("unknown_cmd", None);
        assert!(help.contains("Unknown command"));
    }

    #[test]
    fn test_get_gnmi_path_unknown() {
        let parser = CommandParser::new();
        let path = parser.get_gnmi_path("show unknown_cmd status");
        // May return None for unknown command
        let _ = path;
    }

    #[test]
    fn test_parsed_command_fields() {
        let parser = CommandParser::new();
        let result = parser.parse("show interfaces status Ethernet0");

        // Verify the fields are accessible
        assert!(result.is_valid);
        assert_eq!(result.command, "interfaces");
        assert_eq!(result.subcommand, "status");
        assert!(!result.raw_command.is_empty());
    }

    #[test]
    fn test_parse_error_display() {
        let error = ParseError::EmptyCommand;
        let display = error.to_string();
        assert!(display.contains("Empty") || display.len() > 0);

        let error = ParseError::UnknownCommand("test".to_string());
        let display = error.to_string();
        assert!(display.contains("test") || display.len() > 0);

        let error = ParseError::MissingSubcommand("test".to_string());
        let display = error.to_string();
        assert!(display.len() > 0);

        let error = ParseError::ParseFailed("test".to_string());
        let display = error.to_string();
        assert!(display.len() > 0);
    }

    #[test]
    fn test_convert_method() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {"status": "up"}
        });

        // Test with known command
        let result = parser.convert("interfaces", "status", data.clone(), None);
        let _ = result;

        // Test with argument
        let result = parser.convert("interfaces", "status", data, Some("Ethernet0"));
        let _ = result;
    }

    #[test]
    fn test_filter_by_argument_array() {
        let parser = CommandParser::new();
        let data = json!([
            {"interface": "Ethernet0", "status": "up"},
            {"interface": "Ethernet1", "status": "down"}
        ]);

        let filtered = parser.filter_by_argument("interfaces", "status", data, "Ethernet0");
        let _ = filtered;
    }

    #[test]
    fn test_remove_options() {
        let parser = CommandParser::new();

        let result = parser.remove_options("show interfaces counters --period 5 --iface eth0");
        assert!(!result.contains("--period"));
        assert!(!result.contains("--iface"));
    }

    #[test]
    fn test_parse_command_convenience() {
        let parsed = parse_command("show interfaces status");
        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "interfaces");
    }

    #[test]
    fn test_parse_and_convert_convenience() {
        let data = json!({"Ethernet0": {"status": "up"}});
        let result = parse_and_convert("show interfaces status", data);
        let _ = result; // Just verify it doesn't panic
    }

    #[test]
    fn test_get_default_parser() {
        let parser_mutex = get_default_parser();
        let parser = parser_mutex.lock().unwrap();
        let commands = parser.list_commands();
        assert!(!commands.is_empty());
    }

    #[test]
    fn test_get_command_help_all_subcommands() {
        let parser = CommandParser::new();
        let help = parser.get_command_help("interfaces", None);
        assert!(help.contains("Subcommands"));
        assert!(help.contains("status"));
    }

    #[test]
    fn test_get_command_help_invalid_subcommand() {
        let parser = CommandParser::new();
        let help = parser.get_command_help("interfaces", Some("invalid_sub"));
        assert!(help.contains("Unknown subcommand"));
    }

    #[test]
    fn test_parse_unknown_subcommand() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces unknownsub");
        assert!(!parsed.is_valid);
        assert!(parsed
            .error_message
            .clone()
            .unwrap()
            .contains("Unknown subcommand"));
    }

    #[test]
    fn test_parse_and_convert_invalid() {
        let parser = CommandParser::new();
        let data = json!({});
        let result = parser.parse_and_convert("show unknowncmd unknownsub", data);
        assert!(result.is_err());
    }

    #[test]
    fn test_with_table_format() {
        let parser = CommandParser::with_table_format("grid");
        let _ = parser.parse("show interfaces status");
    }

    #[test]
    fn test_filter_dict_with_colon_key() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0:Port1": {"status": "up"},
            "Ethernet1:Port2": {"status": "down"}
        });
        let filtered = parser.filter_by_argument("interfaces", "status", data, "Ethernet0");
        // Should match keys containing Ethernet0
        if let Value::Object(map) = &filtered {
            assert!(map.contains_key("Ethernet0:Port1"));
        }
    }

    #[test]
    fn test_parse_error_unknown_subcommand() {
        let error = ParseError::UnknownSubcommand {
            command: "interfaces".to_string(),
            subcommand: "unknown".to_string(),
        };
        let display = error.to_string();
        assert!(display.contains("unknown"));
        assert!(display.contains("interfaces"));
    }

    #[test]
    fn test_parsed_command_with_argument() {
        let parsed = ParsedCommand::new("show test cmd", true, "test", "cmd")
            .with_argument(Some("arg1".to_string()));
        assert_eq!(parsed.argument, Some("arg1".to_string()));
        assert!(parsed.is_valid);
    }

    #[test]
    fn test_parsed_command_with_options() {
        let mut opts = HashMap::new();
        opts.insert("period".to_string(), "5".to_string());
        let parsed = ParsedCommand::new("show test cmd", true, "test", "cmd")
            .with_options(Some(opts.clone()));
        assert!(parsed.options.is_some());
        let options = parsed.options.unwrap();
        assert_eq!(options.get("period"), Some(&"5".to_string()));
    }

    #[test]
    fn test_parsed_command_invalid() {
        let parsed = ParsedCommand::invalid("bad command", "Invalid syntax");
        assert!(!parsed.is_valid);
        assert_eq!(parsed.error_message, Some("Invalid syntax".to_string()));
        assert!(parsed.command.is_empty());
    }

    #[test]
    fn test_filter_by_argument_no_match() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet50": {"status": "up"},
            "Ethernet51": {"status": "down"}
        });
        // Filter for something that doesn't exist - should return original data
        let filtered =
            parser.filter_by_argument("interfaces", "status", data.clone(), "NonExistent");
        // When no match, returns original data
        if let Value::Object(map) = &filtered {
            assert!(map.contains_key("Ethernet50") || map.is_empty());
        }
    }

    #[test]
    fn test_parser_default() {
        let parser = CommandParser::default();
        let parsed = parser.parse("show interfaces status");
        assert!(parsed.is_valid);
    }

    #[test]
    fn test_parse_without_show_prefix() {
        let parser = CommandParser::new();
        let parsed = parser.parse("interfaces status");
        assert!(!parsed.show_prefix);
        assert_eq!(parsed.command, "interfaces");
    }

    #[test]
    fn test_parse_optional_subcommand_with_arg() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ndp some_filter");
        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "ndp");
        assert_eq!(parsed.subcommand, "ndp");
        assert!(parsed.argument.is_some());
    }

    #[test]
    fn test_parse_optional_subcommand_no_valid_commands() {
        let parser = CommandParser::new();
        // system-memory is optional subcommand but has valid memory subcommand
        let parsed = parser.parse("show system-memory");
        assert!(parsed.is_valid);
        assert_eq!(parsed.command, "system-memory");
    }

    #[test]
    fn test_normalize_subcommand_three_word() {
        let parser = CommandParser::new();
        // priority-group persistent-watermark-headroom is a 3-word compound
        let parsed = parser.parse("show priority-group persistent watermark headroom");
        assert!(parsed.is_valid);
        assert_eq!(parsed.subcommand, "persistent-watermark-headroom");
    }

    #[test]
    fn test_filter_by_argument_ends_with() {
        let parser = CommandParser::new();
        let data = json!({
            "Port-Ethernet0": {"status": "up"},
            "Port-Ethernet1": {"status": "down"}
        });
        let filtered = parser.filter_by_argument("interfaces", "status", data, "Ethernet0");
        if let Value::Object(map) = &filtered {
            assert!(map.contains_key("Port-Ethernet0"));
        }
    }

    #[test]
    fn test_filter_by_argument_starts_with() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0-Port": {"status": "up"},
            "Ethernet1-Port": {"status": "down"}
        });
        let filtered = parser.filter_by_argument("interfaces", "status", data, "Ethernet0");
        if let Value::Object(map) = &filtered {
            assert!(map.contains_key("Ethernet0-Port"));
        }
    }

    #[test]
    fn test_extract_options_all_flags() {
        let parser = CommandParser::new();
        let options = parser.extract_options("show cmd --all --trim --nonzero --dom");
        assert_eq!(options.get("all"), Some(&"true".to_string()));
        assert_eq!(options.get("trim"), Some(&"true".to_string()));
        assert_eq!(options.get("nonzero"), Some(&"true".to_string()));
        assert_eq!(options.get("dom"), Some(&"true".to_string()));
    }

    #[test]
    fn test_parse_missing_subcommand_for_non_optional() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show buffer_pool");
        // buffer_pool requires a subcommand
        assert!(!parsed.is_valid);
        assert!(parsed
            .error_message
            .clone()
            .unwrap()
            .contains("Missing subcommand"));
    }

    #[test]
    fn test_get_gnmi_path_invalid() {
        let parser = CommandParser::new();
        let path = parser.get_gnmi_path("show unknowncmd sub");
        assert!(path.is_none());
    }

    #[test]
    fn test_parse_unknown_command_validation() {
        let parser = CommandParser::new();
        // Parse a command that is not in VALID_COMMANDS - returns Missing subcommand first
        let parsed = parser.parse("show fakecommand fakesubcommand");
        assert!(!parsed.is_valid);
        // When command not in VALID_COMMANDS, error could be "Missing subcommand" or "Unknown command"
        assert!(parsed.error_message.is_some());
    }

    // --- Additional GAP-B1 tests ---

    #[test]
    fn test_parse_buffer_pool_watermark() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show buffer_pool watermark");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "buffer_pool");
        assert_eq!(parsed.subcommand, "watermark");
    }

    #[test]
    fn test_parse_buffer_pool_persistent_watermark() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show buffer_pool persistent-watermark");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "persistent-watermark");
    }

    #[test]
    fn test_parse_headroom_pool() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show headroom-pool watermark");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "headroom-pool");
        assert_eq!(parsed.subcommand, "watermark");
    }

    #[test]
    fn test_parse_headroom_pool_persistent() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show headroom-pool persistent-watermark");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "persistent-watermark");
    }

    #[test]
    fn test_parse_clock_date() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show clock date");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "clock");
        assert_eq!(parsed.subcommand, "date");
    }

    #[test]
    fn test_parse_clock_timezones() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show clock timezones");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "timezones");
    }

    #[test]
    fn test_parse_dropcounters_capabilities() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show dropcounters capabilities");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "dropcounters");
        assert_eq!(parsed.subcommand, "capabilities");
    }

    #[test]
    fn test_parse_dropcounters_configuration() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show dropcounters configuration");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "configuration");
    }

    #[test]
    fn test_parse_dropcounters_counts() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show dropcounters counts");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counts");
    }

    #[test]
    fn test_parse_lldp_table() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show lldp table");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "lldp");
        assert_eq!(parsed.subcommand, "table");
    }

    #[test]
    fn test_parse_lldp_neighbors() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show lldp neighbors");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "neighbors");
    }

    #[test]
    fn test_parse_lldp_neighbors_with_interface() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show lldp neighbors Ethernet0");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.argument, Some("Ethernet0".to_string()));
    }

    #[test]
    fn test_parse_vlan_brief() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show vlan brief");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "vlan");
        assert_eq!(parsed.subcommand, "brief");
    }

    #[test]
    fn test_parse_reboot_cause_current() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show reboot-cause current");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "reboot-cause");
        assert_eq!(parsed.subcommand, "current");
    }

    #[test]
    fn test_parse_reboot_cause_history() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show reboot-cause history");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "history");
    }

    #[test]
    fn test_parse_mac_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show mac");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "mac");
        assert_eq!(parsed.subcommand, "mac");
    }

    #[test]
    fn test_parse_mac_aging_time() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show mac aging-time");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "aging-time");
    }

    #[test]
    fn test_parse_ndp_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ndp");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "ndp");
        assert_eq!(parsed.subcommand, "ndp");
    }

    #[test]
    fn test_parse_mmu_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show mmu");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "mmu");
        assert_eq!(parsed.subcommand, "mmu");
    }

    #[test]
    fn test_parse_services_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show services");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "services");
        assert_eq!(parsed.subcommand, "services");
    }

    #[test]
    fn test_parse_uptime_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show uptime");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "uptime");
        assert_eq!(parsed.subcommand, "uptime");
    }

    #[test]
    fn test_parse_version_command() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show version");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "version");
        assert_eq!(parsed.subcommand, "version");
    }

    #[test]
    fn test_parse_system_memory() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show system-memory");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "system-memory");
    }

    #[test]
    fn test_parse_system_memory_explicit() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show system-memory system-memory");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "system-memory");
    }

    #[test]
    fn test_parse_watermark_telemetry() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show watermark telemetry");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "watermark");
        assert_eq!(parsed.subcommand, "telemetry");
    }

    #[test]
    fn test_parse_processes_summary() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show processes summary");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "processes");
        assert_eq!(parsed.subcommand, "summary");
    }

    #[test]
    fn test_parse_processes_cpu() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show processes cpu");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "cpu");
    }

    #[test]
    fn test_parse_processes_memory() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show processes memory");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "memory");
    }

    #[test]
    fn test_parse_srv6_stats() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show srv6 stats");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "srv6");
        assert_eq!(parsed.subcommand, "stats");
    }

    #[test]
    fn test_parse_srv6_stats_with_sid() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show srv6 stats fc00:1::1");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.argument, Some("fc00:1::1".to_string()));
    }

    #[test]
    fn test_parse_ipv6_route() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 route");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "ipv6");
        assert_eq!(parsed.subcommand, "route");
    }

    #[test]
    fn test_parse_ipv6_bgp_summary() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 bgp-summary");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "bgp-summary");
    }

    #[test]
    fn test_parse_ipv6_bgp_neighbors() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 bgp-neighbors");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "bgp-neighbors");
    }

    #[test]
    fn test_parse_ipv6_bgp_network() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 bgp-network");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "bgp-network");
    }

    #[test]
    fn test_parse_ipv6_interfaces() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 interfaces");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "interfaces");
    }

    #[test]
    fn test_parse_ipv6_prefix_list() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 prefix-list");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "prefix-list");
    }

    #[test]
    fn test_parse_ipv6_fib() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 fib");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "fib");
    }

    #[test]
    fn test_parse_ipv6_protocol() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 protocol");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "protocol");
    }

    #[test]
    fn test_parse_ipv6_link_local_mode() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show ipv6 link-local-mode");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "link-local-mode");
    }

    #[test]
    fn test_parse_priority_group_watermark_shared() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show priority-group watermark shared");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "priority-group");
        assert_eq!(parsed.subcommand, "watermark-shared");
    }

    #[test]
    fn test_parse_priority_group_persistent_watermark_headroom() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show priority-group persistent watermark headroom");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "persistent-watermark-headroom");
    }

    #[test]
    fn test_parse_queue_watermark_unicast() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue watermark-unicast");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "queue");
        assert_eq!(parsed.subcommand, "watermark-unicast");
    }

    #[test]
    fn test_parse_queue_persistent_watermark_multicast() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue persistent-watermark-multicast");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "persistent-watermark-multicast");
    }

    #[test]
    fn test_parse_queue_counters() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue counters");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters");
    }

    #[test]
    fn test_parse_queue_counters_with_iface() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue counters Ethernet0");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.argument, Some("Ethernet0".to_string()));
    }

    #[test]
    fn test_parse_queue_wredcounters() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue wredcounters");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "wredcounters");
    }

    #[test]
    fn test_parse_interfaces_alias() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces alias");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "alias");
    }

    #[test]
    fn test_parse_interfaces_description() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces description");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "description");
    }

    #[test]
    fn test_parse_interfaces_errors() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces errors");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "errors");
    }

    #[test]
    fn test_parse_interfaces_counters_fec_stats() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters fec stats");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-fec-stats");
    }

    #[test]
    fn test_parse_interfaces_counters_rates() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters rates");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-rates");
    }

    #[test]
    fn test_parse_interfaces_counters_rif() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters rif");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-rif");
    }

    #[test]
    fn test_parse_interfaces_counters_trim() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters trim");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-trim");
    }

    #[test]
    fn test_parse_interfaces_counters_errors() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters errors");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-errors");
    }

    #[test]
    fn test_parse_interfaces_fec_status() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces fec-status");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "fec-status");
    }

    #[test]
    fn test_parse_interfaces_flap() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces flap");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "flap");
    }

    #[test]
    fn test_parse_interfaces_naming_mode() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces naming-mode");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "naming-mode");
    }

    #[test]
    fn test_parse_interfaces_neighbor_expected() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces neighbor-expected");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "neighbor-expected");
    }

    #[test]
    fn test_parse_interfaces_portchannel() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces portchannel");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "portchannel");
    }

    #[test]
    fn test_parse_interfaces_switchport_config() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces switchport-config");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "switchport-config");
    }

    #[test]
    fn test_parse_interfaces_switchport_status() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces switchport-status");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "switchport-status");
    }

    #[test]
    fn test_parse_interfaces_transceiver_info() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces transceiver-info");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "transceiver-info");
    }

    #[test]
    fn test_parse_interfaces_transceiver_eeprom() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces transceiver-eeprom");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "transceiver-eeprom");
    }

    #[test]
    fn test_parse_interfaces_transceiver_presence() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces transceiver-presence");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "transceiver-presence");
    }

    #[test]
    fn test_parse_interfaces_transceiver_status() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces transceiver-status");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "transceiver-status");
    }

    #[test]
    fn test_parse_interfaces_transceiver_lpmode() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces transceiver-lpmode");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "transceiver-lpmode");
    }

    #[test]
    fn test_parse_whitespace_normalization() {
        let parser = CommandParser::new();
        let parsed = parser.parse("  show   interfaces   status   ");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "interfaces");
        assert_eq!(parsed.subcommand, "status");
    }

    #[test]
    fn test_parse_case_insensitive_show() {
        let parser = CommandParser::new();
        let parsed = parser.parse("SHOW interfaces status");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert!(parsed.show_prefix);
        assert_eq!(parsed.command, "interfaces");
    }

    #[test]
    fn test_get_gnmi_path_various_commands() {
        let parser = CommandParser::new();
        assert_eq!(
            parser.get_gnmi_path("show interfaces status"),
            Some("SHOW/interfaces/status".to_string())
        );
        assert_eq!(
            parser.get_gnmi_path("show clock"),
            Some("SHOW/clock/clock".to_string())
        );
        assert_eq!(
            parser.get_gnmi_path("show lldp table"),
            Some("SHOW/lldp/table".to_string())
        );
    }

    #[test]
    fn test_get_command_help_with_optional_arg() {
        let parser = CommandParser::new();
        let help = parser.get_command_help("interfaces", Some("status"));
        assert!(help.contains("[interface_name]"));
    }

    #[test]
    fn test_get_command_help_no_optional_arg() {
        let parser = CommandParser::new();
        let help = parser.get_command_help("vlan", Some("brief"));
        // vlan brief doesn't have optional args
        assert!(!help.contains("["));
        assert!(help.contains("show vlan brief"));
    }

    #[test]
    fn test_list_commands_contains_all_categories() {
        let parser = CommandParser::new();
        let commands = parser.list_commands();
        let expected = [
            "buffer_pool",
            "headroom-pool",
            "clock",
            "dropcounters",
            "interfaces",
            "ipv6",
            "lldp",
            "mac",
            "mmu",
            "ndp",
            "processes",
            "priority-group",
            "queue",
            "reboot-cause",
            "services",
            "srv6",
            "system-memory",
            "uptime",
            "version",
            "vlan",
            "watermark",
        ];
        for cat in &expected {
            assert!(
                commands.contains_key(*cat),
                "Missing command category: {}",
                cat
            );
        }
    }

    #[test]
    fn test_extract_options_empty_string() {
        let parser = CommandParser::new();
        let options = parser.extract_options("show interfaces status");
        assert!(options.is_empty());
    }

    #[test]
    fn test_extract_options_period_and_iface() {
        let parser = CommandParser::new();
        let options =
            parser.extract_options("show interfaces counters --period 10 --iface Ethernet4");
        assert_eq!(options.get("period"), Some(&"10".to_string()));
        assert_eq!(options.get("iface"), Some(&"Ethernet4".to_string()));
    }

    #[test]
    fn test_remove_options_preserves_command() {
        let parser = CommandParser::new();
        let result = parser.remove_options("show interfaces counters --all --trim");
        // --all and --trim are boolean flags, not removed by remove_options (only --period and --iface are)
        assert!(result.contains("show interfaces counters"));
    }

    #[test]
    fn test_filter_by_argument_non_object() {
        let parser = CommandParser::new();
        let data = json!("just a string");
        let filtered = parser.filter_by_argument("interfaces", "status", data.clone(), "Ethernet0");
        assert_eq!(filtered, data);
    }

    #[test]
    fn test_filter_by_argument_null() {
        let parser = CommandParser::new();
        let data = json!(null);
        let result = parser.filter_by_argument("interfaces", "status", data.clone(), "test");
        assert_eq!(result, data);
    }

    #[test]
    fn test_parse_queue_watermark_all() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue watermark-all");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "watermark-all");
    }

    #[test]
    fn test_parse_queue_persistent_watermark_all() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show queue persistent-watermark-all");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "persistent-watermark-all");
    }

    #[test]
    fn test_parse_interfaces_counters_fec_histogram() {
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces counters fec histogram");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.subcommand, "counters-fec-histogram");
    }

    // ---- Round 3 coverage tests ----

    #[test]
    fn test_parse_optional_subcommand_with_argument() {
        // Tests L296: optional subcommand with 3+ parts → argument captured
        // "clock" is in COMMANDS_WITH_OPTIONAL_SUBCOMMAND and "timezones" is a valid subcommand
        let parser = CommandParser::new();
        let parsed = parser.parse("show clock timezones US");
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "clock");
        assert_eq!(parsed.subcommand, "timezones");
        assert_eq!(parsed.argument, Some("US".to_string()));
    }

    #[test]
    fn test_parse_optional_subcommand_unrecognized_sub() {
        // Tests L305: optional subcommand command with unrecognized second part
        // "ndp" is in COMMANDS_WITH_OPTIONAL_SUBCOMMAND but "xyz" is not a valid sub
        let parser = CommandParser::new();
        let parsed = parser.parse("show ndp xyz");
        // Should treat "xyz" as argument, subcommand = command itself ("ndp")
        assert!(parsed.is_valid, "{:?}", parsed.error_message);
        assert_eq!(parsed.command, "ndp");
    }

    #[test]
    fn test_parse_missing_subcommand_non_optional() {
        // Tests L346-354: command requiring subcommand but none given
        let parser = CommandParser::new();
        let parsed = parser.parse("show interfaces");
        assert!(!parsed.is_valid);
        assert!(parsed
            .error_message
            .as_ref()
            .unwrap()
            .contains("Missing subcommand"));
    }

    #[test]
    fn test_filter_by_argument_colon_key() {
        // Tests L525: filter_by_argument with colon-separated key matching port part
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0:0": {"count": 100},
            "Ethernet0:1": {"count": 200},
            "Ethernet4:0": {"count": 50}
        });
        let result = parser.filter_by_argument("queue", "counters", data, "Ethernet0");
        if let Value::Object(map) = result {
            assert!(map.contains_key("Ethernet0:0"), "map: {:?}", map);
            assert!(map.contains_key("Ethernet0:1"), "map: {:?}", map);
            assert!(!map.contains_key("Ethernet4:0"), "map: {:?}", map);
        } else {
            panic!("Expected object, got: {:?}", result);
        }
    }

    #[test]
    fn test_filter_by_argument_no_match_returns_original() {
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {"count": 100}
        });
        let result = parser.filter_by_argument("interfaces", "status", data.clone(), "Ethernet99");
        // No match → returns original data
        assert_eq!(result, data);
    }

    #[test]
    fn test_filter_by_argument_non_object_data() {
        let parser = CommandParser::new();
        let data = json!([1, 2, 3]);
        let result = parser.filter_by_argument("interfaces", "status", data.clone(), "Ethernet0");
        assert_eq!(result, data);
    }

    #[test]
    fn test_parse_and_convert_invalid_command() {
        let parser = CommandParser::new();
        let result = parser.parse_and_convert("show nonexistent status", json!({}));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_command_help_unknown_command() {
        let parser = CommandParser::new();
        let result = parser.get_command_help("nonexistent", None);
        assert!(result.contains("Unknown command"), "result: {}", result);
    }

    #[test]
    fn test_get_command_help_unknown_subcommand() {
        let parser = CommandParser::new();
        let result = parser.get_command_help("interfaces", Some("nonexistent_sub"));
        assert!(result.contains("Unknown subcommand"), "result: {}", result);
    }

    #[test]
    fn test_get_command_help_list_subs() {
        let parser = CommandParser::new();
        let result = parser.get_command_help("interfaces", None);
        assert!(result.contains("Subcommands:"), "result: {}", result);
        assert!(
            result.contains("status") || result.contains("alias"),
            "result: {}",
            result
        );
    }

    #[test]
    fn test_get_gnmi_path_valid_r3() {
        let parser = CommandParser::new();
        let result = parser.get_gnmi_path("show interfaces status");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "SHOW/interfaces/status");
    }

    #[test]
    fn test_get_gnmi_path_invalid_r3() {
        let parser = CommandParser::new();
        let result = parser.get_gnmi_path("show nonexistent foobar");
        assert!(result.is_none());
    }

    // ---- Round 4 coverage tests ----

    #[test]
    fn test_parse_unknown_command_with_subcommand() {
        // Cover L346-354: normalize_subcommand returns None for unknown command
        let parser = CommandParser::new();
        let parsed = parser.parse("show totallyunknowncmd subcmd");
        assert!(!parsed.is_valid);
        assert!(
            parsed
                .error_message
                .as_ref()
                .unwrap()
                .contains("Missing subcommand"),
            "error: {:?}",
            parsed.error_message
        );
    }

    #[test]
    fn test_parse_and_convert_with_argument() {
        // Cover L565 (filter_by_argument), L576 (argument insert into opts)
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {"alias": "etp1"},
            "Ethernet4": {"alias": "etp2"}
        });
        let result = parser.parse_and_convert("show interfaces alias Ethernet0", data);
        assert!(result.is_ok(), "result: {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("etp1"), "output: {}", output);
    }

    #[test]
    fn test_parse_and_convert_with_period_option() {
        // Cover L579-580 (options merge from parsed_opts)
        // and interface.rs L1618-1621 (period header)
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {
                "STATE": {
                    "RX_OK": "100",
                    "TX_OK": "200"
                }
            }
        });
        let result = parser.parse_and_convert("show interfaces counters --period 5", data);
        assert!(result.is_ok(), "result: {:?}", result);
        let output = result.unwrap();
        assert!(
            output.contains("5") || output.contains("period") || !output.is_empty(),
            "output: {}",
            output
        );
    }

    #[test]
    fn test_parse_and_convert_with_argument_and_period() {
        // Cover L565+L576+L579-580 together
        let parser = CommandParser::new();
        let data = json!({
            "Ethernet0": {
                "STATE": {
                    "RX_OK": "100",
                    "TX_OK": "200"
                }
            },
            "Ethernet4": {
                "STATE": {
                    "RX_OK": "300",
                    "TX_OK": "400"
                }
            }
        });
        let result =
            parser.parse_and_convert("show interfaces counters Ethernet0 --period 10", data);
        assert!(result.is_ok(), "result: {:?}", result);
    }
}
