"""
Baseline UT: show buffer_pool persistent-watermark (Cmd #02)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_buffer_pool_persistent_wm
  - Expected CLI Output Var : show_buffer_pool_persistent_wm_output (from wm_test_vectors.py)
  - Mapping Doc Section     : ### #2

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show buffer_pool persistent-watermark"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "egress_lossless_pool": {
        "Bytes": "2000"
    },
    "egress_lossy_pool": {
        "Bytes": "3000"
    },
    "ingress_lossless_pool": {
        "Bytes": "4000"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool     2000
    egress_lossy_pool     3000
ingress_lossless_pool     4000"""


@pytest.mark.sonic_baseline
class TestBufferPoolPersistentWatermark:
    """
    Baseline test for 'show buffer_pool persistent-watermark' (Cmd #02).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_buffer_pool_persistent_wm
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
