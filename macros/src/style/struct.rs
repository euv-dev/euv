use super::*;

/// Named (optionally parameterized) style constructor function definition inside `style!`.
pub(crate) struct StyleDef {
    /// Visibility (`pub`, `pub(crate)`, or private).
    pub(crate) vis: Visibility,
    /// Generated function name.
    pub(crate) name: Ident,
    /// Optional function parameters (`c_pill(bg: Color, fg: Color) { ... }`).
    pub(crate) params: Vec<FnArg>,
    /// Ordered semicolon-terminated style statements.
    pub(crate) stmts: Vec<StyleStmt>,
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
