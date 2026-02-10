"""
Unified test case executor — single source of truth.

This module is the **only** place where test execution logic lives.
Both ``test_all_commands.py`` (pytest) and ``generate_analysis.py`` (report)
delegate to :func:`run_case` instead of duplicating the
skip → parse → convert → compare pipeline.
"""

from __future__ import annotations

import json
from typing import Any

from tests.core.comparison import compare_outputs
from tests.core.test_case import CaseResult, CaseStatus, VerificationCase


def run_case(tc: VerificationCase, parser: Any) -> CaseResult:
    """Run a single verification case through the full pipeline.

    Pipeline:
        1. Check skip conditions (``json_error``, empty ``json_string``)
        2. ``json.loads`` the gNMI JSON string
        3. ``parser.parse_and_convert(full_command, json_data)``
        4. ``compare_outputs`` (7-step engine)

    Args:
        tc: The verification case to execute.
        parser: A ``CommandParser`` instance (or any object with
                ``parse_and_convert(command, json_data)``).

    Returns:
        A fully populated :class:`CaseResult`.
    """
    if tc.should_skip:
        return CaseResult(
            test_case=tc,
            status=CaseStatus.SKIP,
            message=tc.device.json_error or "no JSON data",
        )

    try:
        json_data = json.loads(tc.device.json_string)
    except json.JSONDecodeError as exc:
        return CaseResult(
            test_case=tc,
            status=CaseStatus.SKIP,
            message=f"JSON parse error: {exc}",
        )

    try:
        actual = parser.parse_and_convert(tc.full_command, json_data)
    except Exception as exc:
        return CaseResult(
            test_case=tc,
            status=CaseStatus.ERROR,
            message=f"Converter error: {exc}",
        )

    expected = tc.device.cli_string
    passed, diff_info, method = compare_outputs(
        actual, expected, json_data, tc.full_command
    )

    return CaseResult(
        test_case=tc,
        status=CaseStatus.PASS if passed else CaseStatus.FAIL,
        match_type=method,
        message=diff_info or "",
        expected=expected,
        actual=actual,
    )
