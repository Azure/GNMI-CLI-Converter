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
from comparison import (
    normalize_output,
    normalize_help_options,
    normalize_line_spacing,
    normalize_timestamp_values,
    compare_with_normalized_spacing,
    compare_with_normalized_timestamps,
    extract_json_values,
    format_number_with_comma,
    normalize_numeric_value,
    normalize_string_value,
    check_multiline_string_coverage,
    check_truncated_value_coverage,
    check_all_entries_empty,
    check_gnmi_coverage,
    check_bgp_routes_acceptable_diff,
    compare_outputs,
    INTENTIONALLY_FILTERED_VALUES,
    CRITICAL_VALUE_TRANSFORMATIONS,
)

BASE_DIR = Path(__file__).parent
TEST_DATA_DIR = BASE_DIR / "test_data"
OUTPUT_FILE = BASE_DIR / "ALL_COMMANDS_ANALYSIS.md"


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


SHOW_FULL_OUTPUT = False  # Set to True to show full outputs without truncation in the detailed analysis


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
    device_stats = {device: {"pass": 0, "fail": 0, "total": 0} for device in all_devices}
    
    # Summary stats
    total_tests = 0
    total_pass = 0
    total_fail = 0
    
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
            "device_stats": {device: {"pass": 0, "fail": 0} for device in all_devices},
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
                
                # Skip cases where test data is not available
                if result["status"] == "SKIP":
                    continue
                
                total_tests += 1
                device_stats[device_name]["total"] += 1
                
                if result["status"] == "PASS":
                    total_pass += 1
                    cmd_result["pass_count"] += 1
                    cmd_result["device_stats"][device_name]["pass"] += 1
                    device_stats[device_name]["pass"] += 1
                else:  # FAIL or ERROR
                    total_fail += 1
                    cmd_result["fail_count"] += 1
                    cmd_result["device_stats"][device_name]["fail"] += 1
                    device_stats[device_name]["fail"] += 1
            
            cmd_result["test_cases"].append(case_result)
        
        command_results.append(cmd_result)
        print(f"  [{idx}] {command}: P={cmd_result['pass_count']} F={cmd_result['fail_count']}")
    
    # Overall Summary
    pass_rate = total_pass / total_tests * 100 if total_tests > 0 else 0
    
    output_lines.append("## Overall Summary")
    output_lines.append("")
    output_lines.append(f"| Metric | Count |")
    output_lines.append(f"|--------|-------|")
    output_lines.append(f"| Total Test Cases | {total_tests} |")
    output_lines.append(f"| ✅ PASS | {total_pass} |")
    output_lines.append(f"| ❌ FAIL | {total_fail} |")
    output_lines.append(f"| **Pass Rate** | **{pass_rate:.1f}%** |")
    output_lines.append("")
    
    # Per-Device Summary
    output_lines.append("## Per-Device Summary")
    output_lines.append("")
    output_lines.append("| Device | Total | ✅ Pass | ❌ Fail | Pass Rate |")
    output_lines.append("|--------|-------|---------|---------|-----------|")
    
    for device in all_devices:
        stats = device_stats[device]
        dev_total = stats["total"]
        dev_pass = stats["pass"]
        dev_fail = stats["fail"]
        dev_pass_rate = dev_pass / dev_total * 100 if dev_total > 0 else 0
        output_lines.append(
            f"| `{device}` | {dev_total} | {dev_pass} | {dev_fail} | {dev_pass_rate:.1f}% |"
        )
    output_lines.append("")
    
    # Quick reference table
    output_lines.append("## Quick Reference by Command")
    output_lines.append("")
    output_lines.append("| # | Command | Variants | Pass | Fail |")
    output_lines.append("|---|---------|----------|------|------|")
    
    for cmd in command_results:
        status_emoji = "✅" if cmd["fail_count"] == 0 else "❌"
        cmd_anchor = cmd['command'].replace(' ', '-').replace('_', '-').lower()
        output_lines.append(
            f"| {cmd['index']} | [`{cmd['command']}`](#{cmd['index']}-{cmd_anchor}) | {len(cmd['test_cases'])} | "
            f"{cmd['pass_count']} | {cmd['fail_count']} |"
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
        
        all_pass = cmd["fail_count"] == 0
        if all_pass:
            cmd_total = cmd['pass_count'] + cmd['fail_count']
            output_lines.append(f"**Status:** ✅ All {cmd['pass_count']}/{cmd_total} test cases passed (across {len(all_devices)} devices)")
        else:
            cmd_total = cmd['pass_count'] + cmd['fail_count']
            output_lines.append(f"**Status:** ❌ {cmd['fail_count']} failures out of {cmd_total} test cases")
        output_lines.append("")
        
        # Per-device summary for this command
        output_lines.append("**Per-Device Results:**")
        output_lines.append("")
        output_lines.append("| Device | Pass | Fail |")
        output_lines.append("|--------|------|------|")
        for device in all_devices:
            ds = cmd["device_stats"][device]
            output_lines.append(f"| `{device}` | {ds['pass']} | {ds['fail']} |")
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
                
                if status == "SKIP":
                    continue
                
                display_status = "FAIL" if status == "ERROR" else status
                status_emoji = {"PASS": "✅", "FAIL": "❌"}.get(display_status, "❓")
                
                details = ""
                if status == "PASS":
                    details = result.get("coverage_info", "Passed")
                elif status == "FAIL":
                    details = "See diff below"
                elif status == "ERROR":
                    details = result.get("error", "Error")
                
                output_lines.append(f"| `{device_name}` | {status_emoji} {display_status} | {details} |")
            
            output_lines.append("")
            
            # Add sample invocation script
            output_lines.append("**Sample Invocation:**")
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
            
            # Show detailed output for EVERY device with data (exclude SKIP/N/A)
            devices_with_data = []
            for device_name in all_devices:
                result = tc["device_results"].get(device_name, {})
                if result.get("json_string") and result.get("status") not in ("N/A", "SKIP"):
                    devices_with_data.append(device_name)
            
            if devices_with_data:
                for device_name in devices_with_data:
                    result = tc["device_results"][device_name]
                    status = result.get("status", "UNKNOWN")
                    display_status = "FAIL" if status == "ERROR" else status
                    status_emoji = {"PASS": "✅", "FAIL": "❌"}.get(display_status, "❓")
                    
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
    print(f"Total test cases: {total_tests}")
    print(f"Pass: {total_pass}, Fail: {total_fail}")
    print(f"Pass rate: {pass_rate:.1f}%")


if __name__ == "__main__":
    generate_analysis()
