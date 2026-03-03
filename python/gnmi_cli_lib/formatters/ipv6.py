"""
IPv6-related formatters.

This module contains formatters for IPv6 commands:
- IPv6BgpNeighborsFormatter: show ipv6 bgp neighbors
- IPv6BgpNetworkFormatter: show ipv6 bgp network (passthrough)
- IPv6BgpSummaryFormatter: show ipv6 bgp summary
- IPv6FibFormatter: show ipv6 fib
- IPv6InterfacesFormatter: show ipv6 interfaces
- IPv6LinkLocalModeFormatter: show ipv6 link-local-mode
- IPv6PrefixListFormatter: show ipv6 prefix-list
- IPv6ProtocolFormatter: show ipv6 protocol
"""

from typing import Dict, List, Optional, Union
from tabulate import tabulate

from gnmi_cli_lib.formatters.base import OutputFormatter
from gnmi_cli_lib.common.types import CommandConfig
from gnmi_cli_lib.utils.sorting import natural_sort_key


class IPv6BgpNeighborsFormatter(OutputFormatter):
    """
    Formats IPv6 BGP neighbors output in detailed multi-line format.
    
    Each neighbor gets a detailed block with all properties.
    
    Example output:
        BGP neighbor is fc00::2, remote AS 4200300000, local AS 4200200000, confed-external link
          Local Role: undefined
          Remote Role: undefined
         Description: ARISTA01FT2
         Member of peer-group FABRICSPINE_V6 for session parameters
          BGP version 4, remote router ID 100.1.0.1, local router ID 10.1.0.32
          ...
    """
    
    # View types that require route table format instead of neighbor details
    ROUTE_VIEW_TYPES = {"advertised-routes", "received-routes", "routes"}
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format BGP neighbors as detailed multi-line output.
        
        Detects view type from the data structure:
        - If data contains 'advertisedRoutes', 'receivedRoutes', 'routes', 
          'bgpTableVersion', or 'tableVersion' keys, delegate to route table formatter
        - Otherwise format as neighbor details
        """
        if not data:
            return ""
        
        # Check if this is a routes view by looking at the data structure
        # advertised-routes/received-routes use: bgpTableVersion, advertisedRoutes, etc.
        # routes view uses: tableVersion, routes, routerId, etc.
        if isinstance(data, dict):
            route_view_keys = ["advertisedRoutes", "receivedRoutes", "bgpTableVersion", 
                              "tableVersion", "routerId"]
            # Also check for 'routes' key but only if it's a dict of prefixes (not neighbor data)
            has_routes_dict = ("routes" in data and isinstance(data.get("routes"), dict) and
                              any("/" in k or "::" in k for k in data.get("routes", {}).keys()))
            
            if any(key in data for key in route_view_keys) or has_routes_dict:
                # This is a routes table view, delegate to routes formatter
                routes_formatter = IPv6BgpRoutesTableFormatter()
                return routes_formatter.format(data, config, table_format, options)
        
        lines = []
        
        # Sort neighbors by IP address using proper IPv6 sorting
        neighbors = sorted(data.keys(), key=self._ipv6_sort_key)
        
        for neighbor_ip in neighbors:
            neighbor_data = data[neighbor_ip]
            if not isinstance(neighbor_data, dict):
                continue
            
            # Build the detailed output for this neighbor
            lines.append(self._format_neighbor(neighbor_ip, neighbor_data))
        
        return "\n".join(lines)
    
    def _ipv6_sort_key(self, addr: str) -> tuple:
        """Generate a sortable key for IPv6 addresses."""
        import ipaddress
        try:
            return (0, ipaddress.ip_address(addr).packed)
        except ValueError:
            # If it's not a valid IP, sort it at the end alphabetically
            return (1, addr.encode())
    
    def _format_neighbor(self, neighbor_ip: str, data: Dict) -> str:
        """Format a single neighbor's detailed output."""
        lines = []
        
        remote_as = data.get("remoteAs", "N/A")
        local_as = data.get("localAs", "N/A")
        external_link = "external link" if data.get("nbrExternalLink", False) else "confed-external link"
        
        # First line
        lines.append(f"BGP neighbor is {neighbor_ip}, remote AS {remote_as}, local AS {local_as}, {external_link}")
        
        # Local/Remote roles
        if "localRole" in data or "remoteRole" in data:
            local_role = data.get("localRole", "undefined")
            remote_role = data.get("remoteRole", "undefined")
            lines.append(f"  Local Role: {local_role}")
            lines.append(f"  Remote Role: {remote_role}")
        
        # Description
        desc = data.get("nbrDesc", "")
        if desc:
            lines.append(f" Description: {desc}")
        
        # Peer group
        peer_group = data.get("peerGroup", "")
        if peer_group:
            lines.append(f" Member of peer-group {peer_group} for session parameters")
        
        # BGP version, router IDs
        bgp_version = data.get("bgpVersion", 4)
        remote_router_id = data.get("remoteRouterId", "")
        local_router_id = data.get("localRouterId", "")
        lines.append(f"  BGP version {bgp_version}, remote router ID {remote_router_id}, local router ID {local_router_id}")
        
        # Neighbor under common administration (confederation)
        if not data.get("nbrExternalLink", False):
            lines.append("  Neighbor under common administration")
        
        # Hostname
        hostname = data.get("hostname", "")
        if hostname and hostname != "Unknown":
            lines.append(f"  Hostname: {hostname}")
        
        # Software version
        sw_version = data.get("softwareVersion", "")
        if sw_version and sw_version != "n/a":
            lines.append(f"  Software Version: {sw_version}")
        
        # BGP state
        bgp_state = data.get("bgpState", "")
        up_time = data.get("bgpTimerUpString", "")
        bgp_up_msec = data.get("bgpTimerUpMsec", 0)
        if bgp_state:
            lines.append(f"  BGP state = {bgp_state}, up for {up_time}")
        
        # BGP In Update elapsed time
        in_update_elapsed = data.get("bgpInUpdateElapsedTimeMsecs", 0)
        if in_update_elapsed:
            lines.append(f"  Last update from {neighbor_ip} {self._format_uptime(in_update_elapsed)} ago")
        
        # Last read/write
        last_read = data.get("bgpTimerLastRead", 0)
        last_write = data.get("bgpTimerLastWrite", 0)
        lines.append(f"  Last read {self._format_time(last_read)}, Last write {self._format_time(last_write)}")
        
        # Hold time, keepalive
        hold_time = data.get("bgpTimerHoldTimeMsecs", 0) // 1000
        keepalive = data.get("bgpTimerKeepAliveIntervalMsecs", 0) // 1000
        lines.append(f"  Hold time is {hold_time}, keepalive interval is {keepalive} seconds")
        
        # Configured hold/keepalive
        cfg_hold = data.get("bgpTimerConfiguredHoldTimeMsecs", 0) // 1000
        cfg_keepalive = data.get("bgpTimerConfiguredKeepAliveIntervalMsecs", 0) // 1000
        lines.append(f"  Configured hold time is {cfg_hold}, keepalive interval is {cfg_keepalive} seconds")
        
        # TCP MSS
        if "bgpTcpMssConfigured" in data or "bgpTcpMssSynced" in data:
            tcp_mss_cfg = data.get("bgpTcpMssConfigured", 0)
            tcp_mss_synced = data.get("bgpTcpMssSynced", 0)
            lines.append(f"  Configured tcp-mss is {tcp_mss_cfg}, synced tcp-mss is {tcp_mss_synced}")
        
        # Conditional advertisements interval
        if "bgpTimerConfiguredConditionalAdvertisementsSec" in data:
            cond_adv = data.get("bgpTimerConfiguredConditionalAdvertisementsSec", 60)
            lines.append(f"  Configured conditional advertisements interval is {cond_adv} seconds")
        
        # TCP MSS already handled above
        
        # BGP timer up established epoch (Unix timestamp)
        up_epoch = data.get("bgpTimerUpEstablishedEpoch", 0)
        if up_epoch:
            lines.append(f"  Peer established epoch: {up_epoch}")
        
        # Extended optional parameters length
        ext_opt_params = data.get("extendedOptionalParametersLength", False)
        if ext_opt_params:
            lines.append("  Extended Optional Parameters Length: Enabled")
        
        # External BGP neighbor max hops
        max_hops = data.get("externalBgpNbrMaxHopsAway", 0)
        if max_hops and data.get("nbrExternalLink", False):
            lines.append(f"  External BGP neighbor may be up to {max_hops} hops away.")
        
        # Neighbor capabilities
        caps = data.get("neighborCapabilities", {})
        if caps:
            lines.append("  Neighbor capabilities:")
            lines.extend(self._format_capabilities(caps))
        
        # Graceful restart info
        gr_info = data.get("gracefulRestartInfo", {})
        if gr_info:
            lines.extend(self._format_graceful_restart(gr_info))
        
        # Message statistics
        msg_stats = data.get("messageStats", {})
        if msg_stats:
            lines.extend(self._format_message_stats(msg_stats))
        
        # Minimum time between advertisement runs
        min_adv_time = data.get("minBtwnAdvertisementRunsTimerMsecs", 0) // 1000
        lines.append(f"  Minimum time between advertisement runs is {min_adv_time} seconds")
        lines.append("")
        
        # Address family info - look inside addressFamilyInfo dict
        af_info = data.get("addressFamilyInfo", {})
        for af_name, af_data in af_info.items():
            if isinstance(af_data, dict):
                lines.extend(self._format_address_family(af_name, af_data))
        
        # Connection established/dropped info
        conn_est = data.get("connectionsEstablished", 0)
        conn_drop = data.get("connectionsDropped", 0)
        lines.append(f"  Connections established {conn_est}; dropped {conn_drop}")
        
        # Last reset info
        last_reset_msec = data.get("lastResetTimerMsecs", 0)
        last_reset_reason = data.get("lastResetDueTo", "")
        if last_reset_msec or last_reset_reason:
            reset_time = self._format_uptime(last_reset_msec)
            lines.append(f"  Last reset {reset_time},  {last_reset_reason}")
        
        # --- Bottom section (no indent, after connections/last reset) ---
        # Host local/foreign addresses and ports
        host_local = data.get("hostLocal", "")
        host_foreign = data.get("hostForeign", "")
        port_local = data.get("portLocal", 0)
        port_foreign = data.get("portForeign", 0)
        if host_local and host_foreign:
            lines.append(f"Local host: {host_local}, Local port: {port_local}")
            lines.append(f"Foreign host: {host_foreign}, Foreign port: {port_foreign}")
        
        # Nexthop information
        nexthop = data.get("nexthop", "")
        nexthop_global = data.get("nexthopGlobal", "")
        nexthop_local = data.get("nexthopLocal", "")
        if nexthop:
            lines.append(f"Nexthop: {nexthop}")
        if nexthop_global:
            lines.append(f"Nexthop global: {nexthop_global}")
        if nexthop_local:
            lines.append(f"Nexthop local: {nexthop_local}")
        
        # BGP connection type
        bgp_conn = data.get("bgpConnection", "")
        if bgp_conn:
            import re as _re
            bgp_conn_formatted = _re.sub(r'([a-z])([A-Z])', r'\1 \2', bgp_conn).lower()
            lines.append(f"BGP connection: {bgp_conn_formatted}")
        
        # Connect retry timer
        connect_retry = data.get("connectRetryTimer", 0)
        if connect_retry:
            lines.append(f"BGP Connect Retry Timer in Seconds: {connect_retry}")
        
        # Estimated RTT
        rtt = data.get("estimatedRttInMsecs", 0)
        if rtt:
            lines.append(f"Estimated round trip time: {rtt} ms")
        
        # Read/write thread status + FD
        read_thread = data.get("readThread", "")
        write_thread = data.get("writeThread", "")
        fd_used = data.get("fdUsed", "")
        if read_thread or write_thread:
            fd_str = f"  FD used: {fd_used}" if fd_used != "" else ""
            lines.append(f"Read thread: {read_thread}  Write thread: {write_thread}{fd_str}")
        
        return "\n".join(lines)
    
    def _format_time(self, msecs: int) -> str:
        """Format milliseconds as HH:MM:SS."""
        seconds = msecs // 1000
        hours = seconds // 3600
        minutes = (seconds % 3600) // 60
        secs = seconds % 60
        return f"{hours:02d}:{minutes:02d}:{secs:02d}"
    
    def _format_uptime(self, msecs: int) -> str:
        """Format milliseconds as uptime string like '1d00h33m' or '04:43:30'."""
        if not msecs:
            return "never"
        seconds = msecs // 1000
        days = seconds // 86400
        hours = (seconds % 86400) // 3600
        minutes = (seconds % 3600) // 60
        if days > 0:
            return f"{days}d{hours:02d}h{minutes:02d}m"
        else:
            secs = seconds % 60
            return f"{hours:02d}:{minutes:02d}:{secs:02d}"
    
    def _format_capabilities(self, caps: Dict) -> List[str]:
        """Format neighbor capabilities."""
        lines = []
        
        # 4 byte AS
        four_byte = caps.get("4byteAs", "")
        if four_byte:
            lines.append(f"    4 Byte AS: {self._format_cap_value(four_byte)}")
        
        # Extended message
        ext_msg = caps.get("extendedMessage", "")
        if ext_msg:
            lines.append(f"    Extended Message: {ext_msg}")
        
        # Add Path
        add_path = caps.get("addPath", {})
        if add_path:
            lines.append("    AddPath:")
            for af, af_data in add_path.items():
                if isinstance(af_data, dict):
                    af_name = self._format_af_name(af)
                    if af_data.get("rxAdvertisedAndReceived"):
                        lines.append(f"      {af_name}: RX advertised {af_name} and received")
        
        # Long-lived GR
        llgr = caps.get("longLivedGracefulRestart", "")
        if llgr:
            lines.append(f"    Long-lived Graceful Restart: {llgr}")
        
        # Route refresh
        route_ref = caps.get("routeRefresh", "")
        if route_ref:
            lines.append(f"    Route refresh: {self._format_cap_value(route_ref)}")
        
        # Enhanced route refresh
        enh_route_ref = caps.get("enhancedRouteRefresh", "")
        if enh_route_ref:
            lines.append(f"    Enhanced Route Refresh: {self._format_cap_value(enh_route_ref)}")
        
        # Multiprotocol extensions
        mp_ext = caps.get("multiprotocolExtensions", {})
        if mp_ext:
            for af, status in mp_ext.items():
                if status:
                    lines.append(f"    Address Family {self._format_af_name(af)}: {self._format_cap_value(status)}")
        
        # Hostname capability
        hostname = caps.get("hostName", {})
        if hostname and isinstance(hostname, dict):
            local_name = hostname.get("advHostName", "")
            domain = hostname.get("advDomainName", "n/a")
            received = hostname.get("rcvHostName", "")
            recv_str = "received" if received else "not received"
            if local_name:
                lines.append(f"    Hostname Capability: advertised (name: {local_name},domain name: {domain}) {recv_str}")
        
        # Software version capability
        sw_version = caps.get("softwareVersion", {})
        if sw_version and isinstance(sw_version, dict):
            lines.append("    Version Capability: not advertised not received")
        
        # Graceful restart capability
        gr_cap = caps.get("gracefulRestart", "")
        if gr_cap:
            # gr_cap is a string value like "advertisedAndReceived"
            lines.append(f"    Graceful Restart Capability: {self._format_cap_value(gr_cap)}")
            remote_timer = caps.get("gracefulRestartRemoteTimerMsecs", 0) // 1000
            if remote_timer:
                lines.append(f"      Remote Restart timer is {remote_timer} seconds")
            
            # Address families by peer
            af_by_peer = caps.get("addressFamiliesByPeer", {})
            if af_by_peer:
                lines.append("      Address families by peer:")
                if isinstance(af_by_peer, str):
                    # Value is a string like "none" - output as-is
                    lines.append(f"        {af_by_peer}")
                else:
                    for af in af_by_peer:
                        lines.append(f"        {self._format_af_name(af)}(not preserved)")
        
        return lines
    
    def _format_cap_value(self, value) -> str:
        """Format capability value."""
        if isinstance(value, dict):
            # Handle dict like {'advertisedAndReceived': True}
            if value.get("advertisedAndReceived"):
                return "advertised and received"
            elif value.get("advertised"):
                return "advertised"
            elif value.get("received"):
                return "received"
            return "advertised and received"  # Default for true dict values
        if value == "advertisedAndReceived":
            return "advertised and received"
        elif value == "advertisedAndReceivedNew":
            return "advertised and received(new)"
        elif value == "advertised":
            return "advertised"
        elif value == "received":
            return "received"
        return str(value)
    
    def _format_af_name(self, af_key: str) -> str:
        """Format address family name."""
        mapping = {
            "ipv6Unicast": "IPv6 Unicast",
            "ipv4Unicast": "IPv4 Unicast",
        }
        return mapping.get(af_key, af_key)
    
    def _format_graceful_restart(self, gr_info: Dict) -> List[str]:
        """Format graceful restart information."""
        lines = ["  Graceful restart information:"]
        
        # End-of-RIB info
        eor_send = gr_info.get("endOfRibSend", {})
        eor_recv = gr_info.get("endOfRibRecv", {})
        
        for af in eor_send:
            lines.append(f"    End-of-RIB send: {self._format_af_name(af)}")
        for af in eor_recv:
            lines.append(f"    End-of-RIB received: {self._format_af_name(af)}")
        
        # Local/Remote GR mode
        local_mode = gr_info.get("localGrMode", "")
        remote_mode = gr_info.get("remoteGrMode", "")
        if local_mode:
            lines.append(f"    Local GR Mode: {local_mode}")
        if remote_mode:
            lines.append(f"    Remote GR Mode: {remote_mode}")
        
        # R bit and N bit
        r_bit = gr_info.get("rBit", False)
        lines.append(f"    R bit: {r_bit}")
        if "nBit" in gr_info:
            n_bit = gr_info.get("nBit", False)
            lines.append(f"    N bit: {n_bit}")
        
        # Timers
        timers = gr_info.get("timers", {})
        if timers:
            lines.append("    Timers:")
            cfg_restart = timers.get("configuredRestartTimer", 0)
            rcv_restart = timers.get("receivedRestartTimer", 0)
            lines.append(f"      Configured Restart Time(sec): {cfg_restart}")
            lines.append(f"      Received Restart Time(sec): {rcv_restart}")
            if "configuredLlgrStaleTime" in timers:
                cfg_llgr = timers.get("configuredLlgrStaleTime", 0)
                lines.append(f"      Configured LLGR Stale Path Time(sec): {cfg_llgr}")
        
        # Per-AFI graceful restart info
        for af_key in ["ipv6Unicast", "ipv4Unicast"]:
            af_gr = gr_info.get(af_key, {})
            if af_gr:
                lines.append(f"    {self._format_af_name(af_key)}:")
                f_bit = af_gr.get("fBit", False)
                lines.append(f"      F bit: {f_bit}")
                
                eor_status = af_gr.get("endOfRibStatus", {})
                if eor_status:
                    eor_sent = "Yes" if eor_status.get("endOfRibSend", False) else "No"
                    eor_sent_after = "Yes" if eor_status.get("endOfRibSentAfterUpdate", False) else "No"
                    eor_recv = "Yes" if eor_status.get("endOfRibRecv", False) else "No"
                    lines.append(f"      End-of-RIB sent: {eor_sent}")
                    lines.append(f"      End-of-RIB sent after update: {eor_sent_after}")
                    lines.append(f"      End-of-RIB received: {eor_recv}")
                
                af_timers = af_gr.get("timers", {})
                if af_timers:
                    lines.append("      Timers:")
                    stale_path = af_timers.get("stalePathTimer", 0)
                    lines.append(f"        Configured Stale Path Time(sec): {stale_path}")
                    if "selectionDeferralTimer" in af_timers:
                        sel_def = af_timers.get("selectionDeferralTimer", 0)
                        lines.append(f"        Configured Selection Deferral Time(sec): {sel_def}")
                    elif "llgrStaleTime" in af_timers:
                        llgr_stale = af_timers.get("llgrStaleTime", 0)
                        lines.append(f"        LLGR Stale Path Time(sec): {llgr_stale}")
        
        return lines
    
    def _format_message_stats(self, stats: Dict) -> List[str]:
        """Format message statistics."""
        lines = ["  Message statistics:"]
        
        # Input/output queues (gNMI uses depthInq/depthOutq)
        inq = stats.get("depthInq", 0)
        outq = stats.get("depthOutq", 0)
        lines.append(f"    Inq depth is {inq}")
        lines.append(f"    Outq depth is {outq}")
        
        # Header line
        lines.append("                         Sent       Rcvd")
        
        # Message types - gNMI uses camelCase field names
        msg_types = [
            ("Opens", "opensSent", "opensRecv"),
            ("Notifications", "notificationsSent", "notificationsRecv"),
            ("Updates", "updatesSent", "updatesRecv"),
            ("Keepalives", "keepalivesSent", "keepalivesRecv"),
            ("Route Refresh", "routeRefreshSent", "routeRefreshRecv"),
            ("Capability", "capabilitySent", "capabilityRecv"),
            ("Total", "totalSent", "totalRecv"),
        ]
        
        for label, sent_key, recv_key in msg_types:
            sent = stats.get(sent_key, 0)
            recv = stats.get(recv_key, 0)
            label_colon = f"{label}:"
            lines.append(f"    {label_colon:<18}{sent:>7}{recv:>11}")
        
        return lines
    
    def _format_address_family(self, af_name: str, af_data: Dict) -> List[str]:
        """Format address family information."""
        lines = [f" For address family: {self._format_af_name(af_name)}"]
        
        # Peer group member
        peer_group = af_data.get("peerGroupMember", "")
        if peer_group:
            lines.append(f"  {peer_group} peer-group member")
        
        # Update group, subgroup
        update_group = af_data.get("updateGroupId", "")
        subgroup = af_data.get("subGroupId", "")
        if update_group:
            lines.append(f"  Update group {update_group}, subgroup {subgroup}")
        
        # Packet queue length
        pkt_queue = af_data.get("packetQueueLength", 0)
        lines.append(f"  Packet Queue length {pkt_queue}")
        
        # Inbound soft reconfig
        if af_data.get("inboundSoftConfigPermit", False):
            lines.append("  Inbound soft reconfiguration allowed")
        
        # Allow AS in
        allow_as_in = af_data.get("allowAsInCount", 0)
        if allow_as_in:
            lines.append(f"  Local AS allowed in path, {allow_as_in} occurrences")
        
        # NEXT_HOP setting (if present)
        # Community attribute
        comm_attr = af_data.get("commAttriSentToNbr", "")
        if comm_attr:
            lines.append(f"  Community attribute sent to this neighbor({comm_attr})")
        
        # Path policy config
        if af_data.get("inboundPathPolicyConfig", False):
            lines.append("  Inbound path policy configured")
        if af_data.get("outboundPathPolicyConfig", False):
            lines.append("  Outbound path policy configured")
        
        # Route maps
        in_routemap = af_data.get("routeMapForIncomingAdvertisements", "")
        out_routemap = af_data.get("routeMapForOutgoingAdvertisements", "")
        if in_routemap:
            lines.append(f"  Route map for incoming advertisements is *{in_routemap}")
        if out_routemap:
            lines.append(f"  Route map for outgoing advertisements is *{out_routemap}")
        
        # Accepted/sent prefixes
        accepted = af_data.get("acceptedPrefixCounter", 0)
        sent = af_data.get("sentPrefixCounter", 0)
        if sent:
            lines.append(f"  {accepted} accepted prefixes, {sent} sent prefixes")
        else:
            lines.append(f"  {accepted} accepted prefixes")
        
        # Maximum prefixes
        max_prefix = af_data.get("prefixAllowedMax", 0)
        warning_only = af_data.get("prefixAllowedMaxWarning", False)
        warning_thresh = af_data.get("prefixAllowedWarningThresh", 0)
        if max_prefix:
            warn_str = "(warning-only)" if warning_only else ""
            lines.append(f"  Maximum prefixes allowed {max_prefix} {warn_str}")
            if warning_thresh:
                lines.append(f"  Threshold for warning message {warning_thresh}%")
        
        lines.append("")  # Empty line
        
        return lines


