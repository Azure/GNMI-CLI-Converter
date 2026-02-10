//! Registry module - command registration and lookup.
//!
//! This module provides the `CommandRegistry` which manages all registered
//! CLI commands and their configurations. Commands can be looked up by name
//! or by telemetry path.

use crate::common::types::{Alignment, CliMapping, CommandConfig};
use crate::formatters::{FormatterRegistry, OutputFormatter};
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all CLI commands.
///
/// The registry stores command configurations and provides lookup by
/// command name or telemetry path.
pub struct CommandRegistry {
    /// Map of command name to configuration
    commands: HashMap<String, CommandConfig>,
    /// Map of telemetry path to command name
    path_to_command: HashMap<String, String>,
    /// Formatter registry for output formatting
    formatter_registry: FormatterRegistry,
    /// CLI command string to mapping info (e.g., "show interfaces status" -> CliMapping)
    cli_mappings: HashMap<String, CliMapping>,
}

impl CommandRegistry {
    /// Create a new empty command registry.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            path_to_command: HashMap::new(),
            formatter_registry: FormatterRegistry::new(),
            cli_mappings: HashMap::new(),
        }
    }

    /// Register a command with its configuration.
    pub fn register(&mut self, name: String, config: CommandConfig) {
        // Register path mappings
        for path in config.telemetry_paths() {
            self.path_to_command.insert(path.to_string(), name.clone());
        }
        self.commands.insert(name, config);
    }

    /// Get a command configuration by name.
    pub fn get(&self, name: &str) -> Option<&CommandConfig> {
        self.commands.get(name)
    }

    /// Get a command configuration by telemetry path.
    pub fn get_by_path(&self, path: &str) -> Option<&CommandConfig> {
        self.path_to_command
            .get(path)
            .and_then(|name| self.commands.get(name))
    }

    /// Get the command name for a telemetry path.
    pub fn get_command_name_for_path(&self, path: &str) -> Option<&String> {
        self.path_to_command.get(path)
    }

    /// Get all registered command names.
    pub fn command_names(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }

    /// Get all registered telemetry paths.
    pub fn telemetry_paths(&self) -> Vec<&String> {
        self.path_to_command.keys().collect()
    }

    /// Check if a command is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Get the number of registered commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Get a formatter for a command.
    pub fn get_formatter(&self, name: &str) -> Option<Arc<dyn OutputFormatter + Send + Sync>> {
        if let Some(config) = self.commands.get(name) {
            if let Some(formatter_name) = config.formatter() {
                return self.formatter_registry.get(formatter_name);
            }
        }
        None
    }

    /// Get a formatter by name directly.
    pub fn get_formatter_by_name(
        &self,
        name: &str,
    ) -> Option<Arc<dyn OutputFormatter + Send + Sync>> {
        self.formatter_registry.get(name)
    }

    /// Get all commands in a category.
    pub fn get_commands_by_category(&self, category: &str) -> Vec<(&String, &CommandConfig)> {
        self.commands
            .iter()
            .filter(|(_, config)| config.category() == Some(category))
            .collect()
    }

    /// Get all unique categories.
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self
            .commands
            .values()
            .filter_map(|c| c.category().map(|s| s.to_string()))
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Register a CLI mapping (maps a CLI command string to its gNMI info).
    pub fn register_cli_mapping(&mut self, cli_command: String, mapping: CliMapping) {
        self.cli_mappings.insert(cli_command, mapping);
    }

    /// List all CLI mappings.
    pub fn list_cli_mappings(&self) -> &HashMap<String, CliMapping> {
        &self.cli_mappings
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating CommandConfig instances.
pub struct CommandBuilder {
    name: String,
    category: Option<String>,
    description: Option<String>,
    cli_command: Option<String>,
    telemetry_paths: Vec<String>,
    formatter: Option<String>,
    cli_mappings: Vec<CliMapping>,
    column_order: Vec<String>,
    field_map: HashMap<String, String>,
    alignment: HashMap<String, Alignment>,
    default_values: HashMap<String, String>,
    hidden: bool,
    deprecated: bool,
}

impl CommandBuilder {
    /// Create a new command builder.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            category: None,
            description: None,
            cli_command: None,
            telemetry_paths: Vec::new(),
            formatter: None,
            cli_mappings: Vec::new(),
            column_order: Vec::new(),
            field_map: HashMap::new(),
            alignment: HashMap::new(),
            default_values: HashMap::new(),
            hidden: false,
            deprecated: false,
        }
    }

    /// Set the category.
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the CLI command.
    pub fn cli_command(mut self, command: &str) -> Self {
        self.cli_command = Some(command.to_string());
        self
    }

    /// Add a telemetry path.
    pub fn telemetry_path(mut self, path: &str) -> Self {
        self.telemetry_paths.push(path.to_string());
        self
    }

    /// Add multiple telemetry paths.
    pub fn telemetry_paths(mut self, paths: &[&str]) -> Self {
        for path in paths {
            self.telemetry_paths.push(path.to_string());
        }
        self
    }

    /// Set the formatter.
    pub fn formatter(mut self, formatter: &str) -> Self {
        self.formatter = Some(formatter.to_string());
        self
    }

    /// Add a CLI mapping.
    pub fn cli_mapping(mut self, mapping: CliMapping) -> Self {
        self.cli_mappings.push(mapping);
        self
    }

    /// Set column order.
    pub fn column_order(mut self, columns: &[&str]) -> Self {
        self.column_order = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add a field mapping.
    pub fn field(mut self, json_key: &str, display_name: &str) -> Self {
        self.field_map
            .insert(json_key.to_string(), display_name.to_string());
        self
    }

    /// Set alignment for a column.
    pub fn align(mut self, column: &str, alignment: Alignment) -> Self {
        self.alignment.insert(column.to_string(), alignment);
        self
    }

    /// Set a default value.
    pub fn default(mut self, field: &str, value: &str) -> Self {
        self.default_values
            .insert(field.to_string(), value.to_string());
        self
    }

    /// Mark as hidden.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Mark as deprecated.
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }

    /// Build the CommandConfig.
    pub fn build(self) -> CommandConfig {
        let mut builder = CommandConfig::builder().name(&self.name);

        if let Some(ref cat) = self.category {
            builder = builder.category(cat);
        }
        if let Some(ref desc) = self.description {
            builder = builder.description(desc);
        }
        if let Some(ref cli) = self.cli_command {
            builder = builder.cli_command(cli);
        }
        if !self.telemetry_paths.is_empty() {
            builder = builder.telemetry_paths(
                &self
                    .telemetry_paths
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            );
        }
        if let Some(ref fmt) = self.formatter {
            builder = builder.formatter(fmt);
        }
        if !self.column_order.is_empty() {
            builder = builder.column_order(
                &self
                    .column_order
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            );
        }

        builder.build()
    }
}

