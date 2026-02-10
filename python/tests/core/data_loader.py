"""
Unified data loader for test fixtures.

Loads JSON fixture files from ``tests/fixtures/commands/`` and converts
them into ``TestCase`` / ``CommandTestSuite`` data model instances used by both
the parameterized integration tests and the report generator.

The loader is the **single** place that knows how to parse the fixture JSON
schema — all consumers get structured ``TestCase`` objects.
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from tests.core.test_case import (
    CommandSuite,
    DeviceData,
    VerificationCase,
)

# Default location of fixture data
_DEFAULT_TEST_DATA_DIR = (
    Path(__file__).resolve().parent.parent
    / "fixtures"
    / "commands"
)


def _extract_index_from_filename(filename: str) -> int:
    """Extract the numeric index from a fixture filename like ``data_13_show_xxx.json``."""
    m = re.match(r"data_(\d+)_", filename)
    return int(m.group(1)) if m else 0


def load_fixture_file(
    filepath: Path,
) -> CommandSuite:
    """Load a single JSON fixture file and return a ``CommandSuite``.

    Args:
        filepath: Path to a ``data_NN_show_xxx.json`` file.

    Returns:
        A ``CommandSuite`` containing all ``VerificationCase`` instances.
    """
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)

    command = data["command"]
    command_index = _extract_index_from_filename(filepath.name)
    test_cases: List[VerificationCase] = []

    for tc_block in data.get("test_cases", []):
        input_options = tc_block.get("input_options", "")
        devices = tc_block.get("devices", {})

        for device_name, device_data in devices.items():
            device = DeviceData.from_dict(device_name, device_data)
            test_cases.append(
                VerificationCase(
                    command=command,
                    input_options=input_options,
                    device=device,
                    command_index=command_index,
                )
            )

    return CommandSuite(
        command=command,
        command_index=command_index,
        data_filename=filepath.name,
        test_cases=test_cases,
    )


def load_all_fixtures(
    test_data_dir: Optional[Path] = None,
) -> List[CommandSuite]:
    """Discover and load all fixture files.

    Args:
        test_data_dir: Directory containing ``data_*.json`` files.
                       Defaults to ``tests/fixtures/commands/``.

    Returns:
        Sorted list of ``CommandSuite`` (by command_index).
    """
    data_dir = test_data_dir or _DEFAULT_TEST_DATA_DIR
    if not data_dir.exists():
        raise FileNotFoundError(f"Test data directory not found: {data_dir}")

    suites: List[CommandSuite] = []
    for filepath in sorted(data_dir.glob("data_*.json")):
        suites.append(load_fixture_file(filepath))

    return suites


def collect_all_test_cases(
    test_data_dir: Optional[Path] = None,
) -> List[VerificationCase]:
    """Flatten all fixtures into a single list of ``VerificationCase`` instances.

    This is the primary entry point for pytest parameterization:

    .. code-block:: python

        @pytest.mark.parametrize("test_case", collect_all_test_cases(), ids=lambda tc: tc.test_id)
        def test_command(test_case):
            ...

    Args:
        test_data_dir: Optional override for the fixture directory.

    Returns:
        Flat list of all ``VerificationCase`` instances across all commands.
    """
    cases: List[VerificationCase] = []
    for suite in load_all_fixtures(test_data_dir):
        cases.extend(suite.test_cases)
    return cases


def collect_runnable_test_cases(
    test_data_dir: Optional[Path] = None,
) -> Tuple[List[VerificationCase], List[VerificationCase]]:
    """Split test cases into runnable and skippable sets.

    Returns:
        ``(runnable, skippable)`` tuple.
    """
    all_cases = collect_all_test_cases(test_data_dir)
    runnable = [tc for tc in all_cases if not tc.should_skip]
    skippable = [tc for tc in all_cases if tc.should_skip]
    return runnable, skippable
