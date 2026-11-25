use super::*;

/// Parsed AST node inside a `view!` declarative tree.
pub(crate) enum ViewNode {
    /// Element invocation with properties and children (`row { ... }`).
    Element(ViewElement),
    /// Static UTF-8 text literal (`"Hello"`).
    StaticText(LitStr),
    /// Dynamic Rust expression or closure (`{ expr }`).
    Expr(Expr),
}
