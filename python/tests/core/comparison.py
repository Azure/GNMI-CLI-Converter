"""
Multi-step comparison engine — single source of truth.

This module is the **only** place where comparison logic lives. Previously it was
duplicated across ``run_all_tests.py``, ``generate_all_commands_analysis.py``,
and every individual ``test_*.py`` file.

The engine uses a 7-step cascade:

1. Exact match (after output normalization)
2. Spacing-normalized match (table column alignment differences)
3. Timestamp-normalized match (CLI vs gNMI captured at different times)
4. Help option order normalized (``-h, -?`` vs ``-?, -h``)
5. gNMI value coverage (all JSON leaf values present in output)
6. Empty JSON + ``--nonzero`` filter (no output is correct)
7. BGP routes acceptable nuances (``total paths`` not in gNMI)

Design notes for Rust portability:
- Each comparison step is a standalone function returning a simple tuple.
- The cascade order and step list are data-driven and can be serialized.
- No class hierarchy required — plain functions composed in ``compare_outputs``.
"""

from __future__ import annotations

import re
from typing import Any, Dict, List, Optional, Tuple

from tests.core.normalization import (
    normalize_output,
    normalize_line_spacing,
    compare_with_normalized_spacing,
    compare_with_normalized_timestamps,
    compare_with_normalized_help,
)
from tests.core.coverage import check_gnmi_coverage


# ---------------------------------------------------------------------------
# BGP-specific acceptable differences
# ---------------------------------------------------------------------------

def check_bgp_routes_acceptable_diff(
    actual_lines: List[str],
    expected_lines: List[str],
    cli_command: str,
) -> Tuple[bool, Optional[str]]:
    """Check if differences in BGP routes output are acceptable nuances.

    Known acceptable differences:
    1. "Displayed X routes and Y total paths" vs "Displayed X routes"
       — ``total paths`` is a global BGP counter not available in gNMI.
    2. Header line spacing differences.
    """
    if "bgp neighbors" not in cli_command or "routes" not in cli_command:
        return False, None
    if len(actual_lines) != len(expected_lines):
        return False, None

    differences: List[str] = []

    for i, (actual, expected) in enumerate(zip(actual_lines, expected_lines)):
        if actual == expected:
            continue

        # "Displayed X routes and Y total paths" vs "Displayed X routes"
        displayed_pattern = r"^Displayed\s+(\d+)\s+routes"
        actual_m = re.match(displayed_pattern, actual.strip())
        expected_m = re.match(displayed_pattern, expected.strip())
        if actual_m and expected_m and actual_m.group(1) == expected_m.group(1):
            differences.append(f"Line {i+1}: total paths not in GNMI (acceptable)")
            continue

        # Header spacing
        if normalize_line_spacing(actual) == normalize_line_spacing(expected):
            differences.append(f"Line {i+1}: spacing difference only (acceptable)")
            continue

        # Non-acceptable difference
        return False, None

    if differences:
        return True, f"BGP routes acceptable nuances: {'; '.join(differences)}"
    return False, None


# ---------------------------------------------------------------------------
# Main comparison entry point
# ---------------------------------------------------------------------------

class ComparisonResult:
    """Result of a comparison between expected and actual output."""

    __slots__ = ("passed", "match_type", "diff_info")

    def __init__(
        self,
        passed: bool,
        match_type: Optional[str] = None,
        diff_info: Optional[str] = None,
    ):
        self.passed = passed
        self.match_type = match_type
        self.diff_info = diff_info

    # Keep the old tuple interface for backward compatibility
    def as_tuple(self) -> Tuple[bool, Optional[str], Optional[str]]:
        return (self.passed, self.diff_info, self.match_type)

    def __repr__(self) -> str:
        status = "PASS" if self.passed else "FAIL"
        detail = self.match_type or self.diff_info or ""
        return f"ComparisonResult({status}, {detail!r})"


def compare_outputs(
    actual: str,
    expected: str,
    json_data: Optional[Dict[str, Any]] = None,
    cli_command: str = "",
) -> Tuple[bool, Optional[str], Optional[str]]:
    """Compare actual and expected outputs with multi-step logic.

    Returns ``(is_pass, diff_info, match_reason)``.

    Steps:
    1. Exact match
    2. Spacing-normalized match
    3. Timestamp-normalized match
    4. Help option order normalized
    5. gNMI value coverage
    6. BGP routes acceptable nuances
    """
    actual_norm = normalize_output(actual)
    expected_norm = normalize_output(expected)

    # Step 1 — exact match
    if actual_norm == expected_norm:
        return True, None, "Exact match"

    actual_lines = actual_norm.split("\n")
    expected_lines = expected_norm.split("\n")

    # Step 2 — spacing-normalized
    if compare_with_normalized_spacing(actual_lines, expected_lines):
        return True, None, "Match (spacing differences only)"

    # Step 3 — timestamp-normalized
    if compare_with_normalized_timestamps(actual_lines, expected_lines):
        return (
            True,
            None,
            "Match (timestamp/uptime differences only - CLI vs gNMI time drift)",
        )

    # Step 4 — help option ordering
    if compare_with_normalized_help(actual_lines, expected_lines):
        return True, None, "Match after normalizing help option order"

    # Step 5 — gNMI value coverage (cache result for FAIL path)
    _coverage_info: Optional[str] = None
    if json_data is not None:
        is_covered, _coverage_info = check_gnmi_coverage(actual, json_data, cli_command)
        if is_covered:
            return True, None, _coverage_info

    # Step 6 — BGP routes acceptable nuances
    is_acceptable, reason = check_bgp_routes_acceptable_diff(
        actual_lines, expected_lines, cli_command,
    )
    if is_acceptable:
        return True, None, reason

    # ----- FAIL — build diff info -----
    diff_parts: List[str] = []
    diff_parts.append(
        f"Line count: expected={len(expected_lines)}, actual={len(actual_lines)}"
    )
    for i, (exp, act) in enumerate(zip(expected_lines, actual_lines)):
        if exp != act:
            diff_parts.append(f"First diff at line {i+1}:")
            diff_parts.append(f"  Expected: {exp[:80]}")
            diff_parts.append(f"  Actual:   {act[:80]}")
            break
    if len(actual_lines) != len(expected_lines):
        diff_parts.append("Line count mismatch")

    if _coverage_info is not None:
        diff_parts.append(f"gNMI Coverage: {_coverage_info}")

    return False, "\n".join(diff_parts), None
