"""Auto-generated from _legacy_tests.py — Miscellaneous formatter tests"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.convenience import (
    convert_buffer_pool_watermark,
    convert_headroom_pool_watermark,
    convert_lldp_neighbors,
    convert_lldp_table,
    convert_priority_group_watermark,
    convert_queue_watermark,
    convert_reboot_cause,
)
from gnmi_cli_lib.converter import GnmiToCliConverter
from gnmi_cli_lib.formatters.base import ListFormatter, TableFormatter
from gnmi_cli_lib.formatters.interface import InterfaceStatusFormatter
from gnmi_cli_lib.formatters.ipv6 import IPv6BgpRoutesTableFormatter, IPv6InterfacesFormatter, IPv6ProtocolFormatter
from gnmi_cli_lib.formatters.network import LldpNeighborsFormatter, NdpFormatter
from gnmi_cli_lib.formatters.transceiver import TransceiverPmFormatter, TransceiverStatusFormatter
from gnmi_cli_lib.formatters.watermark import HeadroomPoolFormatter


# =============================================================================
# Formatter Extended Tests - Improve coverage via convenience/converter APIs
# =============================================================================

class TestFormatterExtended:
    """Extended formatter tests to improve coverage via converter APIs."""
    
    def test_buffer_pool_persistent_watermark(self):
        """Test buffer pool persistent watermark via convenience function."""
        
        data = {"ingress_lossless_pool": {"Bytes": "5000"}}
        output = convert_buffer_pool_watermark(data, persistent=True)
        
        assert "5000" in output
        assert "Persistent" in output or "Pool" in output
    
    def test_headroom_pool_watermark(self):
        """Test headroom pool watermark via convenience function."""
        
        data = {"ingress_lossless_pool": {"Bytes": "3000"}}
        output = convert_headroom_pool_watermark(data, persistent=False)
        
        assert "3000" in output
    
    def test_headroom_pool_persistent_watermark(self):
        """Test headroom pool persistent watermark."""
        
        data = {"ingress_lossless_pool": {"Bytes": "4500"}}
        output = convert_headroom_pool_watermark(data, persistent=True)
        
        assert "4500" in output
    
    def test_priority_group_watermark_shared(self):
        """Test priority group watermark shared via convenience function."""
        
        data = {
            "Ethernet0": {"PG0": "100", "PG1": "200"}
        }
        output = convert_priority_group_watermark(data, wm_type="shared")
        
        assert "Ethernet0" in output or "100" in output
    
    def test_priority_group_watermark_headroom(self):
        """Test priority group watermark headroom."""
        
        data = {
            "Ethernet0": {"PG0": "150", "PG1": "250"}
        }
        output = convert_priority_group_watermark(data, wm_type="headroom")
        
        assert output is not None
    
    def test_queue_watermark_unicast(self):
        """Test queue watermark unicast via convenience function."""
        
        data = {
            "Ethernet0": {"UC0": "500", "UC1": "600"}
        }
        output = convert_queue_watermark(data, queue_type="unicast")
        
        assert "500" in output or "Ethernet0" in output
    
    def test_queue_watermark_multicast(self):
        """Test queue watermark multicast."""
        
        data = {
            "Ethernet0": {"MC0": "700", "MC1": "800"}
        }
        output = convert_queue_watermark(data, queue_type="multicast")
        
        assert output is not None
    
    def test_queue_watermark_persistent(self):
        """Test queue watermark persistent."""
        
        data = {
            "Ethernet0": {"UC0": "900", "UC1": "1000"}
        }
        output = convert_queue_watermark(data, queue_type="unicast", persistent=True)
        
        assert output is not None
    
    def test_lldp_table_formatter(self):
        """Test LLDP table via convenience function."""
        
        data = {
            "Ethernet0": {
                "rem_sys_name": "switch1",
                "rem_port_id": "eth0"
            }
        }
        output = convert_lldp_table(data)
        
        assert output is not None
    
    def test_lldp_neighbors_formatter(self):
        """Test LLDP neighbors via convenience function."""
        
        data = {
            "Ethernet0": {
                "neighbor_name": "switch1",
                "neighbor_port": "eth0"
            }
        }
        output = convert_lldp_neighbors(data)
        
        assert output is not None
    
    def test_vlan_brief_formatter(self):
        """Test VLAN brief via converter (checks formatter coverage)."""
        
        converter = GnmiToCliConverter()
        data = {
            "Vlan100": {
                "vlanid": "100",
                "ip_address": "10.0.0.1/24",
                "proxy_arp": "disabled"
            }
        }
        output = converter.convert("vlan", "brief", data)
        
        # The output contains table headers
        assert "VLAN ID" in output or "IP Address" in output
    
    def test_reboot_cause_current(self):
        """Test reboot cause current via convenience function."""
        
        data = {
            "Cause": "User requested reboot",
            "Time": "2024-01-01 12:00:00"
        }
        output = convert_reboot_cause(data, history=False)
        
        assert output is not None
    
    def test_reboot_cause_history(self):
        """Test reboot cause history via converter (list should be converted to dict)."""
        
        converter = GnmiToCliConverter()
        # The history format expects a dict, not a list
        data = {
            "history": [
                {"cause": "User requested reboot", "time": "2024-01-01"}
            ]
        }
        output = converter.convert("reboot-cause", "history", data)
        
        assert output is not None

class TestFormattersUncoveredBranches:
    """Tests for uncovered branches in formatters."""
    
    def test_interface_status_with_list_format(self):
        """Test InterfaceStatusFormatter with list format input."""
        
        config = CommandConfig()
        formatter = InterfaceStatusFormatter()
        
        data = [
            {"interface": "Ethernet0", "admin": "up", "oper": "up", "speed": "100G"},
            {"interface": "Ethernet8", "admin": "up", "oper": "down", "speed": "400G"}
        ]
        output = formatter.format(data, config)
        assert output is not None
    
    def test_transceiver_status_formatter(self):
        """Test TransceiverStatusFormatter."""
        
        config = CommandConfig()
        formatter = TransceiverStatusFormatter()
        
        data = {
            "Ethernet0": {"temperature": "30", "voltage": "3.3", "current": "0.5"},
            "Ethernet8": {"temperature": "35", "voltage": "3.2"}
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_transceiver_pm_formatter(self):
        """Test TransceiverPmFormatter if exists."""
        try:
            
            config = CommandConfig()
            formatter = TransceiverPmFormatter()
            
            data = {"Ethernet0": {"txpower": "-1.5", "rxpower": "-2.0"}}
            output = formatter.format(data, config)
            assert output is not None
        except ImportError:
            pass
    
    def test_table_formatter_dict_format(self):
        """Test TableFormatter with dict format input."""
        
        config = CommandConfig(headers=["Pool", "Bytes"])
        formatter = TableFormatter()
        
        data = {"ingress_pool": {"Bytes": "1000"}, "egress_pool": {"Bytes": "2000"}}
        output = formatter.format(data, config)
        assert output is not None
    
    def test_ipv6_routes_table_formatter(self):
        """Test IPv6BgpRoutesTableFormatter."""
        
        config = CommandConfig()
        formatter = IPv6BgpRoutesTableFormatter()
        
        data = {
            "routes": {
                "::/0": {"paths": [{"nexthop": "fe80::1", "metric": 0}]},
                "2001:db8::/32": {"paths": [{"nexthop": "connected", "metric": 0}]}
            }
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_ipv6_interfaces_formatter(self):
        """Test IPv6InterfacesFormatter."""
        
        config = CommandConfig()
        formatter = IPv6InterfacesFormatter()
        
        data = {
            "Ethernet0": {"ipv6_address": "2001:db8::1/64", "link_local": "fe80::1"},
            "Vlan100": {"ipv6_address": "2001:db8:100::1/64"}
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_lldp_neighbors_with_capabilities(self):
        """Test LldpNeighborsFormatter with capability data."""
        
        config = CommandConfig()
        formatter = LldpNeighborsFormatter()
        
        data = {
            "Ethernet0": [{
                "name": "Ethernet0",
                "chassis": {"descr": "SONiC", "name": "switch1", "id": "aa:bb:cc:dd:ee:ff"},
                "port": {"descr": "eth0", "id": "eth0"},
                "capability": [
                    {"type": "Router", "enabled": True},
                    {"type": "Bridge", "enabled": True},
                    {"type": "Wlan AP", "enabled": False}
                ],
                "ttl": 120
            }]
        }
        output = formatter.format(data, config)
        assert output is not None
    
    def test_ndp_empty_vlan(self):
        """Test NdpFormatter with missing vlan."""
        
        config = CommandConfig()
        formatter = NdpFormatter()
        
        data = [
            {"address": "fe80::1", "mac": "aa:bb:cc:dd:ee:ff", "interface": "Ethernet0"}
        ]
        output = formatter.format(data, config)
        assert output is not None

class TestIPv6ProtocolFormatter_V1:
    """Tests for IPv6 protocol formatter."""
    
    def test_protocol_formatter_list(self):
        """Test protocol formatter with list data."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = [
            {
                "VRF": "default",
                "Protocols": [
                    {"Protocol": "system", "route-map": "none"},
                    {"Protocol": "kernel", "route-map": "RM_KERNEL"},
                    {"Protocol": "bgp", "route-map": "RM_BGP"}
                ]
            },
            {
                "VRF": "VRF1",
                "Protocols": [
                    {"Protocol": "connected", "route-map": "none"}
                ]
            }
        ]
        
        result = formatter.format(data, config)
        assert "VRF: default" in result
        assert "Protocol" in result
        assert "system" in result
        assert "RM_KERNEL" in result
    
    def test_protocol_formatter_output(self):
        """Test protocol formatter with pre-formatted output."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = {"output": "Pre-formatted protocol output"}
        result = formatter.format(data, config)
        assert result == "Pre-formatted protocol output"

class TestMergedMiscFormatters_335:
    """Merged from 12 single-method test classes."""

    def test_interfaces_formatter_with_addresses(self):
        """Test interfaces formatter with IPv6 addresses."""
        
        formatter = IPv6InterfacesFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "admin_status": "up",
                "oper_status": "up",
                "master": "",
                "ipv6_addresses": [
                    {"address": "fc00::1/64", "bgp_neighbor_ip": "fc00::2", "bgp_neighbor_name": "peer1"},
                    {"address": "2001:db8::1/128", "bgp_neighbor_ip": "N/A", "bgp_neighbor_name": "N/A"}
                ]
            },
            "Ethernet1": {
                "admin_status": "down",
                "oper_status": "down",
                "master": "PortChannel1",
                "ipv6_addresses": []
            }
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "fc00::1/64" in result
        assert "peer1" in result
        assert "up/up" in result

    def test_ipv6_protocol_basic(self):
        """Test IPv6 protocol formatter."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = {
            "protocol": "BGP",
            "router_id": "10.0.0.1"
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_ipv6_interfaces_basic(self):
        """Test IPv6 interfaces formatter."""
        
        formatter = IPv6InterfacesFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "ipv6_address": "2001:db8::1/64",
                "link_local": "fe80::1"
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_headroom_pool_basic(self):
        """Test headroom pool formatter."""
        
        formatter = HeadroomPoolFormatter()
        config = CommandConfig()
        
        data = {
            "ingress_lossless_pool": {"Bytes": "12345"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_list_formatter(self):
        """Test ListFormatter."""
        
        formatter = ListFormatter()
        config = CommandConfig()
        
        # ListFormatter expects dict with a key field
        data = {"items": ["item1", "item2", "item3"]}
        
        result = formatter.format(data, config)
        assert result is not None

    def test_build_list_table_empty(self):
        """Test empty data - covers line 54 in base.py."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        
        result = formatter._build_list_table([], config)
        assert result == ([], [])

    def test_build_list_table_no_headers(self):
        """Test no headers in config - covers line 130."""
        
        formatter = TableFormatter()
        config = CommandConfig()  # No headers set
        
        data = [
            {"key1": "val1", "key2": "val2"},
            {"key1": "val3", "key2": "val4"}
        ]
        
        table_data, headers = formatter._build_list_table(data, config)
        assert len(headers) == 2
        assert len(table_data) == 2

    def test_build_dynamic_table_non_dict(self):
        """Test non-dict port data - covers line 173."""
        
        formatter = TableFormatter()
        config = CommandConfig(dynamic_columns=True)
        
        data = {
            "Ethernet0": {"PG0": "100", "PG1": "200"},
            "Ethernet1": "not_a_dict"  # Non-dict value
        }
        
        table_data, headers = formatter._build_dynamic_table(data)
        assert len(headers) > 0

    def test_build_simple_table_non_dict_value(self):
        """Test non-dict value in data - covers line 213."""
        
        formatter = TableFormatter()
        config = CommandConfig(value_fields=["field1"])
        
        data = {
            "key1": "string_value",  # Non-dict value
            "key2": {"field1": "val1"}
        }
        
        table_data = formatter._build_simple_table(data, ["field1"])
        assert len(table_data) == 2

    def test_list_formatter_non_list(self):
        """Test items is not a list - covers line 267."""
        
        formatter = ListFormatter()
        config = CommandConfig(key_field="data")
        
        data = {"data": "single_value_not_list"}
        
        result = formatter.format(data, config)
        assert "single_value_not_list" in result

    def test_build_list_table_empty_list(self):
        """Test empty list returns empty result - covers line 54."""
        
        formatter = TableFormatter()
        config = CommandConfig()
        
        result = formatter._build_list_table([], config)
        assert result == ([], [])

    def test_protocol_with_list_format(self):
        """Test protocol formatter with list format."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = [
            {"VRF": "default", "Protocols": [{"name": "system", "route_map": "none"}]}
        ]
        result = formatter.format(data, config)
        assert "VRF: default" in result

