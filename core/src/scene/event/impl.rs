use super::*;

/// Safe handler lookup for `EventHandlers`.
impl EventHandlers {
    /// Constructs an empty `EventHandlers` table in `const` contexts without heap allocation.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty event handler collection with all callback slots set to `None`.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            on_click: None,
            on_pointer_down: None,
            on_pointer_move: None,
            on_pointer_up: None,
            on_pointer_enter: None,
            on_pointer_leave: None,
            on_scroll: None,
            on_key_down: None,
            on_change: None,
            on_focus: None,
            on_blur: None,
        }
    }

    /// Returns `true` when zero event callbacks are registered in this collection.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Event handler collection.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether every handler slot is `None`.
    #[must_use]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let Self {
            on_click,
            on_pointer_down,
            on_pointer_move,
            on_pointer_up,
            on_pointer_enter,
            on_pointer_leave,
            on_scroll,
            on_key_down,
            on_change,
            on_focus,
            on_blur,
        } = self;
        on_click.is_none()
            && on_pointer_down.is_none()
            && on_pointer_move.is_none()
            && on_pointer_up.is_none()
            && on_pointer_enter.is_none()
            && on_pointer_leave.is_none()
            && on_scroll.is_none()
            && on_key_down.is_none()
            && on_change.is_none()
            && on_focus.is_none()
            && on_blur.is_none()
    }

    /// Clones the registered handler matching `kind`, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Event handler collection.
    /// - `&UiEventKind` - Event kind being dispatched.
    ///
    /// # Returns
    ///
    /// - `Option<UiEventHandler>` - Matching callback handle.
    #[must_use]
    pub fn handler_for(&self, kind: &UiEventKind) -> Option<UiEventHandler> {
        let Self {
            on_click,
            on_pointer_down,
            on_pointer_move,
            on_pointer_up,
            on_pointer_enter,
            on_pointer_leave,
            on_scroll,
            on_key_down,
            on_change,
            on_focus,
            on_blur,
        } = self;
        match kind {
            UiEventKind::Click(_) => on_click.clone(),
            UiEventKind::PointerDown(_) => on_pointer_down.clone(),
            UiEventKind::PointerMove(_) => on_pointer_move.clone(),
            UiEventKind::PointerUp(_) => on_pointer_up.clone(),
            UiEventKind::PointerEnter => on_pointer_enter.clone(),
            UiEventKind::PointerLeave => on_pointer_leave.clone(),
            UiEventKind::Scroll(_) => on_scroll.clone(),
            UiEventKind::KeyDown(_, _) => on_key_down.clone(),
            UiEventKind::KeyUp(_, _) | UiEventKind::TextInput(_) => None,
            UiEventKind::Change(_) => on_change.clone(),
            UiEventKind::Focus => on_focus.clone(),
            UiEventKind::Blur => on_blur.clone(),
        }
    }
}
