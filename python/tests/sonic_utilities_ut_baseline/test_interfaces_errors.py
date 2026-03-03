"""
Baseline UT: show interfaces errors (Cmd #19)

Source:
  - sonic-utilities UT File : interfaces_test.py
  - sonic-utilities Test Func: test_show_intf_errors_filled_data
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #19

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces errors"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = [
    {
        "Port Errors": "code group error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "crc rate",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "data unit crc error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "data unit misalignment error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "data unit size",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "fec alignment loss",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "fec sync loss",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "high ber error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "high ser error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "mac local fault",
        "Count": "26",
        "Last timestamp(UTC)": "2025-01-17 18:40:56"
    },
    {
        "Port Errors": "mac remote fault",
        "Count": "14483",
        "Last timestamp(UTC)": "2025-01-17 19:51:12"
    },
    {
        "Port Errors": "no rx reachability",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "oper error status",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    },
    {
        "Port Errors": "signal local error",
        "Count": "0",
        "Last timestamp(UTC)": "Never"
    }
]

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Port Errors                     Count  Last timestamp(UTC)
----------------------------  -------  ---------------------
code group error                    0  Never
crc rate                            0  Never
data unit crc error                 0  Never
data unit misalignment error        0  Never
data unit size                      0  Never
fec alignment loss                  0  Never
fec sync loss                       0  Never
high ber error                      0  Never
high ser error                      0  Never
mac local fault                    26  2025-01-17 18:40:56
mac remote fault                14483  2025-01-17 19:51:12
no rx reachability                  0  Never
oper error status                   0  Never
signal local error                  0  Never"""


@pytest.mark.sonic_baseline
class TestInterfacesErrors:
    """
    Baseline test for 'show interfaces errors' (Cmd #19).

    Mirrors sonic-utilities test:
      File: interfaces_test.py
      Func: test_show_intf_errors_filled_data
    """

    def test_converter_output_matches_sonic_ut(self, command_parser):
        """JSON -> Converter -> CLI text should match sonic-utilities expected output."""
        actual = command_parser.parse_and_convert(CLI_COMMAND, json.dumps(JSON_INPUT))
        assert actual.strip() == EXPECTED_CLI_OUTPUT.strip(), (
            f"Converter output for '{CLI_COMMAND}' does not match "
            f"sonic-utilities baseline.\n"
            f"--- Expected ---\n{EXPECTED_CLI_OUTPUT}\n"
            f"--- Actual ---\n{actual}"
        )
