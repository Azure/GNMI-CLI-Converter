"""Auto-generated from _legacy_tests.py — Import validation and data model tests"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from gnmi_cli_lib import (
    CLI_TO_GNMI_MAPPING,
    CliMapping,
    CommandConfig,
    CommandRegistry,
    GnmiToCliConverter,
    get_default_registry,
)
from gnmi_cli_lib.common.constants import (
    INTERFACE_COLUMN_ORDER,
    QUEUE_COLUMN_ORDER,
    SAI_STAT_FIELDS,
    VERSION_FIELD_ORDER,
)
from gnmi_cli_lib.common.types import CliMapping, CommandConfig
from gnmi_cli_lib.convenience import (
    convert,
    convert_buffer_pool_watermark,
    convert_clock,
    convert_clock_timezones,
    convert_dropcounters_capabilities,
    convert_dropcounters_configuration,
    convert_dropcounters_counts,
    convert_headroom_pool_watermark,
    convert_interface_counters,
    convert_interface_description,
    convert_interface_errors,
    convert_interface_status,
    convert_lldp_neighbors,
    convert_lldp_table,
    convert_mac,
    convert_mmu,
    convert_ndp,
    convert_portchannel,
    convert_priority_group_watermark,
    convert_processes,
    convert_queue_counters,
    convert_queue_watermark,
    convert_queue_wredcounters,
    convert_reboot_cause,
    convert_services,
    convert_srv6_stats,
    convert_system_memory,
    convert_uptime,
    convert_version,
    convert_vlan_brief,
    convert_watermark_telemetry,
)
from gnmi_cli_lib.formatters.base import ListFormatter, TableFormatter, TextFormatter
from gnmi_cli_lib.utils.parsing import parse_json_data, parse_reasons_string
from gnmi_cli_lib.utils.sorting import natural_sort_key
from tests.fixtures.sample_data import EXPECTED_CLI_OUTPUTS, GNMI_JSON_TEST_DATA, get_test_data
from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.formatters.dropcounters import DropcountersCapabilitiesFormatter
from gnmi_cli_lib.formatters.interface import InterfaceStatusFormatter
from gnmi_cli_lib.formatters.network import LldpTableFormatter
from gnmi_cli_lib.formatters.queue import QueueCountersFormatter
from gnmi_cli_lib.formatters.system import VersionFormatter


class TestImports:
    """Test that all library components can be imported correctly."""
    
    def test_core_classes(self):
        """Test import of core classes."""
        assert GnmiToCliConverter is not None
        assert CommandRegistry is not None
    
    def test_types(self):
        """Test import of data types."""
        assert CommandConfig is not None
        assert CliMapping is not None
    
    def test_convenience_functions(self):
        """Test import of convenience functions."""
        # All imports should succeed
        assert convert is not None
    
    def test_mappings(self):
        """Test import of CLI_TO_GNMI_MAPPING."""
        assert isinstance(CLI_TO_GNMI_MAPPING, dict)
        # 70+ CLI mappings using show command format
        assert len(CLI_TO_GNMI_MAPPING) >= 60
    
    def test_test_data(self):
        """Test import of test data."""
        assert isinstance(GNMI_JSON_TEST_DATA, dict)
        assert isinstance(EXPECTED_CLI_OUTPUTS, dict)
    
    def test_registry_factory(self):
        """Test import of registry factory function."""
        registry = get_default_registry()
        assert registry is not None
    
    def test_formatters_base(self):
        """Test import of base formatters."""
        assert OutputFormatter is not None
    
    def test_formatters_interface(self):
        """Test import of interface formatters."""
        assert InterfaceStatusFormatter is not None
    
    def test_formatters_system(self):
        """Test import of system formatters."""
        assert VersionFormatter is not None
    
    def test_formatters_network(self):
        """Test import of network formatters."""
        assert LldpTableFormatter is not None
    
    def test_formatters_queue(self):
        """Test import of queue formatters."""
        assert QueueCountersFormatter is not None
    
    def test_formatters_dropcounters(self):
        """Test import of dropcounters formatters."""
        assert DropcountersCapabilitiesFormatter is not None
    
    def test_utils(self):
        """Test import of utility functions."""
        assert natural_sort_key is not None


# =============================================================================
# Test Data - Centralized test data for all tests
# =============================================================================

class TestData:
    """Centralized test data for consistent testing across all test classes."""
    
    # Buffer Pool Test Data
    BUFFER_POOL_SIMPLE = {
        "egress_lossless_pool": {"Bytes": "1000"},
        "egress_lossy_pool": {"Bytes": "2000"},
        "ingress_lossless_pool": {"Bytes": "3000"},
    }
    
    BUFFER_POOL_SINGLE = {
        "ingress_lossless_pool": {"Bytes": "5000"},
    }
    
    # Headroom Pool Test Data
    HEADROOM_POOL = {
        "ingress_lossless_pool": {"Bytes": "432640"},
    }
    
    # Priority Group Test Data
    PRIORITY_GROUP = {
        "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102"},
        "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402"},
        "Ethernet8": {"PG0": "800", "PG1": "801", "PG2": "802"},
    }
    
    # Queue Test Data
    QUEUE_UNICAST = {
        "Ethernet0": {"UC0": "2057328", "UC1": "2056704", "UC2": "0"},
        "Ethernet4": {"UC0": "0", "UC1": "0", "UC2": "0"},
    }
    
    QUEUE_MULTICAST = {
        "Ethernet0": {"MC10": "N/A", "MC11": "N/A"},
        "Ethernet4": {"MC10": "N/A", "MC11": "N/A"},
    }
    
    QUEUE_ALL = {
        "Ethernet0": {"ALL20": "1234567", "ALL21": "7654321"},
        "Ethernet4": {"ALL20": "0", "ALL21": "0"},
    }
    
    # Clock Test Data
    CLOCK_DATE = {"date": "Mon Mar 25 20:25:16 UTC 2019"}
    CLOCK_DATE_ZERO_EPOCH = {"date": "Thu Jan  1 00:00:00 UTC 1970"}
    CLOCK_DATE_NORMAL = {"date": "Fri Jul 18 18:00:00 UTC 2025"}
    
    CLOCK_TIMEZONES = {
        "timezones": [
            "America/Anchorage",
            "America/New_York",
            "Asia/Tokyo",
            "UTC",
            "Universal"
        ]
    }
    
    # Dropcounters Test Data
    DROPCOUNTERS_CAPABILITIES = {
        "PORT_INGRESS_DROPS": {
            "count": "10",
            "reasons": "[MPLS_MISS,IP_HEADER_ERROR,ACL_ANY]"
        },
        "SWITCH_EGRESS_DROPS": {
            "count": "2",
            "reasons": "[ACL_ANY,L2_ANY]"
        }
    }
    
    DROPCOUNTERS_CONFIGURATION = [
        {
            "name": "DEBUG_0",
            "alias": "DEBUG_0",
            "group": "N/A",
            "type": "PORT_INGRESS_DROPS",
            "reason": "None",
            "description": "N/A"
        },
        {
            "name": "DEBUG_1",
            "alias": "SWITCH_DROPS",
            "group": "PACKET_DROPS",
            "type": "SWITCH_EGRESS_DROPS",
            "reason": "None",
            "description": "Outgoing packet drops"
        }
    ]
    
    DROPCOUNTERS_COUNTS = {
        "Ethernet0": {
            "State": "D",
            "RX_ERR": "10",
            "RX_DROPS": "100",
            "TX_ERR": "0",
            "TX_DROPS": "0",
            "DEBUG_0": "80",
            "DEBUG_2": "20"
        },
        "Ethernet4": {
            "State": "N/A",
            "RX_ERR": "0",
            "RX_DROPS": "1000",
            "TX_ERR": "0",
            "TX_DROPS": "0",
            "DEBUG_0": "800",
            "DEBUG_2": "100"
        }
    }
    
    # Version Test Data
    VERSION_DATA = {
        "sonic_software_version": "SONiC.test.1-abc123",
        "sonic_os_version": "11",
        "distribution": "Debian 11.4",
        "kernel": "5.10.0-18-2-amd64",
        "build_commit": "abc123",
        "build_date": "2025-07-18",
        "built_by": "test_user",
        "platform": "x86_64-test-r0",
        "hwsku": "Test-SKU",
        "asic": "test_asic",
        "asic_count": "1",
        "serial_number": "SN123456",
        "uptime": "07:42:51 up 16 days",
        "date": "Fri 18 Jul 2025 18:00:00"
    }
    
    # System Memory Test Data
    SYSTEM_MEMORY = [
        {"type": "Mem", "total": "64252", "used": "15017", "free": "7526",
         "shared": "563", "buff/cache": "41708", "available": "46228"},
        {"type": "Swap", "total": "0", "used": "0", "free": "0"}
    ]
    
    # Uptime Test Data
    UPTIME_DATA = {"uptime": "up 3 weeks, 4 days, 10 hours, 15 minutes"}
    
    # Interface Test Data
    INTERFACE_STATUS = {
        "PORT_TABLE:Ethernet0": {
            "admin_status": "up", "oper_status": "up",
            "speed": "100000", "mtu": "9100",
            "alias": "etp0", "description": "Uplink"
        },
        "PORT_TABLE:Ethernet4": {
            "admin_status": "up", "oper_status": "down",
            "speed": "25000", "mtu": "9100",
            "alias": "etp1", "description": "Server1"
        }
    }
    
    INTERFACE_DESCRIPTION = {
        "Ethernet0": {"Admin": "up", "Alias": "etp0",
                      "Description": "ARISTA01T1:Ethernet1", "Oper": "up"},
        "Ethernet4": {"Admin": "up", "Alias": "etp1",
                      "Description": "Server1:eth0", "Oper": "down"}
    }
    
    # LLDP Test Data
    LLDP_TABLE = {
        "capability_codes_helper": "Capability codes: (R) Router, (B) Bridge",
        "neighbors": [
            {"localPort": "Ethernet0", "remoteDevice": "switch1",
             "remotePortId": "Ethernet48", "capability": "BR",
             "remotePortDescr": "Uplink"},
            {"localPort": "Ethernet4", "remoteDevice": "switch2",
             "remotePortId": "Ethernet52", "capability": "R",
             "remotePortDescr": "Downlink"}
        ],
        "total": 2
    }
    
    # VLAN Test Data
    VLAN_BRIEF = {
        "Vlan1000": {
            "vlan_id": "1000",
            "ip_address": ["192.168.0.1/24"],
            "ports": [{"name": "Ethernet0", "port_tagging": "untagged"}],
            "proxy_arp": "disabled",
            "dhcp_helper_addresses": ["192.0.0.1"]
        }
    }
    
    # NDP Test Data
    NDP_DATA = [
        {"address": "fc00::176", "interface": "Ethernet60",
         "mac": "aa:f9:f4:cb:d2:3f", "state": "REACHABLE"},
        {"address": "fe80::1", "interface": "Vlan1000",
         "mac": "b8:ce:f6:e5:50:0d", "state": "STALE"}
    ]
    
    # MAC Test Data
    MAC_DATA = [
        {"No.": "1", "vlan": "1000", "macAddress": "aa:bb:cc:dd:ee:ff",
         "port": "Ethernet0", "type": "Dynamic"},
        {"No.": "2", "vlan": "1000", "macAddress": "11:22:33:44:55:66",
         "port": "Ethernet4", "type": "Static"}
    ]
    
    # Reboot Cause Test Data
    REBOOT_CAUSE_CURRENT = {
        "cause": "reboot",
        "time": "Thu Jul 10 08:05:33 PM UTC 2025",
        "user": "admin"
    }
    
    REBOOT_CAUSE_HISTORY = {
        "REBOOT_CAUSE|2025_07_10_20_06_34": {
            "cause": "reboot",
            "comment": "N/A",
            "time": "Thu Jul 10 08:05:33 PM UTC 2025",
            "user": "admin"
        }
    }
    
    # Multi-ASIC Test Data
    MULTI_ASIC_DATA = {
        "asic0": {"ingress_lossless_pool": {"Bytes": "N/A"}},
        "asic1": {"ingress_lossless_pool": {"Bytes": "3000"}},
    }


# =============================================================================
# Test Data Module Tests
# =============================================================================

class TestTestDataModule:
    """Tests for the test_data module."""
    
    def test_gnmi_json_test_data_exists(self):
        """Test GNMI_JSON_TEST_DATA is available."""
        
        assert isinstance(GNMI_JSON_TEST_DATA, dict)
        assert "buffer_pool" in GNMI_JSON_TEST_DATA
        assert "clock" in GNMI_JSON_TEST_DATA
    
    def test_expected_cli_outputs_exists(self):
        """Test EXPECTED_CLI_OUTPUTS is available."""
        
        assert isinstance(EXPECTED_CLI_OUTPUTS, dict)
        assert "buffer_pool" in EXPECTED_CLI_OUTPUTS
        assert "clock" in EXPECTED_CLI_OUTPUTS
    
    def test_get_test_data_function(self):
        """Test get_test_data helper function."""
        
        # gNMI data
        gnmi_data = get_test_data("buffer_pool", "watermark", "gnmi")
        assert isinstance(gnmi_data, dict)
        
        # CLI data
        cli_data = get_test_data("buffer_pool", "watermark", "cli")
        assert isinstance(cli_data, str)
    
    def test_get_test_data_invalid_type(self):
        """Test get_test_data with invalid type."""
        
        with pytest.raises(ValueError) as excinfo:
            get_test_data("buffer_pool", "watermark", "invalid")
        assert "Unknown data_type" in str(excinfo.value)


# =============================================================================
# Common Types Tests
# =============================================================================

class TestCommonTypes:
    """Tests for common types (CommandConfig, CliMapping)."""
    
    def test_command_config_creation(self):
        """Test CommandConfig dataclass creation."""
        
        config = CommandConfig(
            title="Test Title:",
            output_type="table",
            headers=["Col1", "Col2"],
            key_field="name",
            value_fields=["value"],
            dynamic_columns=True,
            alignment="right",
            description="Test description"
        )
        
        assert config.title == "Test Title:"
        assert config.output_type == "table"
        assert config.headers == ["Col1", "Col2"]
        assert config.dynamic_columns is True
    
    def test_command_config_defaults(self):
        """Test CommandConfig default values."""
        
        config = CommandConfig()
        
        assert config.title == ""
        assert config.output_type == "table"
        assert config.headers == []
        assert config.dynamic_columns is False
    
    def test_cli_mapping_creation(self):
        """Test CliMapping dataclass creation."""
        
        mapping = CliMapping(
            cli_command="show test",
            gnmi_path="SHOW/test",
            command="test",
            subcommand="test",
            description="Test mapping",
            out_of_process_cmd="test_cmd"
        )
        
        assert mapping.cli_command == "show test"
        assert mapping.gnmi_path == "SHOW/test"
        assert mapping.out_of_process_cmd == "test_cmd"
    
    def test_cli_mapping_defaults(self):
        """Test CliMapping default values."""
        
        mapping = CliMapping(
            cli_command="show test",
            gnmi_path="SHOW/test",
            command="test",
            subcommand="test"
        )
        
        assert mapping.description == ""
        assert mapping.out_of_process_cmd == ""


# =============================================================================
# Constants Tests
# =============================================================================

class TestConstants:
    """Tests for constants module."""
    
    def test_queue_column_order(self):
        """Test QUEUE_COLUMN_ORDER constant."""
        
        assert isinstance(QUEUE_COLUMN_ORDER, list)
        # QUEUE_COLUMN_ORDER contains counter columns, not Port
        assert "Counter/pkts" in QUEUE_COLUMN_ORDER
    
    def test_interface_column_order(self):
        """Test INTERFACE_COLUMN_ORDER constant."""
        
        assert isinstance(INTERFACE_COLUMN_ORDER, list)
    
    def test_version_field_order(self):
        """Test VERSION_FIELD_ORDER constant."""
        
        assert isinstance(VERSION_FIELD_ORDER, list)
        # VERSION_FIELD_ORDER is a list of tuples (key, display_name)
        assert len(VERSION_FIELD_ORDER) > 0
        first_item = VERSION_FIELD_ORDER[0]
        assert isinstance(first_item, tuple)
        assert first_item[0] == "sonic_software_version"
    
    def test_sai_stat_fields(self):
        """Test SAI_STAT_FIELDS constant."""
        
        assert isinstance(SAI_STAT_FIELDS, dict)
        assert "buffer_pool" in SAI_STAT_FIELDS

class TestCommonTypesCoverage:
    """Tests for common/types.py to boost coverage from 85%."""
    
    def test_command_config_all_fields(self):
        """Test CommandConfig with all fields."""
        
        config = CommandConfig(
            title="Test Title",
            output_type="table",
            headers=["Col1", "Col2", "Col3"],
            key_field="key",
            value_fields=["value1", "value2"],
            dynamic_columns=True,
            alignment="left",
            description="Test description"
        )
        
        assert config.title == "Test Title"
        assert config.dynamic_columns is True
        assert len(config.headers) == 3

class TestCommonTypesMaxCoverage:
    """Tests for common types module."""
    
    def test_command_config_with_all_params(self):
        """Test CommandConfig with all parameters."""
        
        config = CommandConfig(
            title="Test Title",
            headers=["Name", "Value"],
            output_type="table",
            key_field="name",
            value_fields=["Bytes"],
            dynamic_columns=True,
            alignment="left",
            description="Test description"
        )
        assert config.headers == ["Name", "Value"]
        assert config.output_type == "table"
    
    def test_cli_mapping(self):
        """Test CliMapping class."""
        
        mapping = CliMapping(
            cli_command="show interfaces status",
            gnmi_path="SHOW/interfaces/status",
            command="interfaces",
            subcommand="status"
        )
        assert mapping.cli_command == "show interfaces status"

class TestCommonTypesValidation:
    """Test validation in common types."""
    
    def test_command_config_invalid_alignment(self):
        """Test CommandConfig with invalid alignment raises error."""
        
        with pytest.raises(ValueError) as excinfo:
            CommandConfig(alignment="center")  # Invalid alignment
        assert "alignment must be one of" in str(excinfo.value)
    
    def test_cli_mapping_empty_command(self):
        """Test CliMapping with empty command raises error."""
        
        with pytest.raises(ValueError) as excinfo:
            CliMapping(
                cli_command="show test",
                gnmi_path="SHOW/test",
                command="",  # Empty command
                subcommand="test"
            )
        assert "command cannot be empty" in str(excinfo.value)
    
    def test_cli_mapping_empty_subcommand(self):
        """Test CliMapping with empty subcommand raises error."""
        
        with pytest.raises(ValueError) as excinfo:
            CliMapping(
                cli_command="show test",
                gnmi_path="SHOW/test",
                command="test",
                subcommand=""  # Empty subcommand
            )
        assert "subcommand cannot be empty" in str(excinfo.value)
    
    def test_cli_mapping_empty_cli_command(self):
        """Test CliMapping with empty cli_command raises error."""
        
        with pytest.raises(ValueError) as excinfo:
            CliMapping(
                cli_command="",  # Empty cli_command
                gnmi_path="SHOW/test",
                command="test",
                subcommand="test"
            )
        assert "cli_command cannot be empty" in str(excinfo.value)
    
    def test_cli_mapping_to_dict(self):
        """Test CliMapping.to_dict method."""
        
        mapping = CliMapping(
            cli_command="show interfaces status",
            gnmi_path="SHOW/interfaces/status",
            command="interfaces",
            subcommand="status",
            description="Interface status info",
            out_of_process_cmd="show interfaces status"
        )
        
        d = mapping.to_dict()
        assert d["cli_command"] == "show interfaces status"
        assert d["gnmi_path"] == "SHOW/interfaces/status"
        assert d["command"] == "interfaces"
        assert d["subcommand"] == "status"