class IPv6BgpRoutesTableFormatter(OutputFormatter):
    """
    Formats IPv6 BGP routes table output for advertised-routes, received-routes, routes views.
    
    This formatter handles the route table output when a neighbor IP and view type
    (advertised-routes, received-routes, routes) are specified.
    
    Example output:
        BGP table version is 91647, local router ID is 10.1.0.32, vrf id 0
        Default local pref 100, local AS 65100
        Status codes:  s suppressed, d damped, h history, u unsorted, * valid, > best, = multipath,
                       i internal, r RIB-failure, S Stale, R Removed
        Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self
        Origin codes:  i - IGP, e - EGP, ? - incomplete
        RPKI validation codes: V valid, I invalid, N Not found

            Network          Next Hop            Metric LocPrf Weight Path
         *> ::/0             ::                                     0 64600 65534 6666 6667 i
         *> 2064:100:0:41::/128
                             ::                                     0 64600 i
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format BGP routes table."""
        if not data:
            return ""
        
        lines = []
        
        # Header info - handle both JSON structures
        # advertisedRoutes uses bgpTableVersion, routes uses tableVersion
        table_version = data.get("bgpTableVersion", data.get("tableVersion", 0))
        router_id = data.get("bgpLocalRouterId", data.get("routerId", ""))
        local_pref = data.get("defaultLocPrf", 100)
        local_as = data.get("localAS", "")
        vrf_id = data.get("vrfId", 0)
        
        lines.append(f"BGP table version is {table_version}, local router ID is {router_id}, vrf id {vrf_id}")
        lines.append(f"Default local pref {local_pref}, local AS {local_as}")
        
        # Status codes
        lines.append("Status codes:  s suppressed, d damped, h history, u unsorted, * valid, > best, = multipath,")
        lines.append("               i internal, r RIB-failure, S Stale, R Removed")
        lines.append("Nexthop codes: @NNN nexthop's vrf id, < announce-nh-self")
        lines.append("Origin codes:  i - IGP, e - EGP, ? - incomplete")
        lines.append("RPKI validation codes: V valid, I invalid, N Not found")
        lines.append("")
        
        # Column header
        lines.append("    Network          Next Hop            Metric LocPrf Weight Path")
        
        # Determine which routes dict to use
        routes = {}
        is_list_format = False  # routes view uses list per prefix, others use dict
        if "advertisedRoutes" in data:
            routes = data["advertisedRoutes"]
        elif "receivedRoutes" in data:
            routes = data["receivedRoutes"]
        elif "routes" in data:
            routes = data["routes"]
            # If routes dict is empty, CLI shows nothing
            if not routes:
                return ""
            is_list_format = True  # routes view has list of paths per prefix
        else:
            # Try treating the whole data as routes
            routes = {k: v for k, v in data.items() 
                     if isinstance(v, (dict, list)) and k not in ["bgpTableVersion", "bgpLocalRouterId", 
                         "defaultLocPrf", "localAS", "tableVersion", "routerId", "vrfId", "vrfName",
                         "totalPrefixCounter", "filteredPrefixCounter"]}
        
        # Sort routes by network prefix
        sorted_prefixes = sorted(routes.keys(), key=self._prefix_sort_key)
        
        for prefix in sorted_prefixes:
            route_info = routes[prefix]
            
            if is_list_format and isinstance(route_info, list):
                # routes view: each prefix has a list of paths
                for path_info in route_info:
                    if isinstance(path_info, dict):
                        lines.append(self._format_route(prefix, path_info))
            elif isinstance(route_info, dict):
                lines.append(self._format_route(prefix, route_info))
        
        # Total counts - use "Displayed X routes" format, add "and Y total paths" only if available
        total_prefixes = data.get("totalPrefixCounter", len(routes))
        total_paths = data.get("totalPathCounter")  # Only include if explicitly provided
        
        lines.append("")
        if total_paths is not None:
            lines.append(f"Displayed {total_prefixes} routes and {total_paths} total paths")
        else:
            lines.append(f"Displayed {total_prefixes} routes")
        
        return "\n".join(lines)
    
    def _prefix_sort_key(self, prefix: str) -> tuple:
        """Generate a sortable key for IPv6 prefixes."""
        import ipaddress
        try:
            net = ipaddress.ip_network(prefix, strict=False)
            return (0, net.network_address.packed, net.prefixlen)
        except ValueError:
            return (1, prefix.encode(), 0)
    
    def _format_route(self, prefix: str, route: Dict) -> str:
        """Format a single route entry."""
        # Status flags
        status = " "
        if route.get("valid", False):
            status += "*"
        else:
            status += " "
        # bestpath can be 'best' or 'bestpath' depending on JSON source
        if route.get("best", False) or route.get("bestpath", False):
            status += ">"
        elif route.get("multipath", False):
            status += "="
        else:
            status += " "
        
        network = route.get("network", prefix)
        # Next hop can be in different fields depending on JSON source
        next_hop = route.get("nextHopGlobal", route.get("nextHop", ""))
        if not next_hop:
            # routes view uses nexthops array
            nexthops = route.get("nexthops", [])
            if nexthops and isinstance(nexthops, list) and nexthops[0]:
                next_hop = nexthops[0].get("ip", "::")
            else:
                next_hop = "::"
        
        metric = route.get("metric", "")
        loc_prf = route.get("locPrf", "")
        weight = route.get("weight", 0)
        path = route.get("path", "")
        origin = self._format_origin(route.get("origin", ""))
        
        # Format the line - network may be long so use continuation
        # CLI column layout (determined by header positions):
        #   Status: 4 chars (positions 0-3: " *= " or similar)
        #   Network: 17 chars (positions 4-20)
        #   Next Hop: 20 chars (positions 21-40)
        #   Metric: 7 chars (positions 41-47)
        #   LocPrf: 7 chars (positions 48-54)
        #   Weight: 6 chars (positions 55-60)
        #   Space: 1 char (position 61)
        #   Path + Origin: rest (positions 62+)
        # Networks with length < 17 fit on one line; networks with length >= 17
        # go to continuation (because 17-char network leaves no padding before next_hop)
        if len(network) < 17:
            # Short prefix - fits on one line
            line = f"{status} {network:<17}{next_hop:<20}{metric:>7}{loc_prf:>7}{weight:>6} {path} {origin}"
        else:
            # Long prefix - network on first line, rest on continuation
            # Continuation indent: 20 spaces (aligns with Next Hop at position 21)
            line = f"{status} {network}\n                    {next_hop:<20}{metric:>7}{loc_prf:>7}{weight:>6} {path} {origin}"
        
        return line.rstrip()
    
    def _format_origin(self, origin: str) -> str:
        """Format origin code."""
        if origin == "IGP" or origin == "i":
            return "i"
        elif origin == "EGP" or origin == "e":
            return "e"
        elif origin == "incomplete" or origin == "?":
            return "?"
        return origin


