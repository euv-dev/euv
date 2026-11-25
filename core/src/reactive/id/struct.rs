use super::*;

/// Generation-checked slot identifier inside the reactive arena.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct SlotId {
    /// Zero-based slot index inside the arena storage vector.
    #[get(type(copy))]
    pub(crate) index: u32,
    /// Monotonic generation counter preventing ABA use-after-free bugs.
    #[get(type(copy))]
    pub(crate) generation: u32,
}

/// Unique identifier for a reactive signal slot.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignalId(pub(crate) SlotId);

/// Unique identifier for a reactive memo slot.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoId(pub(crate) SlotId);

/// Unique identifier for a reactive effect slot.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectId(pub(crate) SlotId);

/// Unique identifier for a reactive ownership scope.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScopeId(pub(crate) SlotId);
