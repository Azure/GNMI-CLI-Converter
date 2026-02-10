# gNMI CLI Lib Verification Suite

This directory contains Python-based end-to-end verification tests that compare the Rust `gnmi-cli-convert` binary output against known-good reference outputs from the original Python implementation.

## Purpose

The verification suite ensures that the Rust port (`gnmi_cli_lib`) produces output that matches the original Python `gnmi_cli_lib` for all supported SONiC CLI commands (73+ commands, 398 test cases).

## Prerequisites

- **Rust binary**: Build the `gnmi-cli-convert` binary first:
  ```bash
  cd ../gnmi_cli_lib
  cargo build --release
  ```
- **Python 3.8+**: Required to run the test scripts
- **Dependencies**: `json`, `subprocess`, `os`, `sys` (all standard library)

## Directory Structure

```
gnmi_cli_lib_verifying/
├── run_all_tests.py              # Main test runner (runs all test_*.py files)
├── rust_wrapper.py               # Wrapper matching Python gnmi_cli_lib API
├── generate_all_commands_analysis.py  # Generate ALL_COMMANDS_ANALYSIS.md report
├── generate_test_files.py        # Auto-generate test files from command table
├── test_01_show_buffer_pool_watermark.py
├── test_02_show_buffer_pool_persistent_watermark.py
├── ...
├── test_70_show_ipv6_protocol.py
└── temp_*.json                   # Temporary test data files (auto-generated)
```

## How to Run Tests

### Run all tests
```bash
python run_all_tests.py
```

Output shows PASS/FAIL for each test case. Expected: **398/398 PASS (100%)**.

### Run a single test
```bash
python test_01_show_buffer_pool_watermark.py
```

### Run with verbose output
```bash
python run_all_tests.py --verbose
```

## How to Generate the Analysis Report

```bash
python generate_all_commands_analysis.py
```

This generates `ALL_COMMANDS_ANALYSIS.md` with:
- Command-by-command verification results
- Output comparison (Rust vs Python)
- Pass/fail summary statistics

## Test Data Format

Each test file contains JSON test data embedded inline or loaded from `temp_*.json` files. The test data represents gNMI JSON responses from the SONiC switch that the converter transforms into CLI-style tabular output.

## Adding New Tests

1. Create a new `test_NN_show_<command>.py` file
2. Define test data as JSON
3. Use `rust_wrapper.py` to invoke the converter
4. Compare output against expected reference

Or use `generate_test_files.py` to auto-generate test files from the command table.

## Troubleshooting

- **Binary not found**: Ensure `cargo build --release` was run in `../gnmi_cli_lib/`
- **Test failure**: Compare actual vs expected output — differences usually indicate a formatter bug
- **Import errors**: Run from this directory or add it to `PYTHONPATH`
