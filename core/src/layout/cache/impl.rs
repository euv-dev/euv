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

    /// Marks whether this node's auto width or height is stretched to fill the available slot.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source available space.
    /// - `bool` - Whether auto width stretches to `width`.
    /// - `bool` - Whether auto height stretches to `height`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated available space with stretch flags applied.
    #[must_use]
    pub fn with_stretch(mut self, stretch_width: bool, stretch_height: bool) -> Self {
        self.set_stretch_width(stretch_width);
        self.set_stretch_height(stretch_height);
        self
    }
}
