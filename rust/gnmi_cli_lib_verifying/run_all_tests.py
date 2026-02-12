#!/usr/bin/env python3
"""
Master test runner for gnmi_cli_lib verification (Rust version).
Uses multi-step verification to compare Rust binary output against expected CLI output.

This script:
1. Loads all test data from test_data/*.json
2. Runs the Rust converter for each command/device/variant combination
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

# Directories
SCRIPT_DIR = Path(__file__).parent.resolve()
TEST_DATA_DIR = SCRIPT_DIR / "test_data"
REPORT_FILE = SCRIPT_DIR / "verification_report.md"

# Devices
DEVICES = ["str4-7060x6-512-9", "str4-7060x6-512-13", "str4-7060x6-512-1", "str2-8102-01", "str3-8101-02", "str4-sn5640-7"]



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
                    "status": "SKIP",
                    "message": "No data for device"
                }
                continue
            
            device_data = devices_data[device]
            json_string = device_data.get("json_string")
            json_error = device_data.get("json_error")
            
            if json_error:
                tc_result["results"][device] = {
                    "status": "SKIP",
                    "message": f"JSON extraction error: {json_error}"
                }
                continue
            
            if not json_string or json_string.strip() == "":
                tc_result["results"][device] = {
                    "status": "SKIP",
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
            
            # Use multi-step comparison:
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
    
    # Count stats per device (exclude SKIP, merge ERROR into FAIL)
    stats = {device: {"PASS": 0, "FAIL": 0} for device in DEVICES}
    for tcr in all_test_case_results:
        if tcr["status"] == "SKIP":
            continue
        if tcr["status"] in ("FAIL", "ERROR"):
            stats[tcr["device"]]["FAIL"] += 1
        else:
            stats[tcr["device"]][tcr["status"]] += 1
    
    # Calculate totals
    total_pass = sum(s["PASS"] for s in stats.values())
    total_fail = sum(s["FAIL"] for s in stats.values())
    total_tests = total_pass + total_fail
    pass_rate = (total_pass / total_tests) * 100 if total_tests > 0 else 0
    
    lines = [
        "# gNMI CLI Library Verification Report",
        "",
        f"Generated: {timestamp}",
        "",
        "## Summary",
        "",
        f"**Total Test Cases:** {total_tests}",
        f"**Pass Rate:** {pass_rate:.1f}% ({total_pass} Pass / {total_tests} Total)",
        "",
        "| Device | PASS | FAIL | Total |",
        "|--------|------|------|-------|",
    ]
    
    for device in DEVICES:
        s = stats[device]
        total = sum(s.values())
        lines.append(f"| {device} | {s['PASS']} | {s['FAIL']} | {total} |")
    
    lines.extend([
        "",
        f"| **Total** | **{total_pass}** | **{total_fail}** | **{total_tests}** |",
        "",
        "## Detailed Results",
        "",
    ])
    
    # Group failures by command (ERROR is also FAIL)
    failures = [tcr for tcr in all_test_case_results if tcr["status"] in ("FAIL", "ERROR")]
    
    if failures:
        lines.extend([
            "### ❌ Failures",
            "",
            "| Command | Options | Device | Details |",
            "|---------|---------|--------|----------|",
        ])
        for f in failures:
            opts = f["input_options"] if f["input_options"] else "(default)"
            lines.append(f"| `{f['command']}` | {opts} | {f['device']} | {f['message']} |")
        lines.append("")
    
    # Summary by command (exclude SKIP, merge ERROR into FAIL)
    lines.extend([
        "### ✅ Pass Summary by Command",
        "",
        "| Command | Pass | Fail |",
        "|---------|------|------|",
    ])
    
    cmd_stats = {}
    for tcr in all_test_case_results:
        if tcr["status"] == "SKIP":
            continue
        cmd = tcr["command"]
        if cmd not in cmd_stats:
            cmd_stats[cmd] = {"PASS": 0, "FAIL": 0}
        if tcr["status"] in ("FAIL", "ERROR"):
            cmd_stats[cmd]["FAIL"] += 1
        else:
            cmd_stats[cmd][tcr["status"]] += 1
    
    for cmd in sorted(cmd_stats.keys()):
        s = cmd_stats[cmd]
        lines.append(f"| `{cmd}` | {s['PASS']} | {s['FAIL']} |")
    
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
    
    for i, test_data in enumerate(all_test_data, 1):
        command = test_data.get("command", "unknown")
        test_cases = test_data.get("test_cases", [])
        print(f"[{i:02d}/{len(all_test_data):02d}] Testing: {command} ({len(test_cases)} variants)")
        
        result = run_single_test(test_data, parser)
        all_results.append(result)
        
        # Count results for this command (exclude SKIP, merge ERROR into FAIL)
        for tc in result["test_cases"]:
            for device in DEVICES:
                if device in tc["results"]:
                    status = tc["results"][device]["status"]
                    if status == "SKIP":
                        continue
                    total_tests += 1
                    if status == "PASS":
                        total_pass += 1
                    else:  # FAIL or ERROR
                        total_fail += 1
    
    print()
    print("=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Total Test Cases: {total_tests}")
    print(f"  PASS:  {total_pass}")
    print(f"  FAIL:  {total_fail}")
    
    if total_tests > 0:
        pass_rate = total_pass / total_tests * 100
        print(f"\nPass Rate: {pass_rate:.1f}% ({total_pass}/{total_tests})")
    
    # Generate report
    print()
    print(f"Generating report: {REPORT_FILE}")
    report = generate_report(all_results)
    with open(REPORT_FILE, 'w', encoding='utf-8') as f:
        f.write(report)
    print("Done!")
    
    # Return exit code based on failures
    return 0 if total_fail == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
