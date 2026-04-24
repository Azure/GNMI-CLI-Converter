//! Integration tests for intermediate (non-leaf) show commands.
//!
//! An intermediate command (e.g. `show processes`, `show reboot-cause`,
//! `show interfaces transceiver`) is a path prefix that has children but is
//! itself queryable via gNMI. The server returns a click-style
//! "subcommand-discovery" payload which the client renders via the
//! `command_help` formatter.
//!
//! The list below mirrors the real device capture.
use gnmi_cli_lib::parser::CommandParser;
use gnmi_cli_lib::registry::create_default_registry;
use serde_json::json;

/// (cli, gnmi_path, category, subcommand, sample_subs)
fn intermediates() -> Vec<(&'static str, &'static str, &'static str, &'static str, Vec<&'static str>)>
{
    vec![
        // Single-segment intermediates.
        ("show processes", "SHOW/processes", "processes", "processes",
         vec!["cpu", "memory", "summary"]),
        ("show buffer_pool", "SHOW/buffer_pool", "buffer_pool",
         "buffer_pool", vec!["watermark", "persistent-watermark"]),
        ("show headroom-pool", "SHOW/headroom-pool", "headroom-pool",
         "headroom-pool", vec!["watermark", "persistent-watermark"]),
        ("show priority-group", "SHOW/priority-group", "priority-group",
         "priority-group", vec!["drop", "watermark", "persistent-watermark"]),
        ("show queue", "SHOW/queue", "queue", "queue",
         vec!["counters", "watermark", "persistent-watermark", "wredcounters"]),
        ("show dropcounters", "SHOW/dropcounters", "dropcounters",
         "dropcounters",
         vec!["capabilities", "configuration", "counts", "persistent-drops"]),
        ("show watermark", "SHOW/watermark", "watermark", "watermark",
         vec!["telemetry"]),
        ("show lldp", "SHOW/lldp", "lldp", "lldp",
         vec!["neighbors", "table"]),
        ("show vlan", "SHOW/vlan", "vlan", "vlan",
         vec!["brief", "config"]),
        ("show interfaces", "SHOW/interfaces", "interfaces", "interfaces",
         vec!["alias", "counters", "description", "status", "transceiver"]),
        ("show srv6", "SHOW/srv6", "srv6", "srv6", vec!["stats"]),
        ("show ipv6", "SHOW/ipv6", "ipv6", "ipv6",
         vec!["bgp", "fib", "interfaces", "route", "protocol"]),
        // Multi-segment intermediates.
        ("show queue watermark", "SHOW/queue/watermark",
         "queue", "watermark", vec!["all", "multicast", "unicast"]),
        ("show queue persistent-watermark",
         "SHOW/queue/persistent-watermark",
         "queue", "persistent-watermark",
         vec!["all", "multicast", "unicast"]),
        ("show priority-group watermark",
         "SHOW/priority-group/watermark", "priority-group", "watermark",
         vec!["headroom", "shared"]),
        ("show priority-group persistent-watermark",
         "SHOW/priority-group/persistent-watermark",
         "priority-group", "persistent-watermark",
         vec!["headroom", "shared"]),
        ("show interfaces neighbor", "SHOW/interfaces/neighbor",
         "interfaces", "neighbor", vec!["expected"]),
        ("show interfaces fec", "SHOW/interfaces/fec",
         "interfaces", "fec", vec!["status"]),
        ("show interfaces transceiver", "SHOW/interfaces/transceiver",
         "interfaces", "transceiver",
         vec!["eeprom", "info", "lpmode", "pm", "presence", "status"]),
        ("show interfaces switchport", "SHOW/interfaces/switchport",
         "interfaces", "switchport", vec!["config", "status"]),
        ("show ipv6 bgp", "SHOW/ipv6/bgp", "ipv6", "bgp",
         vec!["neighbors", "network", "summary"]),
    ]
}

#[test]
fn test_all_intermediates_registered_as_command_help() {
    let registry = create_default_registry();
    for (cli, _gnmi, category, subcommand, _subs) in intermediates() {
        let key = format!("{}_{}", category, subcommand.replace('-', "_"));
        let config = registry.get(&key).unwrap_or_else(|| {
            panic!("{cli}: registry key {key:?} not registered")
        });
        assert_eq!(
            config.formatter(),
            Some("command_help"),
            "{cli}: expected command_help formatter, got {:?}",
            config.formatter()
        );
    }
}

#[test]
fn test_all_intermediates_have_cli_mapping() {
    let registry = create_default_registry();
    let mappings = registry.list_cli_mappings();
    for (cli, gnmi, category, subcommand, _subs) in intermediates() {
        let mapping = mappings
            .get(cli)
            .unwrap_or_else(|| panic!("{cli}: missing CliMapping"));
        assert_eq!(mapping.gnmi_path, gnmi, "{cli}: wrong gnmi_path");
        assert_eq!(mapping.command, category);
        assert_eq!(mapping.subcommand, subcommand);
    }
}

#[test]
fn test_all_intermediates_parse_correctly() {
    let parser = CommandParser::new();
    for (cli, _gnmi, category, subcommand, _subs) in intermediates() {
        let parsed = parser.parse(cli);
        assert!(parsed.is_valid, "{cli}: {:?}", parsed.error_message);
        assert_eq!(parsed.command, category, "{cli}: wrong command");
        assert_eq!(parsed.subcommand, subcommand, "{cli}: wrong subcommand");
        assert!(parsed.argument.is_none(), "{cli}: unexpected argument");
    }
}

#[test]
fn test_all_intermediates_format_to_usage_block() {
    let parser = CommandParser::new();
    for (cli, _gnmi, category, subcommand, subs) in intermediates() {
        let data = json!({
            "subcommands": subs.iter()
                .map(|s| (s.to_string(), serde_json::Value::String(String::new())))
                .collect::<serde_json::Map<_, _>>()
        });
        let out = parser
            .convert(category, subcommand, data, None)
            .unwrap_or_else(|e| panic!("{cli}: convert failed: {e}"));
        assert!(!out.is_empty(), "{cli}: empty output");
        assert!(out.contains("Usage:"), "{cli}: missing Usage block:\n{out}");
        assert!(out.contains("Commands:"), "{cli}: missing Commands block:\n{out}");
        for sub in &subs {
            assert!(
                out.contains(sub),
                "{cli}: subcommand {sub:?} missing in:\n{out}"
            );
        }
    }
}

/// Leaf commands beneath intermediate parents must still route to their
/// correct compound-subcommand keys even after the parent was added to
/// COMMANDS_WITH_OPTIONAL_SUBCOMMAND.
#[test]
fn test_leaf_under_intermediate_routes_to_compound_key() {
    let parser = CommandParser::new();
    let cases: &[(&str, &str, Option<&str>)] = &[
        // interfaces compound leafs
        ("show interfaces counters detailed", "counters-detailed", None),
        ("show interfaces transceiver info", "transceiver-info", None),
        ("show interfaces fec status", "fec-status", None),
        ("show interfaces switchport config", "switchport-config", None),
        ("show interfaces neighbor expected", "neighbor-expected", None),
        ("show interfaces status Ethernet0", "status", Some("Ethernet0")),
        // queue compound leafs
        ("show queue watermark unicast", "watermark-unicast", None),
        ("show queue persistent-watermark multicast",
            "persistent-watermark-multicast", None),
        ("show queue counters Ethernet8", "counters", Some("Ethernet8")),
        // priority-group compound leafs
        ("show priority-group watermark shared", "watermark-shared", None),
        ("show priority-group persistent-watermark headroom",
            "persistent-watermark-headroom", None),
        // ipv6 compound leafs
        ("show ipv6 bgp summary", "bgp-summary", None),
        ("show ipv6 prefix-list", "prefix-list", None),
        ("show ipv6 link-local-mode", "link-local-mode", None),
        // watermark leaf with positional argument
        ("show watermark telemetry interval", "telemetry", Some("interval")),
        // Simple single-segment leafs under intermediate parents
        ("show buffer_pool watermark", "watermark", None),
        ("show headroom-pool persistent-watermark",
            "persistent-watermark", None),
    ];
    for (cli, expected_sub, expected_arg) in cases {
        let parsed = parser.parse(cli);
        assert!(parsed.is_valid, "{cli}: {:?}", parsed.error_message);
        assert_eq!(
            parsed.subcommand, *expected_sub,
            "{cli}: got subcommand={:?}", parsed.subcommand
        );
        assert_eq!(
            parsed.argument.as_deref(), *expected_arg,
            "{cli}: got argument={:?}", parsed.argument
        );
    }
}
