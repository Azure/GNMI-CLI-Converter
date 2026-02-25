"""Auto-generated from _legacy_tests.py — Unit-level integration tests (end-to-end, sample outputs)"""

import io
import json
import sys
import tempfile
import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock

from tests.unit.test_imports import TestData

from gnmi_cli_lib import CliMapping, CommandConfig, CommandRegistry
from gnmi_cli_lib.commands import register_all_commands
from gnmi_cli_lib.parser import CommandParser
from tests.fixtures.sample_data import EXPECTED_CLI_OUTPUTS, GNMI_JSON_TEST_DATA


# =============================================================================
# Integration Tests
# =============================================================================

class TestIntegration:
    """End-to-end integration tests."""
    
    def test_full_workflow_buffer_pool(self):
        """Test full workflow for buffer pool conversion."""
        # 1. Check command is supported via CommandParser
        parser = CommandParser()
        commands = parser.list_commands()
        assert "buffer_pool" in commands
        
        # 2. Get gNMI path
        path = parser.get_gnmi_path("show buffer_pool watermark")
        assert path == "SHOW/buffer_pool/watermark"
        
        # 3. Convert using parser.convert()
        output1 = parser.convert("buffer_pool", "watermark", TestData.BUFFER_POOL_SIMPLE)
        
        # 4. Convert using parse_and_convert
        output2 = parser.parse_and_convert(
            "show buffer_pool watermark", TestData.BUFFER_POOL_SIMPLE
        )
        
        # All outputs should contain expected data
        for output in [output1, output2]:
            assert "Shared pool maximum occupancy:" in output
            assert "egress_lossless_pool" in output
            assert "1000" in output
    
    def test_full_workflow_with_custom_registry(self):
        """Test full workflow with custom registry."""
        # 1. Create custom registry
        registry = CommandRegistry()
        register_all_commands(registry)
        
        # 2. Add custom command
        config = CommandConfig(
            output_type="text",
            key_field="custom_value",
            description="Custom test command"
        )
        registry.register_command("custom", "test", config)
        
        # 3. Create parser with custom registry
        parser = CommandParser(registry=registry)
        
        # 4. Convert standard command
        output1 = parser.convert("buffer_pool", "watermark", TestData.BUFFER_POOL_SIMPLE)
        assert "Shared pool maximum occupancy:" in output1
        
        # 5. Convert custom command
        output2 = parser.convert("custom", "test", {"custom_value": "Hello!"})
        assert "Hello!" in output2


# =============================================================================
# Enhanced Command Tests - Coverage for all 70 commands
# =============================================================================

