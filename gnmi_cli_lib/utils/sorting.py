"""
Sorting utilities for natural ordering of interface and port names.

Natural sorting ensures that:
- Ethernet0, Ethernet4, Ethernet10 are sorted correctly (not Ethernet0, Ethernet10, Ethernet4)
- PG0, PG1, PG2, ... PG10 are sorted correctly
- UC0, UC1, UC2, ... UC10 are sorted correctly
"""

import re
from typing import List, Any, Callable


def natural_sort_key(s: str) -> List:
    """
    Generate a sort key for natural ordering of strings with numbers.
    
    This function splits a string into text and numeric components,
    allowing for natural sorting where "Ethernet10" comes after "Ethernet4".
    
    Args:
        s: String to generate sort key for
    
    Returns:
        List of alternating lowercase strings and integers
    
    Example:
        >>> natural_sort_key("Ethernet10")
        ['ethernet', 10, '']
        >>> natural_sort_key("PG5")
        ['pg', 5, '']
        >>> sorted(["Ethernet10", "Ethernet4", "Ethernet0"], key=natural_sort_key)
        ['Ethernet0', 'Ethernet4', 'Ethernet10']
    """
    return [int(c) if c.isdigit() else c.lower() for c in re.split(r'(\d+)', s)]


def natural_sorted(items: List[str], key: Callable[[Any], str] = None) -> List[str]:
    """
    Sort a list of strings using natural ordering.
    
    Args:
        items: List of strings to sort
        key: Optional key function to extract string from each item
    
    Returns:
        Sorted list
    
    Example:
        >>> natural_sorted(["Ethernet10", "Ethernet4", "Ethernet0"])
        ['Ethernet0', 'Ethernet4', 'Ethernet10']
        >>> natural_sorted([("a", "Ethernet10"), ("b", "Ethernet4")], key=lambda x: x[1])
        [('b', 'Ethernet4'), ('a', 'Ethernet10')]
    """
    if key is None:
        return sorted(items, key=natural_sort_key)
    else:
        return sorted(items, key=lambda x: natural_sort_key(key(x)))
