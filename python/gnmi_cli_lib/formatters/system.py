"""
System-related formatters.

This module contains formatters for system commands:
- ClockFormatter: show clock
- UptimeFormatter: show uptime
- VersionFormatter: show version
- SystemMemoryFormatter: show system-memory
- ServicesFormatter: show services
- ProcessesFormatter: show processes
- RebootCauseFormatter: show reboot-cause
- MmuFormatter: show mmu
"""

import re
from typing import Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.common.constants import VERSION_FIELD_ORDER


class ClockFormatter(OutputFormatter):
    """
    Formats clock output - returns the date string as-is.
    
    Example input: {"date": "Mon Jan 26 09:56:41 UTC 2026"}
    Example output: Mon Jan 26 09:56:41 UTC 2026
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format clock date."""
        date_str = str(data.get("date", ""))
        
        if not date_str:
            return ""
        
        return date_str


class UptimeFormatter(OutputFormatter):
    """
    Formats uptime output.
    
    Example input: {"uptime": "up 3 weeks, 4 days, 10 hours, 15 minutes"}
    Example output: up 3 weeks, 4 days, 10 hours, 15 minutes
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format uptime as plain text."""
        return str(data.get("uptime", ""))


class VersionFormatter(OutputFormatter):
    """
    Formats show version output.
    
    Example input:
        {"sonic_software_version": "SONiC.xxx", "platform": "x86_64", ...}
    
    Example output:
        
        SONiC Software Version: SONiC.xxx
        SONiC OS Version: 12
        ...
        Built by: user@host
        
        Platform: x86_64
        ...
        Date: Mon 26 Jan 2026 09:56:39
        
        Docker images:
        ...
    """
    
    # Fields that should have a blank line before them
    BLANK_LINE_BEFORE = {"platform", "docker_images"}
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format version info as key-value pairs with proper blank line separators."""
        lines = []
        
        # Start with blank line
        lines.append("")
        
        for field, label in VERSION_FIELD_ORDER:
            if field in data:
                # Add blank line before specific fields
                if field in self.BLANK_LINE_BEFORE:
                    lines.append("")
                
                value = data[field]
                # Filter out <nil> values
                if value and str(value) != "<nil>":
                    lines.append(f"{label}: {value}")
        
        # Add docker images if present
        if "docker_images" in data:
            lines.append("")  # Blank line before Docker images
            lines.append("Docker images:")
            lines.append("REPOSITORY                 TAG           IMAGE ID       SIZE")
            docker_images = data["docker_images"]
            if isinstance(docker_images, list):
                for img in docker_images:
                    if isinstance(img, dict):
                        repo = img.get("Repository", "N/A")
                        tag = img.get("Tag", "N/A")
                        # Use the full Image ID as provided in JSON
                        img_id = img.get("ID", "N/A")
                        size = img.get("Size", "N/A")
                        # Format with fixed column widths (wider for full sha256)
                        lines.append(f"{repo:26} {tag:13} {img_id} {size}")
        
        return "\n".join(lines)


class SystemMemoryFormatter(OutputFormatter):
    """
    Formats system memory output in 'free' command style.
    
    Example input:
        [{"type": "Mem", "total": "31905", "used": "5418", "free": "21983",
          "shared": "451", "buff/cache": "5421", "available": "26486"}]
    
    Example output:
                   total        used        free      shared  buff/cache   available
        Mem:       31905        5418       21983         451        5421       26486
        Swap:          0           0           0
    """
    
    def format(self, data: Union[Dict, List], config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format system memory in 'free' command style with fixed-width columns."""
        if isinstance(data, dict):
            items = [data] if data else []
        elif isinstance(data, list):
            items = data
        else:
            return ""
        
        if not items:
            return ""
        
        # Build output with fixed column widths matching 'free' command
        # Column end positions: total@19, used@31, free@43, shared@55, buff/cache@67, available@79
        lines = []
        
        # Header line (80 chars total)
        header = "               total        used        free      shared  buff/cache   available"
        lines.append(header)
        
        # Data rows - use fixed column positions
        for item in items:
            if isinstance(item, dict):
                row_type = item.get("type", "")
                total = item.get("total", "")
                used = item.get("used", "")
                free = item.get("free", "")
                shared = item.get("shared", "")
                buff_cache = item.get("buff/cache", "")
                available = item.get("available", "")
                
                if row_type == "Mem":
                    # Full row with all columns
                    # Label: 5 chars, then pad to position 15 for total
                    # total ends at 19 (5 chars from 15-19), used ends at 31, etc.
                    line = f"Mem:{' ' * 11}{total:>5}{' ' * 8}{used:>4}{' ' * 7}{free:>5}{' ' * 9}{shared:>3}{' ' * 8}{buff_cache:>4}{' ' * 7}{available:>5}"
                elif row_type == "Swap":
                    # Swap row only has total, used, free (no shared, buff/cache, available)
                    line = f"Swap:{' ' * 14}{total:>1}{' ' * 11}{used:>1}{' ' * 11}{free:>1}"
                else:
                    # Generic row
                    line = f"{row_type}:{' ' * 11}{total:>5}{' ' * 8}{used:>4}{' ' * 7}{free:>5}{' ' * 9}{shared:>3}{' ' * 8}{buff_cache:>4}{' ' * 7}{available:>5}"
                
                lines.append(line)
        
        return "\n".join(lines)


