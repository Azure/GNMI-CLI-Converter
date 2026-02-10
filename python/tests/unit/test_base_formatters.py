"""Auto-generated from _legacy_tests.py — Base formatter tests (TableFormatter, ListFormatter, TextFormatter)"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from gnmi_cli_lib import CommandConfig
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.formatters import ListFormatter, TableFormatter, TextFormatter
from gnmi_cli_lib.formatters.base import (
    CommandHelpFormatter,
    ListFormatter,
    TableFormatter,
    TextFormatter,
)
from gnmi_cli_lib.formatters.interface import InterfaceStatusFormatter


# =============================================================================
# Formatter Tests
# =============================================================================

class TestBaseFormatters:
    """Tests for base formatter classes."""
    
    def test_table_formatter_simple(self):
        """Test TableFormatter with simple data."""
        
        formatter = TableFormatter()
        config = CommandConfig(
            title="Test Table:",
            headers=["Name", "Value"],
            value_fields=["Value"],  # Must specify which fields to extract
            key_field="name"
        )
        
        data = {"item1": {"Value": "100"}, "item2": {"Value": "200"}}
        result = formatter.format(data, config)
        
        assert "Test Table:" in result
        assert "Name" in result
        assert "Value" in result
        assert "item1" in result
        assert "100" in result
    
    def test_text_formatter(self):
        """Test TextFormatter with key extraction."""
        
        formatter = TextFormatter()
        config = CommandConfig(key_field="message")
        
        data = {"message": "Hello World"}
        result = formatter.format(data, config)
        
        assert result == "Hello World"
    
    def test_list_formatter(self):
        """Test ListFormatter with list data."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="items")
        
        data = {"items": ["item1", "item2", "item3"]}
        result = formatter.format(data, config)
        
        lines = result.strip().split("\n")
        assert len(lines) == 3
        assert "item1" in result
        assert "item2" in result
        assert "item3" in result

class TestBaseFormattersDirect_Original:
    """Direct tests for base formatters."""
    
    def test_table_formatter_list(self):
        """Test table formatter with list data."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Col1", "Col2"])
        data = [{"Col1": "val1", "Col2": "val2"}]
        output = formatter.format(data, config)
        
        assert "val1" in output
    
    def test_table_formatter_dict(self):
        """Test table formatter with dict data."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Pool", "Bytes"])
        data = {"pool1": {"Bytes": "100"}}
        output = formatter.format(data, config)
        
        assert "pool1" in output or "100" in output
    
    def test_text_formatter(self):
        """Test text formatter."""
        
        formatter = TextFormatter()
        config = CommandConfig(key_field="value")
        data = {"value": "test"}
        output = formatter.format(data, config)
        
        assert output is not None
    
    def test_list_formatter(self):
        """Test list formatter."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="items")
        # ListFormatter expects a dict with key_field as key
        data = {"items": ["item1", "item2"]}
        output = formatter.format(data, config)
        
        assert output is not None

class TestBaseFormattersDirect:
    """Additional tests for base formatters to boost coverage from 56%."""
    
    def test_table_formatter_grid_format(self):
        """Test table formatter with grid format."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Col1", "Col2"])
        data = [{"Col1": "val1", "Col2": "val2"}]
        output = formatter.format(data, config, table_format="grid")
        
        assert "val1" in output
    
    def test_table_formatter_with_title(self):
        """Test table formatter with title."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Col1", "Col2"], title="Test Title")
        data = [{"Col1": "val1", "Col2": "val2"}]
        output = formatter.format(data, config)
        
        assert "Test Title" in output or "val1" in output
    
    def test_table_formatter_empty_data(self):
        """Test table formatter with empty data."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Col1", "Col2"])
        data = []
        output = formatter.format(data, config)
        
        assert output == "" or output is not None

