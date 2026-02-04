#!/usr/bin/env python3
"""
Rebuild test data from source device folders.
Ensures all test cases are included and properly structured with base commands and input_options.
"""

import os
import json
import re
from pathlib import Path
from collections import defaultdict

# Source device folders
SOURCE_FOLDERS = {
    "str4-7060x6-512-9": r"C:\Users\shixz\OneDrive - Microsoft\WorkingFolder\AdhocAsks\Investigation\nanali\show-data-collect-tools\str4-7060x6-512-9",
    "str4-7060x6-512-13": r"C:\Users\shixz\OneDrive - Microsoft\WorkingFolder\AdhocAsks\Investigation\nanali\show-data-collect-tools\str4-7060x6-512-13",
    "str2-8102-01": r"C:\Users\shixz\OneDrive - Microsoft\WorkingFolder\AdhocAsks\Investigation\nanali\show-data-collect-tools-v2\str2-8102-01",
    "str4-7060x6-512-1": r"C:\Users\shixz\OneDrive - Microsoft\WorkingFolder\AdhocAsks\Investigation\nanali\show-data-collect-tools-v2\str4-7060x6-512-1",
}

# Command table - base commands with their properties
COMMAND_TABLE = {
    1: {"command": "show buffer_pool watermark", "minArgs": 0, "maxArgs": 0},
    2: {"command": "show buffer_pool persistent-watermark", "minArgs": 0, "maxArgs": 0},
    3: {"command": "show clock", "minArgs": 0, "maxArgs": 0},
    4: {"command": "show clock timezones", "minArgs": 0, "maxArgs": 0},
    5: {"command": "show dropcounters capabilities", "minArgs": 0, "maxArgs": 0},
    6: {"command": "show dropcounters counts", "minArgs": 0, "maxArgs": 0},
    7: {"command": "show headroom-pool watermark", "minArgs": 0, "maxArgs": 0},
    8: {"command": "show headroom-pool persistent-watermark", "minArgs": 0, "maxArgs": 0},
    9: {"command": "show interfaces alias", "minArgs": 0, "maxArgs": 1},
    10: {"command": "show interfaces counters", "minArgs": 0, "maxArgs": 0},
    11: {"command": "show interfaces counters detailed", "minArgs": 1, "maxArgs": 1},
    12: {"command": "show interfaces counters errors", "minArgs": 0, "maxArgs": 0},
    13: {"command": "show interfaces counters fec-histogram", "minArgs": 1, "maxArgs": 1},
    14: {"command": "show interfaces counters fec-stats", "minArgs": 0, "maxArgs": 0},
    15: {"command": "show interfaces counters rates", "minArgs": 0, "maxArgs": 0},
    16: {"command": "show interfaces counters rif", "minArgs": 0, "maxArgs": 1},
    17: {"command": "show interfaces counters trim", "minArgs": 0, "maxArgs": 1},
    18: {"command": "show interfaces description", "minArgs": 0, "maxArgs": 1},
    19: {"command": "show interfaces errors", "minArgs": 1, "maxArgs": 1},
    20: {"command": "show interfaces fec status", "minArgs": 0, "maxArgs": 1},
    21: {"command": "show interfaces flap", "minArgs": 0, "maxArgs": 1},
    22: {"command": "show interfaces naming_mode", "minArgs": 0, "maxArgs": 0},
    23: {"command": "show interfaces neighbor expected", "minArgs": 0, "maxArgs": 0},
    24: {"command": "show interfaces portchannel", "minArgs": 0, "maxArgs": 0},
    25: {"command": "show interfaces status", "minArgs": 0, "maxArgs": 1},
    26: {"command": "show interfaces switchport config", "minArgs": 0, "maxArgs": 0},
    27: {"command": "show interfaces switchport status", "minArgs": 0, "maxArgs": 0},
    28: {"command": "show interfaces transceiver eeprom", "minArgs": 0, "maxArgs": 1},
    29: {"command": "show interfaces transceiver error-status", "minArgs": 0, "maxArgs": 1},
    30: {"command": "show interfaces transceiver info", "minArgs": 0, "maxArgs": 1},
    31: {"command": "show interfaces transceiver lpmode", "minArgs": 0, "maxArgs": 1},
    32: {"command": "show interfaces transceiver pm", "minArgs": 0, "maxArgs": 1},
    33: {"command": "show interfaces transceiver presence", "minArgs": 0, "maxArgs": 1},
    34: {"command": "show interfaces transceiver status", "minArgs": 0, "maxArgs": 1},
    35: {"command": "show ipv6 prefix-list", "minArgs": 0, "maxArgs": 1},
    36: {"command": "show ipv6 interfaces", "minArgs": 0, "maxArgs": 0},
    37: {"command": "show ipv6 route", "minArgs": 0, "maxArgs": -1},
    38: {"command": "show ipv6 protocol", "minArgs": 0, "maxArgs": 0},
    39: {"command": "show ipv6 bgp neighbors", "minArgs": 0, "maxArgs": 2},
    40: {"command": "show ipv6 bgp network", "minArgs": 0, "maxArgs": 2},
    41: {"command": "show ipv6 bgp summary", "minArgs": 0, "maxArgs": 0},
    42: {"command": "show ipv6 link-local-mode", "minArgs": 0, "maxArgs": 0},
    43: {"command": "show ipv6 fib", "minArgs": 0, "maxArgs": 1},
    44: {"command": "show lldp neighbors", "minArgs": 0, "maxArgs": 1},
    45: {"command": "show lldp table", "minArgs": 0, "maxArgs": 0},
    46: {"command": "show mac", "minArgs": 0, "maxArgs": 0},
    47: {"command": "show mac aging_time", "minArgs": 0, "maxArgs": 0},
    48: {"command": "show mmu", "minArgs": 0, "maxArgs": 0},
    49: {"command": "show ndp", "minArgs": 0, "maxArgs": 1},
    50: {"command": "show processes summary", "minArgs": 0, "maxArgs": 0},
    51: {"command": "show processes cpu", "minArgs": 0, "maxArgs": 0},
    52: {"command": "show processes memory", "minArgs": 0, "maxArgs": 0},
    53: {"command": "show queue counters", "minArgs": 0, "maxArgs": 1},
    54: {"command": "show queue persistent-watermark", "minArgs": 0, "maxArgs": 0},
    55: {"command": "show queue persistent-watermark all", "minArgs": 0, "maxArgs": 0},
    56: {"command": "show queue persistent-watermark multicast", "minArgs": 0, "maxArgs": 0},
    57: {"command": "show queue persistent-watermark unicast", "minArgs": 0, "maxArgs": 0},
    58: {"command": "show queue watermark", "minArgs": 0, "maxArgs": 0},
    59: {"command": "show queue watermark all", "minArgs": 0, "maxArgs": 0},
    60: {"command": "show queue watermark multicast", "minArgs": 0, "maxArgs": 0},
    61: {"command": "show queue watermark unicast", "minArgs": 0, "maxArgs": 0},
    62: {"command": "show queue wredcounters", "minArgs": 0, "maxArgs": 1},
    63: {"command": "show reboot-cause history", "minArgs": 0, "maxArgs": 0},
    64: {"command": "show services", "minArgs": 0, "maxArgs": 0},
    65: {"command": "show srv6 stats", "minArgs": 0, "maxArgs": 1},
    66: {"command": "show system-memory", "minArgs": 0, "maxArgs": 0},
    67: {"command": "show uptime", "minArgs": 0, "maxArgs": 0},
    68: {"command": "show version", "minArgs": 0, "maxArgs": 0},
    69: {"command": "show vlan brief", "minArgs": 0, "maxArgs": 0},
    70: {"command": "show watermark telemetry interval", "minArgs": 0, "maxArgs": 0},
}


