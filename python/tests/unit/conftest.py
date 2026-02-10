"""
Unit test conftest — auto-mark all tests in this directory as ``@pytest.mark.unit``.

This allows running only unit tests via::

    pytest -m unit -q
"""

import pytest


def pytest_collection_modifyitems(items):
    """Automatically add the ``unit`` marker to every test collected under tests/unit/."""
    for item in items:
        if "/tests/unit/" in str(item.fspath):
            item.add_marker(pytest.mark.unit)