/// Create a standard registry with all default commands registered.
pub fn create_default_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Register all commands using the commands module
    crate::commands::register_all_commands(&mut registry);

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_registry_new() {
        let registry = CommandRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_command_registry_register() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .telemetry_paths(&["path/to/data"])
            .build();

        registry.register("test_command".to_string(), config);
        assert!(registry.contains("test_command"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_command_registry_get() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .description("Test description")
            .build();

        registry.register("test_command".to_string(), config);

        let retrieved = registry.get("test_command");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().description(), Some("Test description"));
    }

    #[test]
    fn test_command_registry_get_by_path() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .telemetry_paths(&["sonic-db:COUNTERS_DB"])
            .build();

        registry.register("test_command".to_string(), config);

        let retrieved = registry.get_by_path("sonic-db:COUNTERS_DB");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_command_registry_command_names() {
        let mut registry = CommandRegistry::new();
        registry.register(
            "cmd1".to_string(),
            CommandConfig::builder().name("cmd1").build(),
        );
        registry.register(
            "cmd2".to_string(),
            CommandConfig::builder().name("cmd2").build(),
        );

        let names = registry.command_names();
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn test_command_builder() {
        let config = CommandBuilder::new("show_interfaces")
            .category("interface")
            .description("Show interface status")
            .cli_command("show interfaces status")
            .telemetry_path("sonic-db:APPL_DB/PORT_TABLE")
            .formatter("interface_status")
            .column_order(&["Interface", "Status", "Speed"])
            .field("iface", "Interface")
            .align("Speed", Alignment::Right)
            .default("Speed", "N/A")
            .build();

        assert_eq!(config.name(), Some("show_interfaces"));
        assert_eq!(config.category(), Some("interface"));
        assert_eq!(config.description(), Some("Show interface status"));
    }

    #[test]
    fn test_command_builder_hidden_deprecated() {
        let builder = CommandBuilder::new("old_command").hidden().deprecated();

        assert!(builder.hidden);
        assert!(builder.deprecated);
    }

    #[test]
    fn test_command_registry_categories() {
        let mut registry = CommandRegistry::new();
        registry.register(
            "cmd1".to_string(),
            CommandConfig::builder()
                .name("cmd1")
                .category("interface")
                .build(),
        );
        registry.register(
            "cmd2".to_string(),
            CommandConfig::builder()
                .name("cmd2")
                .category("system")
                .build(),
        );
        registry.register(
            "cmd3".to_string(),
            CommandConfig::builder()
                .name("cmd3")
                .category("interface")
                .build(),
        );

        let cats = registry.categories();
        assert_eq!(cats.len(), 2);
        assert!(cats.contains(&"interface".to_string()));
        assert!(cats.contains(&"system".to_string()));
    }

    #[test]
    fn test_get_commands_by_category() {
        let mut registry = CommandRegistry::new();
        registry.register(
            "cmd1".to_string(),
            CommandConfig::builder()
                .name("cmd1")
                .category("interface")
                .build(),
        );
        registry.register(
            "cmd2".to_string(),
            CommandConfig::builder()
                .name("cmd2")
                .category("system")
                .build(),
        );

        let iface_cmds = registry.get_commands_by_category("interface");
        assert_eq!(iface_cmds.len(), 1);
    }

    #[test]
    fn test_telemetry_paths() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .telemetry_paths(&["path1", "path2"])
            .build();

        registry.register("test_command".to_string(), config);

        let paths = registry.telemetry_paths();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_get_command_name_for_path() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .telemetry_paths(&["sonic-db:TEST_PATH"])
            .build();

        registry.register("test_command".to_string(), config);

        let name = registry.get_command_name_for_path("sonic-db:TEST_PATH");
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "test_command");
    }

    #[test]
    fn test_get_command_name_for_path_not_found() {
        let registry = CommandRegistry::new();
        let name = registry.get_command_name_for_path("nonexistent");
        assert!(name.is_none());
    }

    #[test]
    fn test_get_formatter() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("test_command")
            .formatter("table")
            .build();

        registry.register("test_command".to_string(), config);

        let formatter = registry.get_formatter("test_command");
        assert!(formatter.is_some());
    }

    #[test]
    fn test_get_formatter_no_formatter_set() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder().name("test_command").build();

        registry.register("test_command".to_string(), config);

        let formatter = registry.get_formatter("test_command");
        assert!(formatter.is_none());
    }

    #[test]
    fn test_get_formatter_by_name() {
        let registry = CommandRegistry::new();
        let formatter = registry.get_formatter_by_name("table");
        assert!(formatter.is_some());
    }

    #[test]
    fn test_get_formatter_by_name_not_found() {
        let registry = CommandRegistry::new();
        let formatter = registry.get_formatter_by_name("nonexistent_formatter");
        assert!(formatter.is_none());
    }

    #[test]
    fn test_create_default_registry() {
        let registry = create_default_registry();
        assert!(!registry.is_empty());
        // The default registry should have many commands
        assert!(registry.len() > 50);
    }

    #[test]
    fn test_registry_default() {
        let registry = CommandRegistry::default();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_command_builder_telemetry_paths() {
        let config = CommandBuilder::new("test")
            .telemetry_paths(&["path1", "path2", "path3"])
            .build();

        let paths = config.telemetry_paths();
        assert_eq!(paths.len(), 3);
    }

    #[test]
    fn test_command_builder_cli_mapping() {
        let mapping = CliMapping::new("show test", "test", "sub", "cli_command");
        let builder = CommandBuilder::new("test").cli_mapping(mapping);

        assert_eq!(builder.cli_mappings.len(), 1);
    }

    #[test]
    fn test_get_by_path_not_found() {
        let registry = CommandRegistry::new();
        let result = registry.get_by_path("nonexistent/path");
        assert!(result.is_none());
    }

    #[test]
    fn test_registry_contains_not_found() {
        let registry = CommandRegistry::new();
        assert!(!registry.contains("nonexistent"));
    }

    // =======================================================================
    // Phase 3.6 — Additional edge case tests for registry (GAP-B6)
    // =======================================================================

    #[test]
    fn test_register_cli_mapping_and_list() {
        let mut registry = CommandRegistry::new();
        let mapping = CliMapping::new("show test cmd", "path/test", "test", "cmd");
        registry.register_cli_mapping("show test cmd".to_string(), mapping);

        let mappings = registry.list_cli_mappings();
        assert_eq!(mappings.len(), 1);
        assert!(mappings.contains_key("show test cmd"));
    }

    #[test]
    fn test_register_overwrites_existing() {
        let mut registry = CommandRegistry::new();
        let config1 = CommandConfig::builder()
            .name("cmd")
            .description("first")
            .build();
        registry.register("cmd".to_string(), config1);

        let config2 = CommandConfig::builder()
            .name("cmd")
            .description("second")
            .build();
        registry.register("cmd".to_string(), config2);

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.get("cmd").unwrap().description(), Some("second"));
    }

    #[test]
    fn test_get_nonexistent_command() {
        let registry = CommandRegistry::new();
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_get_formatter_nonexistent_command() {
        let registry = CommandRegistry::new();
        assert!(registry.get_formatter("nonexistent").is_none());
    }

    #[test]
    fn test_get_commands_by_nonexistent_category() {
        let mut registry = CommandRegistry::new();
        registry.register(
            "cmd1".to_string(),
            CommandConfig::builder()
                .name("cmd1")
                .category("real")
                .build(),
        );

        let cmds = registry.get_commands_by_category("nonexistent");
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_command_builder_all_methods_chained() {
        let mapping = CliMapping::new("show x y", "path", "x", "y");
        let config = CommandBuilder::new("full_test")
            .category("cat")
            .description("desc")
            .cli_command("show full test")
            .telemetry_path("sonic-db:PATH1")
            .telemetry_paths(&["sonic-db:PATH2"])
            .formatter("table")
            .column_order(&["A", "B"])
            .field("json_key", "Display Name")
            .align("A", Alignment::Left)
            .default("B", "N/A")
            .cli_mapping(mapping)
            .build();

        assert_eq!(config.name(), Some("full_test"));
        assert_eq!(config.category(), Some("cat"));
    }

    #[test]
    fn test_multiple_telemetry_paths_same_command() {
        let mut registry = CommandRegistry::new();
        let config = CommandConfig::builder()
            .name("multi_path_cmd")
            .telemetry_paths(&["path/a", "path/b", "path/c"])
            .build();
        registry.register("multi_path_cmd".to_string(), config);

        assert!(registry.get_by_path("path/a").is_some());
        assert!(registry.get_by_path("path/b").is_some());
        assert!(registry.get_by_path("path/c").is_some());
        assert_eq!(
            registry.get_command_name_for_path("path/a").unwrap(),
            "multi_path_cmd"
        );
    }

    #[test]
    fn test_categories_empty_registry() {
        let registry = CommandRegistry::new();
        assert!(registry.categories().is_empty());
    }
}
