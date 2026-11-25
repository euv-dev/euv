//! euv-macros
//!
//! Zero-overhead procedural macros for the cross-platform euv UI framework,
//! compiling declarative `view!`, `style!`, `class!`, `tokens!`, `watch!`,
//! `computed!`, `#[component]`, and `euv_main!` directly into arena scene
//! graph operations with zero VDOM overhead.

mod entry;
mod reactive;
mod style;
mod view;

pub(crate) use {entry::*, reactive::*, style::*, view::*};

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        Error, Expr, Ident, LitStr, Result, Token, Type, Visibility, braced, parenthesized,
        parse::{Parse, ParseBuffer, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
        token::Brace,
    },
};

/// Declarative zero-VDOM, zero-context UI construction macro managing `BuildContext` automatically.
///
/// # Arguments
///
/// - `TokenStream` - Invocation tokens `view! { ... }`.
///
/// # Returns
///
/// - `TokenStream` - Expanded arena node construction and reactive binding tokens.
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let parsed: ViewMacroInput = parse_macro_input!(input as ViewMacroInput);
    expand_view(parsed).into()
}

/// Alias for `view!` providing concise zero-context UI DSL ergonomics.
///
/// # Arguments
///
/// - `TokenStream` - Invocation tokens `ui! { ... }`.
///
/// # Returns
///
/// - `TokenStream` - Expanded arena node construction tokens.
#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let parsed: ViewMacroInput = parse_macro_input!(input as ViewMacroInput);
    expand_view(parsed).into()
}

/// Compile-time style macro compiling declarative rules directly into `Style` constructor functions.
///
/// # Arguments
///
/// - `TokenStream` - Style rule block tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded typed `Style` functions.
#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    let parsed: StyleBlockInput = parse_macro_input!(input as StyleBlockInput);
    expand_style(parsed).into()
}

/// Alias for `style!` preserving familiar `class!` naming while compiling to native `Style` values.
///
/// # Arguments
///
/// - `TokenStream` - Style rule block tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded typed `Style` functions.
#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    let parsed: StyleBlockInput = parse_macro_input!(input as StyleBlockInput);
    expand_style(parsed).into()
}

/// Compile-time design token macro generating typed `pub const` values.
///
/// # Arguments
///
/// - `TokenStream` - Token definition entries.
///
/// # Returns
///
/// - `TokenStream` - Expanded constant declarations.
#[proc_macro]
pub fn tokens(input: TokenStream) -> TokenStream {
    let parsed: TokenBlockInput = parse_macro_input!(input as TokenBlockInput);
    expand_tokens(parsed).into()
}

/// Ergonomic reactive side-effect macro watching one or more signals.
///
/// # Arguments
///
/// - `TokenStream` - Signal list and effect closure tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `Effect::new` expression.
#[proc_macro]
pub fn watch(input: TokenStream) -> TokenStream {
    let parsed: WatchInput = parse_macro_input!(input as WatchInput);
    expand_watch(parsed).into()
}

/// Ergonomic memoized derived signal macro with equality short-circuiting.
///
/// # Arguments
///
/// - `TokenStream` - Signal list, return type, and computation closure tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `Memo::new` expression.
#[proc_macro]
pub fn computed(input: TokenStream) -> TokenStream {
    let parsed: ComputedInput = parse_macro_input!(input as ComputedInput);
    expand_computed(parsed).into()
}

/// Cross-platform entry-point macro generating native desktop, mobile, and WebAssembly bootstraps.
///
/// # Arguments
///
/// - `TokenStream` - Application builder function identifier.
///
/// # Returns
///
/// - `TokenStream` - Target-specific entry point declarations.
#[proc_macro]
pub fn euv_main(input: TokenStream) -> TokenStream {
    let parsed: EntryInput = parse_macro_input!(input as EntryInput);
    expand_entry(parsed).into()
}

/// Attribute macro marking declarative cross-platform component functions.
///
/// # Arguments
///
/// - `TokenStream` - Optional attribute arguments.
/// - `TokenStream` - Annotated component function item.
///
/// # Returns
///
/// - `TokenStream` - Component function item.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