def strip_cli_command_prompt(content):
    """Strip command prompt and comment lines from CLI output.
    
    CLI output files often start with lines like:
    $ show interfaces description Ethernet0
    # Original: show interfaces description Ethernet252
    
    This function removes leading lines that start with '$ ' or '# Original:'.
    """
    if not content:
        return content
    
    lines = content.split('\n')
    
    # Remove leading prompt and comment lines
    while lines:
        first_line = lines[0]
        if first_line.startswith('$ ') or first_line.startswith('# Original:'):
            lines.pop(0)
        else:
            break
    
    return '\n'.join(lines)


def extract_json_from_gnmi_rpc(content):
    """Extract JSON data from gNMI RPC text output.
    
    The raw gnmi output looks like:
    == getResponse:
    notification: <
      ...
      update: <
        path: <...>
        val: <
          json_ietf_val: "{\\"key\\": \\"value\\", ...}"
        >
      >
    >
    
    The JSON is stored with escaped quotes and newlines:
    - \\" becomes "
    - \\n becomes newline
    
    We need to:
    1. Extract the raw escaped string from json_ietf_val or json_val field
    2. Decode the escape sequences
    3. Return the decoded JSON string
    """
    if not content:
        return None
    
    # Find json_ietf_val or json_val
    for field_name in ['json_ietf_val', 'json_val']:
        idx = content.find(field_name + ':')
        if idx == -1:
            continue
            
        # Skip past "json_ietf_val:" or "json_val:"
        after_colon = content[idx + len(field_name) + 1:]
        after_colon = after_colon.lstrip()  # Remove leading whitespace
        
        # Check for opening quote
        if not after_colon or after_colon[0] not in '"\'':
            continue
            
        quote_char = after_colon[0]
        json_content = after_colon[1:]  # Skip opening quote
        
        # Find the end of the JSON - look for closing brace or bracket followed by quote
        # JSON objects end with } and arrays end with ]
        end_obj = json_content.rfind('}' + quote_char)
        end_arr = json_content.rfind(']' + quote_char)
        
        if end_obj < 0 and end_arr < 0:
            continue
            
        end_idx = max(end_obj, end_arr)
        if end_idx < 0:
            continue
            
        # Extract the raw escaped string (including the final } or ])
        raw_escaped = json_content[:end_idx + 1]
        
        # Decode escape sequences by treating it as a JSON string value
        try:
            decoded = json.loads('"' + raw_escaped + '"')
        except json.JSONDecodeError:
            # Try with different quote handling
            try:
                # Sometimes the escaping is different
                fixed = raw_escaped.replace('\\"', '"').replace('\\n', '\n')
                json.loads(fixed)
                decoded = fixed
            except json.JSONDecodeError:
                continue
        
        # Validate the decoded string is valid JSON
        try:
            json.loads(decoded)
            return decoded
        except json.JSONDecodeError:
            continue
    
    # Fallback: try to find any JSON object/array in the content
    # Look for content between first { and last } or first [ and last ]
    for start_char, end_char in [('{', '}'), ('[', ']')]:
        start = content.find(start_char)
        end = content.rfind(end_char)
        if start != -1 and end != -1 and end > start:
            json_str = content[start:end+1]
            try:
                json.loads(json_str)
                return json_str
            except json.JSONDecodeError:
                pass
    
    return None