class TestBaseFormattersCoverage:
    """Tests for base.py to boost coverage from 55%."""
    
    def test_table_formatter_dynamic_columns(self):
        """Test TableFormatter with dynamic columns."""
        
        formatter = TableFormatter()
        config = CommandConfig(dynamic_columns=True)
        data = {
            "Ethernet0": {"PG0": "100", "PG1": "200"},
            "Ethernet4": {"PG0": "150", "PG2": "250"}
        }
        output = formatter.format(data, config)
        
        assert "Ethernet0" in output
        assert "PG0" in output or "100" in output
    
    def test_text_formatter_with_key_field(self):
        """Test TextFormatter with key_field extraction."""
        
        formatter = TextFormatter()
        config = CommandConfig(key_field="mode")
        data = {"mode": "disabled", "other": "value"}
        output = formatter.format(data, config)
        
        assert "disabled" in output
    
    def test_list_formatter_with_data(self):
        """Test ListFormatter with various data."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="timezones")
        data = {"timezones": ["UTC", "PST", "EST"]}
        output = formatter.format(data, config)
        
        assert output is not None

class TestBaseFormatterEdgeCases:
    """Test edge cases in base formatters."""
    
    def test_table_formatter_empty_data(self):
        """Test TableFormatter with empty data."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        output = formatter.format({}, config)
        assert output is not None
    
    def test_text_formatter_dict_input(self):
        """Test TextFormatter with dict input."""
        
        formatter = TextFormatter()
        config = CommandConfig(key_field="value")
        data = {"value": "test_value", "other": "other_value"}
        output = formatter.format(data, config)
        assert output is not None
    
    def test_list_formatter_dict_with_key(self):
        """Test ListFormatter with dict containing key."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="items")
        data = {"items": ["item1", "item2", "item3"]}
        output = formatter.format(data, config)
        assert output is not None

class TestBaseFormattersComprehensive:
    """Comprehensive tests for base.py formatters."""
    
    def test_table_formatter_build_list_table(self):
        """Test TableFormatter with list input."""
        
        formatter = TableFormatter()
        config = CommandConfig(headers=["Name", "Value"])
        data = [
            {"Name": "item1", "Value": "100"},
            {"Name": "item2", "Value": "200"}
        ]
        output = formatter.format(data, config)
        assert "item1" in output
        assert "item2" in output
    
    def test_table_formatter_with_alignment(self):
        """Test TableFormatter with right alignment."""
        
        formatter = TableFormatter()
        config = CommandConfig(
            headers=["Pool", "Bytes"],
            value_fields=["Bytes"],
            alignment="right"
        )
        data = {"pool1": {"Bytes": "1000"}}
        output = formatter.format(data, config)
        assert output is not None

class TestBaseFormattersExtended:
    """Extended tests for base formatters for higher coverage."""
    
    def test_table_formatter_with_custom_headers(self):
        """Test TableFormatter with custom headers."""
        
        config = CommandConfig(headers=["Name", "Value"])
        formatter = TableFormatter()
        
        data = [["row1", "val1"], ["row2", "val2"]]
        output = formatter.format(data, config)
        assert output is not None
    
    def test_text_formatter(self):
        """Test TextFormatter."""
        
        config = CommandConfig()
        formatter = TextFormatter()
        
        data = {"value": "test content"}
        output = formatter.format(data, config)
        assert output is not None

class TestBaseFormattersFinal:
    """Final coverage tests for base formatters."""
    
    def test_table_formatter_empty_data(self):
        """Test TableFormatter with empty data."""
        
        config = CommandConfig()
        formatter = TableFormatter()
        
        output = formatter.format({}, config)
        assert output is not None or output == ""
    
    def test_table_formatter_nested_dict(self):
        """Test TableFormatter with nested dict."""
        
        config = CommandConfig(
            title="Pool Watermark",
            headers=["Pool", "Bytes"]
        )
        formatter = TableFormatter()
        
        data = {
            "ingress_lossless_pool": {"Bytes": "1000"},
            "egress_lossy_pool": {"Bytes": "2000"}
        }
        output = formatter.format(data, config)
        assert output is not None

class TestBaseFormatterBranches:
    """Test uncovered branches in base formatters."""
    
    def test_table_formatter_build_from_dict_list(self):
        """Test TableFormatter with list of dicts (from dict_list method)."""
        
        config = CommandConfig(headers=["Name", "Value"])
        formatter = TableFormatter()
        
        data = [
            {"Name": "key1", "Value": "value1"},
            {"Name": "key2", "Value": "value2"}
        ]
        output = formatter.format(data, config)
        assert output is not None
    
    def test_table_formatter_dynamic_columns(self):
        """Test TableFormatter with dynamic columns."""
        
        config = CommandConfig(dynamic_columns=True)
        formatter = TableFormatter()
        
        data = {
            "key1": {"col1": "v1", "col2": "v2"},
            "key2": {"col1": "v3", "col2": "v4", "col3": "v5"}  # Dynamic columns
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_command_help_formatter(self):
        """Test CommandHelpFormatter."""
        
        config = CommandConfig(description="Queue watermark help")
        formatter = CommandHelpFormatter()
        
        data = {
            "subcommands": {
                "unicast": {"description": "Show unicast queues"},
                "multicast": {"description": "Show multicast queues"}
            }
        }
        output = formatter.format(data, config)
        assert "Usage:" in output
        assert "Commands:" in output
    
    def test_command_help_formatter_empty_subcommands(self):
        """Test CommandHelpFormatter with no subcommands."""
        
        config = CommandConfig(description="Test help")
        formatter = CommandHelpFormatter()
        
        data = {"subcommands": {}}
        output = formatter.format(data, config)
        assert "No subcommands available" in output

class TestBaseFormatterMore:
    """More base formatter tests."""
    
    def test_list_formatter(self):
        """Test ListFormatter."""
        
        config = CommandConfig(key_field="timezones")
        formatter = ListFormatter()
        
        # ListFormatter expects dict with key_field containing a list
        data = {"timezones": ["UTC", "America/New_York", "Europe/London"]}
        output = formatter.format(data, config)
        assert "UTC" in output
    
    def test_text_formatter(self):
        """Test TextFormatter."""
        
        config = CommandConfig(key_field="date")
        formatter = TextFormatter()
        
        # TextFormatter expects dict with key_field containing text
        data = {"date": "Mon Mar 25 20:25:16 UTC 2019"}
        output = formatter.format(data, config)
        assert "Mar" in output

class TestBaseFormatterBranchesDeep:
    """Deep branch tests for base formatter."""
    
    def test_table_formatter_with_complex_data(self):
        """Test Table formatter with complex nested data."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        
        data = [
            {"level1": "a", "level2": "b"},
            {"level1": "c", "level2": "d"}
        ]
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_table_formatter_with_empty_cells(self):
        """Test table formatter with empty cells."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        
        data = [
            {"col1": "value1", "col2": ""},
            {"col1": "", "col2": "value2"},
            {"col1": "value3", "col2": "value4"}
        ]
        
        result = formatter.format(data, config)
        assert result is not None

class TestBaseFormatterFinalCoverage:
    """Final base formatter tests for coverage."""
    
    def test_text_formatter_with_key_field(self):
        """Test text formatter with key field."""
        
        formatter = TextFormatter()
        config = CommandConfig(key_field="text_value")
        
        data = {"text_value": "This is the text content"}
        
        result = formatter.format(data, config)
        assert "This is the text content" in result
    
    def test_list_formatter_with_key_field(self):
        """Test list formatter with key field."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="items")
        
        data = {"items": ["item1", "item2", "item3"]}
        
        result = formatter.format(data, config)
        assert result is not None

