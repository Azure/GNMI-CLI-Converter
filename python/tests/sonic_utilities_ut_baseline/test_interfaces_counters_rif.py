"""
Baseline UT: show interfaces counters rif (Cmd #16)

Source:
  - sonic-utilities UT File : intfstat_test.py
  - sonic-utilities Test Func: TestIntfstat::test_sobject_show
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #16

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces counters rif"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet20": {
        "RX_OK": "4",
        "RX_BPS": "3.00 B/s",
        "RX_PPS": "4.00/s",
        "RX_ERR": "2",
        "TX_OK": "8",
        "TX_BPS": "754.00 B/s",
        "TX_PPS": "8.00/s",
        "TX_ERR": "6"
    },
    "PortChannel0001": {
        "RX_OK": "883",
        "RX_BPS": "608.99 KB/s",
        "RX_PPS": "0.00/s",
        "RX_ERR": "0",
        "TX_OK": "0",
        "TX_BPS": "883.00 B/s",
        "TX_PPS": "0.00/s",
        "TX_ERR": "0"
    },
    "PortChannel0002": {
        "RX_OK": "883",
        "RX_BPS": "608.99 KB/s",
        "RX_PPS": "0.00/s",
        "RX_ERR": "0",
        "TX_OK": "0",
        "TX_BPS": "883.00 B/s",
        "TX_PPS": "0.00/s",
        "TX_ERR": "0"
    },
    "PortChannel0003": {
        "RX_OK": "0",
        "RX_BPS": "0.00 B/s",
        "RX_PPS": "0.00/s",
        "RX_ERR": "0",
        "TX_OK": "0",
        "TX_BPS": "0.00 B/s",
        "TX_PPS": "0.00/s",
        "TX_ERR": "0"
    },
    "PortChannel0004": {
        "RX_OK": "883",
        "RX_BPS": "608.99 KB/s",
        "RX_PPS": "0.00/s",
        "RX_ERR": "0",
        "TX_OK": "0",
        "TX_BPS": "883.00 B/s",
        "TX_PPS": "0.00/s",
        "TX_ERR": "0"
    },
    "Vlan1000": {
        "RX_OK": "0",
        "RX_BPS": "0.00 B/s",
        "RX_PPS": "0.00/s",
        "RX_ERR": "0",
        "TX_OK": "0",
        "TX_BPS": "0.00 B/s",
        "TX_PPS": "0.00/s",
        "TX_ERR": "0"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
          IFACE    RX_OK       RX_BPS    RX_PPS    RX_ERR    TX_OK      TX_BPS    TX_PPS    TX_ERR
---------------  -------  -----------  --------  --------  -------  ----------  --------  --------
     Ethernet20        4     3.00 B/s    4.00/s         2        8  754.00 B/s    8.00/s         6
PortChannel0001      883  608.99 KB/s    0.00/s         0        0  883.00 B/s    0.00/s         0
PortChannel0002      883  608.99 KB/s    0.00/s         0        0  883.00 B/s    0.00/s         0
PortChannel0003        0     0.00 B/s    0.00/s         0        0    0.00 B/s    0.00/s         0
PortChannel0004      883  608.99 KB/s    0.00/s         0        0  883.00 B/s    0.00/s         0
       Vlan1000        0     0.00 B/s    0.00/s         0        0    0.00 B/s    0.00/s         0"""


@pytest.mark.sonic_baseline
class TestInterfacesCountersRif:
    """
    Baseline test for 'show interfaces counters rif' (Cmd #16).

    Mirrors sonic-utilities test:
      File: intfstat_test.py
      Func: TestIntfstat::test_sobject_show
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
