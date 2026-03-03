
/// Source: `wm_input/wm_test_vectors.py` variable `show_pg_wm_shared_output`
#[allow(dead_code)]
pub const SHOW_PG_WM_SHARED_OUTPUT: &str = r#"Ingress shared pool occupancy per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    100    101    102    103    104    105    106    107
Ethernet4    400    401    402    403    404    405    406    407
Ethernet8    800    801    802    803    804    805    806    807
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_pg_wm_hdrm_output`
#[allow(dead_code)]
pub const SHOW_PG_WM_HDRM_OUTPUT: &str = r#"Ingress headroom per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    100    101    102    103    104    105    106    107
Ethernet4    400    401    402    403    404    405    406    407
Ethernet8    800    801    802    803    804    805    806    807
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_pg_persistent_wm_shared_output`
#[allow(dead_code)]
pub const SHOW_PG_PERSISTENT_WM_SHARED_OUTPUT: &str = r#"Ingress shared pool occupancy per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    200    201    202    203    204    205    206    207
Ethernet4    500    501    502    503    504    505    506    507
Ethernet8    900    901    902    903    904    905    906    907
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_pg_persistent_wm_hdrm_output`
#[allow(dead_code)]
pub const SHOW_PG_PERSISTENT_WM_HDRM_OUTPUT: &str = r#"Ingress headroom per PG:
     Port    PG0    PG1    PG2    PG3    PG4    PG5    PG6    PG7
---------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0    200    201    202    203    204    205    206    207
Ethernet4    500    501    502    503    504    505    506    507
Ethernet8    900    901    902    903    904    905    906    907
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_queue_wm_unicast_output`
#[allow(dead_code)]
pub const SHOW_QUEUE_WM_UNICAST_OUTPUT: &str = r#"Egress shared pool occupancy per unicast queue:
     Port      UC0      UC1    UC2    UC3    UC4    UC5    UC6    UC7    UC8    UC9
---------  -------  -------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0  2057328  2056704      0      0      0      0      0   2704    416     20
Ethernet4        0        0      0   1986   2567      0      0      0      0      0
Ethernet8        0        0   1040      0      0      0      0      0   8528   7696
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_queue_pwm_unicast_output`
#[allow(dead_code)]
pub const SHOW_QUEUE_PWM_UNICAST_OUTPUT: &str = r#"Egress shared pool occupancy per unicast queue:
     Port      UC0      UC1    UC2    UC3    UC4    UC5    UC6    UC7    UC8    UC9
---------  -------  -------  -----  -----  -----  -----  -----  -----  -----  -----
Ethernet0  3057328  3056704      0      0      0      0      0   3704    516     30
Ethernet4        0        0      0   2986   3567      0      0      0      0      0
Ethernet8        0        0   2040      0      0      0      0      0   9528   8696
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_queue_wm_multicast_output`
#[allow(dead_code)]
pub const SHOW_QUEUE_WM_MULTICAST_OUTPUT: &str = r#"Egress shared pool occupancy per multicast queue:
     Port    MC10    MC11    MC12    MC13    MC14    MC15    MC16    MC17    MC18    MC19
---------  ------  ------  ------  ------  ------  ------  ------  ------  ------  ------
Ethernet0     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A
Ethernet4     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A
Ethernet8     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A     N/A
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_queue_wm_all_output`
#[allow(dead_code)]
pub const SHOW_QUEUE_WM_ALL_OUTPUT: &str = r#"Egress shared pool occupancy per all queues:
     Port    ALL20    ALL21    ALL22    ALL23    ALL24    ALL25    ALL26    ALL27    ALL28    ALL29
---------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------
Ethernet0  1234567  7654321        0        0        0       20      500      200        0       10
Ethernet4        0        0        0     1986     2567        0        0        0        0        0
Ethernet8       20        5     1998        0        0        0        0        0     8528     7696
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_queue_pwm_all_output`
#[allow(dead_code)]
pub const SHOW_QUEUE_PWM_ALL_OUTPUT: &str = r#"Egress shared pool occupancy per all queues:
     Port    ALL20    ALL21    ALL22    ALL23    ALL24    ALL25    ALL26    ALL27    ALL28    ALL29
