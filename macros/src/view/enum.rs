use super::*;

/// Condition expression for a `ViewIf` branch inside `view!`.
pub(crate) enum ViewIfCond {
    /// Boolean expression condition (`if cond { ... }`).
    Expr(Box<Expr>),
    /// Pattern-matching condition (`if let pat = expr { ... }`).
    Let(Box<Pat>, Box<Expr>),
}

/// Optional `else` continuation of a `ViewIf` node.
pub(crate) enum ViewElseBranch {
    /// Chained `else if ... { ... }`.
    ElseIf(Box<ViewIf>),
    /// Terminal `else { nodes... }` sibling block.
    ElseBlock(Vec<ViewNode>),
}

/// Parsed AST node inside a `view!` declarative tree.
pub(crate) enum ViewNode {
    /// Primitive element or typed component invocation (`row { ... }` or `EuvCard(a, b) { ... }`).
    Element(Box<ViewElement>),
    /// Static UTF-8 text literal (`"Hello"`).
    StaticText(LitStr),
    /// Dynamic Rust expression or closure (`{ expr }`).
    Expr(Box<Expr>),
    /// Spread sibling iterator (`..children_iter`).
    Spread(Span, Box<Expr>),
    /// Conditional sibling branch (`if cond { ... } [else { ... }]`).
    If(Box<ViewIf>),
    /// Iterative sibling loop (`for pat in iter { ... }`).
    For(Box<ViewFor>),
    /// Pattern-matching sibling switch (`match expr { ... }`).
    Match(Box<ViewMatch>),
}
