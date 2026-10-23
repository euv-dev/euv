use super::*;

/// Splits a whitespace-separated class list into its names.
///
/// # Arguments
///
/// - `&str` - The class list.
///
/// # Returns
///
/// - `Vec<String>` - The class names, without the empty entries.
pub(crate) fn split_class_names(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|name: &str| name.to_string())
        .collect()
}