def normalize_command(cmd):
    """Normalize command for comparison.
    
    Handles naming differences between CLI and GNMI:
    - CLI: show queue counters --nonzero --all
    - GNMI: show queue counters nonzero true all true
    Both should normalize to the same key.
    
    Also handles IPv6 address format differences:
    - CLI: fc00::182 or 2064:100:0:a1::/128
    - GNMI filename: fc00-182 or 2064-100-0-a1-128 (colons and slashes become dashes)
    """
    # Replace hyphens, underscores, colons, and slashes with spaces, then lowercase
    # This ensures IPv6 addresses match: fc00::182 -> "fc00  182" and fc00-182 -> "fc00 182"
    result = cmd.lower().replace("-", " ").replace("_", " ").replace(":", " ").replace("/", " ")
    
    # Split into tokens and remove "true" tokens (GNMI uses flag=true, CLI uses --flag)
    tokens = result.split()
    tokens = [t for t in tokens if t != "true"]
    
    # Sort tokens for consistent matching
    return " ".join(sorted(tokens))


def normalize_command_ordered(cmd):
    """Normalize command while preserving token order (for prefix matching)."""
    result = cmd.lower().replace("-", " ").replace("_", " ")
    tokens = result.split()
    tokens = [t for t in tokens if t != "true"]
    return " ".join(tokens)


def cli_filename_to_command(filename):
    """Convert CLI filename to command string.
    
    Note: This is a fallback. The actual command should be extracted from
    the first line of the CLI file ($ show ...) when available.
    """
    # Remove .txt extension
    name = filename.replace(".txt", "")
    # Replace underscores with spaces
    cmd = name.replace("_", " ")
    return cmd


def extract_command_from_cli_content(content):
    """Extract the original command from CLI output.
    
    CLI files start with a command prompt line like:
    $ show ipv6 bgp neighbors fc00::182 advertised-routes
    
    Returns the command or None if not found.
    """
    if not content:
        return None
    
    lines = content.split('\n')
    if lines and lines[0].startswith('$ '):
        return lines[0][2:].strip()  # Remove '$ ' prefix
    return None


def gnmi_filename_to_command(filename):
    """Convert GNMI filename to command string."""
    # Remove .txt extension and gnmi-get- prefix
    name = filename.replace(".txt", "")
    if name.startswith("gnmi-get-"):
        name = name[9:]  # Remove "gnmi-get-"
    # Replace hyphens with spaces
    cmd = "show " + name.replace("-", " ")
    return cmd


