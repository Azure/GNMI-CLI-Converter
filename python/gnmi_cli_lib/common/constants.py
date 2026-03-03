"""
Constants used throughout the gnmi_cli_lib library.

This module centralizes all magic values, column orderings, and other
constants to make the code more maintainable and easier to update.
"""

from typing import List, Tuple

# =============================================================================
# Column Orderings
# =============================================================================

# Standard port drop counter columns (in order)
STANDARD_PORT_COLUMNS: List[str] = [
    "State",
    "RX_ERR",
    "RX_DROPS",
    "TX_ERR",
    "TX_DROPS",
]

# Queue counter columns (in order)
QUEUE_COLUMN_ORDER: List[str] = [
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
]

# WRED counter columns (in order)
WRED_COLUMN_ORDER: List[str] = [
    "WredDrp/pkts",
    "WredDrp/bytes",
    "EcnMarked/pkts",
    "EcnMarked/bytes",
]

# Interface counter columns (in order) - STATE comes first
# Different commands use different subsets of these columns
INTERFACE_COLUMN_ORDER: List[str] = [
    "STATE",
    "RX_OK",
    "RX_BPS",
    "RX_PPS",  # For rates command
    "RX_UTIL",
    "RX_ERR",
    "RX_DRP",
    "RX_OVR",
    "TX_OK",
    "TX_BPS",
    "TX_PPS",  # For rates command
    "TX_UTIL",
    "TX_ERR",
    "TX_DRP",
    "TX_OVR",
    # RIF counter bits columns
    "RX_OK_BITS",
    "TX_OK_BITS",
    "RX_ERR_BITS",
    "TX_ERR_BITS",
    # Trim counter columns
    "TRIM_PKTS",
    "TRIM_TX_PKTS",
    "TRIM_DRP_PKTS",
    # FEC stats columns
    "FEC_CORR",
    "FEC_UNCORR",
    "FEC_SYMBOL_ERR",
    "FEC_PRE_BER",
    "FEC_POST_BER",
]

# Mapping from gNMI field names to CLI column names for interface counters
INTERFACE_COUNTERS_FIELD_MAP: dict = {
    "State": "STATE",
    "RxOk": "RX_OK",
    "RxBps": "RX_BPS",
    "RxPps": "RX_PPS",
    "RxUtil": "RX_UTIL",
    "RxErr": "RX_ERR",
    "RxDrp": "RX_DRP",
    "RxOvr": "RX_OVR",
    "TxOk": "TX_OK",
    "TxBps": "TX_BPS",
    "TxPps": "TX_PPS",
    "TxUtil": "TX_UTIL",
    "TxErr": "TX_ERR",
    "TxDrp": "TX_DRP",
    "TxOvr": "TX_OVR",
    # RIF counter field names
    "RxOkPackets": "RX_OK",
    "RxErrPackets": "RX_ERR",
    "TxOkPackets": "TX_OK",
    "TxErrPackets": "TX_ERR",
    # Trim counter field names
    "TrimPkts": "TRIM_PKTS",
    "TrimSent": "TRIM_TX_PKTS",
    "TrimDrp": "TRIM_DRP_PKTS",
    # FEC stats field names
    "FecCorr": "FEC_CORR",
    "FecUncorr": "FEC_UNCORR",
    "FecSymbolErr": "FEC_SYMBOL_ERR",
    "FecPreBer": "FEC_PRE_BER",
    "FecPostBer": "FEC_POST_BER",
    # RIF counter bits fields (displayed in output)
    "RxOkBits": "RX_OK_BITS",
    "TxOkBits": "TX_OK_BITS",
    "RxErrBits": "RX_ERR_BITS",
    "TxErrBits": "TX_ERR_BITS",
}

# =============================================================================
# Version Output Field Ordering
# =============================================================================

VERSION_FIELD_ORDER: List[Tuple[str, str]] = [
    ("sonic_software_version", "SONiC Software Version"),
    ("sonic_os_version", "SONiC OS Version"),
    ("distribution", "Distribution"),
    ("kernel", "Kernel"),
    ("build_commit", "Build commit"),
    ("build_date", "Build date"),
    ("built_by", "Built by"),
    ("platform", "Platform"),
    ("hwsku", "HwSKU"),
    ("asic", "ASIC"),
    ("asic_count", "ASIC Count"),
    ("serial_number", "Serial Number"),
    ("model_number", "Model Number"),
    ("hardware_revision", "Hardware Revision"),
    ("uptime", "Uptime"),
    ("date", "Date"),
]

# =============================================================================
# Dropcounters Configuration Field Mapping
# =============================================================================

DROPCOUNTERS_FIELD_MAPPING = {
    "Counter": "name",
    "Alias": "alias",
    "Group": "group",
    "Type": "type",
    "Reasons": "reason",
    "Description": "description",
}

DROPCOUNTERS_HEADERS: List[str] = [
    "Counter",
    "Alias",
    "Group",
    "Type",
    "Reasons",
    "Description",
]

# =============================================================================
# LLDP Constants
# =============================================================================

DEFAULT_LLDP_CAPABILITY_CODES = "Capability codes: (R) Router, (B) Bridge, (O) Other"

