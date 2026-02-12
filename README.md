# gnmi_cli_lib — gNMI to SONiC CLI Conversion Library

A library that converts gNMI JSON telemetry responses into the human-readable tabular output used by SONiC network switch CLI `show` commands.

## Purpose

SONiC switches expose telemetry data via [gNMI](https://github.com/openconfig/gnmi) as JSON. This library transforms that raw JSON into the familiar, human-readable tables that network engineers expect from traditional CLI `show` commands (e.g., `show interfaces status`, `show version`, `show ipv6 bgp summary`).

## Implementations

This repository contains two independent implementations. Each is self-contained with its own documentation, tests, and build system:

| Directory | Language | Documentation |
| --------- | -------- | ------------- |
| [`python/`](python/gnmi_cli_lib/README.md) | Python | [python/gnmi_cli_lib/README.md](python/gnmi_cli_lib/README.md) |
| [`rust/`](rust/gnmi_cli_lib/README.md) | Rust | [rust/gnmi_cli_lib/README.md](rust/gnmi_cli_lib/README.md) |

See each folder's README for installation, usage, API reference, and testing instructions.

## Supported Command Categories

Both implementations support **70 commands** across **20 command categories**:

| Category | Example Commands |
| -------- | --------------- |
| buffer_pool | `show buffer_pool watermark`, `show buffer_pool persistent-watermark` |
| clock | `show clock`, `show clock timezones` |
| dropcounters | `show dropcounters capabilities`, `show dropcounters counts` |
| headroom-pool | `show headroom-pool watermark`, `show headroom-pool persistent-watermark` |
| interfaces | `show interfaces status`, `show interfaces counters`, `show interfaces description`, ... (19 sub-commands) |
| ipv6 | `show ipv6 route`, `show ipv6 bgp summary`, `show ipv6 bgp neighbors`, ... |
| lldp | `show lldp neighbors`, `show lldp table` |
| mac | `show mac`, `show mac aging_time` |
| mmu | `show mmu` |
| ndp | `show ndp` |
| processes | `show processes summary`, `show processes cpu`, `show processes memory` |
| queue | `show queue counters`, `show queue watermark`, `show queue wredcounters`, ... |
| reboot-cause | `show reboot-cause history` |
| services | `show services` |
| srv6 | `show srv6 stats` |
| system-memory | `show system-memory` |
| uptime | `show uptime` |
| version | `show version` |
| vlan | `show vlan brief` |
| watermark | `show watermark telemetry interval` |

## Architecture

Both implementations follow the same modular architecture:

1. **Parser** — Parses `show ...` CLI command strings into structured command objects
2. **Registry** — Maps command keys to `CommandConfig` objects with metadata
3. **Converter** — Orchestrates: parse command → look up config → select formatter → produce output
4. **Formatters** — 78 concrete formatters organized into 9 domain modules, each transforming gNMI JSON into formatted tables
5. **Mappings** — Maps CLI commands to gNMI xpath paths

---

## License

[MIT License](LICENSE) — Copyright (c) Microsoft Corporation.

## Security

See [SECURITY.md](SECURITY.md) for Microsoft's security reporting policy.
