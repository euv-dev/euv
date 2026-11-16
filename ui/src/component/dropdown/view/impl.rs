use super::*;

/// Official submenu hover defaults; click and keyboard activation remain independent.
impl Default for EuvDropdownMenuSubTriggerProps {
    /// Constructs a default [`EuvDropdownMenuSubTriggerProps`] value.
    fn default() -> Self {
        Self {
            label: "",
            open: Signal::default(),
            disabled: false,
            open_on_hover: true,
            delay_ms: 100,
            close_delay_ms: 0,
        }
    }
}

/// Default construction for [`EuvDropdownMenuItemProps`].
impl Default for EuvDropdownMenuItemProps {
    /// Constructs a default [`EuvDropdownMenuItemProps`] value.
    fn default() -> Self {
        Self {
            label: "",
            shortcut: "",
            value: "",
            disabled: false,
            on_select: None,
            open: Signal::default(),
            close_on_click: true,
        }
    }
}

#[cfg(target_arch = "wasm32")]
/// Inherent implementation of [`DropdownSubHover`].
impl DropdownSubHover {
    /// Invalidates pending work before any lifecycle or option transition.
    pub(super) fn cancel(&self) {
        self.get_intent().set(None);
        if let Some(id) = self.get_timer().take()
            && let Some(window) = window()
        {
            window.clear_timeout_with_handle(id);
        }
    }

    /// Commits only a still-live intent; opening never enters the child popup.
    pub(super) fn commit(&self) {
        self.get_timer().set(None);
        let intent: Option<bool> = self.get_intent().take();
        if !self.get_mounted().get() || !self.get_parent().get().get() {
            return;
        }
        let Some(trigger) = self
            .get_target()
            .get_cloned()
            .filter(|trigger: &Element| trigger.is_connected())
        else {
            return;
        };
        if let Some(next) = intent {
            let (disabled, enabled, _, _): (bool, bool, u32, u32) = self.get_options().get();
            if next
                && !dropdown_hover_eligible(
                    DROPDOWN_POINTER_MOUSE,
                    true,
                    disabled,
                    enabled,
                    self.get_parent().get().get(),
                )
            {
                return;
            }
            if next {
                if let Some(panel) = trigger.owner_document().and_then(|document: Document| {
                    document.get_element_by_id(&format!(
                        "euv-dropdown-{}-menu",
                        self.get_open().get_inner()
                    ))
                }) {
                    let _: Result<(), JsValue> = panel.remove_attribute(DROPDOWN_FOCUS_ATTRIBUTE);
                }
                if let Some(item) = trigger.dyn_ref::<HtmlElement>() {
                    let options: web_sys::FocusOptions = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);
                    let _: Result<(), JsValue> = item.focus_with_options(&options);
                }
            }
            self.get_open().set(next);
        }
    }

    /// Keeps a single deadline, rather than restarting it for every pointer move.
    ///
    /// # Arguments
    ///
    /// - `Option<bool>` - A `Option<bool>` parameter.
    pub(super) fn schedule(&self, intent: Option<bool>) {
        if intent.is_some() && self.get_intent().get() == intent {
            return;
        }
        self.cancel();
        let Some(next) = intent.filter(|next: &bool| *next != self.get_open().get()) else {
            return;
        };
        self.get_intent().set(Some(next));
        let (_, _, delay, close_delay): (bool, bool, u32, u32) = self.get_options().get();
        let millis: u32 = if next { delay } else { close_delay };
        if millis == 0 {
            self.commit();
        } else if let Some(window) = window()
            && let Some(callback) = self.get_callback().borrow().as_ref()
            && let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                millis.min(i32::MAX as u32) as i32,
            )
        {
            self.get_timer().set(Some(id));
        }
    }
}
