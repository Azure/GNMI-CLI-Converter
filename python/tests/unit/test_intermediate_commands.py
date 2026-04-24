"""
Unit tests for intermediate (non-leaf) show commands.

An intermediate command (e.g. ``show processes``, ``show reboot-cause``,
``show interfaces transceiver``) is a path prefix that has children but is
itself queryable via gNMI. The server returns a click-style
"subcommand-discovery" payload which the client renders with the
``command_help`` formatter.

These tests verify that every intermediate captured from the real device
run is:

1. Registered in the command registry (``CommandRegistry.has_command``).
2. Mapped to the ``command_help`` output type.
3. Reachable via the CLI mapping table (``get_cli_mapping``).
4. Parsed correctly by ``CommandParser`` with subcommand / argument as
   appropriate.
5. Rendered to a usable click-style help string with "Usage:" + "Commands:"
   sections and a line per discovered subcommand.
"""

import pytest

from gnmi_cli_lib.parser import CommandParser
from gnmi_cli_lib.registry import get_default_registry


# (cli_command, command, subcommand, sample_subcommands)
# sample_subcommands comes from real device output.
INTERMEDIATES = [
    ("show buffer_pool", "buffer_pool", "buffer_pool",
     ["watermark", "persistent-watermark"]),
    ("show headroom-pool", "headroom-pool", "headroom-pool",
     ["watermark", "persistent-watermark"]),
    ("show priority-group", "priority-group", "priority-group",
     ["drop", "persistent-watermark", "watermark"]),
    ("show queue", "queue", "queue",
     ["counters", "persistent-watermark", "watermark", "wredcounters"]),
    ("show dropcounters", "dropcounters", "dropcounters",
     ["capabilities", "configuration", "counts", "persistent-drops"]),
    ("show watermark", "watermark", "watermark", ["telemetry"]),
    ("show lldp", "lldp", "lldp", ["neighbors", "table"]),
    ("show vlan", "vlan", "vlan", ["brief", "config"]),
    ("show interfaces", "interfaces", "interfaces",
     ["alias", "counters", "description", "status", "transceiver"]),
    ("show srv6", "srv6", "srv6", ["stats"]),
    ("show ipv6", "ipv6", "ipv6",
     ["bgp", "fib", "interfaces", "route", "protocol"]),
    ("show priority-group watermark", "priority-group", "watermark",
     ["headroom", "shared"]),
    ("show priority-group persistent-watermark", "priority-group",
     "persistent-watermark", ["headroom", "shared"]),
    ("show interfaces neighbor", "interfaces", "neighbor", ["expected"]),
    ("show interfaces fec", "interfaces", "fec", ["status"]),
    ("show interfaces transceiver", "interfaces", "transceiver",
     ["eeprom", "info", "lpmode", "pm", "presence", "status"]),
    ("show interfaces switchport", "interfaces", "switchport",
     ["config", "status"]),
    ("show ipv6 bgp", "ipv6", "bgp", ["neighbors", "network", "summary"]),
]


@pytest.mark.parametrize(
    "cli,command,subcommand,subs",
    INTERMEDIATES,
    ids=[item[0] for item in INTERMEDIATES],
)
class TestIntermediateCommands:
    """Parametrized over every intermediate command."""

    def test_registered_as_command_help(self, cli, command, subcommand, subs):
        """Intermediate is registered with output_type='command_help'."""
        registry = get_default_registry()
        assert registry.has_command(command, subcommand), (
            f"{cli}: ({command!r}, {subcommand!r}) not registered"
        )
        config = registry.get_config(command, subcommand)
        assert config.output_type == "command_help", (
            f"{cli}: expected command_help formatter, got {config.output_type}"
        )

    def test_cli_mapping_exists(self, cli, command, subcommand, subs):
        """CliMapping exists and points to the intermediate's gNMI path."""
        registry = get_default_registry()
        mapping = registry.get_cli_mapping(cli)
        # The gNMI path should mirror the CLI with SHOW/ prefix and / joins.
        parts = cli.split()
        assert parts[0] == "show"
        expected_path = "SHOW/" + "/".join(parts[1:])
        assert mapping.gnmi_path == expected_path, (
            f"{cli}: gnmi_path {mapping.gnmi_path!r} != {expected_path!r}"
        )
        assert mapping.command == command
        assert mapping.subcommand == subcommand

    def test_parser_routes_to_intermediate(self, cli, command, subcommand, subs):
        """The parser resolves the CLI string to (command, subcommand)."""
        parser = CommandParser()
        parsed = parser.parse(cli)
        assert parsed.is_valid, (
            f"{cli}: parse invalid: {parsed.error_message}"
        )
        assert parsed.command == command
        assert parsed.subcommand == subcommand
        assert parsed.argument is None

    def test_formatter_renders_usage_block(
        self, cli, command, subcommand, subs
    ):
        """Rendering a subcommands payload produces a click-like help block."""
        parser = CommandParser()
        data = {"subcommands": {s: "" for s in subs}}
        output = parser.convert(command, subcommand, data)
        assert output, f"{cli}: empty output"
        assert "Usage:" in output
        assert "Commands:" in output
        assert "Options:" in output
        # Each discovered subcommand should be listed.
        for sub in subs:
            assert sub in output, (
                f"{cli}: subcommand {sub!r} missing from:\n{output}"
            )


