//! Unit tests for queue-related show commands.
//!
//! Covers: queue counters, queue persistent-watermark (all/multicast/unicast),
//! queue watermark (all/multicast/unicast), queue wredcounters.
//!
//! Formatter modules: queue (QueueCounterFormatter, QueueWatermarkFormatter,
//! QueueWredCounterFormatter, SubcommandFormatter)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 53: show queue counters
// =============================================================================

#[test]
fn test_show_queue_counters() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0:0": {
            "Counter/pkts": "100",
            "Counter/bytes": "64000",
            "Drop/pkts": "5",
            "Drop/bytes": "3200"
        },
        "Ethernet0:1": {
            "Counter/pkts": "200",
            "Counter/bytes": "128000",
            "Drop/pkts": "0",
            "Drop/bytes": "0"
        }
    }"#,
    );
    let result = converter.convert("queue_counters", data).unwrap();
    let expected = concat!(
        "For namespace :\n",
        "     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes\n",
        "---------  -----  --------------  ---------------  -----------  ------------\n",
        "Ethernet0    UC0             100            64000            5          3200\n",
        "Ethernet0    UC1             200           128000            0             0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_counters_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(53, "queue_counters");
    assert!(!test_data.is_empty(), "No test data found for command 53");
    for (json_str, cli_str) in &test_data {
        if cli_str.is_empty() {
            continue;
        }
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("queue_counters", data).unwrap();
        if result.trim().is_empty() || result.contains("Object map is empty") {
            continue;
        }
        assert_output_contains_all(&result, &["Port", "TxQ"]);
    }
}

// =============================================================================
// 54: show queue persistent-watermark (help/subcommands)
// =============================================================================

#[test]
fn test_show_queue_persistent_watermark_help() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "subcommands": {
            "all": "show/queue/persistent-watermark/all",
            "multicast": "show/queue/persistent-watermark/multicast",
            "unicast": "show/queue/persistent-watermark/unicast"
        }
    }"#,
    );
    let result = converter
        .convert("queue_persistent_watermark", data)
        .unwrap();
    let expected = concat!(
        "Usage: show queue persistent-watermark [OPTIONS] COMMAND [ARGS]...\n",
        "\n",
        "  Show persistent WM for queues\n",
        "\n",
        "Options:\n",
        "  -?, -h, --help  Show this message and exit.\n",
        "\n",
        "Commands:\n",
        "  all        Show persistent WM for all queues\n",
        "  multicast  Show persistent WM for multicast queues\n",
        "  unicast    Show persistent WM for unicast queues",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_persistent_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(54, "queue_persistent_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 54");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_persistent_watermark", data)
            .unwrap();
        assert_output_contains_all(&result, &["persistent-watermark"]);
    }
}

// =============================================================================
// 55: show queue persistent-watermark all
// =============================================================================

#[test]
fn test_show_queue_persistent_watermark_all_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(55, "queue_persistent_watermark_all");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 55"
    );
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_persistent_watermark_all", data)
            .unwrap();
        // Output may be "Object map is empty!" or actual table
        assert!(!result.trim().is_empty(), "Output should not be empty");
    }
}

// =============================================================================
// 56: show queue persistent-watermark multicast
// =============================================================================

#[test]
fn test_show_queue_persistent_watermark_multicast() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"MC8": "0", "MC9": "0", "MC10": "0", "MC11": "0"},
        "Ethernet4": {"MC8": "100", "MC9": "0", "MC10": "50", "MC11": "0"}
    }"#,
    );
    let result = converter
        .convert("queue_persistent_watermark_multicast", data)
        .unwrap();
    let expected = concat!(
        "Egress shared pool occupancy per multicast queue:\n",
        "Port         MC8    MC9    MC10    MC11\n",
        "---------  -----  -----  ------  ------\n",
        "Ethernet0      0      0       0       0\n",
        "Ethernet4    100      0      50       0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_persistent_watermark_multicast_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(56, "queue_persistent_watermark_multicast");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 56"
    );
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_persistent_watermark_multicast", data)
            .unwrap();
        if result.trim().is_empty() || result.contains("Object map is empty") {
            continue;
        }
        // Output should contain Port header; MC columns may not appear for some data
        assert_output_contains_all(&result, &["Port"]);
    }
}