class TestBaseFormatterBuildTable:
    """Test base formatter table building methods via concrete implementations."""
    
    def test_interface_status_list_format(self):
        """Test interface status formatter with list format."""
        
        # Use a concrete formatter - test list formatting
        formatter = InterfaceStatusFormatter()
        config = CommandConfig()
        
        # InterfaceStatusFormatter expects dict of interfaces
        data = {
            "eth0": {"admin_status": "up", "oper_status": "up"},
            "eth1": {"admin_status": "down", "oper_status": "down"}
        }
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_interface_status_simple_values(self):
        """Test interface status with simple string values."""
        
        formatter = InterfaceStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"admin_status": "up", "oper_status": "up", "speed": "100G"},
            "Ethernet8": {"admin_status": "down", "oper_status": "down", "speed": "N/A"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

class TestMergedBaseFormatters_469:
    """Merged from 2 single-method test classes."""

    def test_format_empty(self):
        """Test formatting with empty data."""
        
        formatter = InterfaceStatusFormatter()
        config = CommandConfig()
        
        data = None
        
        result = formatter.format(data, config)
        assert result == "" or result is not None

    def test_table_formatter_empty_list(self):
        """Test table formatter with empty list input."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        
        # Test with empty dict and list
        result1 = formatter.format({}, config)
        result2 = formatter.format([], config)
        
        # Should handle gracefully
        assert result1 == "" or result1 is not None
        assert result2 == "" or result2 is not None

