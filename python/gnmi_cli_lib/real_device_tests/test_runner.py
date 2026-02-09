#!/usr/bin/env python3
"""
Real Device Test Runner

This module provides a master test runner for verifying the gNMI to CLI converter
against real device data. It loads all test data files, runs the converter for 
each command/device/variant combination, and generates a comprehensive report.

Usage:
    # Run from gnmi_cli_lib directory:
    python -m real_device_tests.test_runner
    
    # Or run this script directly:
    cd gnmi_cli_lib
    python real_device_tests/test_runner.py

Test Data:
    test_data/*.json - 70 JSON files containing CLI and gNMI captures from real devices
    
Verification Approach:
    Uses multi-step comparison (see verification.py):
    1. Exact match
    2. Spacing-normalized match
    3. Help option ordering match
    4. gNMI data coverage check
    5. BGP-specific acceptable nuances
"""

import json
import sys
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any, Optional

# Determine script location and add parent to path
SCRIPT_DIR = Path(__file__).parent.resolve()
LIB_DIR = SCRIPT_DIR.parent
sys.path.insert(0, str(LIB_DIR.parent))

from gnmi_cli_lib.parser import CommandParser
from gnmi_cli_lib.real_device_tests.verification import compare_outputs

# Directories
TEST_DATA_DIR = SCRIPT_DIR / "test_data"
REPORT_FILE = SCRIPT_DIR / "verification_report.md"

# Devices covered in test data
DEVICES = [
    "str4-7060x6-512-9",
    "str4-7060x6-512-13", 
    "str4-7060x6-512-1",
    "str2-8102-01"
]


def load_all_test_data() -> List[Dict[str, Any]]:
    """Load all test data JSON files from the test_data directory.
    
    Returns:
        List of test data dictionaries, each containing command info and test cases
    """
    test_files = sorted(TEST_DATA_DIR.glob("data_*.json"))
    all_data = []
    for f in test_files:
        with open(f, 'r', encoding='utf-8') as fp:
            data = json.load(fp)
            data['_file'] = f.name
            all_data.append(data)
    return all_data


def run_single_test(
    test_data: Dict[str, Any], 
    parser: CommandParser
) -> Dict[str, Any]:
    """Run a single test for all devices and all test case variants.
    
    Args:
        test_data: Test data dictionary containing command and test cases
        parser: CommandParser instance for running conversions
        
    Returns:
        Dictionary with results per device per variant:
        {
            "command": "show ...",
            "test_cases": [
                {
                    "input_options": "",
                    "full_command": "show ...",
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


def generate_report(all_results: List[Dict[str, Any]]) -> str:
    """Generate a markdown verification report.
    
    Args:
        all_results: List of test results from run_single_test
        
    Returns:
        Markdown-formatted report string
    """
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
        "# Real Device Verification Report",
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
        "|---------|------|------|------------------|"  ,
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


class TestResults:
    """Container for test results with summary statistics."""
    
    def __init__(self):
        self.total_tests = 0
        self.total_pass = 0
        self.total_fail = 0
        self.total_error = 0
        self.total_no_acceptable_data = 0
        self.all_results = []
    
    @property
    def pass_rate(self) -> float:
        """Calculate pass rate as percentage."""
        executed = self.total_pass + self.total_fail
        if executed > 0:
            return self.total_pass / executed * 100
        return 0.0
    
    def add_result(self, result: Dict[str, Any]):
        """Add a single test result and update statistics."""
        self.all_results.append(result)
        for tc in result["test_cases"]:
            for device in DEVICES:
                if device in tc["results"]:
                    status = tc["results"][device]["status"]
                    self.total_tests += 1
                    if status == "PASS":
                        self.total_pass += 1
                    elif status == "FAIL":
                        self.total_fail += 1
                    elif status == "ERROR":
                        self.total_error += 1
                    else:
                        self.total_no_acceptable_data += 1


def run_all_tests(
    verbose: bool = True, 
    generate_report_file: bool = True
) -> TestResults:
    """Run all real device tests.
    
    Args:
        verbose: If True, print progress to stdout
        generate_report_file: If True, write report to verification_report.md
        
    Returns:
        TestResults object with all results and statistics
    """
    if verbose:
        print("=" * 60)
        print("gNMI CLI Library - Real Device Verification")
        print("=" * 60)
        print()
    
    # Load test data
    if verbose:
        print(f"Loading test data from: {TEST_DATA_DIR}")
    all_test_data = load_all_test_data()
    if verbose:
        print(f"Found {len(all_test_data)} test data files")
        print()
    
    # Create parser
    parser = CommandParser()
    
    # Run all tests
    results = TestResults()
    
    for i, test_data in enumerate(all_test_data, 1):
        command = test_data.get("command", "unknown")
        test_cases = test_data.get("test_cases", [])
        if verbose:
            print(f"[{i:02d}/{len(all_test_data):02d}] Testing: {command} ({len(test_cases)} variants)")
        
        result = run_single_test(test_data, parser)
        results.add_result(result)
    
    if verbose:
        print()
        print("=" * 60)
        print("SUMMARY")
        print("=" * 60)
        print(f"Total Test Cases: {results.total_tests}")
        print(f"  PASS:    {results.total_pass}")
        print(f"  FAIL:    {results.total_fail}")
        print(f"  ERROR:   {results.total_error}")
        print(f"  NO_ACCEPTABLE_DATA: {results.total_no_acceptable_data}")
        print(f"\nPass Rate: {results.pass_rate:.1f}% ({results.total_pass}/{results.total_pass + results.total_fail})")
    
    # Generate report
    if generate_report_file:
        if verbose:
            print()
            print(f"Generating report: {REPORT_FILE}")
        report = generate_report(results.all_results)
        with open(REPORT_FILE, 'w', encoding='utf-8') as f:
            f.write(report)
        if verbose:
            print("Done!")
    
    return results


def main():
    """Main entry point."""
    results = run_all_tests(verbose=True, generate_report_file=True)
    
    # Return exit code based on failures
    return 0 if results.total_fail == 0 and results.total_error == 0 else 1


if __name__ == "__main__":
    sys.exit(main())

