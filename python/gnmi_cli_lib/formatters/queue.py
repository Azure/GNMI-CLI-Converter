"""
Queue-related formatters.

This module contains formatters for queue commands:
- QueueCountersFormatter: show queue counters
- WredCountersFormatter: show queue wredcounters
"""

from typing import Dict, Optional
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import QUEUE_COLUMN_ORDER, WRED_COLUMN_ORDER
from gnmi_cli_lib.utils.sorting import natural_sort_key


class QueueCountersFormatter(OutputFormatter):
    """
    Formats queue counters output grouped by interface.
    
    Dynamically detects which columns are present in the JSON and outputs only those.
    
    Column sets:
    - Counter columns: Counter/pkts, Counter/bytes, Drop/pkts, Drop/bytes
    - Trim columns: Trim/pkts, TrimSent/pkts, TrimDrop/pkts
    
    Example input:
        {"Ethernet0:0": {"Counter/pkts": "100", "Drop/pkts": "2", ...}}
    
    Example output:
        For namespace :
             Port    TxQ    Counter/pkts    Counter/bytes    Drop/pkts    Drop/bytes
        ---------  -----  --------------  ---------------  -----------  ------------
        Ethernet0    UC0               0                0            0             0
        Ethernet0    UC1               0                0            0             0
        ...
    """
    
    # Column definitions
    COUNTER_COLUMNS = ["Counter/pkts", "Counter/bytes", "Drop/pkts", "Drop/bytes"]
    TRIM_COLUMNS = ["Trim/pkts", "TrimSent/pkts", "TrimDrop/pkts"]
    
    def _queue_num_to_txq(self, q_num: int) -> str:
        """Convert queue number to TxQ name (UC0-UC7, MC8-MC11)."""
        if q_num < 8:
            return f"UC{q_num}"
        else:
            return f"MC{q_num}"
    
    def _detect_columns(self, data: Dict) -> tuple:
        """
        Detect which columns are present in the JSON data.
        
        Returns:
            Tuple of (has_counter_columns, has_trim_columns)
        """
        # Check all queue entries for column presence
        has_counter = False
        has_trim = False
        
        for queue_key, queue_data in data.items():
            if not isinstance(queue_data, dict):
                continue
            for col in self.COUNTER_COLUMNS:
                if col in queue_data:
                    has_counter = True
                    break
            for col in self.TRIM_COLUMNS:
                if col in queue_data:
                    has_trim = True
                    break
            if has_counter and has_trim:
                break
        
        return has_counter, has_trim
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format queue counters as table grouped by interface."""
        if not data:
            return ""
        
        options = options or {}
        nonzero_filter = options.get('nonzero', False)
        
        # Detect which columns are actually present in the JSON
        has_counter_cols, has_trim_cols = self._detect_columns(data)
        
        # Group queues by interface
        interfaces = {}
        for queue_key in data.keys():
            if ":" in queue_key:
                port, q_num_str = queue_key.rsplit(":", 1)
                try:
                    q_num = int(q_num_str)
                except ValueError:
                    q_num = 0
            else:
                port = queue_key
                q_num = 0
            
            if port not in interfaces:
                interfaces[port] = []
            interfaces[port].append((q_num, queue_key, data[queue_key]))
        
        # Build output with one paragraph per interface
        paragraphs = []
        
        # Build headers based on what columns are present in JSON
        headers = ["Port", "TxQ"]
        if has_counter_cols:
            headers.extend(self.COUNTER_COLUMNS)
        if has_trim_cols:
            headers.extend(self.TRIM_COLUMNS)
        
        for port in sorted(interfaces.keys(), key=natural_sort_key):
            # Sort queues by number
            queues = sorted(interfaces[port], key=lambda x: x[0])
            
            table_data = []
            for q_num, queue_key, queue_data in queues:
                # Skip empty queue data when --nonzero is specified
                if nonzero_filter and not queue_data:
                    continue
                    
                txq = self._queue_num_to_txq(q_num)
                row = [port, txq]
                if has_counter_cols:
                    row.extend([
                        queue_data.get("Counter/pkts", "0"),
                        queue_data.get("Counter/bytes", "0"),
                        queue_data.get("Drop/pkts", "0"),
                        queue_data.get("Drop/bytes", "0"),
                    ])
                if has_trim_cols:
                    row.extend([
                        queue_data.get("Trim/pkts", "0"),
                        queue_data.get("TrimSent/pkts", "0"),
                        queue_data.get("TrimDrop/pkts", "0"),
                    ])
                table_data.append(row)
            
            # Skip this interface if no data rows after filtering
            if not table_data:
                continue
            
            lines = []
            lines.append("For namespace :")
            
            col_alignments = ["right"] * len(headers)
            table_str = tabulate(
                table_data,
                headers=headers,
                tablefmt=table_format,
                numalign="right",
                colalign=col_alignments,
            )
            lines.append(table_str)
            paragraphs.append("\n".join(lines))
        
        return "\n\n".join(paragraphs)


class WredCountersFormatter(OutputFormatter):
    """
    Formats WRED counters output grouped by interface.
    
    Example input:
        {"Ethernet0:0": {"WredDrp/pkts": "N/A", "EcnMarked/pkts": "0", ...}}
    
    Example output:
             Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
        ---------  -----  --------------  ---------------  ----------------  -----------------
        Ethernet0    UC0             N/A              N/A               N/A                N/A
        ...
        
             Port    TxQ    WredDrp/pkts    WredDrp/bytes    EcnMarked/pkts    EcnMarked/bytes
        ---------  -----  --------------  ---------------  ----------------  -----------------
        Ethernet8    UC0             N/A              N/A               N/A                N/A
    """
    
    def _queue_num_to_txq(self, q_num: int) -> str:
        """Convert queue number to TxQ name (UC0-UC7, MC8-MC11)."""
        if q_num < 8:
            return f"UC{q_num}"
        else:
            return f"MC{q_num}"
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format WRED counters as table grouped by interface."""
        headers = ["Port", "TxQ", "WredDrp/pkts", "WredDrp/bytes", "EcnMarked/pkts", "EcnMarked/bytes"]
        
        if not data:
            # Output empty table with headers
            table_str = tabulate([], headers=headers, tablefmt=table_format)
            return table_str
        
        # Group queues by interface
        interfaces = {}
        for queue_key in data.keys():
            if ":" in queue_key:
                port, q_num_str = queue_key.rsplit(":", 1)
                try:
                    q_num = int(q_num_str)
                except ValueError:
                    q_num = 0
            else:
                port = queue_key
                q_num = 0
            
            if port not in interfaces:
                interfaces[port] = []
            interfaces[port].append((q_num, queue_key, data[queue_key]))
        
        # Build output with one paragraph per interface
        paragraphs = []
        
        for port in sorted(interfaces.keys(), key=natural_sort_key):
            # Sort queues by number
            queues = sorted(interfaces[port], key=lambda x: x[0])
            
            table_data = []
            for q_num, queue_key, queue_data in queues:
                txq = self._queue_num_to_txq(q_num)
                row = [
                    port,
                    txq,
                    queue_data.get("WredDrp/pkts", "N/A"),
                    queue_data.get("WredDrp/bytes", "N/A"),
                    queue_data.get("EcnMarked/pkts", "N/A"),
                    queue_data.get("EcnMarked/bytes", "N/A"),
                ]
                table_data.append(row)
            
            col_alignments = ["right", "right", "right", "right", "right", "right"]
            table_str = tabulate(
                table_data,
                headers=headers,
                tablefmt=table_format,
                numalign="right",
                colalign=col_alignments,
            )
            paragraphs.append(table_str)
        
        return "\n\n".join(paragraphs)