def find_base_command_index(full_command):
    """Find the base command index for a full command (which may include arguments)."""
    # Use ordered normalization to preserve token order for prefix matching
    normalized = normalize_command_ordered(full_command)
    
    # Try to match from longest to shortest
    best_match = None
    best_match_len = 0
    
    for idx, info in COMMAND_TABLE.items():
        base_cmd_norm = normalize_command_ordered(info["command"])
        if normalized.startswith(base_cmd_norm):
            # Check if it's a word boundary match (not partial word)
            remainder = normalized[len(base_cmd_norm):]
            if remainder == "" or remainder.startswith(" "):
                if len(base_cmd_norm) > best_match_len:
                    best_match = idx
                    best_match_len = len(base_cmd_norm)
    
    return best_match


def extract_input_options(full_command, base_command):
    """Extract input_options from full command given the base command."""
    # Normalize both for comparison
    full_norm = full_command.strip()
    base_norm = base_command.strip()
    
    # Find where the base command ends
    full_lower = full_norm.lower().replace("-", "_")
    base_lower = base_norm.lower().replace("-", "_")
    
    if full_lower.startswith(base_lower):
        remainder = full_norm[len(base_norm):].strip()
        return remainder
    
    # Try a more flexible match
    full_parts = full_norm.split()
    base_parts = base_norm.split()
    
    # Match base parts
    matched = 0
    for i, base_part in enumerate(base_parts):
        if i < len(full_parts):
            if full_parts[i].lower().replace("-", "_") == base_part.lower().replace("-", "_"):
                matched += 1
            else:
                break
        else:
            break
    
    if matched == len(base_parts):
        return " ".join(full_parts[matched:])
    
    return ""


def scan_source_folders():
    """Scan all source folders and collect available test data."""
    # Structure: {command_index: {input_options: {device: {cli_file, gnmi_file, cli_content, gnmi_content}}}}
    test_data = defaultdict(lambda: defaultdict(lambda: {}))
    
    for device_name, folder_path in SOURCE_FOLDERS.items():
        cli_folder = os.path.join(folder_path, "cli_outputs")
        gnmi_folder = os.path.join(folder_path, "gnmi-outputs")
        
        if not os.path.exists(cli_folder):
            print(f"Warning: CLI folder not found: {cli_folder}")
            continue
        
        # Scan GNMI files first (for matching later)
        gnmi_files = {}
        if os.path.exists(gnmi_folder):
            for f in os.listdir(gnmi_folder):
                if f.endswith(".txt"):
                    cmd = gnmi_filename_to_command(f)
                    gnmi_files[f] = cmd
        
        # Scan CLI files and extract actual commands from file content
        for cli_file in os.listdir(cli_folder):
            if not cli_file.endswith(".txt"):
                continue
            
            # Read CLI file to extract actual command from first line
            cli_path = os.path.join(cli_folder, cli_file)
            with open(cli_path, 'r', encoding='utf-8', errors='replace') as f:
                cli_raw_content = f.read()
            
            # Try to extract command from the $ prompt line
            cli_cmd = extract_command_from_cli_content(cli_raw_content)
            if not cli_cmd:
                # Fallback to filename-based parsing
                cli_cmd = cli_filename_to_command(cli_file)
            
            cmd_index = find_base_command_index(cli_cmd)
            if cmd_index is None:
                print(f"Warning: No base command found for: {cli_cmd}")
                continue
            
            base_command = COMMAND_TABLE[cmd_index]["command"]
            input_options = extract_input_options(cli_cmd, base_command)
            
            # Find matching GNMI file
            gnmi_file = None
            for gf, gcmd in gnmi_files.items():
                if normalize_command(gcmd) == normalize_command(cli_cmd):
                    gnmi_file = gf
                    break
            
            # Strip command prompt and comment lines from CLI content
            cli_content = strip_cli_command_prompt(cli_raw_content)
            
            # Read GNMI content
            gnmi_content = None
            if gnmi_file:
                gnmi_path = os.path.join(gnmi_folder, gnmi_file)
                if os.path.exists(gnmi_path):
                    with open(gnmi_path, 'r', encoding='utf-8', errors='replace') as f:
                        gnmi_content = f.read()
            
            # Store data
            test_data[cmd_index][input_options][device_name] = {
                "cli_file": cli_file,
                "gnmi_file": gnmi_file,
                "cli_content": cli_content,
                "gnmi_content": gnmi_content,
            }
    
    return test_data


def command_to_filename_part(command):
    """Convert command to filename-friendly format."""
    # Keep 'show' prefix and replace spaces/hyphens with underscores
    cmd = command
    return cmd.replace(" ", "_").replace("-", "_")


