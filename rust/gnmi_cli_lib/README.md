# gnmi_cli_lib

A Rust library (and CLI binary) that converts SONiC gNMI telemetry JSON into
human-readable CLI table output.

**Version** 2.5.0 · **~29 100 lines of Rust** across 25 source files

## Features

| Metric | Value |
| ------ | ----- |
| Verified commands | 70 |
| Unique categories | 19 |
| Concrete formatters (`impl OutputFormatter`) | 78 |
| Formatter modules | 9 |
| Output table formats | 5 (Simple, Grid, Plain, Github, Pipe) |
| Rust test files | 5 + shared helpers (~2 100 lines) |
| Benchmark suite | Criterion (1 file) |

- **70 verified commands** across **19 categories**:
  buffer_pool, clock, dropcounters, headroom-pool, interfaces, ipv6, lldp,
  mac, mmu, ndp, processes, queue, reboot-cause, services, srv6, system,
  vlan, version, watermark.
- **9 formatter modules** – base (4), interface (19), transceiver (8),
  ipv6 (15), system (9), network (11), queue (5), dropcounters (4),
  watermark (3) – producing aligned, column-ordered tables via `comfy-table`.
- **Library + CLI** – use as a Rust crate (`gnmi_cli_lib`) or as a standalone
  binary (`gnmi-cli-convert`).
- **5 table formats** – Simple (default), Grid, Plain, Github, Pipe.
- Unified error types (`GnmiCliError`, `ConvertError`, `FormatError`) with
  `thiserror` integration.
- Convenience functions (`convert`, `available_commands`, …) for one-liner
  access via a lazy-static shared converter.
- CLI-to-gNMI mapping layer.

## Quick start

### Build

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```

### Run clippy

```bash
cargo clippy -- -W clippy::all
```

### Run benchmarks

```bash
cargo bench
```

## Using as a library

Use the same `"show …"` command strings you already know from the CLI:

```rust
use gnmi_cli_lib::parser::parse_and_convert;
use serde_json::json;

fn main() {
    let json_data = json!({"Ethernet0": {"admin_status": "up", "oper_status": "up"}});

    // One-liner — same command string as the CLI
    let output = parse_and_convert("show interfaces status", json_data).unwrap();
    println!("{}", output);
}
```

### More examples

```rust
use gnmi_cli_lib::converter::GnmiToCliConverter;
use gnmi_cli_lib::convenience::convert;
use serde_json::json;

// Quick convert using the internal command key
let output = convert("interfaces_status", json_data).unwrap();

// Or use the converter directly with a CLI command string
let converter = GnmiToCliConverter::new();
let output = converter.convert_from_cli_command("show interfaces status", json_data).unwrap();

// Convert from a raw JSON string
let output = converter.convert_str("interfaces_status", json_str).unwrap();

// List all available commands
let commands = converter.available_commands();

// Choose a different table format
use gnmi_cli_lib::converter::TableFormat;
let converter = GnmiToCliConverter::with_table_format(TableFormat::Grid);
```

## Using as a CLI

```bash
# List all supported CLI commands
gnmi-cli-convert --list-commands

# List commands matching a pattern
gnmi-cli-convert --list-commands --pattern queue

# Convert JSON using an internal command key
gnmi-cli-convert --key interfaces_status --input data.json

# Convert JSON using a CLI command string
gnmi-cli-convert --command "show interfaces status" --input data.json

# Output as JSON envelope
gnmi-cli-convert --command "show interfaces status" --input data.json --format json

# Read from stdin
echo '{}' | gnmi-cli-convert --command "show interfaces status"

# Write output to a file
gnmi-cli-convert --command "show clock" --input data.json --output result.txt

# Enable verbose error output
gnmi-cli-convert --command "show clock" --input data.json --verbose

# Show version
gnmi-cli-convert --version
```

## Project structure

```text
src/                           (25 files, ~29 100 lines)
  lib.rs             – library root, re-exports public API
  main.rs            – CLI binary (clap)
  commands.rs        – registers all 70 commands (+ dash/underscore aliases)
  converter.rs       – GnmiToCliConverter + TableFormat
  parser.rs          – CommandParser + ParsedCommand
  registry.rs        – CommandRegistry + CommandBuilder
  convenience.rs     – one-liner helper functions (lazy-static converter)
  errors.rs          – GnmiCliError unified error type
  mappings.rs        – CLI-to-gNMI mapping
  common/
    mod.rs           – common module root
    types.rs         – shared types (CommandConfig, CliMapping, …)
    constants.rs     – shared constants
  utils/
    mod.rs           – utils module root
    sorting.rs       – natural / interface sorting
    parsing.rs       – JSON parsing helpers
  formatters/
    mod.rs           – FormatterRegistry + re-exports
    base.rs          – generic table / text / list / help formatters (4 formatters)
    interface.rs     – interface-specific formatters (19 formatters)
    transceiver.rs   – transceiver formatters (8 formatters)
    ipv6.rs          – IPv6 formatters (15 formatters)
    system.rs        – system / process formatters (9 formatters)
    network.rs       – LLDP / VLAN / MAC / ARP / SRv6 / NTP formatters (11 formatters)
    queue.rs         – queue counter / priority-group formatters (5 formatters)
    dropcounters.rs  – drop counter formatters (4 formatters)
    watermark.rs     – watermark / buffer-pool / headroom-pool formatters (3 formatters)
benches/
  benchmark.rs       – criterion benchmarks
tests/               (~2 100 lines)
  common/
    mod.rs           – shared test utilities
  test_cli.rs        – CLI binary integration tests
  test_commands.rs   – command registration integration tests
  test_converter.rs  – converter integration tests
  test_formatters.rs – formatter unit tests
  test_parser.rs     – parser integration tests
