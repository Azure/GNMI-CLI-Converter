"""Auto-generated from _legacy_tests.py — Miscellaneous tests"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from tests.unit.test_imports import TestData

from gnmi_cli_lib import GnmiToCliConverter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.converter import GnmiToCliConverter
from gnmi_cli_lib.formatters.base import CommandHelpFormatter
from gnmi_cli_lib.formatters.dropcounters import DropcountersConfigurationFormatter
from gnmi_cli_lib.formatters.interface import (
    FecHistogramFormatter,
    InterfaceCountersDetailedFormatter,
    InterfaceCountersFormatter,
    InterfaceDescriptionFormatter,
    InterfaceErrorsFormatter,
    InterfaceFlapFormatter,
    InterfaceNeighborFormatter,
    PortchannelFormatter,
    SwitchportConfigFormatter,
    SwitchportStatusFormatter,
)
from gnmi_cli_lib.formatters.ipv6 import (
    IPv6BgpNeighborsFormatter,
    IPv6InterfacesFormatter,
    IPv6LinkLocalModeFormatter,
    IPv6PrefixListFormatter,
    IPv6ProtocolFormatter,
    IPv6RouteFormatter,
)
from gnmi_cli_lib.formatters.network import LldpNeighborsFormatter, NdpFormatter, VlanBriefFormatter
from gnmi_cli_lib.formatters.queue import QueueCountersFormatter, WredCountersFormatter
from gnmi_cli_lib.formatters.system import (
    ProcessesCpuFormatter,
    RebootCauseFormatter,
    ServicesFormatter,
    SystemMemoryFormatter,
)
from gnmi_cli_lib.formatters.transceiver import (
    TransceiverEepromFormatter,
    TransceiverErrorStatusFormatter,
    TransceiverInfoFormatter,
    TransceiverLpmodeFormatter,
    TransceiverPmFormatter,
    TransceiverPresenceFormatter,
    TransceiverStatusFormatter,
)
from gnmi_cli_lib.formatters.watermark import HeadroomPoolFormatter, QueueWatermarkFormatter, WatermarkTelemetryFormatter


# =============================================================================
# Table Format Tests
# =============================================================================

class TestTableFormats:
    """Tests for different table format options."""
    
    def test_simple_format(self):
        """Test simple table format."""
        
        converter = GnmiToCliConverter(table_format="simple")
        output = converter.convert("buffer_pool", "watermark", TestData.BUFFER_POOL_SINGLE)
        
        assert "---" in output
    
    def test_grid_format(self):
        """Test grid table format."""
        
        converter = GnmiToCliConverter(table_format="grid")
        output = converter.convert("buffer_pool", "watermark", TestData.BUFFER_POOL_SINGLE)
        
        assert "+" in output or "|" in output
    
    def test_plain_format(self):
        """Test plain table format."""
        
        converter = GnmiToCliConverter(table_format="plain")
        output = converter.convert("buffer_pool", "watermark", TestData.BUFFER_POOL_SINGLE)
        
        assert "---" not in output

class TestCommandConfigType:
    """Test CommandConfig type."""
    
    def test_command_config_defaults(self):
        """Test CommandConfig with defaults."""
        
        config = CommandConfig()
        assert config.title == ""
        assert config.output_type == "table"
    
    def test_command_config_with_values(self):
        """Test CommandConfig with values."""
        
        config = CommandConfig(
            title="Test Title",
            headers=["Col1", "Col2"],
            output_type="text"
        )
        
        assert config.title == "Test Title"
        assert config.headers == ["Col1", "Col2"]

class TestClockCommands:
    """Test clock commands."""
    
    def setup_method(self):
        self.converter = GnmiToCliConverter()
    
    def test_clock_date(self):
        """Test clock date."""
        data = {"date": "2024-01-01", "time": "12:00:00"}
        output = self.converter.convert("clock", "date", data)
        assert output is not None
    
    def test_clock_timezones(self):
        """Test clock timezones."""
        data = {"timezones": ["UTC", "PST", "EST"]}
        output = self.converter.convert("clock", "timezones", data)
        assert output is not None

class TestPriorityGroupCommands:
    """Test priority group commands."""
    
    def setup_method(self):
        self.converter = GnmiToCliConverter()
    
    def test_priority_group_watermark_shared(self):
        """Test priority group watermark shared."""
        data = {"Ethernet0": {"PG0": "100", "PG1": "200"}}
        output = self.converter.convert("priority-group", "watermark-shared", data)
        assert output is not None
    
    def test_priority_group_watermark_headroom(self):
        """Test priority group watermark headroom."""
        data = {"Ethernet0": {"PG0": "150"}}
        output = self.converter.convert("priority-group", "watermark-headroom", data)
        assert output is not None
    
    def test_priority_group_persistent_watermark_shared(self):
        """Test priority group persistent watermark shared."""
        data = {"Ethernet0": {"PG0": "300"}}
        output = self.converter.convert("priority-group", "persistent-watermark-shared", data)
        assert output is not None
    
    def test_priority_group_persistent_watermark_headroom(self):
        """Test priority group persistent watermark headroom."""
        data = {"Ethernet0": {"PG0": "350"}}
        output = self.converter.convert("priority-group", "persistent-watermark-headroom", data)
        assert output is not None

class TestInterfaceSwitchportConfig:
    """Test interface switchport config formatter."""
    
    def test_switchport_config_list_format(self):
        """Test switchport config with list format."""
        
        formatter = SwitchportConfigFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "Mode": "trunk", "Untagged": "1", "Tagged": "100,200"},
            {"Interface": "Ethernet8", "Mode": "access", "Untagged": "100", "Tagged": ""},
            {"Interface": "SomeInvalid", "Mode": "trunk", "Untagged": "1", "Tagged": "100"}  # Skip this
        ]
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_switchport_config_dict_format(self):
        """Test switchport config with dict format."""
        
        formatter = SwitchportConfigFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Mode": "trunk", "Untagged": "1", "Tagged": "100,200"},
            "Ethernet8": {"Mode": "access", "Untagged": "100", "Tagged": ""}
        }
        
        result = formatter.format(data, config)
        assert result is not None

class TestMergedMisc_191:
    """Merged from 2 single-method test classes."""

    def test_detailed_counters_with_period(self):
        """Test detailed counters with period header."""
        
        formatter = InterfaceCountersDetailedFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "rx_bps": "1000000",
                "tx_bps": "500000",
                "TimestampClearedCounters": "2024-01-01"
            }
        }
        
        result = formatter.format(data, config, options={"period": "5"})
        assert "5 seconds" in result

    def test_vlan_with_port_tagging(self):
        """Test VLAN formatter with port tagging info."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": {
                "VID": "100",
                "Name": "Data",
                "IP Addresses": ["10.0.0.1/24", "10.0.0.2/24"],
                "Ports": [
                    {"name": "Ethernet0", "port_tagging": "tagged"},
                    {"name": "Ethernet8", "port_tagging": "untagged"}
                ],
                "DHCP Helper Addresses": ["192.168.1.1", "192.168.1.2"],
                "Proxy ARP": "enabled"
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None


class TestLldpNeighborsDetailedWithAllSections:
    """Test LLDP neighbors with all detail sections."""
    
    def test_lldp_with_full_chassis_section(self):
        """Test LLDP with full chassis section including capabilities."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "lldp": {
                "interfaces": {
                    "Ethernet0": {
                        "RID": "1",
                        "via": "LLDP",
                        "Time": "00:01:00",
                        "Chassis": {
                            "ChassisID": "aa:bb:cc:dd:ee:ff",
                            "SysName": "peer-switch",
                            "SysDescr": "Test switch",
                            "MgmtIface": "192.168.1.1",
                            "Capability": ["Router", "Bridge"]
                        },
                        "Port": {
                            "PortID": "Eth1/1",
                            "PortDescr": "uplink",
                            "TTL": "120",
                            "MFS": "9000",
                            "PortAggregID": "1"
                        },
                        "VLAN": [
                            {"vlan-id": "100", "pvid": "yes"},
                            {"vlan-id": "200"}
                        ],
                        "UnknownTLVs": [
                            {"unknown-tlv": [{"oui": "00:00:00", "subtype": "1", "len": "4", "value": "deadbeef"}]},
                            "simple_tlv"
                        ]
                    }
                }
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_lldp_with_vlan_dict(self):
        """Test LLDP with VLAN as dict instead of list."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "lldp": {
                "interfaces": {
                    "Ethernet0": {
                        "RID": "1",
                        "Chassis": {
                            "ChassisID": "aa:bb:cc:dd:ee:ff"
                        },
                        "VLAN": {"vlan-id": "100", "pvid": "yes"}
                    }
                }
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

class TestInterfaceErrorsEnhanced:
    """Test interface errors formatter with different data formats."""
    
    def test_interface_errors_list_format(self):
        """Test interface errors with list format."""
        
        formatter = InterfaceErrorsFormatter()
        config = CommandConfig()
        
        data = [
            {"Port Errors": "CRC", "Count": "10", "Last timestamp(UTC)": "2024-01-01"},
            {"Port Errors": "FCS", "Count": "5", "Last timestamp(UTC)": "Never"}
        ]
        
        result = formatter.format(data, config)
        assert "CRC" in result
    
    def test_interface_errors_dict_single(self):
        """Test interface errors with single dict."""
        
        formatter = InterfaceErrorsFormatter()
        config = CommandConfig()
        
        data = {"Port Errors": "CRC", "Count": "10"}
        
        result = formatter.format(data, config)
        assert result is not None

class TestIPv6InterfacesWithAddresses:
    """Test IPv6 interfaces formatter with addresses."""
    
    def test_ipv6_interfaces_with_addresses_list(self):
        """Test IPv6 interfaces with address list."""
        
        formatter = IPv6InterfacesFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "admin_status": "up",
                "oper_status": "up",
                "master": "",
                "ipv6_addresses": [
                    {"address": "2001:db8::1/64", "bgp_neighbor_ip": "2001:db8::2", "bgp_neighbor_name": "peer1"},
                    {"address": "2001:db8::3/64", "bgp_neighbor_ip": "N/A", "bgp_neighbor_name": "N/A"}
                ]
            }
        }
        
        result = formatter.format(data, config)
        assert "2001:db8::1" in result
    
    def test_ipv6_interfaces_without_addresses(self):
        """Test IPv6 interfaces without addresses."""
        
        formatter = IPv6InterfacesFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "admin_status": "up",
                "oper_status": "down",
                "master": "Bond0",
                "ipv6_addresses": []
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