def generate_test_data_files(test_data, output_dir):
    """Generate test data JSON files from scanned data."""
    import sys
    sys.path.insert(0, str(Path(__file__).parent.parent / "gnmi_cli_lib"))
    
    # The module is gnmi_cli_lib inside gnmi_cli_lib folder
    # But we don't actually need the converter for generating test data
    # Just generate the JSON files directly
    
    os.makedirs(output_dir, exist_ok=True)
    
    for cmd_index in sorted(test_data.keys()):
        base_command = COMMAND_TABLE[cmd_index]["command"]
        
        # Build test cases list
        test_cases = []
        
        for input_options in sorted(test_data[cmd_index].keys()):
            devices_data = {}
            
            for device_name, data in test_data[cmd_index][input_options].items():
                gnmi_content = data.get("gnmi_content")
                cli_content = data.get("cli_content")
                cli_file = data.get("cli_file")
                gnmi_file = data.get("gnmi_file")
                
                # Parse GNMI JSON
                json_string = None
                json_error = None
                
                if gnmi_content:
                    # First try to extract JSON from gNMI RPC text format
                    extracted_json = extract_json_from_gnmi_rpc(gnmi_content)
                    if extracted_json:
                        gnmi_content_parsed = extracted_json
                    else:
                        gnmi_content_parsed = gnmi_content
                    
                    try:
                        # Try to parse as JSON
                        gnmi_json = json.loads(gnmi_content_parsed)
                        
                        # Store the extracted JSON string (not the original RPC text)
                        json_string = gnmi_content_parsed
                    except json.JSONDecodeError as e:
                        json_error = f"JSON parse error: {e}"
                        json_string = None
                
                devices_data[device_name] = {
                    "cli_file": cli_file,
                    "gnmi_file": gnmi_file,
                    "json_string": json_string,
                    "json_error": json_error,
                    "cli_string": cli_content,
                    "cli_error": None,
                }
            
            if devices_data:
                test_cases.append({
                    "input_options": input_options,
                    "devices": devices_data,
                })
        
        if test_cases:
            # Create JSON structure
            output_data = {
                "command": base_command,
                "test_cases": test_cases,
            }
            
            # Generate filename
            filename_part = command_to_filename_part(base_command)
            filename = f"data_{cmd_index:02d}_{filename_part}.json"
            filepath = os.path.join(output_dir, filename)
            
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(output_data, f, indent=2)
            
            print(f"Generated: {filename} ({len(test_cases)} test cases)")


def analyze_test_data():
    """Analyze what test data is available from source folders."""
    print("Scanning source folders...")
    test_data = scan_source_folders()
    
    print("\n" + "="*80)
    print("AVAILABLE TEST DATA BY COMMAND")
    print("="*80)
    
    for cmd_index in sorted(test_data.keys()):
        base_command = COMMAND_TABLE[cmd_index]["command"]
        min_args = COMMAND_TABLE[cmd_index]["minArgs"]
        max_args = COMMAND_TABLE[cmd_index]["maxArgs"]
        
        print(f"\n[{cmd_index:02d}] {base_command} (minArgs={min_args}, maxArgs={max_args})")
        
        for input_options in sorted(test_data[cmd_index].keys()):
            devices = list(test_data[cmd_index][input_options].keys())
            opts_display = f"'{input_options}'" if input_options else "(no args)"
            print(f"  input_options={opts_display}")
            print(f"    Devices: {', '.join(sorted(devices))}")
    
    # Summary
    print("\n" + "="*80)
    print("SUMMARY")
    print("="*80)
    
    total_test_cases = 0
    for cmd_index in test_data:
        for input_options in test_data[cmd_index]:
            total_test_cases += 1
    
    print(f"Commands with data: {len(test_data)}")
    print(f"Total test cases (unique command + input_options combinations): {total_test_cases}")
    
    # Check for missing commands
    missing = set(COMMAND_TABLE.keys()) - set(test_data.keys())
    if missing:
        print(f"\nCommands with NO test data: {sorted(missing)}")
        for idx in sorted(missing):
            print(f"  [{idx:02d}] {COMMAND_TABLE[idx]['command']}")
    
    return test_data


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--generate", action="store_true", help="Generate new test data files")
    parser.add_argument("--output-dir", default="test_data_new", help="Output directory for generated files")
    args = parser.parse_args()
    
    test_data = analyze_test_data()
    
    if args.generate:
        output_dir = os.path.join(os.path.dirname(__file__), args.output_dir)
        print(f"\nGenerating test data files to: {output_dir}")
        generate_test_data_files(test_data, output_dir)

