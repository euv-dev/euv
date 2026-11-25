use super::*;

/// Single `key: expr;` rule inside a `style!` definition.
pub(crate) struct StyleRule {
    /// Rule property name.
    pub(crate) key: Ident,
    /// Rule expression value.
    pub(crate) value: Expr,
}

/// Named style constructor function definition inside `style!`.
pub(crate) struct StyleDef {
    /// Visibility (`pub`, `pub(crate)`, or private).
    pub(crate) vis: Visibility,
    /// Generated function name.
    pub(crate) name: Ident,
    /// Ordered style property rules.
    pub(crate) rules: Vec<StyleRule>,
}

/// Collection of `StyleDef` entries in a `style! { ... }` invocation.
pub(crate) struct StyleBlockInput {
    /// Parsed style definitions.
    pub(crate) defs: Vec<StyleDef>,
}

/// Single `vis const NAME: Type = expr;` design token definition.
pub(crate) struct TokenEntry {
    /// Visibility modifier.
    pub(crate) vis: Visibility,
    /// Constant name.
    pub(crate) name: Ident,
    /// Explicit token type.
    pub(crate) ty: Type,
    /// Constant initializer expression.
    pub(crate) value: Expr,
}

/// Collection of `TokenEntry` definitions inside `tokens! { ... }`.
pub(crate) struct TokenBlockInput {
    /// Parsed token entries.
    pub(crate) entries: Vec<TokenEntry>,
}
