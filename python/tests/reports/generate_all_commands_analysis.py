#!/usr/bin/env python3
"""
Generate ALL_COMMANDS_ANALYSIS.md — comprehensive analysis report.

Produces a detailed report matching the original template format with:
- Table of Contents with anchor links
- Overall Summary table
- Per-Device Summary with pass rates
- Quick Reference by Command (clickable anchors)
- Detailed per-command analysis:
  - Per-device results table
  - Per-variant subsections with device status tables
  - Sample Python Invocation code blocks
  - Collapsible <details> per device with Input JSON / Expected / Actual

Usage:
    cd python/
    python -m tests.reports.generate_analysis
"""

from __future__ import annotations

import json
import sys
from collections import OrderedDict, defaultdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple

# Ensure project root is importable
_PROJECT_ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(_PROJECT_ROOT))

from gnmi_cli_lib.parser import CommandParser
from tests.core.data_loader import load_all_fixtures
from tests.core.runner import run_case
from tests.core.test_case import CaseStatus, CommandSuite, VerificationCase

REPORT_PATH = Path(__file__).resolve().parent / "ALL_COMMANDS_ANALYSIS.md"

DEVICES = [
    "str2-8102-01",
    "str4-7060x6-512-1",
    "str4-7060x6-512-13",
    "str4-7060x6-512-9",
]

MAX_OUTPUT_LINES = 20  # truncate long blocks in report


# ── helpers ──────────────────────────────────────────────────────────────────

def _truncate(text: str, max_lines: int = MAX_OUTPUT_LINES) -> str:
    """Truncate text and append '... (N more lines)' if too long."""
    if not text:
        return ""
    lines = text.split("\n")
    if len(lines) <= max_lines:
        return text
    shown = "\n".join(lines[:max_lines])
    remaining = len(lines) - max_lines
    return f"{shown}\n... ({remaining} more lines)"


def _make_anchor(idx: int, cmd: str) -> str:
    """Create a Markdown anchor id from command index and name."""
    slug = cmd.replace(" ", "-").replace("_", "-").lower()
    return f"{idx:02d}-{slug}"


_STATUS_ICON = {
    "PASS": "✅ PASS",
    "FAIL": "❌ FAIL",
    "ERROR": "⚠️ ERROR",
    "SKIP": "⏭️ SKIP",
}

_STATUS_ICON_SHORT = {
    "PASS": "✅",
    "FAIL": "❌",
    "ERROR": "⚠️",
    "SKIP": "⏭️",
}


def _run_case(
    tc: VerificationCase, parser: CommandParser
) -> Tuple[CaseStatus, str, str]:
    """Run one test case.  Returns (status, match_method, actual_output)."""
    result = run_case(tc, parser)
    method = result.match_type or result.message or "skipped"
    actual = result.actual or ""
    return result.status, method, actual


# ── main generator ───────────────────────────────────────────────────────────

