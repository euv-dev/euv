use super::*;

/// Parsed input for `watch!((sig1, sig2) => closure)` or auto-tracked `watch!(|| { ... })`.
pub(crate) struct WatchInput {
    /// Watched reactive signal expressions (empty for auto-tracked shorthand).
    pub(crate) signals: Vec<Expr>,
    /// Callback closure invoked with cloned signal values or zero args when auto-tracked.
    pub(crate) callback: Expr,
}

/// Parsed input for `computed!((sig1, sig2) -> RetType => closure)` or auto-tracked `computed!(|| expr)`.
pub(crate) struct ComputedInput {
    /// Source reactive signal expressions (empty for auto-tracked shorthand).
    pub(crate) signals: Vec<Expr>,
    /// Optional explicit return type of the memoized value.
    pub(crate) return_type: Option<Type>,
    /// Derivation closure receiving cloned signal values or zero args when auto-tracked.
    pub(crate) callback: Expr,
}