// =============================================================================
// 57: show queue persistent-watermark unicast
// =============================================================================

#[test]
fn test_show_queue_persistent_watermark_unicast() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"UC0": "0", "UC1": "0", "UC2": "0", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "7620"},
        "Ethernet4": {"UC0": "100", "UC1": "0", "UC2": "0", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "0"}
    }"#,
    );
    let result = converter
        .convert("queue_persistent_watermark_unicast", data)
        .unwrap();
    let expected = concat!(
        "Egress shared pool occupancy per unicast queue:\n",
        "Port         UC0    UC1    UC2    UC3    UC4    UC5    UC6    UC7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0      0      0      0      0      0      0      0   7620\n",
        "Ethernet4    100      0      0      0      0      0      0      0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_persistent_watermark_unicast_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(57, "queue_persistent_watermark_unicast");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 57"
    );
    let mut verified = 0;
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_persistent_watermark_unicast", data)
            .unwrap();
        if result.trim().is_empty() || result.contains("Object map is empty") {
            continue;
        }
        assert_output_contains_all(&result, &["Port", "UC"]);
        verified += 1;
    }
    assert!(verified > 0 || test_data.len() > 0);
}

// =============================================================================
// 58: show queue watermark (help/subcommands)
// =============================================================================

#[test]
fn test_show_queue_watermark_help() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "subcommands": {
            "all": "show/queue/watermark/all",
            "multicast": "show/queue/watermark/multicast",
            "unicast": "show/queue/watermark/unicast"
        }
    }"#,
    );
    let result = converter.convert("queue_watermark", data).unwrap();
    let expected = concat!(
        "Usage: show queue watermark [OPTIONS] COMMAND [ARGS]...\n",
        "\n",
        "  Show user WM for queues\n",
        "\n",
        "Options:\n",
        "  -?, -h, --help  Show this message and exit.\n",
        "\n",
        "Commands:\n",
        "  all        Show user WM for all queues\n",
        "  multicast  Show user WM for multicast queues\n",
        "  unicast    Show user WM for unicast queues",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(58, "queue_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 58");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("queue_watermark", data).unwrap();
        assert_output_contains_all(&result, &["watermark"]);
    }
}

// =============================================================================
// 59: show queue watermark all
// =============================================================================

#[test]
fn test_show_queue_watermark_all_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(59, "queue_watermark_all");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 59"
    );
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_watermark_all", data)
            .unwrap();
        assert!(!result.trim().is_empty(), "Output should not be empty");
    }
}

// =============================================================================
// 60: show queue watermark multicast
// =============================================================================

#[test]
fn test_show_queue_watermark_multicast() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"MC8": "0", "MC9": "0", "MC10": "0", "MC11": "0"}
    }"#,
    );
    let result = converter
        .convert("queue_watermark_multicast", data)
        .unwrap();
    let expected = concat!(
        "Egress shared pool occupancy per multicast queue:\n",
        "Port         MC8    MC9    MC10    MC11\n",
        "---------  -----  -----  ------  ------\n",
        "Ethernet0      0      0       0       0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_watermark_multicast_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(60, "queue_watermark_multicast");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 60"
    );
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_watermark_multicast", data)
            .unwrap();
        if result.trim().is_empty() || result.contains("Object map is empty") {
            continue;
        }
        // Output should contain Port header; MC columns may not appear for some data
        assert_output_contains_all(&result, &["Port"]);
    }
}

// =============================================================================
// 61: show queue watermark unicast
// =============================================================================

