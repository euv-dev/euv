use super::*;

/// Returns the document language associated with a configured locale label.
///
/// # Arguments
///
/// - `&str` - The locale display label from site configuration.
///
/// # Returns
///
/// - `&'static str` - The matching BCP-47 language tag, or the English fallback.
pub(crate) fn document_language_for_label(label: &str) -> &'static str {
    let binding: Option<&(&str, &str)> = DOCUMENT_LANGUAGE_BINDINGS
        .iter()
        .find(|(name, _): &&(&str, &str)| *name == label);
    binding.map_or(DOCUMENT_LANGUAGE_FALLBACK, |(_, tag): &(&str, &str)| *tag)
}
