use super::*;

/// Single `name: expr` property inside a `view!` element or component block.
pub(crate) struct ViewProp {
    /// Property identifier.
    pub(crate) name: Ident,
    /// Property value expression.
    pub(crate) value: Expr,
}

/// Fluent builder method call `flag()` or `.method(args...)` inside a component block.
pub(crate) struct ViewFlag {
    /// Fluent builder method identifier.
    pub(crate) name: Ident,
    /// Optional positional method arguments (empty for `outline()`, non-empty for `.range(0, 20)`).
    pub(crate) args: Vec<Expr>,
}

/// Conditional rendering block inside `view!`.
pub(crate) struct ViewIf {
    /// Span of the `if` keyword.
    pub(crate) span: Span,
    /// Boolean or `let pat = expr` condition.
    pub(crate) cond: ViewIfCond,
    /// Ordered sibling nodes mounted when the condition matches.
    pub(crate) then_nodes: Vec<ViewNode>,
    /// Optional `else if` or `else` continuation.
    pub(crate) else_branch: Option<ViewElseBranch>,
}

/// Iterative `for pat in iter { nodes... }` block inside `view!`.
pub(crate) struct ViewFor {
    /// Span of the `for` keyword.
    pub(crate) span: Span,
    /// Loop item pattern.
    pub(crate) pat: Pat,
    /// Iterable expression.
    pub(crate) iter: Expr,
    /// Ordered sibling nodes mounted per iteration.
    pub(crate) body: Vec<ViewNode>,
}

/// Single arm inside a `match expr { pat [if guard] => ... }` view block.
pub(crate) struct ViewMatchArm {
    /// Match pattern.
    pub(crate) pat: Pat,
    /// Optional `if guard` expression.
    pub(crate) guard: Option<Expr>,
    /// Ordered sibling nodes mounted when this arm matches.
    pub(crate) body: Vec<ViewNode>,
}

/// Pattern-matching `match expr { ... }` block inside `view!`.
pub(crate) struct ViewMatch {
    /// Span of the `match` keyword.
    pub(crate) span: Span,
    /// Scrutinee expression.
    pub(crate) expr: Expr,
    /// Ordered match arms.
    pub(crate) arms: Vec<ViewMatchArm>,
}

/// Element or component node parsed from `Path[(args...)] [{ flags/props/children }]`.
pub(crate) struct ViewElement {
    /// Element tag or component type/function path.
    pub(crate) path: Path,
    /// Optional parenthesized constructor or function call arguments.
    pub(crate) args: Option<Vec<Expr>>,
    /// Zero-argument fluent flag methods on components (`outline()`, `sm()`).
    pub(crate) flags: Vec<ViewFlag>,
    /// Ordered properties declared on the element or component.
    pub(crate) props: Vec<ViewProp>,
    /// Ordered child nodes inside the element or component.
    pub(crate) children: Vec<ViewNode>,
}

/// Top-level `view!` macro input containing one or more root nodes under the framework-managed `Context`.
pub(crate) struct ViewMacroInput {
    /// One or more top-level sibling nodes (multiple roots wrap cleanly in `fragment`).
    pub(crate) roots: Vec<ViewNode>,
}
