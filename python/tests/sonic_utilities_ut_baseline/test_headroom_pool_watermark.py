"""
Baseline UT: show headroom-pool watermark (Cmd #07)

Source:
  - sonic-utilities UT File : watermarkstat_test.py
  - sonic-utilities Test Func: test_show_headroom_pool_wm
  - Expected CLI Output Var : show_hdrm_pool_wm_output (from wm_test_vectors.py)
  - Mapping Doc Section     : ### #7

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show headroom-pool watermark"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "ingress_lossless_pool": {
        "Bytes": "432640"
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool   432640"""


@pytest.mark.sonic_baseline
class TestHeadroomPoolWatermark:
    """
    Baseline test for 'show headroom-pool watermark' (Cmd #07).

    Mirrors sonic-utilities test:
      File: watermarkstat_test.py
      Func: test_show_headroom_pool_wm
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
