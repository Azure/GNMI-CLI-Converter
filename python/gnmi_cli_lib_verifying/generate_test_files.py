#!/usr/bin/env python3
"""
Generate test_*.py files from test_data/*.json files.

Each test file references its corresponding data file and runs verification
tests for all test cases (command + input_options variants).
"""
import json
import re
from pathlib import Path


TEST_TEMPLATE = '''#!/usr/bin/env python3
"""
Verification script for: {command}
Command index: {cmd_index}

This test loads explicit JSON and CLI strings from:
  test_data/{data_filename}

Data structure:
{{
    "command": "...",
    "test_cases": [
        {{
            "input_options": "",  # or "--option value"
            "devices": {{
                "device_name": {{
                    "cli_file": "...",
                    "gnmi_file": "...",
                    "json_string": "...",
                    "cli_string": "...",
                    ...
                }}
            }}
        }}
    ]
}}

The test verifies that gnmi_cli_lib produces the expected CLI output
from the gNMI JSON data for each test case (base command and option variants).
"""
import sys
import json
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from gnmi_cli_lib.parser import CommandParser

# Import comparison logic from analysis generator
from generate_all_commands_analysis import (
    normalize_output,
    compare_outputs,
    extract_json_values,
    check_gnmi_coverage,
)

# Load test data
DATA_FILE = Path(__file__).parent / "test_data" / "{data_filename}"


def load_test_data():
    """Load test data from JSON file."""
    with open(DATA_FILE, 'r', encoding='utf-8') as f:
        return json.load(f)


def run_single_test(command: str, input_options: str, json_string: str, 
                    expected_cli: str, device: str) -> dict:
    """Run a single test case and return results."""
    result = {{
        "device": device,
        "input_options": input_options,
        "status": "UNKNOWN",
        "message": "",
    }}
    
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
            full_command = f"{{command}} {{input_options}}"
        
        # Run converter
        parser = CommandParser()
        actual_output = parser.parse(full_command, json_data)
        
        # Compare outputs
        is_match, match_type, details = compare_outputs(expected_cli, actual_output, json_data)
        
        if is_match:
            result["status"] = "PASS"
            result["message"] = match_type
        else:
            result["status"] = "FAIL"
            result["message"] = details
            result["expected"] = expected_cli
            result["actual"] = actual_output
            
    except json.JSONDecodeError as e:
        result["status"] = "SKIP"
        result["message"] = f"JSON extraction error: {{e}}"
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
        devices = tc.get("devices", {{}})
        
        for device, device_data in devices.items():
            json_string = device_data.get("json_string")
            cli_string = device_data.get("cli_string", "")
            json_error = device_data.get("json_error")
            
            if json_error:
                results.append({{
                    "device": device,
                    "input_options": input_options,
                    "status": "SKIP",
                    "message": json_error,
                }})
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
    pass_count = sum(1 for r in results if r["status"] == "PASS")
    fail_count = sum(1 for r in results if r["status"] == "FAIL")
    skip_count = sum(1 for r in results if r["status"] == "SKIP")
    error_count = sum(1 for r in results if r["status"] == "ERROR")
    
    print(f"\\nResults: Pass={{pass_count}}, Fail={{fail_count}}, Skip={{skip_count}}, Error={{error_count}}")
    
    # Show failures
    failures = [r for r in results if r["status"] == "FAIL"]
    if failures:
        print("\\nFailures:")
        for f in failures[:5]:
            print(f"  - {{f['device']}} [{{f['input_options'] or 'no args'}}]: {{f['message'][:80]}}")
    
    return fail_count == 0 and error_count == 0


if __name__ == "__main__":
    print(f"Testing: {command}")
    print(f"Data file: {{DATA_FILE}}")
    
    results = run_all_tests()
    success = print_results(results)
    
    sys.exit(0 if success else 1)
'''


def generate_test_files():
    """Generate test files for all data files."""
    base_dir = Path(__file__).parent
    test_data_dir = base_dir / "test_data"
    
    generated = 0
    
    for data_file in sorted(test_data_dir.glob("data_*.json")):
        # Extract number and command from filename
        match = re.match(r"data_(\d+)_show_(.+)\.json", data_file.name)
        if not match:
            continue
        
        cmd_index = match.group(1)
        cmd_name = match.group(2)
        
        # Load data to get command
        with open(data_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        command = data.get("command", f"show {cmd_name.replace('_', ' ')}")
        
        # Generate test file content
        content = TEST_TEMPLATE.format(
            command=command,
            cmd_index=cmd_index,
            data_filename=data_file.name,
        )
        
        # Write test file
        test_filename = f"test_{cmd_index}_show_{cmd_name}.py"
        test_file = base_dir / test_filename
        test_file.write_text(content, encoding='utf-8')
        
        print(f"Generated: {test_filename}")
        generated += 1
    
    print(f"\nTotal generated: {generated} test files")


if __name__ == "__main__":
    generate_test_files()

