use super::*;

/// Compact 16-bit surgical dirty flag mask enabling O(1) clean-subtree skipping.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct DirtyFlags {
    /// Raw bitmask composed of `DIRTY_*` constants.
    #[get(type(copy))]
    pub(crate) bits: u16,
}
