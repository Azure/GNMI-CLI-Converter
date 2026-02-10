//! Sorting utilities for natural ordering of interface and port names.
//!
//! Natural sorting ensures that:
//! - Ethernet0, Ethernet4, Ethernet10 are sorted correctly (not Ethernet0, Ethernet10, Ethernet4)
//! - PG0, PG1, PG2, ... PG10 are sorted correctly
//! - UC0, UC1, UC2, ... UC10 are sorted correctly

use std::cmp::Ordering;

/// A component of a string for natural sorting - either text or a number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortComponent {
    /// A textual component for natural sort comparison.
    Text(String),
    /// A numeric component for natural sort comparison.
    Number(u64),
}

impl PartialOrd for SortComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (SortComponent::Text(a), SortComponent::Text(b)) => {
                a.to_lowercase().cmp(&b.to_lowercase())
            }
            (SortComponent::Number(a), SortComponent::Number(b)) => a.cmp(b),
            (SortComponent::Text(_), SortComponent::Number(_)) => Ordering::Greater,
            (SortComponent::Number(_), SortComponent::Text(_)) => Ordering::Less,
        }
    }
}

/// Generate a sort key for natural ordering of strings with numbers.
///
/// This function splits a string into text and numeric components,
/// allowing for natural sorting where "Ethernet10" comes after "Ethernet4".
///
/// # Arguments
///
/// * `s` - String to generate sort key for
///
/// # Returns
///
/// Vec of alternating text strings and integers (as sort components)
///
/// # Examples
///
/// ```
/// use gnmi_cli_lib::utils::sorting::natural_sort_key;
///
/// let key1 = natural_sort_key("Ethernet10");
/// let key2 = natural_sort_key("Ethernet4");
/// assert!(key1 > key2); // 10 > 4 in natural sorting
/// ```
pub fn natural_sort_key(s: &str) -> Vec<SortComponent> {
    let mut components = Vec::new();
    let mut current_text = String::new();
    let mut current_num = String::new();
    let mut in_number = false;

    for c in s.chars() {
        if c.is_ascii_digit() {
            if !in_number && !current_text.is_empty() {
                components.push(SortComponent::Text(current_text.clone()));
                current_text.clear();
            }
            in_number = true;
            current_num.push(c);
        } else {
            if in_number && !current_num.is_empty() {
                if let Ok(n) = current_num.parse::<u64>() {
                    components.push(SortComponent::Number(n));
                }
                current_num.clear();
            }
            in_number = false;
            current_text.push(c);
        }
    }

    // Push remaining content
    if !current_text.is_empty() {
        components.push(SortComponent::Text(current_text));
    }
    if !current_num.is_empty() {
        if let Ok(n) = current_num.parse::<u64>() {
            components.push(SortComponent::Number(n));
        }
    }

    components
}

/// Sort a list of strings using natural ordering.
///
/// # Arguments
///
/// * `items` - Slice of strings to sort
///
/// # Returns
///
/// Sorted vector of strings
///
/// # Examples
///
/// ```
/// use gnmi_cli_lib::utils::sorting::natural_sorted;
///
/// let items = vec!["Ethernet10", "Ethernet4", "Ethernet0"];
/// let sorted = natural_sorted(&items);
/// assert_eq!(sorted, vec!["Ethernet0", "Ethernet4", "Ethernet10"]);
/// ```
pub fn natural_sorted<T: AsRef<str> + Clone>(items: &[T]) -> Vec<T> {
    let mut sorted: Vec<T> = items.to_vec();
    sorted.sort_by_key(|a| natural_sort_key(a.as_ref()));
    sorted
}

/// Sort a list of items using natural ordering with a key function.
///
/// # Arguments
///
/// * `items` - Slice of items to sort
/// * `key_fn` - Function to extract the sort key string from each item
///
/// # Returns
///
/// Sorted vector of items
pub fn natural_sorted_by_key<T: Clone, F>(items: &[T], key_fn: F) -> Vec<T>
where
    F: Fn(&T) -> &str,
{
    let mut sorted: Vec<T> = items.to_vec();
    sorted.sort_by_key(|a| natural_sort_key(key_fn(a)));
    sorted
}

