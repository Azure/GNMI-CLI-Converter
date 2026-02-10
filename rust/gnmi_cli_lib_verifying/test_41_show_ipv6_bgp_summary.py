#!/usr/bin/env python3
"""Verification script for: show ipv6 bgp summary - Command index: 41"""
import sys, json
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent))
from rust_wrapper import RustCommandParser as CommandParser
from generate_all_commands_analysis import normalize_output, compare_outputs, extract_json_values, check_gnmi_coverage
DATA_FILE = Path(__file__).parent / "test_data" / "data_41_show_ipv6_bgp_summary.json"
def load_test_data():
    with open(DATA_FILE, 'r', encoding='utf-8') as f: return json.load(f)
def run_single_test(command, input_options, json_string, expected_cli, device):
    result = {"device": device, "input_options": input_options, "status": "UNKNOWN", "message": ""}
    if not json_string: result["status"] = "SKIP"; result["message"] = "No JSON data available"; return result
    try:
        json_data = json.loads(json_string)
        full_command = f"{command} {input_options}".strip() if input_options else command
        parser = CommandParser(); actual_output = parser.parse(full_command, json_data)
        is_match, match_type, details = compare_outputs(actual_output, expected_cli, json_data)
        if is_match: result["status"] = "PASS"; result["message"] = match_type
        else: result["status"] = "FAIL"; result["message"] = details; result["expected"] = expected_cli; result["actual"] = actual_output
    except json.JSONDecodeError as e: result["status"] = "SKIP"; result["message"] = f"JSON extraction error: {e}"
    except Exception as e: result["status"] = "ERROR"; result["message"] = str(e)
    return result
def run_all_tests():
    data = load_test_data(); command = data["command"]; test_cases = data.get("test_cases", []); results = []
    for tc in test_cases:
        input_options = tc.get("input_options", ""); devices = tc.get("devices", {})
        for device, device_data in devices.items():
            json_string = device_data.get("json_string"); cli_string = device_data.get("cli_string", ""); json_error = device_data.get("json_error")
            if json_error: results.append({"device": device, "input_options": input_options, "status": "SKIP", "message": json_error}); continue
            result = run_single_test(command, input_options, json_string, cli_string, device); results.append(result)
    return results
def print_results(results):
    pass_count = sum(1 for r in results if r["status"] == "PASS"); fail_count = sum(1 for r in results if r["status"] == "FAIL")
    skip_count = sum(1 for r in results if r["status"] == "SKIP"); error_count = sum(1 for r in results if r["status"] == "ERROR")
    print(f"\nResults: Pass={pass_count}, Fail={fail_count}, Skip={skip_count}, Error={error_count}")
    failures = [r for r in results if r["status"] == "FAIL"]
    if failures: print("\nFailures:"); [print(f"  - {f['device']} [{f['input_options'] or 'no args'}]: {f['message'][:80]}") for f in failures[:5]]
    return fail_count == 0 and error_count == 0
if __name__ == "__main__":
    data = load_test_data(); print(f"Testing: {data['command']}"); print(f"Data file: {DATA_FILE}")
    results = run_all_tests(); success = print_results(results); sys.exit(0 if success else 1)
