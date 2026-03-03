"""
Baseline UT: show interfaces counters detailed (Cmd #11)

Source:
  - sonic-utilities UT File : portstat_test.py
  - sonic-utilities Test Func: test_show_intf_counters_detailed
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #11

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters detailed"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": {
        "Rx64": "0",
        "Rx65_127": "0",
        "Rx128_255": "0",
        "Rx256_511": "0",
        "Rx512_1023": "0",
        "Rx1024_1518": "0",
        "Rx1519_2047": "0",
        "Rx2048_4095": "0",
        "Rx4096_9216": "0",
        "Rx9217_16383": "0",
        "RxAll": "4",
        "RxUnicast": "4",
        "RxMulticast": "0",
        "RxBroadcast": "0",
        "RxJabbers": "0",
        "RxFragments": "0",
        "RxUndersize": "0",
        "RxOverruns": "0",
        "Tx64": "0",
        "Tx65_127": "0",
        "Tx128_255": "0",
        "Tx256_511": "0",
        "Tx512_1023": "0",
        "Tx1024_1518": "0",
        "Tx1519_2047": "0",
        "Tx2048_4095": "0",
        "Tx4096_9216": "0",
        "Tx9217_16383": "0",
        "TxAll": "40",
        "TxUnicast": "40",
        "TxMulticast": "0",
        "TxBroadcast": "0",
        "WredGreenDrp": "17",
        "WredYellowDrp": "33",
        "WredRedDrp": "51",
        "WredTotalDrp": "101",
        "TrimPkts": "100",
        "TrimSent": "50",
        "TrimDrp": "50",
        "TimestampClearedCounters": "None"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Packets Received 64 Octets..................... 0
Packets Received 65-127 Octets................. 0
Packets Received 128-255 Octets................ 0
Packets Received 256-511 Octets................ 0
Packets Received 512-1023 Octets............... 0
Packets Received 1024-1518 Octets.............. 0
Packets Received 1519-2047 Octets.............. 0
Packets Received 2048-4095 Octets.............. 0
Packets Received 4096-9216 Octets.............. 0
Packets Received 9217-16383 Octets............. 0

Total Packets Received Without Errors.......... 4
Unicast Packets Received....................... 4
Multicast Packets Received..................... 0
Broadcast Packets Received..................... 0

Jabbers Received............................... 0
Fragments Received............................. 0
Undersize Received............................. 0
Overruns Received.............................. 0

Packets Transmitted 64 Octets.................. 0
Packets Transmitted 65-127 Octets.............. 0
Packets Transmitted 128-255 Octets............. 0
Packets Transmitted 256-511 Octets............. 0
Packets Transmitted 512-1023 Octets............ 0
Packets Transmitted 1024-1518 Octets........... 0
Packets Transmitted 1519-2047 Octets........... 0
Packets Transmitted 2048-4095 Octets........... 0
Packets Transmitted 4096-9216 Octets........... 0
Packets Transmitted 9217-16383 Octets.......... 0

Total Packets Transmitted Successfully......... 40
Unicast Packets Transmitted.................... 40
Multicast Packets Transmitted.................. 0
Broadcast Packets Transmitted.................. 0

WRED Green Dropped Packets..................... 17
WRED Yellow Dropped Packets.................... 33
WRED Red Dropped Packets....................... 51
WRED Total Dropped Packets..................... 101

Trimmed Packets................................ 100
Trimmed Sent Packets........................... 50
Trimmed Dropped Packets........................ 50

Time Since Counters Last Cleared............... None"""


@pytest.mark.sonic_baseline
class TestInterfacesCountersDetailed:
    """
    Baseline test for 'show interfaces counters detailed' (Cmd #11).

    Mirrors sonic-utilities test:
      File: portstat_test.py
      Func: test_show_intf_counters_detailed
    """

    def test_converter_output_matches_sonic_ut(self, command_parser):
        """JSON -> Converter -> CLI text should match sonic-utilities expected output."""
        actual = command_parser.parse_and_convert(CLI_COMMAND, json.dumps(JSON_INPUT))
        assert actual.strip() == EXPECTED_CLI_OUTPUT.strip(), (
            f"Converter output for '{CLI_COMMAND}' does not match "
            f"sonic-utilities baseline.\n"
            f"--- Expected ---\n{EXPECTED_CLI_OUTPUT}\n"
            f"--- Actual ---\n{actual}"
        )
