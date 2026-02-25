#!/usr/bin/env python3
"""
Command-line interface for the gNMI to CLI converter.

This module provides the CLI entry point for the gnmi_cli_lib library,
supporting interactive usage and script integration.

Usage:
    python -m gnmi_cli_lib.cli --input <json_file> --command <cli_command>
    python -m gnmi_cli_lib.cli --list-commands
    cat data.json | python -m gnmi_cli_lib.cli --command "show interfaces status"
"""

import argparse
import json
import sys
from typing import Optional

from .parser import CommandParser
from .commands import register_all_commands
from .registry import CommandRegistry


def create_parser() -> argparse.ArgumentParser:
    """Create the argument parser for the CLI."""
    parser = argparse.ArgumentParser(
        description="Convert gNMI JSON data to SONiC CLI output format",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert JSON file using CLI command
  python -m gnmi_cli_lib.cli -i data.json -c "show interfaces status"
  
  # Convert JSON file using internal command
  python -m gnmi_cli_lib.cli -i data.json -k INTERFACE_STATUS
  
  # Read JSON from stdin
  cat data.json | python -m gnmi_cli_lib.cli -c "show version"
  
  # List all supported commands
  python -m gnmi_cli_lib.cli --list-commands
  
  # List commands matching pattern
  python -m gnmi_cli_lib.cli --list-commands --pattern interface
"""
    )
    
    parser.add_argument(
        "-i", "--input",
        type=str,
        help="Path to JSON file containing gNMI data. If not provided, reads from stdin."
    )
    
    parser.add_argument(
        "-c", "--command",
        type=str,
        help="CLI command to convert (e.g., 'show interfaces status')"
    )
    
    parser.add_argument(
        "-k", "--key",
        type=str,
        help="Internal command key to use (e.g., 'INTERFACE_STATUS')"
    )
    
    parser.add_argument(
        "--list-commands",
        action="store_true",
        help="List all supported CLI commands"
    )
    
    parser.add_argument(
        "--pattern",
        type=str,
        default="",
        help="Filter commands by pattern when using --list-commands"
    )
    
    parser.add_argument(
        "-o", "--output",
        type=str,
        help="Output file path. If not provided, writes to stdout."
    )
    
    parser.add_argument(
        "--format",
        choices=["text", "json"],
        default="text",
        help="Output format (default: text)"
    )
    
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose output"
    )
    
    return parser


def load_json_data(input_path: Optional[str]) -> dict:
    """Load JSON data from file or stdin."""
    try:
        if input_path:
            with open(input_path, 'r') as f:
                return json.load(f)
        else:
            # Read from stdin
            if sys.stdin.isatty():
                print("Error: No input provided. Use -i <file> or pipe JSON data.", 
                      file=sys.stderr)
                sys.exit(1)
            return json.load(sys.stdin)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON data: {e}", file=sys.stderr)
        sys.exit(1)
    except FileNotFoundError:
        print(f"Error: File not found: {input_path}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error loading data: {e}", file=sys.stderr)
        sys.exit(1)


def write_output(content: str, output_path: Optional[str]) -> None:
    """Write output to file or stdout."""
    try:
        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)
        else:
            print(content, end='')
    except Exception as e:
        print(f"Error writing output: {e}", file=sys.stderr)
        sys.exit(1)


def list_supported_commands(pattern: str = "") -> str:
    """List all supported CLI commands."""
    registry = CommandRegistry()
    register_all_commands(registry)
    
    lines = ["Supported CLI Commands:", "=" * 60, ""]
    
    # Get CLI mappings
    cli_mappings = registry.list_cli_mappings()
    cli_commands = sorted(cli_mappings.keys())
    count = 0
    
    for cli_cmd in cli_commands:
        # Apply pattern filter
        if pattern and pattern.lower() not in cli_cmd.lower():
            continue
        
        count += 1
        mapping = cli_mappings[cli_cmd]
        lines.append(f"  {cli_cmd}")
        
        # Show internal key
        lines.append(f"    Key: {mapping.command}/{mapping.subcommand}")
    
    lines.append("")
    lines.append(f"Total: {count} commands")
    
    return "\n".join(lines)


def main() -> int:
    """Main entry point for the CLI."""
    parser = create_parser()
    args = parser.parse_args()
    
    # Handle --list-commands
    if args.list_commands:
        output = list_supported_commands(args.pattern)
        write_output(output + "\n", args.output)
        return 0
    
    # Validate arguments
    if not args.command and not args.key:
        parser.error("Either --command or --key is required")
    
    # Create parser
    registry = CommandRegistry()
    register_all_commands(registry)
    parser = CommandParser(registry=registry)
    
    # Load input data
    json_data = load_json_data(args.input)
    
    # Perform conversion
    try:
        if args.command:
            result = parser.parse_and_convert(
                args.command, 
                json_data
            )
        else:
            # --key expects "command/subcommand" format
            parts = args.key.split('/', 1) if '/' in args.key else args.key.split('_', 1)
            if len(parts) == 2:
                result = parser.convert(parts[0], parts[1], json_data)
            else:
                result = parser.parse_and_convert(
                    f"show {args.key}",
                    json_data
                )
        
        # Format output
        if args.format == "json":
            output = json.dumps({"result": result}, indent=2)
        else:
            output = result if result else ""
        
        write_output(output, args.output)
        return 0
        
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        if args.verbose:
            import traceback
            traceback.print_exc()
        print(f"Error during conversion: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
