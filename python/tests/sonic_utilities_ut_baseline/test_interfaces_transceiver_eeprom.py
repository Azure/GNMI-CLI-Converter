"""
Baseline UT: show interfaces transceiver eeprom (Cmd #28)

Source:
  - sonic-utilities UT File : sfp_test.py
  - sonic-utilities Test Func: test_sfp_eeprom_with_dom
  - Expected CLI Output Var :  (from )
  - Mapping Doc Section     : ### #28

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show interfaces transceiver eeprom"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "Ethernet0": "{\"Application Advertisement\": \"N/A\", \"Connector\": \"No separable connector\", \"Encoding\": \"64B66B\", \"Extended Identifier\": \"Power Class 3(2.5W max), CDR present in Rx Tx\", \"Extended RateSelect Compliance\": \"QSFP+ Rate Select Version 1\", \"Identifier\": \"QSFP28 or later\", \"Length Cable Assembly(m)\": \"3\", \"Nominal Bit Rate(100Mbs)\": \"255\", \"Specification compliance\": {\"10/40G Ethernet Compliance Code\": \"40G Active Cable (XLPPI)\"}, \"Vendor Date Code(YYYY-MM-DD Lot)\": \"2017-01-13\", \"Vendor Name\": \"Mellanox\", \"Vendor OUI\": \"00-02-c9\", \"Vendor PN\": \"MFA1A00-C003\", \"Vendor Rev\": \"AC\", \"Vendor SN\": \"MT1706FT02064\", \"ChannelMonitorValues\": \"\", \"RX1Power\": \"0.3802dBm\", \"RX2Power\": \"-0.4871dBm\", \"RX3Power\": \"-0.0860dBm\", \"RX4Power\": \"0.3830dBm\", \"TX1Bias\": \"6.7500mA\", \"TX2Bias\": \"6.7500mA\", \"TX3Bias\": \"6.7500mA\", \"TX4Bias\": \"6.7500mA\", \"ChannelThresholdValues\": \"\", \"RxPowerHighAlarm\": \"3.4001dBm\", \"RxPowerHighWarning\": \"2.4000dBm\", \"RxPowerLowAlarm\": \"-13.5067dBm\", \"RxPowerLowWarning\": \"-9.5001dBm\", \"TxBiasHighAlarm\": \"10.0000mA\", \"TxBiasHighWarning\": \"9.5000mA\", \"TxBiasLowAlarm\": \"0.5000mA\", \"TxBiasLowWarning\": \"1.0000mA\", \"ModuleMonitorValues\": \"\", \"Temperature\": \"30.9258C\", \"Vcc\": \"3.2824Volts\", \"ModuleThresholdValues\": \"\", \"TempHighAlarm\": \"75.0000C\", \"TempHighWarning\": \"70.0000C\", \"TempLowAlarm\": \"-5.0000C\", \"TempLowWarning\": \"0.0000C\", \"VccHighAlarm\": \"3.6300Volts\", \"VccHighWarning\": \"3.4650Volts\", \"VccLowAlarm\": \"2.9700Volts\", \"VccLowWarning\": \"3.1349Volts\"}"
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Ethernet0: SFP EEPROM detected
        Application Advertisement: N/A
        Connector: No separable connector
        Encoding: 64B66B
        Extended Identifier: Power Class 3(2.5W max), CDR present in Rx Tx
        Extended RateSelect Compliance: QSFP+ Rate Select Version 1
        Identifier: QSFP28 or later
        Length Cable Assembly(m): 3
        Nominal Bit Rate(100Mbs): 255
        Specification compliance:
                10/40G Ethernet Compliance Code: 40G Active Cable (XLPPI)
        Vendor Date Code(YYYY-MM-DD Lot): 2017-01-13
        Vendor Name: Mellanox
        Vendor OUI: 00-02-c9
        Vendor PN: MFA1A00-C003
        Vendor Rev: AC
        Vendor SN: MT1706FT02064
        ChannelMonitorValues:
                RX1Power: 0.3802dBm
                RX2Power: -0.4871dBm
                RX3Power: -0.0860dBm
                RX4Power: 0.3830dBm
                TX1Bias: 6.7500mA
                TX2Bias: 6.7500mA
                TX3Bias: 6.7500mA
                TX4Bias: 6.7500mA
        ChannelThresholdValues:
                RxPowerHighAlarm  : 3.4001dBm
                RxPowerHighWarning: 2.4000dBm
                RxPowerLowAlarm   : -13.5067dBm
                RxPowerLowWarning : -9.5001dBm
                TxBiasHighAlarm   : 10.0000mA
                TxBiasHighWarning : 9.5000mA
                TxBiasLowAlarm    : 0.5000mA
                TxBiasLowWarning  : 1.0000mA
        ModuleMonitorValues:
                Temperature: 30.9258C
                Vcc: 3.2824Volts
        ModuleThresholdValues:
                TempHighAlarm  : 75.0000C
                TempHighWarning: 70.0000C
                TempLowAlarm   : -5.0000C
                TempLowWarning : 0.0000C
                VccHighAlarm   : 3.6300Volts
                VccHighWarning : 3.4650Volts
                VccLowAlarm    : 2.9700Volts
                VccLowWarning  : 3.1349Volts"""


@pytest.mark.sonic_baseline
class TestInterfacesTransceiverEeprom:
    """
    Baseline test for 'show interfaces transceiver eeprom' (Cmd #28).

    Mirrors sonic-utilities test:
      File: sfp_test.py
      Func: test_sfp_eeprom_with_dom
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