class IPv6BgpNetworkFormatter(OutputFormatter):
    """
    Formats IPv6 BGP network output (passthrough).
    
    The data contains pre-formatted output from FRR, just extract and return it.
    
    Example input:
        {"output": "BGP table version is 135211, local router ID is 10.1.0.32..."}
    
    Example output:
        BGP table version is 135211, local router ID is 10.1.0.32, vrf id 0
        ...
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format BGP network as passthrough text."""
        if not data:
            return ""
        
        # Extract the output field - it contains the pre-formatted text
        if isinstance(data, dict) and "output" in data:
            return data["output"]
        
        return str(data)


class IPv6BgpSummaryFormatter(OutputFormatter):
    """
    Formats IPv6 BGP summary output with header and table.
    
    Example output:
        IPv6 Unicast Summary:
        BGP router identifier 10.1.0.32, local AS number 4200200000 vrf-id 0
        BGP table version 135211
        RIB entries 32209, using 4122752 bytes of memory
        Peers 88, using 1813504 KiB of memory
        Peer groups 6, using 384 bytes of memory
        
        
        Neighbhor      V          AS    MsgRcvd    MsgSent    TblVer    InQ    OutQ  Up/Down      State/PfxRcd  NeighborName
        -----------  ---  ----------  ---------  ---------  --------  -----  ------  ---------  --------------  --------------
        fc00::1a       4  4200300000       1852       1875    135211      0       0  1d00h33m                1  ARISTA07FT2
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format BGP summary with header and table."""
        if not data:
            return ""
        
        lines = []
        
        # Get the ipv6Unicast data
        unicast_data = data.get("ipv6Unicast", {})
        if not unicast_data:
            return ""
        
        # Header section
        lines.append("IPv6 Unicast Summary:")
        
        router_id = unicast_data.get("routerId", "")
        local_as = unicast_data.get("as", "")
        vrf_id = unicast_data.get("vrfId", 0)
        lines.append(f"BGP router identifier {router_id}, local AS number {local_as} vrf-id {vrf_id}")
        
        table_version = unicast_data.get("tableVersion", 0)
        lines.append(f"BGP table version {table_version}")
        
        rib_count = unicast_data.get("ribCount", 0)
        rib_memory = unicast_data.get("ribMemory", 0)
        lines.append(f"RIB entries {rib_count}, using {rib_memory} bytes of memory")
        
        peer_count = unicast_data.get("peerCount", 0)
        peer_memory = unicast_data.get("peerMemory", 0)
        lines.append(f"Peers {peer_count}, using {peer_memory} KiB of memory")
        
        pg_count = unicast_data.get("peerGroupCount", 0)
        pg_memory = unicast_data.get("peerGroupMemory", 0)
        lines.append(f"Peer groups {pg_count}, using {pg_memory} bytes of memory")
        
        lines.append("")
        lines.append("")
        
        # Peers table
        peers = unicast_data.get("peers", {})
        if peers:
            table_data = []
            # Preserve JSON insertion order (matches FRR output order)
            for peer_ip, peer in peers.items():
                # Filter out non-peer entries (e.g., "Total numbe" artifact)
                if "::" not in peer_ip:
                    continue
                row = [
                    peer_ip,
                    peer.get("version", 4),
                    peer.get("remoteAs", ""),
                    peer.get("msgRcvd", 0),
                    peer.get("msgSent", 0),
                    peer.get("tableVersion", 0),
                    peer.get("inq", 0),
                    peer.get("outq", 0),
                    peer.get("peerUptime", ""),
                    peer.get("pfxRcd", peer.get("state", "")),
                    peer.get("NeighborName", ""),
                ]
                table_data.append(row)
            
            headers = ["Neighbhor", "V", "AS", "MsgRcvd", "MsgSent", "TblVer", 
                       "InQ", "OutQ", "Up/Down", "State/PfxRcd", "NeighborName"]
            table_output = tabulate(table_data, headers=headers, tablefmt=table_format)
            lines.append(table_output)
        
        lines.append("")
        lines.append(f"Total number of neighbors {peer_count}")
        
        return "\n".join(lines)


