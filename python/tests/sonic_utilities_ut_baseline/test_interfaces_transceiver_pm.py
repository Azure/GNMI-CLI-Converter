"""
Baseline UT: show interfaces transceiver pm (Cmd #32)

Source:
  - sonic-utilities UT File : sfp_test.py
  - sonic-utilities Test Func: test_qsfp_dd_pm
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #32

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces transceiver pm"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet44": {
        "pm_table": [
            [
                "Tx Power",
                "dBm",
                "-8.22",
                "-8.23",
                "-8.24",
                "-5.0",
                "-6.0",
                "False",
                "-16.99",
                "-16.003",
                "False"
            ],
            [
                "Rx Total Power",
                "dBm",
                "-10.61",
                "-10.62",
                "-10.62",
                "2.0",
                "0.0",
                "False",
                "-21.0",
                "-18.0",
                "False"
            ],
            [
                "Rx Signal Power",
                "dBm",
                "-40.0",
                "0.0",
                "40.0",
                "13.0",
                "10.0",
                "True",
                "-18.0",
                "-15.0",
                "True"
            ],
            [
                "CD-short link",
                "ps/nm",
                "0.0",
                "0.0",
                "0.0",
                "1000.0",
                "500.0",
                "False",
                "-1000.0",
                "-500.0",
                "False"
            ],
            [
                "PDL",
                "dB",
                "0.5",
                "0.6",
                "0.6",
                "4.0",
                "4.0",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "OSNR",
                "dB",
                "36.5",
                "36.5",
                "36.5",
                "99.0",
                "99.0",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "eSNR",
                "dB",
                "30.5",
                "30.5",
                "30.5",
                "99.0",
                "99.0",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "CFO",
                "MHz",
                "54.0",
                "70.0",
                "121.0",
                "3800.0",
                "3800.0",
                "False",
                "-3800.0",
                "-3800.0",
                "False"
            ],
            [
                "DGD",
                "ps",
                "5.37",
                "5.56",
                "5.81",
                "7.0",
                "7.0",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "SOPMD",
                "ps^2",
                "0.0",
                "0.0",
                "0.0",
                "655.35",
                "655.35",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "SOP ROC",
                "krad/s",
                "1.0",
                "1.0",
                "2.0",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A"
            ],
            [
                "Pre-FEC BER",
                "N/A",
                "4.58E-04",
                "4.66E-04",
                "5.76E-04",
                "1.25E-02",
                "1.10E-02",
                "0.0",
                "0.0",
                "0.0",
                "0.0"
            ],
            [
                "Post-FEC BER",
                "N/A",
                "0.0",
                "0.0",
                "0.0",
                "1000.0",
                "1.0",
                "False",
                "0.0",
                "0.0",
                "False"
            ],
            [
                "EVM",
                "%",
                "100.0",
                "100.0",
                "100.0",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A"
            ]
        ]
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Ethernet44:
    Parameter        Unit    Min       Avg       Max       Threshold    Threshold    Threshold     Threshold    Threshold    Threshold
                                                           High         High         Crossing      Low          Low          Crossing
                                                           Alarm        Warning      Alert-High    Alarm        Warning      Alert-Low
    ---------------  ------  --------  --------  --------  -----------  -----------  ------------  -----------  -----------  -----------
    Tx Power         dBm     -8.22     -8.23     -8.24     -5.0         -6.0         False         -16.99       -16.003      False
    Rx Total Power   dBm     -10.61    -10.62    -10.62    2.0          0.0          False         -21.0        -18.0        False
    Rx Signal Power  dBm     -40.0     0.0       40.0      13.0         10.0         True          -18.0        -15.0        True
    CD-short link    ps/nm   0.0       0.0       0.0       1000.0       500.0        False         -1000.0      -500.0       False
    PDL              dB      0.5       0.6       0.6       4.0          4.0          False         0.0          0.0          False
    OSNR             dB      36.5      36.5      36.5      99.0         99.0         False         0.0          0.0          False
    eSNR             dB      30.5      30.5      30.5      99.0         99.0         False         0.0          0.0          False
    CFO              MHz     54.0      70.0      121.0     3800.0       3800.0       False         -3800.0      -3800.0      False
    DGD              ps      5.37      5.56      5.81      7.0          7.0          False         0.0          0.0          False
    SOPMD            ps^2    0.0       0.0       0.0       655.35       655.35       False         0.0          0.0          False
    SOP ROC          krad/s  1.0       1.0       2.0       N/A          N/A          N/A           N/A          N/A          N/A
    Pre-FEC BER      N/A     4.58E-04  4.66E-04  5.76E-04  1.25E-02     1.10E-02     0.0           0.0          0.0          0.0
    Post-FEC BER     N/A     0.0       0.0       0.0       1000.0       1.0          False         0.0          0.0          False
    EVM              %       100.0     100.0     100.0     N/A          N/A          N/A           N/A          N/A          N/A"""


@pytest.mark.sonic_baseline
class TestInterfacesTransceiverPm:
    """
    Baseline test for 'show interfaces transceiver pm' (Cmd #32).

    Mirrors sonic-utilities test:
      File: sfp_test.py
      Func: test_qsfp_dd_pm
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
