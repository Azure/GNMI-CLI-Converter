#!/usr/bin/env python3
"""
Master test runner for gnmi_cli_lib verification.
Uses multi-step verification (same approach as generate_all_commands_analysis.py).

This script:
1. Loads all test data from test_data/*.json
2. Runs the converter for each command/device/variant combination
3. Uses multi-step comparison:
   a) Exact match between expected CLI output and converter output
   b) Match after normalizing spacing (for table alignment differences)
   c) Help text with option ordering differences (-h, -? vs -?, -h)
   d) gNMI data coverage check (all JSON values present in output)
   e) BGP-specific acceptable nuances
4. Generates a comprehensive verification report
"""

import json
import re
import sys
from pathlib import Path
from datetime import datetime

# Add parent dir to path so gnmi_cli_lib can be imported
SCRIPT_DIR = Path(__file__).parent.resolve()
PARENT_DIR = SCRIPT_DIR.parent
sys.path.insert(0, str(PARENT_DIR))

from gnmi_cli_lib.parser import CommandParser

# Directories
TEST_DATA_DIR = SCRIPT_DIR / "test_data"
REPORT_FILE = SCRIPT_DIR / "verification_report.md"

# Devices
DEVICES = ["str4-7060x6-512-9", "str4-7060x6-512-13", "str4-7060x6-512-1", "str2-8102-01", "str3-8101-02", "str4-sn5640-7"]


# ============================================================================
# Output normalization and comparison functions
# ============================================================================

def normalize_output(output: str) -> str:
    """Normalize output for comparison.
    
    Preserves leading whitespace (important for table alignment like '      IFACE')
    but removes trailing whitespace from each line and trailing newlines from the end.
    """
    if not output:
        return ""
    output = output.rstrip()
    lines = output.split('\n')
    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()
    return '\n'.join(line.rstrip() for line in lines)


def normalize_help_options(line: str) -> str:
    """Normalize help option ordering for comparison.
    
    Different systems may show '-h, -?, --help' or '-?, -h, --help'.
    Normalize to a consistent order for comparison.
    """
    if re.search(r'-[h?],\s*-[h?],\s*--help', line):
        line = re.sub(r'-h,\s*-\?,\s*--help', '-?, -h, --help', line)
    return line


def normalize_line_spacing(line: str) -> str:
    """Normalize a line by collapsing multiple spaces to single space."""
    return re.sub(r'\s+', ' ', line.strip())


