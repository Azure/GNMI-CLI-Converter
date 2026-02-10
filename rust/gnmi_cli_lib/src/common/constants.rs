//! Constants used throughout the gnmi_cli_lib library.
//!
//! This module centralizes all magic values, column orderings, and other
//! constants to make the code more maintainable and easier to update.

use lazy_static::lazy_static;
use std::collections::HashMap;

// =============================================================================
// Table Formatting Constants
// =============================================================================

/// Default table format for tabulate output.
pub const DEFAULT_TABLE_FORMAT: &str = "simple";

/// Default column alignment.
pub const DEFAULT_ALIGNMENT: &str = "right";

// =============================================================================
// Column Orderings
// =============================================================================

/// Standard port drop counter columns (in order).
pub const STANDARD_PORT_COLUMNS: &[&str] = &["State", "RX_ERR", "RX_DROPS", "TX_ERR", "TX_DROPS"];

/// Queue counter columns (in order).
pub const QUEUE_COLUMN_ORDER: &[&str] = &[
    "Counter/pkts",
    "Counter/bytes",
    "Drop/pkts",
    "Drop/bytes",
    "Trim/pkts",
    "TrimSent/pkts",
    "TrimDrop/pkts",
    "WredDrp/pkts",
    "WredDrp/bytes",
    "EcnMarked/pkts",
    "EcnMarked/bytes",
];

/// WRED counter columns (in order).
pub const WRED_COLUMN_ORDER: &[&str] = &[
    "WredDrp/pkts",
    "WredDrp/bytes",
    "EcnMarked/pkts",
    "EcnMarked/bytes",
];

/// Interface counter columns (in order) - STATE comes first.
/// Different commands use different subsets of these columns.
pub const INTERFACE_COLUMN_ORDER: &[&str] = &[
    "STATE",
    "RX_OK",
    "RX_BPS",
    "RX_PPS",
    "RX_UTIL",
    "RX_ERR",
    "RX_DRP",
    "RX_OVR",
    "TX_OK",
    "TX_BPS",
    "TX_PPS",
    "TX_UTIL",
    "TX_ERR",
    "TX_DRP",
    "TX_OVR",
    // RIF counter bits columns
    "RX_OK_BITS",
    "TX_OK_BITS",
    "RX_ERR_BITS",
    "TX_ERR_BITS",
    // Trim counter columns
    "TRIM_PKTS",
    "TRIM_TX_PKTS",
    "TRIM_DRP_PKTS",
    // FEC stats columns
    "FEC_CORR",
    "FEC_UNCORR",
    "FEC_SYMBOL_ERR",
    "FEC_PRE_BER",
    "FEC_POST_BER",
];

lazy_static! {
    /// Mapping from gNMI field names to CLI column names for interface counters.
    pub static ref INTERFACE_COUNTERS_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("State", "STATE");
        m.insert("RxOk", "RX_OK");
        m.insert("RxBps", "RX_BPS");
        m.insert("RxPps", "RX_PPS");
        m.insert("RxUtil", "RX_UTIL");
        m.insert("RxErr", "RX_ERR");
        m.insert("RxDrp", "RX_DRP");
        m.insert("RxOvr", "RX_OVR");
        m.insert("TxOk", "TX_OK");
        m.insert("TxBps", "TX_BPS");
        m.insert("TxPps", "TX_PPS");
        m.insert("TxUtil", "TX_UTIL");
        m.insert("TxErr", "TX_ERR");
        m.insert("TxDrp", "TX_DRP");
        m.insert("TxOvr", "TX_OVR");
        // RIF counters (packet variants)
        m.insert("RxOkPackets", "RX_OK");
        m.insert("TxOkPackets", "TX_OK");
        m.insert("RxErrPackets", "RX_ERR");
        m.insert("TxErrPackets", "TX_ERR");
        // RIF counters (bit/byte variants)
        m.insert("RxOkBits", "RX_OK_BITS");
        m.insert("TxOkBits", "TX_OK_BITS");
        m.insert("RxErrBits", "RX_ERR_BITS");
        m.insert("TxErrBits", "TX_ERR_BITS");
        // Trim counters
        m.insert("TrimPkts", "TRIM_PKTS");
        m.insert("TrimTxPkts", "TRIM_TX_PKTS");
        m.insert("TrimDrpPkts", "TRIM_DRP_PKTS");
        // FEC stats
        m.insert("FecCorr", "FEC_CORR");
        m.insert("FecUncorr", "FEC_UNCORR");
        m.insert("FecSymbolErr", "FEC_SYMBOL_ERR");
        m.insert("FecPreBer", "FEC_PRE_BER");
        m.insert("FecPostBer", "FEC_POST_BER");
        m
    };

    /// SAI statistic field names by command type.
    pub static ref SAI_STAT_FIELDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("buffer_pool", "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES");
        m.insert("headroom_pool", "SAI_BUFFER_POOL_STAT_WATERMARK_BYTES");
        m.insert("priority_group", "SAI_INGRESS_PRIORITY_GROUP_STAT_SHARED_WATERMARK_BYTES");
        m.insert("queue", "SAI_QUEUE_STAT_SHARED_WATERMARK_BYTES");
        m
    };

    /// Interface status field mapping from gNMI to internal format.
    pub static ref INTERFACE_STATUS_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Interface", "interface");
        m.insert("Lanes", "lanes");
        m.insert("Speed", "speed");
        m.insert("MTU", "mtu");
        m.insert("FEC", "fec");
        m.insert("Alias", "alias");
        m.insert("Vlan", "vlan");
        m.insert("Oper", "oper_status");
        m.insert("Admin", "admin_status");
        m.insert("Type", "type");
        m.insert("Asym", "asym_pfc");
        m
    };

    /// Interface flap field mapping.
    pub static ref INTERFACE_FLAP_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Interface", "interface");
        m.insert("FlapCount", "flap_count");
        m.insert("LastFlapTime", "last_flap_time");
        m
    };

    /// Interface alias field mapping.
    pub static ref INTERFACE_ALIAS_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Name", "name");
        m.insert("Alias", "alias");
        m
    };

    /// Interface counters RIF field mapping.
    pub static ref INTERFACE_COUNTERS_RIF_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("IFACE", "iface");
        m.insert("RX_OK", "rx_ok");
        m.insert("RX_BPS", "rx_bps");
        m.insert("RX_PPS", "rx_pps");
        m.insert("TX_OK", "tx_ok");
        m.insert("TX_BPS", "tx_bps");
        m.insert("TX_PPS", "tx_pps");
        m
    };

    /// Transceiver info field mapping.
    pub static ref TRANSCEIVER_INFO_FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Port", "port");
        m.insert("Identifier", "identifier");
        m.insert("Vendor", "vendor");
        m.insert("Model", "model");
        m.insert("Serial", "serial");
        m
    };
}

