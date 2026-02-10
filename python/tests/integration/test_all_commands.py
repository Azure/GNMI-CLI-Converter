"""
Parameterized integration test — replaces 70 individual test_*.py files.

This single file tests ALL commands × ALL variants × ALL devices by:
1. Loading JSON fixture data (via conftest parameterization)
2. Running the CommandParser converter
3. Comparing output using the 7-step comparison engine

To run:
    cd python/
    pytest tests/integration/test_all_commands.py -v
    pytest tests/integration/test_all_commands.py -v -k "show_interfaces_status"
    pytest tests/integration/test_all_commands.py -v -k "str4-7060x6-512-9"
"""

from __future__ import annotations

import pytest

from tests.core.runner import run_case
from tests.core.test_case import CaseStatus, VerificationCase


class TestAllCommands:
    """Parameterized integration test for all gNMI → CLI conversions.

    Each ``test_case`` is a ``(command, input_options, device)`` triple
    injected by ``conftest.py::pytest_generate_tests``.

    Delegates all execution logic to :func:`tests.core.runner.run_case`.
    """

    def test_gnmi_to_cli_conversion(self, test_case: VerificationCase, command_parser):
        """Verify that the converter reproduces the expected CLI output."""
        result = run_case(test_case, command_parser)

        if result.status == CaseStatus.SKIP:
            pytest.skip(result.message)

        if result.status == CaseStatus.ERROR:
            pytest.fail(f"Converter error: {result.message}")

        if result.status == CaseStatus.FAIL:
            lines = [
                f"Command: {test_case.full_command}",
                f"Device:  {test_case.device.device_name}",
                "",
                "--- Diff Info ---",
                result.message or "(no diff details)",
            ]
            if result.expected and result.actual:
                lines.extend([
                    "",
                    "--- Expected (first 500 chars) ---",
                    result.expected[:500],
                    "",
                    "--- Actual (first 500 chars) ---",
                    result.actual[:500],
                ])
            pytest.fail("\n".join(lines))
