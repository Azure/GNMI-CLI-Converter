#!/usr/bin/env python3
"""
Pytest Integration for Real Device Tests

This module provides pytest-compatible test cases for running real device
verification tests. It dynamically generates test cases from the JSON test
data files, allowing for granular test reporting and CI integration.

Usage:
    # Run all real device tests
    pytest real_device_tests/test_real_device.py -v
    
    # Run tests for a specific command
    pytest real_device_tests/test_real_device.py -v -k "buffer_pool"
    
    # Run tests for a specific device
    pytest real_device_tests/test_real_device.py -v -k "str2_8102_01"
    
    # Run with coverage
    pytest real_device_tests/test_real_device.py -v --cov=.. --cov-report=html

Test Organization:
    Tests are organized by command and device. Each test case is named:
    test_<command>__<device>__<options>
    
    For example:
    - test_show_buffer_pool_watermark__str2_8102_01__
    - test_show_interfaces_counters__str4_7060x6_512_1__Ethernet0

Verification Approach:
    Uses multi-step comparison from verification.py:
    1. Exact match
    2. Spacing-normalized match
    3. Help option ordering match
    4. gNMI data coverage check
    5. BGP-specific acceptable nuances
"""

import json
import pytest
import sys
from pathlib import Path
from typing import Generator, Tuple, Any, Dict

# Ensure parent directory is in path for imports
SCRIPT_DIR = Path(__file__).parent.resolve()
LIB_DIR = SCRIPT_DIR.parent
sys.path.insert(0, str(LIB_DIR.parent))

from gnmi_cli_lib.parser import CommandParser
from gnmi_cli_lib.real_device_tests.verification import compare_outputs

# Test data directory
TEST_DATA_DIR = SCRIPT_DIR / "test_data"

# Devices covered in test data
DEVICES = [
    "str4-7060x6-512-9",
    "str4-7060x6-512-13",
    "str4-7060x6-512-1",
    "str2-8102-01"
]


def load_test_data() -> Generator[Dict[str, Any], None, None]:
    """Load all test data JSON files."""
    test_files = sorted(TEST_DATA_DIR.glob("data_*.json"))
    for f in test_files:
        with open(f, 'r', encoding='utf-8') as fp:
            data = json.load(fp)
            data['_file'] = f.name
            yield data


def generate_test_cases() -> Generator[Tuple[str, str, str, Dict[str, Any], Dict[str, Any]], None, None]:
    """Generate test cases as tuples of (test_id, full_command, device, device_data, test_data).
    
    Yields:
        Tuple containing:
        - test_id: Unique test identifier for pytest
        - full_command: Full CLI command string
        - device: Device name
        - device_data: Device-specific test data (json_string, cli_string, etc.)
        - test_meta: Metadata about the test case (command, options)
    """
    for test_data in load_test_data():
        command = test_data.get("command", "unknown")
        test_cases = test_data.get("test_cases", [])
        
        for test_case in test_cases:
            devices_data = test_case.get("devices", {})
            input_options = test_case.get("input_options", "")
            full_command = f"{command} {input_options}".strip() if input_options else command
            
            for device in DEVICES:
                if device not in devices_data:
                    continue
                
                device_data = devices_data[device]
                
                # Skip cases with known data issues
                if device_data.get("json_error"):
                    continue
                json_string = device_data.get("json_string")
                if not json_string or json_string.strip() == "":
                    continue
                
                # Generate test ID
                cmd_part = command.replace(" ", "_").replace("-", "_")
                device_part = device.replace("-", "_")
                opts_part = input_options.replace(" ", "_").replace("-", "_").replace("/", "_") if input_options else ""
                test_id = f"{cmd_part}__{device_part}__{opts_part}"
                
                test_meta = {
                    "command": command,
                    "input_options": input_options,
                    "full_command": full_command,
                    "_file": test_data.get("_file", "")
                }
                
                yield test_id, full_command, device, device_data, test_meta


