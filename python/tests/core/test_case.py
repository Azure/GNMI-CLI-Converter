"""
Data models for the unified test framework.

These dataclasses represent:
- ``DeviceTestData``  — raw test input for a single device
- ``TestCase``        — a single (command + options + device) test item
- ``TestResult``      — outcome of running a TestCase
- ``CommandTestSuite``— all test data for one command
"""

from __future__ import annotations

import enum
from dataclasses import dataclass, field, asdict
from typing import Any, Dict, List, Optional


class CaseStatus(str, enum.Enum):
    """Outcome of a single test case."""
    PASS = "PASS"
    FAIL = "FAIL"
    SKIP = "SKIP"
    ERROR = "ERROR"
    UNKNOWN = "UNKNOWN"


# Aliases for backward compatibility
TestStatus = CaseStatus


@dataclass
class DeviceData:
    """Raw test input for a single device within a test case variant.

    This maps directly to the JSON structure in
    ``test_data/data_NN_show_xxx.json → test_cases[].devices{}.``
    """
    device_name: str
    cli_file: str = ""
    gnmi_file: str = ""
    json_string: str = ""
    cli_string: str = ""
    json_error: Optional[str] = None
    cli_error: Optional[str] = None

    @classmethod
    def from_dict(cls, device_name: str, d: Dict[str, Any]) -> "DeviceData":
        return cls(
            device_name=device_name,
            cli_file=d.get("cli_file", ""),
            gnmi_file=d.get("gnmi_file", ""),
            json_string=d.get("json_string", ""),
            cli_string=d.get("cli_string", ""),
            json_error=d.get("json_error"),
            cli_error=d.get("cli_error"),
        )


# Backward compatibility alias
DeviceTestData = DeviceData


@dataclass
class VerificationCase:
    """A single atomic test case: command + input_options + device."""
    command: str
    input_options: str
    device: DeviceData
    command_index: int = 0

    @property
    def full_command(self) -> str:
        """Build the full CLI command string."""
        if self.input_options:
            return f"{self.command} {self.input_options}"
        return self.command

    @property
    def test_id(self) -> str:
        """Unique human-readable test ID for pytest parameterization."""
        cmd_slug = self.command.replace(" ", "_")
        opts = self.input_options.replace(" ", "_").replace("-", "") if self.input_options else "base"
        return f"{cmd_slug}-{opts}-{self.device.device_name}"

    @property
    def should_skip(self) -> bool:
        """Whether this test case should be skipped."""
        return bool(self.device.json_error) or not self.device.json_string


# Backward compatibility alias
TestCase = VerificationCase


@dataclass
class CaseResult:
    """Outcome of running a single VerificationCase."""
    test_case: VerificationCase
    status: CaseStatus = CaseStatus.UNKNOWN
    message: str = ""
    match_type: Optional[str] = None
    expected: Optional[str] = None
    actual: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "command": self.test_case.command,
            "input_options": self.test_case.input_options,
            "device": self.test_case.device.device_name,
            "status": self.status.value,
            "message": self.message,
            "match_type": self.match_type,
        }


# Backward compatibility alias
TestResult = CaseResult


@dataclass
class CommandSuite:
    """All test data for a single command (loaded from one JSON fixture file).

    This corresponds to one ``test_data/data_NN_show_xxx.json`` file.
    """
    command: str
    command_index: int
    data_filename: str
    test_cases: List[VerificationCase] = field(default_factory=list)

    @property
    def test_count(self) -> int:
        return len(self.test_cases)

    def iter_runnable(self):
        """Iterate only over non-skippable test cases."""
        return (tc for tc in self.test_cases if not tc.should_skip)


# Backward compatibility alias
CommandTestSuite = CommandSuite


@dataclass
class SuiteSummary:
    """Aggregate statistics across all test cases."""
    total: int = 0
    passed: int = 0
    failed: int = 0
    skipped: int = 0
    errors: int = 0

    def record(self, status: CaseStatus) -> None:
        self.total += 1
        if status == CaseStatus.PASS:
            self.passed += 1
        elif status == CaseStatus.FAIL:
            self.failed += 1
        elif status == CaseStatus.SKIP:
            self.skipped += 1
        elif status == CaseStatus.ERROR:
            self.errors += 1

    @property
    def pass_rate(self) -> float:
        run = self.passed + self.failed
        return (self.passed / run * 100) if run > 0 else 0.0

    @property
    def all_passed(self) -> bool:
        return self.failed == 0 and self.errors == 0


# Backward compatibility alias
TestSummary = SuiteSummary
