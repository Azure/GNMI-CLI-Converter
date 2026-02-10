#![recursion_limit = "256"]
//! # gnmi_cli_lib
//!
//! A Rust library for converting gNMI JSON responses to SONiC CLI-style output.
//!
//! This library provides a complete solution for transforming telemetry data
//! from gNMI (gRPC Network Management Interface) JSON responses into human-readable
//! CLI output that matches SONiC command output format.
//!
//! ## Features
//!
//! - **70 CLI Commands**: Full coverage of SONiC show commands
//! - **Flexible Formatters**: Extensible formatting system for different output types
//! - **Natural Sorting**: Interface names sorted naturally (Ethernet1, Ethernet2, Ethernet10)
//! - **Command Parser**: Parse and validate CLI command strings
//! - **High Performance**: Pure Rust implementation optimized for speed
//!
//! ## Quick Start
//!
//! ```rust
//! use gnmi_cli_lib::converter::GnmiToCliConverter;
//! use serde_json::json;
//!
//! let converter = GnmiToCliConverter::new();
//!
//! // Convert interface status data
//! let json_data = json!({
//!     "Ethernet0": {
//!         "admin_status": "up",
//!         "oper_status": "up",
//!         "speed": "100G"
//!     }
//! });
//!
//! // Convert to CLI output
//! let output = converter.convert("interfaces_status", json_data);
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several main modules:
//!
//! - [`common`]: Core types and constants
//! - [`utils`]: Utility functions for sorting and parsing
//! - [`formatters`]: Output formatters for different command types
//! - [`registry`]: Command registration and lookup
//! - [`commands`]: All command definitions
//! - [`converter`]: Main conversion logic
//! - [`parser`]: CLI command parsing
//!
//! ## Example: Using the Parser
//!
//! ```rust
//! use gnmi_cli_lib::parser::CommandParser;
//! use serde_json::json;
//!
//! let parser = CommandParser::new();
//!
//! // Parse a CLI command
//! let parsed = parser.parse("show interfaces status Ethernet0");
//! assert!(parsed.is_valid);
//! assert_eq!(parsed.command, "interfaces");
//! assert_eq!(parsed.subcommand, "status");
//! assert_eq!(parsed.argument, Some("Ethernet0".to_string()));
//! ```
//!
//! ## Supported Commands
//!
//! The library supports all major SONiC show commands including:
//!
//! - Interface commands (status, counters, description, etc.)
//! - Queue commands (counters, watermarks)
//! - Buffer pool commands (watermarks)
//! - System commands (version, uptime, services)
//! - Transceiver commands (info, status, eeprom)
//! - Network commands (LLDP, VLAN, MAC, ARP)
//! - IPv6 commands (route, neighbors, BGP)
//! - And many more...
//!
//! See the [`commands`] module for the complete list.

#![warn(missing_docs)]

pub mod commands;
pub mod common;
pub mod convenience;
pub mod converter;
pub mod errors;
pub mod formatters;
pub mod mappings;
pub mod parser;
pub mod registry;
pub mod utils;

// Re-export commonly used items at crate root
pub use common::types::{CliMapping, CommandConfig};
pub use converter::{CommandInfo, ConvertError, ConvertResult, GnmiToCliConverter, TableFormat};
pub use errors::GnmiCliError;
pub use formatters::FormatError;
pub use mappings::CLI_TO_GNMI_MAPPING;
pub use parser::{
    get_default_parser, parse_and_convert, parse_command, CommandParser, ParsedCommand,
};
pub use registry::{create_default_registry, CommandRegistry};

// Re-export all convenience functions at crate root
pub use convenience::*;

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get the library version.
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_converter_available() {
        let converter = GnmiToCliConverter::new();
        let commands = converter.available_commands();
        assert!(!commands.is_empty());
    }

    #[test]
    fn test_parser_available() {
        let parser = CommandParser::new();
        let commands = parser.list_commands();
        assert!(!commands.is_empty());
    }

    #[test]
    fn test_registry_available() {
        let registry = registry::create_default_registry();
        assert!(!registry.is_empty());
    }
}