# Collect all test cases
TEST_CASES = list(generate_test_cases())


# Create a shared parser instance
@pytest.fixture(scope="module")
def parser():
    """Provide a shared CommandParser instance for all tests."""
    return CommandParser()


class TestRealDeviceConversion:
    """Test class for real device data verification."""
    
    @pytest.mark.parametrize(
        "test_id,full_command,device,device_data,test_meta",
        TEST_CASES,
        ids=[tc[0] for tc in TEST_CASES]
    )
    def test_converter_output(
        self,
        parser: CommandParser,
        test_id: str,
        full_command: str,
        device: str,
        device_data: Dict[str, Any],
        test_meta: Dict[str, Any]
    ):
        """Test that converter output matches expected CLI output or covers gNMI data.
        
        This test uses a multi-step verification approach:
        1. Exact match - output exactly matches expected CLI
        2. Spacing match - output matches after normalizing table spacing
        3. Help match - output matches after normalizing help option order
        4. Coverage match - all gNMI JSON values appear in output
        5. BGP nuance - acceptable BGP-specific differences
        
        Args:
            parser: CommandParser instance
            test_id: Unique test identifier
            full_command: Full CLI command being tested
            device: Device name
            device_data: Device-specific data with json_string, cli_string
            test_meta: Metadata about the test
        """
        # Parse input JSON
        json_string = device_data.get("json_string")
        try:
            data = json.loads(json_string)
        except json.JSONDecodeError as e:
            pytest.fail(f"JSON parse error: {e}")
        
        # Run converter
        try:
            actual_output = parser.parse_and_convert(full_command, data)
        except Exception as e:
            pytest.fail(f"Converter error: {type(e).__name__}: {e}")
        
        # Get expected output
        expected_output = device_data.get("cli_string", "")
        
        # Compare with multi-step verification
        is_pass, diff_info, match_reason = compare_outputs(
            actual_output, expected_output, data, full_command
        )
        
        if not is_pass:
            # Build detailed failure message
            failure_msg = [
                f"Command: {full_command}",
                f"Device: {device}",
                f"Match failed: {diff_info}",
                "",
                "Expected (first 500 chars):",
                expected_output[:500] if expected_output else "(empty)",
                "",
                "Actual (first 500 chars):",
                actual_output[:500] if actual_output else "(empty)"
            ]
            pytest.fail('\n'.join(failure_msg))
        
        # Test passed - optionally log match reason
        # print(f"  ✓ {match_reason}")


class TestRealDeviceSummary:
    """Summary tests for real device verification."""
    
    def test_all_test_data_files_loaded(self):
        """Verify all 70 test data files are present."""
        test_files = list(TEST_DATA_DIR.glob("data_*.json"))
        assert len(test_files) == 70, f"Expected 70 test data files, found {len(test_files)}"
    
    def test_all_devices_represented(self):
        """Verify test data includes all expected devices."""
        devices_found = set()
        for test_data in load_test_data():
            for tc in test_data.get("test_cases", []):
                devices_found.update(tc.get("devices", {}).keys())
        
        for device in DEVICES:
            assert device in devices_found, f"Device {device} not found in test data"
    
    def test_minimum_test_cases(self):
        """Verify minimum number of executable test cases."""
        # We expect at least 398 executable test cases (PASS cases from verification)
        assert len(TEST_CASES) >= 390, f"Expected at least 390 test cases, found {len(TEST_CASES)}"


# Convenience function for running tests programmatically
def run_tests(verbose: bool = True) -> int:
    """Run all tests and return exit code.
    
    Args:
        verbose: If True, use verbose output
        
    Returns:
        0 if all tests pass, non-zero otherwise
    """
    args = [__file__]
    if verbose:
        args.append("-v")
    return pytest.main(args)


if __name__ == "__main__":
    sys.exit(run_tests())

