#!/usr/bin/env python3
"""
Import test data from str3-8101-02 and str4-sn5640-7 device folders
into test_data JSON files under gnmi_cli_lib/real_device_tests/test_data/.

This script:
1. Scans gNMI and CLI output directories for each device
2. Builds gNMI-to-CLI file mappings
3. Reads gNMI JSON and CLI text content
4. Matches each pair to existing test_data JSON files by gnmi_file field
5. Appends the new device data into the matched test case

Usage:
    python import_new_devices.py [--dry-run]
"""
import json
import sys
import re
from pathlib import Path
from typing import Dict, List, Tuple, Optional

BASE_DIR = Path(__file__).parent
PYTHON_DIR = BASE_DIR.parent  # python/ folder

# Target test data directory
TEST_DATA_DIR = BASE_DIR.parent / "gnmi_cli_lib" / "real_device_tests" / "test_data"

# Source device folders (relative to python/)
DEVICES = {
    "str3-8101-02": PYTHON_DIR / "str3-8101-02",
    "str4-sn5640-7": PYTHON_DIR / "str4-sn5640-7",
}


def read_file_content(filepath: Path) -> Optional[str]:
    """Read file content, return None if file doesn't exist."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.read()
    except FileNotFoundError:
        return None
    except Exception as e:
        return f"ERROR: {e}"


def extract_cli_from_source(content: str) -> Optional[str]:
    """Extract CLI output from source file, removing command header lines."""
    if content is None:
        return None

    lines = content.split('\n')
    result_lines = []
    for line in lines:
        if line.startswith('$ '):
            continue
        result_lines.append(line)

    return '\n'.join(result_lines)


def extract_json_from_gnmi(content: str) -> Tuple[Optional[str], Optional[str]]:
    """
    Extract JSON from gNMI output file.
    Returns (json_string, error_message)
    """
    if content is None:
        return None, "File not found"

    # Find json_ietf_val: "..." and extract the content
    if 'json_ietf_val:' in content:
        start_marker = 'json_ietf_val:'
        start_idx = content.find(start_marker)
        if start_idx != -1:
            quote_start = content.find('"', start_idx + len(start_marker))
            if quote_start != -1:
                remaining = content[quote_start + 1:]

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

    depth = 0
    for i, c in enumerate(content[start:], start):
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                json_str = content[start:i + 1]
                try:
                    data = json.loads(json_str)
                    return json.dumps(data, separators=(',', ':')), None
                except json.JSONDecodeError as e:
                    return None, f"JSON parse error: {e}"

    return None, "No valid JSON found"


def gnmi_filename_to_cli_filename(gnmi_name: str) -> str:
    """
    Convert gNMI filename to CLI filename.
    Reuses the same logic from import_device_data.py.
    """
    name = gnmi_name.replace('gnmi-get-', '')
    name = name.replace('.txt', '')

    # Handle boolean parameters FIRST
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

    # Preserve hyphenated subcommands
    hyphenated_commands = [
        'persistent-watermark',
        'headroom-pool',
        'fec-stats',
        'fec-histogram',
        'error-status',
        'naming_mode',
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

    # Restore parameter placeholders
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


def build_mapping(gnmi_dir: Path, cli_dir: Path) -> Tuple[Dict[str, str], List[str], List[str]]:
    """
    Build mapping between gNMI and CLI files.
    Returns (mapping, orphan_gnmi, orphan_cli).
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


def determine_base_command(gnmi_file: str) -> str:
    """
    Determine the base show command from a gnmi filename.
    """
    name = gnmi_file.replace('gnmi-get-', '').replace('.txt', '')

    # Remove boolean parameters
    name = re.sub(r'-nonzero-true', '', name)
    name = re.sub(r'-all-true', '', name)
    name = re.sub(r'-trim-true', '', name)
    name = re.sub(r'-printall-true', '', name)
    name = re.sub(r'-dom-true', '', name)
    name = re.sub(r'-period-\d+', '', name)

    # Remove named parameters
    name = re.sub(r'-counter_type-[A-Z_]+', '', name)
    name = re.sub(r'-group-[A-Z]+', '', name)
    name = re.sub(r'-interface-[A-Za-z0-9]+', '', name)
    name = re.sub(r'-iface-[A-Za-z0-9]+', '', name)
    name = re.sub(r'-vrf-[A-Za-z]+', '', name)

    parts = name.split('-')

    # Find where interface/argument starts
    command_parts = []
    for part in parts:
        if re.match(r'^(Ethernet\d+|PortChannel\d+|Vlan\d+|fc00|fe80|2064|20c0|DEFAULT)', part, re.IGNORECASE):
            break
        command_parts.append(part)

    command = 'show ' + ' '.join(command_parts)

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


