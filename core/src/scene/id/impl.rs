use super::*;

/// Indexing utilities for `NodeId`.
impl NodeId {
    /// Converts the node slot index to `usize`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Node handle.
    ///
    /// # Returns
    ///
    /// - `usize` - Zero-based arena vector index.
    #[must_use]
    pub fn as_usize(&self) -> usize {
        self.get_index() as usize
    }

    /// Returns true when the handle carries a non-zero generation tag.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Node handle.
    ///
    /// # Returns
    ///
    /// - `bool` - True when non-null.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.get_generation() != 0
    }
}
