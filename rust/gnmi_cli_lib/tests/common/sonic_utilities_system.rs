
// =============================================================================
// Mock JSON input data (gNMI telemetry simulation)
// =============================================================================

/// Mock JSON for `show clock`
pub const MOCK_CLOCK: &str = r#"{"date": "Mon Jan 26 09:56:41 UTC 2026"}"#;

/// Mock JSON for `show clock` — with AM/PM already present
pub const MOCK_CLOCK_AMPM: &str = r#"{"date": "Mon Jan 26 09:56:41 AM UTC 2026"}"#;

/// Mock JSON for `show uptime`
pub const MOCK_UPTIME: &str = r#"{"uptime": "up 3 days, 12:45"}"#;

/// Mock JSON for `show uptime` — component fields
pub const MOCK_UPTIME_COMPONENTS: &str = r#"{
    "time": "14:30:00",
    "up": "3 days, 12:45",
    "users": "2 users",
    "load": "0.50, 0.45, 0.40"
}"#;

/// Mock JSON for `show version`
pub const MOCK_VERSION: &str = r#"{
    "sonic_software_version": "SONiC.20230531.01",
    "sonic_os_version": "11",
    "distribution": "Debian 11.7",
    "kernel": "5.10.0-23-2-amd64",
    "build_commit": "abc123def",
    "build_date": "Fri May 31 12:00:00 UTC 2023",
    "built_by": "johndoe@sonic-build-server",
    "platform": "x86_64-mlnx_msn2700-r0",
    "hwsku": "Mellanox-SN2700",
    "asic": "mellanox",
    "serial_number": "MT1234567890",
    "uptime": "up 3 days, 12:45",
    "date": "Mon Jan 26 09:56:41 UTC 2026",
    "docker_version": "20.10.24",
    "docker_images": [
        {"REPOSITORY": "docker-sonic-telemetry", "TAG": "latest", "IMAGE ID": "sha256:abc123def456", "SIZE": "500MB"},
        {"REPOSITORY": "docker-database", "TAG": "latest", "IMAGE ID": "sha256:def789ghi012", "SIZE": "300MB"}
    ]
}"#;

/// Mock JSON for `show system memory`
pub const MOCK_SYSTEM_MEMORY: &str = r#"[
    {"type": "Mem", "total": "16384000", "used": "8192000", "free": "4096000", "shared": "512000", "buff/cache": "4096000", "available": "7680000"},
    {"type": "Swap", "total": "2048000", "used": "0", "free": "2048000"}
]"#;

/// Mock JSON for `show services`
pub const MOCK_SERVICES: &str = r#"[
    {
        "dockerProcessName": "database",
        "status": "Up 3 days",
        "processes": [
            {"user": "root", "pid": "100", "cpuPercentage": "0.5", "memPercentage": "2.1", "vsz": "500000", "rss": "200000", "tty": "?", "stat": "Ss", "start": "Jan01", "time": "0:30", "command": "redis-server *:6379"}
        ]
    },
    {
        "dockerProcessName": "swss",
        "status": "Up 3 days",
        "processes": [
            {"user": "root", "pid": "200", "cpuPercentage": "1.2", "memPercentage": "3.5", "vsz": "800000", "rss": "350000", "tty": "?", "stat": "Sl", "start": "Jan01", "time": "1:45", "command": "orchagent"}
        ]
    }
]"#;

/// Mock JSON for `show processes summary`
pub const MOCK_PROCESSES_SUMMARY: &str = r#"[
    {"PID": "1", "PPID": "0", "CMD": "systemd", "%MEM": "0.5", "%CPU": "0.1", "STIME": "Jan01", "TIME": "0:30", "TT": "?", "UID": "root"},
    {"PID": "100", "PPID": "1", "CMD": "redis-server *:6379", "%MEM": "2.1", "%CPU": "0.5", "STIME": "Jan01", "TIME": "0:30", "TT": "?", "UID": "root"},
    {"PID": "200", "PPID": "1", "CMD": "orchagent", "%MEM": "3.5", "%CPU": "1.2", "STIME": "Jan01", "TIME": "1:45", "TT": "?", "UID": "root"}
]"#;

