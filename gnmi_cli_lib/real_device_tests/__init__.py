#!/usr/bin/env python3
"""
Real Device Tests Package

This package contains comprehensive tests using real device data captured from 
production SONiC switches. It provides a robust verification framework that ensures 
the gNMI to CLI converter produces outputs matching actual SONiC CLI behavior.

Modules:
    verification: Core verification logic with multi-step comparison
    test_runner: Master test runner for all real device tests
    test_real_device: Pytest integration for real device tests

Test Data:
    test_data/: Contains 70 JSON files with CLI/gNMI captures from 4 real devices
    
Devices Covered:
    - str4-7060x6-512-9
    - str4-7060x6-512-13  
    - str4-7060x6-512-1
    - str2-8102-01

Usage:
    # Run from gnmi_cli_lib directory:
    python -m real_device_tests.test_runner
    
    # Or run with pytest:
    pytest real_device_tests/test_real_device.py -v
"""

from .verification import (
    compare_outputs,
    check_gnmi_coverage,
    normalize_output,
    normalize_line_spacing,
)

__all__ = [
    'compare_outputs',
    'check_gnmi_coverage', 
    'normalize_output',
    'normalize_line_spacing',
]

