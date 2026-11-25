use super::*;

/// Generation-checked handle to a node inside the flat `SceneTree` arena.
#[derive(Clone, Copy, Debug, Default, Eq, Getter, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    /// Zero-based slot index inside the scene arena.
    #[get(type(copy))]
    pub(crate) index: u32,
    /// Generation counter guarding against stale node references.
    #[get(type(copy))]
    pub(crate) generation: u32,
}