/// Mock JSON for `show processes cpu`
pub const MOCK_PROCESSES_CPU: &str = r#"{
    "uptime": "up 3 days, 12:45",
    "tasks": "150 total, 2 running, 148 sleeping, 0 stopped, 0 zombie",
    "cpu_usage": "5.0 us, 2.0 sy, 0.0 ni, 92.0 id, 0.5 wa, 0.0 hi, 0.5 si, 0.0 st",
    "memory_usage": "16384000 total, 8192000 free, 4096000 used, 4096000 buff/cache",
    "swap_usage": "2048000 total, 2048000 free, 0 used. 7680000 avail Mem",
    "processes": [
        {"pid": "200", "user": "root", "pr": "20", "ni": "0", "virt": "800000", "res": "350000", "shr": "50000", "s": "S", "cpu": "1.2", "mem": "3.5", "time": "1:45.00", "command": "orchagent"},
        {"pid": "100", "user": "root", "pr": "20", "ni": "0", "virt": "500000", "res": "200000", "shr": "30000", "s": "S", "cpu": "0.5", "mem": "2.1", "time": "0:30.00", "command": "redis-server"}
    ]
}"#;

/// Mock JSON for `show reboot-cause`
pub const MOCK_REBOOT_CAUSE: &str = r#"{"cause": "User issued 'reboot' command [User: admin, Time: Mon Jan 26 09:56:41 UTC 2026]"}"#;

/// Mock JSON for `show reboot-cause history`
pub const MOCK_REBOOT_CAUSE_HISTORY: &str = r#"[
    {"name": "2026_01_26_09_56_41", "cause": "User issued 'reboot' command [User: admin, Time: Mon Jan 26 09:56:41 UTC 2026]", "time": "Mon Jan 26 09:56:41 UTC 2026", "user": "admin", "comment": ""},
    {"name": "2026_01_20_15_30_00", "cause": "Hardware - Loss of power", "time": "Fri Jan 20 15:30:00 UTC 2026", "user": "N/A", "comment": "Unexpected power loss"}
]"#;

/// Mock JSON for `show mmu`
/// Source: mmuconfig_input/mmuconfig_test_vectors.py `show_mmu_config`
pub const MOCK_MMU: &str = r#"{
    "pools": {
        "egress_lossless_pool": {"mode": "dynamic", "size": "13924352", "type": "egress"},
        "egress_lossy_pool": {"mode": "dynamic", "size": "7sobr924352", "type": "egress"},
        "ingress_lossless_pool": {"mode": "dynamic", "size": "12766208", "type": "ingress", "xoff": "4625920"}
    },
    "profiles": {
        "egress_lossless_profile": {"dynamic_th": "7", "pool": "egress_lossless_pool", "size": "0"},
        "egress_lossy_profile": {"dynamic_th": "3", "pool": "egress_lossy_pool", "size": "1518"},
        "ingress_lossy_profile": {"dynamic_th": "3", "pool": "ingress_lossless_pool", "size": "0"},
        "pg_lossless_10000_40m_profile": {"dynamic_th": "0", "pool": "ingress_lossless_pool", "size": "56368", "xoff": "55120", "xon": "18432", "xon_offset": "2496"}
    }
}"#;

/// Mock JSON for `show watermark telemetry interval`
pub const MOCK_WATERMARK_TELEMETRY_INTERVAL: &str = r#"{"interval": "120"}"#;

/// Mock JSON for empty reboot cause (edge case)
pub const MOCK_REBOOT_CAUSE_EMPTY: &str = r#"{"cause": "Unknown"}"#;

// =========================================================================
// Expected output constants (captured from converter for assert_exact_match)
// =========================================================================

/// Source: mock data — `show clock`
pub const SHOW_CLOCK_OUTPUT: &str = r#"Mon Jan 26 09:56:41 AM UTC 2026"#;

/// Source: mock data — `show clock` (AM/PM format)
pub const SHOW_CLOCK_AMPM_OUTPUT: &str = r#"Mon Jan 26 09:56:41 AM UTC 2026"#;

/// Source: mock data — `show uptime`
pub const SHOW_UPTIME_OUTPUT: &str = r#"up 3 days, 12:45"#;

/// Source: mock data — `show uptime` (with components)
pub const SHOW_UPTIME_COMPONENTS_OUTPUT: &str = r#"14:30:00, up 3 days, 12:45, 2 users users, load average: 0.50, 0.45, 0.40"#;

/// Source: mock data — `show version`
pub const SHOW_VERSION_OUTPUT: &str = r#"
SONiC Software Version: SONiC.20230531.01
SONiC OS Version: 11
Distribution: Debian 11.7
Kernel: 5.10.0-23-2-amd64
Build commit: abc123def
Build date: Fri May 31 12:00:00 UTC 2023
Built by: johndoe@sonic-build-server

Platform: x86_64-mlnx_msn2700-r0
HwSKU: Mellanox-SN2700
ASIC: mellanox
Serial Number: MT1234567890
Uptime: up 3 days, 12:45
Date: Mon Jan 26 09:56:41 UTC 2026
Docker Version: 20.10.24

