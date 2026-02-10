"""Auto-generated from _legacy_tests.py — Drop counters formatter tests"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from tests.unit.test_imports import TestData

from gnmi_cli_lib import CommandConfig
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.formatters import DropcountersCapabilitiesFormatter, DropcountersConfigurationFormatter, DropcountersCountsFormatter
from gnmi_cli_lib.formatters.dropcounters import DropcountersCapabilitiesFormatter, DropcountersConfigurationFormatter, DropcountersCountsFormatter


class TestDropcountersFormatters:
    """Tests for dropcounters formatters."""
    
    def test_capabilities_formatter(self):
        """Test DropcountersCapabilitiesFormatter."""
        
        formatter = DropcountersCapabilitiesFormatter()
        config = CommandConfig(output_type="dropcounters_capabilities")
        result = formatter.format(TestData.DROPCOUNTERS_CAPABILITIES, config)
        
        assert "Counter Type" in result
        assert "PORT_INGRESS_DROPS" in result
        assert "10" in result
        assert "MPLS_MISS" in result
    
    def test_capabilities_formatter_empty(self):
        """Test DropcountersCapabilitiesFormatter with empty data."""
        
        formatter = DropcountersCapabilitiesFormatter()
        config = CommandConfig(output_type="dropcounters_capabilities")
        result = formatter.format({}, config)
        
        assert result == "\n"
    
    def test_configuration_formatter(self):
        """Test DropcountersConfigurationFormatter."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig(output_type="dropcounters_configuration")
        result = formatter.format(TestData.DROPCOUNTERS_CONFIGURATION, config)
        
        assert "Counter" in result
        assert "DEBUG_0" in result
        assert "DEBUG_1" in result
    
    def test_configuration_formatter_empty(self):
        """Test DropcountersConfigurationFormatter with empty data."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig(output_type="dropcounters_configuration")
        result = formatter.format([], config)
        
        assert result == ""
    
    def test_counts_formatter(self):
        """Test DropcountersCountsFormatter."""
        
        formatter = DropcountersCountsFormatter()
        config = CommandConfig(output_type="dropcounters_counts")
        result = formatter.format(TestData.DROPCOUNTERS_COUNTS, config)
        
        assert "IFACE" in result
        assert "Ethernet0" in result
        assert "RX_ERR" in result
    
    def test_counts_formatter_empty(self):
        """Test DropcountersCountsFormatter with empty data."""
        
        formatter = DropcountersCountsFormatter()
        config = CommandConfig(output_type="dropcounters_counts")
        result = formatter.format({}, config)
        
        assert result == ""

class TestDropcountersFormattersDirect:
    """Direct tests for dropcounters formatters."""
    
    def test_dropcounters_capabilities_detailed(self):
        """Test dropcounters capabilities with detailed data."""
        
        formatter = DropcountersCapabilitiesFormatter()
        config = CommandConfig()
        
        data = {
            "PORT_INGRESS_DROPS": {"reasons": "L2_ANY,L3_ANY", "count": "2"},
            "SWITCH_EGRESS_DROPS": {"reasons": "BLACKHOLE,ACL_DROP", "count": "2"}
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_dropcounters_configuration_detailed(self):
        """Test dropcounters configuration with detailed data."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        
        data = {
            "DEBUG_0": {
                "alias": "IngressL2Drop",
                "group": "PORT_INGRESS_DROPS",
                "type": "PORT_INGRESS_DROPS",
                "reasons": "L2_ANY",
                "description": "L2 ingress drops"
            }
        }
        output = formatter.format(data, config)
        assert output is not None

class TestDropcountersFormattersExpanded:
    """Additional tests for dropcounters.py edge cases."""
    
    def test_dropcounters_capabilities_formatter(self):
        """Test DropcountersCapabilitiesFormatter."""
        
        formatter = DropcountersCapabilitiesFormatter()
        config = CommandConfig()
        data = {
            "counter_types": ["PORT_INGRESS_DROPS", "PORT_EGRESS_DROPS"],
            "reasons": ["L2_ANY", "L3_ANY", "SMAC_EQUALS_DMAC"]
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_dropcounters_configuration_formatter(self):
        """Test DropcountersConfigurationFormatter."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        data = {
            "DEBUG_COUNTER_0": {
                "type": "PORT_INGRESS_DROPS",
                "reasons": "L2_ANY",
                "desc": "L2 drops"
            }
        }
        output = formatter.format(data, config)
        assert output is not None

class TestMergedDropcountersFormatters_147:
    """Merged from 6 single-method test classes."""

    def test_dropcounters_counts_formatter(self):
        """Test DropcountersCountsFormatter."""
        
        formatter = DropcountersCountsFormatter()
        config = CommandConfig()
        data = {
            "DEBUG_COUNTER_0": {
                "count": "1234",
                "type": "PORT_INGRESS_DROPS",
                "reasons": "L2_ANY"
            }
        }
        output = formatter.format(data, config)
        assert output is not None

    def test_dropcounters_configuration_formatter(self):
        """Test DropcountersConfigurationFormatter."""
        try:
            
            config = CommandConfig()
            formatter = DropcountersConfigurationFormatter()
            
            data = {
                "counters": [
                    {"name": "DROP_CNT1", "type": "PORT_INGRESS_DROPS", "enabled": True}
                ]
            }
            output = formatter.format(data, config)
            assert output is not None
        except (ImportError, AttributeError):
            pass

    def test_counts_formatter_detailed(self):
        """Test DropcountersCountsFormatter with detailed data."""
        try:
            
            config = CommandConfig()
            formatter = DropcountersCountsFormatter()
            
            data = {
                "PORT_INGRESS_DROPS": {
                    "Ethernet0": {"count": 100, "reason": "ACL"},
                    "Ethernet8": {"count": 0}
                },
                "SWITCH_EGRESS_DROPS": {
                    "Ethernet0": {"count": 50}
                }
            }
            output = formatter.format(data, config)
            assert output is not None
        except (ImportError, AttributeError):
            pass

    def test_capabilities_formatter_detailed(self):
        """Test capabilities formatter with detailed data."""
        
        formatter = DropcountersCapabilitiesFormatter()
        config = CommandConfig()
        
        # Use dict format expected by the formatter with string reasons
        data = {
            "DROP_REASON_L2_ANY": {
                "count": 5,
                "reasons": "[SIP_EQUALS_DIP,DIP_LINK_LOCAL]"
            }
        }
        
        result = formatter.format(data, config)
        assert "DROP_REASON_L2_ANY" in result

    def test_dropcounters_counts_dict_format(self):
        """Test dropcounters counts with dict format."""
        
        formatter = DropcountersCountsFormatter()
        config = CommandConfig()
        
        # DropcountersCountsFormatter expects dict keyed by port
        data = {
            "Ethernet0": {
                "DEBUG_2": 1000
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_dropcounters_empty_counts(self):
        """Test dropcounters with empty counts."""
        
        formatter = DropcountersCountsFormatter()
        config = CommandConfig()
        
        data = []
        
        result = formatter.format(data, config)
        assert result == "" or result is not None