/// Compare two strings using natural ordering.
///
/// # Arguments
///
/// * `a` - First string
/// * `b` - Second string
///
/// # Returns
///
/// Ordering result
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    natural_sort_key(a).cmp(&natural_sort_key(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_sort_key_simple() {
        let key = natural_sort_key("Ethernet10");
        assert_eq!(key.len(), 2);
    }

    #[test]
    fn test_natural_sort_key_text_only() {
        let key = natural_sort_key("abc");
        assert_eq!(key.len(), 1);
        assert_eq!(key[0], SortComponent::Text("abc".to_string()));
    }

    #[test]
    fn test_natural_sort_key_number_only() {
        let key = natural_sort_key("123");
        assert_eq!(key.len(), 1);
        assert_eq!(key[0], SortComponent::Number(123));
    }

    #[test]
    fn test_natural_sort_key_mixed() {
        let key = natural_sort_key("Eth10Port5");
        assert_eq!(key.len(), 4);
    }

    #[test]
    fn test_natural_sorted_ethernet() {
        let items = vec!["Ethernet10", "Ethernet4", "Ethernet0", "Ethernet100"];
        let sorted = natural_sorted(&items);
        assert_eq!(
            sorted,
            vec!["Ethernet0", "Ethernet4", "Ethernet10", "Ethernet100"]
        );
    }

    #[test]
    fn test_natural_sorted_pg() {
        let items = vec!["PG10", "PG2", "PG1", "PG0"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["PG0", "PG1", "PG2", "PG10"]);
    }

    #[test]
    fn test_natural_sorted_uc() {
        let items = vec!["UC10", "UC2", "UC1", "UC0"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["UC0", "UC1", "UC2", "UC10"]);
    }

    #[test]
    fn test_natural_sorted_mixed_types() {
        let items = vec!["Ethernet10", "PortChannel1", "Ethernet0", "PortChannel10"];
        let sorted = natural_sorted(&items);
        assert_eq!(
            sorted,
            vec!["Ethernet0", "Ethernet10", "PortChannel1", "PortChannel10"]
        );
    }

    #[test]
    fn test_natural_sorted_empty() {
        let items: Vec<&str> = vec![];
        let sorted = natural_sorted(&items);
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_natural_sorted_single() {
        let items = vec!["Ethernet0"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["Ethernet0"]);
    }

    #[test]
    fn test_natural_sorted_case_insensitive() {
        let items = vec!["ethernet10", "Ethernet4", "ETHERNET0"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["ETHERNET0", "Ethernet4", "ethernet10"]);
    }

    #[test]
    fn test_natural_cmp() {
        assert_eq!(natural_cmp("Ethernet0", "Ethernet10"), Ordering::Less);
        assert_eq!(natural_cmp("Ethernet10", "Ethernet4"), Ordering::Greater);
        assert_eq!(natural_cmp("Ethernet5", "Ethernet5"), Ordering::Equal);
    }

    #[test]
    fn test_natural_sorted_by_key() {
        let items = vec![("a", "Ethernet10"), ("b", "Ethernet4"), ("c", "Ethernet0")];
        let sorted = natural_sorted_by_key(&items, |x| x.1);
        assert_eq!(
            sorted,
            vec![("c", "Ethernet0"), ("b", "Ethernet4"), ("a", "Ethernet10"),]
        );
    }

    #[test]
    fn test_natural_sort_with_leading_zeros() {
        // When numeric values are equal (01 = 1 = 1), the sort is stable
        // The current implementation treats 01 and 1 as the same number
        // So the order depends on which comes first in input for equal values
        let items = vec!["Eth01", "Eth1", "Eth10", "Eth02"];
        let sorted = natural_sorted(&items);
        // 01 and 1 both parse to 1, 02 parses to 2, 10 parses to 10
        // Order: items with value 1, then 2, then 10
        // Within equal values, order is stable from input
        assert_eq!(sorted, vec!["Eth01", "Eth1", "Eth02", "Eth10"]);
    }

    #[test]
    fn test_natural_sort_all_numbers() {
        let items = vec!["100", "10", "1", "20"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["1", "10", "20", "100"]);
    }

    #[test]
    fn test_natural_sort_special_chars() {
        let items = vec!["port-10", "port-2", "port-1"];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted, vec!["port-1", "port-2", "port-10"]);
    }

    #[test]
    fn test_natural_sort_string_ownership() {
        let items = vec![
            "Ethernet10".to_string(),
            "Ethernet4".to_string(),
            "Ethernet0".to_string(),
        ];
        let sorted = natural_sorted(&items);
        assert_eq!(sorted[0], "Ethernet0");
        assert_eq!(sorted[1], "Ethernet4");
        assert_eq!(sorted[2], "Ethernet10");
    }

    // =======================================================================
    // Phase 3.6 — Additional edge case tests for sorting (GAP-B6)
    // =======================================================================

    #[test]
    fn test_natural_sort_key_empty_string() {
        let key = natural_sort_key("");
        assert!(key.is_empty());
    }

    #[test]
    fn test_natural_sort_key_whitespace_only() {
        let key = natural_sort_key("   ");
        assert_eq!(key.len(), 1);
        assert_eq!(key[0], SortComponent::Text("   ".to_string()));
    }

    #[test]
    fn test_natural_cmp_equal_strings() {
        assert_eq!(natural_cmp("abc", "abc"), Ordering::Equal);
    }

    #[test]
    fn test_sort_component_number_vs_text() {
        // Number < Text in the ordering
        let num = SortComponent::Number(1);
        let text = SortComponent::Text("a".to_string());
        assert!(num < text);
    }

    // ---- Round 5 coverage tests ----

    #[test]
    fn test_sort_component_text_vs_text() {
        // Cover L28: Text(a) vs Text(b) comparison
        let a = SortComponent::Text("apple".to_string());
        let b = SortComponent::Text("banana".to_string());
        assert!(a < b);
        assert_eq!(
            SortComponent::Text("abc".to_string()).cmp(&SortComponent::Text("abc".to_string())),
            Ordering::Equal
        );
    }

    #[test]
    fn test_sort_component_text_vs_number() {
        // Cover L32: Text(_) vs Number(_) => Greater
        let text = SortComponent::Text("a".to_string());
        let num = SortComponent::Number(1);
        assert!(text > num);
    }
}
