use super::*;

/// Single `name: expr` property inside a `view!` element block.
pub(crate) struct ViewProp {
    /// Property identifier.
    pub(crate) name: Ident,
    /// Property value expression.
    pub(crate) value: Expr,
}

/// Element node parsed from `tag { prop: val ... children ... }`.
pub(crate) struct ViewElement {
    /// Element tag or component identifier.
    pub(crate) tag: Ident,
    /// Ordered properties declared on the element.
    pub(crate) props: Vec<ViewProp>,
    /// Ordered child nodes inside the element.
    pub(crate) children: Vec<ViewNode>,
}

/// Top-level `view!(ctx_expr, { root_node })` input.
pub(crate) struct ViewMacroInput {
    /// Expression evaluating to `&mut BuildContext`.
    pub(crate) ctx: Expr,
    /// Root view element or expression.
    pub(crate) root: ViewNode,
}