---------  -------  -------  -------  -------  -------  -------  -------  -------  -------  -------
Ethernet0      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A
Ethernet4      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A
Ethernet8      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A      N/A
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_buffer_pool_wm_output`
#[allow(dead_code)]
pub const SHOW_BUFFER_POOL_WM_OUTPUT: &str = r#"Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool     1000
    egress_lossy_pool     2000
ingress_lossless_pool     3000
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_buffer_pool_persistent_wm_output`
#[allow(dead_code)]
pub const SHOW_BUFFER_POOL_PERSISTENT_WM_OUTPUT: &str = r#"Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
 egress_lossless_pool     2000
    egress_lossy_pool     3000
ingress_lossless_pool     4000
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_hdrm_pool_wm_output`
#[allow(dead_code)]
pub const SHOW_HDRM_POOL_WM_OUTPUT: &str = r#"Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool   432640
"#;

/// Source: `wm_input/wm_test_vectors.py` variable `show_hdrm_pool_persistent_wm_output`
#[allow(dead_code)]
pub const SHOW_HDRM_POOL_PERSISTENT_WM_OUTPUT: &str = r#"Headroom pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool   863616
"#;

// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Source: wm_test_vectors.py `show_buffer_pool_wm_output`
pub const MOCK_BUFFER_POOL_WM: &str = r#"{
    "egress_lossless_pool": {"Bytes": "1000"},
    "egress_lossy_pool": {"Bytes": "2000"},
    "ingress_lossless_pool": {"Bytes": "3000"}
}"#;

/// Mock JSON for `show buffer_pool persistent-watermark` — 3 pools
/// Source: wm_test_vectors.py `show_buffer_pool_persistent_wm_output`
pub const MOCK_BUFFER_POOL_PWM: &str = r#"{
    "egress_lossless_pool": {"Bytes": "2000"},
    "egress_lossy_pool": {"Bytes": "3000"},
    "ingress_lossless_pool": {"Bytes": "4000"}
}"#;

/// Mock JSON for `show headroom-pool watermark`
/// Source: wm_test_vectors.py `show_hdrm_pool_wm_output`
pub const MOCK_HDRM_POOL_WM: &str = r#"{
    "ingress_lossless_pool": {"Bytes": "432640"}
}"#;

/// Mock JSON for `show headroom-pool persistent-watermark`
/// Source: wm_test_vectors.py `show_hdrm_pool_persistent_wm_output`
pub const MOCK_HDRM_POOL_PWM: &str = r#"{
    "ingress_lossless_pool": {"Bytes": "863616"}
}"#;

/// Mock JSON for `show priority-group watermark shared`
/// Source: wm_test_vectors.py `show_pg_wm_shared_output`
/// Ports: Ethernet0 (100-107), Ethernet4 (400-407), Ethernet8 (800-807)
pub const MOCK_PG_WM_SHARED: &str = r#"{
    "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103", "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"},
    "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402", "PG3": "403", "PG4": "404", "PG5": "405", "PG6": "406", "PG7": "407"},
    "Ethernet8": {"PG0": "800", "PG1": "801", "PG2": "802", "PG3": "803", "PG4": "804", "PG5": "805", "PG6": "806", "PG7": "807"}
}"#;

/// Mock JSON for `show priority-group watermark headroom`
/// Source: wm_test_vectors.py `show_pg_wm_hdrm_output`
pub const MOCK_PG_WM_HDRM: &str = r#"{
    "Ethernet0": {"PG0": "100", "PG1": "101", "PG2": "102", "PG3": "103", "PG4": "104", "PG5": "105", "PG6": "106", "PG7": "107"},
    "Ethernet4": {"PG0": "400", "PG1": "401", "PG2": "402", "PG3": "403", "PG4": "404", "PG5": "405", "PG6": "406", "PG7": "407"},
    "Ethernet8": {"PG0": "800", "PG1": "801", "PG2": "802", "PG3": "803", "PG4": "804", "PG5": "805", "PG6": "806", "PG7": "807"}
}"#;

/// Mock JSON for `show priority-group persistent-watermark shared`
/// Source: wm_test_vectors.py `show_pg_persistent_wm_shared_output`
pub const MOCK_PG_PWM_SHARED: &str = r#"{
    "Ethernet0": {"PG0": "200", "PG1": "201", "PG2": "202", "PG3": "203", "PG4": "204", "PG5": "205", "PG6": "206", "PG7": "207"},
    "Ethernet4": {"PG0": "500", "PG1": "501", "PG2": "502", "PG3": "503", "PG4": "504", "PG5": "505", "PG6": "506", "PG7": "507"},
    "Ethernet8": {"PG0": "900", "PG1": "901", "PG2": "902", "PG3": "903", "PG4": "904", "PG5": "905", "PG6": "906", "PG7": "907"}
}"#;

/// Mock JSON for `show priority-group persistent-watermark headroom`
/// Source: wm_test_vectors.py `show_pg_persistent_wm_hdrm_output`
pub const MOCK_PG_PWM_HDRM: &str = r#"{
    "Ethernet0": {"PG0": "200", "PG1": "201", "PG2": "202", "PG3": "203", "PG4": "204", "PG5": "205", "PG6": "206", "PG7": "207"},
    "Ethernet4": {"PG0": "500", "PG1": "501", "PG2": "502", "PG3": "503", "PG4": "504", "PG5": "505", "PG6": "506", "PG7": "507"},
    "Ethernet8": {"PG0": "900", "PG1": "901", "PG2": "902", "PG3": "903", "PG4": "904", "PG5": "905", "PG6": "906", "PG7": "907"}
}"#;

/// Mock JSON for `show queue watermark unicast`
/// Source: wm_test_vectors.py `show_queue_wm_unicast_output`
pub const MOCK_QUEUE_WM_UNICAST: &str = r#"{
    "Ethernet0": {"UC0": "2057328", "UC1": "2056704", "UC2": "0", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "2704", "UC8": "416", "UC9": "20"},
    "Ethernet4": {"UC0": "0", "UC1": "0", "UC2": "0", "UC3": "1986", "UC4": "2567", "UC5": "0", "UC6": "0", "UC7": "0", "UC8": "0", "UC9": "0"},
    "Ethernet8": {"UC0": "0", "UC1": "0", "UC2": "1040", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "0", "UC8": "8528", "UC9": "7696"}
}"#;

/// Mock JSON for `show queue persistent-watermark unicast`
/// Source: wm_test_vectors.py `show_queue_pwm_unicast_output`
pub const MOCK_QUEUE_PWM_UNICAST: &str = r#"{
    "Ethernet0": {"UC0": "3057328", "UC1": "3056704", "UC2": "0", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "3704", "UC8": "516", "UC9": "30"},
    "Ethernet4": {"UC0": "0", "UC1": "0", "UC2": "0", "UC3": "2986", "UC4": "3567", "UC5": "0", "UC6": "0", "UC7": "0", "UC8": "0", "UC9": "0"},
    "Ethernet8": {"UC0": "0", "UC1": "0", "UC2": "2040", "UC3": "0", "UC4": "0", "UC5": "0", "UC6": "0", "UC7": "0", "UC8": "9528", "UC9": "8696"}
}"#;

/// Mock JSON for `show queue watermark multicast`
/// Source: wm_test_vectors.py `show_queue_wm_multicast_output`
pub const MOCK_QUEUE_WM_MULTICAST: &str = r#"{
    "Ethernet0": {"MC10": "N/A", "MC11": "N/A", "MC12": "N/A", "MC13": "N/A", "MC14": "N/A", "MC15": "N/A", "MC16": "N/A", "MC17": "N/A", "MC18": "N/A", "MC19": "N/A"},
    "Ethernet4": {"MC10": "N/A", "MC11": "N/A", "MC12": "N/A", "MC13": "N/A", "MC14": "N/A", "MC15": "N/A", "MC16": "N/A", "MC17": "N/A", "MC18": "N/A", "MC19": "N/A"},
    "Ethernet8": {"MC10": "N/A", "MC11": "N/A", "MC12": "N/A", "MC13": "N/A", "MC14": "N/A", "MC15": "N/A", "MC16": "N/A", "MC17": "N/A", "MC18": "N/A", "MC19": "N/A"}
}"#;

/// Mock JSON for `show queue watermark all`
/// Source: wm_test_vectors.py `show_queue_wm_all_output`
pub const MOCK_QUEUE_WM_ALL: &str = r#"{
    "Ethernet0": {"ALL20": "1234567", "ALL21": "7654321", "ALL22": "0", "ALL23": "0", "ALL24": "0", "ALL25": "20", "ALL26": "500", "ALL27": "200", "ALL28": "0", "ALL29": "10"},
    "Ethernet4": {"ALL20": "0", "ALL21": "0", "ALL22": "0", "ALL23": "1986", "ALL24": "2567", "ALL25": "0", "ALL26": "0", "ALL27": "0", "ALL28": "0", "ALL29": "0"},
    "Ethernet8": {"ALL20": "20", "ALL21": "5", "ALL22": "1998", "ALL23": "0", "ALL24": "0", "ALL25": "0", "ALL26": "0", "ALL27": "0", "ALL28": "8528", "ALL29": "7696"}
}"#;

/// Mock JSON for `show queue persistent-watermark all`
/// Source: wm_test_vectors.py `show_queue_pwm_all_output`
pub const MOCK_QUEUE_PWM_ALL: &str = r#"{
    "Ethernet0": {"ALL20": "N/A", "ALL21": "N/A", "ALL22": "N/A", "ALL23": "N/A", "ALL24": "N/A", "ALL25": "N/A", "ALL26": "N/A", "ALL27": "N/A", "ALL28": "N/A", "ALL29": "N/A"},
    "Ethernet4": {"ALL20": "N/A", "ALL21": "N/A", "ALL22": "N/A", "ALL23": "N/A", "ALL24": "N/A", "ALL25": "N/A", "ALL26": "N/A", "ALL27": "N/A", "ALL28": "N/A", "ALL29": "N/A"},
    "Ethernet8": {"ALL20": "N/A", "ALL21": "N/A", "ALL22": "N/A", "ALL23": "N/A", "ALL24": "N/A", "ALL25": "N/A", "ALL26": "N/A", "ALL27": "N/A", "ALL28": "N/A", "ALL29": "N/A"}
}"#;

/// Mock JSON for `show watermark telemetry interval`
pub const MOCK_WATERMARK_TELEMETRY: &str = r#"{"interval": "120"}"#;

/// Mock JSON for `show watermark telemetry interval` with suffix
pub const MOCK_WATERMARK_TELEMETRY_SUFFIX: &str = r#"{"interval": "180s"}"#;

/// Mock JSON for empty buffer pool (edge case)
pub const MOCK_BUFFER_POOL_EMPTY: &str = r#"{}"#;

/// Mock JSON for single pool
pub const MOCK_BUFFER_POOL_SINGLE: &str = r#"{
    "ingress_lossless_pool": {"Bytes": "5000"}
}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `wm_input/wm_test_vectors.py` — `show watermark telemetry interval`
pub const SHOW_WATERMARK_TELEMETRY_OUTPUT: &str = r#"Telemetry interval 120 second(s)"#;

/// Source: `wm_input/wm_test_vectors.py` — `show watermark telemetry interval` (with suffix)
pub const SHOW_WATERMARK_TELEMETRY_SUFFIX_OUTPUT: &str = r#"Telemetry interval 180 second(s)"#;

/// Source: `wm_input/wm_test_vectors.py` — `show buffer_pool watermark` (single pool)
pub const SHOW_BUFFER_POOL_SINGLE_OUTPUT: &str = r#"Shared pool maximum occupancy:
                 Pool    Bytes
---------------------  -------
ingress_lossless_pool     5000"#;

/// Source: `wm_input/wm_test_vectors.py` — `show buffer_pool watermark` (empty)
pub const SHOW_BUFFER_POOL_EMPTY_OUTPUT: &str = r#"Shared pool maximum occupancy:
  Pool    Bytes
------  -------"#;
