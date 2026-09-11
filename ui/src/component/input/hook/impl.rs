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
}