/// Prefix for user watermarks in COUNTERS_DB.
pub const USER_WATERMARKS_PREFIX: &str = "USER_WATERMARKS:";

/// Prefix for persistent watermarks in COUNTERS_DB.
pub const PERSISTENT_WATERMARKS_PREFIX: &str = "PERSISTENT_WATERMARKS:";

/// Priority group column prefixes.
pub const PG_COLUMN_PREFIXES: &[&str] = &[
    "PG0", "PG1", "PG2", "PG3", "PG4", "PG5", "PG6", "PG7", "PG8", "PG9", "PG10", "PG11", "PG12",
    "PG13", "PG14", "PG15",
];

/// Unicast queue column prefixes.
pub const UC_COLUMN_PREFIXES: &[&str] = &[
    "UC0", "UC1", "UC2", "UC3", "UC4", "UC5", "UC6", "UC7", "UC8", "UC9", "UC10", "UC11", "UC12",
    "UC13", "UC14", "UC15",
];

/// Multicast queue column prefixes.
pub const MC_COLUMN_PREFIXES: &[&str] = &[
    "MC0", "MC1", "MC2", "MC3", "MC4", "MC5", "MC6", "MC7", "MC8", "MC9", "MC10", "MC11", "MC12",
    "MC13", "MC14", "MC15",
];

/// All queue column prefixes (combined).
pub const ALL_QUEUE_PREFIXES: &[&str] = &[
    "UC0", "UC1", "UC2", "UC3", "UC4", "UC5", "UC6", "UC7", "MC8", "MC9", "MC10", "MC11", "MC12",
    "MC13", "MC14", "MC15",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_table_format() {
        assert_eq!(DEFAULT_TABLE_FORMAT, "simple");
    }

    #[test]
    fn test_default_alignment() {
        assert_eq!(DEFAULT_ALIGNMENT, "right");
    }

    #[test]
    fn test_standard_port_columns() {
        assert_eq!(STANDARD_PORT_COLUMNS.len(), 5);
        assert_eq!(STANDARD_PORT_COLUMNS[0], "State");
    }

    #[test]
    fn test_queue_column_order() {
        assert_eq!(QUEUE_COLUMN_ORDER.len(), 11);
        assert_eq!(QUEUE_COLUMN_ORDER[0], "Counter/pkts");
    }

    #[test]
    fn test_wred_column_order() {
        assert_eq!(WRED_COLUMN_ORDER.len(), 4);
        assert_eq!(WRED_COLUMN_ORDER[0], "WredDrp/pkts");
    }

    #[test]
    fn test_interface_column_order() {
        assert!(INTERFACE_COLUMN_ORDER.contains(&"STATE"));
        assert!(INTERFACE_COLUMN_ORDER.contains(&"RX_OK"));
        assert!(INTERFACE_COLUMN_ORDER.contains(&"TX_OK"));
    }

    #[test]
    fn test_interface_counters_field_map() {
        assert_eq!(INTERFACE_COUNTERS_FIELD_MAP.get("State"), Some(&"STATE"));
        assert_eq!(INTERFACE_COUNTERS_FIELD_MAP.get("RxOk"), Some(&"RX_OK"));
        assert_eq!(INTERFACE_COUNTERS_FIELD_MAP.get("TxOk"), Some(&"TX_OK"));
    }

    #[test]
    fn test_sai_stat_fields() {
        assert_eq!(
            SAI_STAT_FIELDS.get("buffer_pool"),
            Some(&"SAI_BUFFER_POOL_STAT_WATERMARK_BYTES")
        );
    }

    #[test]
    fn test_watermark_prefixes() {
        assert_eq!(USER_WATERMARKS_PREFIX, "USER_WATERMARKS:");
        assert_eq!(PERSISTENT_WATERMARKS_PREFIX, "PERSISTENT_WATERMARKS:");
    }

    #[test]
    fn test_pg_column_prefixes() {
        assert_eq!(PG_COLUMN_PREFIXES.len(), 16);
        assert_eq!(PG_COLUMN_PREFIXES[0], "PG0");
        assert_eq!(PG_COLUMN_PREFIXES[15], "PG15");
    }

    #[test]
    fn test_uc_column_prefixes() {
        assert_eq!(UC_COLUMN_PREFIXES.len(), 16);
        assert_eq!(UC_COLUMN_PREFIXES[0], "UC0");
    }

    #[test]
    fn test_mc_column_prefixes() {
        assert_eq!(MC_COLUMN_PREFIXES.len(), 16);
        assert_eq!(MC_COLUMN_PREFIXES[0], "MC0");
    }
}
