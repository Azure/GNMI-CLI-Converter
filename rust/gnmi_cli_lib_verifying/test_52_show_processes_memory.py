#!/usr/bin/env python3
"""
Verification script for: show processes memory
Command index: 52

This test loads explicit JSON and CLI strings from:
  test_data/data_52_show_processes_memory.json

The test verifies that gnmi_cli_lib produces the expected CLI output
from the gNMI JSON data for each test case (base command and option variants).
"""
import sys
import json
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from rust_wrapper import RustCommandParser as CommandParser

# Import comparison logic from analysis generator
from generate_all_commands_analysis import (
    normalize_output,
    compare_outputs,
    extract_json_values,
    check_gnmi_coverage,
)

# Load test data
DATA_FILE = Path(__file__).parent / "test_data" / "data_52_show_processes_memory.json"


def load_test_data():
    """Load test data from JSON file."""
    with open(DATA_FILE, 'r', encoding='utf-8') as f:
        return json.load(f)


def run_single_test(command: str, input_options: str, json_string: str, 
                    expected_cli: str, device: str) -> dict:
    """Run a single test case and return results."""
    result = {
        "device": device,
        "input_options": input_options,
        "status": "UNKNOWN",
        "message": "",
    }
    
    if not json_string:
        result["status"] = "SKIP"
        result["message"] = "No JSON data available"
        return result
    
    try:
        # Parse JSON
        json_data = json.loads(json_string)
        
        # Build full command
        full_command = command
        if input_options:
            full_command = f"{command} {input_options}"
        
        # Run converter
        parser = CommandParser()
        actual_output = parser.parse_and_convert(full_command, json_data)
        
        # Compare outputs (signature: actual, expected, json_data, cli_command)
        is_match, diff_info, match_reason = compare_outputs(actual_output, expected_cli, json_data, full_command)
        
        if is_match:
            result["status"] = "PASS"
            result["message"] = match_reason or "Passed"
        else:
            result["status"] = "FAIL"
            result["message"] = diff_info or "Mismatch"
            result["expected"] = expected_cli
            result["actual"] = actual_output
            
    except json.JSONDecodeError as e:
        result["status"] = "SKIP"
        result["message"] = f"JSON extraction error: {e}"
    except Exception as e:
        result["status"] = "ERROR"
        result["message"] = str(e)
    
    return result


def run_all_tests():
    """Run all test cases from the data file."""
    data = load_test_data()
    command = data["command"]
    test_cases = data.get("test_cases", [])
    
    results = []
    
    for tc in test_cases:
        input_options = tc.get("input_options", "")
        devices = tc.get("devices", {})
        
        for device, device_data in devices.items():
            json_string = device_data.get("json_string")
            cli_string = device_data.get("cli_string", "")
            json_error = device_data.get("json_error")
            
            if json_error:
                results.append({
                    "device": device,
                    "input_options": input_options,
                    "status": "SKIP",
                    "message": json_error,
                })
                continue
            
            result = run_single_test(
                command=command,
                input_options=input_options,
                json_string=json_string,
                expected_cli=cli_string,
                device=device,
            )
            results.append(result)
    
    return results


def print_results(results):
    """Print test results summary."""
    # Exclude SKIP (no test data available), count ERROR as FAIL
    actionable = [r for r in results if r["status"] not in ("SKIP",)]
    pass_count = sum(1 for r in actionable if r["status"] == "PASS")
    fail_count = sum(1 for r in actionable if r["status"] in ("FAIL", "ERROR"))
    total = pass_count + fail_count
    
    print(f"\nResults: Pass={pass_count}, Fail={fail_count}, Total={total}")
    
    # Show failures
    failures = [r for r in actionable if r["status"] in ("FAIL", "ERROR")]
    if failures:
        print("\nFailures:")
        for f in failures[:5]:
            print(f"  - {f['device']} [{f['input_options'] or 'no args'}]: {(f.get('message') or 'Unknown')[:80]}")
    
    return fail_count == 0


if __name__ == "__main__":
    print(f"Testing: show processes memory")
    print(f"Data file: {DATA_FILE}")
    
    results = run_all_tests()
    success = print_results(results)
    
    sys.exit(0 if success else 1)
