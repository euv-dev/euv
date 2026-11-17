use super::*;

/// Hook eviction tears down the binding before any retained app cleanup callback.
impl Drop for ContextMenuSubBindingScope {
    /// Releases the identity and subscriptions on direct hook storage drop.
    fn drop(&mut self) {
        self.get_slot().borrow_mut().take();
    }
}

/// Detaches both listeners and closes descendants on replacement or unmount.
impl Drop for ContextMenuSubBinding {
    /// Implements the `drop` operation on [`ContextMenuSubBinding`].
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(controller) = self
                .get_hover_owner()
                .get_controller()
                .borrow()
                .as_ref()
                .and_then(Weak::upgrade)
            {
                controller.dispose();
            }
            CONTEXT_MENU_SUB_PARENTS.with(
                |parents: &RefCell<HashMap<usize, Weak<ContextMenuHoverOwner>>>| {
                    let mut parents: RefMut<HashMap<usize, Weak<ContextMenuHoverOwner>>> =
                        parents.borrow_mut();
                    if parents
                        .get(&self.get_open().get_inner())
                        .and_then(Weak::upgrade)
                        .is_some_and(|owner: Rc<ContextMenuHoverOwner>| {
                            Rc::ptr_eq(&owner, self.get_hover_owner())
                        })
                    {
                        parents.remove(&self.get_open().get_inner());
                    }
                },
            );
        }
        self.get_parent_open()
            .unsubscribe(self.get_parent_subscription());
        self.get_open().unsubscribe(self.get_open_subscription());
        if self.get_open().get() {
            self.get_open().set(false);
        }
    }
}

/// Gesture decisions shared by browser handlers and native mechanical tests.
impl ContextMenuLongPress {
    /// Starts only enabled, single-touch gestures, replacing any previous hold.
    ///
    /// # Arguments
    ///
    /// - `bool` - A boolean (`bool`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `f64` - A 64-bit float (`f64`).
    pub(crate) fn start(&mut self, disabled: bool, count: u32, x: i32, y: i32, now: f64) {
        self.cancel();
        if !disabled && count == 1 {
            self.set_origin(Some((x, y, now)));
        }
    }

    /// Cancels multi-touch or movement exceeding the tolerance on either axis.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub(crate) fn moved(&mut self, count: u32, x: i32, y: i32) {
        if let Some((start_x, start_y, _)) = self.get_origin()
            && (count != 1
                || (i64::from(x) - i64::from(start_x)).abs() > i64::from(CONTEXT_MENU_TOUCH_SLOP)
                || (i64::from(y) - i64::from(start_y)).abs() > i64::from(CONTEXT_MENU_TOUCH_SLOP))
        {
            self.cancel();
        }
    }

    /// Consumes a completed hold once, retaining its original viewport position.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `Option<(i32, i32)>` - `Some(...)` on success, `None` otherwise.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn finish(&mut self, now: f64) -> Option<(i32, i32)> {
        let (x, y, start): (i32, i32, f64) = self.get_origin()?;
        if now - start < f64::from(CONTEXT_MENU_LONG_PRESS_MS) {
            return None;
        }
        self.set_origin(None);
        Some((x, y))
    }

    /// Clears the browser timer, releases its closure, and forgets the gesture.
    pub(crate) fn cancel(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(timer) = self.get_mut_timer().take()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(timer);
        }
        self.set_timer(None);
        self.set_callback(None);
        self.set_origin(None);
    }
}

/// Default construction for [`EuvContextMenuItemProps`].
impl Default for EuvContextMenuItemProps {
    /// Constructs a default [`EuvContextMenuItemProps`] value.
    fn default() -> Self {
        Self {
            open: Signal::default(),
            value: "",
            label: "",
            disabled: false,
            close_on_click: true,
            on_select: None,
        }
    }
}

/// Hover and native activation defaults.
impl Default for EuvContextMenuSubTriggerProps {
    /// Constructs an enabled, hover-opening submenu trigger.
    fn default() -> Self {
        Self {
            label: "",
            open: Signal::default(),
            disabled: false,
            open_on_hover: true,
            delay_ms: CONTEXT_MENU_HOVER_DELAY_MS,
            close_delay_ms: CONTEXT_MENU_HOVER_CLOSE_DELAY_MS,
        }
    }
}

#[cfg(target_arch = "wasm32")]
/// Inherent implementation of [`ContextMenuSubHover`].
impl ContextMenuSubHover {
    /// Detaches listeners and subscriptions before freeing any browser closures.
    pub(super) fn dispose(&self) {
        if !self.get_mounted().replace(false) {
            return;
        }
        self.cancel();
        if let Some(cleanup) = self.get_cleanup().borrow_mut().take() {
            cleanup();
        }
    }

    /// Invalidates pending work before any lifecycle or option transition.
    pub(super) fn cancel(&self) {
        self.get_intent().set(None);
        if let Some(id) = self.get_timer().take()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(id);
        }
    }

    /// Commits only a still-live intent; opening never enters the child popup.
    pub(super) fn commit(&self) {
        self.get_timer().set(None);
        let intent: Option<bool> = self.get_intent().take();
        if !context_menu_hover_owner_live(*self.get_open(), self.get_parent()) {
            self.cancel();
            return;
        }
        if !self.get_mounted().get() || !self.get_parent().get_parent().get() {
            return;
        }
        let Some(_trigger) = self
            .get_target()
            .get_cloned()
            .filter(|trigger: &Element| trigger.is_connected())
        else {
            return;
        };
        if let Some(next) = intent {
            let (disabled, enabled, _, _): (bool, bool, u32, u32) = self.get_options().get();
            if next
                && !context_menu_hover_eligible(
                    CONTEXT_MENU_POINTER_MOUSE,
                    true,
                    disabled,
                    enabled,
                    self.get_parent().get_parent().get(),
                )
            {
                return;
            }
            if !next {
                if let Some(trigger) = self.get_target().get_cloned()
                    && let Some(owner) = trigger.closest(CONTEXT_MENU_SUB_SELECTOR).ok().flatten()
                    && let Some(panel) = owner
                        .query_selector(CONTEXT_MENU_SUB_CONTENT_SELECTOR)
                        .ok()
                        .flatten()
                    && let Some(active) = trigger
                        .owner_document()
                        .and_then(|document: Document| document.active_element())
                    && panel.contains(Some(&active))
                    && let Some(parent) =
                        trigger.closest(CONTEXT_MENU_POPUP_SELECTOR).ok().flatten()
                    && let Some(parent) = parent.dyn_ref::<HtmlElement>()
                {
                    let options: FocusOptions = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);
                    let _: Result<(), JsValue> = parent.focus_with_options(&options);
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
        } else if let Some(window) = web_sys::window()
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

/// Hook eviction must cancel timers even without an app cleanup pass.
#[cfg(target_arch = "wasm32")]
/// Implements [`Drop`] for [`ContextMenuSubHover`].
impl Drop for ContextMenuSubHover {
    /// Disposes while callback fields still exist.
    fn drop(&mut self) {
        self.dispose();
    }
}
