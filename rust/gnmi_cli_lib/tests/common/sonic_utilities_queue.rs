
// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Source: queuestat_input/assert_show_output.py `counters_all`
pub const MOCK_QUEUE_COUNTERS_ETH0: &str = r#"{
    "Ethernet0:0": {"Counter/pkts": "0", "Counter/bytes": "0", "Drop/pkts": "0", "Drop/bytes": "0", "Trim/pkts": "0"},
    "Ethernet0:1": {"Counter/pkts": "60", "Counter/bytes": "43", "Drop/pkts": "39", "Drop/bytes": "1", "Trim/pkts": "100"},
    "Ethernet0:2": {"Counter/pkts": "82", "Counter/bytes": "7", "Drop/pkts": "39", "Drop/bytes": "21"},
    "Ethernet0:3": {"Counter/pkts": "52", "Counter/bytes": "70", "Drop/pkts": "19", "Drop/bytes": "76"},
    "Ethernet0:4": {"Counter/pkts": "11", "Counter/bytes": "59", "Drop/pkts": "12", "Drop/bytes": "94"},
    "Ethernet0:5": {"Counter/pkts": "36", "Counter/bytes": "62", "Drop/pkts": "35", "Drop/bytes": "40"},
    "Ethernet0:6": {"Counter/pkts": "49", "Counter/bytes": "91", "Drop/pkts": "2", "Drop/bytes": "88"},
    "Ethernet0:7": {"Counter/pkts": "33", "Counter/bytes": "17", "Drop/pkts": "94", "Drop/bytes": "74"},
    "Ethernet0:8": {"Counter/pkts": "40", "Counter/bytes": "71", "Drop/pkts": "95", "Drop/bytes": "33"},
    "Ethernet0:9": {"Counter/pkts": "54", "Counter/bytes": "8", "Drop/pkts": "93", "Drop/bytes": "78"},
    "Ethernet0:10": {"Counter/pkts": "83", "Counter/bytes": "96", "Drop/pkts": "74", "Drop/bytes": "9"},
    "Ethernet0:11": {"Counter/pkts": "15", "Counter/bytes": "60", "Drop/pkts": "61", "Drop/bytes": "31"},
    "Ethernet0:12": {"Counter/pkts": "45", "Counter/bytes": "52", "Drop/pkts": "82", "Drop/bytes": "94"},
    "Ethernet0:13": {"Counter/pkts": "55", "Counter/bytes": "88", "Drop/pkts": "89", "Drop/bytes": "52"},
    "Ethernet0:14": {"Counter/pkts": "14", "Counter/bytes": "70", "Drop/pkts": "95", "Drop/bytes": "79"},
    "Ethernet0:15": {"Counter/pkts": "68", "Counter/bytes": "60", "Drop/pkts": "66", "Drop/bytes": "81"},
    "Ethernet0:16": {"Counter/pkts": "63", "Counter/bytes": "4", "Drop/pkts": "48", "Drop/bytes": "76"},
    "Ethernet0:17": {"Counter/pkts": "41", "Counter/bytes": "73", "Drop/pkts": "77", "Drop/bytes": "74"},
    "Ethernet0:18": {"Counter/pkts": "60", "Counter/bytes": "21", "Drop/pkts": "56", "Drop/bytes": "54"},
    "Ethernet0:19": {"Counter/pkts": "57", "Counter/bytes": "31", "Drop/pkts": "12", "Drop/bytes": "39"}
}"#;

/// Mock JSON for `show queue counters` — simple two-port example
/// Source: queuestat_input/assert_show_output.py `counters_all` (simplified)
pub const MOCK_QUEUE_COUNTERS_SIMPLE: &str = r#"{
    "Ethernet0:0": {"Counter/pkts": "100", "Counter/bytes": "6400", "Drop/pkts": "0", "Drop/bytes": "0"},
    "Ethernet0:1": {"Counter/pkts": "200", "Counter/bytes": "12800", "Drop/pkts": "5", "Drop/bytes": "320"},
    "Ethernet4:0": {"Counter/pkts": "50", "Counter/bytes": "3200", "Drop/pkts": "0", "Drop/bytes": "0"},
    "Ethernet4:1": {"Counter/pkts": "75", "Counter/bytes": "4800", "Drop/pkts": "2", "Drop/bytes": "128"}
}"#;

/// Mock JSON for `show queue counters` with trim data
/// Source: queuestat_input/assert_show_output.py `trim_counters_all` (Ethernet0 portion)
pub const MOCK_TRIM_COUNTERS_ETH0: &str = r#"{
    "Ethernet0:0": {"Counter/pkts": "0", "Counter/bytes": "0", "Drop/pkts": "0", "Drop/bytes": "0", "Trim/pkts": "0", "TrimSent/pkts": "0", "TrimDrop/pkts": "0"},
    "Ethernet0:1": {"Counter/pkts": "60", "Counter/bytes": "43", "Drop/pkts": "39", "Drop/bytes": "1", "Trim/pkts": "100", "TrimSent/pkts": "20", "TrimDrop/pkts": "80"}
}"#;

