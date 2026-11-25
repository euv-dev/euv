use super::*;

/// Bitmask helpers for `DirtyFlags`.
impl DirtyFlags {
    /// Returns a clean flag set (`DIRTY_NONE`).
    ///
    /// # Returns
    ///
    /// - `Self` - Empty dirty flags.
    #[must_use]
    pub const fn none() -> Self {
        Self { bits: DIRTY_NONE }
    }

    /// Returns a fully dirty flag set (`DIRTY_ALL`).
    ///
    /// # Returns
    ///
    /// - `Self` - Fully dirty flags.
    #[must_use]
    pub const fn all() -> Self {
        Self { bits: DIRTY_ALL }
    }

    /// Sets the specified bitmask flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dirty flags.
    /// - `u16` - Bits to set.
    pub fn insert(&mut self, mask: u16) {
        self.set_bits(self.get_bits() | mask);
    }

    /// Clears the specified bitmask flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable dirty flags.
    /// - `u16` - Bits to clear.
    pub fn remove(&mut self, mask: u16) {
        self.set_bits(self.get_bits() & !mask);
    }

    /// Returns true if any bit in `mask` is currently set.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dirty flags.
    /// - `u16` - Mask to test.
    ///
    /// # Returns
    ///
    /// - `bool` - True if at least one bit overlaps.
    #[must_use]
    pub fn intersects(&self, mask: u16) -> bool {
        (self.get_bits() & mask) != 0
    }

    /// Returns true when no dirty bits are set.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Dirty flags.
    ///
    /// # Returns
    ///
    /// - `bool` - True when completely clean.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.get_bits() == DIRTY_NONE
    }
}