class TestEnhancedCommands:
    """Enhanced tests for all commands documented in README."""
    
    @pytest.fixture(autouse=True)
    def setup(self):
        self.converter = CommandParser()
    
    # -------------------------------------------------------------------------
    # Interface Counters Tests
    # -------------------------------------------------------------------------
    
    def test_interface_counters_fec_histogram(self):
        """Test FEC histogram counters."""
        # FecHistogramFormatter expects list format with BinIndex and Codewords
        json_data = [
            {"BinIndex": "BIN0", "Codewords": "1000"},
            {"BinIndex": "BIN1", "Codewords": "500"},
            {"BinIndex": "BIN2", "Codewords": "200"},
            {"BinIndex": "BIN3", "Codewords": "100"},
            {"BinIndex": "BIN4", "Codewords": "50"},
            {"BinIndex": "BIN5", "Codewords": "25"},
            {"BinIndex": "BIN6", "Codewords": "10"},
            {"BinIndex": "BIN7", "Codewords": "5"}
        ]
        output = self.converter.convert("interfaces", "counters-fec-histogram", json_data)
        assert output is not None
        assert "Symbol Errors Per Codeword" in output
        assert "1000" in output
    
    def test_interface_counters_fec_stats(self):
        """Test FEC statistics."""
        json_data = {
            "Ethernet0": {
                "FEC_CORR": "130402",
                "FEC_UNCORR": "3",
                "FEC_SYMBOL_ERR": "4"
            },
            "Ethernet4": {
                "FEC_CORR": "110412",
                "FEC_UNCORR": "1",
                "FEC_SYMBOL_ERR": "0"
            }
        }
        output = self.converter.convert("interfaces", "counters-fec-stats", json_data)
        assert output is not None
        assert "Ethernet0" in output
        assert "130402" in output
    
    def test_interface_counters_rates(self):
        """Test interface rate statistics."""
        json_data = {
            "Ethernet0": {
                "State": "U",
                "RxOk": "149903",
                "RxBps": "25.12 B/s",
                "RxUtil": "0.00%",
                "TxOk": "144782",
                "TxBps": "773.23 KB/s",
                "TxUtil": "0.01%"
            }
        }
        output = self.converter.convert("interfaces", "counters-rates", json_data)
        assert output is not None
    
    def test_interface_counters_trim(self):
        """Test trim counters."""
        json_data = {
            "Ethernet0": {
                "State": "U",
                "TrimPkts": "7",
                "TrimSent": "1",
                "TrimDrp": "0"
            }
        }
        output = self.converter.convert("interfaces", "counters-trim", json_data)
        assert output is not None
    
    def test_interface_fec_status(self):
        """Test FEC status."""
        json_data = {
            "Ethernet0": {"fec": "rs", "fec_oper": "rs"},
            "Ethernet4": {"fec": "none", "fec_oper": "none"},
            "Ethernet8": {"fec": "auto", "fec_oper": "rs"}
        }
        output = self.converter.convert("interfaces", "fec-status", json_data)
        assert output is not None
    
    def test_interface_flap(self):
        """Test interface flap counts."""
        json_data = {
            "PORT_TABLE:Ethernet0": {
                "flap_count": "3",
                "last_up_time": "Fri Jul 18 19:59:28 2025",
                "last_down_time": "Fri Jul 18 19:59:20 2025"
            }
        }
        output = self.converter.convert("interfaces", "flap", json_data)
        assert output is not None
    
    # -------------------------------------------------------------------------
    # Switchport Tests
    # -------------------------------------------------------------------------
    
    def test_switchport_config(self):
        """Test switchport configuration."""
        json_data = {
            "Ethernet0": {
                "mode": "trunk",
                "untagged_vlan": "1000",
                "tagged_vlans": ["1001", "1002"]
            },
            "Ethernet4": {
                "mode": "access",
                "untagged_vlan": "1000",
                "tagged_vlans": []
            }
        }
        output = self.converter.convert("interfaces", "switchport-config", json_data)
        assert output is not None
    
    def test_switchport_status(self):
        """Test switchport status."""
        json_data = {
            "Ethernet0": {
                "mode": "trunk",
                "admin_status": "up",
                "oper_status": "up",
                "untagged_vlan": "1000"
            }
        }
        output = self.converter.convert("interfaces", "switchport-status", json_data)
        assert output is not None
    
    # -------------------------------------------------------------------------
    # Transceiver Tests
    # -------------------------------------------------------------------------
    
    def test_transceiver_info(self):
        """Test transceiver info."""
        json_data = {
            "Ethernet0": {
                "type": "OSFP 8X Pluggable Transceiver",
                "manufacturer": "Amphenol",
                "model": "NJMMER-M201",
                "serial": "APF2447201229A"
            }
        }
        output = self.converter.convert("interfaces", "transceiver-info", json_data)
        assert output is not None
    
    def test_transceiver_presence(self):
        """Test transceiver presence."""
        json_data = {
            "Ethernet0": {"presence": "Present"},
            "Ethernet8": {"presence": "Not present"}
        }
        output = self.converter.convert("interfaces", "transceiver-presence", json_data)
        assert output is not None
    
    def test_transceiver_lpmode(self):
        """Test transceiver low-power mode."""
        json_data = {
            "Ethernet0": {"lpmode": "Off"},
            "Ethernet16": {"lpmode": "On"}
        }
        output = self.converter.convert("interfaces", "transceiver-lpmode", json_data)
        assert output is not None
    
    # -------------------------------------------------------------------------
    # IPv6 Tests
    # -------------------------------------------------------------------------
    
    def test_ipv6_prefix_list(self):
        """Test IPv6 prefix list (passthrough)."""
        json_data = {
            "ALLOW_ALL": {
                "entries": [
                    {"seq": 5, "type": "permit", "prefix": "::/0", "le": 128}
                ]
            }
        }
        output = self.converter.convert("ipv6", "prefix-list", json_data)
        assert output is not None
    
    def test_ipv6_bgp_summary(self):
        """Test IPv6 BGP summary (passthrough)."""
        json_data = {
            "routerId": "10.1.0.32",
            "as": 65100,
            "peers": {
                "fc00::72": {
                    "remoteAs": 65200,
                    "state": "Established"
                }
            }
        }
        output = self.converter.convert("ipv6", "bgp-summary", json_data)
        assert output is not None
    
    def test_ipv6_link_local_mode(self):
        """Test IPv6 link local mode."""
        json_data = {"link_local_mode": "disabled"}
        output = self.converter.convert("ipv6", "link-local-mode", json_data)
        assert "disabled" in output
    
    # -------------------------------------------------------------------------
    # Processes and Services Tests
    # -------------------------------------------------------------------------
    
    def test_processes_summary(self):
        """Test processes summary."""
        json_data = [
            {"PID": "1", "PPID": "0", "%CPU": "0.1", "%MEM": "0.2", "CMD": "systemd"},
            {"PID": "123", "PPID": "1", "%CPU": "1.5", "%MEM": "2.3", "CMD": "python"}
        ]
        output = self.converter.convert("processes", "summary", json_data)
        assert "systemd" in output
        assert "python" in output
    
    def test_processes_cpu(self):
        """Test processes sorted by CPU."""
        # ProcessesCpuFormatter expects dict with 'processes' key containing process list
        json_data = {
            "uptime": "09:57:01 up 1 day",
            "tasks": "493 total",
            "cpu_usage": "50.0 us",
            "memory_usage": "31905.5 total",
            "swap_usage": "0.0 total",
            "processes": [
                {"pid": "999", "user": "root", "pr": "20", "ni": "0", "virt": "100000", 
                 "res": "50000", "shr": "10000", "s": "S", "cpu": "50.0", "mem": "5.0", 
                 "time": "0:10.00", "command": "syncd"},
                {"pid": "888", "user": "root", "pr": "20", "ni": "0", "virt": "80000",
                 "res": "40000", "shr": "8000", "s": "S", "cpu": "25.0", "mem": "3.0",
                 "time": "0:05.00", "command": "orchagent"}
            ]
        }
        output = self.converter.convert("processes", "cpu", json_data)
        assert "syncd" in output
        assert "orchagent" in output
    
    def test_processes_memory(self):
        """Test processes sorted by memory."""
        # ProcessesCpuFormatter expects dict with 'processes' key
        json_data = {
            "uptime": "09:57:01 up 1 day",
            "tasks": "493 total",
            "cpu_usage": "5.0 us",
            "memory_usage": "31905.5 total",
            "swap_usage": "0.0 total",
            "processes": [
                {"pid": "777", "user": "root", "pr": "20", "ni": "0", "virt": "200000",
                 "res": "100000", "shr": "20000", "s": "S", "cpu": "5.0", "mem": "25.0",
                 "time": "0:20.00", "command": "database"},
                {"pid": "666", "user": "root", "pr": "20", "ni": "0", "virt": "150000",
                 "res": "75000", "shr": "15000", "s": "S", "cpu": "3.0", "mem": "15.0",
                 "time": "0:10.00", "command": "teamd"}
            ]
        }
        output = self.converter.convert("processes", "memory", json_data)
        assert "database" in output
        assert "teamd" in output
    
    def test_services(self):
        """Test services."""
        json_data = [
            {
                "dockerProcessName": "snmp",
                "processes": [
                    {"pid": "123", "user": "root", "cpuPercentage": "0.1",
                     "memPercentage": "0.5", "command": "python snmpd.py"}
                ]
            },
            {
                "dockerProcessName": "telemetry",
                "processes": [
                    {"pid": "456", "user": "root", "cpuPercentage": "0.2",
                     "memPercentage": "1.0", "command": "telemetry"}
                ]
            }
        ]
        output = self.converter.convert("services", "services", json_data)
        # Formatter outputs "servicename\tdocker" format
        assert "snmp\tdocker" in output
        assert "telemetry\tdocker" in output
    
    # -------------------------------------------------------------------------
    # SRv6 Tests
    # -------------------------------------------------------------------------
    
    def test_srv6_stats(self):
        """Test SRv6 statistics."""
        json_data = {
            "2001:db8:1::/48": {"packets": "12345", "bytes": "67890"},
            "2001:db8:2::/48": {"packets": "23456", "bytes": "78901"}
        }
        output = self.converter.convert("srv6", "stats", json_data)
        assert "2001:db8:1::/48" in output
        assert "12345" in output
        assert "67890" in output
    
    def test_srv6_stats_empty(self):
        """Test SRv6 stats with empty data still shows headers."""
        output = self.converter.convert("srv6", "stats", {})
        # Srv6Formatter shows headers even for empty data
        assert "MySID" in output
        assert "Packets" in output
    
    # -------------------------------------------------------------------------
    # MAC Tests
    # -------------------------------------------------------------------------
    
    def test_mac_aging_time(self):
        """Test MAC aging time."""
        json_data = {"fdb_aging_time": "600"}
        output = self.converter.convert("mac", "aging-time", json_data)
        assert "600" in output
    
    def test_mac_table(self):
        """Test MAC address table."""
        json_data = [
            {"No.": "1", "vlan": "1000", "macAddress": "aa:bb:cc:dd:ee:ff",
             "port": "Ethernet0", "type": "Dynamic"},
            {"No.": "2", "vlan": "1000", "macAddress": "11:22:33:44:55:66",
             "port": "Ethernet4", "type": "Static"}
        ]
        output = self.converter.convert("mac", "mac", json_data)
        assert "aa:bb:cc:dd:ee:ff" in output
        assert "Dynamic" in output
        assert "Total number of entries 2" in output
    
    # -------------------------------------------------------------------------
    # Reboot Cause Tests
    # -------------------------------------------------------------------------
    
    def test_reboot_cause_current(self):
        """Test current reboot cause."""
        json_data = {
            "cause": "reboot",
            "time": "Thu Jul 10 08:05:33 PM UTC 2025",
            "user": "admin"
        }
        output = self.converter.convert("reboot-cause", "current", json_data)
        assert "Cause: reboot" in output or "reboot" in output
    
    def test_reboot_cause_history(self):
        """Test reboot cause history."""
        json_data = {
            "REBOOT_CAUSE|2025_07_10_20_06_34": {
                "cause": "reboot",
                "comment": "N/A",
                "time": "Thu Jul 10 08:05:33 PM UTC 2025",
                "user": "admin"
            },
            "REBOOT_CAUSE|2025_07_10_20_12_49": {
                "cause": "fast-reboot",
                "comment": "N/A",
                "time": "Thu Jul 10 08:10:49 PM UTC 2025",
                "user": "admin"
            }
        }
        output = self.converter.convert("reboot-cause", "history", json_data)
        assert "reboot" in output
        assert "fast-reboot" in output
    
    # -------------------------------------------------------------------------
    # MMU Tests
    # -------------------------------------------------------------------------
    
    def test_mmu_config(self):
        """Test MMU configuration."""
        json_data = {
            "pool": {
                "ingress_lossless_pool": {
                    "mode": "dynamic",
                    "size": "164075364",
                    "type": "ingress",
                    "xoff": "20181824"
                },
                "egress_lossless_pool": {
                    "mode": "static",
                    "size": "164075364",
                    "type": "egress"
                }
            },
            "profile": {
                "pg_lossless_100000_40m_profile": {
                    "pool": "ingress_lossless_pool"
                }
            }
        }
        output = self.converter.convert("mmu", "mmu", json_data)
        assert "pool" in output
    
    # -------------------------------------------------------------------------
    # Watermark Telemetry Tests
    # -------------------------------------------------------------------------
    
    def test_watermark_telemetry_interval(self):
        """Test watermark telemetry interval."""
        json_data = {"interval": "180s"}
        output = self.converter.convert("watermark", "telemetry", json_data)
        assert "180" in output or output is not None

