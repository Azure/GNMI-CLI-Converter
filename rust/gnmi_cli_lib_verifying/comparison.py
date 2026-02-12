"""
Shared comparison and normalization logic for output verification.

Used by both generate_all_commands_analysis.py and run_all_tests.py.
Any fix or enhancement to comparison logic should be made here once.
"""
import json
import re


# Values that are intentionally filtered/skipped by the formatter.
# Only truly non-operational values belong here.  Operationally critical values
# (session states, admin states, boolean flags) belong in
# CRITICAL_VALUE_TRANSFORMATIONS so their presence is actually verified.
INTENTIONALLY_FILTERED_VALUES = {
    'Unknown',       # Unknown hostname – not useful to display
}

# Operationally critical values that were previously blanket-skipped.
# Each maps to known alternative representations the formatter may use.
# The coverage check verifies the literal value OR one of its alternatives
# appears in the output.  If neither is found the value is counted as MISSING.
CRITICAL_VALUE_TRANSFORMATIONS = {
    # BGP session state — CLI may show prefix count instead of the word.
    # "State/PfxRcd" column header = Established sessions show prefix counts.
    'Established': ['established', 'ESTABLISHED', 'Estab', 'State/PfxRcd', 'PfxRcd'],
    # Boolean values — formatters often transform to yes/no, active/inactive, etc.
    # BGP routes use '*' for valid and '>' for best (status flags in route table).
    'True':  ['true',  'Yes', 'yes', 'YES', 'active',  'Active',  'selected',  'Selected', '*', '>'],
    'true':  ['True',  'Yes', 'yes', 'YES', 'active',  'Active',  'selected',  'Selected', '*', '>'],
    'False': ['false', 'No',  'no',  'NO',  'inactive', 'Inactive', 'deselected', 'Deselected'],
    'false': ['False', 'No',  'no',  'NO',  'inactive', 'Inactive', 'deselected', 'Deselected'],
    # Status values
    'enabled':  ['Enabled',  'ENABLED',  'Yes', 'yes', 'true',  'True',  'active',  'Active'],
    'disabled': ['Disabled', 'DISABLED', 'No',  'no',  'false', 'False', 'inactive', 'Inactive'],
    'up':   ['Up', 'UP'],
    'down': ['Down', 'DOWN', 'Dw'],
    # none/None — low-value but still verified
    'none': ['None', 'NONE'],
    'None': ['none', 'NONE'],
}


# ============================================================================
# Output normalization functions
# ============================================================================

def normalize_output(output: str) -> str:
    """Normalize output for comparison.

    Preserves leading whitespace (important for table alignment like '      IFACE')
    but removes trailing whitespace from each line and trailing newlines from the end.
    """
    if not output:
        return ""
    output = output.rstrip()
    lines = output.split('\n')
    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()
    return '\n'.join(line.rstrip() for line in lines)


def normalize_help_options(line: str) -> str:
    """Normalize help option ordering for comparison.

    Different systems may show '-h, -?, --help' or '-?, -h, --help'.
    Normalize to a consistent order for comparison.
    """
    if re.search(r'-[h?],\s*-[h?],\s*--help', line):
        line = re.sub(r'-h,\s*-\?,\s*--help', '-?, -h, --help', line)
    return line


def normalize_line_spacing(line: str) -> str:
    """Normalize a line by collapsing multiple spaces to single space."""
    return re.sub(r'\s+', ' ', line.strip())


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


# ============================================================================
# Line-level comparison functions
# ============================================================================

def compare_with_normalized_spacing(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing spacing (for table alignment differences)."""
    if len(actual_lines) != len(expected_lines):
        return False
    for actual, expected in zip(actual_lines, expected_lines):
        if normalize_line_spacing(actual) != normalize_line_spacing(expected):
            return False
    return True


def compare_with_normalized_timestamps(actual_lines: list, expected_lines: list) -> bool:
    """Compare lines after normalizing timestamp/uptime values.

    Returns True if lines match after replacing all uptime-like values
    with a placeholder. This handles the case where CLI and gNMI data
    were captured at different times.
    """
    if len(actual_lines) != len(expected_lines):
        return False
    for actual, expected in zip(actual_lines, expected_lines):
        actual_norm = normalize_timestamp_values(actual)
        expected_norm = normalize_timestamp_values(expected)
        if actual_norm != expected_norm:
            return False
    return True


# ============================================================================
# Value extraction and matching functions (for gNMI coverage check)
# ============================================================================

def extract_json_values(data, values=None, keys=None) -> tuple:
    """Recursively extract all leaf values and keys from JSON data.

    Also handles nested JSON strings (e.g., {"Ethernet0": "{\"key\": \"value\"}"})
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
        if str_val.startswith('{') or str_val.startswith('['):
            if str_val.startswith('[') and str_val.endswith(']') and not str_val.startswith('[['):
                inner = str_val[1:-1]
                if inner and not inner.startswith('"') and '":' not in inner:
                    items = [item.strip() for item in inner.split(',') if item.strip()]
                    for item in items:
                        values.add(item)
                    return values, keys
            try:
                parsed = json.loads(str_val)
                extract_json_values(parsed, values, keys)
                return values, keys
            except json.JSONDecodeError:
                pass
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)
    elif data is not None:
        str_val = str(data)
        if str_val and str_val not in ("N/A", "n/a", "NA", "na"):
            values.add(str_val)

    return values, keys


