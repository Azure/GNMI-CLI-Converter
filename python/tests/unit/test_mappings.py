"""Auto-generated from _legacy_tests.py — CLI-to-gNMI mapping tests"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from gnmi_cli_lib import CLI_TO_GNMI_MAPPING


# =============================================================================
# CLI_TO_GNMI_MAPPING Tests
# =============================================================================

class TestCliToGnmiMapping:
    """Tests for CLI_TO_GNMI_MAPPING completeness and structure."""
    
    def test_mapping_count(self):
        """Test that mapping has expected number of entries."""
        # 70+ mappings using show command format
        assert len(CLI_TO_GNMI_MAPPING) >= 60
    
    def test_buffer_pool_commands(self):
        """Test buffer_pool command mappings exist."""
        
        assert "show buffer_pool watermark" in CLI_TO_GNMI_MAPPING
        assert "show buffer_pool persistent-watermark" in CLI_TO_GNMI_MAPPING
    
    def test_headroom_pool_commands(self):
        """Test headroom-pool command mappings exist."""
        
        assert "show headroom-pool watermark" in CLI_TO_GNMI_MAPPING
        assert "show headroom-pool persistent-watermark" in CLI_TO_GNMI_MAPPING
    
    def test_priority_group_commands(self):
        """Test priority-group command mappings exist."""
        
        assert "show priority-group watermark shared" in CLI_TO_GNMI_MAPPING
        assert "show priority-group persistent-watermark shared" in CLI_TO_GNMI_MAPPING
        assert "show priority-group watermark headroom" in CLI_TO_GNMI_MAPPING
        assert "show priority-group persistent-watermark headroom" in CLI_TO_GNMI_MAPPING
    
    def test_queue_commands(self):
        """Test queue command mappings exist."""
        
        assert "show queue watermark unicast" in CLI_TO_GNMI_MAPPING
        assert "show queue persistent-watermark unicast" in CLI_TO_GNMI_MAPPING
        assert "show queue watermark multicast" in CLI_TO_GNMI_MAPPING
        assert "show queue persistent-watermark multicast" in CLI_TO_GNMI_MAPPING
    
    def test_clock_commands(self):
        """Test clock command mappings exist."""
        
        assert "show clock" in CLI_TO_GNMI_MAPPING
        assert "show clock timezones" in CLI_TO_GNMI_MAPPING
    
    def test_dropcounters_commands(self):
        """Test dropcounters command mappings exist."""
        
        assert "show dropcounters capabilities" in CLI_TO_GNMI_MAPPING
        assert "show dropcounters configuration" in CLI_TO_GNMI_MAPPING
        assert "show dropcounters counts" in CLI_TO_GNMI_MAPPING
    
    def test_mapping_structure(self):
        """Test that each mapping has required fields."""
        
        for cmd, mapping in CLI_TO_GNMI_MAPPING.items():
            assert "gnmi_path" in mapping, f"Missing gnmi_path in {cmd}"
            assert "command" in mapping, f"Missing command in {cmd}"
            assert "subcommand" in mapping, f"Missing subcommand in {cmd}"
    
    def test_mapping_gnmi_paths(self):
        """Test specific gNMI paths are correct."""
        
        assert CLI_TO_GNMI_MAPPING["show buffer_pool watermark"]["gnmi_path"] == \
            "SHOW/buffer_pool/watermark"
        assert CLI_TO_GNMI_MAPPING["show clock"]["gnmi_path"] == "SHOW/clock"

class TestNewCliMappings:
    """Tests for new CLI mappings with 'show' prefix."""
    
    def test_show_buffer_pool_watermark_mapping(self):
        """Test show buffer_pool watermark CLI mapping."""
        
        assert "show buffer_pool watermark" in CLI_TO_GNMI_MAPPING
    
    def test_show_headroom_pool_watermark_mapping(self):
        """Test show headroom-pool watermark CLI mapping."""
        
        assert "show headroom-pool watermark" in CLI_TO_GNMI_MAPPING
    
    def test_show_mac_aging_time_mapping(self):
        """Test show mac aging-time CLI mapping (new naming)."""
        
        assert "show mac aging-time" in CLI_TO_GNMI_MAPPING
    
    def test_show_ipv6_prefix_list_mapping(self):
        """Test show ipv6 prefix-list CLI mapping (new naming)."""
        
        assert "show ipv6 prefix-list" in CLI_TO_GNMI_MAPPING
    
    def test_show_ipv6_link_local_mode_mapping(self):
        """Test show ipv6 link-local-mode CLI mapping (new naming)."""
        
        assert "show ipv6 link-local-mode" in CLI_TO_GNMI_MAPPING
    
    def test_show_interfaces_naming_mode_mapping(self):
        """Test show interfaces naming-mode CLI mapping (new naming)."""
        
        assert "show interfaces naming-mode" in CLI_TO_GNMI_MAPPING
    
    def test_show_queue_watermark_unicast_mapping(self):
        """Test show queue watermark unicast CLI mapping."""
        
        assert "show queue watermark unicast" in CLI_TO_GNMI_MAPPING