def extract_input_options_from_gnmi(gnmi_file: str, cli_file: str) -> str:
    """
    Extract input_options from gNMI/CLI filenames by comparing against base command.
    Uses CLI filename since it preserves the structure more clearly.
    """
    name = cli_file.replace('.txt', '').replace('show_', '', 1)

    # Determine base command parts from the gnmi file
    base_cmd = determine_base_command(gnmi_file)
    base_parts = base_cmd.replace('show ', '').split(' ')

    # The CLI filename parts
    name_parts = name.split('_')

    # Remove base command parts from the beginning
    remaining = name_parts[len(base_parts):]

    if not remaining:
        return ""

    # Reconstruct options
    options = []
    for part in remaining:
        if part.startswith('--'):
            options.append(part)
        else:
            options.append(part)

    return ' '.join(options)


def find_test_data_file_for_command(command: str) -> Optional[Path]:
    """Find the test data JSON file that matches this base command."""
    for json_file in sorted(TEST_DATA_DIR.glob('data_*.json')):
        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
            if data.get('command') == command:
                return json_file
        except Exception:
            continue
    return None


def build_test_data_index() -> Dict[str, Tuple[Path, int]]:
    """
    Build an index of gnmi_file -> (json_file_path, test_case_index)
    by scanning all test data JSON files.
    """
    index = {}
    for json_file in sorted(TEST_DATA_DIR.glob('data_*.json')):
        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)

            for tc_idx, test_case in enumerate(data.get('test_cases', [])):
                # Check if gnmi_file is at test_case level
                if 'gnmi_file' in test_case:
                    index[test_case['gnmi_file']] = (json_file, tc_idx)
                # Also check inside devices (the format used in real_device_tests)
                for device_name, device_data in test_case.get('devices', {}).items():
                    if 'gnmi_file' in device_data:
                        gnmi_file = device_data['gnmi_file']
                        if gnmi_file not in index:
                            index[gnmi_file] = (json_file, tc_idx)
        except Exception as e:
            print(f"  WARNING: Error reading {json_file.name}: {e}")

    return index


def import_device(device_name: str, source_dir: Path, index: Dict[str, Tuple[Path, int]],
                  dry_run: bool = False) -> Tuple[int, int, int, List[str], List[str]]:
    """
    Import data for a single device.
    Returns (updated_count, created_count, skipped_count, errors, unmatched).
    """
    gnmi_dir = source_dir / 'gnmi-outputs'
    cli_dir = source_dir / 'cli_outputs'

    if not gnmi_dir.exists():
        return 0, 0, 0, [f"gNMI directory not found: {gnmi_dir}"], []
    if not cli_dir.exists():
        return 0, 0, 0, [f"CLI directory not found: {cli_dir}"], []

    # Build file mapping
    mapping, orphan_gnmi, orphan_cli = build_mapping(gnmi_dir, cli_dir)

    print(f"\n  Mapped pairs: {len(mapping)}")
    print(f"  Orphan gNMI files (no CLI match): {len(orphan_gnmi)}")
    print(f"  Orphan CLI files (no gNMI match): {len(orphan_cli)}")

    if orphan_gnmi:
        print(f"\n  Orphan gNMI files:")
        for f in orphan_gnmi[:5]:
            expected = gnmi_filename_to_cli_filename(f)
            print(f"    - {f}  (expected CLI: {expected})")
        if len(orphan_gnmi) > 5:
            print(f"    ... and {len(orphan_gnmi) - 5} more")

    if orphan_cli:
        print(f"\n  Orphan CLI files:")
        for f in orphan_cli[:5]:
            print(f"    - {f}")
        if len(orphan_cli) > 5:
            print(f"    ... and {len(orphan_cli) - 5} more")

    # Process mapped pairs
    updated_count = 0
    created_count = 0
    skipped_count = 0
    errors = []
    unmatched = []

    # Track which JSON files were modified (to batch writes)
    modified_files: Dict[str, dict] = {}  # filepath -> data

    for gnmi_file, cli_file in sorted(mapping.items()):
        if gnmi_file in index:
            # --- Existing test case: append device data ---
            json_file_path, tc_idx = index[gnmi_file]
            json_key = str(json_file_path)

            if json_key not in modified_files:
                try:
                    with open(json_file_path, 'r', encoding='utf-8') as f:
                        modified_files[json_key] = json.load(f)
                except Exception as e:
                    errors.append(f"Error loading {json_file_path.name}: {e}")
                    continue

            data = modified_files[json_key]
            test_case = data['test_cases'][tc_idx]

            if device_name in test_case.get('devices', {}):
                skipped_count += 1
                continue

            gnmi_content = read_file_content(gnmi_dir / gnmi_file)
            cli_content = read_file_content(cli_dir / cli_file)
            json_str, json_err = extract_json_from_gnmi(gnmi_content)
            cli_str = extract_cli_from_source(cli_content)

            if 'devices' not in test_case:
                test_case['devices'] = {}

            test_case['devices'][device_name] = {
                'cli_file': cli_file,
                'gnmi_file': gnmi_file,
                'json_string': json_str,
                'json_error': json_err,
                'cli_string': cli_str,
                'cli_error': None
            }
            updated_count += 1

        else:
            # --- No existing test case: try to create a new one ---
            base_cmd = determine_base_command(gnmi_file)
            test_data_file = find_test_data_file_for_command(base_cmd)

            if test_data_file is None:
                unmatched.append(gnmi_file)
                continue

            json_key = str(test_data_file)
            if json_key not in modified_files:
                try:
                    with open(test_data_file, 'r', encoding='utf-8') as f:
                        modified_files[json_key] = json.load(f)
                except Exception as e:
                    errors.append(f"Error loading {test_data_file.name}: {e}")
                    continue

            data = modified_files[json_key]
            input_opts = extract_input_options_from_gnmi(gnmi_file, cli_file)

            # Check if a test_case with same input_options already exists
            existing_tc = None
            for tc in data['test_cases']:
                if tc.get('input_options') == input_opts:
                    existing_tc = tc
                    break

            gnmi_content = read_file_content(gnmi_dir / gnmi_file)
            cli_content = read_file_content(cli_dir / cli_file)
            json_str, json_err = extract_json_from_gnmi(gnmi_content)
            cli_str = extract_cli_from_source(cli_content)

            device_entry = {
                'cli_file': cli_file,
                'gnmi_file': gnmi_file,
                'json_string': json_str,
                'json_error': json_err,
                'cli_string': cli_str,
                'cli_error': None
            }

            if existing_tc is not None:
                if device_name not in existing_tc.get('devices', {}):
                    if 'devices' not in existing_tc:
                        existing_tc['devices'] = {}
                    existing_tc['devices'][device_name] = device_entry
                    updated_count += 1
                else:
                    skipped_count += 1
            else:
                # Create new test case
                new_tc = {
                    'input_options': input_opts,
                    'devices': {
                        device_name: device_entry
                    }
                }
                data['test_cases'].append(new_tc)
                # Also add to index so second device can find it
                index[gnmi_file] = (test_data_file, len(data['test_cases']) - 1)
                created_count += 1

    # Write modified files
    if not dry_run:
        for json_key, data in modified_files.items():
            try:
                with open(json_key, 'w', encoding='utf-8') as f:
                    json.dump(data, f, indent=2, ensure_ascii=False)
                    f.write('\n')
            except Exception as e:
                errors.append(f"Error writing {json_key}: {e}")

    return updated_count, created_count, skipped_count, errors, unmatched


