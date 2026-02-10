//! CLI-to-gNMI mapping dictionary.
//!
//! This module exports the `CLI_TO_GNMI_MAPPING` which maps CLI command
//! strings to their internal command names and gNMI paths.
//! This is the Rust equivalent of Python's `mappings.py`.

use crate::convenience::list_supported_commands;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Mapping of CLI command strings to their gNMI path details.
    ///
    /// This is the Rust equivalent of Python's `CLI_TO_GNMI_MAPPING`.
    /// Keys are CLI command strings (e.g., "show interfaces status"),
    /// values are HashMaps with: gnmi_path, command, subcommand, description, out_of_process_cmd.
    pub static ref CLI_TO_GNMI_MAPPING: HashMap<String, HashMap<String, String>> = {
        list_supported_commands()
    };
}

/// Get a reference to the CLI-to-gNMI mapping.
pub fn get_cli_to_gnmi_mapping() -> &'static HashMap<String, HashMap<String, String>> {
    &CLI_TO_GNMI_MAPPING
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_to_gnmi_mapping_not_empty() {
        let mapping = get_cli_to_gnmi_mapping();
        assert!(!mapping.is_empty());
    }

    #[test]
    fn test_cli_to_gnmi_mapping_has_many_commands() {
        let mapping = get_cli_to_gnmi_mapping();
        // Should contain entries for the major commands
        assert!(
            mapping.len() > 50,
            "Expected > 50 commands, got {}",
            mapping.len()
        );
    }

    #[test]
    fn test_cli_to_gnmi_mapping_contains_known_commands() {
        let mapping = get_cli_to_gnmi_mapping();
        // Should contain some well-known commands
        assert!(
            mapping.contains_key("show buffer_pool watermark"),
            "Should contain 'show buffer_pool watermark'"
        );
        assert!(
            mapping.contains_key("show interfaces status"),
            "Should contain 'show interfaces status'"
        );
    }

    #[test]
    fn test_cli_to_gnmi_mapping_values_have_required_fields() {
        let mapping = get_cli_to_gnmi_mapping();
        for (cmd, info) in mapping.iter() {
            assert!(
                info.contains_key("gnmi_path"),
                "Command '{}' missing gnmi_path",
                cmd
            );
            assert!(
                info.contains_key("command"),
                "Command '{}' missing command",
                cmd
            );
            assert!(
                info.contains_key("subcommand"),
                "Command '{}' missing subcommand",
                cmd
            );
            assert!(
                info.contains_key("description"),
                "Command '{}' missing description",
                cmd
            );
        }
    }
}
