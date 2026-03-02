//! Unit tests for watermark-related show commands.
//!
//! Covers: buffer_pool watermark/persistent-watermark, headroom_pool watermark/persistent-watermark,
//! priority_group watermark/persistent-watermark (shared/headroom), watermark telemetry interval.
//!
//! Formatter modules: watermark (BufferPoolFormatter, HeadroomPoolFormatter, QueueWatermarkFormatter,
//! WatermarkTelemetryFormatter)

mod common;

use common::{
    assert_cli_output_matches, assert_output_contains_all, load_test_data,
    load_verifying_test_data, normalize_output,
};
use gnmi_cli_lib::converter::GnmiToCliConverter;
use serde_json::Value;

// =============================================================================
// 01: show buffer_pool watermark
// =============================================================================

#[test]
fn test_show_buffer_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    // Mock data from sonic-utilities wm_test_vectors.py pattern
    let data = load_test_data(
        r#"{
        "egress_lossless_pool": {"Bytes": "0"},
        "ingress_lossless_pool": {"Bytes": "0"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    // Expected output matches sonic-utilities watermarkstat format
    let expected = "\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool        0
ingress_lossless_pool        0";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_watermark_with_values() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "egress_lossless_pool": {"Bytes": "1000"},
        "egress_lossy_pool": {"Bytes": "2000"},
        "ingress_lossless_pool": {"Bytes": "3000"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    let expected = concat!(
        "Shared pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        " egress_lossless_pool     1000\n",
        "    egress_lossy_pool     2000\n",
        "ingress_lossless_pool     3000",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(1, "buffer_pool_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 01");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("buffer_pool_watermark", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

#[test]
fn test_show_buffer_pool_watermark_empty() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{}"#);
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    let _ = normalize_output(&result);
}

#[test]
fn test_show_buffer_pool_watermark_with_zero_pool() {
    let converter = GnmiToCliConverter::new();
    // Some platforms have ingress_zero_pool
    let data = load_test_data(
        r#"{
        "egress_lossless_pool": {"Bytes": "0"},
        "egress_lossy_pool": {"Bytes": "0"},
        "ingress_lossless_pool": {"Bytes": "59520"},
        "ingress_zero_pool": {"Bytes": "0"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    let expected = concat!(
        "Shared pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        " egress_lossless_pool        0\n",
        "    egress_lossy_pool        0\n",
        "ingress_lossless_pool    59520\n",
        "    ingress_zero_pool        0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_watermark_lossy_only() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "egress_lossy_pool": {"Bytes": "28448"},
        "ingress_lossy_pool": {"Bytes": "0"}
    }"#,
    );
    let result = converter.convert("buffer_pool_watermark", data).unwrap();
    let expected = concat!(
        "Shared pool maximum occupancy:\n",
        "              Pool    Bytes\n",
        "------------------  -------\n",
        " egress_lossy_pool    28448\n",
        "ingress_lossy_pool        0",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_watermark_via_convert_str() {
    let converter = GnmiToCliConverter::new();
    let json_str =
        r#"{"egress_lossy_pool":{"Bytes":"39936"},"ingress_lossless_pool":{"Bytes":"21888"}}"#;
    let result = converter
        .convert_str("buffer_pool_watermark", json_str)
        .unwrap();
    let expected = concat!(
        "Shared pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        "    egress_lossy_pool    39936\n",
        "ingress_lossless_pool    21888",
    );
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 02: show buffer_pool persistent-watermark
// =============================================================================

#[test]
fn test_show_buffer_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "egress_lossless_pool": {"Bytes": "0"},
        "ingress_lossless_pool": {"Bytes": "0"}
    }"#,
    );
    let result = converter
        .convert("buffer_pool_persistent_watermark", data)
        .unwrap();
    let expected = "\
Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool        0
ingress_lossless_pool        0";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_persistent_watermark_with_values() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "egress_lossless_pool": {"Bytes": "2000"},
        "egress_lossy_pool": {"Bytes": "3000"},
        "ingress_lossless_pool": {"Bytes": "4000"}
    }"#,
    );
    let result = converter
        .convert("buffer_pool_persistent_watermark", data)
        .unwrap();
    let expected = concat!(
        "Shared pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        " egress_lossless_pool     2000\n",
        "    egress_lossy_pool     3000\n",
        "ingress_lossless_pool     4000",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_buffer_pool_persistent_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(2, "buffer_pool_persistent_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 02");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("buffer_pool_persistent_watermark", data)
            .unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 07: show headroom-pool watermark
// =============================================================================

#[test]
fn test_show_headroom_pool_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"ingress_lossless_pool": {"Bytes": "0"}}"#);
    let result = converter.convert("headroom_pool_watermark", data).unwrap();
    let expected = "\
Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool        0";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_headroom_pool_watermark_with_values() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"ingress_lossless_pool": {"Bytes": "432640"}}"#);
    let result = converter.convert("headroom_pool_watermark", data).unwrap();
    let expected = concat!(
        "Headroom pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        "ingress_lossless_pool   432640",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_headroom_pool_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(7, "headroom_pool_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 07");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("headroom_pool_watermark", data).unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

#[test]
fn test_show_headroom_pool_watermark_via_convert_str() {
    let converter = GnmiToCliConverter::new();
    let json_str = r#"{"ingress_lossless_pool":{"Bytes":"432640"}}"#;
    let result = converter
        .convert_str("headroom_pool_watermark", json_str)
        .unwrap();
    let expected = concat!(
        "Headroom pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        "ingress_lossless_pool   432640",
    );
    assert_cli_output_matches(&result, expected);
}

// =============================================================================
// 08: show headroom-pool persistent-watermark
// =============================================================================

#[test]
fn test_show_headroom_pool_persistent_watermark() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"ingress_lossless_pool": {"Bytes": "0"}}"#);
    let result = converter
        .convert("headroom_pool_persistent_watermark", data)
        .unwrap();
    let expected = "\
Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool        0";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_headroom_pool_persistent_watermark_with_values() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"ingress_lossless_pool": {"Bytes": "863616"}}"#);
    let result = converter
        .convert("headroom_pool_persistent_watermark", data)
        .unwrap();
    let expected = concat!(
        "Headroom pool maximum occupancy:\n",
        "                 Pool    Bytes\n",
        "---------------------  -------\n",
        "ingress_lossless_pool   863616",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_headroom_pool_persistent_watermark_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(8, "headroom_pool_persistent_watermark");
    assert!(!test_data.is_empty(), "No test data found for command 08");
    for (json_str, cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter
            .convert("headroom_pool_persistent_watermark", data)
            .unwrap();
        assert_cli_output_matches(&result, cli_str);
    }
}

// =============================================================================
// 70: show watermark telemetry interval
// =============================================================================

#[test]
fn test_show_watermark_telemetry_interval() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(r#"{"interval": "120s"}"#);
    let result = converter.convert("watermark_telemetry", data).unwrap();
    let expected = "Telemetry interval 120 second(s)";
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_watermark_telemetry_interval_real_device() {
    let converter = GnmiToCliConverter::new();
    let test_data = load_verifying_test_data(70, "watermark_telemetry_interval");
    assert!(!test_data.is_empty(), "No test data found for command 70");
    for (json_str, _cli_str) in &test_data {
        let data: Value = serde_json::from_str(json_str).unwrap();
        let result = converter.convert("watermark_telemetry", data).unwrap();
        assert_output_contains_all(&result, &["Telemetry interval", "second"]);
    }
}

// =============================================================================
// Priority group watermark tests (from sonic-utilities wm_test_vectors.py)
// =============================================================================

#[test]
fn test_show_priority_group_watermark_shared() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103", "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"},
        "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402", "PG3": "403", "PG4": "404", "PG5": "405", "PG6": "406", "PG7": "407"},
        "Ethernet8": {"PG0": "800", "PG1": "801", "PG2": "802", "PG3": "803", "PG4": "804", "PG5": "805", "PG6": "806", "PG7": "807"}
    }"#,
    );
    let result = converter
        .convert("priority_group_watermark_shared", data)
        .unwrap();
    let expected = concat!(
        "Ingress shared pool occupancy per PG:\n",
        "Port         PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0    100    101    102    103    104    105    106    107\n",
        "Ethernet4    400    401    402    403    404    405    406    407\n",
        "Ethernet8    800    801    802    803    804    805    806    807",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_priority_group_watermark_headroom() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103", "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"},
        "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402", "PG3": "403", "PG4": "404", "PG5": "405", "PG6": "406", "PG7": "407"}
    }"#,
    );
    let result = converter
        .convert("priority_group_watermark_headroom", data)
        .unwrap();
    let expected = concat!(
        "Ingress headroom per PG:\n",
        "Port         PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0    100    101    102    103    104    105    106    107\n",
        "Ethernet4    400    401    402    403    404    405    406    407",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_priority_group_persistent_watermark_shared() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"PG0": "200", "PG1": "201", "PG2": "202", "PG3": "203", "PG4": "204", "PG5": "205", "PG6": "206", "PG7": "207"},
        "Ethernet4": {"PG0": "500", "PG1": "501", "PG2": "502", "PG3": "503", "PG4": "504", "PG5": "505", "PG6": "506", "PG7": "507"},
        "Ethernet8": {"PG0": "900", "PG1": "901", "PG2": "902", "PG3": "903", "PG4": "904", "PG5": "905", "PG6": "906", "PG7": "907"}
    }"#,
    );
    let result = converter
        .convert("priority_group_persistent_watermark_shared", data)
        .unwrap();
    let expected = concat!(
        "Ingress shared pool occupancy per PG:\n",
        "Port         PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0    200    201    202    203    204    205    206    207\n",
        "Ethernet4    500    501    502    503    504    505    506    507\n",
        "Ethernet8    900    901    902    903    904    905    906    907",
    );
    assert_cli_output_matches(&result, expected);
}

#[test]
fn test_show_priority_group_persistent_watermark_headroom() {
    let converter = GnmiToCliConverter::new();
    let data = load_test_data(
        r#"{
        "Ethernet0": {"PG0": "200", "PG1": "201", "PG2": "202", "PG3": "203", "PG4": "204", "PG5": "205", "PG6": "206", "PG7": "207"},
        "Ethernet4": {"PG0": "500", "PG1": "501", "PG2": "502", "PG3": "503", "PG4": "504", "PG5": "505", "PG6": "506", "PG7": "507"}
    }"#,
    );
    let result = converter
        .convert("priority_group_persistent_watermark_headroom", data)
        .unwrap();
    let expected = concat!(
        "Ingress headroom per PG:\n",
        "Port         PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7\n",
        "---------  -----  -----  -----  -----  -----  -----  -----  -----\n",
        "Ethernet0    200    201    202    203    204    205    206    207\n",
        "Ethernet4    500    501    502    503    504    505    506    507",
    );
    assert_cli_output_matches(&result, expected);
}