```

## Public API surface

| Module | Key exports |
| ------ | ----------- |
| `converter` | `GnmiToCliConverter`, `TableFormat`, `ConvertError`, `ConvertResult`, `CommandInfo` |
| `parser` | `CommandParser`, `ParsedCommand`, `parse_command`, `parse_and_convert`, `get_default_parser` |
| `registry` | `CommandRegistry`, `create_default_registry` |
| `common::types` | `CommandConfig`, `CliMapping` |
| `formatters` | `OutputFormatter` trait, `FormatterRegistry`, 78 concrete formatters |
| `errors` | `GnmiCliError` |
| `mappings` | `CLI_TO_GNMI_MAPPING` |
| `convenience` | `convert()`, `available_commands()`, per-command helpers |

## All 70 verified commands

| # | CLI Command | Category |
| -- | ---------- | -------- |
| 01 | `show buffer_pool watermark` | buffer_pool |
| 02 | `show buffer_pool persistent-watermark` | buffer_pool |
| 03 | `show clock` | clock |
| 04 | `show clock timezones` | clock |
| 05 | `show dropcounters capabilities` | dropcounters |
| 06 | `show dropcounters counts` | dropcounters |
| 07 | `show headroom-pool watermark` | headroom-pool |
| 08 | `show headroom-pool persistent-watermark` | headroom-pool |
| 09 | `show interfaces alias` | interfaces |
| 10 | `show interfaces counters` | interfaces |
| 11 | `show interfaces counters detailed` | interfaces |
| 12 | `show interfaces counters errors` | interfaces |
| 13 | `show interfaces counters fec-histogram` | interfaces |
| 14 | `show interfaces counters fec-stats` | interfaces |
| 15 | `show interfaces counters rates` | interfaces |
| 16 | `show interfaces counters rif` | interfaces |
| 17 | `show interfaces counters trim` | interfaces |
| 18 | `show interfaces description` | interfaces |
| 19 | `show interfaces errors` | interfaces |
| 20 | `show interfaces fec status` | interfaces |
| 21 | `show interfaces flap` | interfaces |
| 22 | `show interfaces naming_mode` | interfaces |
| 23 | `show interfaces neighbor expected` | interfaces |
| 24 | `show interfaces portchannel` | interfaces |
| 25 | `show interfaces status` | interfaces |
| 26 | `show interfaces switchport config` | interfaces |
| 27 | `show interfaces switchport status` | interfaces |
| 28 | `show interfaces transceiver eeprom` | interfaces |
| 29 | `show interfaces transceiver error-status` | interfaces |
| 30 | `show interfaces transceiver info` | interfaces |
| 31 | `show interfaces transceiver lpmode` | interfaces |
| 32 | `show interfaces transceiver pm` | interfaces |
| 33 | `show interfaces transceiver presence` | interfaces |
| 34 | `show interfaces transceiver status` | interfaces |
| 35 | `show ipv6 prefix-list` | ipv6 |
| 36 | `show ipv6 interfaces` | ipv6 |
| 37 | `show ipv6 route` | ipv6 |
| 38 | `show ipv6 protocol` | ipv6 |
| 39 | `show ipv6 bgp neighbors` | ipv6 |
| 40 | `show ipv6 bgp network` | ipv6 |
| 41 | `show ipv6 bgp summary` | ipv6 |
| 42 | `show ipv6 link-local-mode` | ipv6 |
| 43 | `show ipv6 fib` | ipv6 |
| 44 | `show lldp neighbors` | lldp |
| 45 | `show lldp table` | lldp |
| 46 | `show mac` | mac |
| 47 | `show mac aging-time` | mac |
| 48 | `show mmu` | mmu |
| 49 | `show ndp` | ndp |
| 50 | `show processes summary` | processes |
| 51 | `show processes cpu` | processes |
| 52 | `show processes memory` | processes |
| 53 | `show queue counters` | queue |
| 54 | `show queue persistent-watermark` | queue |
| 55 | `show queue persistent-watermark all` | queue |
| 56 | `show queue persistent-watermark multicast` | queue |
| 57 | `show queue persistent-watermark unicast` | queue |
| 58 | `show queue watermark` | queue |
| 59 | `show queue watermark all` | queue |
| 60 | `show queue watermark multicast` | queue |
| 61 | `show queue watermark unicast` | queue |
| 62 | `show queue wredcounters` | queue |
| 63 | `show reboot-cause history` | reboot-cause |
| 64 | `show services` | services |
| 65 | `show srv6 stats` | srv6 |
| 66 | `show system-memory` | system |
| 67 | `show uptime` | system |
| 68 | `show version` | version |
| 69 | `show vlan brief` | vlan |
| 70 | `show watermark telemetry interval` | watermark |

## Dependencies

| Crate              | Version | Purpose                             |
| ------------------ | ------- | ----------------------------------- |
| serde / serde_json | 1.0     | JSON serialisation (preserve_order) |
| comfy-table        | 7.1     | aligned CLI table output            |
| regex              | 1.10    | pattern matching in formatters      |
| clap               | 4.4     | CLI argument parsing (derive)       |
| thiserror          | 1.0     | ergonomic error types               |
| lazy_static        | 1.4     | global singletons                   |
| once_cell          | 1.19    | lazy static initialisation          |

### Dev-dependencies

| Crate              | Version | Purpose                              |
| ------------------ | ------- | ------------------------------------ |
| pretty_assertions  | 1.4     | readable test diffs                  |
| tempfile           | 3.10    | temporary files in tests             |
| criterion          | 0.5     | benchmarking framework               |

### Release profile

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## License

Apache-2.0