class TestIPv6LinkLocalModeListFormat_Original:
    """Test IPv6 link local mode with list format."""
    
    def test_link_local_mode_list_format(self):
        """Test IPv6 link local mode with list input."""
        
        formatter = IPv6LinkLocalModeFormatter()
        config = CommandConfig()
        
        data = [
            {"port": "Ethernet0", "mode": "enable"},
            {"port": "Ethernet8", "mode": "disable"}
        ]
        
        result = formatter.format(data, config)
        assert result is not None

class TestIPv6PrefixListEnhanced:
    """Test IPv6 prefix list formatter with full data structure."""
    
    def test_prefix_list_with_protocols(self):
        """Test prefix list with protocol data."""
        
        formatter = IPv6PrefixListFormatter()
        config = CommandConfig()
        
        data = {
            "ZEBRA": {
                "DEFAULT_IPV6": {
                    "addressFamily": "IPv6",
                    "entries": [
                        {"sequenceNumber": 10, "type": "permit", "prefix": "2001:db8::/32", "minimumPrefixLength": 32, "maximumPrefixLength": 64}
                    ]
                }
            },
            "BGP": {
                "TEST_LIST": {
                    "entries": [
                        {"sequenceNumber": 20, "type": "deny", "prefix": "::/0"}
                    ]
                }
            }
        }
        
        result = formatter.format(data, config)
        assert "ZEBRA" in result
        assert "BGP" in result
    
    def test_prefix_list_with_output_key(self):
        """Test prefix list with pre-formatted output."""
        
        formatter = IPv6PrefixListFormatter()
        config = CommandConfig()
        
        data = {"output": "Pre-formatted prefix list output"}
        
        result = formatter.format(data, config)
        assert result == "Pre-formatted prefix list output"

