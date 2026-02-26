"""
Parsing utilities for extracting and transforming data.

This module provides helper functions for:
- Parsing JSON data (string or dict)
- Extracting interface names from various key formats
- Parsing reason strings from drop counters
"""

import json
from typing import Dict, List, Union


def parse_reasons_string(reasons_str: str) -> List[str]:
    """
    Parse a reasons string like '[REASON1,REASON2]' into a list.
    
    This is commonly used for dropcounters capabilities where reasons
    are stored as a string representation of a list.
    
    Args:
        reasons_str: String in format "[REASON1,REASON2,...]"
    
    Returns:
        List of reason strings, stripped of whitespace
    
    Example:
        >>> parse_reasons_string("[IP_HEADER_ERROR,NO_L3_HEADER]")
        ['IP_HEADER_ERROR', 'NO_L3_HEADER']
        >>> parse_reasons_string("[]")
        []
        >>> parse_reasons_string("")
        []
    """
    # Remove brackets and split by comma
    cleaned = reasons_str.strip()
    if cleaned.startswith("[") and cleaned.endswith("]"):
        cleaned = cleaned[1:-1]
    
    if not cleaned:
        return []
    
    return [r.strip() for r in cleaned.split(",") if r.strip()]


def extract_interface_name(key: str) -> str:
    """
    Extract interface name from various key formats.
    
    Handles formats like:
    - "PORT_TABLE:Ethernet0" -> "Ethernet0"
    - "PORTCHANNEL|PortChannel101" -> "PortChannel101"
    - "PORT|Ethernet0" -> "Ethernet0"
    - "Ethernet0" -> "Ethernet0"
    
    Args:
        key: Key string that may contain prefix
    
    Returns:
        Interface name without prefix
    
    Example:
        >>> extract_interface_name("PORT_TABLE:Ethernet0")
        'Ethernet0'
        >>> extract_interface_name("PORTCHANNEL|PortChannel101")
        'PortChannel101'
        >>> extract_interface_name("Ethernet0")
        'Ethernet0'
    """
    if ":" in key:
        return key.split(":")[-1]
    elif "|" in key:
        return key.split("|")[-1]
    return key


def parse_json_data(json_data: Union[Dict, List, str]) -> Union[Dict, List]:
    """
    Parse JSON data from string or return dict/list as-is.
    
    Args:
        json_data: JSON data as dict, list, or string
    
    Returns:
        Parsed dictionary or list
    
    Raises:
        json.JSONDecodeError: If string is not valid JSON
        TypeError: If input is neither dict, list, nor string
    
    Example:
        >>> parse_json_data({"key": "value"})
        {'key': 'value'}
        >>> parse_json_data('{"key": "value"}')
        {'key': 'value'}
        >>> parse_json_data([{"key": "value"}])
        [{'key': 'value'}]
    """
    if isinstance(json_data, (dict, list)):
        return json_data
    elif isinstance(json_data, str):
        return json.loads(json_data)
    else:
        raise TypeError(f"Expected dict, list, or str, got {type(json_data).__name__}")


def array_to_dict(data: Union[Dict, List], key_field: str = "Interface") -> Dict:
    """
    Convert array data to dict format using a specified key field.
    
    Many gNMI responses return data as arrays, but formatters expect dict format.
    This function converts: [{key_field: "Eth0", ...}, ...] -> {"Eth0": {...}, ...}
    
    Args:
        data: Data as list of dicts or already a dict
        key_field: The field name to use as the dict key (default: "Interface")
    
    Returns:
        Dict with key_field values as keys
    
    Example:
        >>> array_to_dict([{"Interface": "Eth0", "Speed": "100G"}])
        {'Eth0': {'Interface': 'Eth0', 'Speed': '100G'}}
        >>> array_to_dict({"Eth0": {"Speed": "100G"}})
        {'Eth0': {'Speed': '100G'}}
    """
    if isinstance(data, dict):
        return data
    
    if not isinstance(data, list):
        return {}
    
    result = {}
    for item in data:
        if isinstance(item, dict):
            key = item.get(key_field, "")
            if key:
                result[key] = item
    return result


# Field name normalization mappings for gNMI to internal format
INTERFACE_STATUS_FIELD_MAP = {
    "Admin": "admin_status",
    "Oper": "oper_status", 
    "Speed": "speed",
    "MTU": "mtu",
    "Alias": "alias",
    "Lanes": "lanes",
    "FEC": "fec",
    "Type": "type",
    "Vlan": "vlan",
    "Asym": "asym_pfc",
    "Interface": "interface",
    "Description": "description",
}

INTERFACE_FLAP_FIELD_MAP = {
    "Interface": "interface",
    "Flap Count": "flap_count",
    "Link Up TimeStamp(UTC)": "last_up_time",
    "Link Down TimeStamp(UTC)": "last_down_time",
}

INTERFACE_ALIAS_FIELD_MAP = {
    "Interface": "interface",
    "Alias": "alias",
}


def normalize_field_names(data: Dict, field_map: Dict[str, str]) -> Dict:
    """
    Normalize field names in a dict using a mapping.
    
    Args:
        data: Dict with original field names
        field_map: Mapping from original -> normalized names
    
    Returns:
        Dict with normalized field names (preserves original names too)
    
    Example:
        >>> normalize_field_names({"Admin": "up"}, {"Admin": "admin_status"})
        {'Admin': 'up', 'admin_status': 'up'}
    """
    if not isinstance(data, dict):
        return data
    
    result = dict(data)  # Keep original fields
    for orig_name, new_name in field_map.items():
        if orig_name in data:
            result[new_name] = data[orig_name]
    return result


def transform_interface_data(data: Union[Dict, List], field_map: Dict[str, str] = None) -> Dict:
    """
    Transform interface data from gNMI format to formatter-expected format.
    
    Handles both array and dict input formats, normalizes field names,
    and outputs a dict with interface names as keys.
    
    Args:
        data: gNMI data (array or dict)
        field_map: Optional field name mapping (defaults to INTERFACE_STATUS_FIELD_MAP)
    
    Returns:
        Dict with interface names as keys and normalized field values
    
    Example:
        >>> transform_interface_data([{"Interface": "Eth0", "Admin": "up"}])
        {'Eth0': {'Interface': 'Eth0', 'Admin': 'up', 'admin_status': 'up', ...}}
    """
    if field_map is None:
        field_map = INTERFACE_STATUS_FIELD_MAP
    
    # Convert array to dict if needed
    if isinstance(data, list):
        dict_data = array_to_dict(data, "Interface")
    else:
        dict_data = data
    
    # Normalize field names for each interface
    result = {}
    for key, value in dict_data.items():
        if isinstance(value, dict):
            result[key] = normalize_field_names(value, field_map)
        else:
            result[key] = value
    
    return result
