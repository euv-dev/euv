use super::*;

/// A parameter definition in a parameterized class.
///
/// Each parameter has a name (identifier), an explicit type annotation,
/// and is used as a placeholder in CSS property values.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassParam {
    /// The parameter name identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// The explicit type annotation for this parameter (e.g., `&str`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) param_type: Type,
}

/// A selector block parsed from the `class!` macro.
///
/// Represents a CSS pseudo-class, pseudo-element, or compound selector block
/// parsed directly from CSS syntax:
/// - Pseudo-class: `:hover { ... }`, `:focus-visible { ... }`, `:nth-child(2n) { ... }`
/// - Pseudo-element: `::before { ... }`, `::-webkit-scrollbar { ... }`
/// - Compound selectors: `::-webkit-scrollbar-thumb:hover { ... }`
///
/// The selector string is reconstructed verbatim from the parsed tokens,
/// preserving the original CSS syntax without modification.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct SelectorBlock {
    /// The CSS selector string (e.g., ":hover", "::before", "::-webkit-scrollbar-thumb:hover").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selector: String,
    /// The style properties inside this selector block.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
    /// Nested selector blocks inside this selector block
    /// (e.g., `::-webkit-scrollbar { ::-webkit-scrollbar-thumb { ... } }`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selector_blocks: Vec<SelectorBlock>,
}

/// An at-rule block parsed from the `class!` macro.
///
/// Represents any CSS at-rule parsed directly from CSS syntax:
/// - `@media (max-width: 767px) { ... }`
/// - `@keyframes fade-in { ... }`
/// - `@supports (display: grid) { ... }`
/// - `@layer base { ... }`
/// - `@container (width > 400px) { ... }`
/// - `@property --my-color { ... }`
/// - `@scope (.style) { ... }`
/// - `@font-face { ... }`
/// - `@charset "UTF-8";`
/// - `@import url("style.css");`
/// - And all other CSS at-rules
#[derive(Clone, Data, Debug, New)]
pub(crate) struct AtRuleBlock {
    /// The kind of at-rule (media, keyframes, supports, etc.).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: AtRuleKind,
    /// The prelude/query string after the at-rule name
    /// (e.g., "(max-width: 767px)" for @media, "fade-in" for @keyframes).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) prelude: String,
    /// The style properties inside this at-rule block.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
    /// Nested selector blocks inside this at-rule block
    /// (e.g., `@media (...) { :hover { ... } ::before { ... } }`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selector_blocks: Vec<SelectorBlock>,
    /// Nested at-rule blocks inside this at-rule block
    /// (e.g., `@supports (...) { @media (...) { ... } }`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) at_rule_blocks: Vec<AtRuleBlock>,
}

/// A parent class reference in an extends clause.
///
/// Represents a class inheritance call like `c_primary_button(1)` or `c_base()`.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassExtend {
    /// The parent class name identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// The arguments passed to the parent class function (as raw token streams).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) args: Vec<proc_macro2::TokenStream>,
}

/// A single class definition parsed from the `class!` macro.
///
/// Contains visibility, name, optional generic parameters, optional parameters,
/// style properties, optional selector blocks (pseudo-classes/pseudo-elements),
/// and optional at-rule blocks (media queries, keyframes, etc.).
#[derive(Clone, Data, Debug)]
pub(crate) struct ClassDef {
    /// The visibility modifier (e.g., `pub`, `pub(crate)`, `pub(super)`, or none).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) visibility: Visibility,
    /// The class name identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: Ident,
    /// Generic parameters and optional where clause
    /// (e.g., `<T: Clone, U> where U: Display`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generics: Generics,
    /// Optional parameter list for a parameterized class.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) params: Option<Vec<ClassParam>>,
    /// Parent classes to inherit properties from (e.g., `c_primary_button(1)`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) extends: Vec<ClassExtend>,
    /// The style properties for this class.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
    /// The selector blocks (pseudo-classes/pseudo-elements) for this class.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) selector_blocks: Vec<SelectorBlock>,
    /// The at-rule blocks (media queries, keyframes, etc.) for this class.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) at_rule_blocks: Vec<AtRuleBlock>,
}

/// The entire `class!` macro input, containing multiple class definitions.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct ClassInput {
    /// The list of class definitions.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) classes: Vec<ClassDef>,
}

/// Intermediate result from parsing a block's content.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct BlockContent {
    /// The CSS properties in this block.
    pub(crate) properties: Vec<(ClassPropKey, ClassPropValue)>,
    /// The nested selector blocks.
    pub(crate) selector_blocks: Vec<SelectorBlock>,
    /// The nested at-rule blocks.
    pub(crate) at_rule_blocks: Vec<AtRuleBlock>,
}

/// Parameters for `emit_once_lock_fn`.
///
/// Bundles the values needed to generate a `OnceLock`-based static function body
/// for a no-param class definition.
pub(crate) struct OnceLockParams<'a> {
    /// The visibility modifier for the generated function.
    pub(crate) visibility: &'a Visibility,
    /// The function name token stream.
    pub(crate) fn_name_token: &'a proc_macro2::TokenStream,
    /// The `OnceLock` constant name token stream.
    pub(crate) const_name_token: &'a proc_macro2::TokenStream,
    /// The class name as a string literal.
    pub(crate) class_name_str: &'a str,
    /// Token stream that evaluates to the CSS style string.
    pub(crate) style_expr: &'a proc_macro2::TokenStream,
    /// Token stream that evaluates to the selector rules vector.
    pub(crate) selector_expr: &'a proc_macro2::TokenStream,
    /// Token stream that evaluates to the at-rule rules vector.
    pub(crate) at_rule_expr: &'a proc_macro2::TokenStream,
}

/// Everything needed to emit the per-class parameter cache function.
///
/// Bundled into one struct so [`emit_param_css_cache_fn`](super::emit_param_css_cache_fn)
/// takes a single argument instead of twelve positional ones.
#[derive(Data)]
pub(crate) struct ParamCssCacheArgs<'a> {
    /// The visibility the generated cache function is emitted with.
    pub(crate) visibility: &'a Visibility,
    /// The generated function's name token.
    pub(crate) fn_name_token: &'a proc_macro2::TokenStream,
    /// The span to attribute the cache constant's name to.
    pub(crate) fn_name_span: proc_macro2::Span,
    /// The kebab-case class name the cache is keyed by.
    pub(crate) class_name_str: &'a str,
    /// The parameter name identifiers forming the cache key.
    pub(crate) param_names: &'a [proc_macro2::TokenStream],
    /// The `unique_name` expression driving cache key uniqueness.
    pub(crate) unique_name_expr: &'a proc_macro2::TokenStream,
    /// The expression producing the class's style declarations.
    pub(crate) style_expr: &'a proc_macro2::TokenStream,
    /// The expression producing the class's selector.
    pub(crate) selector_expr: &'a proc_macro2::TokenStream,
    /// The expression producing the at-rule wrapping the declarations.
    pub(crate) at_rule_expr: &'a proc_macro2::TokenStream,
    /// The generated parameter definitions for the class function.
    pub(crate) param_defs: &'a [proc_macro2::TokenStream],
    /// The class function's generics and where clause.
    pub(crate) generics: &'a syn::Generics,
}
