# Rust – gNMI CLI Library & Verification Suite

This directory contains the **Rust implementation** of the gNMI CLI converter and its companion verification/integration-testing toolchain.

## Directory Layout

| Directory | Purpose |
|-----------|---------|
| [`gnmi_cli_lib/`](gnmi_cli_lib/) | **Library & CLI binary** – core Rust crate that converts SONiC gNMI telemetry JSON into human-readable CLI table output |
| [`gnmi_cli_lib_verifying/`](gnmi_cli_lib_verifying/) | **Verification & integration test suite** – Python scripts that invoke the compiled Rust binary via a wrapper (`rust_wrapper.py`) to run end-to-end tests, compare outputs, and generate reports |

---

## `gnmi_cli_lib/` – Library Code

The core Rust crate (~29 100 lines, 25 source files) providing:

- **70 verified commands** across 19 categories (interfaces, transceiver, IPv6, queue, watermark, etc.)
- **78 concrete formatters** organized into 9 formatter modules
- **5 output table formats** – Simple, Grid, Plain, Github, Pipe
- Usable as a **library crate** or **standalone CLI binary** (`gnmi-cli-convert`)

### Key Documentation

- [gnmi_cli_lib/README.md](gnmi_cli_lib/README.md) – Build instructions, API usage, architecture, and module reference

### Quick Start

```bash
cd gnmi_cli_lib
cargo build --release   # Build
cargo test              # Run unit tests
cargo clippy            # Lint
cargo bench             # Benchmarks
```

---

## `gnmi_cli_lib_verifying/` – Test Suite & Integration Tests

A Python-based verification framework that wraps the compiled Rust binary (`gnmi-cli-convert`) via subprocess calls (`rust_wrapper.py`) and compares its output against known-good reference outputs from real SONiC devices.

### What's Inside

| Component | Description |
|-----------|-------------|
| `run_all_tests.py` | Main test runner – executes all `test_*.py` files |
| `rust_wrapper.py` | Python wrapper that invokes the compiled Rust binary (`gnmi-cli-convert`) via subprocess, exposing the same API so test scripts can call it like a library |
| `test_01_…` through `test_70_…` | **70 individual test files** covering every supported CLI command |
| `generate_test_files.py` | Auto-generates test files from the command table |
| `generate_all_commands_analysis.py` | Produces the full analysis report |
| `comparison.py` | Output comparison utilities |

### Test Results & Reports

- [gnmi_cli_lib_verifying/ALL_COMMANDS_ANALYSIS.md](gnmi_cli_lib_verifying/ALL_COMMANDS_ANALYSIS.md) – comprehensive command-by-command verification report (695 test cases, 6 devices, **100% pass rate**)
- [gnmi_cli_lib_verifying/README.md](gnmi_cli_lib_verifying/README.md) – how to run tests, add new tests, and troubleshoot failures

### Quick Start

```bash
# 1. Build the Rust binary first
cd gnmi_cli_lib && cargo build --release && cd ..

# 2. Run the full verification suite
cd gnmi_cli_lib_verifying
python run_all_tests.py          # Expect 695/695 PASS (100%)
python run_all_tests.py --verbose  # With detailed output

# 3. Regenerate the analysis report
python generate_all_commands_analysis.py
```

---

## Documentation Index

| Document | Location | Description |
|----------|----------|-------------|
| Library README | [gnmi_cli_lib/README.md](gnmi_cli_lib/README.md) | Architecture, API, build & usage guide |
| Verification README | [gnmi_cli_lib_verifying/README.md](gnmi_cli_lib_verifying/README.md) | Test runner usage, adding tests, troubleshooting |
| All Commands Analysis | [gnmi_cli_lib_verifying/ALL_COMMANDS_ANALYSIS.md](gnmi_cli_lib_verifying/ALL_COMMANDS_ANALYSIS.md) | Full verification report with per-device & per-command results |
