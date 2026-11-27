use super::*;

/// Single newline-separated statement inside a `style! { ... }` block.
pub(crate) enum StyleStmt {
    /// Named property assignment (`width: Length::Percent(100.0)`).
    KeyValue {
        /// Property key identifier.
        key: Ident,
        /// Property value expression.
        value: Expr,
    },
    /// Type-directed value shorthand (`AlignItems::Center`, `Overflow::Hidden`, `base_style`).
    PropExpr(Expr),
}

/// Top-level `style! { ... }` invocation form: either named functions or an inline `Style` expression.
pub(crate) enum StyleBlockInput {
    /// Named style constructor functions (`style! { c_box { ... } }`).
    Defs(Vec<StyleDef>),
    /// Inline anonymous `Style` expression (`style! { AlignItems::Center\n Overflow::Hidden }`).
    Inline(Vec<StyleStmt>),
}
