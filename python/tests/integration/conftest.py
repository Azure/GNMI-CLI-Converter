"""
Integration test conftest — parameterized test case discovery and markers.

This conftest automatically discovers all JSON fixture files and expands them
into ``(command × variant × device)`` pytest parameters.  This replaces the
70 individual ``test_*.py`` files with a single parameterized test function.

All tests collected here are auto-marked with ``@pytest.mark.integration``.
Large BGP data tests are additionally marked ``@pytest.mark.slow``.

Usage::

    pytest -m integration -q          # run only integration tests
    pytest -m "not slow" -q           # skip slow BGP tests
    pytest -m "integration and not slow" -q
"""

import json
import sys
from pathlib import Path

import pytest

from tests.core.data_loader import collect_all_test_cases
from tests.core.test_case import VerificationCase


def pytest_collection_modifyitems(items):
    """Auto-mark integration tests and tag slow BGP tests."""
    for item in items:
        if "/tests/integration/" in str(item.fspath):
            item.add_marker(pytest.mark.integration)
            # BGP neighbor tests with large JSON fixtures (~12 MB) take 1-9s each
            if "ipv6_bgp_neighbors" in item.nodeid:
                item.add_marker(pytest.mark.slow)


def pytest_generate_tests(metafunc):
    """Dynamically parameterize tests that request ``test_case``."""
    if "test_case" in metafunc.fixturenames:
        all_cases = collect_all_test_cases()
        metafunc.parametrize(
            "test_case",
            all_cases,
            ids=lambda tc: tc.test_id,
        )
