"""
Output normalization utilities for test comparison.

This module provides all normalization functions used by the comparison engine.
It is the single source of truth for output normalization — previously this
logic was duplicated in run_all_tests.py, generate_all_commands_analysis.py,
and each individual test_*.py file.

These functions are intentionally kept pure (no side effects, no state) so they
can be easily ported to Rust or any other language.
"""

import re


def normalize_output(output: str) -> str:
    """Normalize output for comparison.

    Preserves leading whitespace (important for table alignment like '      IFACE')
    but removes trailing whitespace from each line and trailing newlines from the end.
    """
    if not output:
        return ""
    output = output.rstrip()
    lines = output.split('\n')
    # Remove leading empty lines
    while lines and not lines[0].strip():
        lines.pop(0)
    # Remove trailing empty lines
    while lines and not lines[-1].strip():
        lines.pop()
    # Strip only trailing whitespace from each line, preserve leading spaces
    return '\n'.join(line.rstrip() for line in lines)


def normalize_line_spacing(line: str) -> str:
    """Normalize a line by collapsing multiple spaces to single space.

    This allows comparison of table rows where column spacing may vary
    but the actual column values are the same.
    """
    return re.sub(r'\s+', ' ', line.strip())


def normalize_help_options(line: str) -> str:
    """Normalize help option ordering for comparison.

    Different systems may show '-h, -?, --help' or '-?, -h, --help'.
    Normalize to a consistent order for comparison.
    """
    if re.search(r'-[h?],\s*-[h?],\s*--help', line):
        line = re.sub(r'-h,\s*-\?,\s*--help', '-?, -h, --help', line)
    return line


def normalize_timestamp_values(line: str) -> str:
    """Normalize timestamp/uptime values in a line for comparison.

    Handles formats like:
    - '1d08h57m' -> '<UPTIME>'
    - '04:43:27' -> '<UPTIME>'
    - 'Last update 1d08h57m ago' -> 'Last update <UPTIME> ago'
    - '1d09h00m' -> '<UPTIME>'

    This allows comparison when CLI and gNMI were captured at different times.
    """
    # Pattern for day/hour/minute format: 1d08h57m, 2d12h30m, etc.
    line = re.sub(r'\b\d+d\d{2}h\d{2}m\b', '<UPTIME>', line)
    # Pattern for HH:MM:SS format: 04:43:27, 12:30:00, etc.
    line = re.sub(r'\b\d{1,2}:\d{2}:\d{2}\b', '<UPTIME>', line)
    # Pattern for uptime in seconds: "up for 12345"
    line = re.sub(r'\bup for \d+\b', 'up for <UPTIME>', line)
    return line


# ---------------------------------------------------------------------------
# Line-level comparison helpers
# ---------------------------------------------------------------------------

def compare_with_normalized_spacing(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing spacing (for table alignment differences)."""
    if len(actual_lines) != len(expected_lines):
        return False
    return all(
        normalize_line_spacing(a) == normalize_line_spacing(e)
        for a, e in zip(actual_lines, expected_lines)
    )


def compare_with_normalized_timestamps(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing timestamp/uptime values.

    Returns True if lines match after replacing all uptime-like values
    with a placeholder.
    """
    if len(actual_lines) != len(expected_lines):
        return False
    return all(
        normalize_timestamp_values(a) == normalize_timestamp_values(e)
        for a, e in zip(actual_lines, expected_lines)
    )


def compare_with_normalized_help(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing help option ordering."""
    if len(actual_lines) != len(expected_lines):
        return False
    return (
        [normalize_help_options(l) for l in actual_lines]
        == [normalize_help_options(l) for l in expected_lines]
    )