class TestIntermediateParserLongerCommands:
    """Leaf commands under intermediate parents must still parse correctly.

    Regression guard: enabling the ``(parent, parent)`` self-ref puts the
    parent into ``COMMANDS_WITH_OPTIONAL_SUBCOMMAND``. The parser must still
    route longer commands (compound / leaf subcommands) to the correct leaf
    key rather than fall back to the intermediate self-ref.
    """

    @pytest.mark.parametrize(
        "cli,expected_sub,expected_arg",
        [
            # interfaces — compound leaf subcommands
            ("show interfaces counters detailed", "counters-detailed", None),
            ("show interfaces transceiver info",  "transceiver-info",  None),
            ("show interfaces fec status",        "fec-status",        None),
            ("show interfaces switchport config", "switchport-config", None),
            ("show interfaces neighbor expected", "neighbor-expected", None),
            ("show interfaces status Ethernet0",  "status",            "Ethernet0"),
            # queue — compound leaf subcommands
            ("show queue watermark unicast",          "watermark-unicast",          None),
            ("show queue persistent-watermark multicast", "persistent-watermark-multicast", None),
            ("show queue counters Ethernet8",         "counters",                   "Ethernet8"),
            # priority-group — compound leaf subcommands
            ("show priority-group watermark shared",           "watermark-shared",           None),
            ("show priority-group persistent-watermark headroom", "persistent-watermark-headroom", None),
            # ipv6 — compound leaf subcommands
            ("show ipv6 bgp summary",    "bgp-summary",    None),
            ("show ipv6 prefix-list",    "prefix-list",    None),
            ("show ipv6 link-local-mode", "link-local-mode", None),
            # watermark — leaf with positional argument
            ("show watermark telemetry interval", "telemetry", "interval"),
            # buffer_pool / headroom-pool leaf subcommands
            ("show buffer_pool watermark",         "watermark",            None),
            ("show headroom-pool persistent-watermark", "persistent-watermark", None),
        ],
    )
    def test_parser_resolves_leaf_under_intermediate(
        self, cli, expected_sub, expected_arg
    ):
        parser = CommandParser()
        parsed = parser.parse(cli)
        assert parsed.is_valid, f"{cli}: {parsed.error_message}"
        assert parsed.subcommand == expected_sub, (
            f"{cli}: got subcommand={parsed.subcommand!r}, "
            f"expected {expected_sub!r}"
        )
        assert parsed.argument == expected_arg, (
            f"{cli}: got argument={parsed.argument!r}, "
            f"expected {expected_arg!r}"
        )


class TestIntermediateFormatterDeviceOutput:
    """Smoke-check the rendered output against the shape of the real-device
    click-style help (from P0 capture)."""

    def test_show_interfaces_transceiver_lists_all_subs(self):
        """show interfaces transceiver must list all 7 transceiver subs."""
        parser = CommandParser()
        real_subs = [
            "eeprom", "error-status", "info", "lpmode", "pm",
            "presence", "status",
        ]
        data = {"subcommands": {s: "" for s in real_subs}}
        out = parser.convert("interfaces", "transceiver", data)
        for s in real_subs:
            assert f"  {s}" in out or s in out, (
                f"subcommand {s} missing from:\n{out}"
            )

    def test_show_ipv6_bgp_lists_summary(self):
        parser = CommandParser()
        data = {"subcommands": {
            "neighbors": "", "network": "", "summary": "", "vrf": "",
        }}
        out = parser.convert("ipv6", "bgp", data)
        assert "summary" in out
        assert "neighbors" in out

    def test_show_interfaces_bare_rejects_legacy_typo_via_intermediate(self):
        """`show interfaces invalid` now falls through to the intermediate
        (self-ref) with the bogus token carried as argument."""
        parser = CommandParser()
        parsed = parser.parse("show interfaces bogus_token")
        assert parsed.is_valid
        assert parsed.subcommand == "interfaces"
        assert parsed.argument == "bogus_token"