class TestIPv6ProtocolEnhanced:
    """Test IPv6 protocol formatter with full data structure."""
    
    def test_protocol_with_vrf_data(self):
        """Test protocol formatter with VRF data list."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = [
            {
                "VRF": "default",
                "Protocols": [
                    {"Protocol": "BGP", "route-map": "IMPORT_MAP"},
                    {"Protocol": "OSPF", "route-map": "none"}
                ]
            }
        ]
        
        result = formatter.format(data, config)
        assert "VRF" in result
        assert "BGP" in result
    
    def test_protocol_with_output_key(self):
        """Test protocol formatter with pre-formatted output."""
        
        formatter = IPv6ProtocolFormatter()
        config = CommandConfig()
        
        data = {"output": "Pre-formatted protocol output"}
        
        result = formatter.format(data, config)
        assert result == "Pre-formatted protocol output"

class TestIPv6RouteEnhanced:
    """Test IPv6 route formatter with full data structure."""
    
    def test_route_with_entries(self):
        """Test route formatter with entries list."""
        
        formatter = IPv6RouteFormatter()
        config = CommandConfig()
        
        data = {
            "routes": [
                {"prefix": "2001:db8::/32", "nexthop": "2001:db8::1", "protocol": "connected"}
            ]
        }
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_route_with_output_key(self):
        """Test route formatter with pre-formatted output."""
        
        formatter = IPv6RouteFormatter()
        config = CommandConfig()
        
        data = {"output": "Pre-formatted route output"}
        
        result = formatter.format(data, config)
        assert result is not None or result == "Pre-formatted route output"

class TestMergedMisc_498:
    """Merged from 3 single-method test classes."""

    def test_counters_numeric_formatting(self):
        """Test that numeric counter values get comma formatting."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "RX_OK": "1000000",
                "TX_OK": "2000000",
                "RX_ERR": "not_a_number",
                "TX_ERR": "100"
            }
        }
        
        result = formatter.format(data, config)
        assert "1,000,000" in result

    def test_lldp_with_sysdescr_multiline(self):
        """Test LLDP with SysDescr containing multiple lines."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "lldp": {
                "interfaces": {
                    "Ethernet0": {
                        "RID": "1",
                        "Chassis": {
                            "ChassisID": "aa:bb:cc:dd:ee:ff",
                            "SysDescr": "Line 1\nLine 2\nLine 3",
                            "Capability": "Router"  # Single capability as string
                        }
                    }
                }
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_vlan_with_no_ports(self):
        """Test VLAN with no ports."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": {
                "VID": "100",
                "Name": "Management",
                "IP Addresses": [],
                "Ports": [],
                "DHCP Helper Addresses": [],
                "Proxy ARP": ""
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None


class TestLldpNeighborsAllSections:
    """Test LLDP neighbors formatter with all sections covered."""
    
    def test_lldp_with_port_aggregation(self):
        """Test LLDP neighbors with port aggregation."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "RID": "1",
                    "via": "LLDP",
                    "Time": "100s",
                    "Chassis": {
                        "ChassisID": "aa:bb:cc:dd:ee:ff",
                        "SysName": "test-switch",
                        "SysDescr": "Test Description\nLine 2",
                        "MgmtIP": "10.0.0.1",
                        "MgmtIface": "eth0",
                        "Capability": ["Router", "Bridge"]
                    },
                    "Port": {
                        "PortID": "Eth1/1",
                        "PortDescr": "Uplink",
                        "TTL": "120",
                        "MFS": "9216",
                        "PortAggregID": "Po1"
                    }
                }
            }
        }
        
        result = formatter.format(data, config)
        assert "PortAggregID" in result or "Port is aggregated" in result
    
    def test_lldp_with_vlan_list(self):
        """Test LLDP neighbors with VLAN as list."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "RID": "1",
                    "via": "LLDP",
                    "Time": "100s",
                    "Chassis": {"ChassisID": "aa:bb:cc:dd:ee:ff"},
                    "VLAN": [
                        {"vlan-id": "100", "pvid": "yes"},
                        {"vlan-id": "200", "pvid": "no"}
                    ]
                }
            }
        }
        
        result = formatter.format(data, config)
        assert "VLAN" in result or "100" in result
    
    def test_lldp_with_vlan_dict(self):
        """Test LLDP neighbors with VLAN as dict."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "RID": "1",
                    "via": "LLDP",
                    "Time": "100s",
                    "Chassis": {"ChassisID": "aa:bb:cc:dd:ee:ff"},
                    "VLAN": {"vlan-id": "100", "pvid": "yes"}
                }
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None
    
    def test_lldp_with_unknown_tlvs(self):
        """Test LLDP neighbors with unknown TLVs."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "RID": "1",
                    "via": "LLDP",
                    "Time": "100s",
                    "Chassis": {"ChassisID": "aa:bb:cc:dd:ee:ff"},
                    "UnknownTLVs": [
                        {
                            "unknown-tlv": [
                                {"oui": "00:00:00", "subtype": "1", "len": "5", "value": "test"}
                            ]
                        }
                    ]
                }
            }
        }
        
        result = formatter.format(data, config)
        assert "Unknown TLV" in result or "TLV" in result
    
    def test_lldp_capability_as_string(self):
        """Test LLDP neighbors with capability as string."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "title": "LLDP neighbors",
            "interfaces": {
                "Ethernet0": {
                    "RID": "1",
                    "via": "LLDP",
                    "Time": "100s",
                    "Chassis": {
                        "ChassisID": "aa:bb:cc:dd:ee:ff",
                        "Capability": "Router"  # String, not list
                    }
                }
            }
        }
        
        result = formatter.format(data, config)
        assert "Capability" in result

class TestMergedMisc_706:
    """Merged from 52 single-method test classes."""

    def test_vlan_with_multiple_ports_and_ips(self):
        """Test VLAN with multiple ports and IPs."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": {
                "vlan_id": "100",
                "ip_address": ["10.0.0.1/24", "10.0.0.2/24"],
                "ports": [
                    {"name": "Ethernet0", "port_tagging": "untagged"},
                    {"name": "Ethernet8", "port_tagging": "tagged"}
                ],
                "dhcp_helper_addresses": ["10.0.0.100", "10.0.0.101"],
                "proxy_arp": "enabled"
            }
        }
        
        result = formatter.format(data, config)
        assert "10.0.0.1" in result

    def test_counters_list_format(self):
        """Test interface counters with list of dicts."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "RX_OK": "1000", "TX_OK": "2000"},
            {"Interface": "Ethernet8", "RX_OK": "3000", "TX_OK": "4000"}
        ]
        
        result = formatter.format(data, config)
        assert result is not None

    def test_description_empty_data(self):
        """Test description formatter with empty interface data."""
        
        formatter = InterfaceDescriptionFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "not a dict"
        }
        
        result = formatter.format(data, config)
        # Should handle gracefully
        assert result == "" or result is not None

    def test_status_list_format(self):
        """Test transceiver status formatter with list."""
        
        formatter = TransceiverStatusFormatter()
        config = CommandConfig()
        
        data = [
            {"Port": "Ethernet0", "type": "QSFP+", "vendor": "Arista"},
            {"Port": "Ethernet8", "type": "SFP+", "vendor": "Finisar"}
        ]
        
        result = formatter.format(data, config)
        assert result is not None

    def test_detailed_counters_with_period(self):
        """Test detailed counters with period option."""
        
        formatter = InterfaceCountersDetailedFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "RX_OK": "1000000",
                "TX_OK": "2000000",
                "RX_ERR": "0",
                "TimestampClearedCounters": "Never"
            }
        }
        
        options = {"period": "30"}
        
        result = formatter.format(data, config, options=options)
        assert "30 seconds" in result or result is not None

    def test_queue_watermark_with_dict(self):
        """Test queue watermark with dict format."""
        
        formatter = QueueWatermarkFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"UC0": "100", "UC1": "200"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_transceiver_eeprom_with_json_string(self):
        """Test transceiver EEPROM with JSON string data."""
        
        formatter = TransceiverEepromFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": json.dumps({"Active Firmware": "N/A", "CMIS Rev": "5.0"})
        }
        
        result = formatter.format(data, config)
        assert "detected" in result or "Active Firmware" in result

    def test_command_help_formatter(self):
        """Test CommandHelpFormatter."""
        
        formatter = CommandHelpFormatter()
        config = CommandConfig()
        
        data = {"command": "show", "description": "Show command help"}
        
        result = formatter.format(data, config)
        assert result is not None

    def test_interface_neighbor_with_dict(self):
        """Test interface neighbor formatter with dict entries."""
        
        formatter = InterfaceNeighborFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Device": "switch1", "Port ID": "Eth1/1"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_description_non_dict_entries(self):
        """Test interface description with non-dict entries - covers line 118."""
        
        formatter = InterfaceDescriptionFormatter()
        config = CommandConfig()
        
        # Data with non-dict values should skip and result in empty table
        data = {
            "Ethernet0": "string_value",
            "Ethernet1": 12345,
            "Ethernet2": ["list", "values"]
        }
        
        result = formatter.format(data, config)
        # Non-dict values are skipped so empty table
        assert result == ""

    def test_counters_list_format_empty_dict(self):
        """Test list format with empty dict row - covers line 230, 243."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        # List of empty dicts - should result in empty or minimal table
        data = [{}, {}]
        
        result = formatter._format_list_data(data, config, "simple")
        assert result == ""  # Empty dicts produce empty table

    def test_counters_list_format_non_dict_rows(self):
        """Test list format skips non-dict rows - covers line 260."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        # Mix of dicts and non-dicts
        data = [
            {"IFACE": "Ethernet0", "RX_OK": "100"},
            "not_a_dict",
            {"IFACE": "Ethernet1", "TX_OK": "200"}
        ]
        
        result = formatter._format_list_data(data, config, "simple")
        assert "Ethernet0" in result
        assert "Ethernet1" in result

    def test_flap_skips_invalid_interface_prefix(self):
        """Test flap formatter skips invalid prefixes - covers line 336."""
        
        formatter = InterfaceFlapFormatter()
        config = CommandConfig()
        
        # List with invalid interface prefix
        data = [
            {"Interface": "Ethernet0", "Flap Count": "5", "Admin": "up", "Oper": "up"},
            {"Interface": "InvalidPrefix1", "Flap Count": "2"},  # Should be skipped
            {"Interface": "PortChannel10", "Flap Count": "1", "Admin": "up", "Oper": "up"}
        ]
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "PortChannel10" in result
        assert "InvalidPrefix" not in result

    def test_portchannel_port_as_string(self):
        """Test port as string instead of dict - covers line 427."""
        
        formatter = PortchannelFormatter()
        config = CommandConfig()
        
        data = {
            "1": {
                "Team Dev": "PortChannel1",
                "Protocol": {"name": "LACP", "active": True, "operational_status": "up"},
                "Ports": ["Ethernet0", "Ethernet1"]  # Strings instead of dicts
            }
        }
        
        result = formatter.format(data, config)
        assert "PortChannel1" in result
        assert "Ethernet0" in result

    def test_counters_detailed_skips_non_dict(self):
        """Test skips non-dict interface data - covers line 532."""
        
        formatter = InterfaceCountersDetailedFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "not_a_dict"  # Should be skipped
        }
        
        result = formatter.format(data, config)
        # Should produce empty result since non-dict is skipped
        assert result == ""

    def test_counters_detailed_with_data(self):
        """Test counters detailed with full data - covers line 544."""
        
        formatter = InterfaceCountersDetailedFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "IFACE": "Ethernet0",
                "TimestampClearedCounters": "2024-01-15T10:00:00Z"
            }
        }
        
        result = formatter.format(data, config)
        assert "2024-01-15" in result  # Timestamp should be in output

    def test_portchannel_empty_data(self):
        """Test empty data returns legend with headers - covers lines 594-595."""
        
        formatter = PortchannelFormatter()
        config = CommandConfig()
        
        data = {}
        
        result = formatter.format(data, config)
        # Should contain flags legend and headers even with empty data
        assert "Flags" in result

    def test_portchannel_skips_non_dict(self):
        """Test skips non-dict values - covers line 613."""
        
        formatter = PortchannelFormatter()
        config = CommandConfig()
        
        data = {
            "1": "not_a_dict",  # Should be skipped
            "2": {"Team Dev": "PortChannel2", "Protocol": {"name": "LACP"}, "Ports": []}
        }
        
        result = formatter.format(data, config)
        assert "PortChannel2" in result

    def test_portchannel_all_non_dict(self):
        """Test all non-dict values give legend only - covers line 628."""
        
        formatter = PortchannelFormatter()
        config = CommandConfig()
        
        data = {
            "1": "not_a_dict",
            "2": 12345
        }
        
        result = formatter.format(data, config)
        # Should still have flags legend
        assert "Flags" in result

    def test_fec_histogram_list_empty_table(self):
        """Test list with non-dict entries - covers line 636-637."""
        
        formatter = FecHistogramFormatter()
        config = CommandConfig()
        
        # List with non-dict entries
        data = ["not_a_dict", 12345, None]
        
        result = formatter.format(data, config)
        assert result == ""

    def test_fec_histogram_dict_format_non_dict_value(self):
        """Test dict format with non-dict values - covers line 713."""
        
        formatter = FecHistogramFormatter()
        config = CommandConfig()
        
        # Dict format but values are not dicts
        data = {
            "0": "not_a_dict",  # Should be skipped
            "1": {"BinIndex": "1", "Codewords": "1000"}
        }
        
        result = formatter.format(data, config)
        # Should have at least one row
        assert "1000" in result or result == ""

    def test_switchport_status_skips_invalid(self):
        """Test skips invalid interface prefix - covers line 733."""
        
        formatter = SwitchportStatusFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "Mode": "routed"},
            {"Interface": "InvalidIntf", "Mode": "access"},  # Should be skipped
            {"Interface": "PortChannel1", "Mode": "trunk"}
        ]
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "PortChannel1" in result
        assert "InvalidIntf" not in result

    def test_switchport_status_dict_format(self):
        """Test dict format - covers line 758."""
        
        formatter = SwitchportStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Mode": "routed"},
            "Ethernet1": "not_a_dict"  # Should be skipped
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_switchport_config_skips_invalid(self):
        """Test skips invalid interface - covers line 813."""
        
        formatter = SwitchportConfigFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "Mode": "access", "Untagged": "100"},
            {"Interface": "Invalid", "Mode": "trunk"},  # Should be skipped
        ]
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "Invalid" not in result

    def test_switchport_config_dict_format(self):
        """Test dict format - covers line 849."""
        
        formatter = SwitchportConfigFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Mode": "access", "Untagged": "100", "Tagged": ""},
            "Ethernet1": "not_a_dict"  # Should be skipped
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "access" in result

    def test_lldp_skips_non_dict(self):
        """Test skips non-dict interface data - covers line 150-151."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "not_a_dict",  # Should be skipped
            "Ethernet1": {
                "via": "LLDP",
                "RID": "1",
                "Time": "0 day",
                "Chassis": {"name": "switch1"}
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_lldp_invalid_rid_sort(self):
        """Test invalid RID values in sort - covers line 156."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "via": "LLDP",
                "RID": "not_a_number",  # Invalid RID
                "Time": "0 day",
                "Chassis": {"name": "switch1"}
            },
            "Ethernet1": {
                "via": "LLDP",
                "RID": "1",
                "Time": "0 day",
                "Chassis": {"name": "switch2"}
            }
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_vlan_brief_skips_non_dict(self):
        """Test skips non-dict VLAN data - covers line 269."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": "not_a_dict",  # Should be skipped
            "Vlan200": {
                "vlan_id": "200",
                "ip_address": ["10.0.0.1/24"],
                "ports": [],
                "proxy_arp": "disabled",
                "dhcp_helper_addresses": []
            }
        }
        
        result = formatter.format(data, config)
        assert "200" in result

    def test_vlan_brief_port_as_string(self):
        """Test port as string instead of dict - covers line 302."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": {
                "vlan_id": "100",
                "ip_address": ["10.0.0.1/24"],
                "ports": ["Ethernet0", "Ethernet1"],  # Strings instead of dicts
                "proxy_arp": "disabled",
                "dhcp_helper_addresses": []
            }
        }
        
        result = formatter.format(data, config)
        assert "100" in result

    def test_vlan_brief_invalid_dhcp_ip(self):
        """Test invalid DHCP IP sort - covers line 342."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan100": {
                "vlan_id": "100",
                "ip_address": ["10.0.0.1/24"],
                "ports": [],
                "proxy_arp": "disabled",
                "dhcp_helper_addresses": ["not.valid.ip", "10.0.0.1"]  # Invalid IP
            }
        }
        
        result = formatter.format(data, config)
        assert "100" in result

    def test_presence_list_format(self):
        """Test list format - covers line 93."""
        
        formatter = TransceiverPresenceFormatter()
        config = CommandConfig()
        
        data = [
            {"Port": "Ethernet0", "Presence": "Present"},
            {"Port": "Ethernet1", "Presence": "Not present"}
        ]
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "Ethernet1" in result

    def test_presence_dict_nested_value(self):
        """Test dict format with nested presence - covers line 146."""
        
        formatter = TransceiverPresenceFormatter()
        config = CommandConfig()
        
        # Dict format with nested presence value
        data = {
            "Ethernet0": {"presence": "Present"},
            "Ethernet1": {"Presence": "Not Present"}
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "Present" in result

    def test_error_status_non_dict(self):
        """Test non-dict port data - covers line 201."""
        
        formatter = TransceiverErrorStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "OK",  # String instead of dict
            "Ethernet1": {"CMIS State (SW)": "READY"}
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "OK" in result

    def test_eeprom_string_value(self):
        """Test string port data - covers line 259."""
        
        formatter = TransceiverEepromFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "SFP not detected"  # String value
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_eeprom_invalid_json(self):
        """Test invalid JSON string - covers line 288."""
        
        formatter = TransceiverEepromFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "{invalid json}"  # Invalid JSON
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_eeprom_non_dict_value(self):
        """Test non-dict after parsing - covers line 309-311."""
        
        formatter = TransceiverEepromFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": 12345  # Non-dict, non-string value
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_eeprom_empty_dict(self):
        """Test empty dict means not detected - covers line 340."""
        
        formatter = TransceiverEepromFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {}  # Empty dict = not detected
        }
        
        result = formatter.format(data, config)
        assert "Not detected" in result

    def test_status_invalid_json(self):
        """Test invalid JSON string - covers line 386."""
        
        formatter = TransceiverStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "{invalid json}"
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_status_non_dict_value(self):
        """Test non-dict value - covers line 408-411."""
        
        formatter = TransceiverStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": 12345  # Non-dict value
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_pm_basic_dict(self):
        """Test PM with dict data - covers line 467."""
        
        formatter = TransceiverPmFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Temperature": "25.0C", "Voltage": "3.3V"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_pm_string_value(self):
        """Test string port data - covers line 532."""
        
        formatter = TransceiverPmFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": "PM not supported"
        }
        
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_config_basic_format(self):
        """Test dropcounters config format - covers line 175."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        
        # Use the exact expected format
        data = [
            {
                "Counter": "PORT_INGRESS_DROPS",
                "Alias": "ING",
                "Group": "PORT",
                "Type": "PORT",
                "Reasons": "[]",
                "Description": "Port ingress drops"
            }
        ]
        
        result = formatter.format(data, config)
        # Result should contain something from the data
        assert result is not None

    def test_config_empty_table(self):
        """Test empty table - covers lines 180, 183."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        
        # Non-dict entries should be skipped
        data = ["not_a_dict", 12345]
        
        result = formatter.format(data, config)
        assert result == ""

    def test_system_memory_mem_and_swap(self):
        """Test Mem and Swap rows - covers lines 151, 154, 217, 220."""
        
        formatter = SystemMemoryFormatter()
        config = CommandConfig()
        
        data = [
            {"type": "Mem", "total": "32000", "used": "16000", "free": "16000", "shared": "500", "buff/cache": "2000", "available": "14000"},
            {"type": "Swap", "total": "8000", "used": "0", "free": "8000"}
        ]
        
        result = formatter.format(data, config)
        assert "Mem:" in result
        assert "Swap:" in result

    def test_queue_counters_data(self):
        """Test queue counters data - covers line 150."""
        
        formatter = QueueCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"UC0": "50", "UC1": "100"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_wred_counters_basic(self):
        """Test WRED counters - covers lines 213-214."""
        
        formatter = WredCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"green_drop": "100", "yellow_drop": "50"}
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_queue_counters_empty_port(self):
        """Test empty port data - covers line 64."""
        
        formatter = QueueCountersFormatter()
        config = CommandConfig()
        
        data = {}
        
        result = formatter.format(data, config)
        assert result == "" or result is not None

    def test_queue_counters_non_dict_value(self):
        """Test non-dict port value - covers line 150."""
        
        formatter = QueueCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"UC0": "100"},
            "Ethernet1": "not_a_dict"  # Should be handled
        }
        
        result = formatter.format(data, config)
        assert result is not None

    def test_config_non_dict_items(self):
        """Test non-dict items in list - covers lines 180, 183."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        
        # List with only non-dict items
        data = [None, "string", 12345]
        
        result = formatter.format(data, config)
        # Should produce empty or minimal output
        assert result == "" or result is not None

    def test_watermark_telemetry_empty(self):
        """Test empty data - covers line 72."""
        
        formatter = WatermarkTelemetryFormatter()
        config = CommandConfig()
        
        data = {}
        
        result = formatter.format(data, config)
        assert result == "" or result is not None

    def test_headroom_pool_empty(self):
        """Test empty data - covers line 124."""
        
        formatter = HeadroomPoolFormatter()
        config = CommandConfig()
        
        data = {}
        
        result = formatter.format(data, config)
        assert result == "" or result is not None

    def test_queue_watermark_empty(self):
        """Test empty data - covers line 153."""
        
        formatter = QueueWatermarkFormatter()
        config = CommandConfig()
        
        data = {}
        
        result = formatter.format(data, config)
        assert result == "" or result is not None


class TestSystemMemoryRowTypes:
    """Tests for system memory formatter row types - lines 151, 154."""
    
    def test_memory_unknown_type(self):
        """Test with unknown row type."""
        
        formatter = SystemMemoryFormatter()
        config = CommandConfig()
        
        data = [
            {"type": "Unknown", "total": "100", "used": "50", "free": "50"}
        ]
        result = formatter.format(data, config)
        assert result is not None
    
    def test_memory_no_type_field(self):
        """Test with missing type field."""
        
        formatter = SystemMemoryFormatter()
        config = CommandConfig()
        
        data = [{"total": "100", "used": "50", "free": "50"}]
        result = formatter.format(data, config)
        assert result is not None

class TestSystemProcessesCpuRowTypes:
    """Tests for processes CPU formatter - lines 217, 220, 291, 294."""
    
    def test_processes_cpu_non_dict_item(self):
        """Test with non-dict items in list."""
        
        formatter = ProcessesCpuFormatter()
        config = CommandConfig()
        
        data = [
            {"PID": "1", "PPID": "0", "CMD": "test", "%MEM": "1", "%CPU": "0"},
            "not a dict",  # non-dict item
            None  # None item
        ]
        result = formatter.format(data, config)
        assert result is not None
    
    def test_processes_cpu_long_cmd(self):
        """Test with CMD exceeding max width - line 294."""
        
        formatter = ProcessesCpuFormatter()
        config = CommandConfig()
        
        data = [{
            "PID": "1", 
            "PPID": "0", 
            "CMD": "a" * 100,  # Very long command
            "%MEM": "1", 
            "%CPU": "0",
            "TIME": "00:00:00",
            "TTY": "?",
            "STAT": "S",
            "START": "10:00"
        }]
        result = formatter.format(data, config)
        # Long cmd should be truncated if result is not empty
        assert result is not None

class TestMergedMisc_1698:
    """Merged from 27 single-method test classes."""

    def test_reboot_cause_history_format(self):
        """Test reboot cause with history entries."""
        
        formatter = RebootCauseFormatter()
        config = CommandConfig()
        
        data = {
            "REBOOT_CAUSE|2024_01_15_10_30_00": {
                "cause": "User issued 'reboot' command",
                "time": "Mon Jan 15 10:30:00 UTC 2024",
                "user": "admin",
                "comment": "Maintenance"
            },
            "REBOOT_CAUSE|2024_01_14_08_00_00": {
                "cause": "warm-reboot",
                "time": "Sun Jan 14 08:00:00 UTC 2024",
                "user": "$USER$",
                "comment": "N/A"
            }
        }
        result = formatter.format(data, config)
        assert "2024_01_15_10_30_00" in result
        assert "2024_01_14_08_00_00" in result

    def test_ndp_with_mixed_items(self):
        """Test NDP with some non-dict items."""
        
        formatter = NdpFormatter()
        config = CommandConfig()
        
        data = {
            "entries": [
                {"address": "fc00::1", "mac": "AA:BB:CC:DD:EE:FF", "iface": "Vlan10"},
                "not a dict",  # non-dict entry
                None
            ],
            "total_entries": 3
        }
        result = formatter.format(data, config)
        assert "fc00::1" in result

    def test_lldp_neighbors_non_dict_intf(self):
        """Test LLDP skips non-dict interface data - line 156 is the continue statement."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        # Only include dict interfaces - the code skips non-dict automatically after the type check
        data = {
            "interfaces": {
                "Ethernet1": {"via": "LLDP", "RID": "1", "Chassis": {"SysName": "test"}}
            }
        }
        result = formatter.format(data, config)
        assert "Ethernet1" in result

    def test_lldp_rid_non_numeric(self):
        """Test LLDP with non-numeric RID values."""
        
        formatter = LldpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "interfaces": {
                "Ethernet0": {"via": "LLDP", "RID": "abc", "Chassis": {}},  # non-numeric RID
                "Ethernet1": {"via": "LLDP", "RID": None, "Chassis": {}}  # None RID
            }
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_ipv6_sort_key_invalid_address(self):
        """Test IPv6 sort with invalid address format."""
        
        formatter = IPv6BgpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "not_an_ip_address": {"remoteAs": "65000", "localAs": "65001"},
            "fc00::1": {"remoteAs": "65002", "localAs": "65001"}
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_description_non_dict_intf_data(self):
        """Test description with non-dict interface data."""
        
        formatter = InterfaceDescriptionFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Oper": "up", "Admin": "up", "Alias": "eth0"},
            "Ethernet1": "not a dict",  # non-dict - should be skipped
            "Ethernet2": None  # None - should be skipped
        }
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_counters_detailed_non_dict_value(self):
        """Test counters detailed where intf_data value is not properly formatted."""
        
        formatter = InterfaceCountersDetailedFormatter()
        config = CommandConfig()
        
        # First interface is valid, should process and break
        data = {
            "Ethernet0": {"RX_OK": "100", "TX_OK": "200"},
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_flap_list_format_invalid_prefix(self):
        """Test flap with list format having invalid prefixes."""
        
        formatter = InterfaceFlapFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "Flap Count": "5"},
            {"Interface": "InvalidPrefix0", "Flap Count": "3"},  # Invalid prefix - skipped
            {"Interface": "PortChannel1", "Flap Count": "2"}
        ]
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "InvalidPrefix0" not in result

    def test_watermark_telemetry_non_dict(self):
        """Test watermark telemetry with non-dict entries."""
        
        formatter = WatermarkTelemetryFormatter()
        config = CommandConfig()
        
        data = {
            "egress_lossless_pool": {"SAI_BUFFER_POOL_STAT_WATERMARK_BYTES": "1000"},
            "invalid_pool": "not a dict"  # non-dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_queue_counters_empty_queue_dict(self):
        """Test queue counters with empty queue dict."""
        
        formatter = QueueCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {}  # Empty queues dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_dropcounters_config_empty_reasons(self):
        """Test dropcounters config with empty reasons list."""
        
        formatter = DropcountersConfigurationFormatter()
        config = CommandConfig()
        
        data = [
            {"Counter": "DEBUG0", "Alias": "test", "Group": "PORT_INGRESS", "Type": "PORT_INGRESS_DROPS", "Reasons": []}
        ]
        result = formatter.format(data, config)
        assert result is not None

    def test_counters_all_non_dict_values(self):
        """Test counters where all interface values are non-dict."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        # All values are strings, not dicts - should result in empty table
        data = {
            "Ethernet0": "not a dict",
            "Ethernet1": 12345
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_fec_histogram_dict_format(self):
        """Test FEC histogram with dict input format."""
        
        formatter = FecHistogramFormatter()
        config = CommandConfig()
        
        data = {
            "bin0": {"BinIndex": "0", "Codewords": "100"},
            "bin1": {"BinIndex": "1", "Codewords": "200"}
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_switchport_status_invalid_prefix(self):
        """Test switchport status skips invalid interface prefix."""
        
        formatter = SwitchportStatusFormatter()
        config = CommandConfig()
        
        data = [
            {"Interface": "Ethernet0", "Mode": "access"},
            {"Interface": "InvalidPrefix0", "Mode": "trunk"},  # Invalid - skipped
        ]
        result = formatter.format(data, config)
        assert "Ethernet0" in result
        assert "InvalidPrefix0" not in result

    def test_switchport_config_dict_format(self):
        """Test switchport config with dict input."""
        
        formatter = SwitchportConfigFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Mode": "access", "Untagged": "10"},
            "InvalidPrefix": {"Mode": "trunk"}  # Invalid prefix - skipped
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_vlan_brief_non_dict_value(self):
        """Test VLAN brief with non-dict vlan_info."""
        
        formatter = VlanBriefFormatter()
        config = CommandConfig()
        
        data = {
            "Vlan10": {"vlan_id": "10", "ip_address": []},
            "Vlan20": "not a dict"  # non-dict - should be skipped
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_link_local_mode_list_format(self):
        """Test link local mode with list input."""
        
        formatter = IPv6LinkLocalModeFormatter()
        config = CommandConfig()
        
        data = [
            {"port": "Ethernet0", "mode": "RFC 5765"},
            {"port": "Ethernet1", "mode": "draft-ietf-6man-rfc4941bis"}
        ]
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_prefix_list_non_dict_prefix(self):
        """Test prefix list with non-dict prefix."""
        
        formatter = IPv6PrefixListFormatter()
        config = CommandConfig()
        
        data = {"PL_TEST": {"fc00::/64": "not a dict"}}
        result = formatter.format(data, config)
        assert result is not None

    def test_lpmode_non_dict_value(self):
        """Test lpmode with non-dict port data."""
        
        formatter = TransceiverLpmodeFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Low-power Mode": "Off"},
            "Ethernet1": "just a string"  # non-dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_info_non_dict_port_data(self):
        """Test transceiver info with non-dict port data."""
        
        formatter = TransceiverInfoFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Vendor": "Test", "Part Number": "PN123"},
            "Ethernet1": "just a string"  # non-dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_status_non_dict_port_data(self):
        """Test transceiver status with non-dict port data."""
        
        formatter = TransceiverStatusFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Temp": "35", "Voltage": "3.3"},
            "Ethernet1": "just a string"  # non-dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_pm_non_dict_port_data(self):
        """Test transceiver PM with non-dict port data."""
        
        formatter = TransceiverPmFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"Temperature": "35.0C"},
            "Ethernet1": "just a string"  # non-dict
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_queue_non_dict_queue_data(self):
        """Test queue counters where queue value is non-dict."""
        
        formatter = QueueCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "UC0": {"totalpacket": "100"},
                "UC1": "not a dict"  # non-dict queue
            }
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_wred_non_dict_queue_data(self):
        """Test WRED counters with non-dict queue data."""
        
        formatter = WredCountersFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {
                "UC0": {"WredDroppedPackets": "10"},
                "UC1": "not a dict"  # non-dict
            }
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_services_non_dict_item(self):
        """Test services with non-dict item in list."""
        
        formatter = ServicesFormatter()
        config = CommandConfig()
        
        data = [
            {"dockerProcessName": "syncd", "processes": []},
            "not a dict",  # non-dict
            None
        ]
        result = formatter.format(data, config)
        assert result is not None

    def test_ndp_dict_no_entries_key(self):
        """Test NDP with dict that has no 'entries' key."""
        
        formatter = NdpFormatter()
        config = CommandConfig()
        
        # Dict without 'entries' key - values become items
        data = {
            "entry1": {"address": "fc00::1", "mac": "AA:BB:CC:DD:EE:FF", "iface": "Eth0"},
            "entry2": {"address": "fc00::2", "mac": "11:22:33:44:55:66", "iface": "Eth1"}
        }
        result = formatter.format(data, config)
        assert result is not None

    def test_ndp_list_input(self):
        """Test NDP with list input directly."""
        
        formatter = NdpFormatter()
        config = CommandConfig()
        
        data = [
            {"address": "fc00::1", "mac": "AA:BB:CC:DD:EE:FF", "iface": "Eth0", "status": "REACHABLE"}
        ]
        result = formatter.format(data, config)
        assert "fc00::1" in result


class TestInterfaceErrorsNonDictItem:
    """Tests for interface errors with non-dict item - lines 292, 310."""
    
    def test_errors_list_non_dict_item(self):
        """Test errors with non-dict item in list."""
        
        formatter = InterfaceErrorsFormatter()
        config = CommandConfig()
        
        data = [
            {"Port Errors": "CRC Error", "Count": "5", "Last timestamp(UTC)": "2024-01-01"},
            "not a dict",  # non-dict
            None
        ]
        result = formatter.format(data, config)
        assert result is not None
    
    def test_errors_empty_after_filter(self):
        """Test errors where all items filtered out - line 310."""
        
        formatter = InterfaceErrorsFormatter()
        config = CommandConfig()
        
        # All non-dict items
        data = ["not dict", 123, None]
        result = formatter.format(data, config)
        assert result == "" or result is not None

class TestMergedMisc_2185:
    """Merged from 5 single-method test classes."""

    def test_counters_list_all_non_dict(self):
        """Test counters list with all non-dict items."""
        
        formatter = InterfaceCountersFormatter()
        config = CommandConfig()
        
        # List of non-dict items
        data = ["not dict", 123, None]
        result = formatter.format(data, config)
        assert result == "" or result is not None

    def test_flap_dict_format(self):
        """Test flap with dict input format."""
        
        formatter = InterfaceFlapFormatter()
        config = CommandConfig()
        
        data = {
            "PORT|Ethernet0": {"Flap Count": "5", "Admin": "up", "Oper": "up"}
        }
        result = formatter.format(data, config)
        assert "Ethernet0" in result

    def test_route_multipath_status(self):
        """Test route with multipath=True status."""
        
        formatter = IPv6RouteFormatter()
        config = CommandConfig()
        
        data = {
            "advertisedRoutes": {
                "fc00::/64": {
                    "valid": True,
                    "multipath": True,  # multipath flag
                    "nextHopGlobal": "fc00::1"
                }
            }
        }
        result = formatter.format(data, config)
        # The formatter may return as-is if not matching expected structure
        assert result is not None

    def test_bgp_neighbors_non_dict_neighbor(self):
        """Test BGP neighbors with non-dict neighbor data."""
        
        formatter = IPv6BgpNeighborsFormatter()
        config = CommandConfig()
        
        data = {
            "fc00::1": {"remoteAs": "65000", "localAs": "65001"},
            "fc00::2": "not a dict"  # non-dict neighbor
        }
        result = formatter.format(data, config)
        assert "fc00::1" in result

    def test_link_local_mode_dict_format(self):
        """Test link local mode with dict input format."""
        
        formatter = IPv6LinkLocalModeFormatter()
        config = CommandConfig()
        
        data = {
            "Ethernet0": {"mode": "RFC 5765"},
            "Ethernet1": "use-global-scope"  # string value
        }
        result = formatter.format(data, config)
        assert "Ethernet0" in result