def generate() -> str:  # noqa: C901 — intentionally monolithic for single-file report gen
    parser = CommandParser()
    suites = load_all_fixtures()
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    # ── run all tests, collect results per suite ──
    # suite_results[suite_index] = [(tc, status, method, actual), ...]
    suite_results: Dict[int, list] = {}
    for si, suite in enumerate(suites):
        results: list = []
        for tc in suite.test_cases:
            status, method, actual = _run_case(tc, parser)
            results.append((tc, status, method, actual))
        suite_results[si] = results

    # ── global aggregates ──
    total = sum(len(r) for r in suite_results.values())
    g_pass = g_fail = g_error = g_skip = 0
    dev_stats: Dict[str, Dict[str, int]] = {d: defaultdict(int) for d in DEVICES}

    for results in suite_results.values():
        for tc, status, _m, _a in results:
            if status == CaseStatus.PASS:
                g_pass += 1
            elif status == CaseStatus.FAIL:
                g_fail += 1
            elif status == CaseStatus.ERROR:
                g_error += 1
            elif status == CaseStatus.SKIP:
                g_skip += 1
            dname = tc.device.device_name
            if dname in dev_stats:
                dev_stats[dname][status.value] += 1

    run_total = g_pass + g_fail + g_error
    pass_rate = (g_pass / run_total * 100) if run_total else 0

    # ── build markdown ──────────────────────────────────────────────────
    L: List[str] = []
    w = L.append

    # ── Header ──
    w("# ALL COMMANDS ANALYSIS")
    w("")
    w(f"**Generated:** {now}")
    w(f"**Total Commands:** {len(suites)}")
    w(f"**Devices Tested:** {len(DEVICES)}")
    w("")

    # ── Devices list ──
    w("### Devices")
    w("")
    for i, d in enumerate(DEVICES, 1):
        w(f"{i}. `{d}`")
    w("")

    # ── Table of Contents ──
    w("## Table of Contents")
    w("")
    w("- [Overall Summary](#overall-summary)")
    w("- [Per-Device Summary](#per-device-summary)")
    w("- [Quick Reference by Command](#quick-reference-by-command)")
    w("- [Detailed Analysis](#detailed-analysis)")

    # TOC entries for each command
    for si, suite in enumerate(suites):
        anchor = _make_anchor(suite.command_index, suite.command)
        w(f"  - [{suite.command_index:02d}. {suite.command}](#{anchor})")
    w("")

    # ── Overall Summary ──
    w("## Overall Summary")
    w("")
    w("| Metric | Count |")
    w("|--------|-------|")
    w(f"| Total Test Cases (with data) | {total} |")
    w(f"| ✅ PASS | {g_pass} |")
    w(f"| ❌ FAIL | {g_fail} |")
    w(f"| ⚠️ ERROR | {g_error} |")
    w(f"| ⏭️ SKIP (data error) | {g_skip} |")
    w(f"| **Pass Rate** | **{pass_rate:.1f}%** |")
    w("")

    # ── Per-Device Summary ──
    w("## Per-Device Summary")
    w("")
    w("| Device | Total | ✅ Pass | ❌ Fail | ⚠️ Error | ⏭️ Skip | Pass Rate |")
    w("|--------|-------|---------|---------|----------|---------|-----------|")
    for d in DEVICES:
        ds = dev_stats[d]
        dt = sum(ds.values())
        dp = ds.get("PASS", 0)
        df = ds.get("FAIL", 0)
        de = ds.get("ERROR", 0)
        dk = ds.get("SKIP", 0)
        dr = dp + df + de
        drate = (dp / dr * 100) if dr else 0
        w(f"| `{d}` | {dt} | {dp} | {df} | {de} | {dk} | {drate:.1f}% |")
    w("")

    # ── Quick Reference ──
    w("## Quick Reference by Command")
    w("")
    w("| # | Command | Variants | Pass | Fail | Error | Skip |")
    w("|---|---------|----------|------|------|-------|------|")

    for si, suite in enumerate(suites):
        results = suite_results[si]
        variants = set()
        sp = sf = se = sk = 0
        for tc, status, _m, _a in results:
            variants.add(tc.input_options)
            if status == CaseStatus.PASS:
                sp += 1
            elif status == CaseStatus.FAIL:
                sf += 1
            elif status == CaseStatus.ERROR:
                se += 1
            elif status == CaseStatus.SKIP:
                sk += 1
        anchor = _make_anchor(suite.command_index, suite.command)
        idx_str = f"{suite.command_index:02d}"
        w(f"| {idx_str} | [`{suite.command}`](#{anchor}) | {len(variants)} | {sp} | {sf} | {se} | {sk} |")

    w("")
    w("---")
    w("")

    # ── Detailed Analysis ──
    w("## Detailed Analysis")
    w("")

    for si, suite in enumerate(suites):
        results = suite_results[si]
        idx = suite.command_index
        cmd = suite.command
        anchor = _make_anchor(idx, cmd)

        w(f'<a id="{anchor}"></a>')
        w(f"### {idx:02d}. `{cmd}`")
        w("")
        w("[↑ Back to Quick Reference](#quick-reference-by-command)")
        w("")

        # aggregate for this command
        sp = sum(1 for _, s, _, _ in results if s == CaseStatus.PASS)
        sf = sum(1 for _, s, _, _ in results if s == CaseStatus.FAIL)
        se = sum(1 for _, s, _, _ in results if s == CaseStatus.ERROR)
        sk = sum(1 for _, s, _, _ in results if s == CaseStatus.SKIP)
        total_cmd = sp + sf + se + sk
        run_cmd = sp + sf + se

        if sf + se == 0:
            w(f"**Status:** ✅ All {run_cmd}/{total_cmd} test cases passed (across {len(DEVICES)} devices)")
        else:
            w(f"**Status:** ❌ {sf + se} failures out of {total_cmd} test cases")
        w("")

        # Per-device results table
        cmd_dev: Dict[str, Dict[str, int]] = {d: defaultdict(int) for d in DEVICES}
        for tc, status, _m, _a in results:
            dname = tc.device.device_name
            if dname in cmd_dev:
                cmd_dev[dname][status.value] += 1

        w("**Per-Device Results:**")
        w("")
        w("| Device | Pass | Fail | Error | Skip |")
        w("|--------|------|------|-------|------|")
        for d in DEVICES:
            cd = cmd_dev[d]
            if sum(cd.values()) > 0:
                w(f"| `{d}` | {cd.get('PASS',0)} | {cd.get('FAIL',0)} | {cd.get('ERROR',0)} | {cd.get('SKIP',0)} |")
        w("")

        # Group results by variant (input_options)
        variants: OrderedDict[str, list] = OrderedDict()
        for tc, status, method, actual in results:
            key = tc.input_options
            if key not in variants:
                variants[key] = []
            variants[key].append((tc, status, method, actual))

        for vi, (opts, var_results) in enumerate(variants.items(), 1):
            variant_label = f"{cmd} {opts}".strip() if opts else cmd
            variant_type = f"({opts})" if opts else "(base command)"

            w(f"#### {idx:02d}.{vi} `{variant_label}` {variant_type}")
            w("")

            # Per-device status table for this variant
            w("| Device | Status | Details |")
            w("|--------|--------|---------|")
            for tc, status, method, actual in var_results:
                icon = _STATUS_ICON.get(status.value, status.value)
                w(f"| `{tc.device.device_name}` | {icon} | {method} |")
            w("")

            # Sample Python invocation
            w("**Sample Python Invocation:**")
            w("```python")
            w("from gnmi_cli_lib.parser import CommandParser")
            w("")
            w(f'cli_command = "{variant_label}"')
            w("json_data = <load_json_from_gnmi_response>  # See device-specific JSON below")
            w("")
            w("parser = CommandParser()")
            w("result = parser.parse_and_convert(cli_command, json_data)")
            w("print(result)")
            w("```")
            w("")

            # Per-device collapsible details
            for tc, status, method, actual in var_results:
                icon = _STATUS_ICON_SHORT.get(status.value, "")
                w("<details>")
                w(f"<summary><b>{icon} Device: {tc.device.device_name}</b> (click to expand)</summary>")
                w("")

                # Input JSON
                w("**Input JSON:**")
                w("```json")
                try:
                    parsed = json.loads(tc.device.json_string) if tc.device.json_string else {}
                    formatted = json.dumps(parsed, indent=2)
                    w(_truncate(formatted))
                except Exception:
                    w(tc.device.json_string or "(no data)")
                w("```")
                w("")

                # Expected CLI Output
                w("**Expected CLI Output:**")
                w("```")
                w(_truncate(tc.device.cli_string or "(no data)"))
                w("```")
                w("")

                # Actual Converter Output
                w("**Actual Converter Output:**")
                w("```")
                if status == CaseStatus.SKIP:
                    w(f"(skipped: {method})")
                else:
                    w(_truncate(actual or "(empty output)"))
                w("```")
                w("")
                w("</details>")
                w("")

        w("---")
        w("")

    return "\n".join(L)


def main():
    print("Running all integration tests and generating detailed report...")
    report = generate()
    REPORT_PATH.write_text(report, encoding="utf-8")
    line_count = report.count("\n")
    size_kb = len(report.encode("utf-8")) / 1024
    print(f"✅ Report written to {REPORT_PATH}")
    print(f"   Size: {size_kb:,.1f} KB, {line_count:,} lines")


if __name__ == "__main__":
    main()
