# Sonic-Utilities Baseline UT Design Document

> Based on the mapping in `gnmi_converter_vs_show_ut_mapping.md`, add a new set of regression tests for the GNMI-CLI-Converter.  
> Core idea: **Use mock data (Expected CLI Output) from sonic-utilities UTs as a baseline to verify that the Converter produces identical CLI output for the same JSON input.**

---

## 1. Directory Structure

```
tests/
├── conftest.py                          # Existing (provides command_parser fixture)
├── unit/                                # Existing unit tests
├── integration/                         # Existing integration tests
├── sonic_utilities_ut_baseline/          # ★ New directory (independent from existing tests)
│   ├── __init__.py
│   ├── README.md                        # Brief description
│   ├── conftest.py                      # Baseline-specific fixtures
│   ├── test_buffer_pool_watermark.py
│   ├── test_buffer_pool_persistent_watermark.py
│   ├── test_dropcounters_capabilities.py
│   ├── ...                              # One file per command
│   └── test_vlan_brief.py
```

### Naming Convention

- File name: `test_{command_slug}.py`
  - `command_slug` = spaces and `-` in the command name replaced with `_`, e.g. `show buffer_pool watermark` → `buffer_pool_watermark`
  - The mapping document number is included in the file's docstring, not in the file name
- Test function name: `test_converter_output_matches_sonic_ut` (each file has only one main test function)

### Why a Separate Directory?

1. **No interference with existing tests**: Existing `unit/` tests focus on formatter-level functionality; baseline tests focus on end-to-end command-level verification
2. **Can be run independently**: `pytest tests/sonic_utilities_ut_baseline/` runs them in isolation
3. **Easy to skip on demand**: Can be controlled via pytest markers or directory-level conftest

---

## 2. JSON Input & Expected CLI Output Storage Approach

### Decision: **All inline within test files**

#### Considered Options