class ServicesFormatter(OutputFormatter):
    """
    Formats services output.
    
    Example input:
        [{"dockerProcessName": "snmp", 
          "processes": [{"pid": "123", "user": "root", ...}]}]
    
    Example output:
        snmp\tdocker
        ---------------------------
        USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
        root         123  0.1  0.5  12345  1234 ?        S    Jan25   0:00 python snmpd.py
    """
    
    def format(self, data: Union[Dict, List], config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format services output in ps aux style grouped by docker container."""
        if isinstance(data, dict):
            items = [data] if data else []
        elif isinstance(data, list):
            items = data
        else:
            return ""
        
        if not items:
            return ""
        
        lines = []
        for i, service in enumerate(items):
            if isinstance(service, dict):
                docker_name = service.get("dockerProcessName", "Unknown")
                # Header: servicename\tdocker
                lines.append(f"{docker_name}\tdocker")
                # Separator
                lines.append("---------------------------")
                # Column header - ps aux style format (exactly 74 chars)
                lines.append("USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND")
                
                processes = service.get("processes", [])
                for proc in processes:
                    if isinstance(proc, dict):
                        user = proc.get("User", proc.get("user", ""))
                        pid = proc.get("Pid", proc.get("pid", ""))
                        cpu = proc.get("%CPU", proc.get("cpuPercentage", ""))
                        mem = proc.get("%MEM", proc.get("memPercentage", ""))
                        vsz = proc.get("VSZ", proc.get("vsz", ""))
                        rss = proc.get("RSS", proc.get("rss", ""))
                        tty = proc.get("TTY", proc.get("tty", ""))
                        stat = proc.get("STAT", proc.get("stat", ""))
                        start = proc.get("START", proc.get("start", ""))
                        time_val = proc.get("TIME", proc.get("time", ""))
                        command = proc.get("COMMAND", proc.get("command", ""))
                        
                        # Build USER+PID combined (16 chars total, PID right-aligned at end)
                        user_pid = f"{user}{pid:>{16 - len(user)}}"
                        
                        # ps aux format: columns after TTY are fixed-position
                        # STAT starts at position 49, so we need to pad TTY to reach there
                        # Build prefix: USER+PID (16) + "  " (2) + CPU + "  " (2) + MEM + " " + VSZ + " " + RSS + " " + TTY
                        prefix = f"{user_pid}  {cpu}  {mem} {vsz:>6} {rss:>5} {tty}"
                        
                        # Pad to position 49 for STAT (add spaces to reach position 49)
                        if len(prefix) < 49:
                            prefix = prefix + " " * (49 - len(prefix))
                        
                        # STAT (5 chars left-aligned), START (8 chars), TIME (4 chars right), space, COMMAND
                        line = f"{prefix}{stat:<5}{start:<8}{time_val:>4} {command}"
                        lines.append(line)
        
        return "\n".join(lines)


class ProcessesFormatter(OutputFormatter):
    """
    Formats processes output (show processes summary).
    
    Example input:
        [{"PID": "123", "PPID": "1", "CMD": "python", 
          "%MEM": "1.2", "%CPU": "0.5", "STIME": "Jan25",
          "TIME": "0:00:01", "TT": "None", "UID": "0"}]
    
    Example output:
        PID  PPID  CMD     %MEM  %CPU  STIME  TIME     TT    UID
        123     1  python   1.2   0.5  Jan25  0:00:01  None    0
    """
    
    def format(self, data: Union[Dict, List], config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format processes as table with all 9 columns from JSON."""
        if isinstance(data, dict):
            items = list(data.values()) if data else []
        elif isinstance(data, list):
            items = data
        else:
            return ""
        
        if not items:
            return ""
        
        # Maximum width for CMD column (adjust as needed for readability)
        CMD_MAX_WIDTH = 50
        
        table_data = []
        for proc in items:
            if isinstance(proc, dict):
                cmd = proc.get("CMD", "")
                # Truncate CMD with ellipsis if exceeds max width
                if len(cmd) > CMD_MAX_WIDTH:
                    cmd = cmd[:CMD_MAX_WIDTH - 3] + "..."
                row = [
                    proc.get("PID", ""),
                    proc.get("PPID", ""),
                    cmd,
                    proc.get("%MEM", ""),
                    proc.get("%CPU", ""),
                    proc.get("STIME", ""),
                    proc.get("TIME", ""),
                    proc.get("TT", ""),
                    proc.get("UID", ""),
                ]
                table_data.append(row)
        
        if not table_data:
            return ""
        
        headers = ["PID", "PPID", "CMD", "%MEM", "%CPU", "STIME", "TIME", "TT", "UID"]
        return tabulate(table_data, headers=headers, tablefmt=table_format, numalign="right")


class ProcessesCpuFormatter(OutputFormatter):
    """
    Formats 'show processes cpu' output in top-like format.
    
    Example input:
        {"uptime": "09:57:01 up 1 day, ...",
         "tasks": "493 total, 1 running, ...",
         "cpu_usage": "0.0 us, 5.3 sy, ...",
         "memory_usage": "31905.5 total, ...",
         "swap_usage": "0.0 total, ...",
         "processes": [{"pid": "210272", "user": "root", ...}]}
    
    Example output:
        top - 09:56:25 up 1 day,  4:17,  2 users,  load average: 1.24, 0.85, 0.89
        Tasks: 493 total,   2 running, 479 sleeping,   0 stopped,  12 zombie
        %Cpu(s): 15.0 us, 10.0 sy,  0.0 ni, 75.0 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st 
        MiB Mem :  31905.5 total,  21949.7 free,   5453.8 used,   5419.4 buff/cache     
        MiB Swap:      0.0 total,      0.0 free,      0.0 used.  26451.7 avail Mem 
        
            PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
        1257533 root      20   0  132852  58360  19212 S  88.2   0.2   0:00.80 route_c+
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format processes cpu in top-like format."""
        if not data or not isinstance(data, dict):
            return ""
        
        lines = []
        
        # Line 1: top - uptime info
        uptime = data.get("uptime", "")
        if uptime:
            lines.append(f"top - {uptime}")
        
        # Line 2: Tasks
        tasks = data.get("tasks", "")
        if tasks:
            lines.append(f"Tasks: {tasks}")
        
        # Line 3: CPU usage
        cpu_usage = data.get("cpu_usage", "")
        if cpu_usage:
            lines.append(f"%Cpu(s): {cpu_usage}")
        
        # Line 4: Memory usage
        memory_usage = data.get("memory_usage", "")
        if memory_usage:
            lines.append(f"MiB Mem : {memory_usage}")
        
        # Line 5: Swap usage
        swap_usage = data.get("swap_usage", "")
        if swap_usage:
            lines.append(f"MiB Swap: {swap_usage}")
        
        # Blank line
        lines.append("")
        
        # Process table header
        lines.append("    PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND")
        
        # Process rows
        processes = data.get("processes", [])
        for proc in processes:
            if isinstance(proc, dict):
                pid = proc.get("pid", "")
                user = proc.get("user", "")
                pr = proc.get("pr", "")
                ni = proc.get("ni", "")
                virt = proc.get("virt", "")
                res = proc.get("res", "")
                shr = proc.get("shr", "")
                s = proc.get("s", "")
                cpu = proc.get("cpu", "")
                mem = proc.get("mem", "")
                time = proc.get("time", "")
                command = proc.get("command", "")
                
                # Format each field with proper width/alignment to match top output
                # PID: 7 chars right-aligned
                # USER: 9 chars left-aligned (no space after, flows into PR)
                # PR: 3 chars right-aligned
                # NI: 4 chars right-aligned (3 + 1 space before)
                # VIRT: 7 chars right-aligned
                # RES: 6 chars right-aligned
                # SHR: 6 chars right-aligned
                # S: 1 char
                # %CPU: 5 chars right-aligned
                # %MEM: 5 chars right-aligned
                # TIME+: 9 chars right-aligned
                # COMMAND: rest
                
                line = f"{pid:>7} {user:<9}{pr:>3} {ni:>3} {virt:>7} {res:>6} {shr:>6} {s} {cpu:>5} {mem:>5} {time:>9} {command}"
                lines.append(line)
        
        return "\n".join(lines)


class RebootCauseFormatter(OutputFormatter):
    """
    Formats reboot cause output.
    
    Handles two formats:
    1. Current cause: {"cause": "reboot", "time": "...", "user": "admin"}
    2. History: {"2026_01_24_06_59_57": {"cause": "...", ...}} or {"REBOOT_CAUSE|timestamp": {...}}
    
    Example output (current):
        Cause: reboot
        Time: Thu Jul 10 08:05:33 PM UTC 2025
        User: admin
    
    Example output (history):
        Name                 Cause    Time                             User    Comment
        -------------------  -------  -------------------------------  ------  ---------
        2025_07_10_20_12_49  reboot   Thu Jul 10 08:10:49 PM UTC...    admin   N/A
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format reboot cause."""
        history_entries = []
        current_entry = None
        
        for key, value in data.items():
            # Check if it's a history entry (timestamp key pattern or REBOOT_CAUSE| prefix)
            if isinstance(value, dict) and "cause" in value:
                # Extract timestamp from key
                if key.startswith("REBOOT_CAUSE|"):
                    timestamp_key = key.replace("REBOOT_CAUSE|", "")
                else:
                    # Direct timestamp key like "2026_01_24_06_59_57"
                    timestamp_key = key
                
                # Display user value as-is from JSON (including $USER$ placeholder)
                user = value.get("user", "N/A")
                
                entry = {
                    "name": timestamp_key,
                    "cause": value.get("cause", "Unknown"),
                    "time": value.get("time", "N/A"),
                    "user": user,
                    "comment": value.get("comment", "N/A"),
                }
                history_entries.append(entry)
            elif key in ["cause", "time", "user", "comment"]:
                if current_entry is None:
                    current_entry = {}
                current_entry[key] = value
        
        # If it's current cause (simple format)
        if current_entry and not history_entries:
            cause = current_entry.get("cause", "Unknown")
            time_str = current_entry.get("time", "")
            user = current_entry.get("user", "")
            result = f"Cause: {cause}"
            if time_str:
                result += f"\nTime: {time_str}"
            if user:
                result += f"\nUser: {user}"
            return result
        
        # History format - table
        if history_entries:
            # Sort by timestamp descending
            history_entries.sort(key=lambda x: x["name"], reverse=True)
            
            # Calculate column widths (minimum widths based on CLI output)
            # Name: 19, Cause: 7, Time: 31, User: 6, Comment: 9
            col_widths = [19, 7, 31, 6, 9]
            headers = ["Name", "Cause", "Time", "User", "Comment"]
            
            # Adjust widths based on actual data
            for entry in history_entries:
                col_widths[0] = max(col_widths[0], len(entry["name"]))
                col_widths[1] = max(col_widths[1], len(entry["cause"]))
                col_widths[2] = max(col_widths[2], len(entry["time"]))
                col_widths[3] = max(col_widths[3], len(entry["user"]))
                col_widths[4] = max(col_widths[4], len(entry["comment"]))
            
            # Build table with proper column widths
            lines = []
            
            # Header line
            header_parts = [h.ljust(w) for h, w in zip(headers, col_widths)]
            lines.append("  ".join(header_parts).rstrip())
            
            # Separator line
            sep_parts = ["-" * w for w in col_widths]
            lines.append("  ".join(sep_parts))
            
            # Data rows
            for entry in history_entries:
                row = [
                    entry["name"].ljust(col_widths[0]),
                    entry["cause"].ljust(col_widths[1]),
                    entry["time"].ljust(col_widths[2]),
                    entry["user"].ljust(col_widths[3]),
                    entry["comment"].ljust(col_widths[4]),
                ]
                lines.append("  ".join(row).rstrip())
            
            return "\n".join(lines)
        
        return ""


class MmuFormatter(OutputFormatter):
    """
    Formats MMU configuration output.
    
    Example input:
        {"pools": {"egress_lossless_pool": {"mode": "static", "size": "147226020", "type": "egress"}},
         "profiles": {"egress_lossless_profile": {"pool": "egress_lossless_pool", "size": "0", "static_th": "165364160"}}}
    
    Example output:
        Pool: egress_lossless_pool
        ----  ---------
        mode  static
        size  147226020
        type  egress
        ----  ---------

        Profile: egress_lossless_profile
        ---------  --------------------
        pool       egress_lossless_pool
        size       0
        static_th  165364160
        ---------  --------------------
    """
    
    def format(self, data: Dict, config: CommandConfig,
               table_format: str = "simple",
               options: Optional[Dict] = None) -> str:
        """Format MMU config with Pool and Profile sections."""
        if not data:
            return ""
        
        output_blocks = []
        
        # Process pools
        pools = data.get("pools", data.get("pool", {}))
        if pools:
            for pool_name in sorted(pools.keys()):
                pool_data = pools[pool_name]
                if isinstance(pool_data, dict):
                    block = self._format_block("Pool", pool_name, pool_data)
                    output_blocks.append(block)
        
        # Process profiles
        profiles = data.get("profiles", data.get("profile", {}))
        if profiles:
            for profile_name in sorted(profiles.keys()):
                profile_data = profiles[profile_name]
                if isinstance(profile_data, dict):
                    block = self._format_block("Profile", profile_name, profile_data)
                    output_blocks.append(block)
        
        return "\n\n".join(output_blocks)
    
    def _format_block(self, block_type: str, name: str, data: Dict) -> str:
        """Format a single Pool or Profile block with table-style output."""
        lines = []
        
        # Header line
        lines.append(f"{block_type}: {name}")
        
        # Build table rows (key-value pairs, sorted by key)
        rows = []
        for key in sorted(data.keys()):
            value = data[key]
            rows.append([key, str(value)])
        
        if rows:
            # Calculate column widths for separator
            max_key_len = max(len(row[0]) for row in rows)
            max_val_len = max(len(row[1]) for row in rows)
            
            # Separator line with dashes
            sep_line = f"{'-' * max_key_len}  {'-' * max_val_len}"
            lines.append(sep_line)
            
            # Data rows
            for key, value in rows:
                lines.append(f"{key:<{max_key_len}}  {value}")
            
            # Closing separator
            lines.append(sep_line)
        
        return "\n".join(lines)

