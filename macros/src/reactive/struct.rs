use super::*;

/// Parsed input for `watch!((sig1, sig2) => closure)`.
pub(crate) struct WatchInput {
    /// Watched reactive signal expressions.
    pub(crate) signals: Vec<Expr>,
    /// Callback closure invoked with cloned signal values.
    pub(crate) callback: Expr,
}

/// Parsed input for `computed!((sig1, sig2) -> RetType => closure)`.
pub(crate) struct ComputedInput {
    /// Source reactive signal expressions.
    pub(crate) signals: Vec<Expr>,
    /// Explicit return type of the memoized value.
    pub(crate) return_type: Type,
    /// Derivation closure receiving cloned signal values.
    pub(crate) callback: Expr,
}
