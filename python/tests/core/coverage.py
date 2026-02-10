"""
gNMI value coverage analysis.

This module checks whether the converter output covers all data values from the
gNMI JSON response.  It is the single source of truth for coverage logic —
previously duplicated in run_all_tests.py and generate_all_commands_analysis.py.

Design notes for Rust portability:
- All helper functions are pure and stateless.
- Configuration data (INTENTIONALLY_FILTERED_VALUES, transformations dict)
  are plain sets / dicts that can be serialized to JSON for sharing with Rust.
"""

from __future__ import annotations

import json as json_module
from typing import Any, Dict, List, Optional, Set, Tuple


# ---------------------------------------------------------------------------
# Configuration — extract to JSON for Rust sharing if needed
# ---------------------------------------------------------------------------

# Values that are intentionally filtered/skipped by the formatter.
# These count as "covered" because the decision to not output them is intentional
# OR because they are transformed to other representations.
INTENTIONALLY_FILTERED_VALUES: Set[str] = {
    "Unknown",       # Unknown hostname – not useful to display
    "none",          # No address families by peer
    "None",
    "Established",   # BGP state – when Established, CLI shows prefix count instead
    # Boolean values — often transformed to flags or status indicators
    "True", "true",
    "False", "false",
    # Status values — often encoded in flags rather than shown literally
    "enabled", "disabled",
    "up", "down",
}

# Common camelCase → spaced-word transformations (BGP capabilities)
CAMEL_CASE_TRANSFORMATIONS: Dict[str, str] = {
    "advertisedAndReceived": "advertised and received",
    "advertised": "advertised",
    "received": "received",
    "extendedAndStandard": "extended and standard",
}


# ---------------------------------------------------------------------------
# JSON value extraction
# ---------------------------------------------------------------------------

def extract_json_values(
    data: Any,
    values: Optional[Set[str]] = None,
    keys: Optional[Set[str]] = None,
) -> Tuple[Set[str], Set[str]]:
    """Recursively extract all leaf values and keys from JSON data.

    Also handles nested JSON strings (e.g., ``{"Ethernet0": "{\\"key\\": \\"value\\"}"}``)
    by parsing them and extracting values from within.
    """
    if values is None:
        values = set()
    if keys is None:
        keys = set()

    if isinstance(data, dict):
        for k, v in data.items():
            keys.add(str(k))
            extract_json_values(v, values, keys)
    elif isinstance(data, list):
        for item in data:
            extract_json_values(item, values, keys)
    elif isinstance(data, str):
        str_val = data.strip()
        if str_val.startswith("{") or str_val.startswith("["):
            # Check bracket-enclosed comma-separated list (NOT valid JSON)
            if (
                str_val.startswith("[")
                and str_val.endswith("]")
                and not str_val.startswith("[[")
            ):
                inner = str_val[1:-1]
                if inner and not inner.startswith('"') and '":' not in inner:
                    items = [item.strip() for item in inner.split(",") if item.strip()]
                    for item in items:
                        values.add(item)
                    return values, keys
            try:
                parsed = json_module.loads(str_val)
                extract_json_values(parsed, values, keys)
                return values, keys
            except json_module.JSONDecodeError:
                pass
        # Regular string value
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)
    elif data is not None:
        str_val = str(data)
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)

    return values, keys


# ---------------------------------------------------------------------------
# Numeric / string normalization for value matching
# ---------------------------------------------------------------------------

def format_number_with_comma(val: str) -> Optional[str]:
    """Convert a numeric string to comma-formatted version (e.g., '13312' -> '13,312')."""
    try:
        return f"{int(val):,}"
    except ValueError:
        return None


def normalize_numeric_value(val: str) -> List[str]:
    """Get all possible representations of a numeric value for matching.

    Handles scientific notation, msec→sec conversions, floating-point precision.
    """
    representations = [val]
    try:
        num = float(val)
        if num == int(num):
            representations.append(str(int(num)))
        for fmt in (".0f", ".1f", ".2f", ".3f", ".4f", ".5f", ".6f", "g"):
            representations.append(format(num, fmt))

        # Truncated decimal representations
        if "." in val:
            for i in range(3, min(10, len(val))):
                representations.append(val[:i])

        # msec → sec conversion (common in BGP timers)
        if num >= 1000 and num == int(num):
            sec_val = int(num) // 1000
            representations.append(str(sec_val))
            if sec_val > 0:
                hours = sec_val // 3600
                minutes = (sec_val % 3600) // 60
                secs = sec_val % 60
                representations.append(f"{hours:02d}:{minutes:02d}:{secs:02d}")
                if hours == 0:
                    representations.append(f"{minutes:02d}:{secs:02d}")
                # Day/hour/minute format
                days = sec_val // 86400
                remaining = sec_val % 86400
                hrs = remaining // 3600
                remaining = remaining % 3600
                mins = remaining // 60
                if days > 0:
                    representations.append(f"{days}d{hrs:02d}h{mins:02d}m")
                    representations.append(f"{days}d{hrs}h{mins}m")
                else:
                    representations.append(f"{hrs:02d}h{mins:02d}m")
                    representations.append(f"{hrs}h{mins}m")
    except ValueError:
        pass
    return representations


