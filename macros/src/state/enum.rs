use super::*;

/// Parsed `global_state!(...)` invocation supporting lookup (`Type`) or lazy initialization (`Type = init_expr`).
pub(crate) enum GlobalStateInput {
    /// Lookup an already-registered global state (`Option<Signal<T>>`).
    Lookup(Box<Type>),
    /// Lookup or lazily initialize global state (`Signal<T>`).
    Init {
        /// Global state type.
        ty: Box<Type>,
        /// Initial value expression.
        init: Box<Expr>,
    },
}
