use super::*;

/// Parsed input for `watch!((sig1, sig2) => closure)`, `watch!(global Type => closure)`, or auto-tracked `watch!(|| { ... })`.
pub(crate) struct WatchInput {
    /// Watched reactive signal expressions (empty for auto-tracked or global shorthand).
    pub(crate) signals: Vec<Expr>,
    /// Optional global state type watched via `watch!(global MyState => |prev, next| { ... })`.
    pub(crate) global_type: Option<Type>,
    /// Callback closure invoked with cloned signal values, `(prev, next)`, or zero args when auto-tracked.
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
