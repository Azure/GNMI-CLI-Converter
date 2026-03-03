"""
Conftest for sonic-utilities baseline tests.

The ``command_parser`` fixture is inherited from the root conftest.py.
This file adds baseline-specific markers and utilities.
"""

import pytest


def pytest_configure(config):
    """Register custom markers."""
    config.addinivalue_line(
        "markers",
        "sonic_baseline: mark test as sonic-utilities baseline comparison"
    )
