use super::*;

/// Conversion helpers for `SlotId`.
impl SlotId {
    /// Converts the slot index into a `usize` for slice indexing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The slot identifier to convert.
    ///
    /// # Returns
    ///
    /// - `usize` - The zero-based vector index.
    #[must_use]
    pub fn as_usize(&self) -> usize {
        self.get_index() as usize
    }
}

/// Construction and inspection helpers for `SignalId`.
impl SignalId {
    /// Creates a new `SignalId` wrapping the provided `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `SlotId` - The backing arena slot identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - The constructed signal identifier.
    #[must_use]
    pub fn from_slot(slot: SlotId) -> Self {
        Self(slot)
    }

    /// Returns the underlying `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The signal identifier.
    ///
    /// # Returns
    ///
    /// - `SlotId` - The underlying slot identifier.
    #[must_use]
    pub fn slot(&self) -> SlotId {
        self.0
    }
}

/// Construction and inspection helpers for `MemoId`.
impl MemoId {
    /// Creates a new `MemoId` wrapping the provided `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `SlotId` - The backing arena slot identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - The constructed memo identifier.
    #[must_use]
    pub fn from_slot(slot: SlotId) -> Self {
        Self(slot)
    }

    /// Returns the underlying `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The memo identifier.
    ///
    /// # Returns
    ///
    /// - `SlotId` - The underlying slot identifier.
    #[must_use]
    pub fn slot(&self) -> SlotId {
        self.0
    }
}

/// Construction and inspection helpers for `EffectId`.
impl EffectId {
    /// Creates a new `EffectId` wrapping the provided `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `SlotId` - The backing arena slot identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - The constructed effect identifier.
    #[must_use]
    pub fn from_slot(slot: SlotId) -> Self {
        Self(slot)
    }

    /// Returns the underlying `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The effect identifier.
    ///
    /// # Returns
    ///
    /// - `SlotId` - The underlying slot identifier.
    #[must_use]
    pub fn slot(&self) -> SlotId {
        self.0
    }
}

/// Construction and inspection helpers for `ScopeId`.
impl ScopeId {
    /// Creates a new `ScopeId` wrapping the provided `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `SlotId` - The backing arena slot identifier.
    ///
    /// # Returns
    ///
    /// - `Self` - The constructed scope identifier.
    #[must_use]
    pub fn from_slot(slot: SlotId) -> Self {
        Self(slot)
    }

    /// Returns the underlying `SlotId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The scope identifier.
    ///
    /// # Returns
    ///
    /// - `SlotId` - The underlying slot identifier.
    #[must_use]
    pub fn slot(&self) -> SlotId {
        self.0
    }
}
