use super::*;

/// Safe inspection helpers for `FocusManager`.
impl FocusManager {
    /// Returns the currently focused node if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Focus manager reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Focused node identifier.
    #[must_use]
    pub fn focused_node(&self) -> Option<NodeId> {
        let Self { focused, .. } = *self;
        focused
    }

    /// Returns the currently hovered node if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Focus manager reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Hovered node identifier.
    #[must_use]
    pub fn hovered_node(&self) -> Option<NodeId> {
        let Self { hovered, .. } = *self;
        hovered
    }

    /// Returns the currently pressed node if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Focus manager reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Pressed node identifier.
    #[must_use]
    pub fn pressed_node(&self) -> Option<NodeId> {
        let Self { pressed, .. } = *self;
        pressed
    }
}
