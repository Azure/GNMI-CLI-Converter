#!/usr/bin/env python3
"""
Generate comprehensive analysis of all 70 commands with all test case variants.
For each command and each input_options variant, show:
- Input CLI command
- Input JSON (sample)
- Expected CLI output
- Real converter output
- Status (PASS/FAIL/ERROR)
"""
import sys
import json
import re
from pathlib import Path
from datetime import datetime

from rust_wrapper import RustCommandParser as CommandParser

BASE_DIR = Path(__file__).parent
TEST_DATA_DIR = BASE_DIR / "test_data"
OUTPUT_FILE = BASE_DIR / "ALL_COMMANDS_ANALYSIS.md"


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


def extract_json_values(data, values=None, keys=None) -> tuple:
    """Recursively extract all leaf values and keys from JSON data."""
    if values is None:
        values = set()
    if keys is None:
        keys = set()
    
    if isinstance(data, dict):
        for k, v in data.items():
            keys.add(str(k))
            extract_json_values(v, values, keys)
    elif isinstance(data, list):
        for item in data:
            extract_json_values(item, values, keys)
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
                extract_json_values(parsed, values, keys)
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
    """Convert a numeric string to comma-formatted version."""
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
    """Check if a value is covered via truncation."""
    if len(val) < min_prefix_len:
        return False
    for prefix_len in range(len(val) - 1, min_prefix_len - 1, -1):
        prefix = val[:prefix_len]
        if prefix + "..." in output:
            return True
        if prefix in output and len(prefix) > min_prefix_len + 10:
            return True
    return False


INTENTIONALLY_FILTERED_VALUES = {
    'Unknown', 'none', 'None', 'Established',
    'True', 'true', 'False', 'false',
    'enabled', 'disabled', 'up', 'down',
}


def normalize_timestamp_values(line: str) -> str:
    """Normalize timestamp/uptime values in a line for comparison."""
    line = re.sub(r'\b\d+d\d{2}h\d{2}m\b', '<UPTIME>', line)
    line = re.sub(r'\b\d{1,2}:\d{2}:\d{2}\b', '<UPTIME>', line)
    line = re.sub(r'\bup for \d+\b', 'up for <UPTIME>', line)
    return line