def main():
    dry_run = '--dry-run' in sys.argv

    print("=" * 80)
    print("Import Device Data: str3-8101-02, str4-sn5640-7")
    print(f"Test data directory: {TEST_DATA_DIR}")
    print(f"Dry run: {dry_run}")
    print("=" * 80)

    # Validate test data directory
    if not TEST_DATA_DIR.exists():
        print(f"\nERROR: Test data directory not found: {TEST_DATA_DIR}")
        sys.exit(1)

    # Build index of all existing test data
    print("\nBuilding test data index...")
    index = build_test_data_index()
    print(f"  Indexed {len(index)} gNMI file entries across test data files")

    total_updated = 0
    total_created = 0
    total_skipped = 0
    total_unmatched = 0
    total_errors = 0

    for device_name, source_dir in DEVICES.items():
        print(f"\n{'=' * 60}")
        print(f"Processing device: {device_name}")
        print(f"Source: {source_dir}")
        print(f"{'=' * 60}")

        if not source_dir.exists():
            print(f"  ERROR: Source directory not found: {source_dir}")
            continue

        updated, created, skipped, errors, unmatched = import_device(
            device_name, source_dir, index, dry_run
        )

        print(f"\n  Results for {device_name}:")
        print(f"    Updated existing test cases: {updated}")
        print(f"    Created new test cases: {created}")
        print(f"    Skipped (already exists): {skipped}")
        print(f"    Unmatched gNMI files: {len(unmatched)}")
        print(f"    Errors: {len(errors)}")

        if errors:
            print(f"\n  Errors:")
            for err in errors:
                print(f"    - {err}")

        if unmatched:
            print(f"\n  Unmatched gNMI files (no matching test data file):")
            for f in unmatched[:10]:
                base = determine_base_command(f)
                print(f"    - {f}  (base cmd: {base})")
            if len(unmatched) > 10:
                print(f"    ... and {len(unmatched) - 10} more")

        total_updated += updated
        total_created += created
        total_skipped += skipped
        total_unmatched += len(unmatched)
        total_errors += len(errors)

    print(f"\n{'=' * 80}")
    print("SUMMARY")
    print(f"{'=' * 80}")
    print(f"  Total updated (existing test cases): {total_updated}")
    print(f"  Total created (new test cases): {total_created}")
    print(f"  Total skipped: {total_skipped}")
    print(f"  Total unmatched: {total_unmatched}")
    print(f"  Total errors: {total_errors}")

    if dry_run:
        print("\n  (DRY RUN - no files were modified)")

    print(f"\n{'=' * 80}")
    print("DONE")
    print(f"{'=' * 80}")


if __name__ == '__main__':
    main()
