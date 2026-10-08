use super::*;

/// A type alias for a list of HTML attribute key-value pairs.
///
/// Used throughout the `html!` macro parsing to represent parsed attributes.
/// The key is a `proc_macro2::TokenStream` that evaluates to an attribute name string.
pub(crate) type HtmlAttrs = Vec<(proc_macro2::TokenStream, HtmlAttrValue)>;

/// Signature of the merge callback used by `merge_same_key_attributes`:
/// pushes the merged `class` / `style` entry into the result list,
/// wrapping multiple values with the given `HtmlAttrValue` constructor.
pub(crate) type PushMergedAttrFn =
    fn(&mut HtmlAttrs, &str, Vec<HtmlAttrValue>, fn(Vec<HtmlAttrValue>) -> HtmlAttrValue);
