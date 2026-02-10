#!/usr/bin/env python3
"""
Import test data from a new device into the test_data folder.

This script:
1. Scans gNMI and CLI output directories
2. Validates 1:1 mapping between gNMI and CLI files (no orphans)
3. Updates existing test data JSON files with the new device data

Usage:
    python import_device_data.py <device_name> <source_directory>
    
Example:
    python import_device_data.py str2-8102-01 "../Investigation/nanali/show-data-collect-tools-v2/str2-8102-01"
"""
import json
import sys
import re
from pathlib import Path
from typing import Dict, List, Tuple, Optional

BASE_DIR = Path(__file__).resolve().parent.parent
TEST_DATA_DIR = BASE_DIR / "fixtures" / "commands"


def read_file_content(filepath: Path) -> Optional[str]:
    """Read file content, return None if file doesn't exist."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.read()
    except FileNotFoundError:
        return None
    except Exception as e:
        return f"ERROR: {e}"


def extract_cli_from_source(content: str) -> str:
    """Extract CLI output from source file, removing the command header line."""
    if content is None:
        return None
    
    lines = content.split('\n')
    # Skip any lines that start with '$' (command prompt lines)
    result_lines = []
    for line in lines:
        if line.startswith('$ '):
            continue
        result_lines.append(line)
    
    return '\n'.join(result_lines)


def extract_json_from_gnmi(content: str) -> Tuple[Optional[str], Optional[str]]:
    """Extract JSON from gNMI output file.
    
    Returns (json_string, error_message)
    """
    if content is None:
        return None, "File not found"
    
    # Find json_ietf_val: "..." and extract the content
    if 'json_ietf_val:' in content:
        start_marker = 'json_ietf_val:'
        start_idx = content.find(start_marker)
        if start_idx != -1:
            # Find first quote after marker
            quote_start = content.find('"', start_idx + len(start_marker))
            if quote_start != -1:
                remaining = content[quote_start+1:]
                
                # Extract content between quotes, handling escapes
                result = []
                i = 0
                while i < len(remaining):
                    if remaining[i] == '\\' and i + 1 < len(remaining):
                        next_char = remaining[i + 1]
                        if next_char == '"':
                            result.append('"')
                            i += 2
                            continue
                        elif next_char == 'n':
                            result.append('\n')
                            i += 2
                            continue
                        elif next_char == 't':
                            result.append('\t')
                            i += 2
                            continue
                        elif next_char == '\\':
                            result.append('\\')
                            i += 2
                            continue
                    elif remaining[i] == '"':
                        # End of JSON string value
                        break
                    result.append(remaining[i])
                    i += 1
                
                json_str = ''.join(result)
                try:
                    data = json.loads(json_str)
                    return json.dumps(data, separators=(',', ':')), None
                except json.JSONDecodeError as e:
                    return None, f"JSON parse error: {e}"
    
    # Fallback: try to find JSON object directly
    start = content.find('{')
    if start == -1:
        return None, "No JSON found in file"
    
    # Find matching closing brace
    depth = 0
    for i, c in enumerate(content[start:], start):
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                json_str = content[start:i+1]
                try:
                    data = json.loads(json_str)
                    return json.dumps(data, separators=(',', ':')), None
                except json.JSONDecodeError as e:
                    return None, f"JSON parse error: {e}"
    
    return None, "No valid JSON found"


def gnmi_filename_to_cli_filename(gnmi_name: str) -> str:
    """
    Convert gNMI filename to CLI filename.
    
    Key patterns to handle:
    - gNMI uses hyphens, CLI uses underscores for most parts
    - Some CLI commands preserve hyphens (headroom-pool, fec-stats, error-status, etc.)
    - Boolean params: -nonzero-true -> --nonzero
    - Named params: -counter_type-VALUE -> --counter_type VALUE
    """
    # Remove gnmi-get- prefix
    name = gnmi_name.replace('gnmi-get-', '')
    name = name.replace('.txt', '')
    
    # Handle boolean parameters FIRST (before hyphen conversion)
    name = re.sub(r'-nonzero-true', '-PARAM_NONZERO', name)
    name = re.sub(r'-all-true', '-PARAM_ALL', name)
    name = re.sub(r'-trim-true', '-PARAM_TRIM', name)
    name = re.sub(r'-printall-true', '-PARAM_PRINTALL', name)
    name = re.sub(r'-dom-true', '-PARAM_DOM', name)
    
    # Handle period parameter
    name = re.sub(r'-period-(\d+)', r'-PARAM_PERIOD_\1', name)
    
    # Handle named parameters
    name = re.sub(r'-counter_type-([A-Z_]+)', r'-PARAM_COUNTER_TYPE_\1', name)
    name = re.sub(r'-group-([A-Z]+)', r'-PARAM_GROUP_\1', name)
    name = re.sub(r'-interface-([A-Za-z0-9]+)', r'-PARAM_INTERFACE_\1', name)
    name = re.sub(r'-iface-([A-Za-z0-9]+)', r'-PARAM_IFACE_\1', name)
    name = re.sub(r'-vrf-([A-Za-z]+)', r'-PARAM_VRF_\1', name)
    
    # Preserve hyphenated subcommands that CLI keeps as hyphens
    # These need to be protected before general hyphen->underscore conversion
    hyphenated_commands = [
        'persistent-watermark',
        'headroom-pool',
        'fec-stats',
        'fec-histogram',
        'error-status',
        'naming_mode',  # Already has underscore in gNMI
        'aging-time',
        'link-local-mode',
        'prefix-list',
        'reboot-cause',
        'system-memory',
        'advertised-routes',
        'received-routes',
        'longer-prefixes',
    ]
    
    for cmd in hyphenated_commands:
        placeholder = cmd.replace('-', '_HYPHEN_')
        name = name.replace(cmd, placeholder)
    
    # Convert remaining hyphens to underscores
    name = name.replace('-', '_')
    
    # Restore hyphenated commands
    name = name.replace('_HYPHEN_', '-')
    
    # Restore parameter placeholders to proper CLI format
    name = name.replace('_PARAM_NONZERO', '_--nonzero')
    name = name.replace('_PARAM_ALL', '_--all')
    name = name.replace('_PARAM_TRIM', '_--trim')
    name = name.replace('_PARAM_PRINTALL', '_--printall')
    name = name.replace('_PARAM_DOM', '_--dom')
    name = re.sub(r'_PARAM_PERIOD_(\d+)', r'_--period_\1', name)
    name = re.sub(r'_PARAM_COUNTER_TYPE_([A-Z_]+)', r'_--counter_type_\1', name)
    name = re.sub(r'_PARAM_GROUP_([A-Z]+)', r'_--group_\1', name)
    name = re.sub(r'_PARAM_INTERFACE_([A-Za-z0-9]+)', r'_--interface_\1', name)
    name = re.sub(r'_PARAM_IFACE_([A-Za-z0-9]+)', r'_--iface_\1', name)
    name = re.sub(r'_PARAM_VRF_([A-Za-z]+)', r'_vrf_\1', name)
    
    return 'show_' + name + '.txt'


def cli_filename_to_gnmi_patterns(cli_name: str) -> List[str]:
    """
    Convert CLI filename to possible gNMI filename patterns.
    Returns a list of possible matches (since mapping isn't always 1:1 reversible).
    """
    patterns = []
    
    # Remove .txt
    name = cli_name.replace('.txt', '')
    
    # Replace show_ with gnmi-get-
    name = name.replace('show_', 'gnmi-get-')
    
    # Convert --param to -param-true
    name = re.sub(r'_--nonzero', '-nonzero-true', name)
    name = re.sub(r'_--all', '-all-true', name)
    name = re.sub(r'_--trim', '-trim-true', name)
    name = re.sub(r'_--printall', '-printall-true', name)
    name = re.sub(r'_--dom', '-dom-true', name)
    name = re.sub(r'_--period_(\d+)', r'-period-\1', name)
    name = re.sub(r'_--counter_type_', r'-counter_type-', name)
    name = re.sub(r'_--group_', r'-group-', name)
    name = re.sub(r'_--interface_', r'-interface-', name)
    name = re.sub(r'_--iface_', r'-iface-', name)
    name = re.sub(r'_vrf_', r'-vrf-', name)
    
    # Convert underscores to hyphens
    name = name.replace('_', '-')
    
    patterns.append(name + '.txt')
    
    return patterns


def build_mapping(gnmi_dir: Path, cli_dir: Path) -> Tuple[Dict[str, str], List[str], List[str]]:
    """
    Build mapping between gNMI and CLI files.
    
    Returns:
        - mapping: dict of gnmi_file -> cli_file
        - orphan_gnmi: list of gNMI files without CLI matches
        - orphan_cli: list of CLI files without gNMI matches
    """
    gnmi_files = {f.name for f in gnmi_dir.glob('*.txt')}
    cli_files = {f.name for f in cli_dir.glob('*.txt')}
    
    mapping = {}
    matched_cli = set()
    orphan_gnmi = []
    
    for gnmi_file in sorted(gnmi_files):
        cli_file = gnmi_filename_to_cli_filename(gnmi_file)
        
        if cli_file in cli_files:
            mapping[gnmi_file] = cli_file
            matched_cli.add(cli_file)
        else:
            orphan_gnmi.append(gnmi_file)
    
    orphan_cli = sorted(cli_files - matched_cli)
    
    return mapping, orphan_gnmi, orphan_cli


def find_test_data_file_for_command(command: str, options: str) -> Optional[Path]:
    """
    Find the test data JSON file that contains this command and options.
    """
    for json_file in TEST_DATA_DIR.glob('data_*.json'):
        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            if data.get('command') == command:
                for test_case in data.get('test_cases', []):
                    if test_case.get('input_options', '') == options:
                        return json_file
        except Exception:
            continue
    
    return None


def parse_cli_command(cli_filename: str) -> Tuple[str, str]:
    """
    Parse CLI filename to extract base command and options.
    
    Examples:
        show_buffer_pool_watermark.txt -> ("show buffer_pool watermark", "")
        show_queue_counters_--nonzero.txt -> ("show queue counters", "--nonzero")
        show_interfaces_counters_Ethernet252.txt -> ("show interfaces counters", "Ethernet252")
    """
    name = cli_filename.replace('.txt', '')
    
    # Split by underscore
    parts = name.split('_')
    
    # Find where options start (options start with -- or are interface names)
    command_parts = []
    option_parts = []
    in_options = False
    
    for part in parts:
        if part.startswith('--') or in_options:
            in_options = True
            option_parts.append(part)
        elif re.match(r'^(Ethernet\d+|PortChannel\d+|Vlan\d+)$', part):
            # Interface name is an argument
            option_parts.append(part)
        elif re.match(r'^(fc00|fe80|2064|DEFAULT).*$', part, re.IGNORECASE):
            # IPv6 prefix or name
            option_parts.append(part)
        else:
            command_parts.append(part)
    
    command = ' '.join(command_parts)
    options = ' '.join(option_parts)
    
    return command, options


def update_test_data(device_name: str, gnmi_dir: Path, cli_dir: Path, mapping: Dict[str, str], dry_run: bool = False) -> Tuple[int, int, List[str], List[str]]:
    """
    Update test data JSON files with new device data.
    
    Returns:
        - updated_count: number of test cases updated
        - new_count: number of new test cases added
        - errors: list of error messages
        - unmatched: list of gnmi files not matching existing test data
    """
    updated_count = 0
    new_count = 0
    errors = []
    unmatched = []
    
    for gnmi_file, cli_file in sorted(mapping.items()):
        # Read source files
        gnmi_content = read_file_content(gnmi_dir / gnmi_file)
        cli_content = read_file_content(cli_dir / cli_file)
        
        # Extract data
        json_str, json_err = extract_json_from_gnmi(gnmi_content)
        cli_str = extract_cli_from_source(cli_content)
        
        # Find which test data file this belongs to
        found = False
        for json_file in TEST_DATA_DIR.glob('data_*.json'):
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                # Check if this file has matching gnmi_file
                updated = False
                for test_case in data.get('test_cases', []):
                    if test_case.get('gnmi_file') == gnmi_file:
                        # Found matching test case - add device data
                        if 'devices' not in test_case:
                            test_case['devices'] = {}
                        
                        test_case['devices'][device_name] = {
                            'json_string': json_str,
                            'json_error': json_err,
                            'cli_string': cli_str,
                            'cli_error': None
                        }
                        updated = True
                        found = True
                        updated_count += 1
                        break
                
                if updated:
                    if not dry_run:
                        with open(json_file, 'w', encoding='utf-8') as f:
                            json.dump(data, f, indent=2, ensure_ascii=False)
                    break
                    
            except Exception as e:
                errors.append(f"Error processing {json_file.name}: {e}")
        
        if not found:
            unmatched.append(gnmi_file)
    
    return updated_count, new_count, errors, unmatched


def determine_base_command(gnmi_file: str) -> str:
    """
    Determine the base show command from a gnmi filename.
    
    Examples:
        gnmi-get-interfaces-counters-detailed-Ethernet252.txt -> show interfaces counters detailed
        gnmi-get-queue-counters-Ethernet252.txt -> show queue counters
        gnmi-get-ipv6-bgp-neighbors-fc00-42.txt -> show ipv6 bgp neighbors
    """
    name = gnmi_file.replace('gnmi-get-', '').replace('.txt', '')
    
    # Remove boolean parameters
    name = re.sub(r'-nonzero-true', '', name)
    name = re.sub(r'-all-true', '', name)
    name = re.sub(r'-trim-true', '', name)
    name = re.sub(r'-printall-true', '', name)
    name = re.sub(r'-dom-true', '', name)
    name = re.sub(r'-period-\d+', '', name)
    
    # Split by hyphen
    parts = name.split('-')
    
    # Find where interface/argument starts
    command_parts = []
    for part in parts:
        # Stop at interface names or IP addresses
        if re.match(r'^(Ethernet\d+|PortChannel\d+|Vlan\d+|fc00|fe80|2064|DEFAULT)', part, re.IGNORECASE):
            break
        command_parts.append(part)
    
    # Convert to command format
    command = 'show ' + ' '.join(command_parts)
    
    # Handle special cases with underscores
    command = command.replace('buffer_pool', 'buffer_pool')
    command = command.replace('dropcounters', 'dropcounters')
    
    # Handle hyphenated subcommands
    command = command.replace('persistent watermark', 'persistent-watermark')
    command = command.replace('headroom pool', 'headroom-pool')
    command = command.replace('error status', 'error-status')
    command = command.replace('fec histogram', 'fec-histogram')
    command = command.replace('fec stats', 'fec-stats')
    command = command.replace('aging time', 'aging-time')
    command = command.replace('link local mode', 'link-local-mode')
    command = command.replace('prefix list', 'prefix-list')
    command = command.replace('reboot cause', 'reboot-cause')
    command = command.replace('system memory', 'system-memory')
    
    return command


def create_new_test_data_files(device_name: str, gnmi_dir: Path, cli_dir: Path, 
                                unmatched: List[str], mapping: Dict[str, str],
                                dry_run: bool = False) -> Tuple[int, List[str]]:
    """
    Create new test data files for unmatched gNMI files.
    
    Groups unmatched files by base command and creates appropriate test data files.
    
    Returns:
        - created_count: number of new files created
        - errors: list of error messages
    """
    created_count = 0
    errors = []
    
    # Group by base command
    command_groups = {}
    for gnmi_file in unmatched:
        base_cmd = determine_base_command(gnmi_file)
        if base_cmd not in command_groups:
            command_groups[base_cmd] = []
        command_groups[base_cmd].append(gnmi_file)
    
    # Find next available file number
    existing_files = list(TEST_DATA_DIR.glob('data_*.json'))
    max_num = 0
    for f in existing_files:
        match = re.match(r'data_(\d+)_', f.name)
        if match:
            max_num = max(max_num, int(match.group(1)))
    
    next_num = max_num + 1
    
    for base_cmd, gnmi_files in sorted(command_groups.items()):
        # Create test data structure
        test_cases = []
        
        for gnmi_file in gnmi_files:
            cli_file = mapping[gnmi_file]
            
            # Read source files
            gnmi_content = read_file_content(gnmi_dir / gnmi_file)
            cli_content = read_file_content(cli_dir / cli_file)
            
            # Extract data
            json_str, json_err = extract_json_from_gnmi(gnmi_content)
            cli_str = extract_cli_from_source(cli_content)
            
            # Determine input options from gnmi filename
            input_options = extract_input_options(gnmi_file)
            
            test_case = {
                'input_options': input_options,
                'cli_file': cli_file,
                'gnmi_file': gnmi_file,
                'devices': {
                    device_name: {
                        'json_string': json_str,
                        'json_error': json_err,
                        'cli_string': cli_str,
                        'cli_error': None
                    }
                }
            }
            test_cases.append(test_case)
        
        # Determine the full command for the filename
        # Use the first gnmi file to extract the interface name
        first_gnmi = gnmi_files[0]
        interface_match = re.search(r'(Ethernet\d+|PortChannel\d+)', first_gnmi)
        if interface_match:
            full_cmd = f"{base_cmd} {interface_match.group(1)}"
        else:
            full_cmd = base_cmd
        
        data = {
            'command': full_cmd,
            'test_cases': test_cases
        }
        
        # Generate filename
        safe_name = full_cmd.replace(' ', '_').replace('-', '_')
        filename = f"data_{next_num:02d}_{safe_name}.json"
        filepath = TEST_DATA_DIR / filename
        
        if not dry_run:
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
        
        print(f"    Created: {filename} ({len(test_cases)} test cases)")
        created_count += 1
        next_num += 1
    
    return created_count, errors


def extract_input_options(gnmi_file: str) -> str:
    """
    Extract input options from gNMI filename.
    
    Examples:
        gnmi-get-interfaces-counters-detailed-Ethernet252-period-2.txt -> "--period 2"
        gnmi-get-queue-counters-Ethernet252-nonzero-true.txt -> "--nonzero"
        gnmi-get-interfaces-transceiver-eeprom-Ethernet252-dom-true.txt -> "--dom"
    """
    options = []
    
    if '-nonzero-true' in gnmi_file:
        options.append('--nonzero')
    if '-all-true' in gnmi_file:
        options.append('--all')
    if '-trim-true' in gnmi_file:
        options.append('--trim')
    if '-printall-true' in gnmi_file:
        options.append('--printall')
    if '-dom-true' in gnmi_file:
        options.append('--dom')
    
    period_match = re.search(r'-period-(\d+)', gnmi_file)
    if period_match:
        options.append(f'--period {period_match.group(1)}')
    
    return ' '.join(options)


def analyze_unmapped_files(orphan_gnmi: List[str], orphan_cli: List[str]) -> Dict[str, List[str]]:
    """
    Analyze orphan files to suggest fixes.
    """
    analysis = {
        'gnmi_suggestions': [],
        'cli_suggestions': [],
        'possible_matches': []
    }
    
    for gnmi_file in orphan_gnmi:
        suggested_cli = gnmi_filename_to_cli_filename(gnmi_file)
        analysis['gnmi_suggestions'].append(f"  {gnmi_file} -> Expected CLI: {suggested_cli}")
    
    for cli_file in orphan_cli:
        patterns = cli_filename_to_gnmi_patterns(cli_file)
        analysis['cli_suggestions'].append(f"  {cli_file} -> Expected gNMI: {patterns}")
    
    return analysis


def main():
    if len(sys.argv) < 3:
        print("Usage: python import_device_data.py <device_name> <source_directory> [--dry-run]")
        print("\nExample:")
        print('  python import_device_data.py str2-8102-01 "../Investigation/nanali/show-data-collect-tools-v2/str2-8102-01"')
        sys.exit(1)
    
    device_name = sys.argv[1]
    source_dir = Path(sys.argv[2]).resolve()
    dry_run = '--dry-run' in sys.argv
    
    print(f"=" * 80)
    print(f"Importing device data: {device_name}")
    print(f"Source directory: {source_dir}")
    print(f"Test data directory: {TEST_DATA_DIR}")
    print(f"Dry run: {dry_run}")
    print(f"=" * 80)
    
    # Validate directories
    gnmi_dir = source_dir / 'gnmi-outputs'
    cli_dir = source_dir / 'cli_outputs'
    
    if not gnmi_dir.exists():
        print(f"\nERROR: gNMI directory not found: {gnmi_dir}")
        sys.exit(1)
    
    if not cli_dir.exists():
        print(f"\nERROR: CLI directory not found: {cli_dir}")
        sys.exit(1)
    
    # Build mapping
    print(f"\n{'=' * 40}")
    print("STEP 1: Building file mapping...")
    print(f"{'=' * 40}")
    
    mapping, orphan_gnmi, orphan_cli = build_mapping(gnmi_dir, cli_dir)
    
    print(f"\n  Mapped pairs: {len(mapping)}")
    print(f"  Orphan gNMI files: {len(orphan_gnmi)}")
    print(f"  Orphan CLI files: {len(orphan_cli)}")
    
    # Report orphans
    if orphan_gnmi:
        print(f"\n  ORPHAN gNMI files (no matching CLI):")
        for f in orphan_gnmi[:10]:  # Show first 10
            expected = gnmi_filename_to_cli_filename(f)
            print(f"    - {f}")
            print(f"      Expected CLI: {expected}")
        if len(orphan_gnmi) > 10:
            print(f"    ... and {len(orphan_gnmi) - 10} more")
    
    if orphan_cli:
        print(f"\n  ORPHAN CLI files (no matching gNMI):")
        for f in orphan_cli[:10]:
            patterns = cli_filename_to_gnmi_patterns(f)
            print(f"    - {f}")
            print(f"      Expected gNMI: {patterns[0]}")
        if len(orphan_cli) > 10:
            print(f"    ... and {len(orphan_cli) - 10} more")
    
    # Stop if orphans exist
    if orphan_gnmi or orphan_cli:
        print(f"\n{'=' * 40}")
        print("ERROR: Cannot import with orphan files!")
        print("Please ensure 1:1 mapping between gNMI and CLI files.")
        print(f"{'=' * 40}")
        
        # Save detailed report
        report_file = BASE_DIR / f"import_orphans_{device_name}.txt"
        with open(report_file, 'w', encoding='utf-8') as f:
            f.write(f"Orphan files report for {device_name}\n")
            f.write(f"=" * 60 + "\n\n")
            
            f.write("ORPHAN gNMI files (no matching CLI):\n")
            for gnmi_f in orphan_gnmi:
                expected_cli = gnmi_filename_to_cli_filename(gnmi_f)
                f.write(f"  gNMI: {gnmi_f}\n")
                f.write(f"  -> Expected CLI: {expected_cli}\n\n")
            
            f.write("\nORPHAN CLI files (no matching gNMI):\n")
            for cli_f in orphan_cli:
                patterns = cli_filename_to_gnmi_patterns(cli_f)
                f.write(f"  CLI: {cli_f}\n")
                f.write(f"  -> Expected gNMI: {patterns[0]}\n\n")
        
        print(f"\nDetailed report saved to: {report_file}")
        sys.exit(1)
    
    # Check which test cases we can match
    print(f"\n{'=' * 40}")
    print("STEP 2: Matching with existing test data...")
    print(f"{'=' * 40}")
    
    matched_tests = 0
    unmatched_gnmi = []
    
    for gnmi_file in sorted(mapping.keys()):
        found = False
        for json_file in TEST_DATA_DIR.glob('data_*.json'):
            try:
                with open(json_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                for test_case in data.get('test_cases', []):
                    if test_case.get('gnmi_file') == gnmi_file:
                        found = True
                        matched_tests += 1
                        break
                
                if found:
                    break
            except Exception:
                continue
        
        if not found:
            unmatched_gnmi.append(gnmi_file)
    
    print(f"\n  Matched to existing test cases: {matched_tests}")
    print(f"  Not in existing test data: {len(unmatched_gnmi)}")
    
    if unmatched_gnmi:
        print(f"\n  Files not in existing test data:")
        for f in unmatched_gnmi[:15]:
            print(f"    - {f}")
        if len(unmatched_gnmi) > 15:
            print(f"    ... and {len(unmatched_gnmi) - 15} more")
    
    # Perform import
    if matched_tests > 0:
        print(f"\n{'=' * 40}")
        print("STEP 3: Updating existing test data...")
        print(f"{'=' * 40}")
        
        updated, new, errors, unmatched = update_test_data(device_name, gnmi_dir, cli_dir, mapping, dry_run)
        
        print(f"\n  Updated test cases: {updated}")
        print(f"  Errors: {len(errors)}")
        
        if errors:
            for err in errors[:10]:
                print(f"    - {err}")
        
        if dry_run:
            print("\n  (DRY RUN - no files were modified)")
    
    # Create new test data files for unmatched commands
    if unmatched_gnmi:
        print(f"\n{'=' * 40}")
        print("STEP 4: Creating new test data files...")
        print(f"{'=' * 40}")
        
        created, create_errors = create_new_test_data_files(
            device_name, gnmi_dir, cli_dir, unmatched_gnmi, mapping, dry_run
        )
        
        print(f"\n  New test data files created: {created}")
        
        if create_errors:
            for err in create_errors[:10]:
                print(f"    - {err}")
        
        if dry_run:
            print("\n  (DRY RUN - no files were created)")
    
    print(f"\n{'=' * 40}")
    print("IMPORT COMPLETE")
    print(f"{'=' * 40}")


if __name__ == '__main__':
    main()