def normalize_string_value(val: str) -> List[str]:
    """Get all possible representations of a string value for matching."""
    representations = [val, val.lower(), val.upper()]
    if val in CAMEL_CASE_TRANSFORMATIONS:
        representations.append(CAMEL_CASE_TRANSFORMATIONS[val])
    if val.lower() == "none":
        representations.extend(["none", "None"])
    return representations


def check_multiline_string_coverage(val: str, output: str) -> bool:
    """Check if a multi-line string is covered by checking each line individually."""
    if "\n" not in val:
        return False
    return all(
        (not line.strip()) or (line.strip() in output)
        for line in val.split("\n")
    )


def check_truncated_value_coverage(
    val: str, output: str, min_prefix_len: int = 20
) -> bool:
    """Check if a value is covered via truncation (value appears as prefix + '...')."""
    if len(val) < min_prefix_len:
        return False
    for prefix_len in range(len(val) - 1, min_prefix_len - 1, -1):
        prefix = val[:prefix_len]
        if prefix + "..." in output:
            return True
        if prefix in output and len(prefix) > min_prefix_len + 10:
            return True
    return False


def check_ipv6_link_local_coverage(val: str, output: str) -> bool:
    """Check IPv6 link-local addresses with zone identifier.

    JSON has: ``fe80::X/64``, output may have: ``fe80::X%Interface/64``.
    """
    if not (val.lower().startswith("fe80:") and "/" in val):
        return False
    addr_part, _ = val.rsplit("/", 1)
    return (addr_part + "%") in output


def check_all_entries_empty(json_data: dict) -> bool:
    """Check if all entries in JSON data are empty dicts."""
    if not isinstance(json_data, dict):
        return False
    if not json_data:
        return True
    for value in json_data.values():
        if isinstance(value, dict) and value:
            return False
        if isinstance(value, list) and value:
            return False
        if not isinstance(value, (dict, list)) and value is not None and value != "":
            return False
    return True


# ---------------------------------------------------------------------------
# Main coverage check
# ---------------------------------------------------------------------------

def check_gnmi_coverage(
    actual_output: str,
    json_data: dict,
    cli_command: str = "",
) -> Tuple[bool, str]:
    """Check if converter output covers all data from gNMI JSON.

    Returns ``(is_pass, coverage_info)``.
    """
    # Special: --nonzero with all-empty JSON → no output is correct
    if "--nonzero" in cli_command and check_all_entries_empty(json_data):
        if not actual_output or not actual_output.strip():
            return True, "All JSON entries empty with --nonzero filter, no output expected (PASS)"
        return True, "All JSON entries empty with --nonzero filter (PASS)"

    if not actual_output:
        return False, "No converter output"

    if not json_data:
        return True, "Empty JSON data with converter output (PASS)"

    json_values, _ = extract_json_values(json_data)

    if not json_values:
        return True, "No values in JSON data, converter produced output (PASS)"

    covered: Set[str] = set()
    missing: Set[str] = set()

    for val in json_values:
        if _is_value_covered(val, actual_output):
            covered.add(val)
        else:
            missing.add(val)

    coverage_pct = len(covered) / len(json_values) * 100 if json_values else 0

    if not missing:
        return True, f"All {len(json_values)} values from gNMI JSON covered (100%)"

    missing_sample = list(missing)[:5]
    return (
        False,
        f"Coverage: {coverage_pct:.1f}% ({len(covered)}/{len(json_values)}). "
        f"Missing: {missing_sample}",
    )


def _is_value_covered(val: str, output: str) -> bool:
    """Return True if a single JSON value is covered in the output."""
    # Intentionally filtered values
    if val in INTENTIONALLY_FILTERED_VALUES:
        return True
    # Exact match
    if val in output:
        return True
    # Comma-formatted number
    formatted = format_number_with_comma(val)
    if formatted and formatted in output:
        return True
    # Numeric representations
    for alt in normalize_numeric_value(val):
        if alt in output:
            return True
    # String representations
    for alt in normalize_string_value(val):
        if alt in output:
            return True
    # Multi-line string
    if check_multiline_string_coverage(val, output):
        return True
    # Truncated value
    if check_truncated_value_coverage(val, output):
        return True
    # IPv6 link-local
    if check_ipv6_link_local_coverage(val, output):
        return True
    return False
