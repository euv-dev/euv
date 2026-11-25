use super::*;

/// Construction helpers for `AvailableSpace`.
impl AvailableSpace {
    /// Creates root-level `AvailableSpace` matching the window viewport.
    ///
    /// # Arguments
    ///
    /// - `Size` - Logical viewport dimensions.
    ///
    /// # Returns
    ///
    /// - `Self` - Root available space at `(0, 0)`.
    #[must_use]
    pub fn root(viewport: Size) -> Self {
        Self::new(
            viewport.get_width(),
            viewport.get_height(),
            viewport,
            0.0,
            0.0,
        )
    }
}