def format_number_with_comma(val: str) -> str:
    """Convert a numeric string to comma-formatted version (e.g., '13312' -> '13,312')."""
    try:
        num = int(val)
        return f"{num:,}"
    except ValueError:
        return None


def normalize_numeric_value(val: str) -> list:
    """Get all possible representations of a numeric value for matching."""
    representations = [val]
    try:
        num = float(val)
        if num == int(num):
            representations.append(str(int(num)))
        representations.append(f"{num:.0f}")
        representations.append(f"{num:.1f}")
        representations.append(f"{num:.2f}")
        representations.append(f"{num:.3f}")
        representations.append(f"{num:.4f}")
        representations.append(f"{num:.5f}")
        representations.append(f"{num:.6f}")
        representations.append(f"{num:g}")

        if '.' in val:
            for i in range(3, min(10, len(val))):
                representations.append(val[:i])

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

                days = sec_val // 86400
                remaining_secs = sec_val % 86400
                hrs = remaining_secs // 3600
                remaining_secs = remaining_secs % 3600
                mins = remaining_secs // 60
                if days > 0:
                    representations.append(f"{days}d{hrs:02d}h{mins:02d}m")
                    representations.append(f"{days}d{hrs}h{mins}m")
                else:
                    representations.append(f"{hrs:02d}h{mins:02d}m")
                    representations.append(f"{hrs}h{mins}m")
    except ValueError:
        pass
    return representations


def normalize_string_value(val: str) -> list:
    """Get all possible representations of a string value for matching."""
    representations = [val]
    representations.append(val.lower())
    representations.append(val.upper())

    transformations = {
        'advertisedAndReceived': 'advertised and received',
        'advertised': 'advertised',
        'received': 'received',
        'extendedAndStandard': 'extended and standard',
        'sharedNetwork': 'shared network',
    }
    if val in transformations:
        representations.append(transformations[val])

    if val.lower() == 'none':
        representations.append('none')
        representations.append('None')

    return representations


def check_multiline_string_coverage(val: str, output: str) -> bool:
    """Check if a multi-line string is covered by checking each line individually."""
    if '\n' not in val:
        return False
    lines = val.split('\n')
    for line in lines:
        line = line.strip()
        if line and line not in output:
            return False
    return True


def check_truncated_value_coverage(val: str, output: str, min_prefix_len: int = 20) -> bool:
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


def check_all_entries_empty(json_data: dict) -> bool:
    """Check if all entries in JSON data are empty dicts."""
    if not isinstance(json_data, dict):
        return False
    if not json_data:
        return True
    for key, value in json_data.items():
        if isinstance(value, dict):
            if value:
                return False
        elif isinstance(value, list):
            if value:
                return False
        elif value is not None and value != "":
            return False
    return True