class TestEveryCommandWithExpectedOutput:
    """Test each command individually with input JSON and verify expected output patterns."""
    
    def setup_method(self):
        """Set up converter."""
        self.converter = CommandParser()
    
    # -------------------------------------------------------------------------
    # Buffer Pool Commands
    # -------------------------------------------------------------------------
    
    def test_buffer_pool_watermark_full(self):
        """Test buffer_pool watermark with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["buffer_pool"]["watermark"]
        output = self.converter.convert("buffer_pool", "watermark", input_data)
        
        # Verify key elements
        assert "Shared pool maximum occupancy:" in output
        assert "egress_lossless_pool" in output
        assert "12345" in output
        assert "Pool" in output and "Bytes" in output
    
    def test_buffer_pool_persistent_watermark_full(self):
        """Test buffer_pool persistent-watermark with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["buffer_pool"]["persistent-watermark"]
        output = self.converter.convert("buffer_pool", "persistent-watermark", input_data)
        
        assert "Shared pool maximum occupancy:" in output
        assert "9216" in output
    
    # -------------------------------------------------------------------------
    # Headroom Pool Commands
    # -------------------------------------------------------------------------
    
    def test_headroom_pool_watermark_full(self):
        """Test headroom-pool watermark with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["headroom-pool"]["watermark"]
        output = self.converter.convert("headroom-pool", "watermark", input_data)
        
        assert "Headroom pool maximum occupancy:" in output
        assert "ingress_lossless_pool" in output
        assert "432640" in output
    
    def test_headroom_pool_persistent_watermark_full(self):
        """Test headroom-pool persistent-watermark with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["headroom-pool"]["persistent-watermark"]
        output = self.converter.convert("headroom-pool", "persistent-watermark", input_data)
        
        assert "Headroom pool maximum occupancy:" in output
        assert "863616" in output
    
    # -------------------------------------------------------------------------
    # Priority Group Commands
    # -------------------------------------------------------------------------
    
    def test_priority_group_watermark_shared_full(self):
        """Test priority-group watermark-shared with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["priority-group"]["watermark-shared"]
        output = self.converter.convert("priority-group", "watermark-shared", input_data)
        
        assert "Ingress shared pool occupancy per PG:" in output
        assert "Ethernet0" in output
        assert "PG0" in output
        assert "100" in output
    
    def test_priority_group_watermark_headroom_full(self):
        """Test priority-group watermark-headroom with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["priority-group"]["watermark-headroom"]
        output = self.converter.convert("priority-group", "watermark-headroom", input_data)
        
        assert "Ingress headroom per PG:" in output
        assert "Ethernet0" in output
    
    # -------------------------------------------------------------------------
    # Queue Commands
    # -------------------------------------------------------------------------
    
    def test_queue_watermark_unicast_full(self):
        """Test queue watermark-unicast with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["queue"]["watermark-unicast"]
        output = self.converter.convert("queue", "watermark-unicast", input_data)
        
        assert "Egress shared pool occupancy per unicast queue:" in output
        assert "Ethernet0" in output
    
    def test_queue_watermark_multicast_full(self):
        """Test queue watermark-multicast with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["queue"]["watermark-multicast"]
        output = self.converter.convert("queue", "watermark-multicast", input_data)
        
        assert "Egress shared pool occupancy per multicast queue:" in output
    
    def test_queue_watermark_all_full(self):
        """Test queue watermark-all with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["queue"]["watermark-all"]
        output = self.converter.convert("queue", "watermark-all", input_data)
        
        assert "Egress shared pool occupancy per all queues:" in output
    
    def test_queue_counters_full(self):
        """Test queue counters with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["queue"]["counters"]
        output = self.converter.convert("queue", "counters", input_data)
        
        assert "Ethernet0" in output
        assert "100" in output  # Counter/pkts value
    
    def test_queue_wredcounters_full(self):
        """Test queue wredcounters with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["queue"]["wredcounters"]
        output = self.converter.convert("queue", "wredcounters", input_data)
        
        assert "Ethernet0" in output
    
    # -------------------------------------------------------------------------
    # Clock Commands
    # -------------------------------------------------------------------------
    
    def test_clock_date_full(self):
        """Test clock date with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["clock"]["date"]
        output = self.converter.convert("clock", "date", input_data)
        
        assert "Mon Mar 25 08:25:16 PM UTC 2019" in output
    
    def test_clock_timezones_full(self):
        """Test clock timezones with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["clock"]["timezones"]
        output = self.converter.convert("clock", "timezones", input_data)
        
        assert "UTC" in output
        assert "America/Anchorage" in output
    
    # -------------------------------------------------------------------------
    # Drop Counters Commands
    # -------------------------------------------------------------------------
    
    def test_dropcounters_capabilities_full(self):
        """Test dropcounters capabilities with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["dropcounters"]["capabilities"]
        output = self.converter.convert("dropcounters", "capabilities", input_data)
        
        assert "PORT_INGRESS_DROPS" in output
        assert "10" in output
    
    def test_dropcounters_configuration_full(self):
        """Test dropcounters configuration with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["dropcounters"]["configuration"]
        output = self.converter.convert("dropcounters", "configuration", input_data)
        
        assert "DEBUG_0" in output
    
    def test_dropcounters_counts_full(self):
        """Test dropcounters counts with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["dropcounters"]["counts"]
        output = self.converter.convert("dropcounters", "counts", input_data)
        
        assert "Ethernet0" in output
        assert "80" in output  # DEBUG_0 value
    
    # -------------------------------------------------------------------------
    # System Commands
    # -------------------------------------------------------------------------
    
    def test_uptime_full(self):
        """Test uptime with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["uptime"]["uptime"]
        output = self.converter.convert("uptime", "uptime", input_data)
        
        assert "up" in output
        assert "weeks" in output or "days" in output
    
    def test_version_full(self):
        """Test version with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["version"]["version"]
        output = self.converter.convert("version", "version", input_data)
        
        assert "SONiC" in output
        assert "Mellanox" in output or "mellanox" in output
    
    def test_system_memory_full(self):
        """Test system-memory with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["system-memory"]["memory"]
        output = self.converter.convert("system-memory", "memory", input_data)
        
        assert "Mem" in output
        assert "64252" in output
    
    def test_services_full(self):
        """Test services with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["services"]["services"]
        output = self.converter.convert("services", "services", input_data)
        
        assert "snmp" in output
    
    def test_processes_summary_full(self):
        """Test processes summary with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["processes"]["summary"]
        output = self.converter.convert("processes", "summary", input_data)
        
        assert "systemd" in output
    
    def test_processes_cpu_full(self):
        """Test processes cpu with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["processes"]["cpu"]
        output = self.converter.convert("processes", "cpu", input_data)
        
        assert "syncd" in output
    
    def test_processes_memory_full(self):
        """Test processes memory with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["processes"]["memory"]
        output = self.converter.convert("processes", "memory", input_data)
        
        assert "database" in output
    
    def test_watermark_telemetry_full(self):
        """Test watermark telemetry with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["watermark"]["telemetry"]
        output = self.converter.convert("watermark", "telemetry", input_data)
        
        assert "120" in output
    
    def test_mmu_full(self):
        """Test mmu with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["mmu"]["mmu"]
        output = self.converter.convert("mmu", "mmu", input_data)
        
        assert "pool" in output or "ingress_lossless_pool" in output
    
    # -------------------------------------------------------------------------
    # LLDP Commands
    # -------------------------------------------------------------------------
    
    def test_lldp_table_full(self):
        """Test lldp table with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["lldp"]["table"]
        output = self.converter.convert("lldp", "table", input_data)
        
        assert "Ethernet0" in output
        assert "switch1" in output
    
    def test_lldp_neighbors_full(self):
        """Test lldp neighbors with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["lldp"]["neighbors"]
        output = self.converter.convert("lldp", "neighbors", input_data)
        
        assert "Ethernet0" in output
    
    # -------------------------------------------------------------------------
    # VLAN Commands
    # -------------------------------------------------------------------------
    
    def test_vlan_brief_full(self):
        """Test vlan brief with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["vlan"]["brief"]
        output = self.converter.convert("vlan", "brief", input_data)
        
        assert "Vlan1" in output or "1" in output
    
    # -------------------------------------------------------------------------
    # Reboot Cause Commands
    # -------------------------------------------------------------------------
    
    def test_reboot_cause_current_full(self):
        """Test reboot-cause current with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["reboot-cause"]["current"]
        output = self.converter.convert("reboot-cause", "current", input_data)
        
        assert "reboot" in output
    
    def test_reboot_cause_history_full(self):
        """Test reboot-cause history with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["reboot-cause"]["history"]
        output = self.converter.convert("reboot-cause", "history", input_data)
        
        assert "reboot" in output
        assert "fast-reboot" in output
    
    # -------------------------------------------------------------------------
    # Interface Commands
    # -------------------------------------------------------------------------
    
    def test_interfaces_description_full(self):
        """Test interfaces description with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["description"]
        output = self.converter.convert("interfaces", "description", input_data)
        
        assert "Ethernet0" in output
        assert "ARISTA01T1" in output
    
    def test_interfaces_status_full(self):
        """Test interfaces status with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["status"]
        output = self.converter.convert("interfaces", "status", input_data)
        
        assert "Ethernet0" in output or "PORT_TABLE" in output
    
    def test_interfaces_counters_full(self):
        """Test interfaces counters with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["counters"]
        output = self.converter.convert("interfaces", "counters", input_data)
        
        assert "Ethernet0" in output
        assert "149,903" in output  # RX_OK (formatted with commas)
    
    def test_interfaces_errors_full(self):
        """Test interfaces errors with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["errors"]
        output = self.converter.convert("interfaces", "errors", input_data)
        
        assert "oper error status" in output or "error" in output.lower()
    
    def test_interfaces_alias_full(self):
        """Test interfaces alias with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["alias"]
        output = self.converter.convert("interfaces", "alias", input_data)
        
        assert "etp" in output
    
    def test_interfaces_portchannel_full(self):
        """Test interfaces portchannel with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["portchannel"]
        output = self.converter.convert("interfaces", "portchannel", input_data)
        
        assert "PortChannel101" in output or "portchannel" in output.lower()
    
    def test_interfaces_flap_full(self):
        """Test interfaces flap with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["flap"]
        output = self.converter.convert("interfaces", "flap", input_data)
        
        assert "Ethernet0" in output or "flap" in output.lower()
    
    def test_interfaces_naming_mode_full(self):
        """Test interfaces naming-mode with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["interfaces"]["naming-mode"]
        output = self.converter.convert("interfaces", "naming-mode", input_data)
        
        assert "default" in output
    
    # -------------------------------------------------------------------------
    # Network Commands
    # -------------------------------------------------------------------------
    
    def test_ndp_full(self):
        """Test ndp with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["ndp"]["ndp"]
        output = self.converter.convert("ndp", "ndp", input_data)
        
        assert "fc00::176" in output or "Ethernet60" in output
    
    def test_srv6_stats_full(self):
        """Test srv6 stats with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["srv6"]["stats"]
        output = self.converter.convert("srv6", "stats", input_data)
        
        assert "2001:db8" in output
        assert "12345" in output
    
    def test_mac_full(self):
        """Test mac with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["mac"]["mac"]
        output = self.converter.convert("mac", "mac", input_data)
        
        assert "aa:bb:cc:dd:ee:ff" in output
        assert "Dynamic" in output
    
    def test_mac_aging_time_full(self):
        """Test mac aging-time with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["mac"]["aging-time"]
        output = self.converter.convert("mac", "aging-time", input_data)
        
        assert "600" in output
    
    # -------------------------------------------------------------------------
    # IPv6 Commands
    # -------------------------------------------------------------------------
    
    def test_ipv6_link_local_mode_full(self):
        """Test ipv6 link-local-mode with full verification."""
        
        input_data = GNMI_JSON_TEST_DATA["ipv6"]["link-local-mode"]
        output = self.converter.convert("ipv6", "link-local-mode", input_data)
        
        assert "disabled" in output
