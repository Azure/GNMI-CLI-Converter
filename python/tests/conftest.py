"""
Root conftest.py for the unified test suite.

Responsibilities:
- Ensure ``gnmi_cli_lib`` is importable (add parent to sys.path).
- Ensure ``tests.core`` is importable.
- Provide shared fixtures available to both unit and integration tests.
- Configure collection to skip non-test modules.
"""

import sys
from pathlib import Path

import pytest

# ---------------------------------------------------------------------------
# Path setup — make gnmi_cli_lib and tests.core importable
# ---------------------------------------------------------------------------

_PROJECT_ROOT = Path(__file__).resolve().parent.parent  # python/
_PYTHON_DIR = _PROJECT_ROOT
_TESTS_DIR = Path(__file__).resolve().parent  # python/tests/

# Ensure gnmi_cli_lib is importable
if str(_PYTHON_DIR) not in sys.path:
    sys.path.insert(0, str(_PYTHON_DIR))

# Ensure tests package is importable
if str(_PROJECT_ROOT) not in sys.path:
    sys.path.insert(0, str(_PROJECT_ROOT))


# ---------------------------------------------------------------------------
# Collection configuration — prevent pytest from collecting dataclass modules
# ---------------------------------------------------------------------------

collect_ignore_glob = [
    "core/test_case.py",   # Data models, not tests
    "core/coverage.py",
    "core/comparison.py",
    "core/normalization.py",
    "core/data_loader.py",
    "core/runner.py",
    "reports/*.py",
]


# ---------------------------------------------------------------------------
# Shared fixtures
# ---------------------------------------------------------------------------

@pytest.fixture(scope="session")
def command_parser():
    """Session-scoped CommandParser instance (expensive to create)."""
    from gnmi_cli_lib.parser import CommandParser
    return CommandParser()


@pytest.fixture(scope="session")
def converter():
    """Session-scoped GnmiToCliConverter instance."""
    from gnmi_cli_lib.converter import GnmiToCliConverter
    return GnmiToCliConverter()


@pytest.fixture(scope="session")
def registry():
    """Session-scoped CommandRegistry with all commands registered."""
    from gnmi_cli_lib.registry import get_default_registry
    return get_default_registry()
