"""
Baseline UT: show mmu (Cmd #48)

Source:
  - sonic-utilities UT File : mmuconfig_test.py
  - sonic-utilities Test Func: TestMmuConfig
  - Expected CLI Output Var : show_mmu_config (from mmuconfig_input/mmuconfig_test_vectors.py)
  - Mapping Doc Section     : ### #48

Verification Status: VERIFIED
"""

import json
import pytest

# -- CLI Command --
CLI_COMMAND = "show mmu"

# -- JSON Input (from mapping doc, reverse-engineered from sonic-utilities UT mock data) --
JSON_INPUT = {
    "lossless_traffic_pattern": {
        "default_dynamic_th": "0",
        "over_subscribe_ratio": "2"
    },
    "pools": {
        "egress_lossless_pool": {
            "mode": "dynamic",
            "size": "13945824",
            "type": "egress"
        },
        "egress_lossy_pool": {
            "mode": "dynamic",
            "type": "egress"
        },
        "ingress_lossless_pool": {
            "mode": "dynamic",
            "type": "ingress"
        },
        "ingress_lossy_pool": {
            "mode": "dynamic",
            "type": "ingress"
        },
        "ingress_lossless_pool_hbm": {
            "mode": "static",
            "size": "139458240",
            "type": "ingress"
        }
    },
    "profiles": {
        "ingress_lossy_profile": {
            "dynamic_th": "3",
            "pool": "ingress_lossy_pool",
            "size": "0"
        },
        "ingress_lossless_profile_hbm": {
            "static_th": "12121212",
            "pool": "ingress_lossless_pool_hbm",
            "size": "0"
        },
        "headroom_profile": {
            "dynamic_th": "0",
            "pool": "ingress_lossless_pool",
            "xon": "18432",
            "xoff": "32768",
            "size": "51200"
        },
        "alpha_profile": {
            "dynamic_th": "0",
            "pool": "ingress_lossless_pool",
            "headroom_type": "dynamic"
        },
        "egress_lossless_profile": {
            "dynamic_th": "0",
            "pool": "egress_lossless_pool",
            "size": "0"
        },
        "egress_lossy_profile": {
            "dynamic_th": "0",
            "pool": "egress_lossy_pool",
            "size": "0"
        },
        "q_lossy_profile": {
            "packet_discard_action": "drop",
            "dynamic_th": "0",
            "pool": "egress_lossy_pool",
            "size": "0"
        }
    }
}

# -- Expected CLI Output (from sonic-utilities UT) --
EXPECTED_CLI_OUTPUT = """\
Lossless traffic pattern:
--------------------  -
default_dynamic_th    0
over_subscribe_ratio  2
--------------------  -

Pool: egress_lossless_pool
----  --------
mode  dynamic
size  13945824
type  egress
----  --------

Pool: egress_lossy_pool
----  -------
mode  dynamic
type  egress
----  -------

Pool: ingress_lossless_pool
----  -------
mode  dynamic
type  ingress
----  -------

Pool: ingress_lossy_pool
----  -------
mode  dynamic
type  ingress
----  -------

Pool: ingress_lossless_pool_hbm
----  ---------
mode  static
size  139458240
type  ingress
----  ---------

Profile: ingress_lossy_profile
----------  ------------------
dynamic_th  3
pool        ingress_lossy_pool
size        0
----------  ------------------

Profile: ingress_lossless_profile_hbm
---------  -------------------------
static_th  12121212
pool       ingress_lossless_pool_hbm
size       0
---------  -------------------------

Profile: headroom_profile
----------  ---------------------
dynamic_th  0
pool        ingress_lossless_pool
xon         18432
xoff        32768
size        51200
----------  ---------------------

Profile: alpha_profile
-------------  ---------------------
dynamic_th     0
pool           ingress_lossless_pool
headroom_type  dynamic
-------------  ---------------------

Profile: egress_lossless_profile
----------  --------------------
dynamic_th  0
pool        egress_lossless_pool
size        0
----------  --------------------

Profile: egress_lossy_profile
----------  -----------------
dynamic_th  0
pool        egress_lossy_pool
size        0
----------  -----------------

Profile: q_lossy_profile
---------------------  -----------------
packet_discard_action  drop
dynamic_th             0
pool                   egress_lossy_pool
size                   0
---------------------  -----------------"""


@pytest.mark.sonic_baseline
class TestMmu:
    """
    Baseline test for 'show mmu' (Cmd #48).

    Mirrors sonic-utilities test:
      File: mmuconfig_test.py
      Func: TestMmuConfig
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
