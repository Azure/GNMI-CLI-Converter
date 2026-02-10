//! Data classes for clean configuration and type definitions.
//!
//! This module provides the core data structures used throughout the library:
//! - `CommandConfig`: Configuration for a single command/subcommand combination
//! - `CliMapping`: Maps CLI command string to internal command/subcommand

use serde::{Deserialize, Serialize};

/// Text alignment for table columns.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    #[default]
    /// Right-aligned text.
    Right,
    /// Left-aligned text.
    Left,
}

impl Alignment {
    /// Convert to string for tabulate compatibility.
    pub fn as_str(&self) -> &'static str {
        match self {
            Alignment::Right => "right",
            Alignment::Left => "left",
        }
    }
}

impl std::str::FromStr for Alignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "right" => Ok(Alignment::Right),
            "left" => Ok(Alignment::Left),
            _ => Err(format!("Invalid alignment: {}", s)),
        }
    }
}

/// Configuration for a single command/subcommand combination.
///
/// This struct defines how a command's output should be formatted,
/// including the title, headers, and output type.
///
/// # Example
///
/// ```
/// use gnmi_cli_lib::common::types::CommandConfig;
///
/// let config = CommandConfig::builder()
///     .name("interfaces_status")
///     .description("Show interface status information")
///     .formatter("interface_status")
///     .build();
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandConfig {
    /// Command name
    name: Option<String>,
    /// Command category
    category: Option<String>,
    /// Title displayed above the table (e.g., "Shared pool maximum occupancy:")
    title: Option<String>,
    /// Type of formatter to use ("table", "text", "list", or custom)
    formatter: Option<String>,
    /// CLI command string
    cli_command: Option<String>,
    /// Column headers for table output
    headers: Vec<String>,
    /// Field name for the primary key (used by text/list formatters)
    key_field: Option<String>,
    /// List of field names to extract values from
    value_fields: Vec<String>,
    /// If true, columns are determined from data keys
    dynamic_columns: bool,
    /// Text alignment ("right" or "left")
    alignment: Alignment,
    /// Human-readable description of the command
    description: Option<String>,
    /// Telemetry paths associated with this command
    telemetry_paths: Vec<String>,
    /// Column ordering for output
    column_order: Vec<String>,
}

impl CommandConfig {
    /// Create a new builder for CommandConfig.
    pub fn builder() -> CommandConfigBuilder {
        CommandConfigBuilder::default()
    }

    /// Create a new empty CommandConfig with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to set the key field.
    pub fn with_key_field(mut self, key_field: &str) -> Self {
        self.key_field = Some(key_field.to_string());
        self
    }

    /// Builder method to enable dynamic columns.
    pub fn with_dynamic_columns(mut self, enabled: bool) -> Self {
        self.dynamic_columns = enabled;
        self
    }

    /// Builder method to set headers.
    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    /// Builder method to set title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Builder method to set value fields.
    pub fn with_value_fields(mut self, fields: Vec<String>) -> Self {
        self.value_fields = fields;
        self
    }

    /// Get the command name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the category.
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Get the title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Get the formatter name.
    pub fn formatter(&self) -> Option<&str> {
        self.formatter.as_deref()
    }

    /// Get the CLI command.
    pub fn cli_command(&self) -> Option<&str> {
        self.cli_command.as_deref()
    }

    /// Get the headers.
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Get the key field.
    pub fn key_field(&self) -> Option<&str> {
        self.key_field.as_deref()
    }

    /// Get the value fields.
    pub fn value_fields(&self) -> &[String] {
        &self.value_fields
    }

    /// Check if dynamic columns are enabled.
    pub fn dynamic_columns(&self) -> bool {
        self.dynamic_columns
    }

    /// Get the alignment.
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Get the description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get the telemetry paths.
    pub fn telemetry_paths(&self) -> &[String] {
        &self.telemetry_paths
    }

    /// Get the column order.
    pub fn column_order(&self) -> &[String] {
        &self.column_order
    }
}

/// Builder for CommandConfig.
#[derive(Debug, Clone, Default)]
pub struct CommandConfigBuilder {
    name: Option<String>,
    category: Option<String>,
    title: Option<String>,
    formatter: Option<String>,
    cli_command: Option<String>,
    headers: Vec<String>,
    key_field: Option<String>,
    value_fields: Vec<String>,
    dynamic_columns: bool,
    alignment: Alignment,
    description: Option<String>,
    telemetry_paths: Vec<String>,
    column_order: Vec<String>,
}

impl CommandConfigBuilder {
    /// Set the command name.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set the category.
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    /// Set the title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the formatter.
    pub fn formatter(mut self, formatter: &str) -> Self {
        self.formatter = Some(formatter.to_string());
        self
    }

    /// Set the CLI command.
    pub fn cli_command(mut self, cli_command: &str) -> Self {
        self.cli_command = Some(cli_command.to_string());
        self
    }

