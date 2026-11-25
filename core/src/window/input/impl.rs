use super::*;

/// Convenience constructors for `PointerEvent`.
impl PointerEvent {
    /// Creates a primary mouse pointer event at logical `(x, y)`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Logical X coordinate.
    /// - `f32` - Logical Y coordinate.
    ///
    /// # Returns
    ///
    /// - `Self` - Mouse pointer event.
    #[must_use]
    pub fn mouse(x: f32, y: f32) -> Self {
        Self::new(
            0,
            PointerDevice::Mouse,
            Point::new(x, y),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        )
    }

    /// Creates a touch pointer event for `finger_id` at logical `(x, y)`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Touch contact identifier.
    /// - `f32` - Logical X coordinate.
    /// - `f32` - Logical Y coordinate.
    ///
    /// # Returns
    ///
    /// - `Self` - Touch pointer event.
    #[must_use]
    pub fn touch(finger_id: u64, x: f32, y: f32) -> Self {
        Self::new(
            finger_id,
            PointerDevice::Touch,
            Point::new(x, y),
            PointerButton::Primary,
            1.0,
            Modifiers::default(),
        )
    }
}