class IPv6FibFormatter(OutputFormatter):
    """
    Formats IPv6 FIB output as a table.
    
    Example input:
        {"total": 16195, "entries": [{"index": 1, "route": "2064:100:0:10::", ...}]}
    
    Example output:
          No.  Vrf    Route                  Nexthop           Ifname
        -----  -----  ---------------------  ----------------  -------
            1         2064:100:0:10::        fc00::3e          Ethernet120
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format FIB as table."""
        if not data:
            return ""
        
        entries = data.get("entries", [])
        
        table_data = []
        for entry in entries:
            if isinstance(entry, dict):
                row = [
                    entry.get("index", ""),
                    entry.get("vrf", ""),  # Vrf column (usually empty)
                    entry.get("route", ""),
                    entry.get("nexthop", ""),
                    entry.get("ifname", ""),
                ]
                table_data.append(row)
        
        headers = ["No.", "Vrf", "Route", "Nexthop", "Ifname"]
        table = tabulate(table_data, headers=headers, tablefmt=table_format)
        
        # Add total count footer
        total = data.get("total", len(table_data))
        table += f"\nTotal number of entries {total}"
        
        return table


class IPv6InterfacesFormatter(OutputFormatter):
    """
    Formats IPv6 interfaces output as a table with multi-row support.
    
    Multiple IPv6 addresses per interface are shown on continuation lines.
    Link-local addresses (fe80::) include zone identifier (%interface_name).
    
    Example output:
        Interface       Master  IPv6 address/mask                            Admin/Oper    BGP Neighbor    Neighbor IP
        --------------  ------  -------------------------------------------  ------------  --------------  -------------
        Ethernet0               fc00::1/126                                  up/up         ARISTA01FT2     fc00::2
                                fe80::7a5f:6cff:fe30:d74c%Ethernet0/64                     N/A             N/A
    """
    
    def _format_ipv6_address(self, address: str, interface_name: str) -> str:
        """
        Format IPv6 address, adding zone identifier for link-local addresses.
        
        Link-local addresses (fe80::) get the interface name appended as a zone ID
        before the prefix length, e.g., fe80::1234%Ethernet0/64
        
        Args:
            address: IPv6 address with prefix (e.g., "fe80::1234/64")
            interface_name: Interface name to use as zone ID
            
        Returns:
            Formatted address with zone ID if link-local
        """
        if not address:
            return address
        
        # Check if it's a link-local address (starts with fe80::)
        addr_lower = address.lower()
        if addr_lower.startswith("fe80:"):
            # Split address and prefix length
            if "/" in address:
                addr_part, prefix = address.rsplit("/", 1)
                return f"{addr_part}%{interface_name}/{prefix}"
            else:
                return f"{address}%{interface_name}"
        
        return address
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format IPv6 interfaces as table with continuation lines."""
        if not data:
            return ""
        
        table_data = []
        
        # Sort interfaces
        for intf in sorted(data.keys(), key=natural_sort_key):
            intf_data = data[intf]
            if not isinstance(intf_data, dict):
                continue
            
            admin_status = intf_data.get("admin_status", "")
            oper_status = intf_data.get("oper_status", "")
            admin_oper = f"{admin_status}/{oper_status}"
            master = intf_data.get("master", "")
            
            ipv6_addresses = intf_data.get("ipv6_addresses", [])
            
            if ipv6_addresses:
                for i, addr_info in enumerate(ipv6_addresses):
                    if isinstance(addr_info, dict):
                        address = addr_info.get("address", "")
                        # Add zone identifier for link-local addresses
                        address = self._format_ipv6_address(address, intf)
                        neighbor_ip = addr_info.get("bgp_neighbor_ip", "N/A")
                        neighbor_name = addr_info.get("bgp_neighbor_name", "N/A")
                        
                        # First row shows interface name and admin/oper
                        if i == 0:
                            row = [intf, master, address, admin_oper, neighbor_name, neighbor_ip]
                        else:
                            # Continuation rows have empty interface/admin columns
                            row = ["", "", address, "", neighbor_name, neighbor_ip]
                        table_data.append(row)
            else:
                # Interface with no IPv6 addresses
                table_data.append([intf, master, "", admin_oper, "N/A", "N/A"])
        
        if not table_data:
            return ""
        
        headers = ["Interface", "Master", "IPv6 address/mask", "Admin/Oper", 
                   "BGP Neighbor", "Neighbor IP"]
        return tabulate(table_data, headers=headers, tablefmt=table_format)


class IPv6LinkLocalModeFormatter(OutputFormatter):
    """
    Formats IPv6 link-local mode output as a grid table.
    
    Expected output uses grid format with +---+ separators.
    
    Example output:
        +------------------+----------+
        | Interface Name   | Mode     |
        +==================+==========+
        | Ethernet0        | Disabled |
        +------------------+----------+
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format link-local mode as grid table."""
        if not data:
            return ""
        
        table_data = []
        
        if isinstance(data, list):
            for item in data:
                if isinstance(item, dict):
                    port = item.get("port", "")
                    mode = item.get("mode", "")
                    table_data.append([port, mode])
            # Keep original order - don't sort (the data comes in the expected order)
        else:
            for port in sorted(data.keys(), key=natural_sort_key):
                port_data = data[port]
                mode = port_data if isinstance(port_data, str) else port_data.get("mode", "")
                table_data.append([port, mode])
        
        if not table_data:
            return ""
        
        headers = ["Interface Name", "Mode"]
        # Use grid format with separators
        return tabulate(table_data, headers=headers, tablefmt="grid")


class IPv6PrefixListFormatter(OutputFormatter):
    """
    Formats IPv6 prefix-list output.
    
    Expected format:
        ZEBRA: ipv6 prefix-list DEFAULT_IPV6: 56 entries
           seq 5 permit ::/0
           seq 10 permit ::/0
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format prefix-list."""
        if not data:
            return ""
        
        # If it's pre-formatted output, just return it
        if isinstance(data, dict) and "output" in data:
            return data["output"]
        
        # Data structure: {"BGP": {...}, "ZEBRA": {...}}
        # Each protocol has prefix lists like: {"DEFAULT_IPV6": {"addressFamily": "IPv6", "entries": [...]}}
        if isinstance(data, dict):
            lines = []
            # ZEBRA first, then other protocols in order
            protocols = sorted(data.keys(), key=lambda x: (0 if x == "ZEBRA" else 1, x))
            for protocol in protocols:
                protocol_data = data[protocol]
                if not isinstance(protocol_data, dict):
                    continue
                for pl_name in sorted(protocol_data.keys()):
                    pl_data = protocol_data[pl_name]
                    if not isinstance(pl_data, dict):
                        continue
                    entries = pl_data.get("entries", [])
                    # Header line
                    lines.append(f"{protocol}: ipv6 prefix-list {pl_name}: {len(entries)} entries")
                    # Entry lines
                    for entry in entries:
                        seq = entry.get("sequenceNumber", "")
                        action = entry.get("type", "")
                        prefix = entry.get("prefix", "")
                        ge = entry.get("minimumPrefixLength")
                        le = entry.get("maximumPrefixLength")
                        
                        # Build entry line
                        entry_line = f"   seq {seq} {action} {prefix}"
                        if ge is not None:
                            entry_line += f" ge {ge}"
                        if le is not None:
                            entry_line += f" le {le}"
                        lines.append(entry_line)
            return "\n".join(lines)
        
        return str(data)


class IPv6ProtocolFormatter(OutputFormatter):
    """
    Formats IPv6 protocol output.
    
    Expected format:
        VRF: default
        Protocol                  : route-map
        -------------------------------------
        system                    : none
        kernel                    : none
    """
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format protocol info."""
        if not data:
            return ""
        
        # If it's pre-formatted output, just return it
        if isinstance(data, dict) and "output" in data:
            return data["output"]
        
        # Data structure: [{"VRF": "default", "Protocols": [...]}, ...]
        if isinstance(data, list):
            lines = []
            for vrf_data in data:
                if not isinstance(vrf_data, dict):
                    continue
                vrf_name = vrf_data.get("VRF", "default")
                lines.append(f"VRF: {vrf_name}")
                lines.append("Protocol                  : route-map")
                lines.append("-" * 37)  # 37 dashes to match
                
                protocols = vrf_data.get("Protocols", [])
                for proto in protocols:
                    if isinstance(proto, dict):
                        name = proto.get("Protocol", "")
                        route_map = proto.get("route-map", "none")
                        # Left-align protocol name in 26 chars
                        lines.append(f"{name:<26}: {route_map}")
            return "\n".join(lines)
        
        return str(data)