Docker images:
REPOSITORY                 TAG           IMAGE ID       SIZE
N/A                        N/A           N/A N/A
N/A                        N/A           N/A N/A"#;

/// Source: mock data — `show system-memory`
pub const SHOW_SYSTEM_MEMORY_OUTPUT: &str = r#"               total        used        free      shared  buff/cache   available
Mem:        16384000     8192000     4096000      512000     4096000     7680000
Swap:        2048000           0     2048000"#;

/// Source: mock data — `show services`
pub const SHOW_SERVICES_OUTPUT: &str = r#"database	Up 3 days
---------------------------
USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         100  0.5  2.1 500000 200000 ?       Ss   Jan01   0:30 redis-server *:6379
swss	Up 3 days
---------------------------
USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         200  1.2  3.5 800000 350000 ?       Sl   Jan01   1:45 orchagent"#;

/// Source: mock data — `show processes summary`
pub const SHOW_PROCESSES_SUMMARY_OUTPUT: &str = r#"  PID    PPID                  CMD    %MEM    %CPU    STIME    TIME    TT    UID
-----  ------  -------------------  ------  ------  -------  ------  ----  -----
    1       0  systemd                 0.5     0.1  Jan01    0:30    ?     root
  100       1  redis-server *:6379     2.1     0.5  Jan01    0:30    ?     root
  200       1  orchagent               3.5     1.2  Jan01    1:45    ?     root"#;

/// Source: mock data — `show processes cpu`
pub const SHOW_PROCESSES_CPU_OUTPUT: &str = r#"top - up 3 days, 12:45
Tasks: 150 total, 2 running, 148 sleeping, 0 stopped, 0 zombie
%Cpu(s): 5.0 us, 2.0 sy, 0.0 ni, 92.0 id, 0.5 wa, 0.0 hi, 0.5 si, 0.0 st
MiB Mem : 16384000 total, 8192000 free, 4096000 used, 4096000 buff/cache
MiB Swap: 2048000 total, 2048000 free, 0 used. 7680000 avail Mem

    PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
    200 root      20   0  800000 350000  50000 S   1.2   3.5   1:45.00 orchagent
    100 root      20   0  500000 200000  30000 S   0.5   2.1   0:30.00 redis-server"#;

/// Source: mock data — `show reboot-cause`
pub const SHOW_REBOOT_CAUSE_OUTPUT: &str = r#"User issued 'reboot' command [User: admin, Time: Mon Jan 26 09:56:41 UTC 2026]"#;

/// Source: mock data — `show reboot-cause history`
pub const SHOW_REBOOT_CAUSE_HISTORY_OUTPUT: &str = r#" Name                 Cause                                                                           Time                          User   Comment               
 2026_01_26_09_56_41  User issued 'reboot' command [User: admin, Time: Mon Jan 26 09:56:41 UTC 2026]  Mon Jan 26 09:56:41 UTC 2026  admin                        
 2026_01_20_15_30_00  Hardware - Loss of power                                                        Fri Jan 20 15:30:00 UTC 2026  N/A    Unexpected power loss"#;

/// Source: mock data — `show reboot-cause` (empty/unknown)
pub const SHOW_REBOOT_CAUSE_EMPTY_OUTPUT: &str = r#"Unknown"#;

/// Source: `mmuconfig_input/mmuconfig_test_vectors.py` — `show mmu`
pub const SHOW_MMU_OUTPUT: &str = r#"Pool: egress_lossless_pool
----  --------
mode  dynamic
size  13924352
type  egress
----  --------

Pool: egress_lossy_pool
----  -----------
mode  dynamic
size  7sobr924352
type  egress
----  -----------

Pool: ingress_lossless_pool
----  --------
mode  dynamic
size  12766208
type  ingress
xoff  4625920
----  --------

Profile: egress_lossless_profile
----------  --------------------
dynamic_th  7
pool        egress_lossless_pool
size        0
----------  --------------------

Profile: egress_lossy_profile
----------  -----------------
dynamic_th  3
pool        egress_lossy_pool
size        1518
----------  -----------------

Profile: ingress_lossy_profile
----------  ---------------------
dynamic_th  3
pool        ingress_lossless_pool
size        0
----------  ---------------------

Profile: pg_lossless_10000_40m_profile
----------  ---------------------
dynamic_th  0
pool        ingress_lossless_pool
size        56368
xoff        55120
xon         18432
xon_offset  2496
----------  ---------------------"#;

/// Source: mock data — `show watermark telemetry interval`
pub const SHOW_WATERMARK_TELEMETRY_INTERVAL_OUTPUT: &str = r#"Telemetry interval 120 second(s)"#;
