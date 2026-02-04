"""
CLI-to-gNMI mapping dictionary.

This module exports the CLI_TO_GNMI_MAPPING dictionary that maps
CLI command strings to their gNMI paths and configuration.
"""

from gnmi_cli_lib.convenience import list_supported_commands

# Export the mapping dictionary
CLI_TO_GNMI_MAPPING = list_supported_commands()

__all__ = ["CLI_TO_GNMI_MAPPING"]