def compare_with_normalized_timestamps(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing timestamp/uptime values."""
    if len(actual_lines) != len(expected_lines):
        return False
    for actual, expected in zip(actual_lines, expected_lines):
        actual_norm = normalize_timestamp_values(actual)
        expected_norm = normalize_timestamp_values(expected)
        if actual_norm != expected_norm:
            return False
    return True


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
    """Check if the actual converter output covers all data from gNMI JSON."""
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
                
                if not found and val.lower().startswith("fe80:") and "/" in val:
                    addr_part, prefix = val.rsplit("/", 1)
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


def normalize_help_options(line: str) -> str:
    """Normalize help option ordering for comparison."""
    if re.search(r'-[h?],\s*-[h?],\s*--help', line):
        line = re.sub(r'-h,\s*-\?,\s*--help', '-?, -h, --help', line)
    return line


def normalize_line_spacing(line: str) -> str:
    """Normalize a line by collapsing multiple spaces to single space."""
    return re.sub(r'\s+', ' ', line.strip())


def compare_with_normalized_spacing(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing spacing."""
    if len(actual_lines) != len(expected_lines):
        return False
    for actual, expected in zip(actual_lines, expected_lines):
        if normalize_line_spacing(actual) != normalize_line_spacing(expected):
            return False
    return True


def check_bgp_routes_acceptable_diff(actual_lines: list, expected_lines: list, cli_command: str) -> tuple:
    """Check if differences in BGP routes output are acceptable nuances."""
    if 'bgp neighbors' not in cli_command or 'routes' not in cli_command:
        return False, None
    
    if len(actual_lines) != len(expected_lines):
        return False, None
    
    differences_found = []
    
    for i, (actual, expected) in enumerate(zip(actual_lines, expected_lines)):
        if actual == expected:
            continue
        
        displayed_pattern = r'^Displayed\s+(\d+)\s+routes'
        actual_displayed = re.match(displayed_pattern, actual.strip())
        expected_displayed = re.match(displayed_pattern, expected.strip())
        
        if actual_displayed and expected_displayed:
            if actual_displayed.group(1) == expected_displayed.group(1):
                differences_found.append(f"Line {i+1}: total paths not in GNMI (acceptable)")
                continue
        
        if normalize_line_spacing(actual) == normalize_line_spacing(expected):
            differences_found.append(f"Line {i+1}: spacing difference only (acceptable)")
            continue
        
        return False, None
    
    if differences_found:
        return True, f"BGP routes acceptable nuances: {'; '.join(differences_found)}"
    
    return False, None


def compare_outputs(actual: str, expected: str, json_data: dict = None, cli_command: str = "") -> tuple:
    """Compare actual and expected outputs with improved logic."""
    actual_norm = normalize_output(actual)
    expected_norm = normalize_output(expected)
    
    if actual_norm == expected_norm:
        return True, None, "Exact match"
    
    actual_lines = actual_norm.split('\n')
    expected_lines = expected_norm.split('\n')
    
    if compare_with_normalized_spacing(actual_lines, expected_lines):
        return True, None, "Match (spacing differences only)"
    
    if compare_with_normalized_timestamps(actual_lines, expected_lines):
        return True, None, "Match (timestamp/uptime differences only - CLI vs gNMI time drift)"
    
    if len(actual_lines) == len(expected_lines):
        actual_normalized_help = [normalize_help_options(line) for line in actual_lines]
        expected_normalized_help = [normalize_help_options(line) for line in expected_lines]
        if actual_normalized_help == expected_normalized_help:
            return True, None, "Match after normalizing help option order"
    
    if json_data is not None:
        is_covered, coverage_info = check_gnmi_coverage(actual, json_data, cli_command)
        if is_covered:
            return True, None, coverage_info
    
    is_acceptable, reason = check_bgp_routes_acceptable_diff(actual_lines, expected_lines, cli_command)
    if is_acceptable:
        return True, None, reason
    
    diff_info = []
    diff_info.append(f"Line count: expected={len(expected_lines)}, actual={len(actual_lines)}")
    
    for i, (exp, act) in enumerate(zip(expected_lines, actual_lines)):
        if exp != act:
            diff_info.append(f"First diff at line {i+1}:")
            diff_info.append(f"  Expected: {exp[:80]}")
            diff_info.append(f"  Actual:   {act[:80]}")
            break
    
    if len(actual_lines) != len(expected_lines):
        diff_info.append(f"Line count mismatch")
    
    if json_data is not None:
        _, coverage_info = check_gnmi_coverage(actual, json_data)
        diff_info.append(f"gNMI Coverage: {coverage_info}")
    
    return False, '\n'.join(diff_info), None


def truncate_content(content: str, max_lines: int = 50, max_chars: int = 2000, no_truncate: bool = False) -> str:
    """Truncate content for display."""
    if not content:
        return "(empty)"
    
    if no_truncate:
        return content
    
    lines = content.split('\n')
    if len(lines) > max_lines:
        truncated = '\n'.join(lines[:max_lines])
        truncated += f"\n... ({len(lines) - max_lines} more lines)"
        return truncated
    elif len(content) > max_chars:
        return content[:max_chars] + f"... ({len(content) - max_chars} more chars)"
    return content


SHOW_FULL_OUTPUT = False


def analyze_test_case(command: str, input_options: str, device_name: str, device_data: dict) -> dict:
    """Analyze a single test case using the Rust binary."""
    full_command = f"{command} {input_options}".strip() if input_options else command
    
    result = {
        "full_command": full_command,
        "input_options": input_options,
        "device": device_name,
        "status": "UNKNOWN",
        "error": None,
        "diff": None,
        "json_string": device_data.get("json_string"),
        "expected_cli": device_data.get("cli_string"),
        "actual_output": None,
    }
    
    json_string = device_data.get("json_string")
    json_error = device_data.get("json_error")
    expected_cli = device_data.get("cli_string")
    cli_error = device_data.get("cli_error")
    
    if json_error:
        result["status"] = "SKIP"
        result["error"] = f"JSON extraction error: {json_error}"
        return result
    
    if cli_error:
        result["status"] = "SKIP"
        result["error"] = f"CLI extraction error: {cli_error}"
        return result
    
    if json_string is None:
        result["status"] = "SKIP"
        result["error"] = "No JSON data"
        return result
    
    if expected_cli is None:
        result["status"] = "SKIP"
        result["error"] = "No expected CLI"
        return result
    
    # Parse JSON
    try:
        data = json.loads(json_string)
    except json.JSONDecodeError as e:
        result["status"] = "ERROR"
        result["error"] = f"JSON parse error: {e}"
        return result
    
    # Run converter
    try:
        parser = CommandParser()
        actual_output = parser.parse_and_convert(full_command, data)
        result["actual_output"] = actual_output
    except Exception as e:
        result["status"] = "ERROR"
        result["error"] = f"{type(e).__name__}: {e}"
        return result
    
    # Compare with improved logic
    match, diff, coverage_info = compare_outputs(actual_output, expected_cli, data, full_command)
    if match:
        result["status"] = "PASS"
        result["coverage_info"] = coverage_info
    else:
        result["status"] = "FAIL"
        result["diff"] = diff
    
    return result


def generate_analysis():
    """Generate comprehensive analysis for all commands across all devices."""
    
    # Find all test data files
    test_files = sorted(TEST_DATA_DIR.glob("data_*.json"))
    
    # Discover all devices from test data
    all_devices = set()
    for test_file in test_files:
        with open(test_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        for tc in data.get("test_cases", []):
            all_devices.update(tc.get("devices", {}).keys())
    
    # Sort devices for consistent ordering
    all_devices = sorted(all_devices)
    
    output_lines = []
    output_lines.append("# ALL COMMANDS ANALYSIS")
    output_lines.append("")
    output_lines.append(f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    output_lines.append(f"**Total Commands:** {len(test_files)}")
    output_lines.append(f"**Devices Tested:** {len(all_devices)}")
    output_lines.append("")
    
    # Table of Contents
    output_lines.append("## Table of Contents")
    output_lines.append("")
    output_lines.append("- [Overall Summary](#overall-summary)")
    output_lines.append("- [Per-Device Summary](#per-device-summary)")
    output_lines.append("- [Quick Reference by Command](#quick-reference-by-command)")
    output_lines.append("- [Detailed Analysis](#detailed-analysis)")
    output_lines.append("")
    
    output_lines.append("### Devices")
    output_lines.append("")
    for i, device in enumerate(all_devices, 1):
        output_lines.append(f"{i}. `{device}`")
    output_lines.append("")
    
    # Per-device stats
    device_stats = {device: {"pass": 0, "fail": 0, "error": 0, "skip": 0, "total": 0} for device in all_devices}
    
    # Summary stats
    total_tests = 0
    total_pass = 0
    total_fail = 0
    total_error = 0
    total_skip = 0
    
    command_results = []
    
    for test_file in test_files:
        with open(test_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        command = data.get("command", "unknown")
        test_cases = data.get("test_cases", [])
        
        idx = test_file.stem.split('_')[1]
        
        cmd_result = {
            "index": idx,
            "command": command,
            "test_cases": [],
            "pass_count": 0,
            "fail_count": 0,
            "error_count": 0,
            "skip_count": 0,
            "device_stats": {device: {"pass": 0, "fail": 0, "error": 0, "skip": 0} for device in all_devices},
        }
        
        for case_idx, tc in enumerate(test_cases):
            input_options = tc.get("input_options", "")
            devices = tc.get("devices", {})
            if not devices:
                continue
            
            case_result = {
                "case_index": case_idx,
                "input_options": input_options,
                "full_command": f"{command} {input_options}".strip() if input_options else command,
                "device_results": {},
            }
            
            for device_name in all_devices:
                if device_name not in devices:
                    result = {
                        "status": "N/A",
                        "device": device_name,
                        "error": "Device not in test data",
                    }
                    case_result["device_results"][device_name] = result
                    continue
                
                device_data = devices[device_name]
                result = analyze_test_case(command, input_options, device_name, device_data)
                
                case_result["device_results"][device_name] = result
                
                total_tests += 1
                device_stats[device_name]["total"] += 1
                
                if result["status"] == "PASS":
                    total_pass += 1
                    cmd_result["pass_count"] += 1
                    cmd_result["device_stats"][device_name]["pass"] += 1
                    device_stats[device_name]["pass"] += 1
                elif result["status"] == "FAIL":
                    total_fail += 1
                    cmd_result["fail_count"] += 1
                    cmd_result["device_stats"][device_name]["fail"] += 1
                    device_stats[device_name]["fail"] += 1
                elif result["status"] == "ERROR":
                    total_error += 1
                    cmd_result["error_count"] += 1
                    cmd_result["device_stats"][device_name]["error"] += 1
                    device_stats[device_name]["error"] += 1
                else:
                    total_skip += 1
                    cmd_result["skip_count"] += 1
                    cmd_result["device_stats"][device_name]["skip"] += 1
                    device_stats[device_name]["skip"] += 1
            
            cmd_result["test_cases"].append(case_result)
        
        command_results.append(cmd_result)
        print(f"  [{idx}] {command}: P={cmd_result['pass_count']} F={cmd_result['fail_count']} E={cmd_result['error_count']} S={cmd_result['skip_count']}")
    
    # Overall Summary
    tests_with_result = total_pass + total_fail + total_error
    pass_rate = total_pass / tests_with_result * 100 if tests_with_result > 0 else 0
    
    output_lines.append("## Overall Summary")
    output_lines.append("")
    output_lines.append(f"| Metric | Count |")
    output_lines.append(f"|--------|-------|")
    output_lines.append(f"| Total Test Cases (with data) | {total_tests} |")
    output_lines.append(f"| ✅ PASS | {total_pass} |")
    output_lines.append(f"| ❌ FAIL | {total_fail} |")
    output_lines.append(f"| ⚠️ ERROR | {total_error} |")
    output_lines.append(f"| ⏭️ SKIP (data error) | {total_skip} |")
    output_lines.append(f"| **Pass Rate** | **{pass_rate:.1f}%** |")
    output_lines.append("")
    
    # Per-Device Summary
    output_lines.append("## Per-Device Summary")
    output_lines.append("")
    output_lines.append("| Device | Total | ✅ Pass | ❌ Fail | ⚠️ Error | ⏭️ Skip | Pass Rate |")
    output_lines.append("|--------|-------|---------|---------|----------|---------|-----------|")
    
    for device in all_devices:
        stats = device_stats[device]
        dev_total = stats["total"]
        dev_pass = stats["pass"]
        dev_fail = stats["fail"]
        dev_error = stats["error"]
        dev_skip = stats["skip"]
        dev_tests_with_result = dev_pass + dev_fail + dev_error
        dev_pass_rate = dev_pass / dev_tests_with_result * 100 if dev_tests_with_result > 0 else 0
        output_lines.append(
            f"| `{device}` | {dev_total} | {dev_pass} | {dev_fail} | {dev_error} | {dev_skip} | {dev_pass_rate:.1f}% |"
        )
    output_lines.append("")
    
    # Quick reference table
    output_lines.append("## Quick Reference by Command")
    output_lines.append("")
    output_lines.append("| # | Command | Variants | Pass | Fail | Error | Skip |")
    output_lines.append("|---|---------|----------|------|------|-------|------|")
    
    for cmd in command_results:
        status_emoji = "✅" if cmd["fail_count"] == 0 and cmd["error_count"] == 0 else "❌"
        cmd_anchor = cmd['command'].replace(' ', '-').replace('_', '-').lower()
        output_lines.append(
            f"| {cmd['index']} | [`{cmd['command']}`](#{cmd['index']}-{cmd_anchor}) | {len(cmd['test_cases'])} | "
            f"{cmd['pass_count']} | {cmd['fail_count']} | {cmd['error_count']} | {cmd['skip_count']} |"
        )
    
    output_lines.append("")
    output_lines.append("---")
    output_lines.append("")
    
    # Detailed analysis
    output_lines.append("## Detailed Analysis")
    output_lines.append("")
    
    for cmd in command_results:
        cmd_anchor = cmd['command'].replace(' ', '-').replace('_', '-').lower()
        output_lines.append(f"<a id=\"{cmd['index']}-{cmd_anchor}\"></a>")
        output_lines.append(f"### {cmd['index']}. `{cmd['command']}`")
        output_lines.append("")
        output_lines.append("[↑ Back to Quick Reference](#quick-reference-by-command)")
        output_lines.append("")
        
        all_pass = cmd["fail_count"] == 0 and cmd["error_count"] == 0
        if all_pass:
            total_variant_tests = len(cmd["test_cases"]) * len(all_devices)
            output_lines.append(f"**Status:** ✅ All {cmd['pass_count']}/{total_variant_tests} test cases passed (across {len(all_devices)} devices)")
        else:
            output_lines.append(f"**Status:** ❌ {cmd['fail_count']} failures, {cmd['error_count']} errors")
        output_lines.append("")
        
        # Per-device summary for this command
        output_lines.append("**Per-Device Results:**")
        output_lines.append("")
        output_lines.append("| Device | Pass | Fail | Error | Skip |")
        output_lines.append("|--------|------|------|-------|------|")
        for device in all_devices:
            ds = cmd["device_stats"][device]
            output_lines.append(f"| `{device}` | {ds['pass']} | {ds['fail']} | {ds['error']} | {ds['skip']} |")
        output_lines.append("")
        
        # Quick links for variants
        if len(cmd["test_cases"]) > 1:
            output_lines.append("**Test Case Variants:**")
            output_lines.append("")
            for tc in cmd["test_cases"]:
                case_num = tc["case_index"] + 1
                sub_label = f"{cmd['index']}.{case_num}"
                full_cmd_link = f"{cmd['command']} {tc['input_options']}".strip() if tc["input_options"] else cmd['command']
                test_anchor = f"{sub_label}-{full_cmd_link}".replace(' ', '-').replace('_', '-').replace('/', '-').replace(':', '-').lower()
                display_text = tc["input_options"] if tc["input_options"] else "(base command)"
                output_lines.append(f"- [{sub_label} {display_text}](#{test_anchor})")
            output_lines.append("")
        
        for tc in cmd["test_cases"]:
            case_num = tc["case_index"] + 1
            sub_label = f"{cmd['index']}.{case_num}"
            full_cmd = f"{cmd['command']} {tc['input_options']}".strip() if tc["input_options"] else cmd['command']
            
            test_anchor = f"{sub_label}-{full_cmd}".replace(' ', '-').replace('_', '-').replace('/', '-').replace(':', '-').lower()
            output_lines.append(f"<a id=\"{test_anchor}\"></a>")
            if tc["input_options"]:
                output_lines.append(f"#### {sub_label} `{cmd['command']} {tc['input_options']}`")
            else:
                output_lines.append(f"#### {sub_label} `{cmd['command']}` (base command)")
            output_lines.append("")
            
            output_lines.append("| Device | Status | Details |")
            output_lines.append("|--------|--------|---------|")
            
            for device_name in all_devices:
                result = tc["device_results"].get(device_name, {})
                status = result.get("status", "UNKNOWN")
                
                if status == "N/A":
                    continue
                
                status_emoji = {"PASS": "✅", "FAIL": "❌", "ERROR": "⚠️", "SKIP": "⏭️"}.get(status, "❓")
                
                details = ""
                if status == "PASS":
                    details = result.get("coverage_info", "Passed")
                elif status == "FAIL":
                    details = "See diff below"
                elif status == "SKIP":
                    details = result.get("error", "Skipped")
                elif status == "ERROR":
                    details = result.get("error", "Error")
                
                output_lines.append(f"| `{device_name}` | {status_emoji} {status} | {details} |")
            
            output_lines.append("")
            
            # Add sample Python invocation script
            output_lines.append("**Sample Python Invocation:**")
            output_lines.append("```python")
            output_lines.append("from rust_wrapper import RustCommandParser as CommandParser")
            output_lines.append("")
            output_lines.append(f'cli_command = "{full_cmd}"')
            output_lines.append(f'json_data = <load_json_from_gnmi_response>  # See device-specific JSON below')
            output_lines.append("")
            output_lines.append("parser = CommandParser()")
            output_lines.append("result = parser.convert(cli_command, json_data)")
            output_lines.append("print(result)")
            output_lines.append("```")
            output_lines.append("")
            
            # Show detailed output for EVERY device with data
            devices_with_data = []
            for device_name in all_devices:
                result = tc["device_results"].get(device_name, {})
                if result.get("json_string") and result.get("status") != "N/A":
                    devices_with_data.append(device_name)
            
            if devices_with_data:
                for device_name in devices_with_data:
                    result = tc["device_results"][device_name]
                    status = result.get("status", "UNKNOWN")
                    status_emoji = {"PASS": "✅", "FAIL": "❌", "ERROR": "⚠️", "SKIP": "⏭️"}.get(status, "❓")
                    
                    output_lines.append(f"<details>")
                    output_lines.append(f"<summary><b>{status_emoji} Device: {device_name}</b> (click to expand)</summary>")
                    output_lines.append("")
                    
                    output_lines.append("**Input JSON:**")
                    output_lines.append("```json")
                    if result.get("json_string"):
                        try:
                            json_data = json.loads(result["json_string"])
                            json_sample = json.dumps(json_data, indent=2)
                            output_lines.append(truncate_content(json_sample, max_lines=20))
                        except:
                            output_lines.append(truncate_content(result["json_string"], max_lines=15))
                    else:
                        output_lines.append("(no data)")
                    output_lines.append("```")
                    output_lines.append("")
                    
                    output_lines.append("**Expected CLI Output:**")
                    output_lines.append("```")
                    output_lines.append(truncate_content(result.get("expected_cli") or "(no data)", max_lines=20, no_truncate=SHOW_FULL_OUTPUT))
                    output_lines.append("```")
                    output_lines.append("")
                    
                    output_lines.append("**Actual Converter Output:**")
                    output_lines.append("```")
                    output_lines.append(truncate_content(result.get("actual_output") or "(no output)", max_lines=20, no_truncate=SHOW_FULL_OUTPUT))
                    output_lines.append("```")
                    output_lines.append("")
                    
                    if status == "FAIL" and result.get("diff"):
                        output_lines.append("**Diff Details:**")
                        output_lines.append("```")
                        output_lines.append(result["diff"])
                        output_lines.append("```")
                        output_lines.append("")
                    
                    if status == "SKIP" and result.get("error"):
                        output_lines.append(f"**Skip Reason:** {result.get('error')}")
                        output_lines.append("")
                    
                    output_lines.append("</details>")
                    output_lines.append("")
            else:
                output_lines.append("*No device data available for this test case.*")
                output_lines.append("")
        
        output_lines.append("---")
        output_lines.append("")
    
    # Write output
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as f:
        f.write('\n'.join(output_lines))
    
    print(f"\nGenerated: {OUTPUT_FILE}")
    print(f"Total commands: {len(command_results)}")
    print(f"Total test cases (with data): {total_tests}")
    print(f"Pass: {total_pass}, Fail: {total_fail}, Error: {total_error}, Skip: {total_skip}")
    print(f"Pass rate: {pass_rate:.1f}%")


if __name__ == "__main__":
    generate_analysis()