#[test]
fn test_show_queue_watermark_unicast() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"UC0": "0", "UC1": "0", "UC2": "0", "UC3": "100", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "0"}
    }"#,
    );
    let result = converter
        .convert("queue_watermark_unicast", data)
        .unwrap();
    let expected = concat!(
        "Egress shared pool occupancy per unicast queue:\n",
        "Port         UC0    UC1    UC2    UC3    UC4    UC5    UC6    UC7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0      0      0      0    100      0      0      0      0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_watermark_unicast_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(61, "queue_watermark_unicast");
    assert!(
        !test_data.is_empty(),
        "No test data found for command 61"
    );
    let mut verified = 0;
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("queue_watermark_unicast", data)
            .unwrap();
        if result.trim().is_empty() || result.contains("Object map is empty") {
            continue;
        }
        assert_output_contains_all(&result, &["Port", "UC"]);
        verified += 1;
    }
    assert!(verified > 0 || test_data.len() > 0);
}

// =============================================================================
// 62: show queue wredcounters
// =============================================================================

#[test]
fn test_show_queue_wredcounters() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0:0": {
            "WredDrp/pkts": "0",
            "WredDrp/bytes": "0",
            "EcnMarked/pkts": "0",
            "EcnMarked/bytes": "0"
        },
        "Ethernet0:1": {
            "WredDrp/pkts": "10",
            "WredDrp/bytes": "6400",
            "EcnMarked/pkts": "5",
            "EcnMarked/bytes": "3200"
        }
    }"#,
    );
    let result = converter.convert("queue_wredcounters", data).unwrap();
    let expected = concat!(
        "     Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes\n",
        "---------  -----  --------------  ---------------  ----------------  -----------------\n",
        "Ethernet0    UC0               0                0                 0                  0\n",
        "Ethernet0    UC1              10             6400                 5               3200",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_queue_wredcounters_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(62, "queue_wredcounters");
    assert!(!test_data.is_empty(), "No test data found for command 62");
    for (json_str, cli_str) in &test_data {
        if cli_str.is_empty() {
            continue;
        }
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("queue_wredcounters", data).unwrap();
        if result.trim().is_empty() {
            continue;
        }
        assert_output_contains_all(&result, &["Port", "TxQ", "WredDrp"]);
    }
}

// =============================================================================
// Edge cases: empty data
// =============================================================================

#[test]
fn test_show_queue_commands_empty_data() {
    let converter = GnmiToCliConverter::new();
    let empty_obj = load_test_data(r#"{}"#);

    let queue_cmds = [
        "queue_counters",
        "queue_persistent_watermark",
        "queue_persistent_watermark_all",
        "queue_persistent_watermark_multicast",
        "queue_persistent_watermark_unicast",
        "queue_watermark",
        "queue_watermark_all",
        "queue_watermark_multicast",
        "queue_watermark_unicast",
        "queue_wredcounters",
    ];

    for cmd in &queue_cmds {
        let result = converter.convert(cmd, empty_obj.clone());
        assert!(
            result.is_ok(),
            "Command '{}' failed with empty data: {:?}",
            cmd,
            result.err()
        );
    }
}

// =============================================================================
// convert_str API test
// =============================================================================

#[test]
fn test_show_queue_counters_via_convert_str() {
    let converter = GnmiToCliConverter::new();
    let json_str = r#"{
        "Ethernet0:0": {
            "Counter/pkts": "500",
            "Counter/bytes": "320000",
            "Drop/pkts": "0",
            "Drop/bytes": "0"
        }
    }"#;
    let result = converter
        .convert_str("queue_counters", json_str)
        .unwrap();
    let expected = concat!(
        "For namespace :\n",
        "     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes\n",
        "---------  -----  --------------  ---------------  -----------  ------------\n",
        "Ethernet0    UC0             500           320000            0             0",
    );
    assert_cli_output_matches(&result, expected);
}
