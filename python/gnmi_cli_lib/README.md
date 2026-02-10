# gnmi_cli_lib - gNMI to SONiC CLI Conversion Library

A modular, extensible Python library for converting gNMI JSON responses into human-readable SONiC CLI output format.

## Version

- **Version**: 2.5.0
- **Author**: SONiC Team
- **License**: Apache-2.0 (See the SONiC project license)
- **Last Updated**: February 2026
- **Status**: Production-Ready ✅

---

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Architecture](#architecture)
5. [Unified Command Parser](#unified-command-parser)
6. [Command Reference](#command-reference)
7. [Error Handling](#error-handling)
8. [Testing](#testing)
9. [API Reference](#api-reference)

---

## Overview

This library provides a clean, extensible framework for transforming gNMI JSON responses into the human-readable format used by SONiC CLI show commands. It supports **70 commands** across **20 command categories**.

### Key Features

- **Unified Command Parser**: Single entry point for parsing CLI commands
- **Consistent Naming**: All commands follow `show <command> [subcommand] [optional_arg|<required_arg>] [options]`
- **Modular Design**: Easy to extend with new commands
- **Production-Ready**: Verified with real-device datasets (see analysis report in `tests/reports/`)
- **Self-Contained**: Only requires Python 3.8+ and `tabulate` library
- **Comprehensive Testing**: Unit + integration tests under `python/tests/`

---

## Installation

The library is self-contained within the `gnmi_cli_lib/` directory:

```bash
# Option 1: Install from source directory
cd /path/to/gnmi_cli_lib
pip install -e .

# Option 2: Install dependencies only (for development)
pip install -r requirements.txt

# Option 3: Minimal install (just the required dependency)
pip install tabulate

# Verify installation
python3 -c "from gnmi_cli_lib import CommandParser; print('OK')"
```

### Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Python | 3.8+ | Runtime |
| tabulate | >=0.9.0 | Table formatting (required) |
| pytest | >=7.0.0 | Testing (optional, dev only) |

### Self-Contained Design

The library has **no external dependencies** except `tabulate`. All functionality is implemented using:
- Python standard library modules: `json`, `re`, `typing`, `dataclasses`, `abc`, `datetime`, `ipaddress`
- Single external package: `tabulate` for table formatting

---

## Quick Start

### Using the Unified Command Parser (Recommended)

The `CommandParser` class provides a **single, consistent entry point** for all CLI command parsing and conversion.

```python
from gnmi_cli_lib import CommandParser, parse_and_convert

# Create a parser instance
parser = CommandParser()

# Parse and convert with a CLI command string
json_data = {"Ethernet0": {"admin_status": "up", "oper_status": "up", "speed": "100G"}}
output = parser.parse_and_convert("show interfaces status", json_data)
print(output)

# Or use the convenience function
output = parse_and_convert("show interfaces status", json_data)
print(output)
```

### Using the Converter Class

```python
from gnmi_cli_lib import GnmiToCliConverter

converter = GnmiToCliConverter()
output = converter.convert("interfaces", "status", json_data)
print(output)
```

### Using Convenience Functions

```python
from gnmi_cli_lib import convert_interface_status

output = convert_interface_status(json_data)
print(output)
```

---

## Architecture

### Quick Links

| Category | File | Description |
|----------|------|-------------|
| **Core** | [parser.py](parser.py) | Unified CommandParser (single entry point) |
| | [converter.py](converter.py) | GnmiToCliConverter class |
| | [registry.py](registry.py) | CommandRegistry (command configurations) |
| | [commands.py](commands.py) | Command registrations (70 CLI mappings) |
| | [convenience.py](convenience.py) | 35+ convenience functions |
| | [mappings.py](mappings.py) | CLI_TO_GNMI_MAPPING dictionary |
| | [cli.py](cli.py) | Command-line interface |
| **Types** | [common/types.py](common/types.py) | CommandConfig, CliMapping dataclasses |
| | [common/constants.py](common/constants.py) | Column orderings, SAI fields |
| **Utils** | [utils/sorting.py](utils/sorting.py) | Natural sorting (Ethernet0, Ethernet4, ...) |
| | [utils/parsing.py](utils/parsing.py) | JSON and string parsing utilities |
| **Formatters** | [formatters/base.py](formatters/base.py) | Abstract OutputFormatter, TableFormatter |
| | [formatters/interface.py](formatters/interface.py) | Interface status/counters formatters |
| | [formatters/transceiver.py](formatters/transceiver.py) | Transceiver EEPROM/status formatters |
| | [formatters/ipv6.py](formatters/ipv6.py) | IPv6 BGP/routing formatters |
| | [formatters/system.py](formatters/system.py) | Version, uptime, memory formatters |
| | [formatters/network.py](formatters/network.py) | LLDP, VLAN, MAC, NDP formatters |
| | [formatters/queue.py](formatters/queue.py) | Queue counter formatters |
| | [formatters/dropcounters.py](formatters/dropcounters.py) | Drop counter formatters |
| | [formatters/watermark.py](formatters/watermark.py) | Watermark formatters |
| **Testing** | [../tests/unit/](../tests/unit/) | Unit test suite |
| | [../tests/fixtures/sample_data.py](../tests/fixtures/sample_data.py) | Test data fixtures |
| | [../tests/integration/](../tests/integration/) | Integration verification suite |
| | [../tests/reports/ALL_COMMANDS_ANALYSIS.md](../tests/reports/ALL_COMMANDS_ANALYSIS.md) | Auto-generated analysis report |
| **Config** | [pyproject.toml](pyproject.toml) | Package configuration |
| | [requirements.txt](requirements.txt) | Dependencies |

### Directory Structure

```
gnmi_cli_lib/
├── __init__.py           # Public API exports (90+ functions/classes)
├── parser.py             # Unified CommandParser (single entry point)
├── converter.py          # GnmiToCliConverter class
├── registry.py           # CommandRegistry (command configurations)
├── commands.py           # Command registrations (70 CLI mappings)
├── convenience.py        # 35+ convenience functions
├── mappings.py           # CLI_TO_GNMI_MAPPING dictionary
├── cli.py                # Command-line interface
├── pyproject.toml        # Package configuration
├── requirements.txt      # Dependencies
├── .gitignore            # Git ignore patterns
├── common/
│   ├── types.py          # CommandConfig, CliMapping dataclasses
│   └── constants.py      # Column orderings, SAI fields
├── utils/
│   ├── sorting.py        # Natural sorting (Ethernet0, Ethernet4, ...)
│   └── parsing.py        # JSON and string parsing utilities
├── formatters/
│   ├── base.py           # Abstract OutputFormatter, TableFormatter
│   ├── interface.py      # Interface status/counters formatters
│   ├── transceiver.py    # Transceiver EEPROM/status formatters
│   ├── ipv6.py           # IPv6 BGP/routing formatters
│   ├── system.py         # Version, uptime, memory formatters
│   ├── network.py        # LLDP, VLAN, MAC, NDP formatters
│   ├── queue.py          # Queue counter formatters
│   ├── dropcounters.py   # Drop counter formatters
│   └── watermark.py      # Watermark formatters
└── ../tests/             # Tests live under python/tests (unit + integration + reports)
```

---

## Unified Command Parser

The `CommandParser` class provides a **single, consistent entry point** for all CLI command parsing and conversion.

### Command Naming Convention

All commands follow the format:
```
show <command> <subcommand> [optional_argument|<required_argument>]
```

- `[optional_argument]` - Square brackets indicate optional
- `<required_argument>` - Angle brackets indicate required

### Parser Usage Examples

```python
from gnmi_cli_lib import CommandParser

parser = CommandParser()

# Parse a command to get its components
parsed = parser.parse("show interfaces status Ethernet0")
print(parsed.command)      # "interfaces"
print(parsed.subcommand)   # "status"
print(parsed.argument)     # "Ethernet0"
print(parsed.is_valid)     # True

# Parse and convert in one step
output = parser.parse_and_convert("show buffer_pool watermark", json_data)

# Get help for a command
help_text = parser.get_command_help("interfaces", "status")
print(help_text)  # "show interfaces status [interface_name]"

# List all supported commands
commands = parser.list_commands()
for cmd, subcommands in commands.items():
    print(f"{cmd}: {subcommands}")
```

---

## Command Reference

All commands use consistent naming:
- `show <command> <subcommand> [optional_arg]` for commands with optional arguments
- `show <command> <subcommand> <required_arg>` for commands with required arguments

---

### 1. Buffer Pool Commands

#### 1.1 `show buffer_pool watermark`
Shows buffer pool watermarks.

**Input JSON:**
```json
{
    "egress_lossless_pool": {"Bytes": "2057328"},
    "egress_lossy_pool": {"Bytes": "2056704"},
    "ingress_lossless_pool": {"Bytes": "0"}
}
```

**Expected Output:**
```
Shared pool maximum occupancy:
Pool                    Bytes
--------------------  -------
egress_lossless_pool  2057328
egress_lossy_pool     2056704
ingress_lossless_pool       0
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import CommandParser, parse_and_convert, convert_buffer_pool_watermark

# Method 1: CommandParser class (recommended)
parser = CommandParser()
output = parser.parse_and_convert("show buffer_pool watermark", data)

# Method 2: Convenience function
output = parse_and_convert("show buffer_pool watermark", data)

# Method 3: Direct convenience function
output = convert_buffer_pool_watermark(data)

# Method 4: Converter class
from gnmi_cli_lib import GnmiToCliConverter
converter = GnmiToCliConverter()
output = converter.convert("buffer_pool", "watermark", data)
```

---

#### 1.2 `show buffer_pool persistent-watermark`
Shows persistent buffer pool watermarks.

**Input JSON:**
```json
{
    "egress_lossless_pool": {"Bytes": "3057328"},
    "egress_lossy_pool": {"Bytes": "3056704"}
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_buffer_pool_watermark

# Using parse_and_convert
output = parse_and_convert("show buffer_pool persistent-watermark", data)

# Using convenience function
output = convert_buffer_pool_watermark(data, persistent=True)
```

---

### 2. Headroom Pool Commands

#### 2.1 `show headroom-pool watermark`
Shows headroom pool watermarks.

> **Note**: Uses hyphen (`headroom-pool`) as per consistent naming convention.

**Input JSON:**
```json
{
    "ingress_lossless_pool": {"Bytes": "1024000"}
}
```

**Expected Output:**
```
Headroom pool maximum occupancy:
Pool                   Bytes
--------------------  ------
ingress_lossless_pool 1024000
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_headroom_pool_watermark

# Using unified parser
output = parse_and_convert("show headroom-pool watermark", data)

# Using convenience function
output = convert_headroom_pool_watermark(data)
```

---

#### 2.2 `show headroom-pool persistent-watermark`
Shows persistent headroom pool watermarks.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show headroom-pool persistent-watermark", data)
output = convert_headroom_pool_watermark(data, persistent=True)
```

---

### 3. Clock Commands

#### 3.1 `show clock`
Shows current date and time.

**Input JSON:**
```json
{
    "date": "Mon 20 Jan 2025 07:42:51 UTC"
}
```

**Expected Output:**
```
Mon 20 Jan 2025 07:42:51 UTC
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_clock

output = parse_and_convert("show clock", data)
output = convert_clock(data)
```

---

#### 3.2 `show clock timezones`
Shows available timezones.

**Input JSON:**
```json
{
    "timezones": ["Africa/Abidjan", "Africa/Accra", "America/New_York", "Asia/Tokyo", "Europe/London", "UTC"]
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_clock_timezones

output = parse_and_convert("show clock timezones", data)
output = convert_clock_timezones(data)
```

---

### 4. Drop Counters Commands

#### 4.1 `show dropcounters capabilities`
Shows drop counter capabilities by type.

**Input JSON:**
```json
{
    "PORT_INGRESS_DROPS": ["L2_ANY", "SMAC_MULTICAST", "INGRESS_VLAN_FILTER"],
    "SWITCH_EGRESS_DROPS": ["L3_EGRESS_LINK_DOWN"]
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_dropcounters_capabilities

output = parse_and_convert("show dropcounters capabilities", data)
output = convert_dropcounters_capabilities(data)
```

---

#### 4.2 `show dropcounters configuration`
Shows drop counter configuration.

**Input JSON:**
```json
{
    "DEBUG_0": {
        "alias": "PORT_DROPS",
        "group": "DEBUG",
        "type": "PORT_INGRESS_DROPS",
        "reason": "L2_ANY,SMAC_MULTICAST",
        "description": "Port ingress drops"
    }
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_dropcounters_configuration

output = parse_and_convert("show dropcounters configuration", data)
output = convert_dropcounters_configuration(data)
```

---

#### 4.3 `show dropcounters counts`
Shows port drop counter values.

**Input JSON:**
```json
{
    "Ethernet0": {"DEBUG_0": "100", "DEBUG_1": "0"},
    "Ethernet4": {"DEBUG_0": "50", "DEBUG_1": "25"}
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_dropcounters_counts

output = parse_and_convert("show dropcounters counts", data)
output = convert_dropcounters_counts(data)
```

---

### 5. Interface Commands

#### 5.1 `show interfaces alias [interface_name]`
Shows interface aliases.

**Optional argument:** `interface_name` to show specific interface.

**Input JSON:**
```json
{
    "Ethernet0": {"alias": "fortyGigE0/0"},
    "Ethernet4": {"alias": "fortyGigE0/4"}
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_alias

output = parse_and_convert("show interfaces alias", data)
output = parse_and_convert("show interfaces alias Ethernet0", data)
output = convert_interface_alias(data)
```

---

#### 5.2 `show interfaces counters [interface_name]`
Shows interface counters.

**Optional argument:** `interface_name` to show specific interface.

**Input JSON:**
```json
{
    "Ethernet0": {
        "SAI_PORT_STAT_IF_IN_UCAST_PKTS": "1000000",
        "SAI_PORT_STAT_IF_OUT_UCAST_PKTS": "2000000",
        "SAI_PORT_STAT_IF_IN_ERRORS": "0",
        "SAI_PORT_STAT_IF_OUT_ERRORS": "0"
    }
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters

output = parse_and_convert("show interfaces counters", data)
output = parse_and_convert("show interfaces counters Ethernet0", data)
output = convert_interface_counters(data)
```

---

#### 5.3 `show interfaces counters detailed <interface_name>`
Shows detailed interface counters for a specific interface.

**Required argument:** `interface_name`

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_detailed

output = parse_and_convert("show interfaces counters detailed Ethernet0", data)
output = convert_interface_counters_detailed(data)
```

---

#### 5.4 `show interfaces counters errors [interface_name]`
Shows interface counter errors.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert

output = parse_and_convert("show interfaces counters errors", data)
```

---

#### 5.5 `show interfaces counters fec-histogram [interface_name]`
Shows FEC histogram counters.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_fec_histogram

output = parse_and_convert("show interfaces counters fec-histogram", data)
output = convert_interface_counters_fec_histogram(data)
```

---

#### 5.6 `show interfaces counters fec-stats [interface_name]`
Shows FEC statistics.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_fec_stats

output = parse_and_convert("show interfaces counters fec-stats", data)
output = convert_interface_counters_fec_stats(data)
```

---

#### 5.7 `show interfaces counters rates [interface_name]`
Shows interface counter rates.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_rates

output = parse_and_convert("show interfaces counters rates", data)
output = convert_interface_counters_rates(data)
```

---

#### 5.8 `show interfaces counters rif [interface_name]`
Shows RIF (Router Interface) counters.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_rif

output = parse_and_convert("show interfaces counters rif", data)
output = convert_interface_counters_rif(data)
```

---

#### 5.9 `show interfaces counters trim [interface_name]`
Shows interface trim counters.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_counters_trim

output = parse_and_convert("show interfaces counters trim", data)
output = convert_interface_counters_trim(data)
```

---

#### 5.10 `show interfaces description [interface_name]`
Shows interface descriptions.

**Input JSON:**
```json
{
    "Ethernet0": {"admin_status": "up", "description": "Server1", "oper_status": "up"},
    "Ethernet4": {"admin_status": "up", "description": "Server2", "oper_status": "down"}
}
```

**Expected Output:**
```
  Interface    Oper    Admin    Alias    Description
-----------  ------  -------  -------  -------------
  Ethernet0      up       up              Server1
  Ethernet4    down       up              Server2
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_description

output = parse_and_convert("show interfaces description", data)
output = convert_interface_description(data)
```

---

#### 5.11 `show interfaces errors [interface_name]`
Shows interface errors.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_errors

output = parse_and_convert("show interfaces errors", data)
output = convert_interface_errors(data)
```

---

#### 5.12 `show interfaces fec-status [interface_name]`
Shows FEC status.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_fec_status

output = parse_and_convert("show interfaces fec-status", data)
output = convert_interface_fec_status(data)
```

---

#### 5.13 `show interfaces flap [interface_name]`
Shows interface flap counts.

**Input JSON:**
```json
{
    "Ethernet0": {"flap_count": "5"},
    "Ethernet4": {"flap_count": "0"}
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_flap

output = parse_and_convert("show interfaces flap", data)
output = convert_interface_flap(data)
```

---

#### 5.14 `show interfaces naming-mode`
Shows interface naming mode.

> **Note**: Uses hyphen (`naming-mode`) as per consistent naming convention.

**Input JSON:**
```json
{
    "naming_mode": "default"
}
```

**Expected Output:**
```
default
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_naming_mode

output = parse_and_convert("show interfaces naming-mode", data)
output = convert_interface_naming_mode(data)
```

---

#### 5.15 `show interfaces neighbor expected [interface_name]`
Shows expected interface neighbors.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_neighbor_expected

output = parse_and_convert("show interfaces neighbor expected", data)
output = convert_interface_neighbor_expected(data)
```

---

#### 5.16 `show interfaces portchannel`
Shows port channel information.

**Input JSON:**
```json
{
    "PortChannel0001": {
        "admin_status": "up",
        "oper_status": "up",
        "members": ["Ethernet0", "Ethernet4"],
        "mtu": "9100",
        "min_links": "1"
    }
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_portchannel

output = parse_and_convert("show interfaces portchannel", data)
output = convert_portchannel(data)
```

---

#### 5.17 `show interfaces status [interface_name]`
Shows interface status.

**Optional argument:** `interface_name` to show specific interface.

**Input JSON:**
```json
{
    "Ethernet0": {
        "admin_status": "up",
        "oper_status": "up",
        "speed": "100G",
        "mtu": "9100",
        "fec": "rs",
        "alias": "fortyGigE0/0"
    }
}
```

**Sample CLI Output:**
```
  Interface    Lanes    Speed    MTU    FEC    Alias           Oper    Admin
-----------  -------  -------  -----  -----  --------------  ------  -------
  Ethernet0  33,34,35,36  100G  9100     rs  fortyGigE0/0       up       up
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_status

output = parse_and_convert("show interfaces status", data)
output = parse_and_convert("show interfaces status Ethernet0", data)
output = convert_interface_status(data)
```

---

#### 5.18 `show interfaces switchport-config [interface_name]`
Shows switchport configuration.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_switchport

output = parse_and_convert("show interfaces switchport-config", data)
output = convert_interface_switchport(data, config_or_status="config")
```

---

#### 5.19 `show interfaces switchport-status [interface_name]`
Shows switchport status.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_switchport

output = parse_and_convert("show interfaces switchport-status", data)
output = convert_interface_switchport(data, config_or_status="status")
```

---

#### 5.20-5.26 Transceiver Commands

All transceiver commands support optional `[interface_name]` argument.

| Command | Description |
|---------|-------------|
| `show interfaces transceiver-eeprom [interface_name]` | Transceiver EEPROM |
| `show interfaces transceiver-error-status [interface_name]` | Transceiver error status |
| `show interfaces transceiver-info [interface_name]` | Transceiver info |
| `show interfaces transceiver-lpmode [interface_name]` | Transceiver LP mode |
| `show interfaces transceiver-pm [interface_name]` | Transceiver PM |
| `show interfaces transceiver-presence [interface_name]` | Transceiver presence |
| `show interfaces transceiver-status [interface_name]` | Transceiver status |

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_interface_transceiver

output = parse_and_convert("show interfaces transceiver-info", data)
output = convert_interface_transceiver(data, subcmd="info")
output = convert_interface_transceiver(data, subcmd="presence")
```

---

### 6. IPv6 Commands

#### 6.1 `show ipv6 bgp-neighbors`
Shows IPv6 BGP neighbors.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_bgp_neighbors

output = parse_and_convert("show ipv6 bgp-neighbors", data)
output = convert_ipv6_bgp_neighbors(data)
```

---

#### 6.2 `show ipv6 bgp-network`
Shows IPv6 BGP network.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show ipv6 bgp-network", data)
```

---

#### 6.3 `show ipv6 bgp-summary`
Shows IPv6 BGP summary.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_bgp_summary

output = parse_and_convert("show ipv6 bgp-summary", data)
output = convert_ipv6_bgp_summary(data)
```

---

#### 6.4 `show ipv6 fib`
Shows IPv6 FIB.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show ipv6 fib", data)
```

---

#### 6.5 `show ipv6 interfaces`
Shows IPv6 interfaces.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_interfaces

output = parse_and_convert("show ipv6 interfaces", data)
output = convert_ipv6_interfaces(data)
```

---

#### 6.6 `show ipv6 link-local-mode`
Shows IPv6 link local mode.

> **Note**: Uses hyphens (`link-local-mode`) as per consistent naming convention.

**Input JSON:**
```json
{
    "link_local_mode": "default"
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_link_local_mode

output = parse_and_convert("show ipv6 link-local-mode", data)
output = convert_ipv6_link_local_mode(data)
```

---

#### 6.7 `show ipv6 prefix-list [name]`
Shows IPv6 prefix list.

> **Note**: Uses hyphen (`prefix-list`) as per consistent naming convention.

**Optional argument:** `name` to show specific prefix list.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_prefix_list

output = parse_and_convert("show ipv6 prefix-list", data)
output = convert_ipv6_prefix_list(data)
```

---

#### 6.8 `show ipv6 protocol`
Shows IPv6 protocol.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show ipv6 protocol", data)
```

---

#### 6.9 `show ipv6 route [prefix]`
Shows IPv6 routes.

**Optional argument:** `prefix` to show specific route.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ipv6_route

output = parse_and_convert("show ipv6 route", data)
output = convert_ipv6_route(data)
```

---

### 7. LLDP Commands

#### 7.1 `show lldp neighbors [interface_name]`
Shows LLDP neighbors.

**Optional argument:** `interface_name` to show specific interface.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_lldp_neighbors

output = parse_and_convert("show lldp neighbors", data)
output = convert_lldp_neighbors(data)
```

---

#### 7.2 `show lldp table [interface_name]`
Shows LLDP table.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_lldp_table

output = parse_and_convert("show lldp table", data)
output = convert_lldp_table(data)
```

---

### 8. MAC Commands

#### 8.1 `show mac [address]`
Shows MAC address table.

**Optional argument:** `address` to show specific MAC.

**Input JSON:**
```json
{
    "FDB_TABLE:Vlan1000:00:11:22:33:44:55": {
        "port": "Ethernet0",
        "type": "dynamic"
    }
}
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_mac

output = parse_and_convert("show mac", data)
output = convert_mac(data)
```

---

#### 8.2 `show mac aging-time`
Shows MAC aging time.

> **Note**: Uses hyphen (`aging-time`) as per consistent naming convention.

**Input JSON:**
```json
{
    "fdb_aging_time": "600"
}
```

**Expected Output:**
```
600
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_mac_aging_time

output = parse_and_convert("show mac aging-time", data)
output = convert_mac_aging_time(data)
```

---

### 9. MMU Commands

#### 9.1 `show mmu`
Shows MMU configuration.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_mmu

output = parse_and_convert("show mmu", data)
output = convert_mmu(data)
```

---

### 10. NDP Commands

#### 10.1 `show ndp`
Shows NDP table.

**Input JSON:**
```json
{
    "2001:db8::1": {
        "mac": "00:11:22:33:44:55",
        "interface": "Ethernet0",
        "vlan": "1000",
        "status": "REACHABLE"
    }
}
```

**Sample CLI Output:**
```
Address             MacAddress         Iface       Vlan    Status
-----------------  -----------------  ----------  ------  ---------
2001:db8::1        00:11:22:33:44:55  Ethernet0   1000    REACHABLE
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_ndp

output = parse_and_convert("show ndp", data)
output = convert_ndp(data)
```

---

### 11. Processes Commands

#### 11.1 `show processes summary`
Shows process summary.

**Input JSON:**
```json
[
    {"PID": "1", "PPID": "0", "CMD": "/sbin/init", "MEM%": "0.1", "CPU%": "0.0"},
    {"PID": "100", "PPID": "1", "CMD": "orchagent", "MEM%": "5.2", "CPU%": "10.5"}
]
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_processes

output = parse_and_convert("show processes summary", data)
output = convert_processes(data, sort_by="summary")
```

---

#### 11.2 `show processes cpu`
Shows processes sorted by CPU usage.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show processes cpu", data)
output = convert_processes(data, sort_by="cpu")
```

---

#### 11.3 `show processes memory`
Shows processes sorted by memory usage.

**Usage of the New Unified Entry Point:**
```python
output = parse_and_convert("show processes memory", data)
output = convert_processes(data, sort_by="memory")
```

---

### 12. Queue Commands

#### 12.1 `show queue counters [interface_name]`
Shows queue counters.

**Optional argument:** `interface_name` to show specific interface.

**Input JSON:**
```json
{
    "Ethernet0:UC0": {"Counter/pkts": "1000", "Counter/bytes": "128000", "Drop/pkts": "0"},
    "Ethernet0:UC1": {"Counter/pkts": "2000", "Counter/bytes": "256000", "Drop/pkts": "5"}
}
```

**Sample CLI Output:**
```
       Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
-----------  -----  --------------  ---------------  -----------  ------------
  Ethernet0    UC0            1000           128000            0             0
  Ethernet0    UC1            2000           256000            5             0
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_queue_counters

output = parse_and_convert("show queue counters", data)
output = parse_and_convert("show queue counters Ethernet0", data)
output = convert_queue_counters(data)
```

---

#### 12.2-12.7 Queue Watermark Commands

| Command | Description |
|---------|-------------|
| `show queue watermark-unicast` | Unicast queue watermarks |
| `show queue watermark-multicast` | Multicast queue watermarks |
| `show queue watermark-all` | All queue watermarks |
| `show queue persistent-watermark-unicast` | Persistent unicast watermarks |
| `show queue persistent-watermark-multicast` | Persistent multicast watermarks |
| `show queue persistent-watermark-all` | Persistent all watermarks |

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_queue_watermark

output = parse_and_convert("show queue watermark-unicast", data)
output = convert_queue_watermark(data, queue_type="unicast")
output = convert_queue_watermark(data, queue_type="multicast", persistent=True)
```

---

#### 12.8 `show queue wredcounters [interface_name]`
Shows WRED drop counters.

**Optional argument:** `interface_name` to show specific interface.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_queue_wredcounters

output = parse_and_convert("show queue wredcounters", data)
output = convert_queue_wredcounters(data)
```

---

### 13. Reboot Cause Commands

#### 13.1 `show reboot-cause`
Shows current reboot cause.

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_reboot_cause

output = parse_and_convert("show reboot-cause", data)
output = convert_reboot_cause(data)
```

---

#### 13.2 `show reboot-cause history`
Shows reboot cause history.

**Input JSON:**
```json
{
    "REBOOT_CAUSE|2025_07_10_20_06_34": {
        "cause": "reboot",
        "comment": "N/A",
        "time": "Thu Jul 10 08:05:33 PM UTC 2025",
        "user": "admin"
    }
}
```

**Sample CLI Output:**
```
Name                 Cause    Time                             User            Comment
-------------------  -------  -------------------------------  --------------  ---------
2025_07_10_20_06_34  reboot   Thu Jul 10 08:05:33 PM UTC 2025  admin           N/A
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_reboot_cause

output = parse_and_convert("show reboot-cause history", data)
output = convert_reboot_cause(data, history=True)
```

---

### 14. Services Commands

#### 14.1 `show services`
Shows running Docker services and their processes.

**Input JSON:**
```json
[
    {
        "dockerProcessName": "snmp",
        "processes": [
            {"pid": "123", "user": "root", "cpuPercentage": "0.1",
             "memPercentage": "0.5", "command": "python snmpd.py"}
        ]
    }
]
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_services

output = parse_and_convert("show services", data)
output = convert_services(data)
```

---

### 15. SRv6 Commands

#### 15.1 `show srv6 stats [sid]`
Shows SRv6 counter statistics.

**Optional argument:** `sid` to show specific SID.

**Input JSON:**
```json
{
    "2001:db8:1::/48": {"packets": "12345", "bytes": "67890"},
    "2001:db8:2::/48": {"packets": "23456", "bytes": "78901"}
}
```

**Sample CLI Output:**
```
MySID             Packets          Bytes
----------------  ---------  -------------
2001:db8:1::/48      12345          67890
2001:db8:2::/48      23456          78901
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_srv6_stats

output = parse_and_convert("show srv6 stats", data)
output = convert_srv6_stats(data)
```

---

### 16. System Memory Commands

#### 16.1 `show system-memory`
Shows system memory usage.

**Input JSON:**
```json
[
    {"type": "Mem", "total": "64252", "used": "15017", "free": "7526",
     "shared": "563", "buff/cache": "41708", "available": "46228"},
    {"type": "Swap", "total": "0", "used": "0", "free": "0"}
]
```

**Sample CLI Output:**
```
               total        used        free      shared  buff/cache   available
Mem:           64252       15017        7526         563       41708       46228
Swap:              0           0           0
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_system_memory

output = parse_and_convert("show system-memory", data)
output = convert_system_memory(data)
```

---

### 17. Uptime Commands

#### 17.1 `show uptime`
Shows system uptime.

**Input JSON:**
```json
{
    "uptime": "up 3 weeks, 4 days, 10 hours, 15 minutes"
}
```

**Expected Output:**
```
up 3 weeks, 4 days, 10 hours, 15 minutes
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_uptime

output = parse_and_convert("show uptime", data)
output = convert_uptime(data)
```

---

### 18. Version Commands

#### 18.1 `show version`
Shows SONiC version information.

**Input JSON:**
```json
{
    "sonic_software_version": "SONiC.20241200-12345",
    "sonic_os_version": "11",
    "distribution": "Debian 11.4",
    "kernel": "5.10.0-18-2-amd64",
    "build_commit": "abc123def",
    "build_date": "2025-01-20",
    "built_by": "Azure Pipelines",
    "platform": "x86_64-mlnx_msn2700-r0",
    "hwsku": "Mellanox-SN2700",
    "asic": "mellanox",
    "asic_count": "1",
    "serial_number": "MT1234567890",
    "uptime": "07:42:51 up 16 days",
    "date": "Mon 20 Jan 2025 18:00:00"
}
```

**Sample CLI Output:**
```
SONiC Software Version: SONiC.20241200-12345
SONiC OS Version: 11
Distribution: Debian 11.4
Kernel: 5.10.0-18-2-amd64
Build commit: abc123def
Build date: 2025-01-20
Built by: Azure Pipelines
Platform: x86_64-mlnx_msn2700-r0
HwSKU: Mellanox-SN2700
ASIC: mellanox
ASIC Count: 1
Serial Number: MT1234567890
Uptime: 07:42:51 up 16 days
Date: Mon 20 Jan 2025 18:00:00
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_version

output = parse_and_convert("show version", data)
output = convert_version(data)
```

---

### 19. VLAN Commands

#### 19.1 `show vlan brief`
Shows VLAN brief information.

**Input JSON:**
```json
{
    "Vlan1000": {
        "vlan_id": "1000",
        "ip_address": ["192.168.0.1/24"],
        "ports": [
            {"name": "Ethernet0", "port_tagging": "untagged"},
            {"name": "Ethernet4", "port_tagging": "tagged"}
        ],
        "proxy_arp": "disabled",
        "dhcp_helper_addresses": ["192.0.0.1"]
    }
}
```

**Sample CLI Output:**
```
+-----------+-----------------+---------------+-----------+----------------+
| VLAN ID   | IP Address      | Ports         | Proxy ARP | DHCP Helper    |
+===========+=================+===============+===========+================+
| 1000      | 192.168.0.1/24  | Ethernet0(U)  | disabled  | 192.0.0.1      |
|           |                 | Ethernet4(T)  |           |                |
+-----------+-----------------+---------------+-----------+----------------+
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_vlan_brief

output = parse_and_convert("show vlan brief", data)
output = convert_vlan_brief(data)
```

---

### 20. Watermark Telemetry Commands

#### 20.1 `show watermark telemetry`
Shows watermark telemetry interval.

**Input JSON:**
```json
{
    "interval": "120"
}
```

**Expected Output:**
```
Telemetry interval 120 second(s)
```

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_watermark_telemetry

output = parse_and_convert("show watermark telemetry", data)
output = convert_watermark_telemetry(data)
```

---

### 21. Priority Group Commands

#### 21.1-21.4 Priority Group Watermark Commands

| Command | Description |
|---------|-------------|
| `show priority-group watermark-headroom` | Priority group headroom watermark |
| `show priority-group watermark-shared` | Priority group shared watermark |
| `show priority-group persistent-watermark-headroom` | Persistent headroom watermark |
| `show priority-group persistent-watermark-shared` | Persistent shared watermark |

**Usage of the New Unified Entry Point:**
```python
from gnmi_cli_lib import parse_and_convert, convert_priority_group_watermark

output = parse_and_convert("show priority-group watermark-headroom", data)
output = convert_priority_group_watermark(data, pg_type="headroom")
output = convert_priority_group_watermark(data, pg_type="shared", persistent=True)
```

---

## Error Handling

```python
from gnmi_cli_lib import CommandParser, GnmiToCliConverter

parser = CommandParser()

# Invalid command
parsed = parser.parse("show invalid_command something")
if not parsed.is_valid:
    print(f"Error: {parsed.error_message}")

# Using converter with error handling
converter = GnmiToCliConverter()
try:
    output = converter.convert("invalid", "command", {})
except ValueError as e:
    print(f"Unknown command: {e}")

try:
    output = converter.convert("clock", "date", "invalid json{")
except json.JSONDecodeError as e:
    print(f"Invalid JSON: {e}")
```

---

## Testing

Unit and integration tests live under `python/tests/`.

### Quick Links

| Resource | Description |
|----------|-------------|
| [../tests/unit/](../tests/unit/) | Unit test suite |
| [../tests/integration/](../tests/integration/) | Integration verification suite |
| [../tests/fixtures/sample_data.py](../tests/fixtures/sample_data.py) | Test data fixtures |
| [../tests/reports/ALL_COMMANDS_ANALYSIS.md](../tests/reports/ALL_COMMANDS_ANALYSIS.md) | Auto-generated analysis report |

### Running Unit Tests

```bash
cd ../
python -m pytest tests/unit -v

# With coverage
python -m pytest tests/unit -v --cov=gnmi_cli_lib --cov-report=term-missing
```

### Running Integration Tests

```bash
cd ../
python -m pytest tests/integration -v
```

### Generating the Command Analysis Report

The report in `tests/reports/ALL_COMMANDS_ANALYSIS.md` is generated by the verification tools:

```bash
cd ../gnmi_cli_lib_verifying
python generate_all_commands_analysis.py
```

### Running Unit Tests

```bash
# From the gnmi_cli_lib directory
cd gnmi_cli_lib
python -m pytest tests.py -v

# Run with coverage
python -m pytest tests.py -v --cov=. --cov-report=html

# Run specific test class
python -m pytest tests.py::TestCommandParser -v

# Run tests matching a pattern
python -m pytest tests.py -k "buffer_pool" -v

# Run command coverage tests
python -m pytest tests.py::TestCommandCoverage -v

# Run all tests (unit + real device)
python -m pytest . -v
```

### Test Organization

| Test Suite | Tests | Description |
|------------|-------|-------------|
| **Unit Tests** (`tests.py`) | 1258 | Core library unit tests (97% coverage) |
| **Real Device Tests** (`real_device_tests/`) | 406 | Production device verification |

#### Unit Test Classes (`tests.py`)

| Test Class | Tests | Description |
|------------|-------|-------------|
| `TestImports` | 13 | Import validation |
| `TestCommandRegistry` | 18 | Registry operations |
| `TestBaseFormatters` | 3 | Base formatter tests |
| `TestDropcountersFormatters` | 6 | Drop counter formatters |
| `TestSystemFormatters` | 4 | System formatters |
| `TestInterfaceFormatters` | 3 | Interface formatters |
| `TestNetworkFormatters` | 4 | Network formatters |
| `TestQueueFormatters` | 2 | Queue formatters |
| `TestGnmiToCliConverter` | 18 | Converter tests |
| `TestConvertFromCliCommand` | 11 | CLI command conversion |
| `TestConvenienceFunctions` | 22 | Convenience function tests |
| `TestUtilityFunctions` | 7 | Utility function tests |
| `TestCliToGnmiMapping` | 8 | CLI mapping tests |
| `TestTestDataModule` | 4 | Test data tests |
| `TestEdgeCases` | 14 | Edge case tests |
| `TestIntegration` | 3 | Integration tests |
| `TestTableFormats` | 3 | Table format tests |
| `TestCliModule` | 7 | CLI module tests |
| `TestParserModuleExtended` | 8 | Extended parser tests |
| `TestConvenienceFunctionsExtended` | 6 | Extended convenience tests |
| `TestFormatterExtended` | 14 | Extended formatter tests |
| `TestUtilityFunctionsExtended` | 5 | Extended utility tests |
| `TestCommonTypes` | 4 | Common type tests |
| `TestConstants` | 4 | Constants tests |
| `TestEnhancedCommands` | 29 | Enhanced command tests |
| `TestAllCommandsAvailable` | 3 | Command availability tests |
| `TestCommandParser` | 18 | Command parser tests |
| `TestNewConvenienceFunctions` | 16 | New convenience functions |
| `TestNewCliMappings` | 8 | New CLI mapping tests |
| `TestSampleCliOutputs` | 11 | Sample CLI output tests |
| `TestCommandCoverage` | 3 | **Full command coverage verification** |
| `TestEveryCommandWithExpectedOutput` | 46 | **Individual command tests with JSON input/CLI output** |

#### Real Device Test Classes (`real_device_tests/test_real_device.py`)

| Test Class | Tests | Description |
|------------|-------|-------------|
| `TestRealDeviceConversion` | 406 | **Parametrized tests for each command/device/variant** |
| `TestRealDeviceSummary` | 3 | Summary validation (file count, device coverage, test count) |

### Command Coverage Guarantees

The `TestCommandCoverage` class ensures:

1. **Every registered CLI command has input JSON test data**
2. **Every registered CLI command has expected CLI output**
3. **Every command converts successfully without errors**

The `TestEveryCommandWithExpectedOutput` class provides individual tests for each command category, verifying:

- Input JSON from `GNMI_JSON_TEST_DATA`
- Expected output patterns from `EXPECTED_CLI_OUTPUTS`
- Key data elements are present in the converted output

---

## API Reference

### Core Classes

#### `CommandParser`
Unified command parser providing single entry point.

```python
from gnmi_cli_lib import CommandParser

parser = CommandParser(table_format="simple")

# Parse a command
parsed = parser.parse("show interfaces status")

# Parse and convert
output = parser.parse_and_convert("show interfaces status", json_data)

# List all commands
commands = parser.list_commands()

# Get command help
help_text = parser.get_command_help("interfaces", "status")
```

#### `GnmiToCliConverter`
Main converter class for JSON to CLI conversion.

```python
from gnmi_cli_lib import GnmiToCliConverter

converter = GnmiToCliConverter(table_format="simple")

# Convert using command/subcommand
output = converter.convert("interfaces", "status", json_data)

# Convert using CLI command string
output = converter.convert_from_cli_command("show interfaces status", json_data)
```

#### `CommandRegistry`
Central registry for command configurations.

```python
from gnmi_cli_lib import CommandRegistry, get_default_registry

registry = get_default_registry()

# Get command configuration
config = registry.get_config("interfaces", "status")

# List all commands
commands = registry.list_commands()

# List all CLI mappings
mappings = registry.list_cli_mappings()
```

### Convenience Functions

All convenience functions are available directly from `gnmi_cli_lib`:

```python
from gnmi_cli_lib import (
    # Universal
    convert,
    parse_and_convert,
    
    # Buffer/Headroom Pool
    convert_buffer_pool_watermark,
    convert_headroom_pool_watermark,
    
    # Queue
    convert_queue_watermark,
    convert_queue_counters,
    convert_queue_wredcounters,
    
    # Interface
    convert_interface_status,
    convert_interface_counters,
    convert_interface_description,
    convert_interface_errors,
    convert_interface_alias,
    convert_interface_flap,
    convert_interface_naming_mode,
    convert_interface_neighbor,
    convert_interface_transceiver,
    convert_interface_fec_status,
    convert_interface_switchport,
    convert_interface_counters_detailed,
    convert_interface_counters_rates,
    convert_interface_counters_rif,
    convert_interface_counters_fec_stats,
    convert_interface_counters_fec_histogram,
    convert_interface_counters_trim,
    
    # System
    convert_version,
    convert_uptime,
    convert_system_memory,
    convert_services,
    convert_processes,
    convert_mmu,
    convert_reboot_cause,
    
    # Network
    convert_lldp_table,
    convert_lldp_neighbors,
    convert_vlan_brief,
    convert_mac,
    convert_mac_aging_time,
    convert_ndp,
    convert_srv6_stats,
    
    # IPv6
    convert_ipv6_prefix_list,
    convert_ipv6_link_local_mode,
    convert_ipv6_route,
    convert_ipv6_bgp_summary,
    convert_ipv6_bgp_neighbors,
    convert_ipv6_interfaces,
    
    # Drop Counters
    convert_dropcounters_capabilities,
    convert_dropcounters_configuration,
    convert_dropcounters_counts,
    
    # Clock
    convert_clock,
    convert_clock_timezones,
    
    # Other
    convert_watermark_telemetry,
    convert_portchannel,
    convert_priority_group_watermark,
    
    # Utilities
    get_gnmi_path,
    list_supported_commands,
)
```

---

## Command Line Interface

```bash
# Convert JSON file using CLI command
python -m gnmi_cli_lib.cli -i data.json -c "show interfaces status"

# Read from stdin
cat data.json | python -m gnmi_cli_lib.cli -c "show version"

# List all supported commands
python -m gnmi_cli_lib.cli --list-commands

# Get gNMI path for a command
python -m gnmi_cli_lib.cli --get-path "show interfaces status"
```

---

## Version History

### v2.5.0 (February 2026) - Current Release
- **Bug Fix**: Fixed `show interfaces transceiver error-status <interface>` single-interface output formatting
- **Bug Fix**: Fixed `show ipv6 interfaces` link-local addresses now include zone identifier (`%interface_name`)
- **Bug Fix**: Fixed `show queue counters` empty JSON no longer outputs spurious "For namespace :" header
- **Bug Fix**: Fixed `show ipv6 fib` now outputs header and "Total number of entries 0" when no routes match (instead of empty string)
- **Verification Enhancement**: Added IPv6 link-local zone ID transformation recognition in verification logic
- **Production Ready**: Self-contained library with automated unit and integration verification

### v2.4.0 (February 2026)
- **Enhanced Test Data**: Fixed test data formats for processes/cpu, processes/memory, and lldp/neighbors
- **Improved Test Comparison**: Added timestamp/uptime normalization to handle CLI vs gNMI time drift
- **Production Ready**: Self-contained library with automated unit and integration verification

### v2.3.0 (February 2026)
- **Integrated Real Device Tests**: Moved verification suite into `real_device_tests/` within the library
- **Production Ready**: Self-contained library with 100% pass rate verification
- **Package Configuration**: Added `pyproject.toml`, `requirements.txt`, `.gitignore`
- **Documentation**: Updated README to reflect latest status and structure

### v2.2.0 (February 2026)
- **Bug Fixes**: Fixed multiple formatter and parser issues discovered during real device verification
  - Fixed BGP routes output formatting (spacing, total paths handling)
  - Fixed queue watermark help text option ordering
  - Fixed transceiver EEPROM DOM threshold formatting
  - Fixed IPv6 prefix-list sequence number handling
  - Fixed LLDP neighbors capability code formatting
- **New Formatters**: Added `transceiver.py` and `ipv6.py` formatters
- **Testing**: Achieved 100% pass rate (406/406 tests) across 696 real device test cases on 4 devices
- **Verification Logic**: Implemented 6-step multi-step comparison for robust verification

### v2.1.0
- Enhanced command coverage with 70 CLI command mappings
- Improved error handling and edge case support
- Added comprehensive unit test suite (285+ tests)

### v2.0.0
- Major refactoring with modular formatter architecture
- Added `CommandParser` as unified entry point
- Introduced convenience functions for common operations

---

## License

Apache-2.0. See the SONiC project license.
