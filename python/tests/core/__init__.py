"""
Test core infrastructure — comparison engine, data models, loaders.

Public API:
    from tests.core import compare_outputs, VerificationCase, CaseResult, collect_all_test_cases
"""

from tests.core.comparison import compare_outputs, ComparisonResult
from tests.core.test_case import (
    # Primary names (avoid pytest collection warnings)
    VerificationCase,
    CaseResult,
    CaseStatus,
    SuiteSummary,
    CommandSuite,
    DeviceData,
    # Backward compatibility aliases
    TestCase,
    TestResult,
    TestStatus,
    TestSummary,
    CommandTestSuite,
    DeviceTestData,
)
from tests.core.data_loader import (
    collect_all_test_cases,
    collect_runnable_test_cases,
    load_all_fixtures,
    load_fixture_file,
)
from tests.core.runner import run_case
from tests.core.normalization import normalize_output
from tests.core.coverage import check_gnmi_coverage, extract_json_values

__all__ = [
    "compare_outputs",
    "ComparisonResult",
    "VerificationCase",
    "CaseResult",
    "CaseStatus",
    "SuiteSummary",
    "CommandSuite",
    "DeviceData",
    # Backward compat
    "TestCase",
    "TestResult",
    "TestStatus",
    "TestSummary",
    "CommandTestSuite",
    "DeviceTestData",
    # Runner
    "run_case",
    # Loaders
    "collect_all_test_cases",
    "collect_runnable_test_cases",
    "load_all_fixtures",
    "load_fixture_file",
    "normalize_output",
    "check_gnmi_coverage",
    "extract_json_values",
]