| Option | Pros | Cons |
|--------|------|------|
| A. Inline (JSON + expected both in .py file) | Complete context visible by opening one file; diffs are straightforward | Some commands have large data (e.g. #37 ipv6 route ~330 lines of JSON) |
| B. External fixture files (separate JSON/expected files) | Data and logic separated | Need to jump between 2-3 files; file count balloons (49×3=147 files) |
| C. Hybrid (small data inline, large data external) | Compromise | Inconsistent, increases maintenance cost |

**Rationale for choosing Option A (all inline):**

1. **"One command, one file" principle**: Each file is self-contained with all information (command, JSON input, expected output, provenance) — no external dependencies.
2. **Reviewer-friendly**: Opening a single file shows: where the JSON comes from, where the expected output comes from, and what the converter does.
3. **Data size is manageable**: Among the 49 commands, the largest JSON (`show ipv6 route`) is ~330 lines — perfectly acceptable for a Python file. Most commands have JSON under 50 lines.
4. **Avoids file proliferation**: 49 .py files vs. 49 .py + 49 .json + 49 .txt = 147 files.

---

## 3. Test File Template

```python
"""
Baseline UT: show buffer_pool watermark (Cmd #01)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_buffer_pool_wm
  - Expected CLI Output Var : show_buffer_pool_wm_output (from wm_test_vectors.py)
  - Mapping Doc Section     : ### #1

Verification Status: ✅ VERIFIED
"""

import pytest

# ── JSON Input (reverse-engineered from sonic-utilities UT mock data) ──
JSON_INPUT = {
    "egress_lossless_pool": {"Bytes": "1000"},
    "egress_lossy_pool": {"Bytes": "2000"},
    "ingress_lossless_pool": {"Bytes": "3000"},
}

# ── Expected CLI Output (from sonic-utilities UT: wm_test_vectors.py) ──
EXPECTED_CLI_OUTPUT = """\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool     1000
    egress_lossy_pool     2000
ingress_lossless_pool     3000
"""

CLI_COMMAND = "show buffer_pool watermark"


class TestBufferPoolWatermark:
    """
    Baseline test for 'show buffer_pool watermark' (Cmd #01).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_buffer_pool_wm
    """

    def test_converter_output_matches_sonic_ut(self, command_parser):
        """JSON → Converter → CLI text should match sonic-utilities expected output."""
        actual = command_parser.parse_and_convert(CLI_COMMAND, JSON_INPUT)
        assert actual.strip() == EXPECTED_CLI_OUTPUT.strip(), (
            f"Converter output for '{CLI_COMMAND}' does not match "
            f"sonic-utilities baseline.\n"
            f"--- Expected ---\n{EXPECTED_CLI_OUTPUT}\n"
            f"--- Actual ---\n{actual}"
        )
```

### Key Design Elements

| Element | Description |
|---------|-------------|
| **Module docstring** | Contains original sonic-utilities UT file name, function name, variable name, mapping doc section number |
| **`JSON_INPUT`** | Python dict, copied directly from the mapping doc's `JSON Input` block |
| **`EXPECTED_CLI_OUTPUT`** | Multi-line string, copied directly from the mapping doc's `Expected CLI Output` block |
| **`CLI_COMMAND`** | Full show command string |
| **Class name** | `Test{PascalCaseSlug}`, corresponds to the file name (number is in the class docstring) |
| **`command_parser` fixture** | From root `conftest.py` (session-scoped `CommandParser` instance) |
| **Assertion** | Exact comparison after `strip()`, prints diff on failure |

---

## 4. Command Classification and Handling Strategy

### 4.1 ✅ VERIFIED (49) — Exact Match Tests

JSON has been verified for these commands: `parse_and_convert(command, json) == expected_output`.

| # | Command | sonic-utils UT File | Test Function |
|---|---------|-------------------|---------------|
| 01 | show buffer_pool watermark | watermarkstat_test.py | test_show_buffer_pool_wm |
| 02 | show buffer_pool persistent-watermark | watermarkstat_test.py | test_show_buffer_pool_persistent_wm |
| 05 | show dropcounters capabilities | drops_group_test.py | test_show_capabilities |
| 06 | show dropcounters counts | drops_group_test.py | test_show_counts |
| 07 | show headroom-pool watermark | watermarkstat_test.py | test_show_hdrm_pool_wm |
| 08 | show headroom-pool persistent-watermark | watermarkstat_test.py | test_show_hdrm_pool_persistent_wm |
| 09 | show interfaces alias | intfutil_test.py | test_show_interfaces_alias |
| 10 | show interfaces counters | portstat_test.py | test_show_intf_counters |
| 11 | show interfaces counters detailed | portstat_test.py | test_show_intf_counters_detailed |
| 13 | show interfaces counters fec-histogram | intfstat_test.py | test_fec_histogram |
| 14 | show interfaces counters fec-stats | intfstat_test.py | test_fec_stats |
| 16 | show interfaces counters rif | intfstat_test.py | test_intf_counters_rif |
| 17 | show interfaces counters trim | intfstat_test.py | test_trim_counters |
| 18 | show interfaces description | intfutil_test.py | test_show_interfaces_description |
| 19 | show interfaces errors | intfstat_test.py | test_intf_errors |
| 20 | show interfaces fec status | intfutil_test.py | test_show_interfaces_fec_status |
| 21 | show interfaces flap | intfutil_test.py | test_show_interfaces_flap |
| 22 | show interfaces naming_mode | intfutil_test.py | test_show_interfaces_naming_mode |
| 23 | show interfaces neighbor expected | intfutil_test.py | test_show_interfaces_neighbor_expected |
| 24 | show interfaces portchannel | intfutil_test.py | test_show_interfaces_portchannel |
| 25 | show interfaces status | intfutil_test.py | test_show_interfaces_status |
| 26 | show interfaces switchport config | switchport_test.py | test_switchport_config |
| 27 | show interfaces switchport status | switchport_test.py | test_switchport_status |
| 28 | show interfaces transceiver eeprom | sfputil_test.py | test_show_eeprom |
| 30 | show interfaces transceiver info | sfputil_test.py | test_show_eeprom |
| 32 | show interfaces transceiver pm | sfputil_test.py | test_show_pm |
| 33 | show interfaces transceiver presence | sfputil_test.py | test_show_presence |
| 34 | show interfaces transceiver status | sfputil_test.py | test_show_status |
| 36 | show ipv6 interfaces | ipv6_intf_test.py | test_show_ipv6_interfaces |
| 37 | show ipv6 route | bgp_v6_test.py | test_show_ipv6_route |
| 39 | show ipv6 bgp neighbors | show_bgp_neighbor_test.py | test_bgp_neighbors |
| 40 | show ipv6 bgp network | bgp_v6_test.py | test_show_ipv6_bgp_network |
| 41 | show ipv6 bgp summary | bgp_commands_test.py | test_bgp_summary_v6 |
| 42 | show ipv6 link-local-mode | ipv6_link_local_test.py | test_show_ipv6_link_local_mode |
| 43 | show ipv6 fib | fib_test.py | test_show_ipv6_fib |
| 46 | show mac | mac_test.py | test_show_mac |
| 47 | show mac aging_time | mac_test.py | test_show_mac_aging_time |
| 48 | show mmu | mmu_test.py | test_show_mmu_config |
| 53 | show queue counters | queuestat_test.py | test_show_queue_counters |
| 55 | show queue persistent-watermark all | watermarkstat_test.py | test_show_queue_all_persistent_wm |
| 56 | show queue persistent-watermark multicast | watermarkstat_test.py | test_show_queue_multicast_persistent_wm |
| 57 | show queue persistent-watermark unicast | watermarkstat_test.py | test_show_queue_unicast_persistent_wm |
| 59 | show queue watermark all | watermarkstat_test.py | test_show_queue_all_wm |
| 60 | show queue watermark multicast | watermarkstat_test.py | test_show_queue_multicast_wm |
| 61 | show queue watermark unicast | watermarkstat_test.py | test_show_queue_unicast_wm |
| 62 | show queue wredcounters | queuestat_test.py | test_show_queue_wredcounters |
| 63 | show reboot-cause history | reboot_cause_test.py | test_reboot_cause_history |
| 65 | show srv6 stats | srv6stat_test.py | test_show_srv6_stats |
| 69 | show vlan brief | vlan_test.py | test_show_vlan_brief |

**Approach:**
```python
def test_converter_output_matches_sonic_ut(self, command_parser):
    actual = command_parser.parse_and_convert(CLI_COMMAND, JSON_INPUT)
    assert actual.strip() == EXPECTED_CLI_OUTPUT.strip()
```

### 4.2 ❌ NO_OUTPUT (17) — No Test Files Created

These sonic-utilities UTs only check exit_code / mock call arguments and have no expected CLI output that can be used as a baseline.

| # | Command |
|---|---------|
| 03 | show clock |
| 04 | show clock timezones |
| 12 | show interfaces counters errors |
| 15 | show interfaces counters rates |
| 29 | show interfaces transceiver error-status |
| 31 | show interfaces transceiver lpmode |
| 35 | show ipv6 prefix-list |
| 38 | show ipv6 protocol |
| 44 | show lldp neighbors |
| 45 | show lldp table |
| 49 | show ndp |
| 50 | show processes summary |
| 51 | show processes cpu |
| 52 | show processes memory |
| 66 | show system-memory |
| 67 | show uptime |
| 70 | show watermark telemetry interval |

**Approach:** No test files generated. Without expected output, baseline comparison is not possible.

### 4.4 Special Cases

| # | Command | Situation | Handling |
|---|---------|-----------|----------|
| 54 | show queue persistent-watermark | Parent command; actual tests are in #55/#56/#57 | No separate file |
| 58 | show queue watermark | Parent command; actual tests are in #59/#60/#61 | No separate file |
| 64 | show services | NA — no sonic-utilities UT exists | No file |
| 68 | show version | PARTIAL — only checks "SONiC OS Version: 11" | No file (no complete expected output) |

---

## 5. conftest.py Design

```python
# tests/sonic_utilities_ut_baseline/conftest.py
"""
Conftest for sonic-utilities baseline tests.

The `command_parser` fixture is inherited from the root conftest.py.
This file adds baseline-specific markers and utilities.
"""

import pytest


def pytest_configure(config):
    """Register custom markers."""
    config.addinivalue_line(
        "markers",
        "sonic_baseline: mark test as sonic-utilities baseline comparison"
    )
```

---

## 6. Provenance Information (Fulfills the "Show Original UT File Name and Function Name" Requirement)

Each test file includes provenance information at the following locations:

1. **Module docstring** (file header):
   ```python
   """
   Baseline UT: show interfaces status (Cmd #25)
   
   Source:
     - sonic-utilities UT File : intfutil_test.py
     - sonic-utilities Test Func: test_show_interfaces_status
     - Expected CLI Output Var : show_interface_status_output (from intfutil_test_vectors.py)
     - Mapping Doc Section     : ### #25

   Verification Status: ✅ VERIFIED
   """
   ```

2. **Class docstring**:
   ```python
   class TestInterfacesStatus:
       """
       Mirrors sonic-utilities test:
         File: intfutil_test.py
         Func: test_show_interfaces_status
       """
   ```

3. **Constant comments**:
   ```python
   # ── Expected CLI Output (from sonic-utilities UT: intfutil_test_vectors.py) ──
   EXPECTED_CLI_OUTPUT = """..."""
   ```

---

## 7. File Generation Summary

| Category | Count | Test Method |
|----------|-------|-------------|
| ✅ VERIFIED | 49 | `assert actual == expected` (exact match) |
| ❌ NO_OUTPUT | 17 | No file created |
| Special (parent/NA/PARTIAL) | 4 | No file created |
| **Total files generated** | **49** | |

### Complete File List

```
tests/sonic_utilities_ut_baseline/
├── __init__.py
├── conftest.py
├── test_buffer_pool_watermark.py                  ✅
├── test_buffer_pool_persistent_watermark.py       ✅
├── test_dropcounters_capabilities.py              ✅
├── test_dropcounters_counts.py                    ✅
├── test_headroom_pool_watermark.py                ✅
├── test_headroom_pool_persistent_watermark.py     ✅
├── test_interfaces_alias.py                       ✅
├── test_interfaces_counters.py                    ✅
├── test_interfaces_counters_detailed.py           ✅
├── test_interfaces_counters_fec_histogram.py      ✅
├── test_interfaces_counters_fec_stats.py          ✅
├── test_interfaces_counters_rif.py                ✅
├── test_interfaces_counters_trim.py               ✅
├── test_interfaces_description.py                 ✅
├── test_interfaces_errors.py                      ✅
├── test_interfaces_fec_status.py                  ✅
├── test_interfaces_flap.py                        ✅
├── test_interfaces_naming_mode.py                 ✅
├── test_interfaces_neighbor_expected.py           ✅
├── test_interfaces_portchannel.py                 ✅
├── test_interfaces_status.py                      ✅
├── test_interfaces_switchport_config.py           ✅
├── test_interfaces_switchport_status.py           ✅
├── test_interfaces_transceiver_eeprom.py          ✅
├── test_interfaces_transceiver_info.py            ✅
├── test_interfaces_transceiver_pm.py              ✅
├── test_interfaces_transceiver_presence.py        ✅
├── test_interfaces_transceiver_status.py          ✅
├── test_ipv6_interfaces.py                        ✅
├── test_ipv6_route.py                             ✅
├── test_ipv6_bgp_neighbors.py                      ✅
├── test_ipv6_bgp_network.py                       ✅
├── test_ipv6_bgp_summary.py                       ✅
├── test_ipv6_link_local_mode.py                   ✅
├── test_ipv6_fib.py                               ✅
├── test_mac.py                                    ✅
├── test_mac_aging_time.py                         ✅
├── test_mmu.py                                    ✅
├── test_queue_counters.py                         ✅
├── test_queue_persistent_watermark_all.py         ✅
├── test_queue_persistent_watermark_mc.py          ✅
├── test_queue_persistent_watermark_uc.py          ✅
├── test_queue_watermark_all.py                    ✅
├── test_queue_watermark_multicast.py              ✅
├── test_queue_watermark_unicast.py                ✅
├── test_queue_wredcounters.py                     ✅
├── test_reboot_cause_history.py                   ✅
├── test_srv6_stats.py                             ✅
└── test_vlan_brief.py                             ✅
```

---

## 8. How to Run

```bash
# Run all baseline tests
pytest tests/sonic_utilities_ut_baseline/ -v

# Run a single command's test
pytest tests/sonic_utilities_ut_baseline/test_buffer_pool_watermark.py -v
```

---

## 9. Maintenance Guidelines

1. **Adding new commands**: Follow the template to add a new file; numbering follows the mapping document.
2. **NO_OUTPUT commands**: If sonic-utilities adds expected output in the future, baseline tests can be supplemented.
3. **Data provenance**: All data can be traced back to the original sonic-utilities UT via the mapping document (`gnmi_converter_vs_show_ut_mapping.md`).
