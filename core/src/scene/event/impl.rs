use super::*;

/// Safe handler lookup for `EventHandlers`.
impl EventHandlers {
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
