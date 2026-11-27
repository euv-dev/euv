//! euv-macros
//!
//! Zero-overhead native procedural macros for the cross-platform euv UI framework,
//! compiling declarative `view!`, `style!`, `tokens!`, `watch!`, `computed!`, and
//! `euv_main!` directly into arena scene graph operations with span-preserving
//! IDE completion and actionable compile-time diagnostics.

mod component;
mod diagnostic;
mod entry;
mod event;
mod props;
mod reactive;
mod state;
mod style;
mod view;

pub(crate) use {
    component::*, diagnostic::*, entry::*, event::*, props::*, reactive::*, state::*, style::*,
    view::*,
};

use std::{collections::HashSet, mem::take};

use {
    proc_macro::TokenStream,
    proc_macro2::Span,
    quote::{quote, quote_spanned},
    syn::{
        Attribute, Data, DeriveInput, Error, Expr, ExprRange, Field, Fields, FnArg, Ident,
        ImplGenerics, ImplItem, ImplItemFn, ItemImpl, ItemStruct, LitStr, Meta, Pat, Path,
        PathArguments, PathSegment, RangeLimits, Result, ReturnType, Stmt, Token, Type,
        TypeGenerics, Visibility, WhereClause, braced, parenthesized,
        parse::{Parse, ParseBuffer, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
        spanned::Spanned,
        token::{Brace, Paren},
    },
};

/// Declarative zero-VDOM, zero-context UI construction macro managing `Context` automatically.
///
/// Component parameters must be passed as self-documenting named attributes inside `Component { prop: value }` blocks; positional `Component(...)` calls are rejected at compile time:
///
/// ```compile_fail
/// use euv::*;
///
/// #[derive(Props)]
/// struct DemoBtn {
///     label: String,
/// }
///
/// #[component]
/// impl DemoBtn {
///     fn render(&self) -> NodeId {
///         view! { text { "ok" } }
///     }
/// }
///
/// fn invalid() -> NodeId {
///     view! {
///         DemoBtn("positional")
///     }
/// }
/// ```
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

/// Compile-time style macro compiling declarative newline-separated rules directly into typed `Style` constructor functions or inline `Style` expressions.
///
/// Every statement inside `style! { ... }` must occupy its own line without semicolons. Placing two rules on one line is a compile-time error:
///
/// ```compile_fail
/// use euv::*;
///
/// let _bad: Style = style! {
///     AlignItems::Center Overflow::Hidden
/// };
/// ```
///
/// Trailing semicolons are also rejected in favor of clean newline separation:
///
/// ```compile_fail
/// use euv::*;
///
/// let _bad: Style = style! {
///     AlignItems::Center;
///     Overflow::Hidden;
/// };
/// ```
///
/// # Arguments
///
/// - `TokenStream` - Style rule block tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded typed `Style` functions or inline `Style` expression.
#[proc_macro]
pub fn style(input: TokenStream) -> TokenStream {
    let parsed: StyleBlockInput = parse_macro_input!(input as StyleBlockInput);
    expand_style(parsed).into()
}

/// Compile-time design token macro generating typed `const` items.
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

/// Attribute macro marking a struct as a typed cross-component `CustomEvent` with ergonomic `::emit` and `::on` helpers.
///
/// # Arguments
///
/// - `TokenStream` - Unused attribute arguments.
/// - `TokenStream` - Target struct item tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded struct plus `CustomEvent` implementation and helpers.
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: ItemStruct = parse_macro_input!(item as ItemStruct);
    expand_event_attr(parsed).into()
}

/// Dispatches a typed cross-component custom event through `EventBus`.
///
/// # Arguments
///
/// - `TokenStream` - Event value expression tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `::euv::EventBus::emit(...)` call.
#[proc_macro]
pub fn emit(input: TokenStream) -> TokenStream {
    let expr: Expr = parse_macro_input!(input as Expr);
    expand_emit(expr).into()
}

/// Subscribes a scope-owned listener to a typed cross-component custom event (`on_event!(MyEvent => |ev| { ... })`).
///
/// # Arguments
///
/// - `TokenStream` - `EventType => handler` tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `::euv::EventBus::on::<EventType, _>(handler)` returning `::euv::Effect`.
#[proc_macro]
pub fn on_event(input: TokenStream) -> TokenStream {
    let parsed: OnEventInput = parse_macro_input!(input as OnEventInput);
    expand_on_event(parsed).into()
}

/// Attribute macro turning a struct into a typed app-wide `GlobalState` with `.provide()`, `::use_global()`, `::get_global()`, `::set_global()`, `::update_global()`, and `::watch_global()`.
///
/// # Arguments
///
/// - `TokenStream` - Unused attribute arguments.
/// - `TokenStream` - Target struct item tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded struct plus global state methods.
#[proc_macro_attribute]
pub fn global_state_store(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: ItemStruct = parse_macro_input!(item as ItemStruct);
    expand_global_state_attr(parsed).into()
}

/// Creates a scoped reactive `LocalState<T>` owned by the active component/page `Scope`.
///
/// # Arguments
///
/// - `TokenStream` - Initial value expression tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `::euv::LocalState::new(...)` expression.
#[proc_macro]
pub fn local_state(input: TokenStream) -> TokenStream {
    let expr: Expr = parse_macro_input!(input as Expr);
    expand_local_state(expr).into()
}

/// Accesses or lazily initializes a typed app-wide `GlobalState` (`global_state!(MyState)` or `global_state!(MyState = init)`).
///
/// # Arguments
///
/// - `TokenStream` - `Type` or `Type = init_expr` tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `::euv::GlobalState` lookup or lazy init call.
#[proc_macro]
pub fn global_state(input: TokenStream) -> TokenStream {
    let parsed: GlobalStateInput = parse_macro_input!(input as GlobalStateInput);
    expand_global_state(parsed).into()
}

/// Attribute macro splitting an `impl MyComponent { fn render(&self) -> NodeId { ... } ... }` block into clean `impl Lifecycle` and `impl Component` blocks.
///
/// # Arguments
///
/// - `TokenStream` - Optional `retention = Freeze | Destroy | Alive` options.
/// - `TokenStream` - Target `impl` block tokens.
///
/// # Returns
///
/// - `TokenStream` - Expanded `Lifecycle`, `Component`, and optional inherent helper `impl` blocks.
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: ComponentAttrArgs = parse_macro_input!(attr as ComponentAttrArgs);
    let parsed: ItemImpl = parse_macro_input!(item as ItemImpl);
    expand_component_attr(args, parsed).into()
}

/// Derive macro implementing `ComponentProps` and span-preserving fluent named-prop setters for `view! { MyComp { prop: value } }`.
///
/// # Arguments
///
/// - `TokenStream` - Target struct definition tokens.
///
/// # Returns
///
/// - `TokenStream` - Generated `ComponentProps` and fluent setter implementations.
#[proc_macro_derive(Props, attributes(prop))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);
    expand_props_derive(parsed).into()
}
