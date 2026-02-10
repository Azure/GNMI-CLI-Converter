//! Unified error types for gnmi_cli_lib.
//!
//! This module provides a single error type that wraps all possible
//! errors from the library, as specified in the conversion plan (Appendix C).

use crate::converter::ConvertError;
use crate::formatters::FormatError;
use thiserror::Error;

/// Unified error type for all gnmi_cli_lib operations.
#[derive(Error, Debug)]
pub enum GnmiCliError {
    /// Unknown command error.
    #[error("Unknown command: {command} {subcommand}")]
    UnknownCommand {
        /// The command name.
        command: String,
        /// The subcommand name.
        subcommand: String,
    },

    /// JSON parsing error.
    #[error("Invalid JSON input: {0}")]
    JsonParseError(#[from] serde_json::Error),

    /// Formatter error.
    #[error("Formatter error: {0}")]
    FormatError(#[from] FormatError),

    /// Conversion error.
    #[error("Conversion error: {0}")]
    ConvertError(#[from] ConvertError),

    /// Invalid CLI command.
    #[error("Invalid CLI command: {0}")]
    ParseError(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_command_error() {
        let err = GnmiCliError::UnknownCommand {
            command: "test".to_string(),
            subcommand: "sub".to_string(),
        };
        assert_eq!(format!("{}", err), "Unknown command: test sub");
    }

    #[test]
    fn test_parse_error() {
        let err = GnmiCliError::ParseError("bad command".to_string());
        assert_eq!(format!("{}", err), "Invalid CLI command: bad command");
    }

    #[test]
    fn test_convert_error_from() {
        let convert_err = ConvertError::UnknownCommand("test_cmd".to_string());
        let err: GnmiCliError = convert_err.into();
        assert!(matches!(err, GnmiCliError::ConvertError(_)));
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: GnmiCliError = io_err.into();
        assert!(matches!(err, GnmiCliError::IoError(_)));
    }

    #[test]
    fn test_json_parse_error_from() {
        let json_result: Result<serde_json::Value, _> = serde_json::from_str("not json");
        let json_err = json_result.unwrap_err();
        let err: GnmiCliError = json_err.into();
        assert!(matches!(err, GnmiCliError::JsonParseError(_)));
    }
}