def check_gnmi_coverage(actual_output: str, json_data: dict, cli_command: str = "") -> tuple:
    """Check if the actual converter output covers all data from gNMI JSON.

    Returns (is_pass, coverage_info)
    """
    if '--nonzero' in cli_command and check_all_entries_empty(json_data):
        if not actual_output or not actual_output.strip():
            return True, "All JSON entries empty with --nonzero filter, no output expected (PASS)"
        else:
            return True, "All JSON entries empty with --nonzero filter (PASS)"

    if not actual_output:
        return False, "No converter output"

    if not json_data:
        return True, "Empty JSON data with converter output (PASS)"

    json_values, _ = extract_json_values(json_data)

    if not json_values:
        return True, "No values in JSON data, converter produced output (PASS)"

    covered = set()
    missing = set()

    for val in json_values:
        found = False

        if val in INTENTIONALLY_FILTERED_VALUES:
            found = True
        elif val in actual_output:
            found = True
        else:
            formatted = format_number_with_comma(val)
            if formatted and formatted in actual_output:
                found = True
            else:
                for alt_val in normalize_numeric_value(val):
                    if alt_val in actual_output:
                        found = True
                        break

                if not found:
                    for alt_val in normalize_string_value(val):
                        if alt_val in actual_output:
                            found = True
                            break

                if not found:
                    if check_multiline_string_coverage(val, actual_output):
                        found = True

                if not found:
                    if check_truncated_value_coverage(val, actual_output):
                        found = True

                # Check IPv6 link-local addresses with zone identifier
                # JSON has: fe80::X/64, output may have: fe80::X%Interface/64
                if not found and val.lower().startswith("fe80:") and "/" in val:
                    addr_part, prefix = val.rsplit("/", 1)
                    # Look for the address with any zone ID appended
                    if addr_part + "%" in actual_output:
                        found = True

                # Critical value transformations — check known alternatives
                if not found and val in CRITICAL_VALUE_TRANSFORMATIONS:
                    for alt_val in CRITICAL_VALUE_TRANSFORMATIONS[val]:
                        if alt_val in actual_output:
                            found = True
                            break

        if found:
            covered.add(val)
        else:
            missing.add(val)

    coverage_pct = len(covered) / len(json_values) * 100 if json_values else 0

    if not missing:
        return True, f"All {len(json_values)} values from gNMI JSON covered (100%)"
    else:
        missing_sample = list(missing)[:5]
        return False, f"Coverage: {coverage_pct:.1f}% ({len(covered)}/{len(json_values)}). Missing: {missing_sample}"


def check_bgp_routes_acceptable_diff(actual_lines: list, expected_lines: list, cli_command: str) -> tuple:
    """Check if differences in BGP routes output are acceptable nuances.

    Known acceptable differences:
    1. "Displayed X routes and Y total paths" vs "Displayed X routes"
       - The "total paths" is a global BGP counter from ALL neighbors, not available in GNMI
       - GNMI only returns neighbor-specific data, so we can't derive global total paths

    2. Header line spacing differences (e.g., 5 spaces vs 4 spaces before "Network")
       - These are device-specific or timing-related nuances

    Returns (is_acceptable, reason)
    """
    # Only apply to BGP neighbors routes commands
    if 'bgp neighbors' not in cli_command or 'routes' not in cli_command:
        return False, None

    # Handle line count mismatches from multipath entries in advertised-routes
    # gNMI JSON uses a dict keyed by prefix (single entry), but CLI may show
    # multipath duplicates (*= entries). If actual is a subset, check if all
    # actual lines appear in expected (just fewer multipath duplicates).
    if len(actual_lines) != len(expected_lines):
        if 'advertised-routes' in cli_command and len(actual_lines) < len(expected_lines):
            # Check that all non-empty actual lines appear in expected
            # and the difference is only multipath duplicate entries
            actual_set = set(l.strip() for l in actual_lines if l.strip())
            expected_set = set(l.strip() for l in expected_lines if l.strip())
            # Actual routes should be a subset of expected (expected has extra *=/multipath lines)
            missing_from_expected = actual_set - expected_set
            # Filter out known acceptable differences (spacing, footer wording)
            truly_missing = set()
            for line in missing_from_expected:
                # Accept spacing-only differences
                if any(normalize_line_spacing(line) == normalize_line_spacing(el) for el in expected_set):
                    continue
                # Accept "Displayed X routes" vs "Total number of prefixes X"
                displayed_match = re.match(r'^Displayed\s+(\d+)\s+routes', line)
                total_match = any(re.match(r'^Total number of prefixes\s+' + re.escape(displayed_match.group(1)) if displayed_match else r'$', el) for el in expected_set)
                if displayed_match and total_match:
                    continue
                truly_missing.add(line)
            if not truly_missing:
                return True, f"BGP routes acceptable nuances: multipath entries in CLI not in gNMI ({len(expected_lines) - len(actual_lines)} extra lines)"
        return False, None

    differences_found = []

    for i, (actual, expected) in enumerate(zip(actual_lines, expected_lines)):
        if actual == expected:
            continue

        # Check for "Displayed X routes" vs "Total number of prefixes X (Y filtered)"
        # received-routes and advertised-routes use "Total number of prefixes" in CLI
        # but converter outputs "Displayed X routes". Both are acceptable since the
        # route counts match and "total paths" / "filtered" info isn't in gNMI.
        displayed_pattern = r'^Displayed\s+(\d+)\s+routes'
        total_prefix_pattern = r'^Total number of prefixes\s+(\d+)'
        actual_displayed = re.match(displayed_pattern, actual.strip())
        expected_displayed = re.match(displayed_pattern, expected.strip())
        actual_total = re.match(total_prefix_pattern, actual.strip())
        expected_total = re.match(total_prefix_pattern, expected.strip())

        if actual_displayed and expected_displayed:
            # Both have "Displayed X routes" - check if only difference is "total paths"
            if actual_displayed.group(1) == expected_displayed.group(1):
                differences_found.append(f"Line {i+1}: total paths not in GNMI (acceptable)")
                continue

        if actual_displayed and expected_total:
            # "Displayed X routes" vs "Total number of prefixes X (Y filtered)"
            if actual_displayed.group(1) == expected_total.group(1):
                differences_found.append(f"Line {i+1}: footer wording difference, route count matches (acceptable)")
                continue

        # Check for header line spacing differences
        # "    Network" vs "     Network" (spacing before Network column)
        if normalize_line_spacing(actual) == normalize_line_spacing(expected):
            differences_found.append(f"Line {i+1}: spacing difference only (acceptable)")
            continue

        # Check for metric 0 vs empty metric in route lines
        # Some devices show metric=0 in CLI but gNMI JSON has no metric field
        if re.match(r'^\s*[*\s][><=\s]', actual) and re.match(r'^\s*[*\s][><=\s]', expected):
            if normalize_line_spacing(actual.replace('  0  ', '  ')) == normalize_line_spacing(expected.replace('  0  ', '  ')):
                differences_found.append(f"Line {i+1}: metric 0 vs empty (gNMI has no metric, acceptable)")
                continue

        # Other differences are not acceptable
        return False, None

    if differences_found:
        return True, f"BGP routes acceptable nuances: {'; '.join(differences_found)}"

    return False, None