    /// Set the headers.
    pub fn headers(mut self, headers: &[&str]) -> Self {
        self.headers = headers.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set the key field.
    pub fn key_field(mut self, key_field: &str) -> Self {
        self.key_field = Some(key_field.to_string());
        self
    }

    /// Set the value fields.
    pub fn value_fields(mut self, fields: &[&str]) -> Self {
        self.value_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set dynamic columns flag.
    pub fn dynamic_columns(mut self, dynamic: bool) -> Self {
        self.dynamic_columns = dynamic;
        self
    }

    /// Set the alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the description.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set a single telemetry path.
    pub fn telemetry_path(mut self, path: &str) -> Self {
        self.telemetry_paths.push(path.to_string());
        self
    }

    /// Set multiple telemetry paths.
    pub fn telemetry_paths(mut self, paths: &[&str]) -> Self {
        self.telemetry_paths = paths.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set the column order.
    pub fn column_order(mut self, columns: &[&str]) -> Self {
        self.column_order = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Build the CommandConfig.
    pub fn build(self) -> CommandConfig {
        CommandConfig {
            name: self.name,
            category: self.category,
            title: self.title,
            formatter: self.formatter,
            cli_command: self.cli_command,
            headers: self.headers,
            key_field: self.key_field,
            value_fields: self.value_fields,
            dynamic_columns: self.dynamic_columns,
            alignment: self.alignment,
            description: self.description,
            telemetry_paths: self.telemetry_paths,
            column_order: self.column_order,
        }
    }
}

/// Maps a CLI command string to internal command/subcommand pair.
///
/// This struct provides the mapping between user-facing CLI commands
/// (like "show interfaces status") and the internal command/subcommand
/// pair used by the converter.
///
/// # Example
///
/// ```
/// use gnmi_cli_lib::common::types::CliMapping;
///
/// let mapping = CliMapping::new(
///     "show interfaces status",
///     "SHOW/interfaces/status",
///     "interfaces",
///     "status",
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliMapping {
    /// The CLI command string (e.g., "show interfaces status")
    pub cli_command: String,
    /// The corresponding gNMI path (e.g., "SHOW/interfaces/status")
    pub gnmi_path: String,
    /// Internal command name (e.g., "interfaces")
    pub command: String,
    /// Internal subcommand name (e.g., "status")
    pub subcommand: String,
    /// Human-readable description
    pub description: String,
    /// Shell command for out-of-process execution
    pub out_of_process_cmd: String,
}

impl CliMapping {
    /// Create a new CliMapping with required fields.
    ///
    /// # Panics
    ///
    /// Panics if `cli_command`, `command`, or `subcommand` is empty.
    pub fn new(
        cli_command: impl Into<String>,
        gnmi_path: impl Into<String>,
        command: impl Into<String>,
        subcommand: impl Into<String>,
    ) -> Self {
        let cli_command = cli_command.into();
        let command = command.into();
        let subcommand = subcommand.into();

        assert!(!cli_command.is_empty(), "cli_command cannot be empty");
        assert!(!command.is_empty(), "command cannot be empty");
        assert!(!subcommand.is_empty(), "subcommand cannot be empty");

        Self {
            cli_command,
            gnmi_path: gnmi_path.into(),
            command,
            subcommand,
            description: String::new(),
            out_of_process_cmd: String::new(),
        }
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the out-of-process command.
    pub fn with_out_of_process_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.out_of_process_cmd = cmd.into();
        self
    }

    /// Convert to a dictionary-like structure for JSON serialization.
    pub fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "cli_command": self.cli_command,
            "gnmi_path": self.gnmi_path,
            "command": self.command,
            "subcommand": self.subcommand,
            "description": self.description,
            "out_of_process_cmd": self.out_of_process_cmd,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_default() {
        assert_eq!(Alignment::default(), Alignment::Right);
    }

    #[test]
    fn test_alignment_as_str() {
        assert_eq!(Alignment::Right.as_str(), "right");
        assert_eq!(Alignment::Left.as_str(), "left");
    }

    #[test]
    fn test_alignment_from_str() {
        assert_eq!("right".parse::<Alignment>().unwrap(), Alignment::Right);
        assert_eq!("left".parse::<Alignment>().unwrap(), Alignment::Left);
        assert_eq!("RIGHT".parse::<Alignment>().unwrap(), Alignment::Right);
        assert!("invalid".parse::<Alignment>().is_err());
    }

    #[test]
    fn test_command_config_builder() {
        let config = CommandConfig::builder()
            .name("test_command")
            .category("test")
            .title("Test Title")
            .formatter("interface_status")
            .headers(&["Col1", "Col2"])
            .key_field("name")
            .value_fields(&["value1", "value2"])
            .dynamic_columns(true)
            .alignment(Alignment::Left)
            .description("Test description")
            .build();

        assert_eq!(config.name(), Some("test_command"));
        assert_eq!(config.category(), Some("test"));
        assert_eq!(config.title(), Some("Test Title"));
        assert_eq!(config.formatter(), Some("interface_status"));
        assert_eq!(config.headers(), &["Col1", "Col2"]);
        assert_eq!(config.key_field(), Some("name"));
        assert_eq!(config.value_fields(), &["value1", "value2"]);
        assert!(config.dynamic_columns());
        assert_eq!(config.alignment(), Alignment::Left);
        assert_eq!(config.description(), Some("Test description"));
    }

    #[test]
    fn test_command_config_telemetry_paths() {
        let config = CommandConfig::builder()
            .name("test")
            .telemetry_paths(&["path/one", "path/two"])
            .build();

        assert_eq!(config.telemetry_paths(), &["path/one", "path/two"]);
    }

    #[test]
    fn test_command_config_column_order() {
        let config = CommandConfig::builder()
            .name("test")
            .column_order(&["Col1", "Col2", "Col3"])
            .build();

        assert_eq!(config.column_order(), &["Col1", "Col2", "Col3"]);
    }

    #[test]
    fn test_cli_mapping_new() {
        let mapping = CliMapping::new(
            "show interfaces status",
            "SHOW/interfaces/status",
            "interfaces",
            "status",
        );

        assert_eq!(mapping.cli_command, "show interfaces status");
        assert_eq!(mapping.gnmi_path, "SHOW/interfaces/status");
        assert_eq!(mapping.command, "interfaces");
        assert_eq!(mapping.subcommand, "status");
        assert!(mapping.description.is_empty());
        assert!(mapping.out_of_process_cmd.is_empty());
    }

    #[test]
    fn test_cli_mapping_builder() {
        let mapping = CliMapping::new(
            "show interfaces status",
            "SHOW/interfaces/status",
            "interfaces",
            "status",
        )
        .with_description("Interface status information")
        .with_out_of_process_cmd("show interfaces status");

        assert_eq!(mapping.description, "Interface status information");
        assert_eq!(mapping.out_of_process_cmd, "show interfaces status");
    }

    #[test]
    #[should_panic(expected = "cli_command cannot be empty")]
    fn test_cli_mapping_empty_cli_command() {
        CliMapping::new("", "path", "cmd", "sub");
    }

    #[test]
    #[should_panic(expected = "command cannot be empty")]
    fn test_cli_mapping_empty_command() {
        CliMapping::new("show test", "path", "", "sub");
    }

    #[test]
    #[should_panic(expected = "subcommand cannot be empty")]
    fn test_cli_mapping_empty_subcommand() {
        CliMapping::new("show test", "path", "cmd", "");
    }

    #[test]
    fn test_cli_mapping_to_dict() {
        let mapping = CliMapping::new("show test", "path/test", "test", "cmd");
        let dict = mapping.to_dict();

        assert_eq!(dict["cli_command"], "show test");
        assert_eq!(dict["gnmi_path"], "path/test");
        assert_eq!(dict["command"], "test");
        assert_eq!(dict["subcommand"], "cmd");
    }

    #[test]
    fn test_command_config_serialize() {
        let config = CommandConfig::builder().name("test").title("Test").build();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"title\":\"Test\""));
    }

    #[test]
    fn test_cli_mapping_serialize() {
        let mapping = CliMapping::new("show test", "path", "test", "cmd");
        let json = serde_json::to_string(&mapping).unwrap();
        assert!(json.contains("\"cli_command\":\"show test\""));
    }

    // =======================================================================
    // Phase 3.6 — Additional edge case tests for types (GAP-B6)
    // =======================================================================

    #[test]
    fn test_command_config_new_with_builder_methods() {
        let config = CommandConfig::new()
            .with_title("Title")
            .with_key_field("key")
            .with_dynamic_columns(false)
            .with_headers(vec!["H1".to_string()])
            .with_value_fields(vec!["v1".to_string()]);

        assert_eq!(config.title(), Some("Title"));
        assert_eq!(config.key_field(), Some("key"));
        assert!(!config.dynamic_columns());
        assert_eq!(config.headers(), &["H1"]);
        assert_eq!(config.value_fields(), &["v1"]);
    }

    #[test]
    fn test_command_config_builder_cli_command() {
        let config = CommandConfig::builder()
            .name("test")
            .cli_command("show test status")
            .build();

        assert_eq!(config.cli_command(), Some("show test status"));
    }

    #[test]
    fn test_command_config_builder_single_telemetry_path() {
        let config = CommandConfig::builder()
            .name("test")
            .telemetry_path("sonic-db:COUNTERS")
            .telemetry_path("sonic-db:APPL_DB")
            .build();

        assert_eq!(config.telemetry_paths().len(), 2);
    }

    #[test]
    fn test_command_config_default_values() {
        let config = CommandConfig::new();
        assert!(config.name().is_none());
        assert!(config.category().is_none());
        assert!(config.title().is_none());
        assert!(config.formatter().is_none());
        assert!(config.cli_command().is_none());
        assert!(config.key_field().is_none());
        assert!(config.description().is_none());
        assert!(config.headers().is_empty());
        assert!(config.value_fields().is_empty());
        assert!(config.telemetry_paths().is_empty());
        assert!(config.column_order().is_empty());
        assert!(!config.dynamic_columns());
        assert_eq!(config.alignment(), Alignment::Right);
    }
}
