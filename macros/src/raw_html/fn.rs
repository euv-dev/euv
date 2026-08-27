use super::*;

/// Parses the input of the `unsafe_no_inline!` macro.
///
/// Accepts a single string literal and emits
/// `::euv_core::RawHtml::new(value.to_string())`. The
/// `unsafe_no_` prefix is a deliberately loud warning
/// that the string is NOT escaped; treat it like
/// `Element.innerHTML` in JavaScript.
///
/// The macro enforces that the input is a string
/// literal so the `unsafe_no_` prefix carries its
/// security warning forward to the call site.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream containing
///   exactly one string literal.
///
/// # Returns
///
/// - `TokenStream` - The expanded `RawHtml::new(value)`
///   expression.
pub(crate) fn parse_unsafe_no_inline(input: TokenStream) -> TokenStream {
    let literal: LitStr = parse_macro_input!(input as LitStr);
    let value: String = literal.value();
    let expanded: TokenStream = quote! {
        ::euv_core::RawHtml::new(#value.to_string())
    }
    .into();
    expanded
}