class IPv6RouteFormatter(OutputFormatter):
    """
    Formats IPv6 route output.
    
    Two output formats are supported:
    1. Single-line format (for protocol filters like connected, static, bgp):
        :
        Codes: K - kernel route, C - connected, L - local, S - static, ...
        IPv6 unicast VRF default:
        C>* fc00::/126 is directly connected, PortChannel102 linkdown, weight 1, 01w1d03h
        
    2. Detailed multi-line format (for specific prefix queries like fe80::):
        Routing entry for fe80::/64
          Known via "connected", distance 0, metric 0
          Last update 04:43:27 ago
          * directly connected, Ethernet496
    """
    
    # Protocol code mapping
    PROTOCOL_CODES = {
        "connected": "C",
        "local": "L",
        "static": "S",
        "kernel": "K",
        "bgp": "B",
        "ospf": "O",
        "ospf6": "O",
        "rip": "R",
        "ripng": "R",
        "isis": "I",
        "nhrp": "N",
        "table": "T",
        "vnc": "v",
        "vnc-direct": "V",
        "babel": "A",
        "pbr": "F",
        "openfabric": "f",
        "table-direct": "t",
    }
    
    # Protocol filter arguments that use single-line format
    PROTOCOL_FILTERS = {"connected", "local", "static", "kernel", "bgp", "ospf", 
                        "ospf6", "rip", "ripng", "isis", "nhrp", "table"}
    
    def format(
        self,
        data: Union[Dict, List],
        config: CommandConfig,
        table_format: str = "simple",
        argument: str = "",
        options: Optional[Dict] = None,
    ) -> str:
        """Format IPv6 route entries."""
        if not data:
            return ""
        
        # If it's pre-formatted output, just return it
        if isinstance(data, dict) and "output" in data:
            return data["output"]
        
        # Handle dict of prefix -> routes
        if not isinstance(data, dict):
            return str(data)
        
        # Determine output format based on argument
        
        # Use detailed format if argument looks like an IPv6 address/prefix
        # (contains :: or is a pure hex:colon pattern with optional /prefix)
        use_detailed = self._is_ipv6_prefix_arg(argument)
        
        if use_detailed:
            return self._format_detailed(data)
        else:
            return self._format_single_line(data)
    
    def _is_ipv6_prefix_arg(self, argument: str) -> bool:
        """Check if argument is an IPv6 address/prefix pattern."""
        if not argument:
            return False
        # vrf <name> is not a prefix
        if argument.lower().startswith('vrf'):
            return False
        # Protocol filters are not prefixes
        if argument.lower() in self.PROTOCOL_FILTERS:
            return False
        # IPv6 addresses contain :: or have colons with hex digits
        if '::' in argument:
            return True
        # Check for pattern like fe80: or fc00:
        import re
        if re.match(r'^[0-9a-fA-F:]+(/\d+)?$', argument):
            return True
        return False
    
    def _format_single_line(self, data: Dict) -> str:
        """Format routes in single-line FRR-style format."""
        lines = []
        
        # Add header
        lines.append(":")
        lines.append("Codes: K - kernel route, C - connected, L - local, S - static,")
        lines.append("       R - RIPng, O - OSPFv3, I - IS-IS, B - BGP, N - NHRP,")
        lines.append("       T - Table, v - VNC, V - VNC-Direct, A - Babel, F - PBR,")
        lines.append("       f - OpenFabric, t - Table-Direct,")
        lines.append("       > - selected route, * - FIB route, q - queued, r - rejected, b - backup")
        lines.append("       t - trapped, o - offload failure")
        lines.append("")
        
        # Get VRF name from first route entry - only output VRF line if vrfName is explicitly present
        has_vrf_name = False
        vrf_name = "default"
        for prefix, routes in data.items():
            if isinstance(routes, list) and routes:
                if "vrfName" in routes[0]:
                    has_vrf_name = True
                    vrf_name = routes[0].get("vrfName", "default")
                break
            elif isinstance(routes, dict):
                if "vrfName" in routes:
                    has_vrf_name = True
                    vrf_name = routes.get("vrfName", "default")
                break
        
        # Only output VRF line if vrfName was explicitly in the data and first nexthop has weight
        # (FRR versions with weight field show VRF line; those without don't)
        first_has_weight = False
        for prefix, routes in data.items():
            if isinstance(routes, list) and routes:
                nexthops = routes[0].get("nexthops", [])
                if nexthops and "weight" in nexthops[0]:
                    first_has_weight = True
                break
            elif isinstance(routes, dict):
                nexthops = routes.get("nexthops", [])
                if nexthops and "weight" in nexthops[0]:
                    first_has_weight = True
                break
        
        if has_vrf_name and first_has_weight:
            lines.append(f"IPv6 unicast VRF {vrf_name}:")
        
        # Sort prefixes for consistent output
        for prefix in sorted(data.keys(), key=self._prefix_sort_key):
            routes = data[prefix]
            
            # routes can be a list (multiple paths to same prefix) or a single dict
            if isinstance(routes, dict):
                routes = [routes]
            elif not isinstance(routes, list):
                continue
            
            # Each route entry in the list represents a different path
            for route in routes:
                if not isinstance(route, dict):
                    continue
                
                route_line = self._format_route_line(prefix, route)
                if route_line:
                    lines.append(route_line)
        
        return "\n".join(lines)
    
    def _format_detailed(self, data: Dict) -> str:
        """Format routes in detailed multi-line format."""
        lines = []
        
        # Sort prefixes for consistent output
        for prefix in sorted(data.keys(), key=self._prefix_sort_key):
            routes = data[prefix]
            
            # routes can be a list (multiple paths to same prefix) or a single dict
            if isinstance(routes, dict):
                routes = [routes]
            elif not isinstance(routes, list):
                continue
            
            # Each route entry in the list represents a different path
            for route in routes:
                if not isinstance(route, dict):
                    continue
                
                lines.extend(self._format_route_entry(prefix, route))
                lines.append("")  # Double blank line between entries (CLI format)
                lines.append("")
        
        return "\n".join(lines).rstrip()
    
    def _prefix_sort_key(self, prefix: str) -> tuple:
        """Generate sort key for IPv6 prefixes."""
        import ipaddress
        try:
            net = ipaddress.ip_network(prefix, strict=False)
            return (0, net.network_address.packed, net.prefixlen)
        except ValueError:
            return (1, prefix.encode(), 0)
    
    def _format_route_line(self, prefix: str, route: Dict) -> str:
        """Format a single route entry as a single line."""
        protocol = route.get("protocol", "unknown")
        protocol_code = self.PROTOCOL_CODES.get(protocol.lower(), "?")
        
        # Check if route is selected/best route
        is_selected = route.get("selected", False) or route.get("destSelected", False)
        is_fib = route.get("installed", False)
        
        # Build status flags: e.g., "C>*" or "C *"
        selected_flag = ">" if is_selected else " "
        fib_flag = "*" if is_fib else " "
        status = f"{protocol_code}{selected_flag}{fib_flag}"
        
        # Get nexthop info
        nexthops = route.get("nexthops", [])
        if nexthops and isinstance(nexthops, list) and nexthops[0]:
            nh = nexthops[0]
            is_directly_connected = nh.get("directlyConnected", False)
            interface = nh.get("interfaceName", "")
            nexthop_ip = nh.get("ip", nh.get("nexthop", ""))
            link_down = nh.get("linkDown", False)
            weight = nh.get("weight")  # None if not present
        else:
            is_directly_connected = False
            interface = ""
            nexthop_ip = ""
            link_down = False
            weight = None
        
        uptime = route.get("uptime", "")
        distance = route.get("distance", 0)
        metric = route.get("metric", 0)
        
        # Build the route description
        # Only include weight if it was present in the data
        weight_str = f", weight {weight}" if weight is not None else ""
        
        if is_directly_connected:
            # Format depends on protocol:
            # - "connected": C>* fc00::/126 is directly connected, PortChannel102, weight 1, 01w1d03h
            # - "static":    S>* fc00:1::/64 [1/0] is directly connected, Loopback0, weight 1, 01w1d03h
            link_status = " linkdown" if link_down else ""
            if protocol.lower() == "connected":
                route_desc = f"is directly connected, {interface}{link_status}{weight_str}, {uptime}"
            else:
                route_desc = f"[{distance}/{metric}] is directly connected, {interface}{link_status}{weight_str}, {uptime}"
        elif nexthop_ip:
            # Format: B>* 2064:100::/48 [200/0] via fc00::2, PortChannel102, weight 1, 01w1d03h
            if interface:
                route_desc = f"[{distance}/{metric}] via {nexthop_ip}, {interface}{weight_str}, {uptime}"
            else:
                route_desc = f"[{distance}/{metric}] via {nexthop_ip}{weight_str}, {uptime}"
        else:
            # Fallback
            route_desc = f"[{distance}/{metric}]{weight_str}, {uptime}"
        
        return f"{status} {prefix} {route_desc}"
    
    def _format_route_entry(self, prefix: str, route: Dict) -> List[str]:
        """Format a single route entry in detailed multi-line format."""
        lines = []
        
        # Routing entry for prefix
        lines.append(f"Routing entry for {prefix}")
        
        # Known via "protocol", distance X, metric Y[, best]
        protocol = route.get("protocol", "unknown")
        distance = route.get("distance", 0)
        metric = route.get("metric", 0)
        is_best = route.get("selected", False) or route.get("destSelected", False)
        best_suffix = ", best" if is_best else ""
        lines.append(f'  Known via "{protocol}", distance {distance}, metric {metric}{best_suffix}')
        
        # Last update
        uptime = route.get("uptime", "")
        if uptime:
            lines.append(f"  Last update {uptime} ago")
        
        # Nexthops
        nexthops = route.get("nexthops", [])
        if isinstance(nexthops, list):
            for nh in nexthops:
                if not isinstance(nh, dict):
                    continue
                
                nh_line = self._format_nexthop(nh)
                if nh_line:
                    lines.append(nh_line)
        
        return lines
    
    def _format_nexthop(self, nh: Dict) -> str:
        """Format a single nexthop entry for detailed format."""
        # Active indicator
        if nh.get("active", False) or nh.get("fib", False):
            active = "*"
        else:
            active = " "
        
        # Check if directly connected
        if nh.get("directlyConnected", False):
            interface = nh.get("interfaceName", "")
            return f"  {active} directly connected, {interface}"
        
        # Regular nexthop with IP address
        nexthop_ip = nh.get("ip", nh.get("nexthop", ""))
        interface = nh.get("interfaceName", "")
        
        if nexthop_ip:
            if interface:
                return f"  {active} via {nexthop_ip}, {interface}"
            else:
                return f"  {active} via {nexthop_ip}"
        elif interface:
            return f"  {active} directly connected, {interface}"
        
        return ""

