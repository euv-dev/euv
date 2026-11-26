use super::*;

/// Deterministic ownership scope managing the lifetime of reactive primitives and cleanups.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scope {
    /// Backing arena identifier for this ownership scope.
    pub(crate) id: ScopeId,
}