/// Mock JSON for `show queue counters` — WRED counters
pub const MOCK_WRED_COUNTERS: &str = r#"{
    "Ethernet0:0": {"WredDrp/pkts": "100", "WredDrp/bytes": "6400", "EcnMarked/pkts": "50", "EcnMarked/bytes": "3200"},
    "Ethernet0:1": {"WredDrp/pkts": "200", "WredDrp/bytes": "12800", "EcnMarked/pkts": "75", "EcnMarked/bytes": "4800"},
    "Ethernet0:8": {"WredDrp/pkts": "10", "WredDrp/bytes": "640", "EcnMarked/pkts": "5", "EcnMarked/bytes": "320"}
}"#;

/// Mock JSON for priority group counters
pub const MOCK_PG_COUNTERS: &str = r#"{
    "Ethernet0": {"PG0": {"ingress_packets": "1000", "ingress_bytes": "64000", "drop_packets": "10", "drop_bytes": "640"}, "PG1": {"ingress_packets": "2000", "ingress_bytes": "128000", "drop_packets": "20", "drop_bytes": "1280"}},
    "Ethernet4": {"PG0": {"ingress_packets": "500", "ingress_bytes": "32000", "drop_packets": "5", "drop_bytes": "320"}, "PG1": {"ingress_packets": "750", "ingress_bytes": "48000", "drop_packets": "8", "drop_bytes": "512"}}
}"#;

/// Mock JSON for empty queue counters (edge case)
pub const MOCK_QUEUE_COUNTERS_EMPTY: &str = r#"{}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: `queuestat_input/assert_show_output.py` — `show queue counters` (Ethernet0)
pub const SHOW_QUEUE_COUNTERS_ETH0_OUTPUT: &str = r#"For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes    Trim/pkts    TrimSent/pkts    TrimDrop/pkts
---------  -----  --------------  ---------------  -----------  ------------  -----------  ---------------  ---------------
Ethernet0    UC0               0                0            0             0            0                0                0
Ethernet0    UC1              60               43           39             1          100                0                0
Ethernet0    UC2              82                7           39            21            0                0                0
Ethernet0    UC3              52               70           19            76            0                0                0
Ethernet0    UC4              11               59           12            94            0                0                0
Ethernet0    UC5              36               62           35            40            0                0                0
Ethernet0    UC6              49               91            2            88            0                0                0
Ethernet0    UC7              33               17           94            74            0                0                0
Ethernet0    MC8              40               71           95            33            0                0                0
Ethernet0    MC9              54                8           93            78            0                0                0
Ethernet0   MC10              83               96           74             9            0                0                0
Ethernet0   MC11              15               60           61            31            0                0                0
Ethernet0   MC12              45               52           82            94            0                0                0
Ethernet0   MC13              55               88           89            52            0                0                0
Ethernet0   MC14              14               70           95            79            0                0                0
Ethernet0   MC15              68               60           66            81            0                0                0
Ethernet0   MC16              63                4           48            76            0                0                0
Ethernet0   MC17              41               73           77            74            0                0                0
Ethernet0   MC18              60               21           56            54            0                0                0
Ethernet0   MC19              57               31           12            39            0                0                0"#;

/// Source: `queuestat_input/assert_show_output.py` — `show queue counters` (simple)
pub const SHOW_QUEUE_COUNTERS_SIMPLE_OUTPUT: &str = r#"For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
---------  -----  --------------  ---------------  -----------  ------------
Ethernet0    UC0             100             6400            0             0
Ethernet0    UC1             200            12800            5           320

For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
---------  -----  --------------  ---------------  -----------  ------------
Ethernet4    UC0              50             3200            0             0
Ethernet4    UC1              75             4800            2           128"#;

/// Source: `queuestat_input/assert_show_output.py` — `show queue counters` (empty)
pub const SHOW_QUEUE_COUNTERS_EMPTY_OUTPUT: &str = "";

/// Source: `queuestat_input/assert_show_output.py` — `show queue counters` (trim, Ethernet0)
pub const SHOW_TRIM_COUNTERS_ETH0_OUTPUT: &str = r#"For namespace :
     Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes    Trim/pkts    TrimSent/pkts    TrimDrop/pkts
---------  -----  --------------  ---------------  -----------  ------------  -----------  ---------------  ---------------
Ethernet0    UC0               0                0            0             0            0                0                0
Ethernet0    UC1              60               43           39             1          100               20               80"#;

/// Source: `queuestat_input/assert_show_output.py` — `show queue wredcounters`
pub const SHOW_WRED_COUNTERS_OUTPUT: &str = r#"     Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
---------  -----  --------------  ---------------  ----------------  -----------------
Ethernet0    UC0             100             6400                50               3200
Ethernet0    UC1             200            12800                75               4800
Ethernet0    MC8              10              640                 5                320"#;

/// Source: `queuestat_input/assert_show_output.py` — `show priority-group counters`
pub const SHOW_PG_COUNTERS_OUTPUT: &str = r#"For namespace :
     Port    TxQ
---------  -----
Ethernet0    UC0

For namespace :
     Port    TxQ
---------  -----
Ethernet4    UC0"#;