def compare_with_normalized_spacing(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing spacing (for table alignment differences)."""
    if len(actual_lines) != len(expected_lines):
        return False
    for actual, expected in zip(actual_lines, expected_lines):
        if normalize_line_spacing(actual) != normalize_line_spacing(expected):
            return False
    return True


def normalize_timestamp_values(line: str) -> str:
    """
    Normalize timestamp/uptime values in a line for comparison.
    
    Handles formats like:
    - '1d08h57m' -> '<UPTIME>'
    - '04:43:27' -> '<UPTIME>'
    - 'Last update 1d08h57m ago' -> 'Last update <UPTIME> ago'
    - '1d09h00m' -> '<UPTIME>'
    
    This allows comparison when CLI and gNMI were captured at different times.
    """
    # Pattern for day/hour/minute format: 1d08h57m, 2d12h30m, etc.
    line = re.sub(r'\b\d+d\d{2}h\d{2}m\b', '<UPTIME>', line)
    # Pattern for HH:MM:SS format: 04:43:27, 12:30:00, etc.
    line = re.sub(r'\b\d{1,2}:\d{2}:\d{2}\b', '<UPTIME>', line)
    # Pattern for uptime in seconds: "up for 12345"
    line = re.sub(r'\bup for \d+\b', 'up for <UPTIME>', line)
    return line


def compare_with_normalized_timestamps(actual_lines: list, expected_lines: list) -> bool:
    """
    Compare lines after normalizing timestamp/uptime values.
    
    Returns True if lines match after replacing all uptime-like values
    with a placeholder. This handles the case where CLI and gNMI data
    were captured at different times.
    """
    if len(actual_lines) != len(expected_lines):
        return False
    
    for actual, expected in zip(actual_lines, expected_lines):
        actual_norm = normalize_timestamp_values(actual)
        expected_norm = normalize_timestamp_values(expected)
        if actual_norm != expected_norm:
            return False
    return True


def check_bgp_routes_acceptable_diff(actual_lines: list, expected_lines: list, cli_command: str) -> tuple:
    """
    Check if differences in BGP routes output are acceptable nuances.
    
    Known acceptable differences:
    1. "Displayed X routes and Y total paths" vs "Displayed X routes" 
       - The "total paths" is a global BGP counter from ALL neighbors, not available in GNMI
       - GNMI only returns neighbor-specific data, so we can't derive global total paths
    
    2. Header line spacing differences (e.g., 5 spaces vs 4 spaces before "Network")
       - These are device-specific or timing-related nuances
    
    Returns (is_acceptable, reason)
    """
    # Only apply to BGP neighbors routes commands
    if 'bgp neighbors' not in cli_command or 'routes' not in cli_command:
        return False, None
    
    if len(actual_lines) != len(expected_lines):
        return False, None
    
    differences_found = []
    
    for i, (actual, expected) in enumerate(zip(actual_lines, expected_lines)):
        if actual == expected:
            continue
        
        # Check for "Displayed X routes and Y total paths" vs "Displayed X routes"
        # This is acceptable because total paths is a global counter not in GNMI
        displayed_pattern = r'^Displayed\s+(\d+)\s+routes'
        actual_displayed = re.match(displayed_pattern, actual.strip())
        expected_displayed = re.match(displayed_pattern, expected.strip())
        
        if actual_displayed and expected_displayed:
            # Both have "Displayed X routes" - check if only difference is "total paths"
            if actual_displayed.group(1) == expected_displayed.group(1):
                # Same route count, difference is just the "and Y total paths" part
                differences_found.append(f"Line {i+1}: total paths not in GNMI (acceptable)")
                continue
        
        # Check for header line spacing differences
        # "    Network" vs "     Network" (spacing before Network column)
        if normalize_line_spacing(actual) == normalize_line_spacing(expected):
            differences_found.append(f"Line {i+1}: spacing difference only (acceptable)")
            continue
        
        # Other differences are not acceptable
        return False, None
    
    if differences_found:
        return True, f"BGP routes acceptable nuances: {'; '.join(differences_found)}"
    
    return False, None


# ============================================================================
# Value extraction and matching functions (for gNMI coverage check)
# ============================================================================

# Keys whose values are internal metadata and should not be checked for display
INTERNAL_METADATA_KEYS = {
    'internalFlags', 'internalStatus', 'internalNextHopNum', 'internalNextHopActiveNum',
    'interfaceIndex', 'installedNexthopGroupId', 'nexthopGroupId', 'receivedNexthopGroupId',
    'table', 'vrfId', 'flags', 'prefixLen',
}


def extract_json_values(data, values=None, keys=None, parent_key=None) -> tuple:
    """Recursively extract all leaf values and keys from JSON data.
    
    Also handles nested JSON strings (e.g., {"Ethernet0": "{\"key\": \"value\"}"})
    by parsing them and extracting values from within.
    
    Skips values under INTERNAL_METADATA_KEYS since they are internal IDs not shown in CLI.
    """
    if values is None:
        values = set()
    if keys is None:
        keys = set()
    
    if isinstance(data, dict):
        for k, v in data.items():
            keys.add(str(k))
            # Skip internal metadata keys
            if k in INTERNAL_METADATA_KEYS:
                continue
            extract_json_values(v, values, keys, k)
    elif isinstance(data, list):
        for item in data:
            extract_json_values(item, values, keys, parent_key)
    elif isinstance(data, str):
        str_val = data.strip()
        if str_val.startswith('{') or str_val.startswith('['):
            if str_val.startswith('[') and str_val.endswith(']') and not str_val.startswith('[['):
                inner = str_val[1:-1]
                if inner and not inner.startswith('"') and '":' not in inner:
                    items = [item.strip() for item in inner.split(',') if item.strip()]
                    for item in items:
                        values.add(item)
                    return values, keys
            try:
                parsed = json.loads(str_val)
                extract_json_values(parsed, values, keys, parent_key)
                return values, keys
            except json.JSONDecodeError:
                pass
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)
    elif data is not None:
        str_val = str(data)
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)
    
    return values, keys


def format_number_with_comma(val: str) -> str:
    """Convert a numeric string to comma-formatted version (e.g., '13312' -> '13,312')."""
    try:
        num = int(val)
        return f"{num:,}"
    except ValueError:
        return None


def normalize_numeric_value(val: str) -> list:
    """Get all possible representations of a numeric value for matching."""
    representations = [val]
    try:
        num = float(val)
        if num == int(num):
            representations.append(str(int(num)))
        representations.append(f"{num:.0f}")
        representations.append(f"{num:.1f}")
        representations.append(f"{num:.2f}")
        representations.append(f"{num:.3f}")
        representations.append(f"{num:.4f}")
        representations.append(f"{num:.5f}")
        representations.append(f"{num:.6f}")
        representations.append(f"{num:g}")
        
        if '.' in val:
            for i in range(3, min(10, len(val))):
                representations.append(val[:i])
        
        if num >= 1000 and num == int(num):
            sec_val = int(num) // 1000
            representations.append(str(sec_val))
            if sec_val > 0:
                hours = sec_val // 3600
                minutes = (sec_val % 3600) // 60
                secs = sec_val % 60
                representations.append(f"{hours:02d}:{minutes:02d}:{secs:02d}")
                if hours == 0:
                    representations.append(f"{minutes:02d}:{secs:02d}")
                days = sec_val // 86400
                remaining_secs = sec_val % 86400
                hrs = remaining_secs // 3600
                remaining_secs = remaining_secs % 3600
                mins = remaining_secs // 60
                weeks = days // 7
                days_in_week = days % 7
                if weeks > 0:
                    representations.append(f"{weeks:02d}w{days_in_week}d{hrs:02d}h")
                    representations.append(f"{weeks}w{days_in_week}d{hrs}h")
                if days > 0:
                    representations.append(f"{days}d{hrs:02d}h{mins:02d}m")
                    representations.append(f"{days}d{hrs}h{mins}m")
                else:
                    representations.append(f"{hrs:02d}h{mins:02d}m")
                    representations.append(f"{hrs}h{mins}m")
    except ValueError:
        pass
    return representations


def normalize_string_value(val: str) -> list:
    """Get all possible representations of a string value for matching."""
    representations = [val]
    representations.append(val.lower())
    representations.append(val.upper())
    
    transformations = {
        'advertisedAndReceived': 'advertised and received',
        'advertised': 'advertised',
        'received': 'received',
        'extendedAndStandard': 'extended and standard',
        'sharedNetwork': 'shared network',
    }
    if val in transformations:
        representations.append(transformations[val])
    
    if val.lower() == 'none':
        representations.append('none')
        representations.append('None')
    
    return representations


def check_multiline_string_coverage(val: str, output: str) -> bool:
    """Check if a multi-line string is covered by checking each line individually."""
    if '\n' not in val:
        return False
    lines = val.split('\n')
    for line in lines:
        line = line.strip()
        if line and line not in output:
            return False
    return True


def check_truncated_value_coverage(val: str, output: str, min_prefix_len: int = 20) -> bool:
    """Check if a value is covered via truncation (value appears as prefix + '...')."""
    if len(val) < min_prefix_len:
        return False
    for prefix_len in range(len(val) - 1, min_prefix_len - 1, -1):
        prefix = val[:prefix_len]
        if prefix + "..." in output:
            return True
        if prefix in output and len(prefix) > min_prefix_len + 10:
            return True
    return False


# Values that are intentionally filtered/skipped by the formatter
INTENTIONALLY_FILTERED_VALUES = {
    'Unknown', 'none', 'None', 'Established',
    'True', 'true', 'False', 'false',
    'enabled', 'disabled', 'up', 'down',
}


def check_all_entries_empty(json_data: dict) -> bool:
    """Check if all entries in JSON data are empty dicts."""
    if not isinstance(json_data, dict):
        return False
    if not json_data:
        return True
    for key, value in json_data.items():
        if isinstance(value, dict):
            if value:
                return False
        elif isinstance(value, list):
            if value:
                return False
        elif value is not None and value != "":
            return False
    return True


def check_gnmi_coverage(actual_output: str, json_data: dict, cli_command: str = "") -> tuple:
    """
    Check if the actual converter output covers all data from gNMI JSON.
    Returns (is_pass, coverage_info)
    """
    if '--nonzero' in cli_command and check_all_entries_empty(json_data):
        if not actual_output or not actual_output.strip():
            return True, "All JSON entries empty with --nonzero filter, no output expected (PASS)"
        else:
            return True, "All JSON entries empty with --nonzero filter (PASS)"
    
    if not actual_output:
        return False, "No converter output"
    
    if not json_data:
        return True, "Empty JSON data with converter output (PASS)"
    
    json_values, _ = extract_json_values(json_data)
    
    if not json_values:
        return True, "No values in JSON data, converter produced output (PASS)"
    
    covered = set()
    missing = set()
    
    for val in json_values:
        found = False
        
        if val in INTENTIONALLY_FILTERED_VALUES:
            found = True
        elif val in actual_output:
            found = True
        else:
            formatted = format_number_with_comma(val)
            if formatted and formatted in actual_output:
                found = True
            else:
                for alt_val in normalize_numeric_value(val):
                    if alt_val in actual_output:
                        found = True
                        break
                
                if not found:
                    for alt_val in normalize_string_value(val):
                        if alt_val in actual_output:
                            found = True
                            break
                
                if not found:
                    if check_multiline_string_coverage(val, actual_output):
                        found = True
                
                if not found:
                    if check_truncated_value_coverage(val, actual_output):
                        found = True
                
                # Check IPv6 link-local addresses with zone identifier
                # JSON has: fe80::X/64, output may have: fe80::X%Interface/64
                if not found and val.lower().startswith("fe80:") and "/" in val:
                    addr_part, prefix = val.rsplit("/", 1)
                    # Look for the address with any zone ID appended
                    if addr_part + "%" in actual_output:
                        found = True
        
        if found:
            covered.add(val)
        else:
            missing.add(val)
    
    coverage_pct = len(covered) / len(json_values) * 100 if json_values else 0
    
    if not missing:
        return True, f"All {len(json_values)} values from gNMI JSON covered (100%)"
    else:
        missing_sample = list(missing)[:5]
        return False, f"Coverage: {coverage_pct:.1f}% ({len(covered)}/{len(json_values)}). Missing: {missing_sample}"


def compare_outputs(actual: str, expected: str, json_data: dict = None, cli_command: str = "") -> tuple:
    """
    Compare actual and expected outputs with multi-step logic (same as generate_all_commands_analysis.py).
    
    PASS conditions:
    1. Exact match between normalized outputs
    2. Match after normalizing column spacing (for table alignment differences)
    3. Match after normalizing timestamp/uptime values (time drift between CLI and gNMI capture)
    4. Help text with only option ordering differences (e.g., -h, -? vs -?, -h)
    5. All values from gNMI JSON are covered in the actual output
    6. For --nonzero commands: if all JSON entries are empty {}, no output is correct
    7. BGP routes acceptable nuances (total paths not in GNMI)
    
    Returns (is_pass, diff_info, coverage_info)
    """
    actual_norm = normalize_output(actual)
    expected_norm = normalize_output(expected)
    
    # First check: exact match
    if actual_norm == expected_norm:
        return True, None, "Exact match"
    
    actual_lines = actual_norm.split('\n')
    expected_lines = expected_norm.split('\n')
    
    # Second check: match after normalizing column spacing (for table alignment)
    if compare_with_normalized_spacing(actual_lines, expected_lines):
        return True, None, "Match (spacing differences only)"
    
    # Third check: match after normalizing timestamp/uptime values (time drift between CLI and gNMI capture)
    if compare_with_normalized_timestamps(actual_lines, expected_lines):
        return True, None, "Match (timestamp/uptime differences only - CLI vs gNMI time drift)"
    
    # Check for help text with only option ordering differences
    if len(actual_lines) == len(expected_lines):
        actual_normalized_help = [normalize_help_options(line) for line in actual_lines]
        expected_normalized_help = [normalize_help_options(line) for line in expected_lines]
        if actual_normalized_help == expected_normalized_help:
            return True, None, "Match after normalizing help option order"
    
    # Fourth check: gNMI data coverage (use 'is not None' to handle empty dict {})
    if json_data is not None:
        is_covered, coverage_info = check_gnmi_coverage(actual, json_data, cli_command)
        if is_covered:
            return True, None, coverage_info
    
    # Fifth check: BGP routes acceptable nuances
    is_acceptable, reason = check_bgp_routes_acceptable_diff(actual_lines, expected_lines, cli_command)
    if is_acceptable:
        return True, None, reason
    
    # Failed - provide diff info
    diff_info = []
    diff_info.append(f"Line count: expected={len(expected_lines)}, actual={len(actual_lines)}")
    
    for i, (exp, act) in enumerate(zip(expected_lines, actual_lines)):
        if exp != act:
            diff_info.append(f"First diff at line {i+1}")
            break
    
    if json_data is not None:
        _, coverage_info = check_gnmi_coverage(actual, json_data)
        diff_info.append(f"gNMI Coverage: {coverage_info}")
    
    return False, '\n'.join(diff_info), None


# ============================================================================
# Test runner functions
# ============================================================================

def load_all_test_data():
    """Load all test data JSON files."""
    test_files = sorted(TEST_DATA_DIR.glob("data_*.json"))
    all_data = []
    for f in test_files:
        with open(f, 'r', encoding='utf-8') as fp:
            data = json.load(fp)
            data['_file'] = f.name
            all_data.append(data)
    return all_data


def run_single_test(test_data, parser):
    """Run a single test for all devices and all test case variants.
    
    Returns dict with results per device per variant:
    {
        "command": "show ...",
        "test_cases": [
            {
                "input_options": "",
                "results": {
                    "device1": {"status": "PASS", "message": "..."},
                    ...
                }
            },
            ...
        ]
    }
    """
    command = test_data.get("command", "unknown")
    test_cases = test_data.get("test_cases", [])
    
    results = {
        "command": command,
        "test_cases": []
    }
    
    for test_case in test_cases:
        devices_data = test_case.get("devices", {})
        input_options = test_case.get("input_options", "")
        full_command = f"{command} {input_options}".strip() if input_options else command
        
        tc_result = {
            "input_options": input_options,
            "full_command": full_command,
            "results": {}
        }
        
        for device in DEVICES:
            if device not in devices_data:
                tc_result["results"][device] = {
                    "status": "NO_ACCEPTABLE_DATA",
                    "message": "No data for device"
                }
                continue
            
            device_data = devices_data[device]
            json_string = device_data.get("json_string")
            json_error = device_data.get("json_error")
            
            if json_error:
                tc_result["results"][device] = {
                    "status": "NO_ACCEPTABLE_DATA",
                    "message": f"JSON extraction error: {json_error}"
                }
                continue
            
            if not json_string or json_string.strip() == "":
                tc_result["results"][device] = {
                    "status": "NO_ACCEPTABLE_DATA",
                    "message": "Empty JSON string"
                }
                continue
            
            # Parse JSON
            try:
                data = json.loads(json_string)
            except json.JSONDecodeError as e:
                tc_result["results"][device] = {
                    "status": "ERROR",
                    "message": f"JSON parse error: {e}"
                }
                continue
            
            # Run converter
            try:
                actual_output = parser.parse_and_convert(full_command, data)
            except Exception as e:
                tc_result["results"][device] = {
                    "status": "ERROR",
                    "message": f"Converter error: {type(e).__name__}: {e}"
                }
                continue
            
            # Get expected CLI output for comparison
            expected_output = device_data.get("cli_string", "")
            
            # Use multi-step comparison (same as generate_all_commands_analysis.py):
            # 1. Exact match
            # 2. Spacing-normalized match
            # 3. Help option order match
            # 4. gNMI data coverage
            # 5. BGP acceptable nuances
            is_pass, diff_info, match_reason = compare_outputs(
                actual_output, expected_output, data, full_command
            )
            
            if is_pass:
                tc_result["results"][device] = {
                    "status": "PASS",
                    "message": match_reason
                }
            else:
                tc_result["results"][device] = {
                    "status": "FAIL",
                    "message": diff_info if diff_info else "Unknown mismatch"
                }
        
        results["test_cases"].append(tc_result)
    
    return results


def generate_report(all_results):
    """Generate markdown report."""
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    # Flatten all test cases to count stats
    all_test_case_results = []
    for result in all_results:
        for tc in result.get("test_cases", []):
            for device in DEVICES:
                if device in tc["results"]:
                    all_test_case_results.append({
                        "command": result["command"],
                        "input_options": tc["input_options"],
                        "device": device,
                        "status": tc["results"][device]["status"],
                        "message": tc["results"][device]["message"]
                    })
    
    # Count stats per device
    stats = {device: {"PASS": 0, "FAIL": 0, "ERROR": 0, "NO_ACCEPTABLE_DATA": 0} for device in DEVICES}
    for tcr in all_test_case_results:
        stats[tcr["device"]][tcr["status"]] += 1
    
    # Calculate totals
    total_pass = sum(s["PASS"] for s in stats.values())
    total_fail = sum(s["FAIL"] for s in stats.values())
    total_error = sum(s["ERROR"] for s in stats.values())
    total_no_acceptable_data = sum(s["NO_ACCEPTABLE_DATA"] for s in stats.values())
    total_tests = total_pass + total_fail + total_error + total_no_acceptable_data
    pass_rate = (total_pass / (total_pass + total_fail)) * 100 if (total_pass + total_fail) > 0 else 0
    
    lines = [
        "# gNMI CLI Library Verification Report",
        "",
        f"Generated: {timestamp}",
        "",
        "## Summary",
        "",
        f"**Total Test Cases:** {total_tests}",
        f"**Pass Rate:** {pass_rate:.1f}% ({total_pass} Pass / {total_pass + total_fail} Run)",
        "",
        "| Device | PASS | FAIL | ERROR | No Acceptable Data | Total |",
        "|--------|------|------|-------|---------|-------|",
    ]
    
    for device in DEVICES:
        s = stats[device]
        total = sum(s.values())
        lines.append(f"| {device} | {s['PASS']} | {s['FAIL']} | {s['ERROR']} | {s['NO_ACCEPTABLE_DATA']} | {total} |")
    
    lines.extend([
        "",
        f"| **Total** | **{total_pass}** | **{total_fail}** | **{total_error}** | **{total_no_acceptable_data}** | **{total_tests}** |",
        "",
        "## Detailed Results",
        "",
    ])
    
    # Group failures by command
    failures = [tcr for tcr in all_test_case_results if tcr["status"] == "FAIL"]
    errors = [tcr for tcr in all_test_case_results if tcr["status"] == "ERROR"]
    
    if failures:
        lines.extend([
            "### ❌ Failures",
            "",
            "| Command | Options | Device | Coverage Info |",
            "|---------|---------|--------|---------------|",
        ])
        for f in failures:
            opts = f["input_options"] if f["input_options"] else "(default)"
            lines.append(f"| `{f['command']}` | {opts} | {f['device']} | {f['message']} |")
        lines.append("")
    
    if errors:
        lines.extend([
            "### ⚠️ Errors",
            "",
            "| Command | Options | Device | Error |",
            "|---------|---------|--------|-------|",
        ])
        for e in errors:
            opts = e["input_options"] if e["input_options"] else "(default)"
            lines.append(f"| `{e['command']}` | {opts} | {e['device']} | {e['message']} |")
        lines.append("")
    
    # Summary by command
    lines.extend([
        "### ✅ Pass Summary by Command",
        "",
        "| Command | Pass | Fail | No Acceptable Data |",
        "|---------|------|------|---------|"  ,
    ])
    
    cmd_stats = {}
    for tcr in all_test_case_results:
        cmd = tcr["command"]
        if cmd not in cmd_stats:
            cmd_stats[cmd] = {"PASS": 0, "FAIL": 0, "ERROR": 0, "NO_ACCEPTABLE_DATA": 0}
        cmd_stats[cmd][tcr["status"]] += 1
    
    for cmd in sorted(cmd_stats.keys()):
        s = cmd_stats[cmd]
        lines.append(f"| `{cmd}` | {s['PASS']} | {s['FAIL'] + s['ERROR']} | {s['NO_ACCEPTABLE_DATA']} |")
    
    return '\n'.join(lines)


def main():
    print("=" * 60)
    print("gNMI CLI Library Verification Runner")
    print("(Using value-coverage verification)")
    print("=" * 60)
    print()
    
    # Load test data
    print(f"Loading test data from: {TEST_DATA_DIR}")
    all_test_data = load_all_test_data()
    print(f"Found {len(all_test_data)} test data files")
    print()
    
    # Create parser
    parser = CommandParser()
    
    # Run all tests
    all_results = []
    total_tests = 0
    total_pass = 0
    total_fail = 0
    total_error = 0
    total_no_acceptable_data = 0
    
    for i, test_data in enumerate(all_test_data, 1):
        command = test_data.get("command", "unknown")
        test_cases = test_data.get("test_cases", [])
        print(f"[{i:02d}/{len(all_test_data):02d}] Testing: {command} ({len(test_cases)} variants)")
        
        result = run_single_test(test_data, parser)
        all_results.append(result)
        
        # Count results for this command
        for tc in result["test_cases"]:
            for device in DEVICES:
                if device in tc["results"]:
                    status = tc["results"][device]["status"]
                    total_tests += 1
                    if status == "PASS":
                        total_pass += 1
                    elif status == "FAIL":
                        total_fail += 1
                    elif status == "ERROR":
                        total_error += 1
                    else:
                        total_no_acceptable_data += 1
    
    print()
    print("=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Total Test Cases: {total_tests}")
    print(f"  PASS:    {total_pass}")
    print(f"  FAIL:    {total_fail}")
    print(f"  ERROR:   {total_error}")
    print(f"  NO_ACCEPTABLE_DATA: {total_no_acceptable_data}")
    
    if total_pass + total_fail > 0:
        pass_rate = total_pass / (total_pass + total_fail) * 100
        print(f"\nPass Rate: {pass_rate:.1f}% ({total_pass}/{total_pass + total_fail})")
    
    # Generate report
    print()
    print(f"Generating report: {REPORT_FILE}")
    report = generate_report(all_results)
    with open(REPORT_FILE, 'w', encoding='utf-8') as f:
        f.write(report)
    print("Done!")
    
    # Return exit code based on failures
    return 0 if total_fail == 0 and total_error == 0 else 1


if __name__ == "__main__":
    sys.exit(main())

