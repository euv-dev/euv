use super::*;

/// Handle to a reactive side effect automatically re-run when tracked dependencies change.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Effect {
    /// Backing arena identifier for this effect.
    pub(crate) id: EffectId,
}
