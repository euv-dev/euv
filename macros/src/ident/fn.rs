use super::*;

/// Checks whether the current position in the parse stream starts an attribute
/// key pattern (e.g., `type:`, `async-onclick:`, `r#box-shadow:`).
///
/// Unlike `peek(Ident)` which only matches regular identifiers, this function
/// also accepts Rust reserved keywords (such as `type`, `async`, `for`, `loop`)
/// as attribute key prefixes. It does this by forking the stream and attempting
/// to parse a `proc_macro2::TokenTree::Ident`, which includes both identifiers
/// and keywords.
///
/// Returns `true` when the current token is an `Ident` (identifier or keyword)
/// followed by `:` or `-`, and the `:` is not part of a `::` path separator.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream to check.
///
/// # Returns
///
/// - `bool` - `true` if the current position matches an attribute key pattern.
pub(crate) fn is_attr_key_pattern(content: ParseStream) -> bool {
    let forked: ParseBuffer<'_> = content.fork();
    let Ok(token_tree) = forked.parse::<TokenTree>() else {
        return false;
    };
    if !matches!(token_tree, TokenTree::Ident(_)) {
        return false;
    }
    if forked.peek(Colon) && !forked.peek(Token![::]) {
        return true;
    }
    forked.peek(Token![-])
}

/// Parses a single identifier or keyword segment from the token stream,
/// stripping the `r#` prefix if present.
///
/// Rust reserved keywords (e.g., `box`, `break`, `type`) cannot be parsed
/// as `Ident` directly. This function uses `proc_macro2::TokenTree` to
/// accept both normal identifiers and keywords, returning the clean name
/// without the `r#` prefix.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<String>` - The clean identifier name (without `r#`).
pub(crate) fn parse_ident_segment(input: ParseStream) -> syn::Result<String> {
    let token_tree: proc_macro2::TokenTree = input.parse()?;
    match token_tree {
        proc_macro2::TokenTree::Ident(ident) => {
            let raw_name: String = ident.to_string();
            Ok(raw_name
                .strip_prefix(RAW_IDENT_PREFIX)
                .unwrap_or(&raw_name)
                .to_string())
        }
        proc_macro2::TokenTree::Literal(literal) => {
            let literal_string: String = literal.to_string();
            if literal_string.starts_with(CHAR_DOUBLE_QUOTE) {
                Err(input.error(ERR_EXPECTED_IDENTIFIER))
            } else {
                Ok(literal_string)
            }
        }
        _ => Err(input.error(ERR_EXPECTED_IDENTIFIER)),
    }
}

/// Parses an identifier chain name from the token stream.
///
/// Supports two input forms:
/// - String literal: `"bg-primary"` → `"bg-primary"`
/// - Unquoted identifier chain: `bg-primary` → `"bg-primary"`
/// - Raw identifiers for Rust keywords: `r#box-shadow` → `"box-shadow"`
/// - Keywords in segments: `word-break` → `"word-break"` (where `break`
///   is a Rust keyword but accepted as an identifier segment)
/// - Leading dashes: `-webkit-user-select` → `"-webkit-user-select"`
/// - Consecutive dashes: `--my-custom-prop` → `"--my-custom-prop"`
///
/// The unquoted form is tokenized by the Rust lexer as a series of
/// `Ident`, `-`, `Ident`, `-`, `Ident`, … tokens. This function
/// collects them and joins with `-` to reconstruct the original
/// identifier chain name.
///
/// # Arguments
///
/// - `ParseStream` - The syn parse stream to read from.
///
/// # Returns
///
/// - `syn::Result<String>` - The reconstructed identifier chain name string.
pub(crate) fn parse_ident_name(input: ParseStream) -> syn::Result<String> {
    if input.peek(LitStr) {
        let literal_string: LitStr = input.parse()?;
        return Ok(literal_string.value());
    }
    let mut name: String = String::new();
    while input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        name.push(CHAR_HYPHEN);
    }
    if !input.is_empty() && !input.peek(Token![:]) {
        let first_segment: String = parse_ident_segment(input)?;
        name.push_str(&first_segment);
    }
    while input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        name.push(CHAR_HYPHEN);
        let next_segment: String = parse_ident_segment(input)?;
        name.push_str(&next_segment);
    }
    Ok(name)
}

/// Reconstructs an identifier chain name from a raw `proc_macro2::TokenStream`.
///
/// Used when `var!(bg-primary)` appears inside `class!` and the macro
/// body tokens are `bg`, `-`, `primary` (three separate tokens). This
/// function walks the token trees and joins `Ident`, `-`, `Ident`
/// sequences into a single identifier chain string.
///
/// Also supports the string-literal form `var!("bg-primary")` by
/// extracting the value from the `LitStr` token.
///
/// Raw identifiers (e.g., `r#box`) are automatically stripped of their
/// `r#` prefix.
///
/// Supports leading dashes (e.g., `-webkit-user-select`) and consecutive
/// dashes (e.g., `--my-custom-prop`) by preserving all `-` punct tokens.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The raw token stream to reconstruct.
///
/// # Returns
///
/// - `String` - The reconstructed identifier chain name.
pub(crate) fn reconstruct_ident_from_tokens(tokens: &proc_macro2::TokenStream) -> String {
    let iter: Peekable<proc_macro2::token_stream::IntoIter> = tokens.clone().into_iter().peekable();
    let mut result: String = String::new();
    for token in iter {
        match token {
            proc_macro2::TokenTree::Ident(ident) => {
                let raw_name: String = ident.to_string();
                let clean_name: String = raw_name
                    .strip_prefix(RAW_IDENT_PREFIX)
                    .unwrap_or(&raw_name)
                    .to_string();
                result.push_str(&clean_name);
            }
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == CHAR_HYPHEN => {
                result.push(CHAR_HYPHEN);
            }
            proc_macro2::TokenTree::Group(group) => {
                let inner: String = reconstruct_ident_from_tokens(&group.stream());
                result.push_str(&inner);
            }
            proc_macro2::TokenTree::Literal(literal) => {
                let literal_token_stream: proc_macro2::TokenStream =
                    proc_macro2::TokenTree::Literal(literal.clone()).into();
                if let Ok(literal_string) = syn::parse2::<LitStr>(literal_token_stream) {
                    result.push_str(&literal_string.value());
                } else {
                    let literal_text: String = literal.to_string();
                    if !literal_text.starts_with(CHAR_DOUBLE_QUOTE) {
                        result.push_str(&literal_text);
                    }
                }
            }
            _ => {}
        }
    }
    result
}