def compare_outputs(actual: str, expected: str, json_data: dict = None, cli_command: str = "") -> tuple:
    """Compare actual and expected outputs with multi-step logic.

    PASS conditions:
    1. Exact match between normalized outputs
    2. Match after normalizing column spacing (for table alignment differences)
    3. Match after normalizing timestamp/uptime values (time drift between CLI and gNMI capture)
    4. Help text with only option ordering differences (e.g., -h, -? vs -?, -h)
    5. All values from gNMI JSON are covered in the actual output
    6. For --nonzero commands: if all JSON entries are empty {}, no output is correct
    7. BGP routes acceptable nuances (total paths not in GNMI)

    Returns (is_pass, diff_info, coverage_info)
    """
    actual_norm = normalize_output(actual)
    expected_norm = normalize_output(expected)

    # First check: exact match
    if actual_norm == expected_norm:
        return True, None, "Exact match"

    actual_lines = actual_norm.split('\n')
    expected_lines = expected_norm.split('\n')

    # Second check: match after normalizing column spacing (for table alignment)
    if compare_with_normalized_spacing(actual_lines, expected_lines):
        return True, None, "Match (spacing differences only)"

    # Third check: match after normalizing timestamp/uptime values
    if compare_with_normalized_timestamps(actual_lines, expected_lines):
        return True, None, "Match (timestamp/uptime differences only - CLI vs gNMI time drift)"

    # Check for help text with only option ordering differences
    if len(actual_lines) == len(expected_lines):
        actual_normalized_help = [normalize_help_options(line) for line in actual_lines]
        expected_normalized_help = [normalize_help_options(line) for line in expected_lines]
        if actual_normalized_help == expected_normalized_help:
            return True, None, "Match after normalizing help option order"

    # Fourth check: gNMI data coverage (use 'is not None' to handle empty dict {})
    if json_data is not None:
        is_covered, coverage_info = check_gnmi_coverage(actual, json_data, cli_command)
        if is_covered:
            return True, None, coverage_info

    # Fifth check: BGP routes acceptable nuances
    is_acceptable, reason = check_bgp_routes_acceptable_diff(actual_lines, expected_lines, cli_command)
    if is_acceptable:
        return True, None, reason

    # Failed - provide diff info
    diff_info = []
    diff_info.append(f"Line count: expected={len(expected_lines)}, actual={len(actual_lines)}")

    for i, (exp, act) in enumerate(zip(expected_lines, actual_lines)):
        if exp != act:
            diff_info.append(f"First diff at line {i+1}:")
            diff_info.append(f"  Expected: {exp[:80]}")
            diff_info.append(f"  Actual:   {act[:80]}")
            break

    if len(actual_lines) != len(expected_lines):
        diff_info.append(f"Line count mismatch")

    if json_data is not None:
        _, coverage_info = check_gnmi_coverage(actual, json_data)
        diff_info.append(f"gNMI Coverage: {coverage_info}")

    return False, '\n'.join(diff_info), None
