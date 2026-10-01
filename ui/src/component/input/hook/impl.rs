use super::*;

/// Implementation of input functionality.
impl UseEuvInput {
    /// Creates a click event handler that toggles a boolean signal.
    ///
    /// Produces a `NativeEventHandler` that flips the value of the given
    /// boolean signal on each click. Useful for toggle buttons, visibility
    /// switches, and drawer open/close patterns.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The boolean signal to toggle.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the signal.
    pub fn use_toggle(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let current: bool = signal.get();
            signal.set(!current);
        }))
    }

    /// Creates an input event handler that updates a string signal.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The signal to update with the input value.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
    pub fn on_input_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let value: Option<String> = event.target().and_then(|target: EventTarget| {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    return Some(input.value());
                }
                if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                    return Some(textarea.value());
                }
                if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    return Some(select.value());
                }
                None
            });
            if let Some(value) = value {
                signal.set(value);
            }
        }))
    }

    /// Creates a change event handler that updates a string signal.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The signal to update with the change value.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
    pub fn on_change_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let value: Option<String> = event.target().and_then(|target: EventTarget| {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    return Some(input.value());
                }
                if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    return Some(select.value());
                }
                if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                    return Some(textarea.value());
                }
                None
            });
            if let Some(value) = value {
                signal.set(value);
            }
        }))
    }

    /// Creates a change event handler that updates a boolean signal from checkbox.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The boolean signal to update with the checked state.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
    pub fn on_change_checked(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            if let Some(target) = event.target()
                && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
            {
                signal.set(input.checked());
            }
        }))
    }

    /// Focus gap (CSS) reserved between the focused input and the on-screen
    /// keyboard. Small enough to feel tight, large enough that the caret does
    /// not graze the IME top edge.
    const FOCUS_GAP_PX: f64 = 12.0;

    /// Time (ms) the browser / WebView is given to bring up the IME and
    /// update the visual viewport before we measure element position.
    const FOCUS_SCROLL_DELAY_MILLIS: i32 = 220;

    /// Creates a focus handler that scrolls the focused input into the
    /// visible area above the soft keyboard.
    ///
    /// The keyboard is accounted for entirely through the visual viewport:
    /// hosts that overlay the IME shrink `visualViewport.height` (mobile
    /// browsers with `interactive-widget=resizes-visual`), while immersive
    /// hosts such as euv-app shrink the layout viewport itself through the
    /// native inset bridge (WebView bottomMargin). Both paths place the
    /// visible bottom edge at `visualViewport.height + offsetTop`, so this
    /// handler never subtracts a keyboard height — doing so double-counts
    /// the IME whenever the host has already resized the view.
    ///
    /// When the document is too short to scroll the input far enough, the
    /// remaining deficit is added to `<main>` as an inline
    /// `padding-bottom` (cleared on blur by [`Self::on_blur_restore_height`]).
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - An event handler performing the scroll on focus.
    pub fn on_focus_scroll_into_view() -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<HtmlElement>() else {
                return;
            };
            let Some(window) = window() else {
                return;
            };
            let element_clone: HtmlElement = element.clone();
            let window_clone: Window = window.clone();
            let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let visible_bottom: f64 = match window_clone.visual_viewport() {
                    Some(viewport) => viewport.height() + viewport.offset_top(),
                    None => window_clone
                        .inner_height()
                        .map(|height: JsValue| height.as_f64().unwrap_or_default())
                        .unwrap_or_default(),
                } - Self::FOCUS_GAP_PX;
                if visible_bottom <= 0.0 {
                    return;
                }
                let input_bottom: f64 = element_clone.get_bounding_client_rect().bottom();
                if input_bottom <= visible_bottom {
                    return;
                }
                let deficit: f64 = input_bottom - visible_bottom;
                window_clone.scroll_by_with_x_and_y(0.0, deficit);
                // Bottom-anchored input in a short document: the scroll above
                // clamps at the document end, so pad <main> by exactly the
                // remaining deficit and scroll once more. The padding equals
                // the missing scroll room — never the full keyboard height.
                let remaining: f64 =
                    element_clone.get_bounding_client_rect().bottom() - visible_bottom;
                if remaining > 0.0 {
                    if let Ok(Some(main_el)) = element_clone.closest("main")
                        && let Ok(main) = main_el.dyn_into::<HtmlElement>()
                    {
                        let _: Result<(), JsValue> = main
                            .style()
                            .set_property("padding-bottom", &format!("{remaining}px"));
                    }
                    window_clone.scroll_by_with_x_and_y(0.0, remaining);
                }
            }));
            let _: Result<i32, JsValue> = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref::<Function>(),
                    Self::FOCUS_SCROLL_DELAY_MILLIS,
                );
            closure.forget();
        }))
    }

    /// Blur handler that strips the inline `padding-bottom` injected by
    /// [`Self::on_focus_scroll_into_view`] so the page returns to its
    /// native layout once the keyboard closes.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - An event handler removing the inline padding on blur.
    pub fn on_blur_restore_height() -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<HtmlElement>() else {
                return;
            };
            if let Ok(Some(main_el)) = element.closest("main")
                && let Ok(main) = main_el.dyn_into::<HtmlElement>()
            {
                let _: Result<String, JsValue> = main.style().remove_property("padding-bottom");
            }
        }))
    }
}
